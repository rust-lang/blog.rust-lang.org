+++
path = "2023/11/16/Rust-1.74.0"
title = "Announcing Rust 1.74.0"
authors = ["The Rust Release Team"]
aliases = [
    "2023/11/16/Rust-1.74.0.html",
    "releases/1.74.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.74.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.74.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.74.0](https://github.com/rust-lang/rust/releases/tag/1.74.0) on GitHub.

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.74.0 stable

### Lint configuration through Cargo

As proposed in [RFC 3389](https://rust-lang.github.io/rfcs/3389-manifest-lint.html),
the `Cargo.toml` manifest now supports a `[lints]` table to configure the
reporting level (forbid, deny, warn, allow) for lints from the compiler and
other tools. So rather than setting `RUSTFLAGS` with `-F`/`-D`/`-W`/`-A`, which
would affect the entire build, or using crate-level attributes like:

```rust
#![forbid(unsafe_code)]
#![deny(clippy::enum_glob_use)]
```

You can now write those in your package manifest for Cargo to handle:

```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
enum_glob_use = "deny"
```

These can also be configured in a `[workspace.lints]` table, then inherited by
`[lints] workspace = true` like many other workspace settings. Cargo will also
track changes to these settings when deciding which crates need to be rebuilt.

For more information, see the [`lints`] and [`workspace.lints`] sections of the
Cargo reference manual.

[`lints`]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-lints-section
[`workspace.lints`]: https://doc.rust-lang.org/stable/cargo/reference/workspaces.html#the-lints-table

### Cargo Registry Authentication

Two more related Cargo features are included in this release: credential
providers and authenticated private registries.

Credential providers allow configuration of how Cargo gets credentials for a
registry. Built-in providers are included for OS-specific secure secret storage
on Linux, macOS, and Windows. Additionally, custom providers can be written to
support arbitrary methods of storing or generating tokens. Using a secure
credential provider reduces risk of registry tokens leaking.

Registries can now optionally require authentication for all operations, not
just publishing. This enables private Cargo registries to offer more secure
hosting of crates. Use of private registries requires the configuration of a
credential provider.

For further information, see the
[Cargo docs](https://doc.rust-lang.org/beta/cargo/reference/registry-authentication.html).

### Projections in opaque return types

If you have ever received the error that a "return type cannot contain a
projection or `Self` that references lifetimes from a parent scope," you may
now rest easy! The compiler now allows mentioning `Self` and
associated types in opaque return types, like `async fn` and `-> impl Trait`.
This is the kind of feature that gets Rust closer to how you might just
_expect_ it to work, even if you have no idea about jargon like "projection".

This functionality had an unstable feature gate because its implementation
originally didn't properly deal with captured lifetimes, and once that was
fixed it was given time to make sure it was sound. For more technical details,
see the [stabilization pull request][115659], which describes the following
examples that are all now allowed:

```rust
struct Wrapper<'a, T>(&'a T);

// Opaque return types that mention `Self`:
impl Wrapper<'_, ()> {
    async fn async_fn() -> Self { /* ... */ }
    fn impl_trait() -> impl Iterator<Item = Self> { /* ... */ }
}

trait Trait<'a> {
    type Assoc;
    fn new() -> Self::Assoc;
}
impl Trait<'_> for () {
    type Assoc = ();
    fn new() {}
}

// Opaque return types that mention an associated type:
impl<'a, T: Trait<'a>> Wrapper<'a, T> {
    async fn mk_assoc() -> T::Assoc { /* ... */ }
    fn a_few_assocs() -> impl Iterator<Item = T::Assoc> { /* ... */ }
}
```

[115659]: https://github.com/rust-lang/rust/pull/115659

### Stabilized APIs

- [`core::num::Saturating`](https://doc.rust-lang.org/stable/std/num/struct.Saturating.html)
- [`impl From<io::Stdout> for std::process::Stdio`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html#impl-From%3CStdout%3E-for-Stdio)
- [`impl From<io::Stderr> for std::process::Stdio`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html#impl-From%3CStderr%3E-for-Stdio)
- [`impl From<OwnedHandle> for std::process::Child{Stdin, Stdout, Stderr}`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html#impl-From%3CStderr%3E-for-Stdio)
- [`impl From<OwnedFd> for std::process::Child{Stdin, Stdout, Stderr}`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html#impl-From%3CStderr%3E-for-Stdio)
- [`std::ffi::OsString::from_encoded_bytes_unchecked`](https://doc.rust-lang.org/stable/std/ffi/struct.OsString.html#method.from_encoded_bytes_unchecked)
- [`std::ffi::OsString::into_encoded_bytes`](https://doc.rust-lang.org/stable/std/ffi/struct.OsString.html#method.into_encoded_bytes)
- [`std::ffi::OsStr::from_encoded_bytes_unchecked`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.from_encoded_bytes_unchecked)
- [`std::ffi::OsStr::as_encoded_bytes`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.as_encoded_bytes)
- [`std::io::Error::other`](https://doc.rust-lang.org/stable/std/io/struct.Error.html#method.other)
- [`impl TryFrom<char> for u16`](https://doc.rust-lang.org/stable/std/primitive.u16.html#impl-TryFrom%3Cchar%3E-for-u16)
- [`impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T>`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-From%3C%26%5BT;+N%5D%3E-for-Vec%3CT,+Global%3E)
- [`impl<T: Clone, const N: usize> From<&mut [T; N]> for Vec<T>`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-From%3C%26mut+%5BT;+N%5D%3E-for-Vec%3CT,+Global%3E)
- [`impl<T, const N: usize> From<[T; N]> for Arc<[T]>`](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#impl-From%3C%5BT;+N%5D%3E-for-Arc%3C%5BT%5D,+Global%3E)
- [`impl<T, const N: usize> From<[T; N]> for Rc<[T]>`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#impl-From%3C%5BT;+N%5D%3E-for-Rc%3C%5BT%5D,+Global%3E)

These APIs are now stable in const contexts:

- [`core::mem::transmute_copy`](https://doc.rust-lang.org/stable/std/mem/fn.transmute_copy.html)
- [`str::is_ascii`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.is_ascii)
- [`[u8]::is_ascii`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.is_ascii)

### Compatibility notes

* As [previously announced][apple-min], Rust 1.74 has increased its
  requirements on Apple platforms. The minimum versions are now:
    - macOS: 10.12 Sierra (First released 2016)
    - iOS: 10 (First released 2016)
    - tvOS: 10 (First released 2016)

[apple-min]: https://blog.rust-lang.org/2023/09/25/Increasing-Apple-Version-Requirements.html

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.74.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-174-2023-11-16), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-174).

## Contributors to 1.74.0

Many people came together to create Rust 1.74.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.74.0/)
