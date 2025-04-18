+++
path = "2023/09/19/Rust-1.72.1"
title = "Announcing Rust 1.72.1"
authors = ["The Rust Release Team"]
aliases = [
    "2023/09/19/Rust-1.72.1.html",
    "releases/1.72.1",
]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.72.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.72.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.72.1

1.72.1 resolves a few regressions introduced in 1.72.0:

- [Partially revert codegen change, improving codegen](https://github.com/rust-lang/rust/pull/115236)
- [rustdoc: Fix self ty params in objects with lifetimes](https://github.com/rust-lang/rust/pull/115276)
- [Fix regression in compile times](https://github.com/rust-lang/rust/pull/114948)
- Resolve some ICEs in the compiler:
  - [#115215](https://github.com/rust-lang/rust/pull/115215)
  - [#115559](https://github.com/rust-lang/rust/pull/115559)

### Contributors to 1.72.1

Many people came together to create Rust 1.72.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.72.1/)
