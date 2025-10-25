+++
path = "2025/10/30/Rust-1.91.0"
title = "Announcing Rust 1.91.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.91.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.91.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.91.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.91.0](https://doc.rust-lang.org/stable/releases.html#version-1910-2025-10-30).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.91.0 stable

### `aarch64-pc-windows-msvc` is now a Tier 1 platform

The Rust compiler supports [a wide variety of targets][platform-support], but
the Rust Team can't provide the same level of support for all of them. To
clearly mark how supported each target is, we use a tiering system:

* Tier 3 targets are technically supported by the compiler, but we don't check
  whether their code build or passes the tests, and we don't provide any
  prebuilt binaries as part of our releases.
* Tier 2 targets are guaranteed to build and we provide prebuilt binaries, but
  we don't execute the test suite on those platforms: the produced binaries
  might not work or might have bugs.
* Tier 1 targets provide the highest support guarantee, and we run the full
  suite on those platforms for every change merged in the compiler. Prebuilt
  binaries are also available.

Rust 1.91.0 promotes the `aarch64-pc-windows-msvc` target to Tier 1 support,
bringing our highest guarantees to users of 64-bit ARM systems running Windows.

### Add lint against dangling raw pointers from local variables

While Rust's borrow checking prevents dangling references from being returned, it doesn't
track raw pointers. With this release, we are adding a warn-by-default lint on raw
pointers to local variables being returned from functions. For example, code like this:

```rust
fn f() -> *const u8 {
    let x = 0;
    &x
}
```

will now produce a lint:

```
warning: a dangling pointer will be produced because the local variable `x` will be dropped
 --> src/lib.rs:3:5
  |
1 | fn f() -> *const u8 {
  |           --------- return type of the function is `*const u8`
2 |     let x = 0;
  |         - `x` is part the function and will be dropped at the end of the function
3 |     &x
  |     ^^
  |
  = note: pointers do not have a lifetime; after returning, the `u8` will be deallocated
    at the end of the function because nothing is referencing it as far as the type system is
    concerned
  = note: `#[warn(dangling_pointers_from_locals)]` on by default
