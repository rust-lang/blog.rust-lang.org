+++
path = "2026/10/01/Rust-1.99.0"
title = "Announcing Rust 1.99.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.99.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.99.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.99.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can get [`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.99.0](https://doc.rust-lang.org/stable/releases.html#version-1990-2026-10-01).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.99.0 stable

### extern "C" variadics

Rust 1.99.0 stabilizes defining C-ABI variadic functions with "C" and
"C-unwind" ABIs. Variadic functions defined this way use a variable argument
list (`...`) and accept an arbitrary number of arguments. Rust could already
call externally-defined variadic functions (e.g., `libc::printf`). With Rust
1.99, these functions can now be written in Rust itself:

```rust
/// SAFETY: must be called with (at least) 2 i32 arguments.
unsafe extern "C" fn sum(mut args: ...) -> i32 {
    // SAFETY: guaranteed by the caller.
    let a = unsafe { args.next_arg::<i32>() };
    let b = unsafe { args.next_arg::<i32>() };
    a + b
}

fn foo() -> i32 {
    unsafe { sum(0i32, 2i32) }
}
```

The type of `...` is [`VaList`](https://doc.rust-lang.org/std/ffi/struct.VaList.html),
which is ABI-compatible with the C `va_list` type across targets. What types can be read from a `VaList` is guarded by the
[`VaArgSafe`](https://doc.rust-lang.org/std/ffi/trait.VaArgSafe.html) trait.

For more details on c-variadic functions, see the
[Reference](https://doc.rust-lang.org/reference/items/functions.html#c-variadic-functions).
This release also stabilizes support for defining naked variadic functions with
non-"C" ABIs, which must be written via inline assembly.

### Layout information from raw pointers

This release settles the safety requirements for retrieving the size and
alignment on raw pointers to both `Sized` (trivially safe, already possible on
stable) and non-`Sized` types.

This is done by stabilizing three functions:

- [`Layout::for_value_raw`](https://doc.rust-lang.org/stable/core/alloc/struct.Layout.html#method.for_value_raw)
- [`mem::size_of_val_raw`](https://doc.rust-lang.org/stable/core/mem/fn.size_of_val_raw.html)
- [`mem::align_of_val_raw`](https://doc.rust-lang.org/stable/core/mem/fn.align_of_val_raw.html)

### Recommend against round-trip unleaking after `Box::leak`

While there are no changes to the language semantics in Rust 1.99, we have
updated the documentation on [`Box::leak`] to recommend against patterns that
later deallocate that memory. This was done because such code was found to have
problematic interactions with current and future potential compiler optimizations,
and is especially problematic with the upcoming stabilization of custom allocators.
Instead, [`Box::into_non_null`] should be preferred.

This guidance also applies to other `leak` functions in the standard library.

[`Box::leak`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.leak
[`Box::into_non_null`]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_non_null

### Stabilized APIs

- [`IntoIterator` for `Box<[T; N]>`](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#impl-IntoIterator-for-Box%3C%5BT;+N%5D,+A%3E)
- [`IntoIterator` for `&Box<[T; N]>`](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#impl-IntoIterator-for-%26Box%3C%5BT;+N%5D,+A%3E)
- [`IntoIterator` for `&mut Box<[T; N]>`](https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#impl-IntoIterator-for-%26mut+Box%3C%5BT;+N%5D,+A%3E)
- [`VecDeque::retain_back`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.retain_back)
- [`core::ffi::VaList`](https://doc.rust-lang.org/stable/core/ffi/struct.VaList.html)
- [`Box::into_non_null`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.into_non_null)
- [`Box::from_non_null`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.from_non_null)
- [`Vec::into_parts`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.into_parts)
- [`Vec::from_parts`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.from_parts)
- [`core::mem::size_of_val_raw`](https://doc.rust-lang.org/stable/core/mem/fn.size_of_val_raw.html)
- [`core::mem::align_of_val_raw`](https://doc.rust-lang.org/stable/core/mem/fn.align_of_val_raw.html)
- [`core::alloc::Layout::for_value_raw`](https://doc.rust-lang.org/stable/core/alloc/struct.Layout.html#method.for_value_raw)
- [`String::from_utf8_lossy_owned`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf8_lossy_owned)
- [`string::FromUtf8Error::into_utf8_lossy`](https://doc.rust-lang.org/stable/std/string/struct.FromUtf8Error.html#method.into_utf8_lossy)
- [`FusedIterator for StepBy<I>`](https://doc.rust-lang.org/stable/std/iter/struct.StepBy.html#impl-FusedIterator-for-StepBy%3CI%3E)
- [`std::fs::set_times`](https://doc.rust-lang.org/stable/std/fs/fn.set_times.html)
- [`std::fs::set_times_nofollow`](https://doc.rust-lang.org/stable/std/fs/fn.set_times_nofollow.html)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.99.0) and [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-199-2026-10-01).

## Contributors to 1.99.0

Many people came together to create Rust 1.99.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.99.0/)
