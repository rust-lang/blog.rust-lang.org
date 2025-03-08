+++
layout = "post"
title = "Return type notation MVP: Call for testing!"
author = "Michael Goulet"
team = "The Async Working Group <https://www.rust-lang.org/governance/wgs/wg-async>"
+++

The async working group is excited to announce that [RFC 3654] return type notation (RTN) is ready for testing on nightly Rust. In this post, we'll briefly describe the feature.

## The backstory

Rust 1.75 [stabilized](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html) async fn in traits (AFIT) and return-position impl Trait in traits (RPITIT). These desugar to anonymous generic associated types (GATs). However, unlike GATs, users of these types cannot use `where` clauses to further restrict these return types. This is known as the ["send bound"](https://smallcultfollowing.com/babysteps/blog/2023/02/01/async-trait-send-bounds-part-1-intro/) problem, since it often affects `Send` bounds on futures in the async ecosystem.

### An example

Consider a trait `Foo` with a `method` that returns a type of `impl Future<Output = ()>`. We want to write a function that calls `method` and spawns the future on another thread:

```rust
fn spawn<T>(f: impl Future<Output = T> + Send + 'static) {}

trait Foo {
    fn method() -> impl Future<Output = ()>; // <-- RPITIT.
}

fn needs_sendable_future<T: Foo>()
where
    // How do we further restrict `T::method()`
    // to be `Send + 'static`?
{
    spawn(T::method());
    //~^ ERROR: `impl Future<Output = ()>` is not `Send`!
}
```

Specifically, we may not want to restrict the *declaration* of `Foo`, since changing it in the declaration would restrict *all* implementations of `Foo`.

```rust
trait Foo {
    fn method() -> impl Future<Output = ()> + Send + 'static;
    //                                      ~~~~~~~~~~~~~~~~
    //                                      Not what we want.
}
```

So, on stable Rust, we have no way of expressing this restriction when using AFIT or RPITIT. In contrast, we can express this today if we were to use a GAT directly:

```rust
trait Foo {
    type MethodFuture: Future<Output = ()>;
    fn method() -> Self::MethodFuture;
}

fn needs_sendable_future<T: Foo>()
where
    // We can restrict this to only implementors of `Foo`
    // whose `MethodFuture` is `Send + 'static`, so we can
    // call `spawn` below:
    T::MethodFuture: Send + 'static
{
    spawn(T::method());
}
```

However, using GATs means that implementors of `Foo` have to write out the return type explicitly, `type MethodFuture = ...`, which doesn't ([yet](https://github.com/rust-lang/rust/pull/120700)) work if we have an anonymous, unnameable `Future` type!

## The solution

In [RFC 3654] we introduced return type notation (RTN). This will allow us to write `where` clause bounds that restrict the return types of functions and methods that use async fn in traits (AFIT) and return-position impl Trait in traits (RPITIT). Extending the example above, RTN lets us write:

```rust
fn needs_sendable_future<T: Foo>()
where
    T::method(..): Send + 'static // Yay!
{
    spawn(T::method());
    //~^ Works!
}
```

## Restrictions

Currently, RTN is only allowed for trait associated functions and methods with lifetime generics (not const or type generics) that use:

* async fn in traits (AFIT) or
* return-position impl Trait in traits (RPITIT) where the impl Trait is the outermost return type, i.e. `-> impl Trait`, but not `-> Box<impl Trait>`.

These restrictions are described in further detail in [RFC 3654].

## How do I help?

We'd love for you to test out this feature on the latest Rust nightly compiler[^nightly].

[^nightly]: Make sure to run `rustup update nightly` (or however you manage your Rust releases), since the feature is very new and is still unstable!

Specifically, we'd like for you to identify traits where you're unnecessarily restricting your trait definitions with `+ Send` or similar bounds:

```rust
// Instead of writing a trait like:

trait Foo {
    fn method() -> impl Future<Output = ()> + Send + 'static;
}

// Write this:

trait Foo {
    async fn method();
}

// And then at the call site, add:

fn use_foo<T: Foo>()
where
    T::method(..): Send + 'static,
{}
```

Similarly, we'd like for you to identify traits that currently are returning GATs for the same reason:

```rust
// Instead of writing this in the trait and call site:

trait Foo {
    type MethodFuture: Future<Output = ()>;
    fn method() -> Self::MethodFuture;
}

fn use_foo<T: Foo>()
where
    T::MethodFuture: Send + 'static,
{}

// Write this:

trait Foo {
    async fn method();
}

fn use_foo<T: Foo>()
where
    T::method(..): Send + 'static,
{}
```

Note, however, that we don't yet support RTN in type position. So while, with the first version, you can write:

```rust
struct Bar<T: Foo> {
    field: T::MethodFuture,
}
```

You can't yet, with the second version, write:

```rust
struct Bar<T: Foo> {
    field: T::method(..),
}
```

We'd be interested in hearing about any places where you would run into this limitation.

We're excited for RTN to make it easier to use async fn in traits (AFIT) in `Send`-bound-heavy async Rust ecosystems.

As always, take a look at the [RFC][RFC 3654] itself for a detailed explanation for why we settled on this design, in particular the [frequently-asked questions and rationale](https://rust-lang.github.io/rfcs/3654-return-type-notation.html#rationale-and-alternatives).

[RFC 3654]: https://rust-lang.github.io/rfcs/3654-return-type-notation.html
[RFC 3425]: https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html
