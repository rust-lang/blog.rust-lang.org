+++
path = "2018/08/02/Rust-1.28"
title = "Announcing Rust 1.28"
authors = ["The Rust Core Team"]
aliases = [
    "2018/08/02/Rust-1.28.html",
    "releases/1.28.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.28.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.28.0 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.28.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1280-2018-08-02

## What's in 1.28.0 stable

### Global Allocators

Allocators are the way that programs in Rust obtain memory from the system at
runtime. Previously, Rust did not allow changing the way memory is obtained,
which prevented some use cases. On some platforms, this meant using jemalloc, on
others, the system allocator, but there was no way for users to control this key
component. With 1.28.0, the `#[global_allocator]` attribute is now stable, which
allows Rust programs to set their allocator to the system allocator, as well as
define new allocators by implementing the [`GlobalAlloc`] trait.

The default allocator for Rust programs on some platforms is jemalloc. The
standard library now provides a handle to the system allocator, which can be
used to switch to the system allocator when desired, by declaring a static and
marking it with the `#[global_allocator]` attribute.

```rust
use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let mut v = Vec::new();
    // This will allocate memory using the system allocator.
    v.push(1);
}
```

However, sometimes you want to define a custom allocator for a given application
domain. This is also relatively easy to do by implementing the `GlobalAlloc`
trait. You can read more about how to do this in the [documentation].

[`GlobalAlloc`]: https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html
[documentation]: https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html

### Improved error message for formatting

Work on diagnostics continues, this time with an emphasis on formatting:

```rust
format!("{_foo}", _foo = 6usize);
```

Previously, the error message emitted here was relatively poor:

```
error: invalid format string: expected `'}'`, found `'_'`
  |
2 |     format!("{_foo}", _foo = 6usize);
  |             ^^^^^^^^
```

Now, we emit a diagnostic that tells you the specific reason the format string
is invalid:

```
error: invalid format string: invalid argument name `_foo`
  |
2 |     let _ = format!("{_foo}", _foo = 6usize);
  |                       ^^^^ invalid argument name in format string
  |
  = note: argument names cannot start with an underscore
```

See the [detailed release notes][notes] for more.

### Library stabilizations

We've already mentioned the stabilization of the `GlobalAlloc` trait, but
another important stabilization is the [`NonZero`] number types. These are wrappers
around the standard unsigned integer types: `NonZeroU8`, `NonZeroU16`,
`NonZeroU32`, `NonZeroU64`, `NonZeroU128`, and `NonZeroUsize`.

This allows for size optimization, for example, `Option<u8>` is two bytes large,
but `Option<NonZeroU8>` is just one byte large. Note that this optimization
remains even when `NonZeroU8` is wrapped inside another struct; the example
below illustrates that `Door` is still 1 byte large despite being placed inside
an `Option`. This optimization applies to user-defined enums as well: `Option`
is not special.

```rust
use std::mem;
use std::num::NonZeroU8;

struct Key(NonZeroU8);

struct Door {
    key: Key,
}

fn main() {
    assert_eq!(mem::size_of::<Door>(), 1);
    assert_eq!(mem::size_of::<Option<Door>>(), 1);
}
```

A number of other libraries have also been stabilized: you can see the more
[detailed release notes][notes] for full details.

[`NonZero`]: https://doc.rust-lang.org/stable/std/num/index.html

### Cargo features

[Cargo will now no longer allow you to publish crates with build scripts that
modify the `src` directory.][cargo/5584] The `src` directory in a crate should be
considered to be immutable.

[cargo/5584]: https://github.com/rust-lang/cargo/pull/5584/

## Contributors to 1.28.0

Many people came together to create Rust 1.28. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.28.0)