```

Note that the code above is not unsafe, as it itself doesn't perform any dangerous
operations. Only dereferencing the raw pointer after the function returns would be
unsafe. We expect future releases of Rust to add more functionality helping authors
to safely interact with raw pointers, and with unsafe code more generally.

### Stabilized APIs

- [`Path::file_prefix`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.file_prefix)
- [`AtomicPtr::fetch_ptr_add`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_ptr_add)
- [`AtomicPtr::fetch_ptr_sub`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_ptr_sub)
- [`AtomicPtr::fetch_byte_add`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_byte_add)
- [`AtomicPtr::fetch_byte_sub`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_byte_sub)
- [`AtomicPtr::fetch_or`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_or)
- [`AtomicPtr::fetch_and`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_and)
- [`AtomicPtr::fetch_xor`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_xor)
- [`{integer}::strict_add`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_add)
- [`{integer}::strict_sub`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_sub)
- [`{integer}::strict_mul`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_mul)
- [`{integer}::strict_div`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_div)
- [`{integer}::strict_div_euclid`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_div_euclid)
- [`{integer}::strict_rem`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_rem)
- [`{integer}::strict_rem_euclid`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_rem_euclid)
- [`{integer}::strict_neg`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_neg)
- [`{integer}::strict_shl`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_shl)
- [`{integer}::strict_shr`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_shr)
- [`{integer}::strict_pow`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_pow)
- [`i{N}::strict_add_unsigned`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.strict_add_unsigned)
- [`i{N}::strict_sub_unsigned`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.strict_sub_unsigned)
- [`i{N}::strict_abs`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.strict_abs)
- [`u{N}::strict_add_signed`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_add_signed)
- [`u{N}::strict_sub_signed`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.strict_sub_signed)
- [`PanicHookInfo::payload_as_str`](https://doc.rust-lang.org/stable/std/panic/struct.PanicHookInfo.html#method.payload_as_str)
- [`core::iter::chain`](https://doc.rust-lang.org/stable/core/iter/fn.chain.html)
- [`u{N}::checked_signed_diff`](https://doc.rust-lang.org/stable/std/primitive.u16.html#method.checked_signed_diff)
- [`core::array::repeat`](https://doc.rust-lang.org/stable/core/array/fn.repeat.html)
- [`PathBuf::add_extension`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.add_extension)
- [`PathBuf::with_added_extension`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.with_added_extension)
- [`Duration::from_mins`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.from_mins)
- [`Duration::from_hours`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.from_hours)
- [`impl PartialEq<str> for PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#impl-PartialEq%3Cstr%3E-for-PathBuf)
- [`impl PartialEq<String> for PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#impl-PartialEq%3CString%3E-for-PathBuf)
- [`impl PartialEq<str> for Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#impl-PartialEq%3Cstr%3E-for-Path)
- [`impl PartialEq<String> for Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#impl-PartialEq%3CString%3E-for-Path)
- [`impl PartialEq<PathBuf> for String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html#impl-PartialEq%3CPathBuf%3E-for-String)
- [`impl PartialEq<Path> for String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html#impl-PartialEq%3CPath%3E-for-String)
- [`impl PartialEq<PathBuf> for str`](https://doc.rust-lang.org/nightly/std/primitive.str.html#impl-PartialEq%3CPathBuf%3E-for-str)
- [`impl PartialEq<Path> for str`](https://doc.rust-lang.org/nightly/std/primitive.str.html#impl-PartialEq%3CPath%3E-for-str)
- [`Ipv4Addr::from_octets`](https://doc.rust-lang.org/stable/std/net/struct.Ipv4Addr.html#method.from_octets)
- [`Ipv6Addr::from_octets`](https://doc.rust-lang.org/stable/std/net/struct.Ipv6Addr.html#method.from_octets)
- [`Ipv6Addr::from_segments`](https://doc.rust-lang.org/stable/std/net/struct.Ipv6Addr.html#method.from_segments)
- [`impl<T> Default for Pin<Box<T>> where Box<T>: Default, T: ?Sized`](https://doc.rust-lang.org/stable/std/default/trait.Default.html#impl-Default-for-Pin%3CBox%3CT%3E%3E)
- [`impl<T> Default for Pin<Rc<T>> where Rc<T>: Default, T: ?Sized`](https://doc.rust-lang.org/stable/std/default/trait.Default.html#impl-Default-for-Pin%3CRc%3CT%3E%3E)
- [`impl<T> Default for Pin<Arc<T>> where Arc<T>: Default, T: ?Sized`](https://doc.rust-lang.org/stable/std/default/trait.Default.html#impl-Default-for-Pin%3CArc%3CT%3E%3E)
- [`Cell::as_array_of_cells`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.as_array_of_cells)
- [`u{N}::carrying_add`](https://doc.rust-lang.org/stable/std/primitive.u64.html#method.carrying_add)
- [`u{N}::borrowing_sub`](https://doc.rust-lang.org/stable/std/primitive.u64.html#method.borrowing_sub)
- [`u{N}::carrying_mul`](https://doc.rust-lang.org/stable/std/primitive.u64.html#method.carrying_mul)
- [`u{N}::carrying_mul_add`](https://doc.rust-lang.org/stable/std/primitive.u64.html#method.carrying_mul_add)
- [`BTreeMap::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html#method.extract_if)
- [`BTreeSet::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html#method.extract_if)
- [`impl Debug for windows::ffi::EncodeWide<'_>`](https://doc.rust-lang.org/stable/std/os/windows/ffi/struct.EncodeWide.html#impl-Debug-for-EncodeWide%3C'_%3E)
- [`str::ceil_char_boundary`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.ceil_char_boundary)
- [`str::floor_char_boundary`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.floor_char_boundary)
- [`impl Sum for Saturating<u{N}>`](https://doc.rust-lang.org/stable/std/num/struct.Saturating.html#impl-Sum-for-Saturating%3Cu32%3E)
- [`impl Sum<&u{N}> for Saturating<u{N}>`](https://doc.rust-lang.org/stable/std/num/struct.Saturating.html#impl-Sum%3C%26Saturating%3Cu32%3E%3E-for-Saturating%3Cu32%3E)
- [`impl Product for Saturating<u{N}>`](https://doc.rust-lang.org/stable/std/num/struct.Saturating.html#impl-Product-for-Saturating%3Cu32%3E)
- [`impl Product<&u{N}> for Saturating<u{N}>`](https://doc.rust-lang.org/stable/std/num/struct.Saturating.html#impl-Product%3C%26Saturating%3Cu32%3E%3E-for-Saturating%3Cu32%3E)

These previously stable APIs are now stable in const contexts:

- [`<[T; N]>::each_ref`](https://doc.rust-lang.org/stable/std/primitive.array.html#method.each_ref)
- [`<[T; N]>::each_mut`](https://doc.rust-lang.org/stable/std/primitive.array.html#method.each_mut)
- [`OsString::new`](https://doc.rust-lang.org/stable/std/ffi/struct.OsString.html#method.new)
- [`PathBuf::new`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#method.new)
- [`TypeId::of`](https://doc.rust-lang.org/stable/std/any/struct.TypeId.html#method.of)
- [`ptr::with_exposed_provenance`](https://doc.rust-lang.org/stable/std/ptr/fn.with_exposed_provenance.html)
- [`ptr::with_exposed_provenance_mut`](https://doc.rust-lang.org/stable/std/ptr/fn.with_exposed_provenance_mut.html)

### Platform Support

- [Promote `aarch64-pc-windows-msvc` to Tier 1](https://github.com/rust-lang/rust/pull/145682)
- [Promote `aarch64-pc-windows-gnullvm` and `x86_64-pc-windows-gnullvm` to Tier 2 with host tools.](https://github.com/rust-lang/rust/pull/143031)
  Note: llvm-tools and MSI installers are missing but will be added in future releases.

Refer to Rust’s [platform support page][platform-support] for more information on Rust’s tiered platform support.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.91.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-191-2025-10-30), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-191).

## Contributors to 1.91.0

Many people came together to create Rust 1.91.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.91.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
