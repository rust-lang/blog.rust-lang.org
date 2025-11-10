+++
path = "2025/11/10/Rust-1.91.1"
title = "Announcing Rust 1.91.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.91.1"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.91.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.91.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.91.1

Rust 1.91.1 includes fixes for two regressions introduced in the 1.91.0 release.

### Linker and runtime errors on Wasm

Most targets supported by Rust identify symbols by their name, but Wasm
identifies them with a symbol name *and* a Wasm module name. The
[`#[link(wasm_import_module)]`][wasm_import_module] attribute allows to
customize the Wasm module name an `extern` block refers to:

```rust
#[link(wasm_import_module = "hello")]
extern "C" {
    pub fn world();
}
```

Rust 1.91.0 introduced a regression in the attribute, which could cause linker
failures during compilation (*"import module mismatch"* errors) or the wrong
function being used at runtime (leading to undefined behavior, including crashes
and silent data corruption). This happened when the same symbol name was
imported from two different Wasm modules across multiple Rust crates.

Rust 1.91.1 fixes the regression. More details are available in [issue #148347].

[wasm_import_module]: https://doc.rust-lang.org/reference/items/external-blocks.html#r-items.extern.attributes.link.wasm_import_module
[issue #148347]: https://github.com/rust-lang/rust/issues/148347

### Cargo target directory locking broken on illumos

Cargo relies on locking the `target/` directory during a build to prevent
concurrent invocations of Cargo from interfering with each other. Not all
filesystems support locking (most notably some networked ones): if the OS
returns the `Unsupported` error when attempting to lock, Cargo assumes locking
is not supported and proceeds without it.

Cargo 1.91.0 switched from custom code interacting with the OS APIs to the
[`File::lock`] standard library method (recently stabilized in Rust 1.89.0). Due
to an oversight, that method always returned `Unsupported` on the illumos
target, causing Cargo to never lock the build directory on illumos regardless of
whether the filesystem supported it.

Rust 1.91.1 fixes the oversight in the standard library by enabling the
[`File::lock`] family of functions on illumos, indirectly fixing the Cargo
regression.

[`File::lock`]: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.lock

### Contributors to 1.91.1

Many people came together to create Rust 1.91.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.91.1/)
