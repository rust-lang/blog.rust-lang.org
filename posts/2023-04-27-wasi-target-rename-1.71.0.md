---
layout: post
title: "Renaming the WASI target in Rust 1.71.0"
author: Yoshua Wuyts
release: false
---

In Rust 1.71 the existing `wasm32-wasi` target will be renamed to
`wasm32-wasi-preview1`. This matches the naming used in the [WASI Preview
Framework][wpf] and more accurately captures the evolving nature of the WASI
targets. By making this change we also prepare Rust to re-adopt the
`wasm32-wasi` target name for the eventual stable WASI 1.0 release.

[wpf]: https://github.com/WebAssembly/meetings/blob/main/wasi/2023/presentations/2023-02-09-gohman-wasi-roadmap.pdf

Work on adding a WASI Preview 2 target to the compiler is currently underway,
but is not yet complete. We expect this to land in a future release of Rust. The
target triple for this target will be `wasm32-wasi-preview2`.

## Migrating Targets

To upgrade to the new WASI target on Rust 1.71 nightly you can run the
following commands using [rustup](https://rustup.rs):

```bash
$ rustup +nightly target remove wasm32-wasi
$ rustup upgrade nightly
$ rustup +nightly target add wasm32-wasi-preview1
```

Once beta and stable builds are available (expected in ~6 and ~12 weeks
respectively) you should be able to replace the word `nightly` with either
`beta` or `stable` in the instructions.
