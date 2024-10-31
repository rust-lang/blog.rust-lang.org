---
layout: post
title: "October Project Goals Update"
author: Niko Matsakis
team: Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>
---
The Rust project is currently working towards a [slate of 26 project
goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html), with 3 of them designed as [Flagship
Goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html#flagship-goals). This post provides selected
updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal
are available in its associated [tracking issue on the rust-project-goals
repository](https://github.com/rust-lang/rust-project-goals/milestone/2).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="15" max="37"></progress>
</div>
</div>

The biggest elements of our goal are solving the ["send bound" problem][sbp] via [return-type notation][RTN] (RTN) and adding support for async closures. This month we made progress towards both. For RTN, @compiler-errors extended the return-type notation landed support for using RTN in self-types like `where Self::method(): Send`. He also authored a [blog post with a call for testing](https://blog.rust-lang.org/inside-rust/2024/09/26/rtn-call-for-testing.html) explaining what RTN is and how it works. For async closures, the lang team reached a preliminary consensus on the `async Fn` syntax, with the understanding that it will also include some "async type" syntax. This rationale was documented in [RFC #3710](https://github.com/rust-lang/rfcs/pull/3710), which is now open for feedback. The team held a [design meeting on Oct 23](https://hackmd.io/@rust-lang-team/ryxu3YLx1e) and @nikomatsakis will be updating the RFC with the conclusions.

[sbp]: 

[RTN]: 

We have also been working towards a release of the `dynosaur` crate that enables dynamic dispatch for traits with async functions. This is intended as a transitionary step before we implement true dynamic dispatch. The next steps are to polish the implementation and issue a public call for testing.

With respect to async drop experiments, @nikomatsakis began reviews. It is expected that reviews will continue for some time as this is a large PR.

Finally, no progress has been made towards async WG reorganization. A meeting was scheduled but deferred. @tmandry is currently drafting an initial proposal.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Resolve the biggest blockers to Linux building on stable Rust</strong></a></div>
    <div style="flex: initial;"><progress value="17" max="37"></progress>
</div>
</div>

We have made significant progress on resolving blockers to Linux building on stable. Support for struct fields in the `offset_of!` macro has been stabilized. The final naming for the "derive-smart-pointer" feature has been decided as `#[derive(CoercePointee)]`; @dingxiangfei2009 prepared [PR #131284](https://github.com/rust-lang/rust/pull/131284) for the rename and is working on modifying the rust-for-linux repository to use the new name. Once that is complete, we will be able to stabilize. We decided to stabilize [support for references to statics in constants pointers-refs-to-static feature](https://github.com/rust-lang/rust/issues/128183) and are now awaiting a stabilization PR from @dingxiangfei2009.

RFL is one of the major users of the asm-goto feature (and inline assembly in general) and we have been examining various extensions. @nbdd0121 authored a [hackmd document](https://hackmd.io/@nbdd0121/BJlVrepa0) detailing RFL's experiences and identifying areas for improvement. This led to two immediate action items: making target blocks safe-by-default ([rust-lang/rust#119364](https://github.com/rust-lang/rust/issues/119364)) and extending `const` to support embedded pointers ([rust-lang/rust#128464](https://github.com/rust-lang/rust/issues/128464)).

Finally, we have been finding an increasing number of stabilization requests at the compiler level, and so @wesleywiser and @davidtwco from the compiler team have started attending meetings to create a faster response. One of the results of that collaboration is [RFC #3716], authored by Alice Ryhl, which proposes a method to manage compiler flags that modify the target ABI. Our previous approach has been to create distinct targets for each combination of flags, but the number of flags needed by the kernel make that impractical. Authoring the RFC revealed more such flags than previously recognized, including those that modify LLVM behavior.

[RFC #3716]: https://github.com/rust-lang/rfcs/pull/3716

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/117'><strong>Rust 2024 Edition</strong></a></div>
    <div style="flex: initial;"><progress value="16" max="30"></progress>
</div>
</div>

 The Rust 2024 edition is progressing well and is on track to be released on schedule. The major milestones include preparing to stabilize the edition by November 22, 2024, with the actual stabilization occurring on November 28, 2024. The edition will then be cut to beta on January 3, 2025, followed by an announcement on January 9, 2025, indicating that Rust 2024 is pending release. The final release is scheduled for February 20, 2025.

The priorities for this edition have been to ensure its success without requiring excessive effort from any individual. The team is pleased with the progress, noting that this edition will be the largest since Rust 2015, introducing many new and exciting features. The process has been carefully managed to maintain high standards without the need for high-stress heroics that were common in past editions. Notably, the team has managed to avoid cutting many items from the edition late in the development process, which helps prevent wasted work and burnout.

All priority language items for Rust 2024 have been completed and are ready for release. These include several key issues and enhancements. Additionally, there are three changes to the standard library, two updates to Cargo, and an exciting improvement to `rustdoc` that will significantly speed up doctests.

This edition also introduces a new style edition for `rustfmt`, which includes several formatting changes. The only remaining task is the completion of a PR for one item, after which final quality assurance crater runs will be conducted. Once these are triaged, the nightly beta for Rust 2024 will be announced, and wider testing will be solicited.

Rust 2024 will be stabilized in nightly in late November 2024, cut to beta on January 3, 2025, and officially released on February 20, 2025. More details about the edition items can be found in the [Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html).

## Goals with updates

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/100'><strong>&quot;Stabilizable&quot; prototype for expanded const generics</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>

 * camlid has started working on using the new lowering schema for more than just const parameters, which once done will allow the introduction of a `min_generic_const_args` feature gate.
* compiler-errors has been working on removing the `eval_x` methods on `Const` that do not perform proper normalization and are incompatible with this feature.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/102'><strong>Assemble project goal slate</strong></a></div>
    <div style="flex: initial;"><progress value="4" max="8"></progress>
</div>
</div>

 * Posted the September update.
* Created more automated infrastructure to prepare the October update, utilizing an LLM to summarize updates into one or two sentences for a concise table.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/103'><strong>Associated type position impl trait</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div>

 * No progress has been made on this goal.
* The goal will be closed as consensus indicates stabilization will not be achieved in this period; it will be revisited in the next goal period.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Begin resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>

 * No major updates to report.
* Preparing a talk for next week's EuroRust has taken away most of the free time.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/106'><strong>Const traits</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>

 * Key developments: With the PR for supporting implied super trait bounds landed ([#129499](https://github.com/rust-lang/rust/pull/129499)), the current implementation is mostly complete in that it allows most code that should compile, and should reject all code that shouldn't.
* Further testing is required, with the next steps being improving diagnostics ([#131152](https://github.com/rust-lang/rust/pull/131152)), and fixing more holes before const traits are added back to core.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/108'><strong>Explore sandboxed build scripts</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>

 * A working-in-process pull request is available at <https://github.com/weihanglo/cargo/pull/66>.
* The use of `wasm32-wasip1` as a default sandbox environment is unlikely due to its lack of support for POSIX process spawning, which is essential for various build script use cases.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for automatic differentiation and GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="9"></progress>
</div>
</div>

 * The Autodiff frontend was merged, including over 2k LoC and 30 files, making the remaining diff much smaller.
* The Autodiff middle-end is likely getting a redesign, moving from a library-based to a pass-based approach for LLVM.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/110'><strong>Extend pubgrub to match cargo&#x27;s dependency resolution</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="2"></progress>
</div>
</div>

 * Significant progress was made with contributions by @x-hgg-x, improving the resolver test suite in Cargo to check feature unification against a SAT solver.
* This was followed by porting the test cases that tripped up PubGrub to Cargo's test suite, laying the groundwork to prevent regression on important behaviors when Cargo switches to PubGrub and preparing for fuzzing of features in dependency resolution.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/112'><strong>Make Rustdoc Search easier to learn</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="9"></progress>
</div>
</div>

 * The team is working on a consensus for handling generic parameters, with both PRs currently blocked on this issue.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/113'><strong>Next-generation trait solver</strong></a></div>
    <div style="flex: initial;"><progress value="10" max="13"></progress>
</div>
</div>

 * Attempted stabilization of `-Znext-solver=coherence` was reverted due to a hang in nalgebra, with subsequent fixes improving but not fully resolving performance issues.
* No significant changes to the new solver have been made in the last month.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="2"></progress>
</div>
</div>

 * GnomedDev pushed rust-lang/rust#130553, which replaced an old Clippy infrastructure with a faster one (string matching into symbol matching).
* Inspections into Clippy's type sizes and cache alignment are being started, but nothing fruitful yet.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/115'><strong>Patterns of empty types</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="8"></progress>
</div>
</div>

 * The linting behavior was reverted until an unspecified date.  
* The next steps are to decide on the future of linting and to write the never patterns RFC.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/101'><strong>Provided reasons for yanked crates</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>

 * The PR https://github.com/rust-lang/crates.io/pull/9423 has been merged.
* Work on the frontend feature is in progress.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/118'><strong>Scalable Polonius support on nightly</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div>

 * Key developments in the 'Scalable Polonius support on nightly' project include fixing test failures due to off-by-one errors from old mid-points, and ongoing debugging of test failures with a focus on automating the tracing work.
* Efforts have been made to accept variations of [issue #47680](https://github.com/rust-lang/rust/issues/47680), with potential adjustments to active loans computation and locations of effects. Amanda has been cleaning up placeholders in the work-in-progress [PR #130227](https://github.com/rust-lang/rust/pull/130227).


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/119'><strong>Stabilize cargo-script</strong></a></div>
    <div style="flex: initial;"><progress value="24" max="33"></progress>
</div>
</div>

 * rust-lang/cargo#14404 and rust-lang/cargo#14591 have been addressed.
* Waiting on time to focus on this in a couple of weeks.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Stabilize parallel front end</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>

 * Key developments: Added the cases in the [issue list](https://github.com/rust-lang/rust/labels/WG-compiler-parallel) to the UI test to reproduce the bug or verify the non-reproducibility.
* Blockers: null.
* Help wanted: Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/labels/WG-compiler-parallel) and try to reproduce the issue.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/126'><strong>Survey tools suitability for Std safety verification</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="6"></progress>
</div>
</div>

 * Students from the CMU Practicum Project have started writing function contracts that include safety conditions for some unsafe functions in the core library, and verifying that safe abstractions respect those pre-conditions and are indeed safe.
* Help is needed to write more contracts, integrate new tools, review [pull requests](https://github.com/model-checking/verify-rust-std/pulls), or participate in the [repository discussions](https://github.com/model-checking/verify-rust-std/discussions).


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/123'><strong>Use annotate-snippets for rustc diagnostic output</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="15"></progress>
</div>
</div>

 * Progress has been made in matching `rustc` suggestion output within `annotate-snippets`, with most cases now aligned.
* The focus has been on understanding and adapting different rendering styles for suggestions to fit within `annotate-snippets`.



## Goals without updates

The following goals have not received updates in the last month:

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="7"></progress>
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
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/122'><strong>Testing infra + contributors for a-mir-formality</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/124'><strong>User-wide build cache</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
