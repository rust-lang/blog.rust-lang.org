---
layout: post
title: "Improving async-await's \"Future is not Send\" diagnostic"
author: David Wood
description: "Highlighting a diagnostic improvement for async-await"
team: the Async Foundations WG <https://rust-lang.github.io/compiler-team/working-groups/async-await/>
---

Async-await is due to hit stable in the 1.39 release (only a month away!), and as announced in the
["Async Foundations Update: Time for polish!"][previous_post] post last month, the Async
Foundations WG has shifted its focus to polish. This post will highlight one aspect of that
focus, diagnostic improvements, and in particular, the improvements that the working group has
been making to the once-unhelpful "future is not send" diagnostic.

# Why doesn't my future implement `Send`?

One of the major places where async-await should be a pleasure to use is in multithreaded contexts,
where having a future that can be sent to other threads is desirable. This might look something
like the following (for brevity, there aren't any threads here, just a requirement that the
future implement `std::marker::Send`):

```rust
use std::sync::{Mutex, MutexGuard};

fn is_send<T: Send>(t: T) { }

async fn foo() {
    bar(&Mutex::new(22)).await
}

async fn bar(x: &Mutex<u32>) {
    let g = x.lock().unwrap();
    baz().await
}

async fn baz() { }

fn main() {
    is_send(foo());
}
```

When we try to compile this, we'll get an unwieldly and hard-to-follow diagnostic:

```
error[E0277]: `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
  --> src/main.rs:23:5
   |
