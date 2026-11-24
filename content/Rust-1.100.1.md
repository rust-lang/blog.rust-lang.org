+++
path = "2026/11/27/Rust-1.100.1"
title = "Announcing Rust 1.100.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.100.1"]

[taxonomies]
tags = ["release"]
+++

The Rust team has published a new point release of Rust, 1.100.1. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.100.1 is as easy as:

```console
$ rustup update stable
```

If you don't have it already, you can get [`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website.

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.100.1

Rust 1.100.1 fixes a [regression where some types were not inferred as having `Iterator` impls](https://github.com/rust-lang/rust/issues/166229).

We backported a change to the unstable `const` version of `Iterator` in Rust 1.100.0,
which cannot actually be used by stable code, but accidentally prevented some types
with valid `Iterator` impls from being seen as actually implementing `Iterator`.

### Contributors to 1.100.1

Many people came together to create Rust 1.100.1. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.100.1/)
