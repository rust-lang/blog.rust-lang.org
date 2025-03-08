+++
layout = "post"
title = "Changes to `impl Trait` in Rust 2024"
author = "Niko Matsakis"
team = "the language team <https://www.rust-lang.org/governance/teams/lang>"
+++
The default way `impl Trait` works in return position is changing in Rust 2024. These changes are meant to simplify `impl Trait` to better match what people want most of the time. We're also adding a flexible syntax that gives you full control when you need it.

## TL;DR

Starting in Rust 2024, we are changing the rules for when a generic parameter can be used in the hidden type of a return-position `impl Trait`:

* a new default that the hidden types for a return-position `impl Trait` can use **any** generic parameter in scope, instead of only types (applicable only in Rust 2024);
* a syntax to declare explicitly what types may be used (usable in any edition).

The new explicit syntax is called a "use bound": `impl Trait + use<'x, T>`, for example, would indicate that the hidden type is allowed to use `'x` and `T` (but not any other generic parameters in scope).

Read on for the details!

## Background: return-position `impl Trait`

This blog post concerns *return-position `impl Trait`*, such as the following example:

```rust
fn process_data(
    data: &[Datum]
) -> impl Iterator<Item = ProcessedDatum> {
    data
        .iter()
        .map(|datum| datum.process())
}
```

The use of `-> impl Iterator` in return position here means that the function returns "some kind of iterator". The actual type will be determined by the compiler based on the function body. It is called the "hidden type" because callers do not get to know exactly what it is; they have to code against the `Iterator` trait. However, at code generation time, the compiler will generate code based on the actual precise type, which ensures that callers are fully optimized.

Although callers don't know the exact type, they do need to know that it will continue to borrow the `data` argument so that they can ensure that the `data` reference remains valid while iteration occurs. Further, callers must be able to figure this out based solely on the type signature, without looking at the function body.

Rust's current rules are that a return-position `impl Trait` value can only use a reference if the lifetime of that reference appears in the `impl Trait` itself. In this example, `impl Iterator<Item = ProcessedDatum>` does not reference any lifetimes, and therefore capturing `data` is illegal. You can see this for yourself [on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2448fc4ec9e763c538aaba897433f9b5).

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
fn process_data<'d>(
    data: &'d [Datum]
) -> impl Iterator<Item = ProcessedDatum> + 'd {
    data
        .iter()
        .map(|datum| datum.process())
}
```

In this version, the lifetime `'d` of the data is explicitly referenced in the `impl Trait` type, and so it is allowed to be used. This is also a signal to the caller that the borrow for `data` must last as long as the iterator is in use, which means that it (correctly) flags an error in an example like this ([try it on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afd9278ac887c0b2fc08bc868200808f)):

```rust
let mut data: Vec<Datum> = vec![Datum::default()];
let iter = process_data(&data);
data.push(Datum::default()); // <-- Error!
iter.next();
```

## Usability problems with this design

The rules for what generic parameters can be used in an `impl Trait` were decided early on based on a limited set of examples. Over time we have noticed a number of problems with them.

### not the right default

Surveys of major codebases (both the compiler and crates on crates.io) found that the vast majority of return-position impl trait values need to use lifetimes, so the default behavior of not capturing is not helpful.

### not sufficiently flexible

