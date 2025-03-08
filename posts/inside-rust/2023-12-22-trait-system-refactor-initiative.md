+++
layout = "post"
title = "Rustc Trait System Refactor Initiative Update: A call for testing"
author = "lcnr"
team = "The Rustc Trait System Refactor Initiative <https://github.com/rust-lang/trait-system-refactor-initiative/>"
+++

It has been nearly half a year since [our last update][prev]. We are reimplementing the trait solver of rustc with the goal of completely replacing the existing systems. This should allow us to fix some long-standing bugs, enable future type system improvements, and reduce compile times. See the previous update for a more detailed introduction. We have continued to make big progress on the new solver, mostly focusing on getting the solver ready for use in coherence. We changed the unstable compiler flag to enable the new solver: you can now use `-Znext-solver=globally` to enable it everywhere and `-Znext-solver=coherence` to enable the new solver only for coherence checking.

The reimplementation of the trait solver is now ready for use in coherence checking, which is responsible for preventing overlapping trait implementations. All known behavior changes from the old solver are intended, and the quality of error messages should match the existing implementation. However, over the last months the handling of non-fatal overflow has emerged as one of the most significant and involved issues.

## Non-fatal overflow

Unlike the existing trait solver, the new solver does not immediately abort compilation when hitting the recursion limit:
```rust
struct Wrapper<T>(T);

trait Overflow {}
impl<T> Overflow for Wrapper<Wrapper<T>> where Wrapper<T>: Overflow {}
impl Overflow for Wrapper<u32> {}

// Checking whether these two implementations overlap
// tries to prove that either `Wrapper<_>: Overflow` or
// `Wrapper<_>: Copy` do not hold.
//
// The existing solver first checks `Wrapper<_>: Overflow`,
// resulting in overflow and aborting compilation.
//
// The new solver does not abort compilation on overflow and
// considers the implementations to be disjoint, given that
// `Wrapper<_>: Copy` does not hold.
trait MayOverlap {}
impl<T: Overflow + Copy> MayOverlap for T {}
impl<T> MayOverlap for Wrapper<T> {}
```

This change is necessary as popular crates, e.g. [typenum](https://github.com/rust-lang/trait-system-refactor-initiative/issues/73), now reach the recursion limit with the new solver and would therefore break if overflow remains fatal. This is caused by [the removal of a heuristic present in the old solver](https://github.com/rust-lang/trait-system-refactor-initiative/issues/56). It is also desirable as the compilation result is otherwise order dependent. This order dependence complicates future changes to the type system, e.g. [an attempt to switch to deferred projection equality in the old solver](https://github.com/rust-lang/rust/pull/96912) also ended up causing an overflow error in typenum, preventing it from getting merged. It is also observable by users, e.g. switching the order of where-clauses in the above example to `T: Copy + Overflow` causes this snippet to compile with the old solver.

The new solver now returns overflow when hitting the recursion limit. However, this change by itself causes the solver to very easily hang due to exponential blowup. We therefore greatly limit the available recursion depth for later goals after encountering overflow and also discard some of the inference constraints from goals resulting in overflow.

It is important to balance the performance of the trait solver with the expressiveness of the trait system. While we're fairly happy with the approach we've settled on for now, getting this right is more art than science. We believe our current approach to be performant in most cases and to allow for further significant performance optimizations in the future. We also expect it to provide the necessary expressiveness to be backwards compatible with the old solver and to behave as expected by users.

## Looking forward and asking for testing

As we believe using the new solver for coherence checking to now be in a stabilization ready state, please try out the new implementation by enabling the unstable `-Znext-solver=coherence` compiler flag. In case you encounter any behavior or performance regressions, diagnostics issues, or even unsoundnesses[^1], please [open an issue on GitHub](https://github.com/rust-lang/rust/issues).

Using the new solver during coherence checking will improve the behavior in some edge-cases, fixing at least one, pretty much unexploitable, [unsoundness](https://github.com/rust-lang/rust/issues/102048). It will also allow us to remove support for "intercrate mode" in the existing solver. However, most of the positive impact from using the new solver[^2] will only apply once it is used in more areas.

We therefore intend to slightly delay the stabilization of its use in coherence to make sure our design choices will not cause complications down the road. Going forward we will refocus our work on enabling the new solver everywhere. By fixing more of the remaining issues with `-Znext-solver=globally`, we should get additional confidence in our approach to overflow handling. We expect to actually stabilize the use of the new solver in coherence in March of 2024 and intend to provide additional learning materials and documentation before then.

In case there are any questions or thoughts about the new solver, please reach out to us on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/364551-t-types.2Ftrait-system-refactor). 

[^1]: i.e. we do not emit an error even though there are overlapping impls

[^2]: see the [introduction of the previous update][prev]

[prev]: https://blog.rust-lang.org/inside-rust/2023/07/17/trait-system-refactor-initiative.html
