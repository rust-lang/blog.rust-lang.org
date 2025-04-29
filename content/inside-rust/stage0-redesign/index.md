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

- Before: a stage 0 (beta by default) compiler is used to build the in-tree std, which in turn is used to build the stage 1 compiler.
- After: a stage 0 (beta by default) compiler and a precompiled stage 0 std is instead used to build the stage 1 compiler.

![Difference in stage 0 bootstrap sequence](stage0-redesign-diff.svg)

Notably, this means that after [redesign stage 0 std #119899](https://github.com/rust-lang/rust/pull/119899) PR lands:

- `./x {build,test,check} library --stage 0` becomes no-op, as stage 0 std is no longer the built in-tree std, and the minimum supported stage to build std is now `1`.
    - Consequently, default (test, check, bench) stage values in the library profile are no longer `0`, but instead defaults to `1`.
- Some additional `cfg(bootstrap)` usages may be needed in the compiler sources for dogfooding unstable library features.

## Comparison of common invocations

For `profile = "library"`:

| Invocation                                 | Before stage 0 std redesign                                 | After stage 0 std redesign                               |
|--------------------------------------------|-------------------------------------------------------------|----------------------------------------------------------|
| `./x {check,build,test} library`           | Checks/builds/tests in-tree library                         | Check/build/tests stage 1 library                        |
| `./x {check,build,test} library --stage 0` | Checks/builds/tests in-tree library                         | No-op in case of build, checks/tests precompiled library |
| `./x {check,build,test} library --stage 1` | Builds in-tree library, checks/builds/tests stage 1 library | Check/build/tests stage 1 library                        |

For `profile = "compiler"`:

| Invocation                                 | Before stage 0 std redesign                                 | After stage 0 std redesign                               |
|--------------------------------------------|-------------------------------------------------------------|----------------------------------------------------------|
| `./x check library`                        | Checks in-tree library                                      | Check/build/tests stage 1 library                        |
| `./x {build,test} library`                 | Builds in-tree library, builds/tests stage 1 library        | Check/build/tests stage 1 library                        |
| `./x {check,build,test} library --stage 0` | Checks/builds/tests in-tree library                         | No-op in case of build, checks/tests precompiled library |
| `./x {check,build,test} library --stage 1` | Builds in-tree library, checks/builds/tests stage 1 library | Check/build/tests stage 1 library                        |

For `profile = "tools"`, by default not affected if `download-rustc` is enabled.

## What does this mean for a typical library workflow?

- Crucially, `./x {build,test,check} library --stage 0` becomes no-op and are no longer supported. Building the in-tree std now requires a stage 1 compiler.
    - Consequently, library contributors are *strongly* encouraged to enable `rust.download-rustc = "if-unchanged"` to avoid having to build a stage 1 compiler. Note that this is the default for `profile = "library"`, but you may need to specify it manually if you don't use a `profile`.
- `cfg(bootstrap)` should no longer be needed for library sources.

## What does this mean for a typical compiler workflow?

- If unstable library features are being dogfooded, some additional `cfg(bootstrap)` usages may now be needed in compiler sources.

## Why are we making this change?

The previous stage 0 bootstrapping sequence was a source of endless confusion for compiler, library and bootstrap contributors alike, because std had to support being built by *both* a previous beta rustc and in-tree rustc, with `cfg(bootstrap)` in std sources necessary to distinguish between them. By adjusting the stage 0 bootstrap sequence to instead use a precompiled stage 0 std instead of building the in-tree std, we hope to:

1. Simplify library development workflow to no longer need `cfg(bootstrap)`, and
2. Enable simplifying some bootstrap logic related to building in-tree std in stage 0.

This was [originally proposed by @jyn514 in the MCP rust-lang/compiler-team#619](https://github.com/rust-lang/compiler-team/issues/619).
