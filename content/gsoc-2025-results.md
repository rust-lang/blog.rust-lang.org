+++
path = "2025/11/14/gsoc-2025-results"
title = "Google Summer of Code 2025 results"
authors = ["Jakub Beránek", "Jack Huey"]

[extra]
team = "the mentorship team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-mentorship"
+++

As we have [announced][gsoc-blog-post] previously this year, the Rust Project participated
in [Google Summer of Code (GSoC)][gsoc] for the second time. Almost twenty contributors have been working very hard on their projects for several months. Same as last year, the projects had various durations, so some of them have ended in September, while the last ones have been concluded in the middle of November. Now that the final reports of all projects have been submitted, we are happy to announce that 18 out of the total 19 contributors have passed the final review! We had a very large number of projects this year, so we consider this number of successfully finished projects to be a great result.

We had awesome interactions with our GSoC contributors over the summer, and through a video call, we also had a chance to see each other and discuss the accepted GSoC projects. Our contributors have learned a lot of new things and collaborated with us on making Rust better for everyone, and we are very grateful for all their contributions! Some of them have even continued contributing after their project has ended, and we hope to keep working with them in the future, to further improve open-source Rust software. **We would like to thank all our Rust GSoC 2025 contributors. You did a great job!**

Same as last year, Google Summer of Code 2025 was overall a success for the Rust Project, this time with more than double the number of projects. We think that GSoC is a great way of introducing new contributors to our community, and we are looking forward to participating in GSoC (or similar programs) again in the near future. If you are interested in becoming a (GSoC) contributor, check out our [GSoC project idea list](https://github.com/rust-lang/google-summer-of-code) and our [guide for new contributors][contributor-guide].

Below you can find a brief summary of each of our GSoC 2025 projects. You can find more information about the original goals of the projects [here][gsoc-project-list]. For easier navigation, here is an index of all the project descriptions in alphabetical order:


- [ABI/Layout handling for the automatic differentiation feature](#abi-layout-handling-for-the-automatic-differentiation-feature) by [Marcelo Domínguez](https://github.com/sa4dus)
- [Add safety contracts](#add-safety-contracts) by [Dawid Lachowicz](https://github.com/dawidl022)
- [Bootstrap of rustc with rustc_codegen_gcc](#bootstrap-of-rustc-with-rustc-codegen-gcc) by [Michał Kostrubiec](https://github.com/FractalFir)
- [Cargo: Build script delegation](#cargo-build-script-delegation) by [Naman Garg](https://github.com/namanlp)
- [Distributed and resource-efficient verification](#distributed-and-resource-efficient-verification) by [Jiping Zhou](https://github.com/zjp-CN)
- [Enable Witness Generation in cargo-semver-checks](#enable-witness-generation-in-cargo-semver-checks) by [Talyn Veugelers](https://github.com/GlitchlessCode)
- [Extend behavioural testing of std::arch intrinsics](#extend-behavioural-testing-of-std-arch-intrinsics) by [Madhav Madhusoodanan](https://github.com/madhav-madhusoodanan)
- [Implement merge functionality in bors](#implement-merge-functionality-in-bors) by [Sakibul Islam](https://github.com/Sakib25800)
- [Improve bootstrap](#improve-bootstrap) by [Shourya Sharma](https://github.com/Shourya742)
- [Improve Wild linker test suites](#improve-wild-linker-test-suites) by [Kei Akiyama](https://github.com/lapla-cogito)
- [Improving the Rustc Parallel Frontend: Parallel Macro Expansion](#improving-the-rustc-parallel-frontend-parallel-macro-expansion) by [Lorrens](https://github.com/LorrensP-2158466)
- [Make cargo-semver-checks faster](#make-cargo-semver-checks-faster) by [Joseph Chung](https://github.com/CLIDragon)
- [Make Rustup Concurrent](#make-rustup-concurrent) by [Francisco Gouveia](https://github.com/FranciscoTGouveia)
- [Mapping the Maze of Rust's UI Test Suite with Established Continuous Integration Practices](#mapping-the-maze-of-rust-s-ui-test-suite-with-established-continuous-integration-practices) by [Julien Robert](https://github.com/oneirical)
- [Modernising the libc Crate](#modernising-the-libc-crate) by [Abdul Muiz](https://github.com/mbyx)
- [New proc-macro Server API for Rust-Analyzer](#new-proc-macro-server-api-for-rust-analyzer) by [Neil Wang](https://github.com/DriedYellowPeach)
- [Prepare stable_mir crate for publishing](#prepare-stable-mir-crate-for-publishing) by [Makai](https://github.com/makai410)
- [Prototype an alternative architecture for cargo fix using cargo check](#prototype-an-alternative-architecture-for-cargo-fix-using-cargo-check) by [Glen Thalakottur](https://github.com/Pyr0de)
- [Prototype Cargo Plumbing Commands](#prototype-cargo-plumbing-commands) by [Vito Secona](https://github.com/secona)

And now strap in, as there is a ton of great content to read about here!

### ABI/Layout handling for the automatic differentiation feature
- Contributor: [Marcelo Domínguez](https://github.com/sa4dus)
- Mentors: [Manuel Drehwald](https://github.com/ZuseZ4), [Oli Scherer](https://github.com/oli-obk)
- [Final report](https://sa4dus.github.io/posts/gsoc2025-final-report/)

The `std::autodiff` module allows computing gradients and derivatives in the calculus sense. It provides two autodiff macros, which can be applied to user-written functions and automatically generate modified versions of those functions, which also compute the requested gradients and derivatives. This functionality is very useful especially in the context of scientific computing and implementation of machine-learning models.

Our autodiff frontend was facing two challenges.
- First, we would generate a new function through our macro expansion, however, we would not have a suitable function body for it yet. Our autodiff implementation relies on an LLVM plugin to generate the function body. However, this plugin only gets called towards the end of the compilation pipeline. Earlier optimization passes, either on the LLVM or the Rust side, could look at the placeholder body and either "optimize" or even delete the function since it has no clear purpose yet.
- Second, the flexibility of our macros was causing issues, since it allows requesting derivative computations on a per-argument basis. However, when we start to lower Rust arguments to our compiler backends like LLVM, we do not always have a 1:1 match of Rust arguments to LLVM arguments. As a simple example, an array with two double values might be passed as two individual double values on LLVM level, whereas an array with three doubles might be passed via a pointer.

Marcelo helped rewrite our `autodiff` macros to not generate hacky placeholder function bodies, but instead introduced a proper `autodiff` intrinsic. This is the proper way for us to declare that an implementation of this function is not available yet and will be provided later in the compilation pipeline. As a consequence, our generated functions were not deleted or incorrectly optimized anymore. The intrinsic PR also allowed removing some previous hacks and therefore reduced the total lines of code in the Rust compiler by over 500! You can find more details in [this PR](https://github.com/rust-lang/rust/pull/142640).

Beyond autodiff work, Marcelo also initiated work on GPU offloading intrinsics, and helped with multiple bugs in our argument handling. We would like to thank Marcelo for all his great work!

### Add safety contracts
- Contributor: [Dawid Lachowicz](https://github.com/dawidl022)
- Mentor: [Michael Tautschnig](https://github.com/tautschnig)
- [Final report](https://gist.github.com/dawidl022/e7ec4e1674156dd43c58e76e05267abe)

The Rust Project has an ambitious goal to [instrument the Rust standard library with safety contracts](https://rust-lang.github.io/rust-project-goals/2025h1/std-contracts.html), moving from informal comments that specify safety requirements of `unsafe` functions to executable Rust code. This transformation represents a significant step toward making Rust's safety guarantees more explicit and verifiable. To prioritize which functions should receive contracts first, there is a [verification contest](https://github.com/model-checking/verify-rust-std) ongoing.

Given that Rust contracts are still in their [early stages](https://github.com/rust-lang/rust/issues/128044), Dawid's project was intentionally open-ended in scope and direction. This flexibility allowed Dawid to identify and tackle several key areas that would add substantial value to the contracts ecosystem. His contributions were in the following three main areas:

- **Pragmatic Contracts Integration**: Refactoring [contract HIR lowering](https://github.com/rust-lang/rust/pull/144438) to ensure no contract code is executed when contract-checks are disabled. This has major impact as it ensures that contracts do not have runtime cost when contract checks are disabled.

- **Variable Reference Capability**: Adding the ability to [refer](https://github.com/rust-lang/rust/pull/144444) to variables from preconditions within postconditions. This fundamental enhancement to the contracts system has been fully implemented and merged into the compiler. This feature provides developers with much more expressive power when writing contracts, allowing them to establish relationships between input and output states.

- **Separation Logic Integration**: The bulk of Dawid's project involved identifying, understanding, and planning the introduction of owned and block ownership predicates for separation-logic style reasoning in contracts for unsafe Rust code. This work required extensive research and collaboration with experts in the field. Dawid engaged in multiple discussions with authors of Rust validation tools and Miri developers, both in person and through Zulip discussion threads. The culmination of this research is captured in a comprehensive MCP (Major Change Proposal) that Dawid [created](https://github.com/rust-lang/compiler-team/issues/942).

Dawid's work represents crucial foundational progress for Rust's safety contracts initiative. By successfully implementing variable reference capabilities and laying the groundwork for separation logic integration, he has positioned the contracts feature for significant future development. His research and design work will undoubtedly influence the direction of this important safety feature as it continues to mature. Thank you very much!

### Bootstrap of rustc with rustc_codegen_gcc
- Contributor: [Michał Kostrubiec](https://github.com/FractalFir)
- Mentor: [antoyo](https://github.com/antoyo)
- [Final report](https://fractalfir.github.io/generated_html/gsoc_2025_wp.html)

The goal of this project was to improve the Rust GCC codegen backend ([`rustc_codegen_gcc`](https://github.com/rust-lang/rustc_codegen_gcc)), so that it would be able to compile the "stage 2"[^bootstrapping] Rust compiler (`rustc`) itself [again](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-10).

You might remember that Michał already participated in GSoC [last year](https://blog.rust-lang.org/2024/11/07/gsoc-2024-results/#rust-to-net-compiler-add-support-for-compiling-running-cargo-tests), where he was working on his own .NET Rust codegen backend, and he did an incredible amount of work. This year, his progress was somehow even faster. Even before the official GSoC implementation period started (!), he essentially completed his original project goal and managed to build `rustc` with GCC. This was no small feat, as he had to investigate and fix several miscompilations that occurred when functions marked with `#[inline(always)]` were called recursively or when the compiled program was trying to work with 128-bit integers. You can read more about this initial work at his [blog](https://fractalfir.github.io/generated_html/cg_gcc_bootstrap.html).

After that, he immediately started working on stretch goals of his project. The first one was to get a "stage-3" `rustc` build working, for which he had to vastly [improve](https://github.com/rust-lang/rustc_codegen_gcc/pull/680) the memory consumption of the codegen backend.

Once that was done, he moved on to yet another goal, which was to build `rustc` for a platform not supported by LLVM. He made progress on this for [Dec Alpha](https://github.com/rust-lang/rustc_codegen_gcc/issues/742) and [m68k](https://github.com/rust-lang/rustc_codegen_gcc/issues/744). He also attempted to compile `rustc` on Aarch64, which led to him finding an ABI bug. Ultimately, he managed to build a `rustc` for m68k (with a few workarounds that we will need to fix in the future). That is a very nice first step to porting Rust to new platforms unsupported by LLVM, and is important for initiatives such as [Rust for Linux](https://rust-for-linux.com/).

Michał had to spend a lot of time starting into assembly code and investigating arcane ABI problems. In order to make this easier for everyone, he implemented support for [fuzzing](https://github.com/rust-lang/rustc_codegen_gcc/pull/688) and automatically checking [ABI mismatches](https://github.com/rust-lang/rustc_codegen_gcc/pull/710) in the GCC codegen backend. You can read more about his testing and fuzzing efforts [here](https://fractalfir.github.io/generated_html/cg_gcc_bootstrap_2.html).

We were really impressed with what Michał was able to achieve, and we really appreciated working with him this summer. Thank you for all your work, Michał!

[^bootstrapping]: You can read about what do those individual compiler stages mean e.g. [here](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html).

### Cargo: Build script delegation
- Contributor: [Naman Garg](https://github.com/namanlp)
- Mentor: [Ed Page](https://github.com/epage)
- [Final report](https://gist.github.com/namanlp/90f96165d92f848b0452b34c68892a1c)

Cargo build scripts come at a compile-time cost, because even to run `cargo check`, they must be built as if you ran `cargo build`, so that they can be executed during compilation. Even though we try to identify ways to reduce the [need](https://github.com/rust-lang/cargo/issues/14948) to write build scripts in the first place, that may not always be doable. However, if we could shift build scripts from being defined in every package that needs them, into a few core build script packages, we could both reduce the compile-time overhead, and also improve their auditability and transparency. You can find more information about this idea [here](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#metabuild).

The first step required to delegate build scripts to packages is to be able to run multiple build scripts per crate, so that is what Naman was primarily working on. He introduced a new unstable `multiple-build-scripts` feature to Cargo, implemented support for parsing an array of build scripts in `Cargo.toml`, and extended Cargo so that it can now execute multiple build scripts while building a single crate. He also added a set of tests to ensure that this feature will work as we expect it to.

Then he worked on ensuring that the execution of builds scripts is performed in a deterministic order, and that crates can access the output of each build script separately. For example, if you have the following configuration:
```toml
[package]
build = ["windows-manifest.rs", "release-info.rs"]
```
then the corresponding crate is able to access the `OUT_DIR`s of both build scripts using `env!("windows-manifest_OUT_DIR")` and `env!("release-info_OUTDIR")`.

As future work, we would like to implement the ability to pass parameters to build scripts through metadata specified in `Cargo.toml` and then implement the actual build script delegation to external build scripts using [artifact-dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies).

We would like to thank Naman for helping improving Cargo and laying the groundwork for a feature that could have compile-time benefits across the Rust ecosystem!

### Distributed and resource-efficient verification
- Contributor: [Jiping Zhou](https://github.com/zjp-CN)
- Mentor: [Michael Tautschnig](https://github.com/tautschnig)
- [Final report](https://github.com/os-checker/distributed-verification/blob/f480981dbf529a25ae50e4cc799e5398165fc325/GSoC2025.md)

The goal of this project was to address critical scalability challenges of formally verifying Rust's standard library by developing a distributed verification system that intelligently manages computational resources and minimizes redundant work. The [Rust standard library verification project](https://github.com/model-checking/verify-rust-std) faces significant computational overhead when verifying large codebases, as traditional approaches re-verify unchanged code components. With Rust's standard library containing thousands of functions and continuous development cycles, this inefficiency becomes a major bottleneck for practical formal verification adoption.

Jiping implemented a distributed verification system with several key innovations:

- **Intelligent Change Detection**: The system uses hash-based analysis to identify which parts of the codebase have actually changed, allowing verification to focus only on modified components and their dependencies.
- **Multi-Tool Orchestration**: The project coordinates multiple verification backends including Kani model checker, with careful version pinning and compatibility management.
- **Distributed Architecture**: The verification workload is distributed across multiple compute nodes, with intelligent scheduling that considers both computational requirements and dependency graphs.
- **Real-time Visualization**: Jiping built a comprehensive web interface that provides live verification status, interactive charts, and detailed proof results. You can check it out [here](https://os-checker.github.io/distributed-verification/chart)!

You can find the created distributed verification tool in [this](https://github.com/os-checker/distributed-verification) repository. Jiping's work established a foundation for scalable formal verification that can adapt to the growing complexity of Rust's ecosystem, while maintaining verification quality and completeness, which will go a long way towards ensuring that Rust's standard library remains safe and sound. Thank you for your great work!

### Enable Witness Generation in cargo-semver-checks
- Contributor: [Talyn Veugelers](https://github.com/GlitchlessCode)
- Mentor: [Predrag Gruevski](https://github.com/obi1kenobi)
- [Final report](https://glitchlesscode.ca/posts/2025-11-05a/)

[`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) is a Cargo subcommand for finding SemVer API breakages in Rust crates. Talyn's project aimed to lay the groundwork for it to tackle our most vexing limitation: the inability to catch SemVer breakage due to type changes.

Imagine a crate makes the following change to its public API:
```rust
// baseline version
pub fn example(value: i64) {}

// new version
pub fn example(value: String) {}
```

This is *clearly* a major breaking change, right? And yet `cargo-semver-checks` with its hundreds of lints is *still* unable to flag this. While this case seems trivial, it's just the tip of an enormous iceberg. Instead of changing `i64` to `String`, what if the change was from `i64` to `impl Into<i64>`, or worse, into some monstrosity like:
```rust
pub fn example<T, U, const N: usize>(
    value: impl for<'a> First<'a, T> + Second<U, N> + Sync
) {}
```

Figuring out whether this change is breaking requires checking whether the original `i64` parameter type can "fit" into that monstrosity of an `impl Trait` type. But reimplementing a Rust type checker and trait solver inside `cargo-semver-checks` is out of the question! Instead, we turn to a technique created for [a previous study of SemVer breakage on crates.io](https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/)—we generate a "witness" program that will fail to compile if, and only if, there's a breaking change between the two versions.

The witness program is a separate crate that can be made to depend on either the old or the new version of the crate being scanned. If our `example` function comes from a crate called `upstream`, its witness program would look something like:
```rust
// take the same parameter type as the baseline version
fn witness(value: i64) {
    upstream::example(value);
}
```

> This example is cherry-picked to be easy to understand. Witness programs are rarely this straightforward!

Attempting to `cargo check` the witness while plugging in the new version of `upstream` forces *the Rust compiler* to decide whether `i64` matches the new `impl Trait` parameter. If `cargo check` passes without errors, there's no breaking change here. But if there's a compilation error, then this is concrete, incontrovertible evidence of breakage!

Over the past 22+ weeks, Talyn worked tirelessly to move this from an idea to a working proof of concept. For every problem we foresaw needing to solve, ten more emerged along the way. Talyn did a lot of design work to figure out an approach that would be able to deal with crates coming from various sources (crates.io, a path on disk, a git revision), would support multiple rustdoc JSON formats for all the hundreds of existing lints, and do so in a fashion that doesn't get in the way of adding hundreds more lints in the future.

Even the above list of daunting challenges fails to do justice to the complexity of this project. Talyn created a witness generation prototype that lays the groundwork for robust checking of type-related SemVer breakages in the future. The success of this work is key to the `cargo-semver-checks` roadmap for 2026 and beyond. We would like to thank Talyn for their work, and we hope to continue working with them on improving witness generation in the future.

### Extend behavioural testing of std::arch intrinsics
- Contributor: [Madhav Madhusoodanan](https://github.com/madhav-madhusoodanan)
- Mentor: [Amanieu d'Antras](https://github.com/amanieu)
- [Final report](https://hackmd.io/@madhav-madhusoodanan/extend-behavioural-testing-of-stdarch-intrinsics)

The `std::arch` module contains target-specific intrinsics (low-level functions that typically correspond to single machine instructions) which are intended to be used by other libraries. These are intended to match the equivalent intrinsics available as vendor-specific extensions in C.

The intrinsics are tested with three approaches. We test that:
- The signatures of the intrinsics match the one specified by the architecture.
- The intrinsics generate the correct instruction.
- The intrinsics have the correct runtime behavior.

These behavior tests are implemented in the [intrinsics-test](https://github.com/rust-lang/stdarch/tree/master/crates/intrinsic-test) crate. Initially, this test framework only covered the AArch64 and AArch32 targets, where it was very useful in finding bugs in the implementation of the intrinsics. Madhav's project was about refactoring and improving this framework to make it easier (or really, possible) to extend it to other CPU architectures.

First, Madhav [split](https://github.com/rust-lang/stdarch/pull/1758) the codebase into a module with shared (architecturally independent) code and a module with ARM-specific logic. Then he implemented support for testing intrinsics for the x86 architecture, which is Rust's most widely used target. In doing so, he allowed us to discover real bugs in the implementation of some intrinsics, which is a great result! Madhav also did a lot of work in optimizing how the test suite is compiled and executed, to reduce CI time needed to run tests, and he laid the groundwork for supporting even more architectures, specifically LoongArch and WebAssembly.

We would like to thank Madhav for all his work on helping us make sure that Rust intrinsics are safe and correct!

### Implement merge functionality in bors
- Contributor: [Sakibul Islam](https://github.com/Sakib25800)
- Mentor: [Jakub Beránek](https://github.com/kobzol)
- [Final report](https://sakib25800.notion.site/Google-Summer-of-Code-2025-Implement-merge-functionality-in-bors-25ca1a53f49280a88ecdf72b00cb912d)

The main [Rust repository](https://github.com/rust-lang/rust) uses a pull request merge queue bot that we call `bors`. Its current [Python implementation](https://github.com/rust-lang/homu) has a lot of issues and was difficult to maintain. The goal of this GSoC project was thus to implement the primary merge queue functionality in our [Rust rewrite](https://github.com/rust-lang/bors) of this bot.

Sakibul first examined the original Python codebase to figure out what it was doing, and then he implemented several bot commands that allow contributors to approve PRs, set their priority, delegate approval rights, temporarily close the merge tree, and many others. He also implemented an asynchronous background process that checks whether a given pull request is mergeable or not (this process is relatively involved, due to how GitHub works), which required implementing a specialized synchronized queue for deduplicating mergeability check requests to avoid overloading the GitHub API. Furthermore, Sakibul also reimplemented (a nicer version of) the merge queue status [webpage](https://bors-prod.rust-lang.net/queue/rust) that can be used to track which pull requests are currently being tested on CI, which ones are approved, etc.

After the groundwork was prepared, Sakibul could work on the merge queue itself, which required him to think about many tricky race conditions and edge cases to ensure that bors doesn't e.g. merge the wrong PR into the default branch or merge a PR multiple times. He covered these edge cases with many integration tests, to give us more confidence that the merge queue will work as we expect it to, and also prepared a script for creating simulated PRs on a test GitHub repository so that we can test bors "in the wild". And so far, it seems to be working very well!

After we finish the final piece of the merge logic (creating so-called ["rollups"](https://forge.rust-lang.org/release/rollups.html)) together with Sakibul, we will start using bors fully in the main Rust repository. Sakibul's work will thus be used to merge all `rust-lang/rust` pull requests. Exciting!

Apart from working on the merge queue, Sakibul made many other awesome contributions to the codebase, like refactoring the test suite or analyzing performance of SQL queries. In total, Sakibul sent around [fifty pull requests](https://github.com/rust-lang/bors/pulls?q=is%3Apr+author%3Asakib25800+is%3Amerged+) that were already merged into bors! What can we say, other than: Awesome work Sakibul, thank you!

### Improve bootstrap
- Contributor: [Shourya Sharma](https://github.com/Shourya742)
- Mentors: [Jakub Beránek](https://github.com/kobzol), [Jieyou Xu](https://github.com/jieyouxu), [Onur Özkan](https://github.com/onur-ozkan)
- [Final report](https://gist.github.com/Shourya742/b15bbbfdc49c98060171a0bf487c4164)

[bootstrap](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html) is the build system of Rust itself, which is responsible for building the compiler, standard library, and pretty much everything else that you can download through `rustup`. This project's goal was very open-ended: "improve bootstrap".

And Shourya did just that! He made meaningful contributions to several parts of bootstrap. First, he added much-needed documentation to several core bootstrap data structures and modules, which were quite opaque and hard to understand without any docs. Then he moved to improving command execution, as each bootstrap invocation invokes hundreds of external binaries, and it was difficult to track them. Shourya finished a long-standing refactoring that routes almost all executed commands through a single place. This allowed him to also implement command caching and also command profiling, which shows us which commands are the slowest.

After that, Shourya moved on to refactoring config parsing. This was no easy task, because bootstrap has A LOT of config options; the single *function* that parses them had over a thousand lines of code (!). A set of complicated config precedence rules was frequently causing bugs when we had to modify that function. It took him several weeks to untangle this mess, but the result is worth it. The [refactored function](https://github.com/rust-lang/rust/blob/master/src/bootstrap/src/core/config/config.rs#L356) is much less brittle and easier to understand and modify, which is great for future maintenance.

The final area that Shourya improved were bootstrap tests. He made it possible to run them using bare `cargo`, which enables debugging them e.g. in an IDE, which is very useful, and mainly he found a way to run the tests in parallel, which makes contributing to bootstrap itself much more pleasant, as it reduced the time to execute the tests from a minute to under ten seconds. These changes required refactoring many bootstrap tests that were using global state, which was not compatible with parallel execution.

Overall, Shourya made more than [30 PRs](https://github.com/rust-lang/rust/pulls?q=is%3Apr+author%3Ashourya742+is%3Amerged+) to bootstrap since April! We are very thankful for all his contributions, as they made bootstrap much easier to maintain. Thank you!

### Improve Wild linker test suites
- Contributor: [Kei Akiyama](https://github.com/lapla-cogito)
- Mentor: [David Lattimore](https://github.com/davidlattimore)
- [Final report](https://lapla.dev/posts/gsoc_final/)

[Wild](https://github.com/davidlattimore/wild) is a very fast linker for Linux that’s written in Rust. It can be used to build executables and shared objects.

Kei’s project was to leverage the test suite of one of the other Linux linkers to help test the Wild linker. This goal was accomplished. Thanks to Kei’s efforts, we now run the Mold test suite against Wild in our CI. This has helped to prevent regressions on at least a couple of occasions and has also helped to show places where Wild has room for improvement.

In addition to this core work, Kei also undertook numerous other changes to Wild during GSoC. Of particular note was the reworking of argument parsing to support `--help`, which we had wanted for some time. Kei also fixed a number of bugs and implemented various previously missing features. This work has helped to expand the range of projects that can use Wild to build executables.

Kei has continued to contribute to Wild even after the GSoC project finished and has now contributed over [sixty PRs](https://github.com/davidlattimore/wild/pulls?q=is%3Apr+author%3Alapla-cogito+is%3Amerged+). We thank Kei for all the hard work and look forward to continued collaboration in the future!

### Improving the Rustc Parallel Frontend: Parallel Macro Expansion
- Contributor: [Lorrens](https://github.com/LorrensP-2158466)
- Mentors: [Sparrow Li](https://github.com/sparrowlii), [Vadim Petrochenkov](https://github.com/petrochenkov)
- [Final report](https://lorrens.me/2025/10/26/GSoC-Parallel-Macro-Expansion.html)

The Rust compiler has a (currently unstable) parallel compilation mode in which some compiler passes run in parallel.
One major part of the compiler that is not yet affected by parallelization is name resolution.
It has several components, but those selected for this GSoC project were import resolution and macro expansion (which are in fact intermingled into a single fixed-point algorithm).
Besides the parallelization itself, another important point of the work was improving the correctness of import resolution by eliminating accidental order dependencies in it, as those also prevent parallelization.

We should note that this was a *very* ambitious project, and we knew from the beginning that it would likely be quite challenging to reach the end goal within the span of just a few months. And indeed, Lorrens did in fact run into several unexpected issues that showed us that the complexity of this work is well beyond a single GSoC project, so he didn't actually get to parallelizing the macro expansion algorithm. Nevertheless, he did a lot of important work to improve the name resolver and prepare it for being parallelized.

The first thing that Lorrens had to do was actually understand how Rust name resolution works and how it is implemented in the compiler. That is, to put it mildly, a *very complex* piece of logic, and is affected by legacy burden in the form of backward compatibility lints, outdated naming conventions, and other technical debt. Even this learned knowledge itself is incredibly useful, as the set of people that understand Rust's name resolution today is very low, so it is important to grow it.

Using this knowledge, he made a lot of refactorings to separate *significant* mutability in name resolver data structures from "cache-like" mutability used for things like lazily loading otherwise immutable data from extern crates, which was needed to unblock parallelization work. He split [various](https://github.com/rust-lang/rust/pull/143657) [parts](https://github.com/rust-lang/rust/pull/143884) of the name resolver, got rid of [unnecessary](https://github.com/rust-lang/rust/pull/144059) [mutability](https://github.com/rust-lang/rust/pull/144605) and performed a bunch of [other](https://github.com/rust-lang/rust/pull/145322) [refactorings](https://github.com/rust-lang/rust/pull/147805). He also had to come up with a very tricky [data structure](https://github.com/rust-lang/rust/pull/144912) that allows providing conditional mutable access to some data.

These refactorings allowed him to implement something called ["batched import resolution"](https://github.com/rust-lang/rust/pull/145108), which splits unresolved imports in the crate into "batches", where all imports in a single batch can be resolved independently and potentially in parallel, which is crucial for parallelizing name resolution. We have to resolve a few remaining language [compatibility](https://github.com/rust-lang/rust/pull/147984) [issues](https://github.com/rust-lang/rust/pull/147995), after which the batched import resolution work will hopefully be merged.

Lorrens laid an important groundwork for fixing potential correctness issues around name resolution and macro expansion, which unblocks further work on parallelizing these compiler passes, which is exciting. His work also helped unblock some [library](https://github.com/rust-lang/rust/pull/137487) [improvements](https://github.com/rust-lang/rust/pull/139493) that were stuck for a long time. We are grateful for your hard work on improving tricky parts of Rust and its compiler, Lorrens. Thank you!

### Make cargo-semver-checks faster
- Contributor: [Joseph Chung](https://github.com/CLIDragon)
- Mentor: [Predrag Gruevski](https://github.com/obi1kenobi)
- [Final report](https://clidragon.github.io/blog/gsoc-2025)

[`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) is a Cargo subcommand for finding SemVer API breakages in Rust crates. It is adding SemVer lints at an *exponential* pace: the number of lints has been doubling every year, and currently stands at `229`. More lints mean more work for `cargo-semver-checks` to do, as well as more work for its test suite which runs over 250000 lint checks!

Joseph's contributions took three forms:
- Improving `cargo-semver-checks` runtime performance—on large crates, our query runtime went from ~8s to ~2s, a 4x improvement!
- Improving the test suite's performance, enabling us to iterate faster. Our test suite used to take ~7min and now finishes in ~1min, a 7x improvement!
- Improving our ability to profile query performance and inspect performance anomalies, both of which were proving a bottleneck for our ability to ship further improvements.

Joseph described all the clever optimization tricks leading to these results in his [final report](https://clidragon.github.io/blog/gsoc-2025). To encourage you to check out the post, we'll highlight a particularly elegant optimization described there.

`cargo-semver-checks` relies on rustdoc JSON, an unstable component of Rust whose output format often has breaking changes. Since each release of `cargo-semver-checks` supports a range of Rust versions, it must also support a range of rustdoc JSON formats. Fortunately, each file carries a version number that tells us which version's `serde` types to use to deserialize the data.

Previously, we used to deserialize the JSON file twice: once with a `serde` type that only loaded the `format_version: u32` field, and a second time with the appropriate `serde` type that matches the format. This works fine, but many large crates generate rustdoc JSON files that are 500 MiB+ in size, requiring us to walk all that data twice. While `serde` is quite fast, there's nothing as fast as *not* doing the work twice in the first place!

So we used a trick: [*optimistically* check](https://github.com/obi1kenobi/trustfall-rustdoc/pull/98/files#diff-b1a35a68f14e696205874893c07fd24fdb88882b47c23cc0e0c80a30c7d53759R46-R82) if the `format_version` field is the last field in the JSON file, which happens to be the case every time (even though it is not guaranteed). Rather than parsing JSON, we merely look for a `,` character in the last few dozen bytes, then look for `:` after the `,` character, and for `format_version` between them. If this is successful, we've discovered the version number while avoiding going through hundreds of MB of data! If we failed for any reason, we just fall back to the original approach having only wasted the effort of looking at 20ish extra bytes.

Joseph did a lot of profiling and performance optimizations to make `cargo-semver-checks` faster for everyone, with awesome results. Thank you very much for your work!

### Make Rustup Concurrent
- Contributor: [Francisco Gouveia](https://github.com/FranciscoTGouveia)
- Mentor: [rami3l](https://github.com/rami3l)
- [Final report](https://web.tecnico.ulisboa.pt/francisco.t.gouveia/posts/03-gsoc-final-report/)

As a very important part of the Rustup team's vision of migrating the [rustup](https://github.com/rust-lang/rustup) codebase to using async IO since the introduction of the global `tokio` runtime in [#3367](https://github.com/rust-lang/rustup/issues/3367), this project's goal was to introduce proper concurrency to rustup. Francisco did that by attacking two aspects of the codebase at once:

1. He created a new set of user interfaces for displaying concurrent progress.
2. He implemented a new toolchain update checking & installation flow that is idiomatically concurrent.

As a warmup, Francisco made `rustup check` concurrent, resulting in a rather easy [3x performance boost](https://web.tecnico.ulisboa.pt/francisco.t.gouveia/posts/02-rustup-concurrent-checks) in certain cases. Along the way, he also introduced a new [indicatif](https://crates.io/crates/indicatif)-based progress bar for reporting progress of concurrent operations, which replaced the original hand-rolled solution.

After that, the focus of the project has moved on to the toolchain installation flow used in commands like `rustup toolchain install` and `rustup update`. In this part, Francisco developed two main improvements:

1. The possibility of downloading multiple components at once when setting up a toolchain, controlled by the `RUSTUP_CONCURRENT_DOWNLOADS` environment variable. Setting this variable to a value greater than 1 is particularly useful in certain internet environments where the speed of a single download connection could be restricted by QoS (Quality of Service) limits.
2. The ability to interleave component network downloads and disk unpacking. For the moment, unpacking will still happen sequentially, but disk and net I/O can finally be overlapped! This introduces a net gain in toolchain installation time, as only the last component being downloaded will have noticeable unpacking delays. In our tests, this typically results in a [reduction of 4-6 seconds](https://web.tecnico.ulisboa.pt/francisco.t.gouveia/posts/03-gsoc-final-report/#my-internet-is-fast-can-i-also-have-faster-installations) (on fast connections, that's ~33% faster!) when setting up a toolchain with the `default` profile.

We have to say that these results are very impressive! While a few seconds shorter toolchain installation might not look so important at a first glance, rustup is ubiquitously used to install Rust toolchains on CI of tens of thousands of Rust projects, so this improvement (and also further improvements that it unlocks) will have an enormous effect across the Rust ecosystem. Many thanks to Francisco Gouveia's enthusiasm and active participation, without which this wouldn't have worked out!

### Mapping the Maze of Rust's UI Test Suite with Established Continuous Integration Practices
- Contributor: [Julien Robert](https://github.com/oneirical)
- Mentor: [Jieyou Xu](https://github.com/jieyouxu)
- [Final report](https://oneirical.github.io/25-09-27gsoc/)

The snapshot-based [UI test suite][tests/ui] is a crucial part of the Rust compiler's test suite. It contains _a lot_ of tests: over 19000 at the time of writing. The organization of this test suite is thus
very important, for at least two reasons:

1. We want to be able to find specific tests, identify related tests, and have some sort of logical grouping of related tests.
2. We have to ensure that no directory contains so many entries such that GitHub gives up rendering the directory.

Furthermore, having informative test names and having some context for each test is particularly important, as otherwise contributors would have to reverse-engineer test intent from `git blame` and friends.

Over the years, we have accumulated a lot of unorganized stray test files in the
top level `tests/ui` directory, and have a lot of generically named `issue-*.rs`
tests in the `tests/ui/issues/` directory. The former makes it annoying to find
more meaningful subdirectories, while the latter makes it completely non-obvious
what each test is about.

Julien's project was about introducing some order into the chaos. And that was indeed achieved!
Through Julien's efforts (in conjunction with efforts from other contributors), we now have:

- No more stray tests under the immediate `tests/ui/` top-level directory, and are organized into more meaningful subdirectories. We were able to then introduce a style check to prevent new stray tests from being added.
- A top-level document contains TL;DRs for each of the immediate subdirectories.
- Substantially fewer generically-named `issue-*.rs`under `tests/ui/issues/`.

Test organization (and more generally, test suite ergonomics) is an often under-
appreciated aspect of maintaining complex codebases. Julien spent a lot of effort
improving test ergonomics of the Rust compiler, both in last year's GSoC (where he vastly
improved our ["run-make"](https://blog.rust-lang.org/2024/11/07/gsoc-2024-results/#rewriting-esoteric-error-prone-makefile-tests-using-robust-rust-features) test suite), and then again this year, where he made our UI test suite more ergonomic.
We would like to appreciate your meticulous work, Julien! Thank you very much.

[tests/ui]: https://github.com/rust-lang/rust/tree/master/tests/ui

### Modernising the libc Crate
- Contributor: [Abdul Muiz](https://github.com/mbyx)
- Mentor: [Trevor Gross](https://github.com/tgross35)
- [Final report](https://github.com/mbyx/gsoc25_blog/blob/5fd32d35ea4237b2f586f0d8abceeb2dabd86139/_posts/2025-09-01-work_submission.markdown)

TODO

### New proc-macro Server API for Rust-Analyzer
- Contributor: [Neil Wang](https://github.com/DriedYellowPeach)
- Mentor: [Lukas Wirth](https://github.com/veykril)

> Note: this project was marked as *failed* in the midterm evaluation in July.

The goal of this project was to implement a new API for the proc-macro server, which is used by IDEs such as RustRover or Rust Analyzer to expand macros in Rust crates, with the goal of making it more performant and remove some existing limitations.

Neil started working on the project in June, and even submitted a [draft PR](https://github.com/rust-lang/rust-analyzer/pull/19978) with a sketch of the implementation. However, soon after the project started, Neil stopped responding to their mentor, and we were thus unable to establish communication, which is problematic for a GSoC project. This situation did not improve for a few weeks, and we thus decided to mark the project as failed during the midterm evaluation, as recommended by Google in similar circumstances. Later we learned from Neil that the lack of communication was caused by a combination of being used to work independently, lack of time and also a lack of motivation caused by issues with receiving the GSoC stipend.

We are sorry that this project was not realized, but it sometimes happens. Nevertheless, we thank Neil for submitting the draft implementation and for enrolling for GSoC.

> We might enlist this project topic again in the next edition of GSoC, although that has not been decided yet.

### Prepare stable_mir crate for publishing
- Contributor: [Makai](https://github.com/makai410)
- Mentor: [Celina Val](https://github.com/celinval)
- [Final report](https://makai410.dev/posts/gsoc-25-final/)

This project's goal was to prepare the Rust compiler's `stable_mir`
crate (eventually renamed to `rustc_public`), which provides a way to interface
with the Rust compiler for analyzing Rust code, for publication on crates.io. While the
existing crate provided easier APIs for tool developers, it lacked proper
versioning and was tightly coupled with compiler versions. The goal was to
enable independent publication with semantic versioning.

The main technical work involved restructuring `rustc_public` and `rustc_public_bridge`
(previously named `rustc_smir`) by inverting their dependency relationship.
Makai resolved circular dependencies by temporarily merging the crates and
gradually separating them with the new architecture. They also split the existing
compiler interface to separate public APIs from internal compiler details.

Furthermore, Makai established infrastructure for dual maintenance: keeping an internal
version in the Rust repository to track compiler changes while developing
the publishable version in a dedicated repository. Makai automated a
system to coordinate between versions, and developed custom tooling to validate
compiler version compatibility and to run tests.

Makai successfully completed the core refactoring and infrastructure
setup, making it possible to publish `rustc_public` independently with proper
versioning support for the Rust tooling ecosystem! As a bonus, Makai contributed
several bug fixes and implemented new APIs that had been requested by the
community. Great job Makai!

### Prototype an alternative architecture for cargo fix using cargo check
- Contributor: [Glen Thalakottur](https://github.com/Pyr0de)
- Mentor: [Ed Page](https://github.com/epage)
- [Final report](https://gist.github.com/Pyr0de/363ef42d31364003a534059e02690565)

The `cargo fix` command applies fixes suggested by lints, which makes it useful for cleaning up sloppy code,
reducing the annoyance of toolchain upgrades when lints change and helping with edition migrations and new lint adoption. However, it has a number of issues. It can be [slow](https://github.com/rust-lang/cargo/issues/13214), it only applies a subset of possible lints, and doesn't provide an easy way to select which lints to fix.

These problems are caused by its current architecture; it is implemented as a variant of `cargo check` that replaces `rustc` with `cargo` being run in a special mode that will call `rustc` in a loop, applying fixes until there are none. While this special `rustc`-proxy mode is running,
a cross-process lock is held to force only one build target to be fixed at a time to avoid race conditions.
This ensures correctness at the cost of performance and difficulty in making the `rustc`-proxy interactive.

Glen implemented a proof of concept of an alternative design called [cargo-fixit](https://github.com/crate-ci/cargo-fixit). `cargo fixit` spawns `cargo check` in a loop, determining which build targets are safe to fix in a given pass, and then applying the suggestions. This puts the top-level program in charge of what fixes get applied, making it easier to coordinate. It also allows the locking to be removed and opens the door to an interactive mode.

Glen performed various [benchmarks](https://github.com/crate-ci/cargo-fixit/blob/main/benchsuite/runs/2025-07-31-af6627c.md) to test how the new approach performs.  And in some benchmarks, `cargo fixit` was able to finish within a few hundred milliseconds, where before the same task took `cargo fix` almost a minute! As always, there are trade-offs; the new approach comes at the cost that fixes in packages lower in the dependency tree can cause later packages to be rebuilt multiple times, slowing things down, so there were also benchmarks where the old design was a bit faster. The initial results are still very promising and impressive!

Further work remains to be done on `cargo-fixit` to investigate how it could be optimized better and how should its interface look like before being stabilized. We thank Glen for all the hard work on this project, and we hope that one day the new design will become used by default in Cargo, to bring faster and more flexible fixing of lint suggestions to everyone!

### Prototype Cargo Plumbing Commands
- Contributor: [Vito Secona](https://github.com/secona)
- Mentors: [Cassaundra](https://github.com/cassaundra), [Ed Page](https://github.com/epage)
- [Final report](https://hackmd.io/@secona/gsoc-2025-final-report)

The goal of this project was to move forward our [Project Goal](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html) for creating low-level ("plumbing") Cargo subcommands to make it easier to reuse parts of Cargo by other tools.

Vito created a prototype of several plumbing commands in the [cargo-plumbing](https://github.com/crate-ci/cargo-plumbing) crate. The idea was to better understand how the plumbing commands should look like, and what is needed from Cargo to implement them. Vito had to make compromises in some of these commands to not be blocked on making changes to the current Cargo Rust APIs, and he helpfully documented those [blockers](https://github.com/crate-ci/cargo-plumbing/issues/82). For example, instead of solely relying on the manifests that the user passed in, the plumbing commands will re-read the manifests within each command, preventing callers from being able to edit them to get specific behavior out of Cargo, e.g. dropping all workspace members to allow resolving dependencies on a per-package basis.

Vito did a lot of work, as he implemented seven different plumbing subcommands:

- `locate-manifest`
- `read-manifest`
- `read-lockfile`
- `lock-dependencies`
- `write-lockfile`
- `resolve-features`
- `plan-build`

As future work, we would like to deal with some unresolved questions around how to integrate these plumbing commands within Cargo itself, and extend the set of plumbing commands.

We thank Vito for all his work on improving the flexibility of Cargo.



## Conclusion

We would like to thank all contributors that have participated in Google Summer of Code 2025 with us! It was a blast, and we cannot wait to see which projects GSoC contributors will come up with in the next year. We would also like to thank Google for organizing the Google Summer of Code program and for allowing us to have so many projects this year. And last, but not least, we would like to thank all the Rust mentors who were tirelessly helping our contributors to complete their projects. Without you, Rust GSoC would not be possible.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc-blog-post]: https://blog.rust-lang.org/2025/05/08/gsoc-2025-selected-projects/
[gsoc-project-list]: https://github.com/rust-lang/google-summer-of-code/blob/main/gsoc/runs/2025.md
[gsoc-repo]: https://github.com/rust-lang/google-summer-of-code
[contributor-guide]: https://forge.rust-lang.org/how-to-start-contributing.html
