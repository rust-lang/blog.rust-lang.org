+++
path = "inside-rust/2024/10/10/test-infra-oct-2024"
title = "This Month in Our Test Infra: September 2024"
authors = ["Jieyou Xu"]
aliases = ["inside-rust/2024/10/10/test-infra-oct-2024.html"]

[extra]
team = "the Bootstrap Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

# This Month in Our Test Infra: September 2024

This is a quick summary of the changes in the test infrastructure for the
[rust-lang/rust][rust] repository[^scope] for September 2024. It also includes
brief descriptions of on-going work.

[^scope]: The test infra here refers to the test harness [compiletest] and
supporting components in our build system [bootstrap]. This test infra is used
mainly by rustc and rustdoc. Other tools like cargo, miri or rustfmt maintain
their own test infra.

As usual, if you encounter bugs or UX issues when using our test infrastrucutre,
please [file an issue][new-issue]. Bugs and papercuts can't be fixed if we don't
know about them!

## Changes

### `run-make` test suite now has access to a properly-staged cargo

[bootstrap] now builds a properly-staged cargo and makes it available for
`run-make` tests. Previously, `run-make` tests just used whatever initial cargo
[bootstrap] had access to, but this caused problems if a `run-make` test uses a
cargo feature that's present in nightly but not in beta. We encountered some
interesting build cache invalidation issues related to differing `RUSTFLAGS` in
the process, but were able to fix them. We want to add mechanisms to [bootstrap]
to make it harder to misuse `RUSTFLAGS` which may lead to hard-to-diagnose build
cache invalidation in the future.

