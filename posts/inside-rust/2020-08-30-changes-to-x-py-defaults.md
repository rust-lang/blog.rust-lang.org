+++
layout = "post"
date = 2020-08-30
title = "Changes to x.py defaults"
author = "Jynn Nelson"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++

Recently, the defaults for [`x.py`], the tool used to [bootstrap] the Rust compiler from source, changed. If you regularly contribute to Rust, this might affect your workflow.

## What changes were made?

- The default stage is now dependent on the subcommand:
  + `dist`: stage 2
  + `build`: stage 1
  + `test`: stage 1
  + `doc`: stage 0

- stage 1 `rustc` artifacts are no longer built by `x.py build --stage 1`. To get the old behavior back, use `x.py build --stage 1 src/rustc`. The new behavior for `build --stage 1` builds everything except `rustc`, which includes the standard library, `rustdoc`, and various other tools (if the tools are enabled).

- `debuginfo` now defaults to `1` when `debug = true`. Previously, the default was 2.

## Why were the changes made?

Previously, `x.py build` would build `rustc` twice:

1. `build/stage0-std`
2. `build/stage0-rustc`
3. `build/stage1-std`
4. `build/stage1-rustc`

Normally, contributors only want to build the compiler once, which lets them test their changes quickly. After this change, that's now the default:

1. `build/stage0-std`
2. `build/stage0-rustc`
3. `build/stage1-std`

`debuginfo = 2` generates several gigabytes of debug information,
making the previous default settings for `debug = true` very painful.

For a detailed rationale of the changes, as well as more information about the alternatives considered, see

- [the MCP]
- the [implementation PR]
- the [Zulip stream]

[`x.py`]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#what-is-xpy
[bootstrap]: https://rustc-dev-guide.rust-lang.org/building/bootstrapping.html
[the MCP]: https://github.com/rust-lang/compiler-team/issues/326
[implementation PR]: https://github.com/rust-lang/rust/pull/73964
[Zulip stream]: https://rust-lang.zulipchat.com/#narrow/stream/233931-t-compiler.2Fmajor-changes/topic/Improve.20defaults.20in.20x.2Epy.20compiler-team.23326
