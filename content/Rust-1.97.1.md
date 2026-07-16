+++
path = "2026/07/16/Rust-1.97.1"
title = "Announcing Rust 1.97.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.97.1"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.97.1. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.97.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.97.1

Rust 1.97.1 fixes a [miscompilation in an LLVM optimization](https://github.com/rust-lang/rust/issues/159035).

We have backported both an LLVM fix and a disable of the underlying change in Rust 1.97.0 of
Rust's generated IR that increased the likelihood of this happening. However,
note that the underlying miscompilation has been present since at least Rust
1.87.

If you'd like to help us out by testing future releases, you might consider
running your code's CI or locally using the beta channel (`rustup default beta`) or the nightly
channel (`rustup default nightly`). Please
[report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you
might come across!

### Contributors to 1.97.1

Many people came together to create Rust 1.97.1. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.97.1/)
