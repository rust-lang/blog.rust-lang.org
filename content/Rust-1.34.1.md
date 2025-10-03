+++
path = "2019/04/25/Rust-1.34.1"
title = "Announcing Rust 1.34.1"
authors = ["The Rust Release Team"]
aliases = [
    "2019/04/25/Rust-1.34.1.html",
    "releases/1.34.1",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.34.1, and a new version of rustup, 1.18.1.
Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup,
getting Rust 1.34.1 and rustup 1.18.1 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the appropriate page on our website.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1341-2019-04-25

## What's in 1.34.1 stable

[Clippy]: https://github.com/rust-lang/rust-clippy
[panic]: https://github.com/rust-lang/rust-clippy/pull/3805

This patch release fixes two false positives and [a panic when checking macros][panic] in [Clippy].
Clippy is a tool which provides a collection of lints to catch common mistakes and improve your Rust code.

### False positive in `clippy::redundant_closure`

A false positive in the `redundant_closure` lint was [fixed](https://github.com/rust-lang/rust-clippy/pull/3821).
The lint did not take into account [differences in the number of borrows](https://github.com/rust-lang/rust-clippy/issues/3802).

In the following snippet, the method `required` expects `dep: &D` but the actual type of `dep` is `&&D`:

```rust
dependencies.iter().filter(|dep| dep.required());
```

Clippy erroneously suggested `.filter(Dependency::required)`,
which is rejected by the compiler due to the difference in borrows.

### False positive in `clippy::missing_const_for_fn`

Another false positive in the `missing_const_for_fn` lint was [fixed](https://github.com/rust-lang/rust-clippy/pull/3844).
This lint did not take into account that functions inside `trait` implementations cannot be `const fn`s.
For example, when given the following snippet, the lint would trigger:

```rust
#[derive(PartialEq, Eq)] // warning: this could be a const_fn
struct Point(isize, isize);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self { // warning: this could be a const_fn
        Point(self.0 + other.0, self.1 + other.1)
    }
}
```

## What's new in rustup 1.18.1

[a regression]: https://github.com/rust-lang/rustup.rs/issues/1794
[full release notes]: https://github.com/rust-lang/rustup.rs/blob/master/CHANGELOG.md#1181---2019-04-25

A recent rustup release, 1.18.0, introduced [a regression] that prevented installing Rust through the shell script on older platforms.
A patch was released that fixes the issue, avoiding to force TLS v1.2 on the platforms that don't support it.

You can check out other rustup changes in its [full release notes].
