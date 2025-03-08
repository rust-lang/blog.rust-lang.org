+++
layout = "post"
title = "Rustc Trait System Refactor Initiative Update"
author = "lcnr"
team = "The Rustc Trait System Refactor Initiative <https://github.com/rust-lang/trait-system-refactor-initiative/>"
+++

As announced in the [Types Team announcement post](https://blog.rust-lang.org/2023/01/20/types-announcement.html) at the start of this year, the Types Team has started to reimplement the trait solver of rustc. This refactor is similar to [Chalk](https://github.com/rust-lang/chalk/), but directly integrated into the existing codebase using the experience gathered over the last few years. Unlike Chalk, the new trait solver has the sole goal of replacing the existing implementation. We are separately working on formalizing the type system in [a-mir-formality](https://github.com/rust-lang/a-mir-formality). It has now been half a year since that announcement which matches the first step of [our roadmap][roadmap].

By reimplementing the trait solver of rustc we are able to fix many subtle bugs and inefficiencies of the existing implementation. This should result in faster compilation speed and fewer bugs. The new trait solver implementation should also unblock many future changes, most notably around implied bounds and coinduction. For example, it will allow us to remove many of the [current restrictions on GATs](https://github.com/rust-lang/rust/issues/91693) and to fix many long-standing unsound issues, like [#25860](https://github.com/rust-lang/rust/issues/25860). Some unsound issues will already be fixed at the point of stabilization while others will require additional work afterwards. The new solver will also enable or greatly simplify other, still experimental type system extensions, e.g. [generic const expressions](https://github.com/rust-lang/rust/issues/76560) and [non-lifetime binders](https://github.com/rust-lang/rust/issues/108185).

## The status quo

The new trait solver implementation can be tested on nightly by using the rustc flag `-Ztrait-solver=next`. To use the new implementation for only coherence checking, `-Ztrait-solver=next-coherence` can be used. See the current implementation progress of the new trait solver in [its tracking issue](https://github.com/rust-lang/rust/issues/107374).

We are now at a point where we exclusively rely on the new implementation when the feature flag is enabled. This is a major step as we've previously still relied on the old solver for ["deep normalization"](https://github.com/rust-lang/rust/pull/113086) and ["selection"](https://github.com/rust-lang/rust/pull/112869). When using the new trait solver many crates and most of our existing regression tests successfully compile.

While there is a significant tail of less common bugs, we currently have two main failure causes:

First, the new solver has a slightly different approach for `impl Trait`. The implementation of which is still broken for instances of nested return position impl trait, e.g. `fn foo() -> impl Trait<Assoc = impl Sized>`. Working on this new approach helped us discover issues of the existing implementation, which allows us to refine its design before the stabilization of the `type_alias_impl_trait` feature.

Second, the inference of implicit `Unsize` coercions, e.g. converting `Box<String>` to `Box<dyn Display>`, relies on implementation details of the existing trait solver. We've recently started to emulate the existing behavior here and should fix most of the remaining breakage from that over the next few weeks.

## Going forward

After fixing the currently open issues, we intend to move parts of rustc to the new trait solver implementation in steps, starting by using it in coherence. We expect to move coherence checking to the new implementation at the end of this year. Moving the type checking of functions to the new trait solver implementation will be a lot more challenging. This will be the last place where we will use the old implementation. We expect to change the default there in 2024, potentially relying on the new edition to help with migration.

A major challenge will be "incompleteness". We use incompleteness as a technical term for cases where the type system unnecessarily guides type inference. Incompleteness allows otherwise ambiguous code to compile, but it also makes the trait system order dependent and can result in incorrect and weird errors. Consider the following example:
```rust
fn impl_trait() -> impl Into<u32> {
    0u16
}

fn main() {
    // There are two possible types for `x`:
    // - `u32` by using the "alias bound" of `impl Into<u32>`
    // - `impl Into<u32>`, i.e. `u16`, by using `impl<T> From<T> for T`
    //
    // We infer the type of `x` to be `u32` even though this is not
    // strictly necessary and can even lead to surprising errors.
    let x = impl_trait().into();
    println!("{}", std::mem::size_of_val(&x));
}
```
How and when we make such inference jumps is quite closely tied to the old trait solver implementation and not something we want to, or even can, copy directly. We have to figure out how to mostly maintain the existing behavior while fitting the rules to the new implementation. The complexity of this issue will only become apparent once the bigger issues are fixed and we start running crater with the new implementation enabled.

Another major hurdle will be error diagnostics. We currently rely on many internal details of the old trait solver implementation to emit actionable and useful errors to the user. These diagnostics have been incrementally improved by relying and working around these internal details. We cannot simply reuse them with the new trait solver implementation. Our goal there is to instead optionally emit "proof trees", an in-memory representation of how trait solving occurred. We intend provide an simplified API in top of these proof trees which will then be used for diagnostics.

[roadmap]: https://blog.rust-lang.org/2023/01/20/types-announcement.html#roadmap
