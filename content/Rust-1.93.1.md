+++
path = "2026/02/12/Rust-1.93.1"
title = "Announcing Rust 1.93.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.93.1"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.93.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.93.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.93.1

Rust 1.93.1 resolves three regressions that were introduced in the 1.93.0 release.

- [Don't try to recover a keyword as a non-keyword identifier](https://github.com/rust-lang/rust/pull/150590), fixing an internal compiler error (ICE) that especially [affected rustfmt](https://github.com/rust-lang/rustfmt/issues/6739).

- [Fix a `clippy::panicking_unwrap` false-positive on field access with an implicit dereference](https://github.com/rust-lang/rust-clippy/pull/16196).

- [Revert an update to wasm-related dependencies in CI](https://github.com/rust-lang/rust/pull/152259), fixing file descriptor leaks on the `wasm32-wasip2` target.

### Contributors to 1.93.1

Many people came together to create Rust 1.93.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.93.1/)
