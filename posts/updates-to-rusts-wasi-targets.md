+++
layout = "post"
date = 2024-04-09
title = "Changes to Rust's WASI targets"
author = "Yosh Wuyts"
+++

[WASI 0.2 was recently
stabilized](https://bytecodealliance.org/articles/WASI-0.2), and Rust has begun
implementing first-class support for it in the form of a dedicated new target.
Rust 1.78 will introduce new `wasm32-wasip1` (tier 2) and `wasm32-wasip2` (tier
3) targets. `wasm32-wasip1` is an effective rename of the existing `wasm32-wasi`
target, freeing the target name up for an eventual WASI 1.0 release. **Starting
Rust 1.78 (May 2nd, 2024), users of WASI 0.1 are encouraged to begin migrating
to the new `wasm32-wasip1` target before the existing `wasm32-wasi` target is
removed in Rust 1.84 (January 5th, 2025).**

In this post we'll discuss the introduction of the new targets, the motivation
behind it, what that means for the existing WASI targets, and a detailed
schedule for these changes. This post is about the WASI targets only; the
existing `wasm32-unknown-unknown` and `wasm32-unknown-emscripten` targets are
unaffected by any changes in this post.

## Introducing `wasm32-wasip2`

After nearly five years of work the [WASI 0.2 specification](https://wasi.dev)
was recently stabilized. This work builds on [WebAssembly
Components](https://component-model.bytecodealliance.org) (think: strongly-typed
ABI for Wasm), providing standard interfaces for things like asynchronous IO,
networking, and HTTP. This will finally make it possible to write asynchronous
networked services on top of WASI, something which wasn't possible using WASI
0.1.

People interested in compiling Rust code to WASI 0.2 today are able to do so
using the [cargo-component](https://github.com/bytecodealliance/cargo-component)
tool. This tool is able to take WASI 0.1 binaries, and transform them to WASI 0.2
Components using a shim. It also provides native support for common cargo
commands such as `cargo build`, `cargo test`, and `cargo run`. While it
introduces some inefficiencies because of the additional translation layer, in
practice this already works really well and people should be able to get
started with WASI 0.2 development.

We're however keen to begin making that translation layer obsolete. And for
that reason we're happy to share that Rust has made its first steps towards
that with the introduction of the [tier
3](https://doc.rust-lang.org/rustc/platform-support.html#tier-3) `wasm32-wasip2`
target landing in Rust 1.78. **This will initially miss a lot of expected**
**features such as stdlib support, and we don't recommend people use this target**
**quite yet.** But as we fill in those missing features over the coming months, we
aim to eventually meet the criteria to become a tier 2 target, at which
point the `wasm32-wasip2` target would be considered ready for general use. This
work will happen through 2024, and we expect for this to land before the end of
the calendar year.

## Renaming `wasm32-wasi` to `wasm32-wasip1`

The original name for what we now call WASI 0.1 was "WebAssembly System
Interface, snapshot 1". Rust shipped support for this in 2019, and we did so
knowing the target would likely undergo significant changes in the future. With
the knowledge we have today though, we would not have chosen to introduce the
"WASI, snapshot 1" target as `wasm32-wasi`. We should have instead chosen to add
some suffix to the initial target triple so that the eventual stable WASI 1.0
target can just be called `wasm32-wasi`.

In anticipation of both an eventual WASI 1.0 target, and to preserve consistency
between target names, we'll begin rolling out a name change to the existing WASI
0.1 target. Starting in Rust 1.78 (May 2nd, 2024) a new `wasm32-wasip1` target
will become available. Starting Rust 1.81 (September 5th, 2024) we will begin
warning existing users of `wasm32-wasi` to migrate to `wasm32-wasip1`. And
finally in Rust 1.84 (January 9th, 2025) the `wasm32-wasi` target will no longer
be shipped on the stable release channel. This will provide an 8 month
transition period for projects to switch to the new target name when they update
their Rust toolchains.

The name `wasip1` can be read as either "WASI (zero) point one" or "WASI preview
one". The official specification uses the "preview" moniker, however in most
communication the form "WASI 0.1" is now preferred. This target triple was
chosen because it not only maps to both terms, but also more closely resembles
the target terminology used in [other programming
languages](https://go.dev/blog/wasi). This is something the WASI Preview 2
specification [also makes note
of](https://github.com/WebAssembly/WASI/tree/f45e72e5294e990c23d548eea32fd35c80525fd6/preview2#introduction).

## Timeline

This table provides the dates and cut-offs for the target rename from
`wasm32-wasi` to `wasm32-wasip1`. The dates in this table do not apply to the
newly-introduced `wasm32-wasi-preview1-threads` target; this will be renamed to
`wasm32-wasip1-threads` in Rust 1.78 without going through a transition period.
The tier 3 `wasm32-wasip2` target will also be made available in Rust 1.78.

| date       | Rust Stable | Rust Beta | Rust Nightly | Notes                                    |
| ---------- | ----------- | --------- | ------------ | ---------------------------------------- |
| 2024-02-08 | 1.76        | 1.77      | 1.78         | `wasm32-wasip1` available on nightly     |
| 2024-03-21 | 1.77        | 1.78      | 1.79         | `wasm32-wasip1` available on beta        |
| 2024-05-02 | 1.78        | 1.79      | 1.80         | `wasm32-wasip1` available on stable      |
| 2024-06-13 | 1.79        | 1.80      | 1.81         | warn if `wasm32-wasi` is used on nightly |
| 2024-07-25 | 1.80        | 1.81      | 1.82         | warn if `wasm32-wasi` is used on beta    |
| 2024-09-05 | 1.81        | 1.82      | 1.83         | warn if `wasm32-wasi` is used on stable  |
| 2024-10-17 | 1.82        | 1.83      | 1.84         | `wasm32-wasi` unavailable on nightly     |
| 2024-11-28 | 1.83        | 1.84      | 1.85         | `wasm32-wasi` unavailable on beta        |
| 2025-01-09 | 1.84        | 1.85      | 1.86         | `wasm32-wasi` unavailable on stable      |

## Conclusion

In this post we've discussed the upcoming updates to Rust's WASI targets. Come
Rust 1.78 the `wasm32-wasip1` (tier 2) and `wasm32-wasip2` (tier 3) targets will
be added. In Rust 1.81 we will begin warning if `wasm32-wasi` is being used. And
in Rust 1.84, the existing `wasm32-wasi` target will be removed. This will free
up `wasm32-wasi` to eventually be used for a WASI 1.0 target. Users will have 8
months to switch to the new target name when they update their Rust toolchains.

The `wasm32-wasip2` target marks the start of native support for WASI 0.2. In
order to target it today from Rust, people are encouraged to use
[cargo-component](https://github.com/bytecodealliance/cargo-component) tool
instead. The plan is to eventually graduate `wasm32-wasip2` to a tier-2 target,
at which point `cargo-component` will be upgraded to support it natively instead.

With WASI 0.2 finally stable, it's an exciting time for WebAssembly development.
We're happy for Rust to begin implementing native support for WASI 0.2, and
we're excited about what this will enable people to build.
