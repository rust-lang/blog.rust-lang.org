+++
layout = "post"
date = 2025-03-11
title = "This Month in Our Test Infra: January and February 2025"
author = "Jieyou Xu"
team = "the Bootstrap Team <https://www.rust-lang.org/governance/teams/infra#team-bootstrap>"
+++

# This Month in Our Test Infra: January and February 2025

<!-- time period: 2025-01-06 through 2025-03-11 -->

This is a quick summary of the changes in the test infrastructure for the [rust-lang/rust] repository[^scope] for **January and February 2025**[^forgot].

[^forgot]: I may or may not have forgotten about the January issue last month. Oops.

[^scope]: The test infra here refers to the test harness [compiletest] and supporting components in our build system [bootstrap]. This test infra is used mainly by rustc and rustdoc. Other tools like cargo, miri or rustfmt maintain their own test infra.

As usual, if you encounter bugs or UX issues when using our test infrastructure, please [file an issue][new-issue]. Bugs and papercuts can't be fixed if we don't know about them!

**Thanks to everyone who contributed to our test infra!**

## Highlights

### `ci.py` is now a proper `citool` Rust crate

The old `ci.py` Python script used to orchestrate CI jobs was unmaintainable. Any changes to the python script risked bringing down the entire queue or bypass testing entirely. There was practically no test coverage. CI UX improvements were hard to implement and difficult to review.

