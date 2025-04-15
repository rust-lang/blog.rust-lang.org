+++
path = "inside-rust/2024/11/04/test-infra-oct-2024-2"
title = "This Month in Our Test Infra: October 2024"
authors = ["Jieyou Xu"]
aliases = ["inside-rust/2024/11/04/test-infra-oct-2024-2.html"]

[extra]
team = "the Bootstrap Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

# This Month in Our Test Infra: October 2024

<!-- time period: 2024-10-09 through 2024-11-04 -->

This is a quick summary of the changes in the test infrastructure for the [rust-lang/rust][r-l/r]
repository[^scope] for **October 2024**[^spec-date]. It also includes brief descriptions of on-going
work.

[^scope]: The test infra here refers to the test harness [compiletest] and supporting components in
our build system [bootstrap]. This test infra is used mainly by rustc and rustdoc. Other tools like
cargo, miri or rustfmt maintain their own test infra.
[^spec-date]: specifically 2024-10-09 to 2024-11-04. Note that the previous issue incorrectly used
    October in the filename and thus URL, but is actually the September issue.

As usual, if you encounter bugs or UX issues when using our test infrastructure, please [file an
issue][new-issue]. Bugs and papercuts can't be fixed if we don't know about them!

**Thanks to everyone who contributed to our test infra!**

## Highlights

### `compiletest` now supports bringing your own custom diff tool

