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

### Fixed combined doctest compilation

Due to a bug in the implementation, [combined doctests](https://doc.rust-lang.org/edition-guide/rust-2024/rustdoc-doctests.html) did not work as intended in the stable 2024 Edition. Internal errors with feature stability caused rustdoc to automatically use its "unmerged" fallback method instead, like in previous editions.

Those errors are now fixed in 1.85.1, realizing the performance improvement of combined doctest compilation as intended! See the [backport issue](https://github.com/rust-lang/rust/issues/138418) for more details, including the risk analysis of making this behavioral change in a point release.

### Other fixes

1.85.1 also resolves a few regressions introduced in 1.85.0:

- [Relax some `target_feature` checks when generating docs.](https://github.com/rust-lang/rust/pull/137632/)
- [Fix errors in `std::fs::rename` on Windows 1607.](https://github.com/rust-lang/rust/pull/137528/)
- [Downgrade bootstrap `cc` to fix custom targets.](https://github.com/rust-lang/rust/pull/137460/)
- [Skip submodule updates when building Rust from a source tarball.](https://github.com/rust-lang/rust/pull/137338/)

### Contributors to 1.85.1

Many people came together to create Rust 1.85.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.85.1/)
