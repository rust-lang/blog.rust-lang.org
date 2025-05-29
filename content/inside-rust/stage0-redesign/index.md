+++
path = "inside-rust/2025/05/29/redesigning-the-initial-bootstrap-sequence"
title = "Redesigning the Initial Bootstrap Sequence"
authors = ["Jieyou Xu"]

[extra]
team = "the Bootstrap team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

This blog post accompanies an [upcoming major change to the `rust-lang/rust` build system][stage0-redesign-pr] (see also [Major Change Proposal 619][redesign-stage0-mcp]). This will have no impact on the distributed artifacts from [rust-lang/rust], but the way we build those artifacts is changing.

This blog post focuses on motivation for the change and attempts to build a mental model for how the system works, rather than deep diving on workflow changes. See the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html) for details on how you might need to change your workflow with these changes. Note that rustc-dev-guide pages will have updated content once the implementation PR merges.

# TL;DR: What is being changed?

We are [redesigning the bootstrap sequence for the Rust toolchain][stage0-redesign-pr]. The standard library will move from supporting being built by both the previous toolchain version and the current version to only supporting the current version. This does not change the artifacts we distribute to end users of Rust.

**Current bootstrap sequence**

![Current stage 0 bootstrap sequence](./stage0-current.svg)

**Redesigned  bootstrap sequence**

![Redesigned stage 0 bootstrap sequence](./stage0-next.svg)

The following section is a quick primer on the concept of bootstrapping and the terminology we use in this blog post.

# A quick primer on bootstrapping and terminology used in this blog post {#preliminary}

