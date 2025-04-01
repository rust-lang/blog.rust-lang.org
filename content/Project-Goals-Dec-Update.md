+++
path = "2025/01/23/Project-Goals-Dec-Update"
title = "December Project Goals Update"
authors = ["David Wood and Niko Matsakis"]
aliases = ["2025/01/23/Project-Goals-Dec-Update.html"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Over the last six months, the Rust project has been working towards a [slate of 26 project
goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html), with 3 of them designated
as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2024h2/goals.html#flagship-goals).
This post provides a final update on our progress towards these goals (or, in some cases, lack
thereof). We are currently [finalizing plans for the next round of project goals, which will cover
2025H1](https://rust-lang.github.io/rust-project-goals/2025h1/index.html). The full details for any
particular goal are available in its associated [tracking issue on the rust-project-goals
repository](https://github.com/rust-lang/rust-project-goals/milestone/2).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="26" max="28"></progress>
</div>
</div>

Our big goal for this period was async closures, and we are excited to announce that work there is done! Stable support for async closures [landed on nightly on Dec 12](https://github.com/rust-lang/rust/pull/132706#issuecomment-2540470500) and it will be included in Rust 1.85, which ships on Feb 20. Big kudos to [compiler-errors](https://github.com/compiler-errors) for driving that.

For our other goals, we made progress, but there remains work to be done:

* **Return Type Notation (RTN)** is implemented and we had a [call for experimentation](https://blog.rust-lang.org/inside-rust/2024/09/26/rtn-call-for-testing.html) but it has not yet reached stable. This will be done as part of our 2025H1 goal.
* Async Functions in Traits (and Return Position Impl Trait in Trait) are currently not consided `dyn` compatible. We would eventually like to have first-class `dyn` support, but as an intermediate step we created a procedural macro crate [`dynosaur`](https://crates.io/crates/dynosaur)[^names] that can create wrappers that enable **dynamic dispatch**. We are planning a comprehensive blog post in 2025H1 that shows how to use this crate and lays out the overall plan for async functions in traits.
* Work was done to prototype an **implementation for async drop** but we didn't account for reviewing bandwidth. [nikomatsakis](https://github.com/nikomatsakis) has done initial reads and is working with PR author to get this done in 2025H1. To be clear though the scope of this is an experiment with the goal of uncovering implementation hurdles. There remains significant language design work before this feature would be considered for stabilization (we don't even have an RFC, and there are lots of unknowns remaining).
* We have had fruitful discussions about the trait for **async iteration** but do not have widespread consensus, that's on the docket for 2025H1.

[^names]: As everyone knows, the hardest part of computer-science is naming. I think we rocked this one.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Resolve the biggest blockers to Linux building on stable Rust</strong></a></div>
    <div style="flex: initial;"><progress value="40" max="55"></progress>
</div>
</div>

We largely completed our goal to stabilize the language features used by the Rust for Linux project. In some cases a small amount of work remains. Over the last six months, we...

* stabilized the `offset_of!` macro to get the offset of fields;
* *almost* stabilized the `CoercePointee` trait -- but [discovered that the current implementaton was revealing unstable details](https://github.com/rust-lang/rust/pull/133820#issuecomment-2559379796), which is currently being resolved;
* `asm_goto` stabilization PR and reference updates are up, excluding the "output" feature.
* completed the majority of the work for arbitrary self types, which is being used by RfL and just needs documentation before stabilisation

We also began work on compiler flag stabilization with [RFC 3716](https://github.com/rust-lang/rfcs/pull/3716), which outlines a scheme for stabilizing flags that modify the target ABI.

Big shout-outs to [Ding Xiang Fei](https://github.com/dingxiangfei2009), [Alice Ryhl](https://github.com/Darksonn/),  [Adrian Taylor](https://github.com/adetaylor), and [Gary Guo](https://github.com/nbdd0121) for doing the lion's share of the work here.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/117'><strong>Rust 2024 Edition</strong></a></div>
    <div style="flex: initial;"><progress value="30" max="30"></progress>
</div>
</div>

The final release of Rust 2024 is confirmed for February 20, 2025 as part of Rust 1.85. Rust 1.85 is currently in beta. Feedback from the nightly beta and crater runs has been actively addressed, with adjustments to migrations and documentation to enhance user experience. 

Big shout-outs to [TC](https://github.com/traviscross) and  [Eric Huss](https://github.com/ehuss/) for their hard work driving this program forward.

## Final goal updates

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/100'><strong>&quot;Stabilizable&quot; prototype for expanded const generics</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="5"></progress>
</div>
</div>

Over the last six months a number of internal refactorings have taken place that are necessary to support a `min_generic_const_args` prototype.

One refactoring is that we have changed how we represent const arguments in the compiler to allow for adding a separate representation for the kinds of const arguments that `min_generic_const_args` will add.

Another big refactoring is that we have changed the API surface for our representation of const arguments in the type system layer, there is no longer a way to evaluate a const argument without going through our general purpose type system logic. This was necessary to ensure that we correctly handle equality of the kinds of const arguments that `min_generic_const_args` will support.

With all of these pre-requisite refactorings completed, a feature gate has been added to the compiler (`feature(min_generic_const_args)`) that uses the new internal representation of const arguments. We are now beginning to implement the actual language changes under this feature gate.

Shout-out to [camelid](https://github.com/camelid), [boxy](https://github.com/boxy) and [compiler-errors](https://github.com/compiler-errors).


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Begin resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="4"></progress>
</div>
</div>

Over the course of the last six months...

* cargo semver-checks began to include generic parameters and bounds in its schema, allowing for more precise lints;
* cargo manifest linting was implemented and merged, allowing for lints that look at the cargo manifest;
* building on cargo manifest linting, the `feature_missing` lint was added, which identifies breakage caused by the removal of a package feature.

In addition, we fleshed out a design sketch for the changes in rustdoc's JSON support that are needed to support cross-crate item linting. This in turn requires compiler extensions to supply that information to rustdoc.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/106'><strong>Const traits</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>

* Progress was made on adding const traits and implementation in the compiler, with improvements being carefully considered. `Add` was constified in [rust#133237](https://github.com/rust-lang/rust/pull/133237) and `Deref`/`DerefMut` in [rust#133260](https://github.com/rust-lang/rust/pull/133260).
* Further progress was made on implementing stability for the const traits feature in [rust#132823](https://github.com/rust-lang/rust/pull/132823) and [rust#133999](https://github.com/rust-lang/rust/pull/133999), with additional PRs constifying more traits open at [rust#133995](https://github.com/rust-lang/rust/pull/133995) and [rust#134628](https://github.com/rust-lang/rust/pull/134628).


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="6"></progress>
</div>
</div>

* Over the last six months, we created a lang-team experiment devoted to this issue and [spastorino](https://github.com/spastorino) began work on an experimental implementation. [joshtriplett](https://github.com/joshtriplett) authored [RFC 3680](https://github.com/rust-lang/rfcs/pull/3680), which has received substantial feedback. The current work is focused on identifying "cheaply cloneable" types and making it easy to create closures that clone them instead of moving them.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/108'><strong>Explore sandboxed build scripts</strong></a></div>
    <div style="flex: initial;"><progress value="4" max="10"></progress>
</div>
</div>

* Alternatives to sandboxed build scripts are going to be investigated instead of continuing this project goal into 2025h1 - namely, declaratively configuring system dependencies with [`system-deps`](https://crates.io/crates/system-deps), using an approach similar to code-checker [Cackle](https://crates.io/crates/cargo-acl) and its sandbox environment [Bubblewrap](https://github.com/containers/bubblewrap), or fully-sandboxed build environments like Docker or Nix.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/110'><strong>Extend pubgrub to match cargo&#x27;s dependency resolution</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="2"></progress>
</div>
</div>

* Significant speedups have been achieved, reducing the slowest crate resolution time from over 120 seconds to 11 seconds, and decreasing the time to check all crates from 178 minutes to 71.42 minutes.
* Performance improvements have been made to both the existing resolver and the new implementation, with the lock file verification time for all crates reduced from 44.90 minutes to 32.77 minutes (excluding some of the hardest cases).


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/112'><strong>Make Rustdoc Search easier to learn</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>

* Our pull request adding example searches and adding a search button has been added to the agenda for the rustdoc team next meeting.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/113'><strong>Next-generation trait solver</strong></a></div>
    <div style="flex: initial;"><progress value="10" max="13"></progress>
</div>
</div>

* The `-Znext-solver=coherence` stabilization is now stable in version 1.84, with a new update blogpost published.
* Significant progress was made on bootstrap with `-Znext-solver=globally`. We're now able to compile rustc and cargo, enabling try-builds and perf runs.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="2"></progress>
</div>
</div>

* An optimisation for the `#[clippy::msrv]` lint is open, benchmarked, and currently under review.
* Help is needed on any issue marked with `performance-project`, especially on issue #13714.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/115'><strong>Patterns of empty types</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="8"></progress>
</div>
</div>

* Over the course of this goal, Nadrieril wrote and posted the never patterns RFC as an attempt to make progress without figuring out the whole picture, and the general feedback was "we want to see the whole picture". Next step will be to write up an RFC that includes a clear proposal for which empty patterns can and cannot be omitted. This is 100% bottlenecked on my own writing bandwidth (reach out if you want to help!). Work will continue but the goal won't be resubmitted for 2025h1.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/118'><strong>Scalable Polonius support on nightly</strong></a></div>
    <div style="flex: initial;"><progress value="7" max="18"></progress>
</div>
</div>

* [Amanda](https://github.com/amandasystems) has made progress on removing placeholders, focusing on lazy constraints and early error reporting, as well as investigating issues with rewriting type tests; a few tests are still failing, and it seems error reporting and diagnostics will be hard to keep exactly as today.
* [@lqd](https://github.com/lqd) has opened PRs to land the prototype of the location-sensitive analysis. It's working well enough that it's worthwhile to land; there is still a lot of work left to do, but it's a major milestone, which we hoped to achieve with this project goal.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/119'><strong>Stabilize cargo-script</strong></a></div>
    <div style="flex: initial;"><progress value="28" max="36"></progress>
</div>
</div>

* A fix stopping cargo-script from overriding the release profile was posted and merged.
* Help is wanted for writing frontmatter support in rustc, as rustfmt folks are requesting it to be represented in the AST.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/120'><strong>Stabilize doc_cfg</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="3"></progress>
</div>
</div>

 * RFC is done, waiting for all rustdoc team members to take a look before implementation can start.

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Stabilize parallel front end</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>

* [SparrowLii](https://github.com/SparrowLii) proposed a 2025H1 project goal to continue stabilizing the parallel front end, focusing on solving reproducible deadlock issues and improving parallel compilation performance.
* The team discussed solutions to avoid potential deadlocks, finding that disabling work-stealing in rayon's subloops is effective, and will incorporate related modifications in a PR.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/123'><strong>Use annotate-snippets for rustc diagnostic output</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="15"></progress>
</div>
</div>

* Progress on `annotate-snippets` continued despite a busy schedule, with a focus on improving suggestions and addressing architectural challenges.
* A new API was designed in collaboration with [epage](https://github.com/epage), aiming to align `annotate-snippets` more closely with `rustc` for easier contribution and integration.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/102'><strong>Assemble project goal slate</strong></a></div>
    <div style="flex: initial;"><progress value="7" max="8"></progress>
</div>
</div>

* The project goal slate for 2025h1 has been posted [as an RFC](https://github.com/rust-lang/rfcs/pull/3764#issuecomment-2593395122) and is waiting on approval from project team leads.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for automatic differentiation and GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="9"></progress>
</div>
</div>

* Another pull request was merged with only one remaining until a working MVP is available on nightly.
* Some features were removed to simplify upstreaming and will be added back as single PRs.
* Will start work on `batching` feature of LLVM/Enzyme which allows Array of Struct and Struct of Array vectorisation.
* There's been a push to add a AMD GPU target to the compiler which would have been needed for the LLVM offload project.


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/126'><strong>Survey tools suitability for Std safety verification</strong></a></div>
    <div style="flex: initial;"><progress value="4" max="6"></progress>
</div>
</div>

* We have written and verified around 220 safety contracts in the verify-rust-std fork.
* 3 out of 14 challenges have been solved.
* We have successfully integrated Kani in the repository CI, and we are working on the integration of 2 other verification tools: VeriFast and Goto-transcoder (ESBMC)


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/122'><strong>Testing infra + contributors for a-mir-formality</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div>

* There wasn't any progress on this goal, but building a community around a-mir-formality is still a goal and future plans are coming.

## Goals without updates

The following goals have not received updates in the last month:


</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/103'><strong>Associated type position impl trait</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/111'><strong>Implement &quot;merged doctests&quot; to save doctest time</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Completed"></img>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/101'><strong>Provided reasons for yanked crates</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div><div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/124'><strong>User-wide build cache</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Will%20not%20complete%20%3A%28-yellow" alt="Will not complete"></img>
</div>
</div>
