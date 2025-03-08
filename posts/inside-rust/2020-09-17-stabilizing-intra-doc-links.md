---
layout: post
title: "Intra-doc links close to stabilization"
author: Manish Goregaokar and Jynn Nelson
team: the rustdoc team <https://www.rust-lang.org/governance/teams/dev-tools#rustdoc>
---

We're excited to share that intra-doc links are stabilizing soon!

[Intra-doc links] are a feature of `rustdoc` that allow you to link to '[items]' - functions, types, and more - by their name, instead of a hard-coded URL. This lets you have accurate links even if your types are [re-exported in a different module or crate][broken-string-links]. Here is a simple example:

```rust
/// Link to [`f()`]
pub struct S;

pub fn f() {}
```

Intra-doc links have been around for a while, all the way back [since 2017][tracking-issue]! They have been available on `nightly` without flags (and thus, on [docs.rs](https://docs.rs)), so you may be surprised to hear that they weren't yet stable. What's changing now is that they will be available on stable Rust, which also means we are more confident in the implementation and would strongly encourage their use. We recommend that you switch your libraries to use intra-doc links, which will fix broken links for re-exported types and links to different crates. We hope to add support for automating this process with [`cargo fix`] in the future.

## The history of intra-doc links

I (Manish) and [QuietMisdreavus](https://github.com/QuietMisdreavus) started working on them in December 2017. Mozilla had given the whole company a couple weeks off after the release of [Firefox Quantum](https://blog.mozilla.org/blog/2017/11/14/introducing-firefox-quantum/), and I was visiting family in Mumbai. This meant that I had a fair amount of free time, and we were in diametrically opposite timezones. QuietMisdreavus had been working on the feature for a while but was less familiar with rustc's path resolution code, so I decided to help. We ended up pairing for those few weeks: during the day I'd write some code, discuss with QuietMisdreavus in the evening, and then hand it over for her to continue overnight. It was a great experience, pairing in open source can be really fun! This ended up in a [46-commit pull request][intra-pr] with commits from both of us.


Unfortunately, we were not able to stabilize the feature at the time. The main blocker was [cross-crate re-exports], things like the following:

```rust
// Crate `inner`
/// Link to [`f()`]
pub struct S;
pub fn f() {}
```

```rust
// outer crate
pub use inner::S;
```


The way `rustdoc` handles reexports is that it renders the reexport in-situ, parsing and rendering all of the markdown. The issue here is that `rustdoc`, when documenting `outer`, does not have access to the local scope information of `inner::S` and cannot resolve `f()`.

These links were the original motivation for intra-doc links, so if we couldn't get them working, there wasn't much point in stabilizing! They also had the downside that they could [silently break] - the documentation would work when you built it, but any user of your API could re-export your types and cause the links to be broken.

At the time, persisting local scope information so that `rustdoc` invocations on downstream crates could access them would involve a significant amount of work on the compiler. It was work the compiler team wanted to be done anyway, but it was a lot, and neither of us had the bandwidth to do it, so we [filed a bug] and went on our way.




## What changed?

Early in June, I (Jynn) got tired of not being able to use intra-doc links. I started investigating the issue to see if there was a fix. It was marked as [`E-hard`], so I wasn't expecting miracles, but I thought I might at least make a start on it.

It turns out there was a simple problem with the implementation - it assumed
all items were in the current crate! Clearly, that's not always the case. [The fix][resolve-cross-crate] turned out to be easy enough that I could implement it as my first contribution to rustdoc.

_Note from Manish:_ Actually, the distinction between [`DefId`] and [`LocalDefId`] _didn't exist_ when we wrote the feature, and the code would only resolve paths based on the resolver's current internal scope (which can only be within the current crate, since that is the only scope information the resolver had at the time). However, over time the compiler [gained the ability][refactor-resolve] to store and query resolution scopes of dependencies. We never noticed, and continued to believe that there was a large piece of work blocking stabilization.

However, my solution had one small problem: on certain [carefully crafted inputs][macro-in-closure], it would crash:

```rust
#![feature(decl_macro)]
fn main() {
    || {
        macro m() {}
    };
}
```
```
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/jyn/src/rust/src/librustc_hir/definitions.rs:358:9
```

## HirIds and DefIds and trees, oh my!

(If you're not interested in the internals of the Rust compiler, feel free to skip this section.)

The error above came because of a pass called [`everybody_loops`]. A compiler 'pass' is a transformation on the source code, for example [finding items without documentation][missing_docs].
The `everybody_loops` pass turns the above code into:

```rust
fn main() {
    {
        macro m { () => { } }
    }
    loop  { }
}
```

As part of my changes for resolving cross-crate items, I needed to know the first parent module, so I could tell what items were in scope. Note however, that after `everybody_loops` the closure has disappeared! The crash happened because `rustdoc` was trying to access a closure that `rustc` didn't think existed (in compiler jargon, it was turning the `DefId` for the closure, which works across crates, into a `HirId`, which is specific to the current crate but contains a lot more info).

# Why is this hard?

This turned out to be an enormous rabbit hole. `everybody_loops` was [introduced][os-specific-modules] all the way back in 2017 to solve another long-standing issue: `rustdoc` doesn't know how to deal with [conditional compilation]. What it lets rustdoc (and by extension, the standard library) do is ignore type and name errors in function bodies. This allows documenting both Linux and Windows APIs on the same host, even though the implementations would [normally be broken][why-everybody-loops]. As seen above, the way it works is by turning every function body into `loop {}` - this is always valid, because `loop {}` has type `!`, which coerces to any type!

<!--
However there's a problem: [function bodies aren't _always_ opaque][preserve-item-decls].
You can implement traits inside a function:

```rust
pub struct S;
fn f() {
    impl Default for S {
        fn default() -> Self {
            S
        }
    }
}
```

If you replace that trait implementation with a loop, you have a problem.
-->
 As we saw above, though, this transformation broke rustdoc. Additionally, it was causing [lots][type-alias-impl-trait] [of][preserve-item-decls] [other][impl-trait] [problems][derive-macros].

So I got rid of it! This was [Don't run everybody_loops]. It is the single largest PR I've ever made to rustc, and hopefully the largest I will ever make. The issue was that the errors from libstd haven't gone away - if anything, it had been expanded since 2017. The hack I came up with was to, instead of running type checking and trying to rewrite the code into something that was valid, never run type checking in function bodies at all! This is both [less work][perf run] and closer to the semantics rustdoc wants. In particular, it never causes the invalid states that were crashing `rustdoc`.

## Aftermath: No good deed goes unpunished

About a month after the PR was merged, rustdoc got a bug report: the docs for `async-std` failed to build on the nightly channel. Their code looked something like [the following][realistic async]:

```rust
mod windows {
    pub trait WinFoo {
        fn foo(&self) {}
    }
    impl WinFoo for () {}
}

#[cfg(any(windows, doc))]
use windows::*;

mod unix {
    pub trait UnixFoo {
        fn foo(&self) {}
    }
    impl UnixFoo for () {}
}

#[cfg(any(unix, doc))]
use unix::*;

async fn bar() {
    ().foo()
}
```

In particular, notice that under `cfg(doc)`, both traits would be in scope with the same method, so it would be ambiguous which to use for `.foo()`. This is exactly the sort of problem meant to be solved by not running type-checking. Unfortunately, since it was used in an `async fn`, type checking was still being run; `bar` desugars to a closure of the following form:

```rust
fn bar() -> impl Future<Output = ()> {
    async {
        ().foo()
    }
}
```

Because the function returned `impl Future`, that required type-checking the body to infer the return type of the function. That's exactly what `rustdoc` wanted not to do!

The [hacky 'fix'][fix-async-std] implemented was to not infer the type of the function at all - rustdoc doesn't care about the exact type, only the traits that it implements. This was such a hack there's an [issue open to fix it][async-std-issue].

## Stabilizing intra-doc links

Now that cross-crate re-exports work, there isn't much standing in the way of stabilizing intra-doc links! There are a [few][assoc-items] [cleanup][cross-crate-traits] [PRs][mismatched-disambiguator], but for the most part, the path to stabilization seems clear.

In the meantime, I've been working on various improvements to intra-doc links:

- [Resolving associated items][assoc-items-rfc]
- [Fixing][cross-crate-trait-method] [various][primitive-impls] [bugs][pub-re-exports] [in][primitive-consts] [the][primitive-self] implementation
- [Using intra-doc links throughout the standard library][std-links-tracking-issue]
- Detecting more cases when [links are ambiguous][primitive-module-ambiguity]
- [Removing disambiguators][remove-disambiguators] that only distract from the docs
- [Improving the errors messages][improve-suggestions] when a link fails to resolve

In particular, there have been a ton of people who stepped up to help [convert the standard library to intra-doc links][std-links-tracking-issue]. A giant thank you to **@camelid**, **@denisvasilik**, **@poliorcetics**, **@nixphix**, **@EllenNyan**, **@kolfs**, **@LeSeulArtichaut**, **@Amjad50**, and **@GuillaumeGomez** for all their help!

[`javadoc`]: https://www.oracle.com/java/technologies/javase/javadoc-tool.html
[`rustdoc`]: https://doc.rust-lang.org/rustdoc/
[Intra-doc links]: https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html
[items]: https://doc.rust-lang.org/reference/items.html
[broken-string-links]: https://github.com/rust-lang/rust/issues/32129
[tracking-issue]: https://github.com/rust-lang/rust/issues/43466
[cross-crate re-exports]: https://github.com/rust-lang/rust/issues/65983
[silently break]: https://github.com/rust-lang/rust/issues/43466#issuecomment-570100948
[`E-hard`]: https://github.com/rust-lang/rust/labels/E-hard
[resolve-cross-crate]: https://github.com/rust-lang/rust/pull/73101
[macro-in-closure]: https://github.com/rust-lang/rust/issues/71820
[os-specific-modules]: https://github.com/rust-lang/rust/pull/43348
[conditional compilation]: https://github.com/rust-lang/rust/issues/1998
[why-everybody-loops]: https://gist.github.com/jyn514/aee31eb1cc99d012ff674bec7d122b5e
[preserve-item-decls]: https://github.com/rust-lang/rust/pull/53002
[type-alias-impl-trait]: https://github.com/rust-lang/rust/issues/65863
[impl-trait]: https://github.com/rust-lang/rust/pull/43878
[derive-macros]: https://github.com/rust-lang/rust/pull/65252/commits/25cc99fca0650f54828e8ba7ad2bab341b231fcc
[Don't run everybody_loops]: https://github.com/rust-lang/rust/pull/73566
[perf run]: https://perf.rust-lang.org/compare.html?start=6ee1b62c811a6eb68d6db6dfb91f66a49956749b&end=5c9e5df3a097e094641f16dab501ab1c4da10e9f&stat=instructions:u
[realistic async]: https://github.com/rust-lang/rust/blob/b146000e910ccd60bdcde89363cb6aa14ecc0d95/src/test/rustdoc-ui/error-in-impl-trait/realistic-async.rs
[fix-async-std]: https://github.com/rust-lang/rust/pull/75127/
[assoc-items]: https://github.com/rust-lang/rust/pull/74489
[cross-crate-traits]: https://github.com/rust-lang/rust/pull/75176
[mismatched-disambiguator]: https://github.com/rust-lang/rust/pull/75079
[missing_docs]: https://github.com/rust-lang/rust/blob/e539dd65f8ba80837f7477c0547c61514bceb3ad/src/librustc_lint/builtin.rs#L302
[filed a bug]: https://github.com/rust-lang/rust/issues/65983
[intra-pr]: https://github.com/rust-lang/rust/pull/47046/commits
[`DefId`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def_id/struct.DefId.html
[`LocalDefId`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def_id/struct.LocalDefId.html
[refactor-resolve]: https://github.com/rust-lang/rust/pull/63400
[`everybody_loops`]: https://github.com/rust-lang/rust/blob/bd49eec3d76d5894b539a28309c2fe24f915ee94/compiler/rustc_interface/src/util.rs#L583
[async-std-issue]: https://github.com/rust-lang/rust/issues/75100
[assoc-items-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md#linking-to-associated-items
[std-links-tracking-issue]: https://github.com/rust-lang/rust/issues/75080
[cross-crate-trait-method]: https://github.com/rust-lang/rust/pull/75176
[primitive-impls]: https://github.com/rust-lang/rust/pull/75649
[pub-re-exports]: https://github.com/rust-lang/rust/pull/76082
[primitive-consts]: https://github.com/rust-lang/rust/pull/76093
[primitive-self]: https://github.com/rust-lang/rust/pull/76467
[primitive-module-ambiguity]: https://github.com/rust-lang/rust/pull/75815
[remove-disambiguators]: https://github.com/rust-lang/rust/pull/76078
[improve-suggestions]: https://github.com/rust-lang/rust/pull/75756
[`cargo fix`]: https://github.com/rust-lang/rust/issues/75805