This section is intended to explain some basic bootstrapping concepts to make it easier for contributors to understand the upcoming bootstrap changes. You can [skip this section](#motivation) if you are already familiar with [the `bootstrap` build system][bootstrap] itself.

## What is bootstrapping? {#intro-to-bootstrapping}

[*Bootstrapping*][compiler-bootstrapping] is the process of producing a newer version of a compiler with an older version of itself. Usually, bootstrap processes involve the concept of *stages*, where a compiler and associated artifacts from a stage is used to build the compiler of the next stage. For example, a stage 2 compiler is built from a stage 1 compiler and associated stage 1 artifacts. The stage number is named after the compiler in question.

## Bootstrapping `rustc` {#bootstrapping-rust}

Building a C++ compiler toolchain like [`gcc`][gcc-bootstrap] or [`clang`][clang-bootstrap] doesn't usually involving building their standard libraries from source, but instead will link against a pre-built standard library across all bootstrap stages. Typically, this specified standard library is the system standard library.[^adapted]

The Rust toolchain as a whole, both currently and after this change, supports being built with just two minor versions of Rust: the previous version and its own version. For example, Rust 1.85.1 requires one of 1.84.0, 1.85.0, or 1.85.1 toolchains to build its source code into the 1.85.1 distributed artifacts.

Rust has historically always built the majority of in-tree source code against the in-tree standard library's API. This meant that the in-tree copy of the standard library had to support being built with both the previous release (e.g., 1.84) and the new release (e.g., 1.85), requiring extensive `cfg(bootstrap)` annotations to be compatible with the unstable API of the compiler (primarily changes in intrinsics and lang items).

The choice to require std to be built with two different compiler toolchains is largely historical. We don't have specific rationale available, but expect it to have made more sense when the standard library API was rapidly evolving and the compiler was unable to use crates.io dependencies for functionality.

## Terminology {#terminology}

![Naive bootstrap stage](./naive-bootstrap-stage.svg)

Let us call these *intra*-stage steps of stage `N - 1` compiler building stage `N - 1` standard library a **bootstrap sequence**, i.e. the sequence of intra-stage steps involved in producing stage `N - 1` artifacts required to build the stage `N` compiler.

![Bootstrap sequence](./naive-bootstrap-stage-bootstrap-sequence.svg)

We must also have a base case, a starting compiler, to build newer compilers with. Indeed, the *initial* compiler is also called the *stage 0 compiler*. The *initial* bootstrap stage is called *stage 0*.

# Motivation {#motivation}

But the naive model we presented above isn't complete.

*Rust* has elected a design choice where the compiler, `rustc`, and the standard library ("std") are tightly coupled. *Intrinsics* and *lang items* form a broad interface between the compiler and the standard library. When intrinsics or lang items are modified, both sides need to be adjusted.

Currently, the standard library currently must support being built with two different compilers, the in-tree compiler and the initial stage 0 compiler[^initial-compiler]. All such changes to intrinsics and lang items thus need to use `cfg(bootstrap)` to gate code that can be built by the in-tree compiler vs the stage 0 compiler. This causes a lot of churn for contributors wanting to introduce, modify or remove intrinsics and lang items (particularly when creating new releases).

The [stage 0 bootstrap sequence redesign][stage0-redesign-pr] aims to mitigate such churn and implementation complexity in the standard library by having the standard library only support *one* version of the compiler.

To better understand this redesign, we will:

1. [Explain how the current stage 0 bootstrapping sequence works](#current-model), and
2. [Explain how the new stage 0 bootstrapping sequence works after the redesign](#new-model), and
3. [Explain the benefits of the redesigned stage 0 bootstrapping sequence is more preferable](#better).

# The current stage 0 bootstrap sequence {#current-model}

![Current stage 0 bootstrap sequence](./stage0-current.svg)

Currently, [bootstrap] downloads a pre-built beta rustc as the initial compiler (stage 0 rustc).

- To produce a stage 1 rustc, we need to produce a stage 0 std. This stage 0 std is built from in-tree standard library sources with `cfg(bootstrap)` active.
- To produce a stage 2 rustc, we need to produce a stage 1 std. However, this is where things get weird. The stage 1 std is built from the *same* in-tree standard library sources (but with `cfg(bootstrap)` inactive). This is what we meant by "the standard library has to support being buildable by two compiler versions."

# The redesigned stage 0 bootstrap sequence {#new-model}

![Redesigned stage 0 bootstrap sequence](./stage0-next.svg)

In the [redesigned stage 0 bootstrapping sequence][stage0-redesign-pr] we instead download *both* the pre-built beta rustc as the stage 0 compiler, and the pre-built beta std as the stage 0 std, *instead* of building stage 0 std from in-tree sources.

- When producing a stage 1 rustc, we already have the stage 0 std (as it is the pre-built std).
- When producing a stage 2 rustc, the stage 1 std is then the std built from in-tree std sources with the stage 1 rustc.

# Why is the redesigned stage 0 bootstrap sequence better? {#better}

There are several benefits of the redesigned stage 0 bootstrap sequence:

1. We no longer have to use `cfg(bootstrap)` in the standard library sources for intrinsics and lang items to distinguish when being built by the beta rustc vs the in-tree rustc, because the standard library now only has to be buildable by exactly one compiler version (the current stage rustc).
2. Rebasing no longer has to rebuild everything. As the standard library is now built *after* the compiler instead of before, all dependencies and compiler crates that are unmodified can be cached when switching `git` branches. It's only necessarily to rebuild everything after bootstrap bumps every 6 weeks as part of the release cycle.
3. Modifying the standard library no longer has to rebuild everything. After the redesign, it is now possible to add documentation comments to functions in the standard library without having to rebuild the compiler and then also rebuild std a second time. `--keep-stage-std=0` is no longer needed.
4. It aligns better with how other rust programs are built. We no longer have a "strange" setup where the stage 1 compiler was built from a *beta* rustc with an *in-tree* std. Now, the stage 1 compiler is built from a beta rustc and a beta std.

# In terms of bootstrap invocations and bootstrap config, what does this redesign mean?

The minimum stage to check, build and test the standard library is now stage 1. `./x {check,build,test} library --stage=0` are now no-ops; switch to `--stage 1` instead. `--keep-stage-std=0` is a no-op.

For `profile = "library"` users, like aforementioned, the default check, build, and test stage are now bumped to 1. `download-rustc = "if-unchanged"` is enabled by default, which downloads a pre-built CI rustc instead of building the compiler if there are no compiler changes, allowing you to build the standard library without building the compiler.

# What does this mean for contributors working on the standard library and the compiler?

- Contributors will now no longer need to use `cfg(bootstrap)` for intrinsics and lang items.
- Contributors may (rarely) need to use `cfg(bootstrap)` in compiler code if they wish to experiment with unstable library features [^dogfood-unstable-lib-features]

# Frequently asked questions (FAQs) {#faqs}

## Doesn't this just shift `cfg(bootstrap)` from library code to compiler code? {#faqs-shift-cfg-bootstrap}

Not quite. `cfg(bootstrap)` usage in standard library code for using new intrinsics / lang items (as in the current bootstrap sequence) is much more common than potential `cfg(bootstrap)` usage in compiler code for experimenting with unstable library features (as in the redesigned bootstrap sequence). This is because the standard library must be changed for new compiler-provided lang items and intrinsics, but the compiler does not (need to) depend on recently added standard library APIs.

Additionally, the compiler only needs to add `cfg(bootstrap)` for anything in the standard library that has changed its unstable API and which is used in the compiler.

> *Example: Implementing a trait solving feature which requires adding core lang items*
>
> This will involve adding a new lang item in the compiler (e.g. [`compiler/rustc_hir/src/lang_items.rs`](https://github.com/rust-lang/rust/blob/5af801b687e6e8b860ae970e725c8b9a3820d0ce/compiler/rustc_hir/src/lang_items.rs#L165)) and the standard library. Prior to the redesign, the usage of the lang item in the standard library requires `cfg(not(bootstrap))` since the beta compiler does not know about the new lang item. Recall that the standard library has to support being built by both the beta compiler and the in-tree compiler! After the redesign, `cfg(not(bootstrap))` usage of the lang item in the standard library is not needed since the standard library is only buildable by the in-tree compiler that adds the new lang item.
>
> A stage 2 compiler is **not** required to test the new feature, as the stage 1 library using the lang item is built by the stage 1 compiler, which is the compiler where the new lang item is added!

# Questions, feedback, bugs?

You can leave a comment in the [zulip support thread for the initial bootstrap sequence redesign effort][zulip-support-thread]


[^initial-compiler]: For the vast majority of contributors, the stage 0 "initial" compiler is going to be the beta compiler. However, it is possible to override the initial compiler, such as when further optimizing a compiler through PGO/BOLT. In this blog post, we make a simplifying assumption that the stage 0 compiler is the beta compiler, even though this is not universally true.
[^dogfood-unstable-lib-features]: Newly added unstable library feature may need to wait until a beta bump before it is usable by the compiler.
[^adapted]: Much of this is adapted from jyn's excellent blog post [*Why is Rust's build system uniquely hard to use?*][hard-to-use-blog-post].

[rust-lang/rust]: https://github.com/rust-lang/rust
[bootstrap]: https://rustc-dev-guide.rust-lang.org/building/bootstrapping/intro.html
[compiler-bootstrapping]: https://en.wikipedia.org/wiki/Bootstrapping_(compilers)
[redesign-stage0-mcp]: https://github.com/rust-lang/compiler-team/issues/619
[hard-to-use-blog-post]: https://jyn.dev/bootstrapping-rust-in-2023/
[gcc-bootstrap]: https://gcc.gnu.org/install/build.html
[clang-bootstrap]: https://llvm.org/docs/AdvancedBuilds.html#bootstrap-builds
[zulip-support-thread]: https://rust-lang.zulipchat.com/#narrow/channel/326414-t-infra.2Fbootstrap/topic/Stage.200.20std.20redesign.20support.20thread/with/515096924
[stage0-redesign-pr]: https://github.com/rust-lang/rust/pull/119899
