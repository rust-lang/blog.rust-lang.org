+++
path = "2020/08/03/Rust-1.45.2"
title = "Announcing Rust 1.45.2"
authors = ["The Rust Release Team"]
aliases = [
    "2020/08/03/Rust-1.45.2.html",
    "releases/1.45.2",
]

[extra]
release = true
+++

The Rust team is announcing a new version of Rust, 1.45.2. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.45.2 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.45.2][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1452-2020-08-03

## What's in 1.45.2 stable

1.45.2 contains two fixes, one to 1.45.1 and the other to 1.45.0.

## `#[track_caller]` on trait objects

Trait objects with methods annotated with `#[track_caller]` would be
miscompiled. `#[track_caller]` is not yet stable on 1.45. However, the standard
library makes use of this on some traits for better error messages. Trait
objects of `SliceIndex`, `Index`, and `IndexMut` were affected by this bug.

## Tuple patterns binding `..` to an identifier

In 1.45.1, we backported a fix for [#74539], but this fix turned out to be
incorrect, causing other unrelated breakage. As such, this release reverts that
fix.

## Contributors to 1.45.2

Many people came together to create Rust 1.45.2. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.45.2/)

[#74539]: https://github.com/rust-lang/rust/issues/74539
