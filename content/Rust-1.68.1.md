+++
path = "2023/03/23/Rust-1.68.1"
title = "Announcing Rust 1.68.1"
authors = ["The Rust Release Team"]
aliases = [
    "2023/03/23/Rust-1.68.1.html",
    "releases/1.68.1",
]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.68.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.68.1 with:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.68.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1681-2023-03-23

## What's in 1.68.1 stable

Rust 1.68.1 stable primarily contains a change to how Rust's CI builds the
Windows MSVC compiler, no longer enabling LTO for the Rust code. This led to a
[miscompilation](https://github.com/rust-lang/rust/issues/109067) that the Rust
team is debugging, but in the meantime we're reverting the change to enable
LTO.

This is currently believed to have no effect on wider usage of ThinLTO. The
Rust compiler used an unstable flag as part of the build process to enable
ThinLTO despite compiling to a dylib.

There are a few other regression fixes included in the release:

* [Fix building the compiler with `--enable-local-rust`](https://github.com/rust-lang/rust/pull/109111/)
* [Treat `$prefix-clang` as `clang` in linker detection code](https://github.com/rust-lang/rust/pull/109156)
* [Fix a panic in the compiler](https://github.com/rust-lang/rust/pull/108162)

### Contributors to 1.68.1

Many people came together to create Rust 1.68.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.68.1/)
