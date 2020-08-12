---
layout: post
title: "Changes to x.py defaults"
author: Joshua Nelson
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

Recently, the defaults for [x.py] - the tool used to [bootstrap] the rust compiler from source - changed. If you regularly contribute to the rust compiler, this might affect your workflow.

## What changes were made?

- The default stage is now dependent on the subcommand.
  + `dist`: stage 2
  + `build`: stage 1
  + `test`: stage 1
  + `doc`: stage 0

- stage1 `rustc` artifacts are no longer built by `x.py build --stage 1`. To get the old behavior back, use `x.py build --stage 1 src/rustc`.

- `debuginfo` now defaults to `1` when `debug = true`. Previously, the default was 2.

## Why were the changes made?

For a detailed rationale of the changes, as well as more information about the alternatives considered, see

- [the MCP]
- the [implementation PR]
- the [Zulip stream]

[x.py]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#what-is-xpy
[bootstrap]: https://rustc-dev-guide.rust-lang.org/building/bootstrapping.html
[the MCP]: https://github.com/rust-lang/compiler-team/issues/326
[implementation PR]: https://github.com/rust-lang/rust/pull/73964
[Zulip stream]: https://rust-lang.zulipchat.com/#narrow/stream/233931-xxx/topic/Use.20sane.20defaults.20in.20x.py.20compiler-team.23326
