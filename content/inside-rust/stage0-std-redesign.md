+++
path = "inside-rust/9999/12/31/redesigning-stage-0-std"
title = "Redesigning stage 0 std"
authors = ["Jieyou Xu"]

[extra]
team = "Bootstrap"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

# Redesigning stage 0 std

## Summary

We are reworking how the stage 0 bootstrap sequence works (the sequence used to build a stage 1 compiler).

- Before: a stage 0 (possibly beta) compiler is used to build the in-tree std, which in turn is used to build the stage 1 compiler.
- After: a stage 0 (possibly beta) compiler and a precompiled stage 0 std is instead used to build the stage 1 compiler.

Notably, this means:

- `./x {build,test,check} library --stage 0` becomes no-op, as stage 0 std is no longer the built in-tree std, and the minimum supported stage to build std is now `1`.
    - Consequently, default (test, check, bench) stage values in the library profile are no longer `0`, but instead defaults to `1`.
- `cfg(bootstrap)` is no longer needed for library development. In (very) rare cases, `cfg(bootstrap)` may be needed in the compiler for dogfooding unstable library features.

## Motivation

The previous stage 0 bootstrapping sequence was a source of endless confusion for compiler, library and bootstrap contributors alike, because std had to support being built by *both* a previous possibly-beta rustc and in-tree rustc, with `cfg(bootstrap)` in std sources necessary to distinguish between them. By adjusting the stage 0 bootstrap sequence to instead use a precompiled stage 0 std instead of building the in-tree std, we hope to (1) simplify library development workflow to no longer need `cfg(bootstrap)` and (2) enable simplifying some bootstrap logic related to building in-tree std in stage 0.

This was [originally proposed by @jyn514 in the MCP rust-lang/compiler-team#619](https://github.com/rust-lang/compiler-team/issues/619).

## What does this mean for a typical library workflow?

- Crucially, `./x {build,test,check} library --stage 0` becomes no-op and are no longer supported. Building the in-tree std now requires a stage 1 compiler.
    - Consequently, library contributors are *strongly* encouraged to enable `rust.download-rustc = "if-unchanged"` to avoid having to build a stage 1 compiler. Note that this is the default for `profile = "library"`, but you may need to specify it manually if you don't use a `profile`.
- `cfg(bootstrap)` should no longer be needed for library sources.

### Caveat: `libtest` changes

See the *Recommended config: have `compiletest` instead depend on precompiled stage 0 libtest* section below.

## What does this mean for a typical compiler workflow?

- If unstable library features are being dogfooded, in rare cases `cfg(bootstrap)` may now be needed in compiler sources.

### Caveat: `libtest` changes

See the *Recommended config: have `compiletest` instead depend on precompiled stage 0 libtest* section below.

## Recommended config: have `compiletest` instead depend on precompiled stage 0 libtest

`compiletest` currently depends on in-tree libtest. For workflows that involve building the compiler, this can cause `compiletest` rebuilds if stage 1 library is rebuilt as a consequence of compiler changes. If you don't intend to change libtest, you can [specify `build.compiletest-use-stage0-libtest = true` to instead have `compiletest` depend on precompiled stage 0 libtest](https://github.com/rust-lang/rust/pull/139386). That way, compiler test iterations can avoid rebuilding `compiletest` unnecessarily. This is already the default unless the `dist` profile is used.

Note that some CI jobs have `build.compiletest-use-stage0-libtest = true` set while others have `build.compiletest-use-stage0-libtest = false`, meaning that libtest programmatic API changes can require adding `cfg(bootstrap)`/`cfg(not(bootstrap))` to `compiletest`'s libtest API use sites. However, in practice we expect this to be very rare.
