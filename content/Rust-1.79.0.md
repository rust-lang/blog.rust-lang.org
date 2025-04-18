+++
path = "2024/06/13/Rust-1.79.0"
title = "Announcing Rust 1.79.0"
authors = ["The Rust Release Team"]
aliases = [
    "2024/06/13/Rust-1.79.0.html",
    "releases/1.79.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.79.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.79.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.79.0](https://doc.rust-lang.org/nightly/releases.html#version-1790-2024-06-13).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.79.0 stable

### Inline `const` expressions

`const { ... }` blocks are now stable in expression position, permitting
explicitly entering a const context without requiring extra declarations (e.g.,
defining `const` items or associated constants on a trait).

Unlike const items (`const ITEM: ... = ...`), inline consts are able to make
use of in-scope generics, and have their type inferred rather than written explicitly, making them particularly useful for inline code snippets. For example, a pattern like:

```rust
const EMPTY: Option<Vec<u8>> = None;
let foo = [EMPTY; 100];
```

can now be written like this:

```rust
let foo = [const { None }; 100];
```

Notably, this is also true of generic contexts, where previously a verbose trait declaration with an associated constant would be required:

```rust
fn create_none_array<T, const N: usize>() -> [Option<T>; N] {
    [const { None::<T> }; N]
}
```

This makes this code much more succinct and easier to read.

See the [reference documentation](https://doc.rust-lang.org/nightly/reference/expressions/block-expr.html#const-blocks) for details.

### Bounds in associated type position

Rust 1.79 stabilizes the associated item bounds syntax, which allows us to put
bounds in associated type position within other bounds, i.e.
`T: Trait<Assoc: Bounds...>`. This avoids the need to provide an extra,
explicit generic type just to constrain the associated type.

This feature allows specifying bounds in a few places that previously either
were not possible or imposed extra, unnecessary constraints on usage:

* **`where` clauses** - in this position, this is equivalent to breaking up the bound into two (or more) `where` clauses. For example, `where T: Trait<Assoc: Bound>` is equivalent to `where T: Trait, <T as Trait>::Assoc: Bound`.
* **Supertraits** - a bound specified via the new syntax is implied when the trait is used, unlike where clauses. Sample syntax: `trait CopyIterator: Iterator<Item: Copy> {}`.
* **Associated type item bounds** - This allows constraining the *nested* rigid projections that are associated with a trait's associated types. e.g. `trait Trait { type Assoc: Trait2<Assoc2: Copy>; }`.
* **opaque type bounds (RPIT, TAIT)** - This allows constraining associated types that are associated with the opaque type without having to *name* the opaque type. For example, `impl Iterator<Item: Copy>` defines an iterator whose item is `Copy` without having to actually name that item bound.

See [the stabilization report](https://github.com/rust-lang/rust/pull/122055/#issue-2170532454) for more details.

### Extending automatic temporary lifetime extension

Temporaries which are immediately referenced in construction are now
automatically lifetime extended in `match` and `if` constructs. This has the
same behavior as lifetime extension for temporaries in block constructs.

For example:

```rust
let a = if true {
    ..;
    &temp() // used to error, but now gets lifetime extended
} else {
    ..;
    &temp() // used to error, but now gets lifetime extended
};
```

and

```rust
let a = match () {
    _ => {
        ..;
        &temp() // used to error, but now gets lifetime extended
    }
};
```

are now consistent with prior behavior:

```rust
let a = {
    ..;
    &temp() // lifetime is extended
};
```

This behavior is backwards compatible since these programs used to fail compilation.

### Frame pointers enabled in standard library builds

The standard library distributed by the Rust project is now compiled with
`-Cforce-frame-pointers=yes`, enabling downstream users to more easily profile
their programs. Note that the standard library also continues to come up with
line-level debug info (e.g., DWARF), though that is [stripped by default] in Cargo's release profiles.

[stripped by default]: https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html#enable-strip-in-release-profiles-by-default

### Stabilized APIs

- [`{integer}::unchecked_add`](https://doc.rust-lang.org/stable/core/primitive.i32.html#method.unchecked_add)
- [`{integer}::unchecked_mul`](https://doc.rust-lang.org/stable/core/primitive.i32.html#method.unchecked_mul)
- [`{integer}::unchecked_sub`](https://doc.rust-lang.org/stable/core/primitive.i32.html#method.unchecked_sub)
- [`<[T]>::split_at_unchecked`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.split_at_unchecked)
- [`<[T]>::split_at_mut_unchecked`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.split_at_mut_unchecked)
- [`<[u8]>::utf8_chunks`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.utf8_chunks)
- [`str::Utf8Chunks`](https://doc.rust-lang.org/stable/core/str/struct.Utf8Chunks.html)
- [`str::Utf8Chunk`](https://doc.rust-lang.org/stable/core/str/struct.Utf8Chunk.html)
- [`<*const T>::is_aligned`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.is_aligned)
- [`<*mut T>::is_aligned`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.is_aligned-1)
- [`NonNull::is_aligned`](https://doc.rust-lang.org/stable/core/ptr/struct.NonNull.html#method.is_aligned)
- [`<*const [T]>::len`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.len)
- [`<*mut [T]>::len`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.len-1)
- [`<*const [T]>::is_empty`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.is_empty)
- [`<*mut [T]>::is_empty`](https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.is_empty-1)
- [`NonNull::<[T]>::is_empty`](https://doc.rust-lang.org/stable/core/ptr/struct.NonNull.html#method.is_empty)
- [`CStr::count_bytes`](https://doc.rust-lang.org/stable/core/ffi/c_str/struct.CStr.html#method.count_bytes)
- [`io::Error::downcast`](https://doc.rust-lang.org/stable/std/io/struct.Error.html#method.downcast)
- [`num::NonZero<T>`](https://doc.rust-lang.org/stable/core/num/struct.NonZero.html)
- [`path::absolute`](https://doc.rust-lang.org/stable/std/path/fn.absolute.html)
- [`proc_macro::Literal::byte_character`](https://doc.rust-lang.org/stable/proc_macro/struct.Literal.html#method.byte_character)
- [`proc_macro::Literal::c_string`](https://doc.rust-lang.org/stable/proc_macro/struct.Literal.html#method.c_string)

These APIs are now stable in const contexts:

- [`Atomic*::into_inner`](https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicUsize.html#method.into_inner)
- [`io::Cursor::new`](https://doc.rust-lang.org/stable/std/io/struct.Cursor.html#method.new)
- [`io::Cursor::get_ref`](https://doc.rust-lang.org/stable/std/io/struct.Cursor.html#method.get_ref)
- [`io::Cursor::position`](https://doc.rust-lang.org/stable/std/io/struct.Cursor.html#method.position)
- [`io::empty`](https://doc.rust-lang.org/stable/std/io/fn.empty.html)
- [`io::repeat`](https://doc.rust-lang.org/stable/std/io/fn.repeat.html)
- [`io::sink`](https://doc.rust-lang.org/stable/std/io/fn.sink.html)
- [`panic::Location::caller`](https://doc.rust-lang.org/stable/std/panic/struct.Location.html#method.caller)
- [`panic::Location::file`](https://doc.rust-lang.org/stable/std/panic/struct.Location.html#method.file)
- [`panic::Location::line`](https://doc.rust-lang.org/stable/std/panic/struct.Location.html#method.line)
- [`panic::Location::column`](https://doc.rust-lang.org/stable/std/panic/struct.Location.html#method.column)


### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.79.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-179-2024-06-13), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-179).

## Contributors to 1.79.0

Many people came together to create Rust 1.79.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.79.0/)
