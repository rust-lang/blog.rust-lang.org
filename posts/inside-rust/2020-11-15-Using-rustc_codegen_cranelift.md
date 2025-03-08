+++
layout = "post"
date = 2020-11-15
title = "Using rustc_codegen_cranelift for debug builds"
author = "Jynn Nelson"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++

## What is `rustc_codegen_cranelift`?

[`rustc_codegen_cranelift`], or just `cg_clif` for short, is a new experimental
codegen backend for the Rust compiler. The existing backend is LLVM, which is very
good at producing fast, highly optimized code, but is not very good at
compiling code quickly. `cg_clif`, which uses the [Cranelift] project, would
provide a fast backend which greatly improves compile times, at the cost of
performing very few optimizations. This is a great fit for debug builds, and the hope is
that `cg_clif` will eventually be the default backend in debug mode.

## What is the progress of using `rustc_codegen_cranelift` for debug builds?

There has been a [Major Change Proposal][MCP] open for some time for making
`cg_clif` part of the main Rust repository. Recently, [the MCP was
accepted][compiler-team#270] and the compiler team [merged][#77975]
`rustc_cranelift_codegen` [into the main Rust git repository][#77975].
`cg_clif` is not yet distributed with `rustup`, but this means you can now
build it from source in-tree!

## How do I use `rustc_codegen_cranelift`?

In this section, I'll walk through step-by-step how to build the new backend from source, then use it on your own projects. All code is copy/paste-able, and each step is explained.

First, let's build `cg_clif` from source.

```sh
$ git clone https://github.com/bjorn3/rustc_codegen_cranelift.git
$ ./prepare.sh
$ ./build.sh
```

Now, we can start using it to compile a project. For demonstration purposes,
I'll be using `cargo`, but you can use any Rust project supported by
`cg_clif`.

```
$ cd ..
$ git clone https://github.com/rust-lang/cargo/
$ cd cargo
$ ../rustc_codegen_cranelift/build/cargo.sh build
...
    Finished dev [unoptimized + debuginfo] target(s) in 49.93s
```

It works! For comparison, let's see how long the equivalent LLVM backend would
take.

```sh
$ rustup install nightly-2020-10-31
$ cargo +nightly-2020-10-31 build
...
    Finished dev [unoptimized + debuginfo] target(s) in 54.64s
```

LLVM takes a full 5 seconds longer for a full build. Next, let's try incremental builds:

```
$ git apply <<EOF
diff --git a/src/cargo/lib.rs b/src/cargo/lib.rs
index bccb41121..703afa754 100644
--- a/src/cargo/lib.rs
+++ b/src/cargo/lib.rs
@@ -36,8 +36,8 @@ use anyhow::Error;
 use log::debug;
 use std::fmt;
 
-pub use crate::util::errors::{InternalError, VerboseError};
 pub use crate::util::{CargoResult, CliError, CliResult, Config};
+pub use crate::util::errors::{InternalError, VerboseError};
 
 pub const CARGO_ENV: &str = "CARGO";
EOF
$ ../rustc_codegen_cranelift/build/cargo.sh build
    Finished dev [unoptimized + debuginfo] target(s) in 7.98s
$ cargo +nightly-2020-10-31 build
   Compiling cargo v0.50.0 (/home/jyn/cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 5.48s
```

LLVM is actually *faster* here: `serde_derive` took longer to run under cranelift, since it wasn't as optimized. Under cranelift it takes ~14% percent of the time, while under LLVM it takes less than 3%.

## Building in-tree

This section is mostly for compiler hackers, but feel free to follow along even
if you're just interested! The reason this isn't the recommended way to build
`cg_clif` is because the Rust compiler takes a very long time to build.

First, download the Rust repository.

```console
$ git clone https://github.com/rust-lang/rust
```

Now, let's set up the build system to use `cg_clif`.

```text
$ cat > config.toml <<EOF
[rust]
codegen-backends = ["cranelift"]
EOF
```

Finally, let's run the build. This can take a long time, over a half-hour in some cases.

```console
$ ./x.py build
```

## How can I help?

You don't need to be a compiler developer to help improve `cg_clif`!  The best
way you can help is by testing `cg_clif` on different Rust crates across the
ecosystem.  Just while writing this article, I found [two][#1102]
[bugs][#1101], so there's plenty of work left to be done. Please report any bugs you find
to the [`rustc_codegen_cranelift` git repository][issue].

In the future, we hope to distribute `cg_clif` with Rustup, and if it matures sufficiently, eventually make it the default backend for debug builds.

[`rustc_codegen_cranelift`]: https://github.com/bjorn3/rustc_codegen_cranelift
[Cranelift]: https://github.com/bytecodealliance/wasmtime/tree/main/cranelift#cranelift-code-generator
[#77975]: https://github.com/rust-lang/rust/pull/77975
[MCP]: https://forge.rust-lang.org/compiler/mcp.html
[compiler-team#270]: https://github.com/rust-lang/compiler-team/issues/270
[`rustc-dev-guide`]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html#creating-a-rustup-toolchain
[git worktree]: https://rustc-dev-guide.rust-lang.org/building/suggested.html#working-on-multiple-branches-at-the-same-time
[#1102]: https://github.com/bjorn3/rustc_codegen_cranelift/issues/1102
[#1101]: https://github.com/bjorn3/rustc_codegen_cranelift/issues/1101
[issue]: https://github.com/bjorn3/rustc_codegen_cranelift/issues/new
