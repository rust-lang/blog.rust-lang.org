+++
path = "inside-rust/2024/08/09/async-closures-call-for-testing"
title = "Async Closures MVP: Call for Testing!"
authors = ["Michael Goulet"]
aliases = ["inside-rust/2024/08/09/async-closures-call-for-testing.html"]

[extra]
team = "The Async Working Group"
team_url = "https://www.rust-lang.org/governance/wgs/wg-async"
+++

The async working group is excited to announce that [RFC 3668] "Async Closures" was recently approved by the Lang team. In this post, we want to briefly motivate why async closures exist, explain their current shortcomings, and most importantly, announce a call for testing them on nightly Rust.

## The backstory

Async closures were originally proposed in [RFC 2394](https://rust-lang.github.io/rfcs/2394-async_await.html#async--closures) which introduced `async`/`await` to the language. Simple handling of async closures has existed in nightly since async-await was implemented [soon thereafter](https://github.com/rust-lang/rust/pull/51580), but until recently async closures simply desugared into closures that returned async blocks:

```rust
let x = async || {};

// ...was just sugar for:
let x = || { async {} };
```

This had a fundamental limitation that it was impossible to express a closure that returns a future that borrows captured state.

Somewhat relatedly, on the callee side, when users want to take an async closure as an argument, they typically express that as a bound of two different generic types:

```rust
fn async_callback<F, Fut>(callback: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = String>;
```

This also led to an additional limitation that it's impossible to express higher-ranked async fn bounds using this without boxing (since a higher-ranked trait bound on `F` cannot lead to a higher-ranked type for `Fut`), leading to unnecessary allocations:

```rust
fn async_callback<F>(callback: F)
where
    F: FnOnce(&str) -> Pin<Box<dyn Future<Output = ()> + '_>>;

async fn do_something(name: &str) {}

async_callback(|name| Box::pin(async {
    do_something(name).await;
}));
```

These limitations were detailed in [Niko's blog post on async closures and lending](https://smallcultfollowing.com/babysteps/blog/2023/05/09/giving-lending-and-async-closures/#async-closures-are-a-lending-pattern), and later in compiler-errors's blog post on [why async closures are the way they are](https://hackmd.io/@compiler-errors/async-closures).

## OK, so how does [RFC 3668] help?

Recent [work](https://github.com/rust-lang/rust/pull/120361) has focused on reimplementing async closures to be lending and designing a set of async fn traits. While async closures already existed as syntax, this work introduced a new family of async fn traits which are implemented by async closures (and all other callable types which return futures). They can be written like:

```rust
fn test<F>(callback: F)
where
    // Either:
    async Fn(Arg, Arg) -> Ret,
    // Or:
    AsyncFn(Arg, Arg) -> Ret,
```

(It's currently an [open question](https://github.com/rust-lang/rust/issues/128129) exactly how to spell this bound, so both syntaxes are implemented in parallel.)

RFC 3668 motivates this implementation work in detail, confirming that we need first-class async closures and async fn traits which allow us to express the *lending* capability of async closures -- read the RFC if you're interested in the whole story!

## So how do I help?

We'd love for you to test out these new features! First, on a recently-updated nightly compiler, enable `#![feature(async_closure)]` (note that, for historical reasons, this feature name is not pluralized).

Async closures are designed to be drop-in compatible (in almost all cases) with closures returning async blocks:

```rust
// Instead of writing:
takes_async_callback(|arg| async {
    // Do things here...
});

// Write this:
takes_async_callback(async |arg| {
    // Do things here...
});
```

And on the callee side, write async fn trait bounds instead of writing "regular" fn trait bounds that return futures:

```rust
// Instead of writing:
fn doesnt_exactly_take_an_async_closure<F, Fut>(callback: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = String>
{ todo!() }

// Write this:
fn takes_an_async_closure<F: async FnOnce() -> String>(callback: F) { todo!() }
// Or this:
fn takes_an_async_closure<F: AsyncFnOnce() -> String>(callback: F) { todo!() }
```

Or if you're emulating a higher-ranked async closure with boxing:

```rust
// Instead of writing:
fn higher_ranked<F>(callback: F)
where
    F: Fn(&Arg) -> Pin<Box<dyn Future<Output = ()> + '_>>
{ todo!() }

// Write this:
fn higher_ranked<F: async Fn(&Arg)> { todo!() }
// Or this:
fn higher_ranked<F: AsyncFn(&Arg)> { todo!() }
```

## Shortcomings interacting with the async ecosystem

If you're going to try to rewrite your async projects, there are a few shortcomings to be aware of.

### You can't directly name the output future

When you name an async callable bound with the *old* style, before first-class async fn trait bounds, then as a side-effect of needing to use two type parameters, you can put additional bounds (e.g. `+ Send` or `+ 'static`) on the `Future` part of the bound, like:

```rust
fn async_callback<F, Fut>(callback: F)
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = String> + Send + 'static
{ todo!() }
```

There isn't currently a way to put similar bounds on the future returned by calling an async closure, so if you need to constrain your callback futures like this, then you won't be able to use async closures just yet.

We expect to support this in the medium/long term via a [return-type-notation syntax](https://rust-lang.github.io/rfcs/3668-async-closures.html#interaction-with-return-type-notation-naming-the-future-returned-by-calling).

### Subtle differences in closure signature inference

Passing an async closure to a generic `impl Fn(A, B) -> C` bound may not always eagerly infer the closure's arguments to `A` and `B`, leading to strange type errors on occasion. For an example of this, see [`rust-lang/rust#127781`](https://github.com/rust-lang/rust/issues/127781).

We expect to improve async closure signature inference as we move forward.

### Async closures can't be coerced to `fn()` pointers

Some libraries take their callbacks as function *pointers* (`fn()`) rather than generics. Async closures don't currently implement the same coercion from closure to `fn() -> ...`. Some libraries may mitigate this problem by adapting their API to take generic `impl Fn()` instead of `fn()` pointers as an argument.

We don't expect to implement this coercion unless there's a particularly good reason to support it, since this can usually be handled manually by the caller by using an inner function item, or by using an `Fn` bound instead, for example:

```rust
fn needs_fn_pointer<T: Future<Output = ()>>(callback: fn() -> T) { todo!() }

fn main() {
    // Instead of writing:
    needs_fn_pointer(async || { todo!() });
    // Since async closures don't currently support coercion to `fn() -> ...`.

    // You can use an inner async fn item:
    async fn callback() { todo!() }
    needs_fn_pointer(callback);
}

// Or if you don't need to take *exactly* a function pointer,
// you can rewrite `needs_fn_pointer` like:
fn needs_fn_pointer(callback: impl async Fn()) { todo!() }
// Or with `AsyncFn`:
fn needs_fn_pointer(callback: impl AsyncFn()) { todo!() }
```

[RFC 3668]: https://rust-lang.github.io/rfcs/3668-async-closures.html
