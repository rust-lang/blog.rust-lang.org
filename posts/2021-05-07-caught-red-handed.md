---
layout: post
title: "1.52.0, fingerprints and compiler error forensics"
author: Felix Klock
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The Rust teams are always excited to report on new features offered with each release. Sometimes, however, an important change that is not yet "fully baked" gets accidentally included in a release.

There was an instance of this in yesterday's release, 1.52.0, which added a new bit of internal-consistency checking, called "incremental compilation hash verification" (abbreviated `verify-ich`). This check is also called an "unstable fingerprint" check, because the diagnostic it currently prints look [like this](https://github.com/rust-lang/rust/issues/84336):

```
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(4565771098143344972, 7869445775526300234))`,
   right: `Some(Fingerprint(14934403843752251060, 623484215826468126))`: found unstable fingerprints for <massive text describing rustc internals elided>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
```

This internal-consistency check, as stated in the diagnostic, yields an "Internal Compiler Error" (or ICE). In other words, it represents a bug in the internals of the Rust compiler itself. In *this* case, though, the ICE is revealing a bug that 1.) is very likely to predate the 1.52.0 release and 2.) could result in miscompilation if it had not been caught by `verify-ich`.

In other words: If you are seeing the above Internal Compiler Error, you may be tempted to respond by reverting to the 1.51 release. I want to stress that a downgrade is *not* the best response to this problem.

This post is going to:

 1. Explain [what the check does][part1], at a high level,
 2. Explain [how the check is presenting itself][part2] in the Rust 1.52.0 release,
 3. Tell you [what you should do][part3] if you see an unstable fingerprint on your project,
 4. Describe our plans for [how the Rust project will address][part4] the problems discussed here.

[part1]: #what-are-fingerprints-why-are-we-checking-them
[part2]: #how-does-this-show-up
[part3]: #what-should-a-rust-programmer-do-in-response
[part4]: #what-is-the-rust-project-going-to-do-to-fix-this

## What are fingerprints? Why are we checking them?

The Rust compiler has support for "incremental compilation", which has been described in a [2016 blog post][]. When incremental compilation is turned on, the compiler breaks the input source into pieces, and tracks how those input pieces influence the final build product. Then, when the inputs change, it detects this and reuses artifacts from previous builds, striving to expend effort solely on building the parts that need to respond to the changes to the input source code.

[2016 blog post]: https://blog.rust-lang.org/2016/09/08/incremental.html

Fingerprints are part of our architecture for detecting when inputs change. More specifically, a fingerprint (along with some other state to establish context) is a 128-bit value intended to uniquely identify internal values used within the compiler. Some compiler-internal results are stored on disk ("cached") between runs. Fingerprints are used to validate that a newly computed result is unchanged from the cached result. (More details about this are available in the [relevant chapter of the rustc dev guide][rustc-dev-guide-fingerprints].)

[rustc-dev-guide-fingerprints]: https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#checking-query-results-for-changes-hashstable-and-fingerprints

The `verify-ich` check is a safeguard asserting internal inconsistency of the fingerprints.
The compiler stores fingerprints for both cached and uncached values.
Every time we compute an uncached value, we double-check that its newly computed fingerprint against the finger print stored in the cache.
There are multiple ways that a fingerprint mismatch could arise, but they all represent bugs within the Rust compiler itself.

## History of deployment

We [initially added][pr-45867] `verify-ich` as a tool to use when developing rustc itself, back in 2017; it was solely provided via an unstable `-Z` flag, only available to nightly and development builds.

More recently, in March, we encountered a [miscompilation][issue-82920] that led us to [turn on `verify-ich` by default][pr-83007]. The Rust compiler team decided it was better to catch fingerprint problems and abort compilation, rather than allow for potential miscompilations (and subsequent misbehavior) to sneak into Rust programmer's binaries.

[pr-45867]: https://github.com/rust-lang/rust/pull/45867
[issue-82920]: https://github.com/rust-lang/rust/issues/82920
[pr-83007]: https://github.com/rust-lang/rust/pull/83007

When we first turned on `verify-ich` by default, we assumed we would have time to iron out all the known causes of unstable fingerprints. When we realized this assumption was false, we started [making plans][issue-84970] to improve the user-experience, so that the diagnostic issued by the check would do a better job of telling the programmer what to do in response. But we made a mistake: We thought that the switch for this check was not going to be on the Rust stable channel until version 1.53.

[issue-84970]: https://github.com/rust-lang/rust/issues/84970

It turns out `verify-ich` was turned on in version 1.52.0, which was [released yesterday][].

[released yesterday]: /2021/05/06/Rust-1.52.0.html

## How does this show up


Essentially, for some crates, certain sequences of edit-compile cycles will cause `rustc` to hit the "unstable fingerprints" ICE. I showed one example at the start of this blog post.

Another recent example looks [like this](https://github.com/rust-lang/rust/issues/85039):

```
thread 'rustc' panicked at 'found unstable fingerprints for predicates_of(<massive text describing rustc internals elided>)', /rustc/88f19c6dab716c6281af7602e30f413e809c5974/compiler/rustc_query_system/src/query/plumbing.rs:593:5
```

They all arise from inconsistencies when comparing the incremental-compilation cache stored on disk against the values computed during a current `rustc` invocation, which means they all arise from using incremental compilation.

There are three ways that you may have incremental compilation turned on: You may have set the [environment variable][env-vars] `CARGO_INCREMENTAL=1`, or you may have enabled the `build.incremental` [setting in your Cargo.toml][cargo-toml], or you may be building with the `dev` or `test` [profiles][], which default to having incremental compilation enabled.

[env-vars]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads
[cargo-toml]: https://doc.rust-lang.org/cargo/reference/config.html#buildincremental
[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html

If your project has not opted into enabling incremental compilation, then none of the fingerprint issues should affect your release builds.

## What should a Rust programmer do in response

The Internal Compiler Error asks you to report a bug, and if you can do so, we still want that information. We *want* to know about the cases that are failing.

But regardless of whether or not you file a bug, the problem here can be resolved by either:

 1. deleting your incremental compilation cache (e.g. by running `cargo clean`), or
 2. force incremental compilation to be disabled, by setting `CARGO_INCREMENTAL=0` in your environment or `build.incremental` to `false` in the `config.toml`.

We recommend that users of 1.52.0 disable incremental compilation, to avoid running into this problem.

We do *not* recommend that users of 1.52.0 downgrade to an earlier version of Rust in response to this problem. As noted above, there is at least one instance of a silent [miscompilation][issue-82920] caused by incremental compilation that was not caught until we added the fingerprint checking.

## What is the Rust project going to do to fix this

### Short-term plan

Based on the number of bug reports that have already come in, we know we need to do something quickly.

We are going to be issuing a point release, 1.52.1. The point release will improve the diagnostic output from `verify-ich`, so that a programmer can more effectively act in response to the message (i.e., they will be told that the problem is due to the use of incremental compilation, and that they need to delete their cache, at the very least).

We might also make incremental compilation opt-in for *all* Cargo [profiles][]; that is, we may turn incremental compilation off for the  `dev` and `test`, so that they match `release` and `bench`, which already have incremental compilation off. (This remains to be decided.)

We do not plan to disable incremental compilation in its entirety. We believe that incremental compilation is providing value for a large number of Rust programmers, and we want to empower them to continue using it, as long as they are equipped with the tools they need to turn it off if they encounter this issue.

### Long-term plan

The long-term plan is to fix the bugs! Incremental compilation is the only realistic way for the Rust compiler to be able to provide a fast edit-compile-run cycle for all of its programmers, and so we need to address [all of the issues][issue-list] that have been identified thus far via `verify-ich`. (There are 32 such issues as of this writing, though many may be duplicates.)

[issue-list]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+unstable+fingerprints

If you want to come help us do it, we would love for you to [join us][] with the effort!

[join us]: https://www.rust-lang.org/community
