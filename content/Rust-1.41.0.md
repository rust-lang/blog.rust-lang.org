+++
path = "2020/01/30/Rust-1.41.0"
title = "Announcing Rust 1.41.0"
authors = ["The Rust Release Team"]
aliases = [
    "2020/01/30/Rust-1.41.0.html",
    "releases/1.41.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.41.0. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.41.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.41.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1410-2020-01-30

## What's in 1.41.0 stable

The highlights of Rust 1.41.0 include relaxed restrictions for trait
implementations, improvements to `cargo install`, a more `git`-friendly
`Cargo.lock`, and new FFI-related guarantees for `Box<T>`. See the [detailed
release notes][notes] to learn about other changes not covered by this post.

### Relaxed restrictions when implementing traits

[book_orphan]: https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type
[ref_orphan]: https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence
[book_newtype]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[report_orphan]: https://github.com/rust-lang/rust/issues/63599
[rfc_orphan]: https://rust-lang.github.io/rfcs/2451-re-rebalancing-coherence.html

To prevent breakages in the ecosystem when a dependency adds a new trait
`impl`, Rust enforces the [*orphan rule*][book_orphan]. The gist of it is that
a trait `impl` is only allowed if either the trait or the type being
implemented is *local* to (defined in) the current crate as opposed to a
*foreign* crate. [What this means exactly][ref_orphan] is complicated, however,
when generics are involved.

Before Rust 1.41.0, the orphan rule was unnecessarily strict, getting in the
way of composition. As an example, suppose your crate defines the
`BetterVec<T>` struct, and you want a way to convert your struct to the
standard library's `Vec<T>`. The code you would write is:

```rust
impl<T> From<BetterVec<T>> for Vec<T> {
    // ...
}
```

...which is an instance of the pattern:

```rust
impl<T> ForeignTrait<LocalType> for ForeignType<T> {
    // ...
}
```

In Rust 1.40.0 this `impl` was forbidden by the orphan rule, as both `From` and
`Vec` are defined in the standard library, which is foreign to the current
crate. There were ways to work around the limitation, such as [the *newtype*
pattern][book_newtype], but they were often cumbersome or even impossible in
some cases.

While it's still true that both `From` and `Vec` were foreign, the trait (in
this case `From`) was parameterized by a local type. Therefore, Rust 1.41.0
allows this `impl`.

For more details, read the [the stabilization report][report_orphan] and [the
RFC proposing the change][rfc_orphan].

### `cargo install` updates packages when outdated

With `cargo install`, you can install binary crates in your system. The command
is often used by the community to install popular CLI tools written in Rust.

Starting from Rust 1.41.0, `cargo install` will also update existing
installations of the crate if a new release came out since you installed it.
Before this release the only option was to pass the `--force` flag, which
reinstalls the binary crate even if it's up to date.

### Less conflict-prone `Cargo.lock` format

To ensure consistent builds, Cargo uses a file named `Cargo.lock`, containing
dependency versions and checksums. Unfortunately, the way the data was arranged
in it caused unnecessary merge conflicts when changing dependencies in separate
branches.

Rust 1.41.0 introduces a new format for the file, explicitly designed to avoid
those conflicts. This new format will be used for all new lockfiles, while
existing lockfiles will still rely on the previous format. You can learn about
the choices leading to the new format [in the PR adding it][cargo/7070].

[cargo/7070]: https://github.com/rust-lang/cargo/pull/7070

### More guarantees when using `Box<T>` in FFI

[box_docs]: https://doc.rust-lang.org/std/boxed/index.html

Starting with Rust 1.41.0, we have declared that a `Box<T>`, where `T: Sized`
is now ABI compatible with the C language's pointer (`T*`) types. So if you
have an `extern "C"` Rust function, called from C, your Rust function can now
use `Box<T>`, for some specific `T`, while using `T*` in C for the
corresponding function. As an example, on the C side you may have:

