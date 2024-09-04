---
layout: post
title: "Changes to impl Trait in Rust 2024"
author: Niko Matsakis
team: the language team <https://www.rust-lang.org/governance/teams/lang>
---
This blog post describes some small but significant changes with (return position) `impl Trait` capture rules that are coming in Rust 2024. The goal of these changes is to simplify how `impl Trait` works to better match what people want while also giving a flexible syntax that allows users to have full control when needed.

## TL;DR

* Return-position impl Trait in Rust 2024 can now reference all fn arguments by default.
* We are introducing a new syntax (`+ use<>`) that you can use to specify precisely which generic parameters can appear in the hidden type.

## Background: return position impl trait

This blog post concerns *return position `impl Trait`*, such as the following example:

```rust
fn process_datums(
    datums: &[Datum]
) -> impl Iterator<Item = ProcessedDatum> {
    datums
        .iter()
        .map(|datum| datum.process())
}
```

The use of `-> impl Iterator` in return position here means that the functions "some kind of iterator". The actual type will be determined by the compiler based on the function body. It is called the "hidden type" because callers do not get to know exactly what it is, they have to code against the `Iterator` trait. However, at code generation time, the compiler will generate code based on the actual precise type, which ensures that callers are fully optimized.

Although callers don't know the exact type, they do need to know that it will continue to borrow the `datums` argument so that they can ensure that the `datums` reference remains valid while iteration occurs. Further, callers must be able to figure this out based solely on the type signature, without looking at the function body.

Rust's current rules are that a return-position `impl Trait` can only capture references if the lifetime of that reference appears in the `impl Trait` itself. In this example, `impl Iterator<Item = ProcessedDatum>` does not reference any lifetimes, and therefore capturing `datums` is illegal. You can see this for yourself [on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2448fc4ec9e763c538aaba897433f9b5).

The error message ("hidden type captures lifetime") you get in this scenario is not the most intuitive, but it does come with a useful suggestion for how to fix it:

```
help: to declare that
      `impl Iterator<Item = ProcessedDatum>`
      captures `'_`, you can add an
      explicit `'_` lifetime bound
  |
5 | ) -> impl Iterator<Item = ProcessedDatum> + '_ {
  |                                           ++++
```

Following a slightly more explicit version of this advice, the function signature becomes:


```rust
fn process_datums<'d>(
    datums: &'d [Datum]
) -> impl Iterator<Item = ProcessedDatum> + 'd {
    datums
        .iter()
        .map(|datum| datum.process())
}
```

In this version, the lifetime `'d` of the datums is explicitly referenced in the `impl Trait` type, and so capture is allowed. This is also a signal to the caller that the borrow for `datums` must last as long as the iterator is in use, which means that it (correctly) flags an error in an example like this ([try it on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afd9278ac887c0b2fc08bc868200808f)):

```rust
let mut datums: Vec<Datum> = vec![Datum::default()];
let iter = process_datums(&datums);
datums.push(Datum::default()); // <-- Error!
iter.next();
```

## Usability problems with this design

The capture rules for `impl Trait` were decided early on based on a limited set of examples. Over time we have noticed a number of problems with them.

### not the right default

Surveys of major codebases (both the compiler and crates on crates.io) found that the vast majority of return-position impl trait values need to capture lifetimes, so the default behavior of not capturing is not helpful.

### not sufficiently flexible

