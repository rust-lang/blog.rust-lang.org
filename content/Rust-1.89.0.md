+++
path = "2025/08/07/Rust-1.89.0"
title = "Announcing Rust 1.89.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.89.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.89.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.89.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.89.0](https://doc.rust-lang.org/stable/releases.html#version-1890-2025-08-07).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.89.0 stable

### Explicitly inferred arguments to const generics

Rust now supports `_` as an argument to const generic parameters, inferring the value from surrounding context:

```rust
pub fn make_bitset<const LEN: usize>() -> [bool; LEN] {
  [false; _]
}
```

Similar to the rules for when `_` is permitted as a type, `_` is not permitted as an argument to const generics when in a signature:

```rust
// This is not allowed
pub fn make_bitset<const LEN: usize>() -> [bool; _] {
  [false; LEN]
}
```

### Mismatched lifetimes syntax lint

TC/LANG to write this

### More x86 target features

The `target_feature` attribute now supports the `sha512`, `sm3`, `sm4`, `kl` and `widekl` target features on x86. Additionally a number of `avx512` intrinsics and target features are also supported on x86:

```rust
#[target_feature(enable = "avx512bw")]
pub fn cool_simd_code(/* .. */) -> /* ... */ {
    /* ... */
}

```

### Platform Support

- [Add new Tier-3 targets `loongarch32-unknown-none` and `loongarch32-unknown-none-softfloat`](https://github.com/rust-lang/rust/pull/142053)

Refer to Rust’s [platform support page][platform_support_page] for more information on Rust’s tiered platform support.

### Stabilized APIs

TODO

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.89.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-189-2025-08-07), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-189).

## Contributors to 1.89.0

Many people came together to create Rust 1.89.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.89.0/)

[platform_support_page]: https://doc.rust-lang.org/rustc/platform-support.html