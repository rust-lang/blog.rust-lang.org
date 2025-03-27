+++
layout = "post"
date = 2023-12-07
title = "Announcing Rust 1.74.1"
author = "The Rust Release Team"
release = true
+++

The Rust team has published a new point release of Rust, 1.74.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.74.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.74.1

1.74.1 resolves a few regressions introduced in 1.74.0:

- [Resolved spurious STATUS_ACCESS_VIOLATIONs in LLVM](https://github.com/rust-lang/rust/pull/118464)
- [Clarify guarantees for std::mem::discriminant](https://github.com/rust-lang/rust/pull/118006)
- [Fix some subtyping-related regressions](https://github.com/rust-lang/rust/pull/116415)

### Contributors to 1.74.1

Many people came together to create Rust 1.74.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.74.1/)
