+++
path = "2025/01/30/Rust-1.84.1"
title = "Announcing Rust 1.84.1"
authors = ["The Rust Release Team"]
aliases = ["2025/01/30/Rust-1.84.1.html"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.84.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.84.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.84.1

1.84.1 resolves a few regressions introduced in 1.84.0:

- [Fix ICE 132920 in duplicate-crate diagnostics.](https://github.com/rust-lang/rust/pull/133304/)
- [Fix errors for overlapping impls in incremental rebuilds.](https://github.com/rust-lang/rust/pull/133828/)
- [Fix slow compilation related to the next-generation trait solver.](https://github.com/rust-lang/rust/pull/135618/)
- [Fix debuginfo when LLVM's location discriminator value limit is exceeded.](https://github.com/rust-lang/rust/pull/135643/)

It also includes several fixes for those building Rust from source:

- [Only try to distribute `llvm-objcopy` if LLVM tools are enabled.](https://github.com/rust-lang/rust/pull/134240/)
- [Add Profile Override for Non-Git Sources.](https://github.com/rust-lang/rust/pull/135433/)
- [Resolve symlinks of LLVM tool binaries before copying them.](https://github.com/rust-lang/rust/pull/135585/)
- [Make it possible to use ci-rustc on tarball sources.](https://github.com/rust-lang/rust/pull/135722/)

### Contributors to 1.84.1

Many people came together to create Rust 1.84.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.84.1/)
