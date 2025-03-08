+++
layout = "post"
date = 2024-09-23
title = "September Project Goals Update"
author = "Niko Matsakis"
team = "Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>"
+++

The Rust project is currently working towards a [slate of 26 project goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html), with 3 of them designed as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/milestone/2).

## Flagship goals

### Prepare Rust 2024 Edition (tracked in [#117])

[#117]: https://github.com/rust-lang/rust-project-goals/issues/117

The Rust 2024 edition is on track to be stabilized on Nightly by Nov 28 and to reach stable as part of Rust v1.85, to be released Feb 20, 2025.

Over the last month, all the "lang team priority items" have landed and are fully ready for release, including migrations and chapters in the [Nightly version of the edition guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html):

* Changes to support new `impl Trait` defaults ([#117587], [#123432]; [read more here](https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html))
* Changes to support generators ([#123904][]).
* Changes to support stabilizing the "never type (`!`) ([#123748][]).

[#117587]: https://github.com/rust-lang/rust/issues/117587
[#123432]: https://github.com/rust-lang/rust/issues/123432
[#123904]: https://github.com/rust-lang/rust/issues/123904
[#123748]: https://github.com/rust-lang/rust/issues/123748

Overall:

* [13 items](https://github.com/rust-lang/rust/issues?q=label%3AS-tracking-ready-for-edition+is%3Aclosed) are fully ready for Rust 2024.
* [10 items](https://github.com/rust-lang/rust/issues?q=label%3AA-edition-2024%20label%3AC-tracking-issue%20-label%3AS-tracking-ready-for-edition%2CS-tracking-impl-incomplete%20-label%3At-libs%20) are fully implemented but still require documentation.
* [6 items](https://github.com/rust-lang/rust/issues?q=label%3AA-edition-2024%20label%3AC-tracking-issue%20-label%3AS-tracking-ready-for-edition%20label%3AS-tracking-impl-incomplete%20-label%3At-libs%20) still need implementation work.

Keep in mind, there will be items that are currently tracked for the edition that will not make it.  That's OK, and we still plan to ship the edition on time and without those items.

### Async Rust Parity (tracked in [#105])

[#105]: https://github.com/rust-lang/rust-project-goals/issues/105
[#129629]: https://github.com/rust-lang/rust/issues/129629

We are generally on track with our marquee features: 

1. Support for *async closures* is available on Nightly and the lang team arrived at a tentative consensus to keep the existing syntax (written rationale and formal decision are in progress). We issued a [call for testing](https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html) as well which has so far uncovered no issues.
2. Partial support for *return-type notation* is available on Nightly with the remainder under review. 

In addition, *dynamic dispatch for async functions* and *experimental async drop work* both made implementation progress. Async WG reorganization has made no progress. 

[Read the full details on the tracking issue.](https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-2361474377)

### Stabilize features needed by Rust for Linux (tracked in [#116])

[#116]: https://github.com/rust-lang/rust-project-goals/issues/116

We have stabilized [extended `offset_of` syntax](https://github.com/rust-lang/rust/pull/128284) and [agreed to stabilize Pointers to Statics in Constants](https://github.com/rust-lang/rust/issues/128183). Credit to @dingxiangfei2009 for driving these forward. 💜

Implementation work proceeds for [arbitrary self types v2](https://github.com/rust-lang/rust/issues/44874#issuecomment-2314739657), [derive smart](https://github.com/rust-lang/rust/pull/125048) [pointer](https://github.com/rust-lang/rust/pull/129467), and [sanitizer support](https://github.com/rust-lang/rust/pull/128348).

RFL on Rust CI is implemented but still waiting on documented policy. The first breakage was detected (and fixed) in [#129416](https://github.com/rust-lang/rust/pull/129416). This is the mechanism working as intended, although it would also be useful to better define what to do when breakage occurs.

## Selected updates

### Begin resolving cargo-semver-checks blockers for merging into cargo (tracked in [#104])

[#104]: https://github.com/rust-lang/rust-project-goals/issues/104

@obi1kenobi has been working on laying the groundwork to enable manifest linting in their project. They have set up the ability to test how CLI invocations are interpreted internally, and can now snapshot the output of any CLI invocation over a given workspace. They have also designed the expansion of the CLI and the necessary Trustfall schema changes to support manifest linting. As of the latest update, they have a working prototype of manifest querying, which enables SemVer lints such as detecting the accidental removal of features between releases. This work is not blocked on anything, and while there are no immediate opportunities to contribute, they indicate there will be some in future updates.

### Expose experimental LLVM features for automatic differentiation and GPU offloading (tracked in [#109])

[#109]: https://github.com/rust-lang/rust-project-goals/issues/109

@ZuseZ4 has been focusing on automatic differentiation in Rust, with their first two upstreaming PRs for the rustc frontend and backend merged, and a third PR covering changes to rustc_codegen_llvm currently under review. They are especially proud of getting a detailed LLVM-IR reproducer from a Rust developer for an Enzyme core issue, which will help with debugging. On the GPU side, @ZuseZ4 is taking advantage of recent LLVM updates to rustc that enable more GPU/offloading work. @ZuseZ4 also had a talk about "When unsafe code is slow - Automatic Differentiation in Rust" accepted for the upcoming LLVM dev meeting, where they'll present benchmarks and analysis comparing Rust-Enzyme to the C++ Enzyme frontend.

### Extend pubgrub to match cargo's dependency resolution (tracked in [#110])

[#110]: https://github.com/rust-lang/rust-project-goals/issues/110

@Eh2406 has achieved the milestone of having the new PubGrub resolver and the existing Cargo resolver accept each other's solutions for all crate versions on crates.io, which involved fixing many bugs related to optional dependencies. Significant progress has also been made in speeding up the resolution process, with over 30% improvements to the average performance of the new resolver, and important changes to allow the existing Cargo resolver to run in parallel. They have also addressed some corner cases where the existing resolver would not accept certain records, and added a check for cyclic dependencies. The latest updates focus on further performance improvements, with the new resolver now taking around 3 hours to process all of crates.io, down from 4.3 hours previously, and a 27% improvement in verifying lock files for non-pathological cases.

### Optimizing Clippy & linting

@blyxyas has been working on improving Clippy, the Rust linting tool, with a focus on performance. They have completed a medium-sized objective to use ControlFlow in more places, and have integrated a performance-related issue into their project. A performance-focused PR has also been merged, and they are remaking their benchmarking tool (benchv2) to help with ongoing efforts. The main focus has been on resolving rust-lang/rust#125116, which is now all green after some work. Going forward, they are working on moving the declare_clippy_lint macro to a macro_rules implementation, and have one open proposal-level issue with the performance project label. There are currently no blockers to their work.

## Completed goals

The following goals have been completed:

* [Implement "merged doctests" to save doctest time](https://github.com/rust-lang/rust-project-goals/issues/111)

## Stalled or orphaned goals

Several goals appear to have stalled or not received updates:

* [Associated type position impl trait](https://github.com/rust-lang/rust-project-goals/issues/103) made significant progress but was not able to reach a stabilizable state before its owner had to scale back their activity. We expect to revisit this in 2025H1.
* [Make Rustdoc Search easier to learn](https://github.com/rust-lang/rust-project-goals/issues/112) has had no updates.
* [Use annotate-snippets for rustc diagnostic output](https://github.com/rust-lang/rust-project-goals/issues/123) has had no updates.

One goal is still waiting for an owner:

* [User-wide build cache](https://github.com/rust-lang/rust-project-goals/issues/124)

## Conclusion

This is a brief summary of the progress towards our a subset of the 2024 project goals. There is a [lot more information available on the website](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html), including the motivation for each goal, as well as detailed status updates. If you'd like more detail, please do check it out! You can also subscribe to individual tracking issues (or the entire rust-project-goals repo) to get regular updates.

The current set of goals target the second half of 2024 (2024H2). Next month we also expect to begin soliciting goals for the first half of 2025 (2025H1).