```c
// C header

// Returns ownership to the caller.
struct Foo* foo_new(void);

// Takes ownership from the caller; no-op when invoked with NULL.
void foo_delete(struct Foo*);
```

...while on the Rust side, you would have:

```rust
#[repr(C)]
pub struct Foo;

#[no_mangle]
pub extern "C" fn foo_new() -> Box<Foo> {
    Box::new(Foo)
}

// The possibility of NULL is represented with the `Option<_>`.
#[no_mangle]
pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
```

Note however that while `Box<T>` and `T*` have the same representation and ABI,
a `Box<T>` must still be non-null, aligned, and ready for deallocation by the
global allocator. To ensure this, it is best to only use `Box`es originating
from the global allocator.

**Important:** At least at present, you should avoid using `Box<T>` types for
functions that are defined in C but invoked from Rust. In those cases, you
should directly mirror the C types as closely as possible. Using types like
`Box<T>` where the C definition is just using `T*` can lead to undefined
behavior.

To read more, [consult the documentation for `Box<T>`][box_docs].

### Library changes

[`Result::map_or`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or
[`Result::map_or_else`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or_else
[`Option::map_or`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or
[`Option::map_or_else`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or_else
[`std::rc::Weak::weak_count`]: https://doc.rust-lang.org/std/rc/struct.Weak.html#method.weak_count
[`std::rc::Weak::strong_count`]: https://doc.rust-lang.org/std/rc/struct.Weak.html#method.strong_count
[`std::sync::Weak::weak_count`]: https://doc.rust-lang.org/std/sync/struct.Weak.html#method.weak_count
[`std::sync::Weak::strong_count`]: https://doc.rust-lang.org/std/sync/struct.Weak.html#method.strong_count
[pr_66277]: https://github.com/rust-lang/rust/pull/66277
[pr_65013]: https://github.com/rust-lang/rust/pull/65013

In Rust 1.41.0, we've made the following additions to the standard library:

- The [`Result::map_or`] and [`Result::map_or_else`] methods were stabilized.

  Similar to [`Option::map_or`] and [`Option::map_or_else`], these methods are
  shorthands for the `.map(|val| process(val)).unwrap_or(default)` pattern.

- [`NonZero*` numerics now implement `From<NonZero*>` if it's a smaller integer
  width.][pr_66277] For example, `NonZeroU16` now implements `From<NonZeroU8>`.

- The `weak_count` and `strong_count` methods on `Weak` pointers were stabilized.

    - [`std::rc::Weak::weak_count`]
    - [`std::rc::Weak::strong_count`]
    - [`std::sync::Weak::weak_count`]
    - [`std::sync::Weak::strong_count`]

  These methods return the number of weak (`rc::Weak<T>` and `sync::Weak<T>`)
  or strong (`Rc<T>` and `Arc<T>`) pointers to the allocation respectively.

- [`MaybeUninit<T>` now implements `fmt::Debug`.][pr_65013]

### Reducing support for 32-bit Apple targets soon

Rust 1.41.0 is the last release with the current level of compiler support for
32-bit Apple targets, including the `i686-apple-darwin` target. Starting from
Rust 1.42.0, these targets will be demoted to the lowest support tier.

[You can learn more about this change in this blog post.][32bit-demotion]

[32bit-demotion]: https://blog.rust-lang.org/2020/01/03/reducing-support-for-32-bit-apple-targets.html

### Other changes

[relnotes-cargo]: https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-141-2020-01-30
[relnotes-clippy]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-141
[mir-opt]: https://blog.rust-lang.org/inside-rust/2019/12/02/const-prop-on-by-default.html

There are other changes in the Rust 1.41.0 release: check out what changed in
[Rust][notes], [Cargo][relnotes-cargo], and [Clippy][relnotes-clippy]. We also
have started landing MIR optimizations, which should improve compile time: you
can learn more about them in the ["Inside Rust" blog post][mir-opt].

## Contributors to 1.41.0

Many people came together to create Rust 1.41.0. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.41.0/)
