+++
layout = "post"
date = 2025-03-18
title = "Announcing Rust 1.85.1"
author = "The Rust Release Team"
release = true
+++

The Rust team has published a new point release of Rust, 1.85.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.85.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.85.1

- [Fix the doctest-merging feature of the 2024 Edition.](https://github.com/rust-lang/rust/pull/137899/)
- [Relax some `target_feature` checks when generating docs.](https://github.com/rust-lang/rust/pull/137632/)
- [Fix errors in `std::fs::rename` on Windows 1607.](https://github.com/rust-lang/rust/pull/137528/)
- [Downgrade bootstrap `cc` to fix custom targets.](https://github.com/rust-lang/rust/pull/137460/)
- [Skip submodule updates when building Rust from a source tarball.](https://github.com/rust-lang/rust/pull/137338/)

### Contributors to 1.85.1

Many people came together to create Rust 1.85.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.85.1/)
