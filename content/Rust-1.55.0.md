+++
path = "2021/09/09/Rust-1.55.0"
title = "Announcing Rust 1.55.0"
authors = ["The Rust Release Team"]
aliases = [
    "2021/09/09/Rust-1.55.0.html",
    "releases/1.55.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.55.0. Rust is a programming language empowering everyone
to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.55.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.55.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-55-2021-09-09

## What's in 1.55.0 stable

### Cargo deduplicates compiler errors

In past releases, when running `cargo test`, `cargo check --all-targets`, or similar commands which built the same Rust crate in multiple configurations, errors and warnings could show up duplicated as the rustc's were run in parallel and both showed the same warning.

For example, in 1.54.0, output like this was common:

```
$ cargo +1.54.0 check --all-targets
    Checking foo v0.1.0
warning: function is never used: `foo`
 --> src/lib.rs:9:4
  |
9 | fn foo() {}
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

warning: function is never used: `foo`
 --> src/lib.rs:9:4
  |
9 | fn foo() {}
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
```

In 1.55, this behavior has been adjusted to deduplicate and print a report at the end of compilation:

```
$ cargo +1.55.0 check --all-targets
    Checking foo v0.1.0
warning: function is never used: `foo`
 --> src/lib.rs:9:4
  |
9 | fn foo() {}
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `foo` (lib) generated 1 warning
warning: `foo` (lib test) generated 1 warning (1 duplicate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
```

### Faster, more correct float parsing

The standard library's implementation of float parsing has been updated to use the Eisel-Lemire algorithm, which brings both speed improvements and improved correctness. In the past, certain edge cases failed to parse, and this has now been fixed.

You can read more details on the new implementation [in the pull request description](https://github.com/rust-lang/rust/pull/86761).

### `std::io::ErrorKind` variants updated

[`std::io::ErrorKind`] is a [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) enum that classifies errors into portable categories, such as `NotFound` or `WouldBlock`. Rust code that has a [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) can call the [`kind` method](https://doc.rust-lang.org/std/io/struct.Error.html#method.kind) to obtain a `std::io::ErrorKind` and match on that to handle a specific error.

Not all errors are categorized into `ErrorKind` values; some are left uncategorized and placed in a catch-all variant. In previous versions of Rust, uncategorized errors used `ErrorKind::Other`; however, user-created `std::io::Error` values also commonly used `ErrorKind::Other`. In 1.55, uncategorized errors now use the internal variant `ErrorKind::Uncategorized`, which we intend to leave hidden and never available for stable Rust code to name explicitly; this leaves `ErrorKind::Other` exclusively for constructing `std::io::Error` values that don't come from the standard library. This enforces the `#[non_exhaustive]` nature of `ErrorKind`.

Rust code should never match `ErrorKind::Other` and expect any particular underlying error code; only match `ErrorKind::Other` if you're catching a constructed `std::io::Error` that uses that error kind. Rust code matching on `std::io::Error` should always use `_` for any error kinds it doesn't know about, in which case it can match the underlying error code, or report the error, or bubble it up to calling code.

We're making this change to smooth the way for introducing new ErrorKind variants in the future; those new variants will start out nightly-only, and only become stable later. This change ensures that code matching variants it doesn't know about must use a catch-all `_` pattern, which will work both with `ErrorKind::Uncategorized` and with future nightly-only variants.

[`std::io::ErrorKind`]: https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html

### Open range patterns added

Rust 1.55 stabilized using open ranges in patterns:

```rust
match x as u32 {
      0 => println!("zero!"),
      1.. => println!("positive number!"),
}
```

Read more details [here](https://github.com/rust-lang/rust/pull/83918).

### Stabilized APIs

The following methods and trait implementations were stabilized.

- [`Bound::cloned`]
- [`Drain::as_str`]
- [`IntoInnerError::into_error`]
- [`IntoInnerError::into_parts`]
- [`MaybeUninit::assume_init_mut`]
- [`MaybeUninit::assume_init_ref`]
- [`MaybeUninit::write`]
- [`array::map`]
- [`ops::ControlFlow`]
- [`x86::_bittest`]
- [`x86::_bittestandcomplement`]
- [`x86::_bittestandreset`]
- [`x86::_bittestandset`]
- [`x86_64::_bittest64`]
- [`x86_64::_bittestandcomplement64`]
- [`x86_64::_bittestandreset64`]
- [`x86_64::_bittestandset64`]

The following previously stable functions are now `const`.

- [`str::from_utf8_unchecked`]

[`array::map`]: https://doc.rust-lang.org/stable/std/primitive.array.html#method.map
[`Bound::cloned`]: https://doc.rust-lang.org/stable/std/ops/enum.Bound.html#method.cloned
[`Drain::as_str`]: https://doc.rust-lang.org/stable/std/string/struct.Drain.html#method.as_str
[`IntoInnerError::into_error`]: https://doc.rust-lang.org/stable/std/io/struct.IntoInnerError.html#method.into_error
[`IntoInnerError::into_parts`]: https://doc.rust-lang.org/stable/std/io/struct.IntoInnerError.html#method.into_parts
[`MaybeUninit::assume_init_mut`]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.assume_init_mut
[`MaybeUninit::assume_init_ref`]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.assume_init_ref
[`MaybeUninit::write`]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.write
[`Seek::rewind`]: https://doc.rust-lang.org/stable/std/io/trait.Seek.html#method.rewind
[`ops::ControlFlow`]: https://doc.rust-lang.org/stable/std/ops/enum.ControlFlow.html
[`str::from_utf8_unchecked`]: https://doc.rust-lang.org/stable/std/str/fn.from_utf8_unchecked.html
[`x86::_bittest`]: https://doc.rust-lang.org/stable/core/arch/x86/fn._bittest.html
[`x86::_bittestandcomplement`]: https://doc.rust-lang.org/stable/core/arch/x86/fn._bittestandcomplement.html
[`x86::_bittestandreset`]: https://doc.rust-lang.org/stable/core/arch/x86/fn._bittestandreset.html
[`x86::_bittestandset`]: https://doc.rust-lang.org/stable/core/arch/x86/fn._bittestandset.html
[`x86_64::_bittest64`]: https://doc.rust-lang.org/stable/core/arch/x86_64/fn._bittest64.html
[`x86_64::_bittestandcomplement64`]: https://doc.rust-lang.org/stable/core/arch/x86_64/fn._bittestandcomplement64.html
[`x86_64::_bittestandreset64`]: https://doc.rust-lang.org/stable/core/arch/x86_64/fn._bittestandreset64.html
[`x86_64::_bittestandset64`]: https://doc.rust-lang.org/stable/core/arch/x86_64/fn._bittestandset64.html

### Other changes

There are other changes in the Rust 1.55.0 release:
check out what changed in [Rust](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-55-2021-09-09), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-155-2021-09-09), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-155).

### Contributors to 1.55.0

Many people came together to create Rust 1.55.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.55.0/)

### Dedication

Anna Harren was a member of the community and contributor to Rust known for coining the term "Turbofish" to describe `::<>` syntax. Anna recently passed away after living with cancer. Her contribution will forever be remembered and be part of the language, and we dedicate this release to her memory.
