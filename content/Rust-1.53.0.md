+++
path = "2021/06/17/Rust-1.53.0"
title = "Announcing Rust 1.53.0"
authors = ["The Rust Release Team"]
aliases = ["2021/06/17/Rust-1.53.0.html"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.53.0. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.53.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.53.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1530-2021-06-17

## What's in 1.53.0 stable

This release contains several new language features and many new library features,
including the long-awaited `IntoIterator` implementation for arrays.
See the [detailed release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1530-2021-06-17)
to learn about other changes not covered by this post.

### IntoIterator for arrays

This is the first Rust release in which arrays implement the `IntoIterator` trait.
This means you can now iterate over arrays by value:

```rust
for i in [1, 2, 3] {
    ..
}
```

Previously, this was only possible by reference, using `&[1, 2, 3]` or `[1, 2, 3].iter()`.

Similarly, you can now pass arrays to methods expecting a `T: IntoIterator`:

```rust
let set = BTreeSet::from_iter([1, 2, 3]);
```

```rust
for (a, b) in some_iterator.chain([1]).zip([1, 2, 3]) {
    ..
}
```

This was not implemented before, due to backwards compatibility problems.
Because `IntoIterator` was already implemented for references to arrays,
`array.into_iter()` already compiled in earlier versions,
resolving to `(&array).into_iter()`.

As of this release, arrays implement `IntoIterator` with a small workaround to avoid breaking code.
The compiler will continue to resolve `array.into_iter()` to `(&array).into_iter()`,
as if the trait implementation does not exist.
This only applies to the `.into_iter()` method call syntax, and does not
affect any other syntax such as `for e in [1, 2, 3]`, `iter.zip([1, 2, 3])` or
`IntoIterator::into_iter([1, 2, 3])`, which all compile fine.

Since this special case for `.into_iter()` is only required to avoid breaking existing code,
it is removed in the new edition, Rust 2021, which will be released later this year.
See [the edition announcement](https://blog.rust-lang.org/2021/05/11/edition-2021.html#intoiterator-for-arrays)
for more information.

### Or patterns

Pattern syntax has been extended to support `|` nested anywhere in the pattern.
This enables you to write `Some(1 | 2)` instead of `Some(1) | Some(2)`.

```rust
match result {
     Ok(Some(1 | 2)) => { .. }
     Err(MyError { kind: FileNotFound | PermissionDenied, .. }) => { .. }
     _ => { .. }
}
```

### Unicode identifiers

Identifiers can now contain non-ascii characters.
All valid identifier characters in Unicode as defined in [UAX #31](https://unicode.org/reports/tr31/) can now be used.
That includes characters from many different scripts and languages, but does not include emoji.

For example:

```rust
const BLÅHAJ: &str = "🦈";

struct 人 {
    名字: String,
}

let α = 1;
```

The compiler will warn about potentially confusing situations involving different scripts.
For example, using identifiers that look very similar will result in a warning.

```
warning: identifier pair considered confusable between `ｓ` and `s`
```

### HEAD branch name support in Cargo

Cargo no longer assumes the default `HEAD` of git repositories is named `master`.
This means you no longer need to specify `branch = "main"` for git dependencies
from a repository where the default branch is called `main`.

### Incremental Compilation remains off by default

As previously discussed on the [blog post for version 1.52.1](/2021/05/10/Rust-1.52.1/), incremental compilation has been turned off by default on the stable Rust release channel. The feature remains available on the beta and nightly release channels. For the 1.53.0 stable release, the method for reenabling incremental is unchanged from 1.52.1.

### Stabilized APIs

The following methods and trait implementations were stabilized.

- [`array::from_ref`](https://doc.rust-lang.org/stable/std/array/fn.from_ref.html)
- [`array::from_mut`](https://doc.rust-lang.org/stable/std/array/fn.from_mut.html)
- [`AtomicBool::fetch_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicBool.html#method.fetch_update)
- [`AtomicPtr::fetch_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.fetch_update)
- [`BTreeSet::retain`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html#method.retain)
- [`BTreeMap::retain`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html#method.retain)
- [`BufReader::seek_relative`](https://doc.rust-lang.org/stable/std/io/struct.BufReader.html#method.seek_relative)
- [`cmp::min_by`](https://doc.rust-lang.org/stable/std/cmp/fn.min_by.html)
- [`cmp::min_by_key`](https://doc.rust-lang.org/stable/std/cmp/fn.min_by_key.html)
- [`cmp::max_by`](https://doc.rust-lang.org/stable/std/cmp/fn.max_by.html)
- [`cmp::max_by_key`](https://doc.rust-lang.org/stable/std/cmp/fn.max_by_key.html)
- [`DebugStruct::finish_non_exhaustive`](https://doc.rust-lang.org/stable/std/fmt/struct.DebugStruct.html#method.finish_non_exhaustive)
- [`Duration::ZERO`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#associatedconstant.ZERO)
- [`Duration::MAX`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#associatedconstant.MAX)
- [`Duration::is_zero`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.is_zero)
- [`Duration::saturating_add`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.saturating_add)
- [`Duration::saturating_sub`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.saturating_sub)
- [`Duration::saturating_mul`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.saturating_mul)
- [`f32::is_subnormal`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.is_subnormal)
- [`f64::is_subnormal`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.is_subnormal)
- [`IntoIterator for array`](https://doc.rust-lang.org/stable/std/primitive.array.html#impl-IntoIterator)
- [`{integer}::BITS`](https://doc.rust-lang.org/stable/std/primitive.usize.html#associatedconstant.BITS)
- [`io::Error::Unsupported`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.Unsupported)
- [`NonZero*::leading_zeros`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.leading_zeros)
- [`NonZero*::trailing_zeros`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.trailing_zeros)
- [`Option::insert`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.insert)
- [`Ordering::is_eq`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_eq)
- [`Ordering::is_ne`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_ne)
- [`Ordering::is_lt`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_lt)
- [`Ordering::is_gt`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_gt)
- [`Ordering::is_le`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_le)
- [`Ordering::is_ge`](https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html#method.is_ge)
- [`OsStr::make_ascii_lowercase`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.make_ascii_lowercase)
- [`OsStr::make_ascii_uppercase`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.make_ascii_uppercase)
- [`OsStr::to_ascii_lowercase`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.to_ascii_lowercase)
- [`OsStr::to_ascii_uppercase`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.to_ascii_uppercase)
- [`OsStr::is_ascii`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.is_ascii)
- [`OsStr::eq_ignore_ascii_case`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.eq_ignore_ascii_case)
- [`Peekable::peek_mut`](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html#method.peek_mut)
- [`Rc::increment_strong_count`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.increment_strong_count)
- [`Rc::decrement_strong_count`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.decrement_strong_count)
- [`slice::IterMut::as_slice`](https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html#method.as_slice)
- [`AsRef<[T]> for slice::IterMut`](https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html#impl-AsRef%3C%5BT%5D%3E)
- [`impl SliceIndex for (Bound<usize>, Bound<usize>)`](https://doc.rust-lang.org/stable/std/primitive.tuple.html#impl-SliceIndex%3C%5BT%5D%3E)
- [`Vec::extend_from_within`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.extend_from_within)

### Other changes

There are other changes in the Rust 1.53.0 release:
check out what changed in
[Rust](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1530-2021-06-17),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-153-2021-06-17),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-153).

### Contributors to 1.53.0

Many people came together to create Rust 1.53.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.53.0/)
