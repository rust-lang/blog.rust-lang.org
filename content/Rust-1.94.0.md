+++
path = "2026/03/05/Rust-1.94.0"
title = "Announcing Rust 1.94.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.94.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.94.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.94.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.94.0](https://doc.rust-lang.org/stable/releases.html#version-1940-2026-03-05).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.94.0 stable

### Array windows

Rust 1.94 adds [`array_windows`], an iterating method for slices. It works just like [`windows`] but with a constant length, so the iterator items are `&[T; N]` rather than dynamically-sized `&[T]`. In many cases, the window length may even be inferred by how the iterator is used!

For example, part of one [2016 Advent of Code puzzle](https://adventofcode.com/2016/day/7) is looking for ABBA patterns: "two different characters followed by the reverse of that pair, such as `xyyx` or `abba`." If we assume only ASCII characters, that could be written by sweeping windows of the byte slice like this:

```rust
fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}
```

The destructuring argument pattern in that closure lets the compiler infer that we want windows of 4 here. If we had used the older `.windows(4)` iterator, then that argument would be a slice which we would have to index manually, *hoping* that runtime bounds-checking will be optimized away.

[`array_windows`]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.array_windows
[`windows`]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.windows

### Cargo config inclusion

Cargo now supports the `include` key in configuration files (`.cargo/config.toml`), enabling better organization, sharing, and management of Cargo configurations across projects and environments. These include paths may also be marked `optional` if they might not be present in some circumstances, e.g. depending on local developer choices.

```toml
# array of paths
include = [
    "frodo.toml",
    "samwise.toml",
]

# inline tables for more control
include = [
    { path = "required.toml" },
    { path = "optional.toml", optional = true },
]
```

See the full [`include` documentation](https://doc.rust-lang.org/nightly/cargo/reference/config.html#including-extra-configuration-files) for more details.

### TOML 1.1 support in Cargo

Cargo now parses [TOML v1.1](https://toml.io/en/v1.1.0) for manifests and configuration files. See the [TOML release notes](https://github.com/toml-lang/toml/releases/tag/1.1.0) for detailed changes, including:

* Inline tables across multiple lines and with trailing commas
* `\xHH` and `\e` string escape characters
* Optional seconds in times (sets to 0)

For example, a dependency like this:

```toml
serde = { version = "1.0", features = ["derive"] }
```

... can now be written like this:

```toml
serde = {
    version = "1.0",
    features = ["derive"],
}
```

Note that using these features in `Cargo.toml` will raise your development MSRV (minimum supported Rust version) to require this new Cargo parser, and third-party tools that read the manifest may also need to update their parsers. However, published manifests are rewritten to remain compatible with older parsers, so it is still possible to support an earlier MSRV for your crate's users.

### Stabilized APIs

- [`<[T]>::array_windows`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.array_windows)
- [`<[T]>::element_offset`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.element_offset)
- [`LazyCell::get`](https://doc.rust-lang.org/stable/std/cell/struct.LazyCell.html#method.get)
- [`LazyCell::get_mut`](https://doc.rust-lang.org/stable/std/cell/struct.LazyCell.html#method.get_mut)
- [`LazyCell::force_mut`](https://doc.rust-lang.org/stable/std/cell/struct.LazyCell.html#method.force_mut)
- [`LazyLock::get`](https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html#method.get)
- [`LazyLock::get_mut`](https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html#method.get_mut)
- [`LazyLock::force_mut`](https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html#method.force_mut)
- [`impl TryFrom<char> for usize`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html#impl-TryFrom%3Cchar%3E-for-usize)
- [`std::iter::Peekable::next_if_map`](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html#method.next_if_map)
- [`std::iter::Peekable::next_if_map_mut`](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html#method.next_if_map_mut)
- [x86 `avx512fp16` intrinsics](https://github.com/rust-lang/rust/issues/127213)
  (excluding those that depend directly on the unstable `f16` type)
- [AArch64 NEON fp16 intrinsics](https://github.com/rust-lang/rust/issues/136306)
  (excluding those that depend directly on the unstable `f16` type)
- [`f32::consts::EULER_GAMMA`](https://doc.rust-lang.org/stable/std/f32/consts/constant.EULER_GAMMA.html)
- [`f64::consts::EULER_GAMMA`](https://doc.rust-lang.org/stable/std/f64/consts/constant.EULER_GAMMA.html)
- [`f32::consts::GOLDEN_RATIO`](https://doc.rust-lang.org/stable/std/f32/consts/constant.GOLDEN_RATIO.html)
- [`f64::consts::GOLDEN_RATIO`](https://doc.rust-lang.org/stable/std/f64/consts/constant.GOLDEN_RATIO.html)

These previously stable APIs are now stable in const contexts:

- [`f32::mul_add`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.mul_add)
- [`f64::mul_add`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.mul_add)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.94.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-194-2026-03-05), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-194).

## Contributors to 1.94.0

Many people came together to create Rust 1.94.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.94.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
