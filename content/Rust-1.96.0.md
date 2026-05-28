+++
path = "2026/05/28/Rust-1.96.0"
title = "Announcing Rust 1.96.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.96.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.96.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.96.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.96.0](https://doc.rust-lang.org/stable/releases.html#version-1960-2026-05-28).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.96.0 stable

## New `Range*` types

Many users expect `Range` and related `core::ops` types to be `Copy`, but this is not the case: they implement `Iterator` directly, and [it is a footgun to implement both `Iterator` and `Copy` on the same type](https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#copy_iterator) so this has been avoided. [RFC3550] proposed a set of replacement range types that implement `IntoIterator` rather than `Iterator`, meaning they can also be `Copy`. The standard library portion of that RFC is now stable, introducing:

- `core::range::Range`
- `core::range::RangeFrom`
- `core::range::RangeInclusive`
- Associated iterators

A Rust version in the near future will also add `core::range::RangeFull` and `core::range::RangeTo` as re-exports from `core::ops` (these do not implement `Iterator` and already implement `Copy`), and `core::range::legacy::*` as the new home for the current ranges. Range syntax like `0..1` still produces the legacy types for now, but will be updated to `core::range` types in a future edition.

With these stabilizations, it is now possible to store slice accessors in `Copy` types without splitting `start` and `end`:

```rust
use core::range::Range;

#[derive(Clone, Copy)]
pub struct Span(Range<usize>);

impl Span {
    pub fn of(self, s: &str) -> &str {
        &s[self.0]
    }
}
```

The new `RangeInclusive` also makes its fields public, unlike the legacy version which avoided exposing the exhausted iterator state. This isn't a concern with the new type since it must be converted to begin iteration.

Library authors should consider making use of `impl RangeBounds` in public API, which accepts both legacy and new range types. If a concrete type is needed, prefer using new ranges as this will eventually become the default.

[RFC3550]: https://rust-lang.github.io/rfcs/3550-new-range.html

### Assert matching patterns

The new macros `assert_matches!` and `debug_assert_matches!` check that a value matches a given pattern, panicking with a `Debug` representation of the value otherwise. These are essentially the same as `assert!(matches!(..))` and `debug_assert!(matches!(..))`, but the printed value improves the possibility of diagnosing the failure.

These new macros have not been added to the standard prelude, because they would collide with popular third-party crates that provide macros with the same name. Instead, they should be manually imported from `core` or `std` before use.

```rust
use core::assert_matches;

unsafe extern "C" {
    safe fn get_current_year() -> u32;
}

fn main() {
    assert_matches!(get_current_year(), 2026..);
}
```

### Changes to WebAssembly targets

WebAssembly targets no longer pass `--allow-undefined` to the linker which means that undefined symbols when linking are now a linker error instead of being converted to WebAssembly imports from the `"env"` module. This change prevents modules from linking unless all linking-related symbols are defined to catch bugs earlier and prevent accidental issues with symbol naming or similar.

Undefined linking-related symbols are often indicative of build-time related bugs or misconfiguration. If, however, the old behavior is intended then it can be re-enabled with `RUSTFLAGS=-Clink-arg=--allow-undefined` or by editing the source code and using `#[link(wasm_import_module = "env")]` on the block defining the symbol.

This change was [previously announced](https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/) on this blog, and now takes effect in Rust 1.96.

### Stabilized APIs

- [`assert_matches!`](https://doc.rust-lang.org/stable/std/macro.assert_matches.html)
- [`debug_assert_matches!`](https://doc.rust-lang.org/stable/std/macro.debug_assert_matches.html)
- [`From<T> for AssertUnwindSafe<T>`](https://doc.rust-lang.org/stable/std/panic/struct.AssertUnwindSafe.html#impl-From%3CT%3E-for-AssertUnwindSafe%3CT%3E)
- [`From<T> for LazyCell<T, F>`](https://doc.rust-lang.org/stable/std/cell/struct.LazyCell.html#impl-From%3CT%3E-for-LazyCell%3CT,+F%3E)
- [`From<T> for LazyLock<T, F>`](https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html#impl-From%3CT%3E-for-LazyLock%3CT,+F%3E)
- [`core::range::RangeToInclusive`](https://doc.rust-lang.org/stable/core/range/struct.RangeToInclusive.html)
- [`core::range::RangeToInclusiveIter`](https://doc.rust-lang.org/stable/core/range/struct.RangeToInclusiveIter.html)
- [`core::range::RangeFrom`](https://doc.rust-lang.org/stable/core/ops/struct.RangeFrom.html)
- [`core::range::RangeFromIter`](https://doc.rust-lang.org/stable/core/ops/struct.RangeFromIter.html)
- [`core::range::Range`](https://doc.rust-lang.org/stable/std/range/struct.Range.html)
- [`core::range::RangeIter`](https://doc.rust-lang.org/stable/std/range/struct.RangeIter.html)

### Two Cargo advisories

Rust 1.96 contains fixes for two vulnerabilities for users of third-party registries.

- [CVE-2026-5223](https://blog.rust-lang.org/2026/05/25/cve-2026-5223/) is a **medium** severity vulnerability regarding extraction of crate tarballs with symlinks.

- [CVE-2026-5222](https://blog.rust-lang.org/2026/05/25/cve-2026-5222/) is a **low** severity vulnerability regarding authentication with normalized URLs.

Users of crates.io are **not affected** by either vulnerability.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.96.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-196-2026-05-28), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-196).

## Contributors to 1.96.0

Many people came together to create Rust 1.96.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.96.0/)
