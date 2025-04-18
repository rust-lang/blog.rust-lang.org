+++
path = "2024/03/21/Rust-1.77.0"
title = "Announcing Rust 1.77.0"
authors = ["The Rust Release Team"]
aliases = [
    "2024/03/21/Rust-1.77.0.html",
    "releases/1.77.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.77.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.77.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.77.0](https://doc.rust-lang.org/nightly/releases.html#version-77-2024-03-21).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.77.0 stable

This release is relatively minor, but as always, even incremental improvements lead to a greater whole. A few of those changes are highlighted in this post, and others may yet fill more niche needs.

### C-string literals

Rust now supports C-string literals (`c"abc"`) which expand to a nul-byte
terminated string in memory of type `&'static CStr`. This makes it easier to write code
interoperating with foreign language interfaces which require nul-terminated
strings, with all of the relevant error checking (e.g., lack of interior nul
byte) performed at compile time.

### Support for recursion in `async fn`

Async functions previously could not call themselves due to a compiler
limitation. In 1.77, that limitation has been lifted, so recursive calls are
permitted so long as they use some form of indirection to avoid an infinite
size for the state of the function.

This means that code like this now works:

```rust
async fn fib(n: u32) -> u32 {
   match n {
       0 | 1 => 1,
       _ => Box::pin(fib(n-1)).await + Box::pin(fib(n-2)).await
   }
}
```

### `offset_of!`

1.77.0 stabilizes [`offset_of!`] for struct fields, which provides access to the
byte offset of the relevant public field of a struct. This macro is most useful
when the offset of a field is required without an existing instance of a type.
Implementing such a macro is already possible on stable, but without an
instance of the type the implementation would require tricky unsafe code which
makes it easy to accidentally introduce undefined behavior.

Users can now access the offset of a public field with `offset_of!(StructName,
field)`. This expands to a `usize` expression with the offset in bytes from the
start of the struct.

[`offset_of!`]: https://doc.rust-lang.org/stable/std/mem/macro.offset_of.html

### Enable strip in release profiles by default

Cargo [profiles](https://doc.rust-lang.org/stable/cargo/reference/profiles.html)
which do not enable [debuginfo](https://doc.rust-lang.org/stable/cargo/reference/profiles.html#debug) in
outputs (e.g., `debug = 0`) will enable `strip = "debuginfo"` by default.

This is primarily needed because the (precompiled) standard library ships with
debuginfo, which means that statically linked results would include the
debuginfo from the standard library even if the local compilations didn't
explicitly request debuginfo.

Users which do want debuginfo can explicitly enable it with the
[debug](https://doc.rust-lang.org/stable/cargo/reference/profiles.html#debug)
flag in the relevant Cargo profile.

### Stabilized APIs

- [`array::each_ref`](https://doc.rust-lang.org/stable/std/primitive.array.html#method.each_ref)
- [`array::each_mut`](https://doc.rust-lang.org/stable/std/primitive.array.html#method.each_mut)
- [`core::net`](https://doc.rust-lang.org/stable/core/net/index.html)
- [`f32::round_ties_even`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.round_ties_even)
- [`f64::round_ties_even`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.round_ties_even)
- [`mem::offset_of!`](https://doc.rust-lang.org/stable/std/mem/macro.offset_of.html)
- [`slice::first_chunk`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.first_chunk)
- [`slice::first_chunk_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.first_chunk_mut)
- [`slice::split_first_chunk`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_first_chunk)
- [`slice::split_first_chunk_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_first_chunk_mut)
- [`slice::last_chunk`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.last_chunk)
- [`slice::last_chunk_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.last_chunk_mut)
- [`slice::split_last_chunk`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_last_chunk)
- [`slice::split_last_chunk_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_last_chunk_mut)
- [`slice::chunk_by`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.chunk_by)
- [`slice::chunk_by_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.chunk_by_mut)
- [`Bound::map`](https://doc.rust-lang.org/stable/std/ops/enum.Bound.html#method.map)
- [`File::create_new`](https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.create_new)
- [`Mutex::clear_poison`](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.clear_poison)
- [`RwLock::clear_poison`](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html#method.clear_poison)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.77.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-177-2024-03-21), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-177).

## Contributors to 1.77.0

Many people came together to create Rust 1.77.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.77.0/)