So, Jakub decided enough was enough and [rewritten `src/ci/github-actions/ci.py` as `src/ci/citool`](https://github.com/rust-lang/rust/pull/136864), a proper Rust CLI tool. This allowed the job definitions to be properly parsed and handled, and also enabled adding unit tests. It also allowed improving some error messages. Furthermore, it improved the UX of running the CI jobs locally (on Linux). Consult the [`rustc-dev-guide` docs in `rust-lang/rust`](https://github.com/rust-lang/rust/blob/master/src/doc/rustc-dev-guide/src/tests/ci.md#docker) for updated running instructions (at the time of writing, this hasn't been synced back to [rust-lang/rustc-dev-guide] yet).

### `try-job`s now supports glob patterns for job names

As part of CI efficiency efforts, many CI jobs have been split into multiple jobs (e.g. `x86_64-apple-{1,2}`) to balance between runner capability and build/test times. This had an unfortunate side effect of making it more difficult to know which job name you need to specify to run the tests you want in custom try jobs.

<https://github.com/rust-lang/rust/pull/138307> permits the contributor to write **glob patterns** to match on job names (up to 20 matching jobs, see next highlight). For instance, if you wanted to run *all* `msvc`-related jobs as part of `try-job`s, you no longer have to specify a whole bunch of e.g. `try-job: x86_64-msvc-1`, `try-job: x86_64-msvc-2`, `try-job: dist-x86_64-msvc`, `try-job: i686-msvc-1`, `try-job: i686-msvc-2`. Instead, you are now able to write

```text
try-job: `*msvc*`
```

Which will expand to match against (and thus run) all of `x86_64-msvc-{1,2}`, `i686-msvc-{1,2}` and `dist-x86_64-msvc`.

Note the backticks (`` ` ``) surrounding the glob pattern. This is needed to prevent GitHub UI from interpreting the glob pattern as (possibly mismatched) markdown italics markup. The backticks will be ignored by CI tooling.

### Max custom `try-job` job limit is now 20 (instead of 10)

You can now run up to 20 custom `try-job`s instead of the previous limit of 10.

### The `Makefile`-based `run-make` test infrastructure has been retired

Almost 8 years ago, astute early contributors noticed that the `Makefile`-based `run-make` tests were both hard-to-run and hard-to-write. It was proposed that we [switch run-make tests from `Makefile` to rust](https://github.com/rust-lang/rust/issues/40713) for multiple motivations, such as:

- Make it more accessible for contributors. `Makefile` syntax (with bash intertwined) and semantics is its own source of bugs and footguns, and is a frequent source of frustrations.
- Reduce dependency on external tools (especially external bin tools) where feasible and beneficial.
- Become *less* platform-dependent.
- Avoid having to deal with *different flavors* of `make`s (GNU make of various versions, `nmake`) that are (subtly) incompatible with each other[^grep].
- Make it possible to *not* have to use some kind of Unix-compatibility layer (e.g. MSYS) to run the test suite on Windows natively (be it MSVC or mingw).

In 2023, after consultation with multiple contributors, we converged on a new [`run-make` test infrastructure][run-make-v2] that has two key components:

1. Each `run-make` test consists of a *test recipe*, `rmake.rs`. This is the main test file.
2. The test recipe has access to a *test support library* called [`run-make-support`][run-make-support]. The support library centralizes common helpers that different `run-make` tests use. It also allows re-exporting useful ecosystem crates for use by tests, such as [`object`] or [`regex`]. Ecosystem crates make it possible for `rmake.rs` tests to perform more precise checks than the text-based manipulations most `Makefile`-based tests use.

The most important difference here is perhaps improved *accessibility* to Rust contributors. The `rmake.rs` tests are just ordinary Rust programs. This means the contributor does not need to be constantly fighting all the `Makefile` and shell syntax quirks (the multitude of quoting styles, interpolation, etc.) or behavioral quirks (e.g. pipefail)[^broken].

There are 200+ `run-make` tests, so we couldn't port them *all* in one go. Instead, the approach taken was:

- The legacy `Makefile`-based `run-make` test infra co-existed with the new `rmake.rs`-based `run-make` test infra. Which test infra was used depended upon whether the test directory contained `Makefile` or `rmake.rs`.
- We maintained a [quest-like tracking issue][run-make-port] that exhaustively listed all the `Makefile`-based `run-make` tests that needed to be ported, and tracked their migration progress. Contributors were invited to claim specific tests that they wanted to help port.
    - This divided the workload between many contributors to make this migration possible. This is still mentored if the contributor needed assistance or wanted to discuss the approach, such as if they wanted to run the test against specific [`try-job`]s.
    - Through a [mentored Google Summer of Code (GSoC) 2024 project][gsoc-2024], [@Oneirical][Oneirical] worked on porting a majority of the `run-make` tests. You can read their [final GSoC report here][gsoc-final].
    - Many maintainers also helped with infrastructure, reviews, testing and providing suggestions, and also authoring migration PRs themselves.
    - **Thanks to everyone who helped in this effort!**
- Adopt a migration process that was *not* a naive 1-to-1 port. *Where possible*, contributors tried to improve the tests to:
    - Become well documented, by linking to relevant context, references, discussions, implementation history and issues where suitable. Many `Makefile` versions of the tests did not have *any* test descriptions. There was *a lot* of git archaeology involved in figuring out what the tests were trying to test in the first place.
    - Actually test what the test wanted to test. For example, `tests/run-make/translation` [did not test what it wanted to test](https://github.com/rust-lang/rust/issues/135817) because the `Makefile` didn't set `SHELL=/bin/bash -o pipefail`.
    - Become more precise and less fragile. Quite a few of `run-make` tests were able to make use of the excellent [`object`] crate to perform structured analysis on binaries (for symbols and debuginfo), as opposed trying to do text grepping on human-readable textual output of bin tools (like `objdump` or `nm`, where the CLI interface and textual output format can also be different between platforms).

The migration effort took around a year, until we were finally able to declare all `Makefile`-based `run-make` tests ported, and thus we were able to [retire the legacy `Makefile`-based test infrastructure in early 2025][retirement].

*Of course*, the new test infrastructure isn't all sunshine and rainbows. There are still issues, desired improvements and test UX papercuts that await to be addressed. However, like the overall test infra, they can be and will be improved over time.

[^grep]: The test suite even had to maintain behavioral tests for `grep` because there are *different* flavors of `grep` that are incompatible with each other and had different CLI interfaces / behavior.
[^broken]: During the porting process, we found multiple tests that had varying degree of brokenness due to hard to notice `Makefile` and shell quirks.

[run-make-v2]: https://github.com/rust-lang/rust/pull/113026
[`object`]: https://crates.io/crates/object
[`regex`]: https://crates.io/crates/regex
[run-make-port]: https://github.com/rust-lang/rust/issues/121876
[`try-job`]: https://rustc-dev-guide.rust-lang.org/tests/ci.html#try-builds
[gsoc-2024]: https://summerofcode.withgoogle.com/archive/2024/projects/P5BC91Hr
[Oneirical]: https://github.com/oneirical
[gsoc-final]: https://oneirical.github.io/gsocfinal/
[retirement]: https://github.com/rust-lang/rust/pull/136581

### Bootstrap test and build step metrics are now available in GitHub job summaries

<https://github.com/rust-lang/rust/pull/137077> implemented postprocessing logic for bootstrap test and build metrics to convert them into [GitHub job summaries][github-job-summaries].

![Sample job summary](../../../../images/2025-03-11-test-infra-jan-feb-2025/example-ci-job-summary.png)

[github-job-summaries]: https://github.blog/news-insights/product-news/supercharging-github-actions-with-job-summaries/


## Notable changes

This section is intended to be like "compatibility notes" but for human test writers.

### `rustc`-based (`ToolRustc`) tools have unified staging handling

Tools that wants to use a locally built `rustc` previously inconsistently implemented their own staging logic in their tool and test steps. This caused a lot of confusion as different `ToolRustc` tools (and their tests) handled the staging differently; some had unnecessarily builds while others were seemingly "off by one stage". There were hacks in various places to "chop off" or "increment" stages separately. To make this situation more maintainable, <https://github.com/rust-lang/rust/pull/137215> unifies `ToolRustc` tool staging logic.

Notably, `./x test` without args and `./x test src/tools/{cargo,clippy}`, where possible, now default to stage 2. Previously, `./x test src/tools/{cargo,clippy}` without explicit test stage configuration corresponded to `--stage 1` but they actually required building stage 2 rustc *anyway*. Bootstrap will now warn if you try to specify a test stage < 2 when testing these two tools (that they don't necessarily work against stage 1 rustc is an pre-existing issue).

Additionally, the *previous* `./x build $rustc_tool --stage 0` invocation (not std or bootstrap tools) is *now* equivalent to `./x build $rustc_tool --stage 1`. Before <https://github.com/rust-lang/rust/pull/137215>, stages for rustc tools in build flows were incremented by inconsistent adjustments, and when `--stage N` was specified on the `./x build $rustc_tool` invocation it would build stage `N+1` rustc. Now, `./x build $rustc_tool --stage N` will produce a rustc-tool using stage `N` rustc.

Consult the new [Writing tools in Bootstrap](https://github.com/rust-lang/rust/blob/master/src/doc/rustc-dev-guide/src/building/bootstrapping/writing-tools-in-bootstrap.md) chapter for further clarification on picking a correct bootstrap tool mode.

### `run-make-support` and `rmake.rs` is now fixed to be built with stage 0 compiler

See <https://github.com/rust-lang/rust/pull/137373> and <https://github.com/rust-lang/rust/pull/137537>.

Previously, `run-make-support` and `rmake.rs` was mistakenly built with top-stage compiler, but this was wrong. `run-make-support` and `rmake.rs` should be built with the *stage 0* compiler (they are test *infra* and needs to be reliable regardless of the possibly broken stage > 0 compiler under test). This caused a few `rmake.rs` tests to accidentally be using unstable features in the test recipes themselves, which will cause issues for beta/stable backports/bumps, and will also cause issues for out-of-tree codegen backends like `rustc_codegen_cranelift` that needs to run `run-make` tests at stage 0.

The docs are also updated to explicitly clarify that `run-make-support` and `rmake.rs` *may not use unstable features*.

### `core` and `alloc` unit tests are now located in separate `coretests` and `alloctests` packages respectively

Having std tests in the same package as a std crate has issues such as

> causing the test to have a dependency on a locally built standard library crate, while also indirectly depending on it through `libtest`

<https://github.com/rust-lang/rust/pull/135937> moves `core` tests and <https://github.com/rust-lang/rust/pull/136642> moves `alloc` tests into separate packages that *does not* depend on `core` to prevent the duplicate crates problem, even when compiler flags don't match between sysroot build and test build.

Other parts of std still has this problem. This is part of an on-going effort to make std tests more robust and more buildable by custom codegen backends.


## PR listing

### Improvements

- [compiletest] and test suites: [Implement `needs-subprocess` directive, and cleanup a bunch of tests to use `needs-{subprocess,threads}`](https://github.com/rust-lang/rust/pull/135926)
- [compiletest]: [Add directives to ignore `arm-unknown-*` targets](https://github.com/rust-lang/rust/pull/136339)
- [compiletest]: [Add `{ignore,only}-rustc_abi-x86-sse2` directives](https://github.com/rust-lang/rust/pull/137074)
- [run-make]: [Port `split-debuginfo` to rmake.rs](https://github.com/rust-lang/rust/pull/135572)
- [library] tests: [Put the `core` unit tests in a separate `coretests` package](https://github.com/rust-lang/rust/pull/135937)
- [library] tests: [Put the `alloc` unit tests in a separate `alloctests` package](https://github.com/rust-lang/rust/pull/136642)
- [bootstrap], [library] tests: [Various `coretests` improvements](https://github.com/rust-lang/rust/pull/137679)
- CI: [Rewrite the `ci.py` script in Rust](https://github.com/rust-lang/rust/pull/136864)
- [bootstrap]: [Stabilize stage management for rustc tools](https://github.com/rust-lang/rust/pull/137215)
- CI, [citool]: [Add post-merge analysis CI workflow](https://github.com/rust-lang/rust/pull/138013)
- CI, [citool]: [Postprocess bootstrap metrics into GitHub job summary](https://github.com/rust-lang/rust/pull/137077)
- CI, [citool]: [Increase the max. custom try jobs requested to `20`](https://github.com/rust-lang/rust/pull/138053)
- CI, [citool]: [Allow specifying glob patterns for try jobs](https://github.com/rust-lang/rust/pull/138307)

### Fixes

- [compiletest]: [Remove a footgun-y feature / relic of the past from the compiletest DSL](https://github.com/rust-lang/rust/pull/136404).[^goober]
- [compiletest]: [Perform deeper compiletest path normalization for `$TEST_BUILD_DIR` to account for compare-mode/debugger cases, and normalize long type file filename hashes](https://github.com/rust-lang/rust/pull/136865)
- [compiletest]: [compiletest should not inherit all host `RUSTFLAGS`](https://github.com/rust-lang/rust/pull/136960)
- [bootstrap], [compiletest], [run-make-support] and [run-make] tests: [Compile `run-make-support` and `run-make` tests with the bootstrap compiler](https://github.com/rust-lang/rust/pull/137373)
- [compiletest] and [run-make] tests: [Prevent `rmake.rs` from using unstable features, and fix 3 run-make tests that currently do](https://github.com/rust-lang/rust/pull/137537)
- [compiletest] and [run-make] tests: [Include `stage0-sysroot` libstd dylib in recipe dylib search path](https://github.com/rust-lang/rust/pull/135389)
- [bootstrap]: [Fix `x test --stage 1 ui-fulldeps` on macOS (until the next beta bump)](https://github.com/rust-lang/rust/pull/136973)
- [bootstrap]: [Add build step log for `run-make-support`](https://github.com/rust-lang/rust/pull/137362)
- [bootstrap]: [Use stage 2 on `cargo` and `clippy` tests when possible](https://github.com/rust-lang/rust/pull/137522)
- CI, [citool]: [Handle empty test suites in GitHub job summary report](https://github.com/rust-lang/rust/pull/138268)

[^goober]: [this person](https://github.com/jieyouxu) is a goober who left a `FIXME` comment to remind themselves to fix this in a follow-up but forgot to follow-up.

### Cleanups

- [compiletest]: [Add erroneous variant to `string_enum`s conversions error](https://github.com/rust-lang/rust/pull/135397)
- [compiletest]: [Cleanup `is_rustdoc` logic and remove a useless path join in `rustdoc-json` runtest logic](https://github.com/rust-lang/rust/pull/136441)
- [compiletest]: [Feed stage number to compiletest directly](https://github.com/rust-lang/rust/pull/136472)
- [compiletest]: [Make the distinction between sources root vs test suite sources root in compiletest less confusing](https://github.com/rust-lang/rust/pull/136474)
- [compiletest]: [Make the distinction between root build directory vs test suite specific build directory in compiletest less confusing](https://github.com/rust-lang/rust/pull/136542)
- [compiletest]: [Retire the legacy `Makefile`-based `run-make` test infra](https://github.com/rust-lang/rust/pull/136581)
- [bootstrap] and [compiletest]: [Use `size_of_val` from the prelude instead of imported](https://github.com/rust-lang/rust/pull/138041)
- [bootstrap]: [Clean up code related to the `rustdoc-js` test suite](https://github.com/rust-lang/rust/pull/135386)
- tests: [Remove generic `//@ ignore-{wasm,wasm32,emscripten}` in tests](https://github.com/rust-lang/rust/pull/136476)


### Documentation updates

Note that since rustc-dev-guide became a josh subtree in [rust-lang/rust], some doc updates are made alongside the [rust-lang/rust] PR themselves.

- CI, [citool]: [Fix docker run-local docs](https://github.com/rust-lang/rust/pull/137946)
- [rustc-dev-guide]: [Document how to find the configuration used in CI](https://github.com/rust-lang/rustc-dev-guide/pull/2205)
- [rustc-dev-guide]: [Fix outdated `rustdoc-js` test suite name](https://github.com/rust-lang/rustc-dev-guide/pull/2212)
- [rustc-dev-guide]: [Rewrite section on executing Docker tests](https://github.com/rust-lang/rustc-dev-guide/pull/2235)
- [rustc-dev-guide]: [Remove "Port run-make tests from Make to Rust" tracking issue from Recurring work](https://github.com/rust-lang/rustc-dev-guide/pull/2242)
- [rustc-dev-guide]: [compiletest directives: `ignore-stage0` and `only-stage0` do not exist](https://github.com/rust-lang/rustc-dev-guide/pull/2272)
- [rustc-dev-guide]: [Clean `--bless` text](https://github.com/rust-lang/rustc-dev-guide/pull/2276)


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
[compiletest]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
[run-make-support]: https://github.com/rust-lang/rust/tree/master/src/tools/run-make-support
[bootstrap]: https://github.com/rust-lang/rust/tree/master/src/bootstrap
[libtest]: https://github.com/rust-lang/rust/tree/master/library/test
[new-issue]: https://github.com/rust-lang/rust/issues/new
[filecheck]: https://llvm.org/docs/CommandGuide/FileCheck.html
[run-make]: https://github.com/rust-lang/rust/tree/master/tests/run-make
[tidy]: https://github.com/rust-lang/rust/tree/master/src/tools/tidy
[library]: https://github.com/rust-lang/rust/tree/master/library
[citool]: https://github.com/rust-lang/rust/tree/master/src/ci/citool
