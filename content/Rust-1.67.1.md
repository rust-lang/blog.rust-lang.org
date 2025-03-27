+++
layout = "post"
date = 2023-02-09
title = "Announcing Rust 1.67.1"
author = "The Rust Release Team"
release = true
+++

The Rust team has published a new point release of Rust, 1.67.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.67.1 with:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.67.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1671-2023-02-09

## What's in 1.67.1 stable

Rust 1.67.1 fixes a regression for projects that link to thin archives
(`.a` files that reference external `.o` objects). The new
archive writer in 1.67.0 could not read thin archives as inputs, leading to the
error "Unsupported archive identifier." The compiler now uses LLVM's archive
writer again, until that format is supported in the new code.

Additionally, the clippy style lint `uninlined_format_args` is temporarily
downgraded to pedantic -- allowed by default. While the compiler has supported
this format since Rust 1.58, `rust-analyzer` does not support it yet, so it's
not necessarily good to use that style everywhere possible.

The final change is a soundness fix in Rust's own bootstrap code. This had no
known problematic uses, but it did raise an error when bootstrap was compiled
with 1.67 itself, rather than the prior 1.66 release as usual.

### Contributors to 1.67.1

Many people came together to create Rust 1.67.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.67.1/)

