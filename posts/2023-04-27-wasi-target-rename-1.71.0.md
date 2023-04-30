---
layout: post
title: "Renaming the WASI target in Rust 1.71.0"
author: Yoshua Wuyts
release: false
---
In Rust 1.71 the existing `wasm32-wasi` target will be renamed
to `wasm32-wasi-preview1`. This matches the naming introduced in the [WASI
Preview Framework][wpf], whose scheme has already been adopted by other
language toolchains such as [Golang] and [Node.js]. It is expected that
eventually WASI will eventually stabilize far enough that it will reach a 1.0
status. By freeing up the `wasm32-wasi` target name now, it enables us to
reintroduce it in the future as the target name for an eventual 1.0 stable WASI
release.

While we don’t have any immediate plans to deprecate the Preview 1 target, the
reality is that most of the development of WASI in recent years has centered
around WASI Preview 2 and [Wasm Components][components]. The expectation is that
Preview 2 will eventually supplant Preview 1, and support across the WASI
ecosystem will gradually wind down for Preview 1. By adopting “preview” in the
target names, we can begin to more clearly communicate that the current WASI
targets should be considered development snapshots, and will eventually be
superseded.

Work on adding a WASI Preview 2 target to the compiler is currently underway,
but is not yet complete. We expect this to land in a future release of Rust. The
target triple for this target will be `wasm32-wasi-preview2`.

## Migrating Targets

To upgrade to the new WASI target on Rust 1.71 nightly you can run the
following commands using [rustup](https://rustup.rs):

```bash
$ rustup +nightly target remove wasm32-wasi
$ rustup upgrade nightly
$ rustup +nightly target add wasm32-wasi-preview1
```

Once beta and stable builds are available (expected in ~6 and ~12 weeks
respectively) you should be able to replace the word `nightly` in the
instructions with either `beta` or `stable`.

[Golang]: https://github.com/golang/go/issues/58141
[Node.js]: https://nodejs.org/en/blog/announcements/v20-release-announce#progress-on-web-assembly-system-interface-wasi
[wpf]: https://github.com/WebAssembly/meetings/blob/main/wasi/2023/presentations/2023-02-09-gohman-wasi-roadmap.pdf
[components]: https://www.youtube.com/watch?v=phodPLY8zNE
