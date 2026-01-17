+++
path = "2026/01/22/Rust-1.93.0"
title = "Announcing Rust 1.93.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.93.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.93.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.93.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.93.0](https://doc.rust-lang.org/stable/releases.html#version-1930-2026-01-22).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.93.0 stable

### Update bundled musl to 1.2.5

The various `*-linux-musl` targets now all [ship](https://github.com/rust-lang/rust/pull/142682) with musl 1.2.5. This primarily affects static musl builds for `x86_64`, `aarch64`, and `powerpc64le` which bundled musl 1.2.3. This update comes with [several fixes and improvements](https://musl.libc.org/releases.html), and a breaking change that affects the Rust ecosystem.

For the Rust ecosystem, the primary motivation for this update is to receive major improvements to
musl's DNS resolver which shipped in 1.2.4 and received bug fixes in 1.2.5. When using `musl`
targets for static linking, this should make portable Linux binaries that do networking more
reliable, particularly in the face of large DNS records and recursive nameservers.

However, 1.2.4 also comes with a breaking change: [the removal of several legacy compatibility symbols that the Rust libc crate was using](https://github.com/rust-lang/libc/issues/2934).
A fix for this [was shipped in libc 0.2.146 in June 2023 (2.5 years ago)](https://github.com/rust-lang/libc/pull/2935),
and we believe has sufficiently widely propagated that we're ready to make the change in Rust
targets.

See our previous [announcement](https://blog.rust-lang.org/2025/12/05/Updating-musl-1.2.5/) for more details.

### Allow the global allocator to use thread-local storage

Rust 1.93 adjusts the internals of the standard library to permit global allocators written in Rust
to use std's [`thread_local!`](https://doc.rust-lang.org/stable/std/macro.thread_local.html) and
[`std::thread::current`](https://doc.rust-lang.org/stable/std/thread/fn.current.html) without
re-entrancy concerns by using the system allocator instead.

See [docs](https://doc.rust-lang.org/nightly/std/alloc/trait.GlobalAlloc.html#re-entrance) for details.

### `cfg` attributes on `asm!` lines

Previously, if individual parts of a section of inline assembly needed to be `cfg`'d, the full `asm!`
block would need to be repeated with and without that section. In 1.93, `cfg` can now be applied to
individual statements within the `asm!` block.

```rust
asm!( // or global_asm! or naked_asm!
    "nop",
    #[cfg(target_feature = "sse2")]
    "nop",
    // ...
    #[cfg(target_feature = "sse2")]
    a = const 123, // only used on sse2
);
```

### Stabilized APIs

- [`<MaybeUninit<T>>::assume_init_drop`](https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#method.assume_init_drop)
- [`<MaybeUninit<T>>::assume_init_ref`](https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#method.assume_init_ref)
- [`<MaybeUninit<T>>::assume_init_mut`](https://doc.rust-lang.org/stable/core/mem/union.MaybeUninit.html#method.assume_init_mut)
- [`<[MaybeUninit<T>]>::write_copy_of_slice`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.write_copy_of_slice)
- [`<[MaybeUninit<T>]>::write_clone_of_slice`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.write_clone_of_slice)
- [`String::into_raw_parts`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.into_raw_parts)
- [`Vec::into_raw_parts`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.into_raw_parts)
- [`<uN>::unchecked_add`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.unchecked_add)
- [`<uN>::unchecked_sub`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.unchecked_sub)
- [`<uN>::unchecked_mul`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.unchecked_mul)
- [`<iN>::unchecked_add`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.unchecked_add)
- [`<iN>::unchecked_sub`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.unchecked_sub)
- [`<iN>::unchecked_mul`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.unchecked_mul)
- [`<[T]>::as_array`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_array)
- [`<[T]>::as_array_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_mut_array)
- [`<*const [T]>::as_array`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_array)
- [`<*mut [T]>::as_array_mut`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_mut_array)
- [`VecDeque::pop_front_if`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.pop_front_if)
- [`VecDeque::pop_back_if`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.pop_back_if)
- [`Duration::from_nanos_u128`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.from_nanos_u128)
- [`char::MAX_LEN_UTF8`](https://doc.rust-lang.org/stable/std/primitive.char.html#associatedconstant.MAX_LEN_UTF8)
- [`char::MAX_LEN_UTF16`](https://doc.rust-lang.org/stable/std/primitive.char.html#associatedconstant.MAX_LEN_UTF16)
- [`std::fmt::from_fn`](https://doc.rust-lang.org/stable/std/fmt/fn.from_fn.html)
- [`std::fmt::FromFn`](https://doc.rust-lang.org/stable/std/fmt/struct.FromFn.html)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.93.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-193-2026-01-22), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-193).

## Contributors to 1.93.0

Many people came together to create Rust 1.93.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.93.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