The current rule is that return-position impl trait *always* allows using type parameters and *sometimes* allows using lifetime parameters (if they appear in the bounds). As noted above, this default is wrong because most functions actually DO want their return type to be allowed to use lifetime parameters: that at least has a workaround (modulo some details we'll note below). But the default is also wrong because some functions want to explicitly state that they do NOT use type parameters in the return type, and there is no way to override that right now. The original intention was that [type alias impl trait](https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html) would solve this use case, but that would be a very non-ergonomic solution (and stabilizing type alias impl trait is taking longer than anticipated due to other complications).

### hard to explain

Because the defaults are wrong, these errors are encountered by users fairly regularly, and yet they are also subtle and hard to explain (as evidenced by this post!). Adding the compiler hint to suggest `+ '_` helps, but it's not great that users have to follow a hint they don't fully understand.

### incorrect suggestion

Adding a `+ '_` argument to `impl Trait` may be confusing, but it's not terribly difficult. Unfortunately, it's often the wrong annotation, leading to unnecessary compiler errors -- and the *right* fix is either complex or sometimes not even possible. Consider an example like this:

```rust
fn process<'c, T> {
    context: &'c Context,
    data: Vec<T>,
) -> impl Iterator<Item = ()> + 'c {
    data
        .into_iter()
        .map(|datum| context.process(datum))
}
```

Here the `process` function applies `context.process` to each of the elements in `data` (of type `T`). Because the return value uses `context`, it is declared as `+ 'c`. Our real goal here is to allow the return type to use `'c`; writing `+ 'c` achieves that goal because `'c` now appears in the bound listing. However, while writing `+ 'c` is a convenient way to make `'c` appear in the bounds, also means that the hidden type must outlive `'c`. This requirement is not needed and will in fact lead to a compilation error in this example ([try it on the playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b742fbf9b083a6e837db0b170489f34a)).

The reason that this error occurs is a bit subtle. The hidden type is an iterator type based on the result of `data.into_iter()`, which will include the type `T`. Because of the `+ 'c` bound, the hidden type must outlive `'c`, which in turn means that `T` must outlive `'c`. But `T` is a generic parameter, so the compiler requires a where-clause like `where T: 'c`. This where-clause means "it is safe to create a reference with lifetime `'c` to the type `T`". But in fact we don't create any such reference, so the where-clause should not be needed. It is only needed because we used the convenient-but-sometimes-incorrect workaround of adding `+ 'c` to the bounds of our `impl Trait`.

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

In practice, because of the problems with the rules around which lifetimes can be used, this is not the actual desugaring. The actual desugaring is to a special kind of `impl Trait` that is allowed to use all lifetimes. But that form of `impl Trait` was not exposed to end-users.

#### impl trait in traits

As we pursued the design for impl trait in traits ([RFC 3425](https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html)), we encountered a number of challenges related to the capturing of lifetimes. [In order to get the symmetries that we wanted to work](https://hackmd.io/zgairrYRSACgTeZHP1x0Zg) (e.g., that one can write `-> impl Future` in a trait and impl with the expected effect), we had to change the rules to allow hidden types to use *all* generic parameters (type and lifetime) uniformly.

## Rust 2024 design

The above problems motivated us to take a new approach in Rust 2024. The approach is a combination of two things:

* a new default that the hidden types for a return-position `impl Trait` can use **any** generic parameter in scope, instead of only types (applicable only in Rust 2024);
* a syntax to declare explicitly what types may be used (usable in any edition).

The new explicit syntax is called a "use bound": `impl Trait + use<'x, T>`, for example, would indicate that the hidden type is allowed to use `'x` and `T` (but not any other generic parameters in scope).

### Lifetimes can now be used by default

In Rust 2024, the default is that the hidden type for a return-position `impl Trait` values use **any** generic parameter that is in scope, whether it is a type or a lifetime. This means that the initial example of this blog post will compile just fine in Rust 2024 ([try it yourself by setting the Edition in the Playground to 2024](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=d366396da2fbd5334b7560c3dfb3290b)):

```rust
fn process_data(
    data: &[Datum]
) -> impl Iterator<Item = ProcessedDatum> {
    data
        .iter()
        .map(|datum| datum.process())
}
```

Yay!

### Impl Traits can include a `use<>` bound to specify precisely which generic types and lifetimes they use

As a side-effect of this change, if you move code to Rust 2024 by hand (without `cargo fix`), you may start getting errors in the callers of functions with an `impl Trait` return type. This is because those `impl Trait` types are now assumed to potentially use input lifetimes and not only types. To control this, you can use the new `use<>` bound syntax that explicitly declares what generic parameters can be used by the hidden type. Our experience porting the compiler suggests that it is very rare to need changes -- most code actually works better with the new default.

The exception to the above is when the function takes in a reference parameter that is only used to read values and doesn't get included in the return value. One such example is the following function `indices()`: it takes in a slice of type `&[T]` but the only thing it does is read the length, which is used to create an iterator. The slice itself is not needed in the return value:

```rust
fn indices<'s, T>(
    slice: &'s [T],
) -> impl Iterator<Item = usize> {
    0 .. slice.len()
}
```

In Rust 2021, this declaration implicitly says that `slice` is not used in the return type. But in Rust 2024, the default is the opposite. That means that callers like this will stop compiling in Rust 2024, since they now assume that `data` is borrowed until iteration completes:

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let i = indices(&data);
    data.push(4); // <-- Error!
    i.next(); // <-- assumed to access `&data`
}
```

