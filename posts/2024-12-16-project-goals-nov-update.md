+++
layout = "post"
title = "November project goals update"
author = "Niko Matsakis"
team = "Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>"
+++
The Rust project is currently working towards a [slate of 26 project
goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html), with 3 of them designed as [Flagship
Goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html#flagship-goals). This post provides selected
updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal
are available in its associated [tracking issue on the rust-project-goals
repository](https://github.com/rust-lang/rust-project-goals/milestone/2).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="21" max="40"></progress>
</div>
</div>

Async closure stabilization has [been approved](https://github.com/rust-lang/rust/pull/132706), though the stabilization has not yet landed! The lang team ultimately opted to stabilize the trait name `AsyncFn` rather than the keyword-based `async Fn` syntax that was originally proposed. This decision came after discussion on the Flavors RFC which made it clear we were not at a consensus about whether the `async Trait` keyword would be used more generally or not. Given that, the team felt that the `AsyncFn` synta was a fine "next step". If we do ultimately adopt some form of `async Trait` keyword syntax, then `AsyncFn` can become a trait alias.

Regarding return-type notation, an extension of return-type notation to cover Self::foo(..): Send [landed](https://github.com/rust-lang/rust/pull/129629) and we landed [#132047](https://github.com/rust-lang/rust/issues/132047) which fixes a known ICE. Stabilization PR is now unblocked.

No major progress towards async drop reviews or team reorganization.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Resolve the biggest blockers to Linux building on stable Rust</strong></a></div>
    <div style="flex: initial;"><progress value="29" max="55"></progress>
</div>
</div>

This month saw steady progress on our checklist. dingxiangfei2009's PR renaming `derive(SmartPointer)` to `derive(CoercePointee)` was merged and he began the work to port the RFL codebase to use the new name. Alice Ryhl opened [RFC #3716](https://github.com/rust-lang/rfcs/pull/3716) proposing a way to manage compiler flags that alter the ABI and discussion (and some implementation work) has ensued. Finally, we landed [PR #119364](https://github.com/rust-lang/rust/issues/119364) making target blocks in asm-goto safe by default; this was based directly on experience from RFL which showed that [safe would be more useful]. We are still working to finalize another extension to asm-goto that arose from RFL requirements, [allowing `const` to support embedded pointers](https://github.com/rust-lang/rust/issues/128464). Finally we prepared [reference PR #1610](https://github.com/rust-lang/reference/issues/1610) describing the change to permit [Pointers to Statics in Constants](https://github.com/rust-lang/rust/issues/119618) that was stabilized last month.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/117'><strong>Rust 2024 Edition</strong></a></div>
    <div style="flex: initial;"><progress value="22" max="30"></progress>
</div>
</div>

Rust 2024 has now entered the nightly beta and is expected to stabilize as part of Rust 1.85 on 2025-02-20.  It has a great many improvements that make the language more consistent and ergonomic, that further upon our relentless commitment to safety, and that will open the door to long-awaited features such as gen blocks, let chains, and the never type `!`. For more on the changes, see the [nightly Edition Guide.](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html) The [call for testing blog post](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html) contains more information and instructions on how you can try it yourself.

## Goals with updates

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/100'><strong>&quot;Stabilizable&quot; prototype for expanded const generics</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>

 * `min_generic_const_args` now exists as a feature gate, though without any functionality, only some gated refactorings, but shouldn't be long before it has actual functionality behind it.
* The refactoring to remove all the `eval_x` methods on `ty::Const` has been completed, making it possible to correctly implement normalization for constants.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/102'><strong>Assemble project goal slate</strong></a></div>
    <div style="flex: initial;"><progress value="6" max="8"></progress>
</div>
</div>

* Posted the October update.
* Created more automated infrastructure to prepare the October update, making use of an LLM to summarize updates into one or two sentences for a concise table.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Begin resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>

* Support for cargo manifest linting is [now merged](https://github.com/obi1kenobi/cargo-semver-checks/pull/1007), making it possible to catch breakage caused by _manifest_ (`Cargo.toml`) changes, not just source code changes. An example of such breakage is the removal of a package feature: any crates that enabled the removed feature will no longer build.
* Partial schema design and implementation of type information in lints, enabling the creation of breaking-change lints and improving diagnostic quality for a subset of type-related breaking changes.
* Resolved multi-team questions that were blocking cross-crate checking, with the compiler team MCP merged and rustdoc improvements discussed and agreed upon.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/106'><strong>Const traits</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>

* The way const traits are desugared was completely restructured, making the design easier to understand and more robust against current unit tests.
* Significant development and cleanup for the feature has been done, with several pull requests merged and two still open, bringing the feature closer to being able to dogfood on the standard library and closer to stabilization.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="6"></progress>
</div>
</div>

* @joshtriplett opened https://github.com/rust-lang/rfcs/pull/3680. The @rust-lang/lang team has not yet truly discussed or reached a decision on that RFC.
* @spastorino began implementation work on a prototype.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/108'><strong>Explore sandboxed build scripts</strong></a></div>
    <div style="flex: initial;"><progress value="4" max="10"></progress>
</div>
</div>

* The sandboxed build scripts exploration is complete. We are unlikely to continue this work in next year but the research may be useful in other areas, such as the possible addition of POSIX process support to WASI or a declarative system dependency configuration in Cargo.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for automatic differentiation and GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="9"></progress>
</div>
</div>

* The re-design of the autodiff middle/backend was implemented, reducing the remaining LoC to be upstreamed from 2.5k to 1.1k, split into two PRs ([1](https://github.com/rust-lang/rust/pull/133429) and [2](https://github.com/rust-lang/rust/pull/130060)), which received initial feedback and are expected to land in early December.
* The preprint of the first paper utilizing `std::autodiff` is [available on Arxiv](https://arxiv.org/abs/2411.17011v1), with code available at [ChemAI-Lab/molpipx](https://github.com/ChemAI-Lab/molpipx/), showcasing significantly faster compilation times in Rust compared to JAX.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/110'><strong>Extend pubgrub to match cargo&#x27;s dependency resolution</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="2"></progress>
</div>
</div>

* The core data structures of PubGrub have been published as a separate `version-ranges` crate, enabling multiple projects to share this core abstraction and benefit from improvements without waiting for the rest of the project.
* This is one of many steps required to publish a new `0.3.0` version of the PubGrub crate.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/112'><strong>Make Rustdoc Search easier to learn</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>

* Rustdoc will now show type signatures in the search results page, and the boxing transform behaves more like Hoogle's does.
* Improvements to matching behavior have been made to fit user expectations.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/113'><strong>Next-generation trait solver</strong></a></div>
    <div style="flex: initial;"><progress value="10" max="13"></progress>
</div>
</div>

* We stabilized `-Znext-solver=coherence`  again in https://github.com/rust-lang/rust/pull/130654. It's looking like the stabilization will actually go through this time.
* We're currently refactoring the way the current "typing mode" is tracked, working to fix [trait-system-refactoring#106](https://github.com/rust-lang/trait-system-refactor-initiative/issues/106). An [FCP was started](https://github.com/rust-lang/rust/pull/132325) to clean up the way we merge candidates when proving trait goals. 

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="2"></progress>
</div>
</div>

* rust-lang/rust#125116 has been merged, marking half of the goal as formally completed.
* Discussions on [using `cargo cache` on CI](https://github.com/rust-lang/rust-clippy/issues/13033#issuecomment-2501279515) are beginning to take form.
* rust-lang/rust#125116 may be contested in results. The impact may not be as large as expected, even on Clippy.
* We've been experimenting with Clippy using `rustc_driver` as a static library, instead of dynamic linking. This would be us both a way to check the performance impact of `rustc_driver` as a shared library, **and** a way to profile Clippy without filtering between `dl_*` calls.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/115'><strong>Patterns of empty types</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="8"></progress>
</div>
</div>

* The [never patterns RFC](https://github.com/rust-lang/rfcs/pull/3719) was posted.
* Feedback on the RFC suggests that the question of "which arms can be omitted" isn't as orthogonal as hoped, so the focus will switch to that.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/101'><strong>Provided reasons for yanked crates</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>

* The PR https://github.com/rust-lang/crates.io/pull/9423 has been merged.
* Work is ongoing on the frontend feature.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/118'><strong>Scalable Polonius support on nightly</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>

* Amanda's EuroRust talk on polonius from last month is also now [available on YouTube](https://www.youtube.com/watch?v=uCN_LRcswts&feature=youtu.be).
* Implementation work continues, mostly on a branch. Major developments include a new debugger which has accelerated progress. There are about 70 test failures left to be analyzed.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/119'><strong>Stabilize cargo-script</strong></a></div>
    <div style="flex: initial;"><progress value="27" max="35"></progress>
</div>
</div>

* rust-lang/cargo#14670 and rust-lang/cargo#14749 have been posted and merged.
* rust-lang/cargo#14792 has been posted.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Stabilize parallel front end</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>

* Still in the process of determining the cause of the deadlock through local testing and compiler code analysis.
* **Help wanted:** Try to reproduce deadlocks described in the [issue list](https://github.com/rust-lang/rust/labels/WG-compiler-parallel).

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/126'><strong>Survey tools suitability for Std safety verification</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="6"></progress>
</div>
</div>

* A [new partnership](https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries/) between the Rust Foundation and AWS will help fund this effort. The [verification challenges](https://model-checking.github.io/verify-rust-std/challenges.html) in the [verify-rust-std fork](https://github.com/model-checking/verify-rust-std) now have financial rewards for those completing them.
* **Help wanted:** Help needed to write more contracts, to integrate new tools, to review [pull requests](https://github.com/model-checking/verify-rust-std/pulls) or to participate in the [repository discussions](https://github.com/model-checking/verify-rust-std/discussions).

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/122'><strong>Testing infra + contributors for a-mir-formality</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div>

* We decided to close this goal as we have not been making steady progress. We are evaluating what to propose the 2025h1 round of goals.

## Goals without updates

The following goals have not received updates in the last month:

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/103'><strong>Associated type position impl trait</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/111'><strong>Implement &quot;merged doctests&quot; to save doctest time</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Completed"></img>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/120'><strong>Stabilize doc_cfg</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/123'><strong>Use annotate-snippets for rustc diagnostic output</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="15"></progress>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/124'><strong>User-wide build cache</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
