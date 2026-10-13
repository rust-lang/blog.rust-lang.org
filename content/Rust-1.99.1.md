+++
path = "2026/10/15/Rust-1.99.1"
title = "Announcing Rust 1.99.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.99.1"]

[taxonomies]
tags = ["release"]
+++

The Rust team has published a new point release of Rust, 1.99.1. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.99.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.99.1

Rust 1.99.1 fixes the following regressions from prior releases:

* [Missing symbol `wcslen`](https://github.com/rust-lang/rust/issues/163614): Some Rust targets were getting link errors for missing `wcslen`, and that function is now present in Rust's compiler-builtins library as a fallback.

  The LLVM `loop-idiom-wcslen` pass may introduce calls to `wcslen` as an optimization, and some Rust targets did not have that function readily available in `libc` or elsewhere. The UEFI targets in particular were broken for any project using `std`, as some of these calls were introduced in the precompiled standard library, but the issue was also reproducible for similar user code patterns.

* [Miscompilation of FFI `bool` return values on x86_64](https://github.com/rust-lang/rust/issues/163911): Rust functions returning `bool` through FFI could leave bits 1-7 of the return register unspecified, causing callers to observe raw values other than `0` or `1`. These return types are now marked for zero-extension again, ensuring those extra bits are clear as required by the ABI.

* [`rust-lld` failed to load `libLLVM` on macOS](https://github.com/rust-lang/rust/issues/163947): This linker could fail to run if the compiler was invoked without the `rustup` proxy, as there was no direct setting for the appropriate library search path. Now `rust-lld` is built with an `rpath` entry for the directory containing `libLLVM.dylib`.

If you'd like to help us out by testing future releases, you might consider using the beta (`rustup default beta`) and nightly (`rustup default nightly`) channels locally and in your CI. Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

### Contributors to 1.99.1

Many people came together to create Rust 1.99.1. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.99.1/)
