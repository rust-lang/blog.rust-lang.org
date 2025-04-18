+++
path = "2023/01/26/Rust-1.67.0"
title = "Announcing Rust 1.67.0"
authors = ["The Rust Release Team"]
aliases = [
    "2023/01/26/Rust-1.67.0.html",
    "releases/1.67.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.67.0. Rust is a
programming language empowering everyone to build reliable and efficient
software.

If you have a previous version of Rust installed via rustup, you can get 1.67.0
with:

```
$ rustup update stable
```

If you don't have it already, you can [get
`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on
our website, and check out the [detailed release notes for
1.67.0](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1670-2023-01-26)
on GitHub.

If you'd like to help us out by testing future releases, you might consider
updating locally to use the beta channel (`rustup default beta`) or the nightly
channel (`rustup default nightly`). Please
[report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you
might come across!

## What's in 1.67.0 stable

### `#[must_use]` effective on `async fn`

`async` functions annotated with `#[must_use]` now apply that attribute to the
output of the returned `impl Future`. The `Future` trait itself is already
annotated with `#[must_use]`, so all types implementing `Future` are
automatically `#[must_use]`, which meant that previously there was no way to
indicate that the output of the `Future` is itself significant and should be used in some way.

With 1.67, the compiler will now warn if the output isn't used in some way.

```rust
#[must_use]
async fn bar() -> u32 { 0 }

async fn caller() {
    bar().await;
}
```

```
warning: unused output of future returned by `bar` that must be used
 --> src/lib.rs:5:5
  |
5 |     bar().await;
  |     ^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
```

### `std::sync::mpsc` implementation updated

Rust's standard library has had a multi-producer, single-consumer channel since
before 1.0, but in this release the implementation is switched out to be based
on [`crossbeam-channel`](https://crates.io/crates/crossbeam-channel). This
release contains no API changes, but the new implementation fixes a number of
bugs and improves the performance and maintainability of the implementation.

Users should not notice any significant changes in behavior as of this release.

### Stabilized APIs

- [`{integer}::checked_ilog`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog)
- [`{integer}::checked_ilog2`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog2)
- [`{integer}::checked_ilog10`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog10)
- [`{integer}::ilog`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog)
- [`{integer}::ilog2`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog2)
- [`{integer}::ilog10`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog10)
- [`NonZeroU*::ilog2`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.ilog2)
- [`NonZeroU*::ilog10`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.ilog10)
- [`NonZero*::BITS`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#associatedconstant.BITS)

These APIs are now stable in const contexts:

- [`char::from_u32`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.from_u32)
- [`char::from_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.from_digit)
- [`char::to_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.to_digit)
- [`core::char::from_u32`](https://doc.rust-lang.org/stable/core/char/fn.from_u32.html)
- [`core::char::from_digit`](https://doc.rust-lang.org/stable/core/char/fn.from_digit.html)

Check out everything that changed in
[Rust](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1670-2023-01-26),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-167-2023-01-26),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-167).

### Contributors to 1.67.0

Many people came together to create Rust 1.67.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.67.0/)