The current rule is that return-position impl trait *always* captures type parameters and *sometimes* captures lifetime parameters (if they appear in the bounds). As noted above, this default is wrong because most functions actually DO want to capture lifetime parameters: that at least has a workaround (modulo some details we'll not below). But the default is also wrong because some functions do NOT want to capture type parameters, and there is no way to override that right now. The original intention was that [type alias impl trait](https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html) would solve this use case, but that would be a very non-ergonomic solution (and stabilizing type alias impl trait is taking longer than anticipated due to other complications).

### hard to explain

Because the defaults are wrong, these errors are encountered by users fairly regularly, and yet they are also subtle and hard to explain (as evidenced by this post!). Adding the compiler hint to suggest `+ '_` helps, but it's not great that users have to follow a hint they don't fully understand.

### incorrect suggestion

Adding a `+ '_` argument to `impl Trait` may be confusing, but it's not terribly difficult. Unfortunately, it's often the wrong annotation, leading to unnecessary compiler errors -- and the *right* fix is either complex or sometimes not even possible. Consider an example like this:

```rust
fn process<T> {
    context: &Context,
    datums: Vec<T>,
) -> impl Iterator<Item = ()> + '_ {
    datums
        .into_iter()
        .map(|datum| context.process(datum))
}
```

Here the `process` function applies `context.process` to each of the elements in `datums` (of type `T`). Because it captures `context`, it is declared as `+ '_`. This tells the compiler that the hidden type must outlive `'_`. Because this hidden type includes the `Vec<T>`, this in turn means that `T` must outlive `'_` -- but this is not true. So you get a compilation error ([try it on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b742fbf9b083a6e837db0b170489f34a)). 

Just as before, this error is obscure, touching on the more complex aspects of Rust's type system. Unlike before, there is no easy fix! This problem in fact occurred frequently in the compiler, leading to an [obscure workaround called the `Captures` trait](https://github.com/rust-lang/rust/issues/34511#issuecomment-373423999). Gross!

We surveyed crates on crates.io and found that the vast majority of cases involving return-position impl trait and generics had bounds that were too strong and which could lead to unnecessary errors (though often they were used in simple ways that didn't trigger an error).

### inconsistencies with other parts of Rust

The current design was also introducing inconsistencies with other parts of Rust.

#### async fn desugaring

Rust defines an `async fn` as desugaring to a normal `fn` that returns `-> impl Future`. You might therefore expect that a function like `process`:

```rust
async fn process(data: &Data) { .. }
```

...would be (roughly) desugared to:

```rust
fn process(
    data: &Data
) -> impl Future<Output = ()> {
    async move {
        ..
    }
}
```

In practice, because of the problems with lifetime capture, this is not the actual desugaring. The actual desugaring is to a special kind of `impl Trait` that is allowed to capture all lifetimes. But that form of `impl Trait` was not exposed to end-users.

#### impl trait in traits

As we pursued the design for impl trait in traits ([RFC 3425](https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html)), we encountered a number of challenges related to the capturing of lifetimes. [In order to get the symmetries that we wanted to work](https://hackmd.io/zgairrYRSACgTeZHP1x0Zg) (e.g., that one can write `-> impl Future` in a trait and impl with the expected effect), we had to change the capture rules to capture all lifetime parameters uniformly.

## Rust 2024 design

The above problems motivated us to take a new approach in Rust 2024. The approach is a combination of two things:

* a new default, applicable only in Rust 2024;
* a syntax for explicit capture, usably in any edition, written like 

We cover each in turn.

### Impl Traits now capture lifetimes by default

In Rust 2024, the default is that return-position impl Trait values can capture all the references that are in scope. Therefore the initial example of this blog post will compile just fine in Rust 2024 ([try it yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=d366396da2fbd5334b7560c3dfb3290b)):

```rust
fn process_datums(
    datums: &[Datum]
) -> impl Iterator<Item = ProcessedDatum> {
    datums
        .iter()
        .map(|datum| datum.process())
}
```

Yay!

### Impl Traits can include a `use<>` bound to specify precisely which generic types and lifetimes they use

There are however times where it is useful not to capture lifetime parameters. Consider this example, which takes a slice of type T `&[T]` and returns an iterator over its indices:

```rust!
fn indices<T>(
    slice: &[T],
) -> impl Iterator<Item = usize> {
    0 .. slice.len()
}
```

If you look closely, you can see that it only needs to read the length of the slice and doesn't hold a reference to `slice` afterwards. Nonetheless, callers today will assume that the return value *may* hold a reference to `slice`, which means that this caller for example would get an error:

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let i = indices(&data);
    data.push(4); // <-- error!
    i.next();
}
```

This may actually be what you want: it gives you room to modify `indices` later and have it actually make use of `slice` in the returned iterator, for example. Put another way, the new default continues the `impl Trait` tradition of retaining flexibility for the function to change its implementation without breaking callers.

But what if it's *not* what you want? What if you want to guarantee that `indices` will not retain a reference to `data` in its return value? You can do that now with the new `use` bounds. The idea is that your `impl Trait` can include a bound written `use<...>` which lists out the generic parameters that the return value may use. If you include `+ use<>`, then, this means "do not capture anything". So changing `indices` to be written as follows will allow `main` to compile:

```rust
fn indices<T>(
    slice: &[T],
) -> impl Iterator<Item = usize> + use<> {
    0 .. slice.len()
}
```

**Implementation limitation.** Unfortunately, if you actually try the above example on nightly today, you'll see that it doesn't compile ([try it for yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=1d6d23ef3813da05ac78a4b97b418f21)). That's because `use<>` bounds have only partially been implemented: currently, they must always include at least the type parameters. This corresponds to the limitations of impl trait in earlier editions, which always *must* capture type parameters. In this case, that means we can write the following, which also avoids the compilation error, but is still more conservative than necessary ([try it yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=7965043f4686d5a89b47aa5bfc4f996f)):

```rust
fn indices<T>(
    slice: &[T],
) -> impl Iterator<Item = usize> + use<T> {
    0 .. slice.len()
}
```

## Conclusion

This example demonstrates the way that editions can help us to remove complexity from Rust. The new `impl Trait` capture rules mean that:

* Most code will "just work" in Rust 2024, avoiding confusing errors.
* For the code where annotations are required, we now have a more powerful annotation mechanism that can let you say exactly what you need to say.

