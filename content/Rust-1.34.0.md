+++
path = "2019/04/11/Rust-1.34.0"
title = "Announcing Rust 1.34.0"
authors = ["The Rust Release Team"]
aliases = [
    "2019/04/11/Rust-1.34.0.html",
    "releases/1.34.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.34.0. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.34.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the appropriate
page on our website.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1340-2019-04-11

## What's in 1.34.0 stable

The largest feature in this release is the introduction of *alternative* `cargo` registries.
The release also includes support for `?` in documentation tests,
some improvements for `#[attribute(..)]`s, as well as the stabilization of `TryFrom`.
Read on for a few highlights, or see the [detailed release notes][notes] for additional information.

### Alternative `cargo` registries

[crates.io]: http://crates.io/
[registry-docs]: https://doc.rust-lang.org/nightly/cargo/reference/registries.html#running-a-registry

Since before 1.0, Rust has had a public crate registry, [crates.io].
People publish crates with `cargo publish` and it's easy to include these crates
in the `[dependencies]` section of your `Cargo.toml`.

However, not everyone _wants_ to publish their crates to crates.io.
People maintaining proprietary/closed-source code cannot use crates.io,
and instead are forced to use `git` or `path` dependencies.
This is usually fine for small projects, but if you have a lot of closed-source crates
within a large organization, you lose the benefit of the versioning support that crates.io has.

With this release, Cargo gains support for alternate registries.
These registries coexist with crates.io, so you can write software that depends
on crates from both crates.io and your custom registry.
Crates on crates.io cannot however depend on external registries.

To use an alternate registry, you must add these lines to your `.cargo/config`.
This file can be in your home directory (`~/.cargo/config`) or relative to the package directory.

```toml
[registries]
my-registry = { index = "https://my-intranet:8080/git/index" }
```

Depending on a crate from an alternate registry is easy.
When specifying dependencies in your `Cargo.toml`, use the `registry` key to
let Cargo know that you wish to fetch the crate from the alternate registry:

```toml
[dependencies]
other-crate = { version = "1.0", registry = "my-registry" }
```

As a crate author, if you wish to publish your crate to an alternate registry,
you first need to save the authentication token into `~/.cargo/credentials` with the `cargo login` command:

```sh
cargo login --registry=my-registry
```

You can then use the `--registry` flag to indicate which registry to use when publishing:

```sh
cargo publish --registry=my-registry
```

There is [documentation][registry-docs] on how to run your own registry.

### `?` in documentation tests

[RFC 1937]: https://rust-lang.github.io/rfcs/1937-ques-in-main.html
[many releases ago]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#main-can-return-a-result

[RFC 1937] proposed adding support for using the `?` operator in `fn main()`,
`#[test]` functions, and doctests, allowing them to return `Option<T>` or `Result<T, E>`,
with error values causing a nonzero exit code in the case of `fn main()`,
and a test failure in the case of the tests.

Support in `fn main()` and `#[test]` was implemented [many releases ago].
However, the support within documentation tests was limited to doctests that have an explicit `fn main()`.

In this release, full support for `?` in doctests has been added.
Now, you can write this in your documentation tests:

````rust
/// ```rust
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
fn my_func() {}
````

You still have to specify the error type being used at the bottom of the documentation test.

### Custom attributes accept arbitrary token streams

[Procedural macros]: https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html
[arbitrary-tts]: https://github.com/rust-lang/rust/pull/57367

[Procedural macros] in Rust can define custom attributes that they consume.
Until now, such attributes were restricted to being trees of paths and literals
according to a specific syntax, like:

```rust
#[foo(bar)]
#[foo = "bar"]
#[foo = 0]
#[foo(bar = true)]
#[foo(bar, baz(quux, foo = "bar"))]
```

Unlike procedural macros, these helper attributes could not accept arbitrary token streams in delimiters,
so you could not write `#[range(0..10)]` or `#[bound(T: MyTrait)]`.
Procedural macro crates would instead use strings for specifying syntaxes like this, e.g. `#[range("0..10")]`

With this Rust release, custom attributes `#[attr($tokens)]` [now accept][arbitrary-tts]
arbitrary token streams in `$tokens`, bringing them on par with macros.
If you're the author of a procedural macro crate, please check if your custom attributes
have unnecessary strings in their syntax and if they can be better expressed with token streams.

### `TryFrom` and `TryInto`

[`from_be_bytes`]: https://doc.rust-lang.org/std/primitive.u32.html#method.from_be_bytes
[never_type]: https://github.com/rust-lang/rust/issues/35121
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[`TryInto`]: https://doc.rust-lang.org/std/convert/trait.TryInto.html
[`Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html

The [`TryFrom`] and [`TryInto`] traits were stabilized to allow fallible type conversions.

For example, the [`from_be_bytes`] and related methods on integer types take arrays,
but data is often read in via slices. Converting between slices and arrays is tedious to do manually.
With the new traits, it can be done inline with `.try_into()`.

```rust
let num = u32::from_be_bytes(slice.try_into()?);
```

For conversions that cannot fail, such as `u8` to `u32`, the [`Infallible`] type was added.
This also permits a blanket implementation of `TryFrom` for all existing `From` implementations.
In the future, we hope to turn `Infallible` into an alias for [the `!` (never) type][never_type].

### `fn before_exec` deprecated in favor of `unsafe fn pre_exec`

[`CommandExt::before_exec`]: https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.before_exec
[`CommandExt::pre_exec`]: https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.pre_exec
[ub-possible]: https://github.com/rust-lang/rust/issues/39575#issuecomment-437658766
[non-dup]: https://github.com/rust-lang/rust/issues/39575#issuecomment-439645949

On Unix-like systems, the function [`CommandExt::before_exec`] allows you to
schedule a closure to be run before `exec` is invoked.

The closure provided will be run in the context of the child process after a fork.
This means that resources, such as file descriptors and memory-mapped regions, may get duplicated.
In other words, you can now copy a value of a non-`Copy` type into a different process
while retaining the original in the parent. This makes [it possible][ub-possible] to cause
undefined behavior and break [libraries assuming non-duplication][non-dup].

The function `before_exec` should therefore have been marked as `unsafe`.
In this release of Rust, we have deprecated `fn before_exec` in favor of the `unsafe fn pre_exec`.
When calling [`CommandExt::pre_exec`], it is your responsibility to make sure that the closure
does not violate library invariants by making invalid use of these duplicates.
If you provide a library that is in a similar situation as `before_exec`,
consider deprecating and providing an `unsafe` alternative as well.

### Library stabilizations

[`AtomicU8`]: https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU8.html
[`NonZeroU8`]: https://doc.rust-lang.org/std/num/struct.NonZeroU8.html
[`NonZeroI8`]: https://doc.rust-lang.org/std/num/struct.NonZeroI8.html
[`iter::from_fn`]: https://doc.rust-lang.org/std/iter/fn.from_fn.html
[`iter::successors`]: https://doc.rust-lang.org/std/iter/fn.successors.html
[prev-1.28]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1280-2018-08-02

In 1.34.0, the set of stable atomic integer types was expanded,
with signed and unsigned variants from 8 ([`AtomicU8`]) to 64 bits now available.

[Previously][prev-1.28], non-zero unsigned integer types, e.g. [`NonZeroU8`], were stabilized.
This gave `Option<NonZeroU8>` the same size as `u8`.
With this Rust release, signed versions, e.g. [`NonZeroI8`], have been stabilized.

The functions [`iter::from_fn`] and [`iter::successors`] have been stabilized.
The former allows you to construct an iterator from `FnMut() -> Option<T>`.
To pop elements from a vector iteratively, you can now write `from_fn(|| vec.pop())`.
Meanwhile, the latter creates a new iterator where each successive item
is computed based on the preceding one.

Additionally, these APIs have become stable:

- [Any::type_id](https://doc.rust-lang.org/std/any/trait.Any.html#tymethod.type_id)
- [Error::type_id](https://doc.rust-lang.org/std/error/trait.Error.html#method.type_id)
- [slice::sort_by_cached_key](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_cached_key)
- [str::escape_debug](https://doc.rust-lang.org/std/primitive.str.html#method.escape_debug)
- [str::escape_default](https://doc.rust-lang.org/std/primitive.str.html#method.escape_default)
- [str::escape_unicode](https://doc.rust-lang.org/std/primitive.str.html#method.escape_unicode)
- [str::split_ascii_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_ascii_whitespace)
- [Instant::checked_add](https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_add)
- [Instant::checked_sub](https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_sub)
- [SystemTime::checked_add](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_add)
- [SystemTime::checked_sub](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_sub)

See the [detailed release notes][notes] for more details.