[`compiletest` (and bootstrap) now supports bringing your own custom diff tool
(#131181)][custom-diff-tool].

![A ui test failure showing stderr snapshot diff produced by a custom differ](compiletest-custom-differ.png)

This only affects the *visual* diff generation (i.e. maybe you like a different visual style). The
`.stderr` snapshots and such are not affected.

If you want to use your favorite diff tool for generating the visual diffs, you can modify
`config.toml`'s `build.compiletest-diff-tool` option:

```toml
[build]
# What custom diff tool to use for displaying compiletest tests.
#compiletest-diff-tool = <none>
```

Thanks to [`@dev-ardi`] for the implementation!

[`@dev-ardi`]: https://github.com/dev-ardi
[custom-diff-tool]: https://github.com/rust-lang/rust/pull/131181

### `minicore` test auxiliary and `//@ add-core-stubs` directive 

[`ui`, `assembly` and `codegen` tests can now use the `//@ add-core-stubs` directive to
conditionally build a `minicore` test auxiliary which provides `core` stubs
(#130693)](https://github.com/rust-lang/rust/pull/130693). This is so that we can share `core` stubs
between cross-compiling tests that only need to *build* for the cross-compile target and avoid
having to reinvent and maintain duplicate `minicore` copies in each of such test[^multicore].

[^multicore]: You can say we currently have more of a... "multicore" situation, heh.

Previously, having to reinvent a `minicore` every time you want to add a distinct
`ui`/`assembly`/`codegen` test (for checking e.g. cross-compile ABI) is a significant source of
contributor friction and makes it more prone to introduce mistakes in the `minicore` copies. This is
also particularly annoying for compiler contributors who want to introduce new lang items, as they
found themselves having to update multiple copies of such `core` stubs.

Note that currently, the shared [`tests/auxiliary/minicore.rs`][minicore] test auxiliary is still
quite minimal. The plan is to land the test infrastructure first, then we can incrementally add more
`core` stubs to the shared test auxiliary.

Example usage:

```rs
// tests/ui/abi/my-abi-test.rs

//@ add-core-stubs
//@ compile-flags: --target x86_64-pc-windows-msvc
//@ needs-llvm-components: x86

#![crate_type = "lib"]
#![feature(no_core)]
#![no_std]
#![no_core]

extern crate minicore;
use minicore::*;

struct Meow;
impl Copy for Meow {} // `Copy` is provided by `minicore`!
```

See the [context issue][minicore-ctxt], [MCP][minicore-mcp] and [tracking issue][minicore-tracking]
for more info on the original motivations. See the [rustc-dev-guide][minicore-dev-guide] chapter for
example usage.

[minicore]: https://github.com/rust-lang/rust/blob/master/tests/auxiliary/minicore.rs
[minicore-ctxt]: https://github.com/rust-lang/rust/issues/130375
[minicore-tracking]: https://github.com/rust-lang/rust/issues/131485
[minicore-mcp]: https://github.com/rust-lang/compiler-team/issues/786
[minicore-dev-guide]: https://rustc-dev-guide.rust-lang.org/tests/minicore.html

## PR listing

### Improvements

- General test infra: [Add `minicore` test auxiliary and support `//@ add-core-stubs` directive in `ui`/`assembly`/`codegen` tests #130693](https://github.com/rust-lang/rust/pull/130693)
- [compiletest]: [Add test infra to explicitly test rustc with `autodiff`/`enzyme` disabled #131470](https://github.com/rust-lang/rust/pull/131470)
- [compiletest]: [Special case error message for a `build-fail` test that failed check build #131642](https://github.com/rust-lang/rust/pull/131642)
- [compiletest]: [Document various parts of compiletest's lib.rs #131679](https://github.com/rust-lang/rust/pull/131679)
- [compiletest]: [Fix unnecessary nesting in run-make test output directories #131764](https://github.com/rust-lang/rust/pull/131764)
- [compiletest]: [Warn on redundant --cfg directive when revisions are used #131925](https://github.com/rust-lang/rust/pull/131925)
- [compiletest]: [Disambiguate html-tidy from rust tidy tool #131941](https://github.com/rust-lang/rust/pull/131941)
- [compiletest]: [Custom differ #131181](https://github.com/rust-lang/rust/pull/131181)
- [compiletest]: [Don't allow test revisions that conflict with built in `cfg`s #131930](https://github.com/rust-lang/rust/pull/131930)
- [compiletest]: [Dynamically link run-make support #132225](https://github.com/rust-lang/rust/pull/132225)
- [compiletest]: [Improve robustness of LLVM version handling #132315](https://github.com/rust-lang/rust/pull/132315)
- [compiletest]: [Add "reference" as a known compiletest header #131382](https://github.com/rust-lang/rust/pull/131382)[^spec]
- `tests/run-make`, CI: [Add `aarch64-gnu-debug` job #131207](https://github.com/rust-lang/rust/pull/131207)
- meta: [Tag PRs affecting compiletest with A-compiletest #131682](https://github.com/rust-lang/rust/pull/131682)

[^spec]: This is part of T-spec efforts to associate tests with Reference rules.

### Fixes

- [compiletest]: [Fix up-to-date checking for run-make tests #131681](https://github.com/rust-lang/rust/pull/131681)
- [compiletest]: [Suppress Windows Error Reporting (WER) for run-make tests](https://github.com/rust-lang/rust/pull/132093)[^wer-fun]
- [compiletest]: [Error on trying to use revisions in run-make tests #131614](https://github.com/rust-lang/rust/pull/131614)
- `tests/run-make`, CI: [Run the full stage 2 `run-make` test suite in `x86_64-gnu-debug` #131917](https://github.com/rust-lang/rust/pull/131917)
- [bootstrap], `tests/run-make`: [Don't stage-off to previous compiler when CI rustc is available #132006](https://github.com/rust-lang/rust/pull/132006)
- [bootstrap], `tests/mir-opt`: [Match std `RUSTFLAGS` for host and target for `mir-opt` test suite to fix double std build/rebuilds #131442](https://github.com/rust-lang/rust/pull/131442) 
- emscripten: Fix [bootstrap] and [compiletest] handling of emscripten target tests as part of [Fix most ui tests on emscripten target #131705](https://github.com/rust-lang/rust/pull/131705)

[^wer-fun]: If you want to see what unsuppressed Windows Errors Reporting looks like for the `run-make` test suite, see <https://github.com/rust-lang/rust/issues/132092#issuecomment-2436615833>.

### Cleanups

- [compiletest]: [Extract auxiliary-crate properties to their own module/struct #131541](https://github.com/rust-lang/rust/pull/131541)
- [compiletest]: [Rename directive `needs-profiler-support` to `needs-profiler-runtime` #131429](https://github.com/rust-lang/rust/pull/131429)
- [compiletest]: [Move debugger setup code out of lib.rs #131638](https://github.com/rust-lang/rust/pull/131638)
- [compiletest]: [Remove the one thing that was checking a directive's original_line #131585](https://github.com/rust-lang/rust/pull/131585)
- [compiletest]: [Store test collection context/state in two structs #131870](https://github.com/rust-lang/rust/pull/131870)
- [compiletest]: [Tidy up how tidy and tidy (html version) are disambiguated #131961](https://github.com/rust-lang/rust/pull/131961)
- [compiletest]: [Make `line_directive` return a `DirectiveLine` #132033](https://github.com/rust-lang/rust/pull/132033)
- [compiletest]: [Rename `command-list.rs` to `directive-list.rs` #132313](https://github.com/rust-lang/rust/pull/132313)
- [compiletest]: [Remove the magic hacks for finding output with `lto=thin` #131524](https://github.com/rust-lang/rust/pull/131524)
- [compiletest]: [Simplify the choice of `--emit` mode for assembly tests #131525](https://github.com/rust-lang/rust/pull/131525)
- [compiletest]: [Move debugger setup code out of lib.rs #131638](https://github.com/rust-lang/rust/pull/131638)

### Documentation updates

- [rustc-dev-guide]: [Document compiletest directives `ignore-coverage-map` and `ignore-coverage-run` #2094](https://github.com/rust-lang/rustc-dev-guide/pull/2094)
- [rustc-dev-guide]: [Rename `needs-profiler-support` to `needs-profiler-runtime` #2095](https://github.com/rust-lang/rustc-dev-guide/pull/2095)
- [rustc-dev-guide]: [Fix and update docs for `needs-force-clang-based-tests` #2085](https://github.com/rust-lang/rustc-dev-guide/pull/2085)
- [rustc-dev-guide]: [Add redirects for integration-testing and headers #2092](https://github.com/rust-lang/rustc-dev-guide/pull/2092)
- [rustc-dev-guide]: [Describe minicore test auxiliary and directive #2097](https://github.com/rust-lang/rustc-dev-guide/pull/2097)
- [rustc-dev-guide]: [Fix minicore.rs link #2122](https://github.com/rust-lang/rustc-dev-guide/pull/2122)
- [rustc-dev-guide]: [Add a link for the `reference` compiletest header #2096](https://github.com/rust-lang/rustc-dev-guide/pull/2096)

## On-going efforts

Note: there are certainly many more spontaneous efforts, this is more what I know is "planned".

- [Reworking `compiletest` directive handling to be more robust and improve test coverage](https://rust-lang.zulipchat.com/#narrow/channel/326414-t-infra.2Fbootstrap/topic/.28Rubberducking.29.20compiletest.20test.20discovery.20.2F.20directives)


[r-l/r]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
[compiletest]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
[bootstrap]: https://github.com/rust-lang/rust/tree/master/src/bootstrap
[new-issue]: https://github.com/rust-lang/rust/issues/new