This may actually be what you want! It means you can modify the definition of `indices()` later so that it actually *does* include `slice` in the result. Put another way, the new default continues the `impl Trait` tradition of retaining flexibility for the function to change its implementation without breaking callers.

But what if it's *not* what you want? What if you want to guarantee that `indices()` will not retain a reference to its argument `slice` in its return value? You now do that by including a `use<>` bound in the return type to say explicitly which generic parameters may be included in the return type. 

In the case of `indices()`, the return type actually uses **none** of the generics, so we would ideally write `use<>`:

```rust
fn indices<'s, T>(
    slice: &'s [T],
) -> impl Iterator<Item = usize> + use<> {
    //                             -----
    //             Return type does not use `'s` or `T`
    0 .. slice.len()
}
```

**Implementation limitation.** Unfortunately, if you actually try the above example on nightly today, you'll see that it doesn't compile ([try it for yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=1d6d23ef3813da05ac78a4b97b418f21)). That's because `use<>` bounds have only partially been implemented: currently, they must always include at least the type parameters. This corresponds to the limitations of `impl Trait` in earlier editions, which always *must* capture type parameters. In this case, that means we can write the following, which also avoids the compilation error, but is still more conservative than necessary ([try it yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=7965043f4686d5a89b47aa5bfc4f996f)):

```rust
fn indices<T>(
    slice: &[T],
) -> impl Iterator<Item = usize> + use<T> {
    0 .. slice.len()
}
```

This implementation limitation is only temporary and will hopefully be lifted soon! You can follow the current status at [tracking issue #130031](https://github.com/rust-lang/rust/issues/130031).

**Alternative: `'static` bounds.** For the special case of capturing **no** references at all, it is also possible to use a `'static` bound, like so ([try it yourself](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=3054bbf64652cb4890d56ac03b47a35c)):

```rust
fn indices<'s, T>(
    slice: &'s [T],
) -> impl Iterator<Item = usize> + 'static {
    //                             -------
    //             Return type does not capture references.
    0 .. slice.len()
}
```

`'static` bounds are convenient in this case, particularly given the current implementation limitations around `use<>` bounds, but `use<>` bound are more flexible overall, and so we expect them to be used more often. (As an example, the compiler has a variant of `indices` that returns newtype'd indices `I` instead of `usize` values, and it therefore includes a `use<I>` declaration.)

## Conclusion

This example demonstrates the way that editions can help us to remove complexity from Rust. In Rust 2021, the default rules for when lifetime parameters can be used in `impl Trait` had not aged well. They frequently didn't express what users needed and led to obscure workarounds being required. They led to other inconsistencies, such as between `-> impl Future` and `async fn`, or between the semantics of return-position `impl Trait` in top-level functions and trait functions. 

Thanks to editions, we are able to address that without breaking existing code. With the newer rules coming in Rust 2024,

* most code will "just work" in Rust 2024, avoiding confusing errors;
* for the code where annotations are required, we now have a more powerful annotation mechanism that can let you say exactly what you need to say.

## Appendix: Relevant links

* Precise capture was proposed in [RFC #3617](https://github.com/rust-lang/rfcs/pull/3617), which left an unresolved question regarding syntax, and its tracking issue was [#123432](https://github.com/rust-lang/rust/issues/123432).
* The unresolved syntax question was resolved in [issue #125836](https://github.com/rust-lang/rust/issues/125836), which introduced the `+ use<>` notation used in this post.
* The implementation limitation is tracked in [#130031](https://github.com/rust-lang/rust/issues/130031).
