+++
path = "inside-rust/2024/12/09/test-infra-nov-2024"
title = "This Month in Our Test Infra: November 2024"
authors = ["Jieyou Xu"]
aliases = ["inside-rust/2024/12/09/test-infra-nov-2024.html"]

[extra]
team = "the Bootstrap Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

# This Month in Our Test Infra: November 2024

<!-- time period: 2024-11-05 through 2024-12-03 -->

This is a quick summary of the changes in the test infrastructure for the
[rust-lang/rust][r-l/r] repository[^scope] for **November 2024**. It also
includes brief descriptions of on-going work.

[^scope]: The test infra here refers to the test harness [compiletest] and
supporting components in our build system [bootstrap]. This test infra is used
mainly by rustc and rustdoc. Other tools like cargo, miri or rustfmt maintain
their own test infra.

As usual, if you encounter bugs or UX issues when using our test infrastructure,
please [file an issue][new-issue]. Bugs and papercuts can't be fixed if we don't
know about them!

**Thanks to everyone who contributed to our test infra!**

## Highlights

### compiletest: Add `proc-macro` auxiliary build directive

[@ehuss](https://github.com/ehuss) added a `//@ proc-macro` directive that
behaves like `//@ aux-build`, but it packages the usual `//@ force-host` and
`//@ no-prefer-dynamic` boilerplate that previously was needed by proc-macro
auxiliaries. If the main test file also uses a sufficiently new edition (i.e.
Edition 2018 onwards), the proc-macro auxiliary is also made available via
extern prelude.

**Before**: test writer need to write `//@ force-host` and `//@
no-prefer-dynamic` for each and every proc-macro auxiliary.

```rs
// tests/ui/foo/my-main-test.rs
//@ aux-build: my-proc-macro.rs
```

```rs
// tests/ui/foo/auxiliary/my-proc-macro.rs
//@ no-prefer-dynamic
//@ force-host
```

**After**: only `//@ proc-macro` directive is needed in main test file.

```rs
// tests/ui/foo/my-main-test.rs
//@ proc-macro: my-proc-macro.rs
```

```rs
// tests/ui/foo/auxiliary/my-proc-macro.rs
```

Thanks Eric!

### rustc: make `rustc` consider itself a stable compiler when `RUSTC_BOOTSTRAP=-1` is set

In [#132993](https://github.com/rust-lang/rust/pull/132993), I modified
`rustc`'s stability checking logic to also now recognize `RUSTC_BOOTSTRAP=-1` to
force any `rustc` to consider itself a stable compiler, regardless of which
channel it is from (e.g. beta or dev or nightly or stable)[^disclaimer]. This is
useful for e.g. diagnostics that differ between nightly and stable, and also
provides a way to make the `rustc` under test behave *as if* it was a stable
compiler. 

[^disclaimer]: This is *only* for internal testing usages. Anything else that
    relies on this that breaks will be considered PEBKAC.

In tests, the `//@ rustc-env` directive may be used with
`RUSTC_BOOTSTRAP=-1`[^known-bug].

[^known-bug]: The `//@ rustc-env` directive handling has a bug where it's
    white-space sensitive between the colon and the value, so avoid whitespace
    for now.

```rs
//@ rustc-env:RUSTC_BOOTSTRAP=-1
//@ compile-flags: -Z unstable-options
//@ regex-error-pattern: error: the option `Z` is only accepted on the nightly compiler
// This will fail because the `rustc` under test rejects the `-Z unstable-options` unstable flag.
```

## PR listing

### Improvements

- [compiletest]:
    - [Add `{ignore,needs}-{rustc,std}-debug-assertions` directive support #131913](https://github.com/rust-lang/rust/pull/131913).
    - [Add `max-llvm-major-version` directive #132310](https://github.com/rust-lang/rust/pull/132310)
    - [Add AIX run-make support #132657](https://github.com/rust-lang/rust/pull/132657)
    - [Add `exact-llvm-major-version` directive #132995](https://github.com/rust-lang/rust/pull/132995)
    - [Add `proc-macro` directive #133540](https://github.com/rust-lang/rust/pull/133540)
- rustc:
    - [Make rustc consider itself a stable compiler when `RUSTC_BOOTSTRAP=-1` #132993](https://github.com/rust-lang/rust/pull/132993)

### Cleanups

- [compiletest]:
    - [Delete `//@ pretty-expanded` directive #133470](https://github.com/rust-lang/rust/pull/133470)

### Documentation updates

- [rustc-dev-guide]:
    - [Update `//@ proc-macro` aux build directive docs #2149](https://github.com/rust-lang/rustc-dev-guide/pull/2149)
    - [Remove `pretty-expanded` as it no longer exists #2147](https://github.com/rust-lang/rustc-dev-guide/pull/2147)
    - [Add instructions to test error code docs #2145](https://github.com/rust-lang/rustc-dev-guide/pull/2145)
    - [Document how to acquire cdb.exe #2137](https://github.com/rust-lang/rustc-dev-guide/pull/2137)
    - [Mention `RUSTC_BOOTSTRAP` for misc testing #2136](https://github.com/rust-lang/rustc-dev-guide/pull/2136)
    - [Document `exact-llvm-major-version` directive #2135](https://github.com/rust-lang/rustc-dev-guide/pull/2135)
    - [Document `max-llvm-major-version` directive #2129](https://github.com/rust-lang/rustc-dev-guide/pull/2129)
    - [Rename `{ignore,only}-debug` -> `{ignore,needs}-{rustc,std}-debug-assertions` #2101](https://github.com/rust-lang/rustc-dev-guide/pull/2101)

## On-going efforts

Note: there are certainly many more spontaneous efforts, this is more what I
know is "planned".

- [Proposed a 2025H1 project goal: compiletest directive handling rework #148](https://github.com/rust-lang/rust-project-goals/pull/148)


[r-l/r]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
[compiletest]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
[bootstrap]: https://github.com/rust-lang/rust/tree/master/src/bootstrap
[new-issue]: https://github.com/rust-lang/rust/issues/new
