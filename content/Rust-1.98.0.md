+++
path = "2026/08/20/Rust-1.98.0"
title = "Announcing Rust 1.98.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.98.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.98.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.98.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can get [`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.98.0](https://doc.rust-lang.org/stable/releases.html#version-1980-2026-08-20).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.98.0 stable

### Algebraic floating-point methods

The floating-point types `f32` and `f64` now have "algebraic" methods for addition, subtraction, multiplication, division, and remainder. These allow optimizations on these operations using the algebraic properties of real numbers, even though these properties do not hold with the limitations of floating-point representations. The exact set of optimizations is not specified, but may be similar to the kind of optimization you would see with the `-ffast-math` option in other languages.

For example, floating-point addition is [not associative](https://en.wikipedia.org/wiki/Associative_property#Nonassociativity_of_floating-point_calculation), so a sum like `a + b + c + d` must be evaluated in the left-associative order in which it is parsed, like `((a + b) + c) + d`. If you write the same sum as a chain of `algebraic_add` calls, then the compiler is free to reorder it, perhaps like `(a + b) + (c + d)` to evaluate the partial sums simultaneously. Broader loop-vectorization is often enabled by using these algebraic methods as well.

See the [library documentation](https://doc.rust-lang.org/stable/core/primitive.f32.html#algebraic-operators) and the original [API change proposal](https://github.com/rust-lang/libs-team/issues/532) for more details.

### Buffered integer formatting

All of the primitive integer types now have a [`format_into`](https://doc.rust-lang.org/stable/core/primitive.usize.html#method.format_into) method that takes a `&mut NumBuffer<Self>` parameter, which is a buffer that is large enough to hold the decimal format of any value of that type. The buffer itself is opaque, but the method returns the formatted `&str` with a lifetime borrowed from that buffer.

This method also bypasses much of the dynamic dispatch that you would get with buffered `write!` formatting, which can be a boon to performance. The [`itoa-benchmark`](https://github.com/dtolnay/itoa-benchmark) repo now shows that `format_into` performs similarly to `itoa` itself, so this could serve as a standard replacement for that dependency and others like it.

### Fix interaction between `ManuallyDrop` and `Box`

Prior to Rust 1.96.0, there was a bug in the Rust compiler, which made the following code undefined behavior:

```rust
let mut x = ManuallyDrop::new(Box::new(1));
unsafe { ManuallyDrop::drop(&mut x) }
let x = x; // UB!
```

This is because the compiler considers it undefined behavior to move a `Box` that has been dropped (deallocated), and `ManuallyDrop` used to propagate that, such that moving `ManuallyDrop<Box<_>>` where the box has been dropped would also be considered UB.

In Rust 1.96.0 we fixed this, so this code was no longer UB. In this release we have updated the `ManuallyDrop` documentation, providing a stable guarantee that this code will continue to not be UB in the future. See [`ManuallyDrop` docs](https://doc.rust-lang.org/stable/std/mem/struct.ManuallyDrop.html#pre-196-interaction-with-box) and the related [RFC 3336](https://rust-lang.github.io/rfcs/3336-maybe-dangling.html) for more information.

### Stabilized APIs

- [`str::substr_range`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.substr_range)
- [`[T]::subslice_range`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.subslice_range)
- [`core::fmt::NumBuffer`](https://doc.rust-lang.org/stable/core/fmt/struct.NumBuffer.html)
- [`<{integer}>::format_into`](https://doc.rust-lang.org/stable/core/primitive.usize.html#method.format_into)
- [`Send/Sync for std::process::CommandArgs`](https://doc.rust-lang.org/stable/std/process/struct.CommandArgs.html#impl-Send-for-CommandArgs%3C'a%3E)
- [`{fN}::algebraic_add`](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.algebraic_add)
- [`{fN}::algebraic_sub`](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.algebraic_sub)
- [`{fN}::algebraic_mul`](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.algebraic_mul)
- [`{fN}::algebraic_div`](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.algebraic_div)
- [`{fN}::algebraic_rem`](https://doc.rust-lang.org/stable/core/primitive.f32.html#method.algebraic_rem)
- [`NonZero<{integer}>::from_str_radix`](https://doc.rust-lang.org/stable/core/num/struct.NonZero.html#method.from_str_radix-4)
- [`String::from_utf16le`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16le)
- [`String::from_utf16le_lossy`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16le_lossy)
- [`String::from_utf16be`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16be)
- [`String::from_utf16be_lossy`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16be_lossy)
- [`[T]::strip_circumfix`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.strip_circumfix)
- [`str::strip_circumfix`](https://doc.rust-lang.org/stable/core/primitive.str.html#method.strip_circumfix)
- [`Atomic<T>::from_mut`](https://doc.rust-lang.org/stable/core/sync/atomic/struct.Atomic.html#method.from_mut)
- [`Atomic<T>::get_mut_slice`](https://doc.rust-lang.org/stable/core/sync/atomic/struct.Atomic.html#method.get_mut_slice)
- [`Atomic<T>::from_mut_slice`](https://doc.rust-lang.org/stable/core/sync/atomic/struct.Atomic.html#method.from_mut_slice)
- [`std::range::legacy`](https://doc.rust-lang.org/stable/std/range/legacy/index.html)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.98.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-198-2026-08-20), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-198).

## Contributors to 1.98.0

Many people came together to create Rust 1.98.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.98.0/)
