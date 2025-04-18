+++
path = "2017/08/31/Rust-1.20"
title = "Announcing Rust 1.20"
authors = ["The Rust Core Team"]
aliases = [
    "2017/08/31/Rust-1.20.html",
    "releases/1.20.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.20.0. Rust
is a systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.20 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.20.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1200-2017-08-31

### What's in 1.20.0 stable

In previous Rust versions, you can already define traits, structs, and enums
that have "associated functions":

```rust
struct Struct;

impl Struct {
    fn foo() {
        println!("foo is an associated function of Struct");
    }
}

fn main() {
    Struct::foo();
}
```

These are called "associated functions" because they are functions that are
associated with the type, that is, they're attached to the type itself, and
not any particular instance.

Rust 1.20 adds the ability to define "associated constants" as well:

```rust
struct Struct;

impl Struct {
    const ID: u32 = 0;
}

fn main() {
    println!("the ID of Struct is: {}", Struct::ID);
}
```

That is, the constant `ID` is associated with `Struct`. Like functions,
associated constants work with traits and enums as well.

Traits have an extra ability with associated constants that gives them some
extra power. With a trait, you can use an associated constant in the same way
you'd use an associated type: by declaring it, but not giving it a value. The
implementor of the trait then declares its value upon implementation:

```rust
trait Trait {
    const ID: u32;
}

struct Struct;

impl Trait for Struct {
    const ID: u32 = 5;
}

fn main() {
    println!("{}", Struct::ID);
}
```

Before this release, if you wanted to make a trait that represented floating
point numbers, you'd have to write this:

```rust
trait Float {
    fn nan() -> Self;
    fn infinity() -> Self;
    ...
}
```

This is slightly unwieldy, but more importantly, because they're functions, they
cannot be used in constant expressions, even though they only return a constant.
Because of this, a design for `Float` would also have to include constants as well:

```rust
mod f32 {
    const NAN: f32 = 0.0f32 / 0.0f32;
    const INFINITY: f32 = 1.0f32 / 0.0f32;

    impl Float for f32 {
        fn nan() -> Self {
            f32::NAN
        }
        fn infinity() -> Self {
            f32::INFINITY
        }
    }
}
```

Associated constants let you do this in a much cleaner way. This trait definition:

```rust
trait Float {
    const NAN: Self;
    const INFINITY: Self;
    ...
}
```

Leads to this implementation:

```rust
mod f32 {
    impl Float for f32 {
        const NAN: f32 = 0.0f32 / 0.0f32;
        const INFINITY: f32 = 1.0f32 / 0.0f32;
    }
}
```

much cleaner, and more versatile.

Associated constants were proposed in [RFC 195], almost exactly three years ago. It's
been quite a while for this feature! That RFC contained all associated items, not just
constants, and so some of them, such as associated types, were implemented faster than
others. In general, we've been doing a lot of internal work for constant evaluation,
to increase Rust's capabilities for compile-time metaprogramming. Expect more on this
front in the future.

[RFC 195]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md

We've also [fixed a bug] with the `include!` macro in documentation tests: for relative
paths, it erroneously was relative to the working directory, rather than to the current file.

[fixed a bug]: https://github.com/rust-lang/rust/pull/43782

See the [detailed release notes][notes] for more.

#### Library stabilizations

There's nothing *super* exciting in libraries this release, just a number of solid
improvements and continued stabilizing of APIs.

The `unimplemented!` macro [now accepts
messages](https://github.com/rust-lang/rust/pull/42155) that let you say why
something is not yet implemented.

We [upgraded to Unicode 10.0.0](https://github.com/rust-lang/rust/pull/42999).

`min` and `max` on floating point types were [rewritten in
Rust](https://github.com/rust-lang/rust/pull/42430), no longer relying on
`cmath`.

We are shipping mitigations against [Stack
Clash](https://access.redhat.com/security/vulnerabilities/stackguard) in this
release, notably, [stack probes], and [skipping the main thread's manual
stack guard on Linux]. You don't need to do anything to get these protections
other than using Rust 1.20.

[stack probes]: https://github.com/rust-lang/rust/pull/42816
[skipping the main thread's manual stack guard on Linux]: https://github.com/rust-lang/rust/pull/43072

We've added a new trio of sorting functions to the standard library:
[`slice::sort_unstable_by_key`], [`slice::sort_unstable_by`], and
[`slice::sort_unstable`]. You'll note that these all have "unstable" in the name.
Stability is a property of sorting algorithms that may or may not matter to you,
but now you have both options! Here's a brief summary: imagine we had a list
of words like this:

```
rust
crate
package
cargo
```

Two of these words, `cargo` and `crate`, both start with the letter `c`. A stable
sort that sorts only on the first letter must produce this result:

```
crate
cargo
package
rust
```

That is, because `crate` came before `cargo` in the original list, it must also be
before it in the final list. An unstable sort could provide that result, but could
also give this answer too:

```
cargo
crate
package
rust
```

That is, the results *may* not be in the same original order.

As you might imagine, fewer constraints often means faster results. If you don't care
about stability, these sorts may be faster for you than the stable variants. As always,
best to check both and see! These functions were added by [RFC 1884], if you'd like
more details, including benchmarks.

[RFC 1884]: https://github.com/rust-lang/rfcs/blob/master/text/1884-unstable-sort.md

Additionally, the following APIs were also stabilized:

- [`CStr::into_c_string`]
- [`CString::as_c_str`] and [`CString::into_boxed_c_str`]
- [`Chain::get_mut`], [`Chain::get_ref`], and [`Chain::into_inner`]
- [`Option::get_or_insert_with`] and [`Option::get_or_insert`]
- [`OsStr::into_os_string`]
- [`OsString::into_boxed_os_str`]
- [`Take::get_mut`] and [`Take::get_ref`]
- [`Utf8Error::error_len`]
- [`char::EscapeDebug`] and [`char::escape_debug`]
- [`compile_error!`]
- [`f32::from_bits`] and [`f32::to_bits`]
- [`f64::from_bits`] and [`f64::to_bits`]
- [`mem::ManuallyDrop`]
- [`str::from_boxed_utf8_unchecked`]
- [`str::as_bytes_mut`]
- [`str::from_utf8_mut`] and [`str::from_utf8_unchecked_mut`]
- [`str::get_unchecked`] and [`str::get_unchecked_mut`]
- [`str::get`] and [`str::get_mut`]
- [`str::into_boxed_bytes`]

[`CStr::into_c_string`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.into_c_string
[`CString::as_c_str`]: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_c_str
[`CString::into_boxed_c_str`]: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.into_boxed_c_str
[`Chain::get_mut`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.get_mut
[`Chain::get_ref`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.get_ref
[`Chain::into_inner`]: https://doc.rust-lang.org/std/io/struct.Chain.html#method.into_inner
[`Option::get_or_insert_with`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with
[`Option::get_or_insert`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert
[`OsStr::into_os_string`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html#method.into_os_string
[`OsString::into_boxed_os_str`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html#method.into_boxed_os_str
[`Take::get_mut`]: https://doc.rust-lang.org/std/io/struct.Take.html#method.get_mut
[`Take::get_ref`]: https://doc.rust-lang.org/std/io/struct.Take.html#method.get_ref
[`Utf8Error::error_len`]: https://doc.rust-lang.org/std/str/struct.Utf8Error.html#method.error_len
[`char::EscapeDebug`]: https://doc.rust-lang.org/std/char/struct.EscapeDebug.html
[`char::escape_debug`]: https://doc.rust-lang.org/std/primitive.char.html#method.escape_debug
[`compile_error!`]: https://doc.rust-lang.org/std/macro.compile_error.html
[`f32::from_bits`]: https://doc.rust-lang.org/std/primitive.f32.html#method.from_bits
[`f32::to_bits`]: https://doc.rust-lang.org/std/primitive.f32.html#method.to_bits
[`f64::from_bits`]: https://doc.rust-lang.org/std/primitive.f64.html#method.from_bits
[`f64::to_bits`]: https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits
[`mem::ManuallyDrop`]: https://doc.rust-lang.org/std/mem/union.ManuallyDrop.html
[`slice::sort_unstable_by_key`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by_key
[`slice::sort_unstable_by`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by
[`slice::sort_unstable`]: https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable
[`str::from_boxed_utf8_unchecked`]: https://doc.rust-lang.org/std/str/fn.from_boxed_utf8_unchecked.html
[`str::as_bytes_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes_mut
[`str::from_utf8_mut`]: https://doc.rust-lang.org/std/str/fn.from_utf8_mut.html
[`str::from_utf8_unchecked_mut`]: https://doc.rust-lang.org/std/str/fn.from_utf8_unchecked_mut.html
[`str::get_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_mut
[`str::get_unchecked_mut`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_unchecked_mut
[`str::get_unchecked`]: https://doc.rust-lang.org/std/primitive.str.html#method.get_unchecked
[`str::get`]: https://doc.rust-lang.org/std/primitive.str.html#method.get
[`str::into_boxed_bytes`]: https://doc.rust-lang.org/std/primitive.str.html#method.into_boxed_bytes

See the [detailed release notes][notes] for more.

#### Cargo features

Cargo has some nice upgrades this release. First of all, your crates.io
authentication token used to be stored in `~/.cargo/config`. As a configuration
file, this would often be stored with `644` permissions, that is, world-readable.
But it has a secret token in it. We've [moved the token] to `~/.cargo/credentials`,
so that it can be permissioned `600`, and hidden from other users on your system.

[moved the token]: https://github.com/rust-lang/cargo/pull/3978

If you used secondary binaries in a Cargo package, you know that they're kept
in `src/bin`. However, sometimes, you want multiple secondary binaries that
have significant logic; in that case, you'd have `src/bin/client.rs` and
`src/bin/server.rs`, and any submodules for either of them would go in the
same directory. This is confusing. Instead, [we now conventionally support]
`src/bin/client/main.rs` and `src/bin/server/main.rs`, so that you can keep
larger binaries more separate from one another.

[we now conventionally support]: https://github.com/rust-lang/cargo/pull/4214

See the [detailed release notes][notes] for more.

### Contributors to 1.20.0

Many people came together to create Rust 1.20. We couldn't have done it without
all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.20.0)