- bootstrap: [Fix cargo staging for run-make tests #130739](https://github.com/rust-lang/rust/pull/130739)
- bootstrap/compiletest: [Pass the current cargo to run-make tests #130642](https://github.com/rust-lang/rust/pull/130642)
- bootstrap: [Prevent building cargo from invalidating build cache of other
  tools due to conditionally applied `-Zon-broken-pipe=kill` via tracked
  `RUSTFLAGS` #131155](https://github.com/rust-lang/rust/pull/131155)

Why `-Zon-broken-pipe=kill` is required when building rustc was its [own entire
rabbit hole](https://github.com/rust-lang/rust/issues/131436), in case you were
curious.

### More `run-make` migrations and fixes

The `emit-to-stdout` `run-make` test was ported to `rmake.rs`, only [10 more to
go]. The remaining ones are stuck on being quite tricky. See the [tracking
issue][port-run-make] for why we are transitioning away from `Makefile`s in
`run-make` tests.

- run-make: [port `emit-to-stdout` to rmake.rs #131355](https://github.com/rust-lang/rust/pull/131355)

Misc:

- run-make: [Add missing `needs-llvm-components` directives for run-make tests
  that need target-specific codegen
  #129605](https://github.com/rust-lang/rust/pull/129605)

[remaining-run-make-tests]: https://github.com/rust-lang/rust/blob/883f9a2c8f8923eafafbeba8b18361424b148f05/src/tools/tidy/src/allowed_run_make_makefiles.txt#L1C1-L10C30
[port-run-make]: https://github.com/rust-lang/rust/issues/121876

### `run_make_support` library updates

`run_make_support` is the support library built and made available to `run-make`
tests.

- run_make_support: [Add `llvm-pdbutil` wrapper #130048](https://github.com/rust-lang/rust/pull/130048)
- run_make_support: [Rename `Command::stdin` to `stdin_buf` and add
  `std{in,out,err}` config helpers
  #129973](https://github.com/rust-lang/rust/pull/129973)
- run_make_support: [Rectify symlink handling
  #130427](https://github.com/rust-lang/rust/pull/130427)

### [compiletest] improvements and fixes

We dropped [compiletest]'s legacy directive check (e.g. `// ignore-test hello`
no longer warns). This was originally added when we migrated from `//` to `//@`
to help test writers notice the new directives, but now a long time has passed
so we can remove it as it was causing friction in adding new directives and
authoring tests. For example, the [Specification Team][t-spec] wanted to add a
`//@ reference` directive, but the legacy directive check would trigger on:

```rust,no_run
// So what if we added a
// reference to the           <- `reference` is a known directive, and
//                               `// reference` looks suspcious!
// rustc-dev-guide?
```

This was added to initially to help migration from `//` to `//@`, but since a
long time has passed we no longer need this check to help contributors know that
legacy directives are being phased out.

- compiletest: [Drop compiletest legacy directive check
  #131392](https://github.com/rust-lang/rust/pull/131392)

[t-spec]: https://www.rust-lang.org/governance/teams/lang#team-spec

We updated some `compiletest` normalizations and directive renaming. In
particular, we restricted `//@ ignore-mode-*` directives to not accept *all*
test suites, and later converted `//@ ignore-mode-coverage-map` and `//@
ignore-mode-coverage-run` to `//@ ignore-coverage-map` and `//@
ignore-coverage-run` because only `coverage-map` and `coverage-run` were special
in that the same test source files ran under two test suite configurations.

- compiletest: [Add `{{rust-src-base}}` (for sysroot src base)
  #129687](https://github.com/rust-lang/rust/pull/129687)
- compiletest: [Restrict `ignore-mode-*` directives
  #131346](https://github.com/rust-lang/rust/pull/131346)
- compiletest: [Simplify the compiletest directives for ignoring coverage-test
  modes #131400](https://github.com/rust-lang/rust/pull/131400)

We broke up [compiletest]'s `runtest.rs` as it was [previously
*massive*][prev-runtest], clocking in at 4710 lines. It's now around 2700 lines,
so still massive, but at least slightly less so.

- compiletest: [Break up compiletest `runtest.rs` into smaller helper modules
  #130566](https://github.com/rust-lang/rust/pull/130566)

[prev-runtest]: https://github.com/rust-lang/rust/blob/b7b9453ea7354ee39b15390ffd0b4f9e2000076b/src/tools/compiletest/src/runtest.rs

We added a help message upon `crashes` test failure that you can set
`COMPILETEST_VERBOSE_CRASHES=1` to get compiler stderr/stdout output from trying
to build the failing `crashes` test.

- compiletest: [Mention `COMPILETEST_VERBOSE_CRASHES` on crashes test failure
  #130793](https://github.com/rust-lang/rust/pull/130793)

We also registered [tool docs][compiletest-tool-docs] for [compiletest]. There
currently isn't much doc comments in [compiletest], but having them getting
built and made available as part of nightly rustc docs is a good first step.

- bootstrap: [Register tool docs for compiletest
  #130567](https://github.com/rust-lang/rust/pull/130567)

[compiletest-tool-docs]: https://doc.rust-lang.org/nightly/nightly-rustc/compiletest/index.html

Misc:

- compiletest: [Rename "runtest/crash.rs" to "runtest/crashes.rs" to be in line
  with the test directory
  #130973](https://github.com/rust-lang/rust/pull/130973)

### Testing documentation improvements

We improved testing docs in [rustc-dev-guide][dev-guide-testing]. We added a
[testing best practices chapter][dev-guide-testing-best-practices], and updated
the [compiletest directives listing][dev-guide-directives-listing].

- rustc-dev-guide: [Revise testing chapters excluding the directives chapter
  #2088](https://github.com/rust-lang/rustc-dev-guide/pull/2088)
- rustc-dev-guide: [Revise directives docs
  #2089](https://github.com/rust-lang/rustc-dev-guide/pull/2089)
- rustc-dev-guide: [Revise test naming advice to discourage using issue numbers
  alone #2090](https://github.com/rust-lang/rustc-dev-guide/pull/2090)
- rustc-dev-guide: [Document compiletest directives `ignore-coverage-map` and
  `ignore-coverage-run`
  #2094](https://github.com/rust-lang/rustc-dev-guide/pull/2094)
- rustc-dev-guide: [Small follow-up to my "internal #[rustc_*] TEST attributes"
  PR #2082](https://github.com/rust-lang/rustc-dev-guide/pull/2082)
- rustc-dev-guide: [Note lldb debuginfo requires `python310.dll` to be present
  in `PATH` on Windows
  #2076](https://github.com/rust-lang/rustc-dev-guide/pull/2076)
- rustc-dev-guide: [Document crashes test suite
  #2075](https://github.com/rust-lang/rustc-dev-guide/pull/2075)
- rustc-dev-guide: [Purge `run-pass-valgrind mentions`
  #2091](https://github.com/rust-lang/rustc-dev-guide/pull/2091)
- rustc-dev-guide: [Add documentation for `{{rust-src-base}}`
  #2079](https://github.com/rust-lang/rustc-dev-guide/pull/2079)

There's still a lot of room for improvement in our testing docs -- in
[compiletest], [bootstrap] and [rustc-dev-guide], but one step at a time.

[dev-guide-testing]: https://rustc-dev-guide.rust-lang.org/tests/intro.html
[dev-guide-testing-best-practices]: https://rustc-dev-guide.rust-lang.org/tests/best-practices.html
[dev-guide-directives-listing]: https://rustc-dev-guide.rust-lang.org/tests/directives.html

### Test suite cleanups

We deleted an entire test suite `run-pass-valgrind` because it was never
properly wired up and properly implemented, and was not used. It turns out
deleting the test suite actually fixes a bug from 2017 [run-pass-valgrind tests
don't actually run in valgrind
#44816](https://github.com/rust-lang/rust/issues/44816) because you can't have a
test suite related bug if the test suite doesn't exist!

- bootstrap/compiletest/opt-dist: [Remove valgrind test suite and support from
  compiletest, bootstrap and opt-dist
  #131351](https://github.com/rust-lang/rust/pull/131351)

Misc:

- rustdoc: [Rename `issue-\d+.rs` tests to have meaningful names (part 9)
  #130287](https://github.com/rust-lang/rust/pull/130287)

## On-going developments

- Add test infrastructure support for a `minicore` test auxiliary, so that
  `#![no_std]` cross-compiling build-only tests don't need to reinvent and
  reimplement `core` prelude stubs again and again. See
  <https://github.com/rust-lang/rust/issues/131485>.
- We want to make `RUSTFLAGS` harder to misuse that can lead to tool build cache
  invalidation, leading to unnecessary rebuilds.
- There's on-going effort to redesign stage0 std, to help make [bootstrap]
  staging more consistent and more intuitive.

[rust]: https://github.com/rust-lang/rust
[compiletest]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
[bootstrap]: https://github.com/rust-lang/rust/tree/master/src/bootstrap
[new-issue]: https://github.com/rust-lang/rust/issues/new
[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/getting-started.html