23 |     is_send(foo());
   |     ^^^^^^^ `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
   = note: required because it appears within the type `for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:13:30: 16:2 x:&std::sync::Mutex<u32> for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:13:30: 16:2 x:&std::sync::Mutex<u32> for<'r, 's> {&'r std::sync::Mutex<u32>, std::sync::MutexGuard<'s, u32>, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `for<'r> {impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:9:16: 11:2 for<'r> {impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:9:16: 11:2 for<'r> {impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `is_send`
  --> src/main.rs:5:1
   |
5  | fn is_send<T: Send>(t: T) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
```

That's.. not great. Let's break down what's happening and understand what this error is trying to
tell us.

```rust
fn main() {
    is_send(foo());
}
```

In `main`, we are calling `foo` and passing the return value to `is_send`. `foo` is an `async fn`,
so it doesn't return `()` (what you might expect for a function with no return type specified).
Instead, it returns `impl std::future::Future<Output = ()>`, some unnamed type that implements
`std::future::Future`:

```rust
async fn foo() {
    bar(&Mutex::new(22)).await
}

// becomes...

fn foo() -> impl std::future::Future<Output = ()> {
    async move {
        bar(&Mutex::new(22)).await
    }
}
```

If you want to learn more about the transformations that happen with async-await, consider
reading the [`async`/`.await` primer chapter of the async book][primer].

```rust
fn is_send<T: Send>(t: T) { }
```

It looks like the error we're getting is because the future returned by `foo` doesn't satisfy
the `T: Send` bound of `is_send`.

## How are async functions implemented?

To explain why our future doesn't implement `Send`, we first need to understand a little bit more
about what async-await is doing under the hood. rustc implements `async fn`s using generators,
an unstable language feature for resumable functions like the co-routines you might be familiar
with from other languages. Generators are laid out like enums with variants containing all of the
variables that are used across await points (which desugar to generator yields):

```rust
async fn bar(x: &Mutex<u32>) {
    let g = x.lock().unwrap();
    baz().await // <- await point (suspend #0), `g` and `x` are in use before await point
} // <- `g` and `x` dropped here, after await point
```

```rust
enum BarGenerator {
    // `bar`'s parameters.
    Unresumed { x: &Mutex<u32> },

    Suspend0 {
        // Future returned by `baz`, which is being polled.
        baz_future: BazGenerator,

        // Locals that are used across the await point.
        x: &Mutex<u32>,
        g: MutexGuard<'_, u32>,
    },

    Returned { value: () }
}
```

If you want to learn more about the `async fn` implementation details, then Tyler Mandry has
written [an excellent blog post][tmandry_post] diving into their work here in more depth which is
definitely worth a read.

## So, why doesn't my future implement `Send`?

We now know that an `async fn` is represented like an enum behind-the-scenes. In synchronous Rust,
you'll be used to your types automatically implementing `Send` when the
[compiler determines it's appropriate][send_doc] - typically when all of the fields of your type
also implement `Send`. It follows that the enum-like that represents our `async fn` would
implement `Send` if all of the types in it do.

In other words, a future is safe to send across threads if all of the types that are held across
`.await` points implement `Send`. This behaviour is useful because it lets us write generic code
that interoperates smoothly with async-await, but without diagnostic support we get confusing error
messages.

## Well, which type is the problem in the example?

Returning to our example, the future must be holding a type across an `.await` point that doesn't
implement `Send`, but where? This is the primary question that the diagnostic improvement aims to
help answer. Let's start by looking at `foo`:

```rust
async fn foo() {
    bar(&Mutex::new(22)).await
}
```

`foo` invokes `bar`, passing a reference to a `std::sync::Mutex` and getting a future back, before
`await`ing it.

```rust
async fn bar(x: &Mutex<u32>) {
    let g: MutexGuard<u32> = x.lock().unwrap();
    baz().await
} // <- `g` is dropped here
```

`bar` locks the mutex before `await`ing `baz`. `std::sync::MutexGuard<u32>` does not implement
`Send` and lives across the `baz().await` point (because `g` is dropped at the end of the scope)
which causes the entire future not to implement `Send`.

That wasn't obvious from the error: we had to know that futures might implement `Send` depending
on the types they capture *and* find the type which lives across an await point ourselves.

Fortunately, the Async Foundations WG has been working to improve this error, and
[in nightly][play], we see the following diagnostic instead:

```
error[E0277]: `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
  --> src/main.rs:23:5
   |
5  | fn is_send<T: Send>(t: T) {
   |    -------    ---- required by this bound in `is_send`
...
23 |     is_send(foo());
   |     ^^^^^^^ `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
note: future does not implement `std::marker::Send` as this value is used across an await
  --> src/main.rs:15:3
   |
14 |   let g = x.lock().unwrap();
   |       - has type `std::sync::MutexGuard<'_, u32>`
15 |   baz().await;
   |   ^^^^^^^^^^^ await occurs here, with `g` maybe used later
16 | }
   | - `g` is later dropped here
```

Much better!

# How does it work?

When rustc's trait system determines that a trait wasn't implemented, in this case
`std::marker::Send`, it emits this error. The trait system produces a chain of "obligations".
Obligations are types which denote where a bound (e.g `T: Send` in `is_send`) originated,
or where a bound was propagated.

To improve this diagnostic, the chain of obligations is now treated like a stack frame, where
each "frame" of obligations represents each function's contribution to the error. Let's make
that more concrete with a very rough approximation:

```rust
Obligation {
    kind: DerivedObligation(/* generator that captured `g` */),
    source: /* `Span` type pointing at `bar`'s location in user code */,
    parent: Some(Obligation {
        kind: DerivedObligation(/* generator calling `bar` */),
        source: /* `Span` type pointing at `foo`'s location in user code */,
        parent: Some(Obligation {
            kind: ItemObligation(/* type representing `std::marker::Send` */),
            source: /* `Span` type pointing at `is_send`'s location in user code */,
            cause: None,
        }),
    }),
}
```

The compiler matches against the chain expecting an `ItemObligation` and some `DerivedObligation`s
containing generators, which identifies the error we want to improve. Using information from these
obligations, rustc can construct the specialized error shown above - if you'd like to see what the
actual implementation looks like, check out PR [#64895][pr64895].

If you're interested in improving diagnostics like this, or even just fixing bugs, consider
contributing to the compiler! There are many working groups to join and resources to help you get
started (like the [rustc guide][rustc_guide] or the [compiler team documentation][compiler_team]).

# What's next?

More improvements to this diagnostic are planned and being worked on, so that it is applicable in
more cases, and has specialized messages for `Send` and `Sync`, like below:

```
error[E0277]: future cannot be sent between threads safely
  --> src/main.rs:23:5
   |
5  | fn is_send<T: Send>(t: T) {
   |    -------    ---- required by this bound in `is_send`
...
23 |     is_send(foo());
   |     ^^^^^^^ future returned by `foo` is not `Send`
   |
   = help:  future is not `Send` as this value is used across an await
note: future does not implement `std::marker::Send` as this value is used across an await
  --> src/main.rs:15:3
   |
14 |   let g = x.lock().unwrap();
   |       - has type `std::sync::MutexGuard<'_, u32>`
15 |   baz().await;
   |   ^^^^^^^^^^^ await occurs here, with `g` maybe used later
16 | }
   | - `g` is later dropped here
```

[primer]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
[previous_post]: https://blog.rust-lang.org/inside-rust/2019/10/07/AsyncAwait-WG-Focus-Issues.html
[tmandry_post]: https://tmandry.gitlab.io/blog/posts/optimizing-await-1/
[send_doc]: https://doc.rust-lang.org/std/marker/trait.Send.html
[compiler_team]: https://rust-lang.github.io/compiler-team
[rustc_guide]: https://rust-lang.github.io/rustc-guide
[pr64895]: https://github.com/rust-lang/rust/pull/64895
[play]: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=7e80a8bc151df8817e0983e55bf2667a
