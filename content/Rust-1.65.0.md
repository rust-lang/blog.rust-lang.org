+++
path = "2022/11/03/Rust-1.65.0"
title = "Announcing Rust 1.65.0"
authors = ["The Rust Release Team"]
aliases = [
    "2022/11/03/Rust-1.65.0.html",
    "releases/1.65.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.65.0. Rust is a
programming language empowering everyone to build reliable and efficient
software.

---

Before going into the details of the new Rust release, we'd like to draw
attention to the tragic [death of Mahsa
Amini](https://en.wikipedia.org/wiki/Death_of_Mahsa_Amini) and the death and
violent suppression of many others, by the religious morality police of Iran.
See <https://en.wikipedia.org/wiki/Mahsa_Amini_protests> for more details. We
stand in solidarity with the people in Iran struggling for human rights.

---

If you have a previous version of Rust installed via rustup, you can get 1.65.0
with:

```
$ rustup update stable
```

If you don't have it already, you can [get
`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on
our website, and check out the [detailed release notes for
1.65.0](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1650-2022-11-03)
on GitHub.

If you'd like to help us out by testing future releases, you might consider
updating locally to use the beta channel (`rustup default beta`) or the nightly
channel (`rustup default nightly`). Please
[report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you
might come across!

## What's in 1.65.0 stable

### Generic associated types (GATs)

Lifetime, type, and const generics can now be defined on associated types, like so:

```rust
trait Foo {
    type Bar<'x>;
}
```

It's hard to put into few words just how useful these can be, so here are a
few example traits, to get a sense of their power:

```rust
/// An `Iterator`-like trait that can borrow from `Self`
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

/// Can be implemented over smart pointers, like `Rc` or `Arc`,
/// in order to allow being generic over the pointer type
trait PointerFamily {
    type Pointer<T>: Deref<Target = T>;

    fn new<T>(value: T) -> Self::Pointer<T>;
}

/// Allows borrowing an array of items. Useful for
/// `NdArray`-like types that don't necessarily store
/// data contiguously.
trait BorrowArray<T> {
    type Array<'x, const N: usize> where Self: 'x;

    fn borrow_array<'a, const N: usize>(&'a self) -> Self::Array<'a, N>;
}
```

As you can see, GATs are quite versatile and enable a number
of patterns that are not currently able to be written. For more
information, check out the post announcing the
[push for stabilization](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html)
published last year or the
[stabilization announcement post](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html)
published last week. The former goes into a bit more depth
of a couple of the examples above, while the latter talks
about some of the known limitations of this stabilization.

More in depth reading can be found in the associated types
section of the [nightly reference](https://doc.rust-lang.org/nightly/reference/items/associated-items.html#associated-types)
or the [original RFC](https://rust-lang.github.io/rfcs/1598-generic_associated_types.html) (which was initially opened over 6.5 years ago!).

### `let`-`else` statements

This introduces a new type of `let` statement with a refutable pattern and a
diverging `else` block that executes when that pattern doesn't match.

    let PATTERN: TYPE = EXPRESSION else {
        DIVERGING_CODE;
    };

Normal `let` statements can only use _irrefutable_ patterns, statically known
to always match. That pattern is often just a single variable binding, but may
also unpack compound types like structs, tuples, and arrays. However, that was
not usable for conditional matches, like pulling out a variant of an enum --
until now! With `let`-`else`, a refutable pattern can match and bind variables
in the surrounding scope like a normal `let`, or else diverge (e.g. `break`,
`return`, `panic!`) when the pattern doesn't match.

```rust
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}
assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
```

The scope of name bindings is the main thing that makes this different from
`match` or `if let`-`else` expressions. You could previously approximate these
patterns with an unfortunate bit of repetition and an outer `let`:

```rust
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };
```

### `break` from labeled blocks

Plain block expressions can now be labeled as a `break` target, terminating
that block early. This may sound a little like a `goto` statement, but it's not
an arbitrary jump, only from within a block to its end. This was already
possible with `loop` blocks, and you may have seen people write loops that
always execute only once, just to get a labeled `break`.

Now there's a language feature specifically for that! Labeled `break` may also
include an expression value, just as with loops, letting a multi-statement
block have an early "return" value.

```rust
let result = 'block: {
    do_thing();
    if condition_not_met() {
        break 'block 1;
    }
    do_next_thing();
    if condition_not_met() {
        break 'block 2;
    }
    do_last_thing();
    3
};
```

### Splitting Linux debuginfo

Back in Rust 1.51, the compiler team added support for [split debug
information](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#splitting-debug-information)
on macOS, and now this option is stable for use on Linux as well.

- `-Csplit-debuginfo=unpacked` will split debuginfo out into multiple `.dwo`
  DWARF object files.
- `-Csplit-debuginfo=packed` will produce a single `.dwp` DWARF package
  alongside your output binary with all the debuginfo packaged together.
- `-Csplit-debuginfo=off` is still the default behavior, which includes DWARF
  data in `.debug_*` ELF sections of the objects and final binary.

Split DWARF lets the linker avoid processing the debuginfo (because it isn't in
the object files being linked anymore), which can speed up link times!

Other targets now also accept `-Csplit-debuginfo` as a stable option with their
platform-specific default value, but specifying other values is still unstable.

### Stabilized APIs

The following methods and trait implementations are now stabilized:

- [`std::backtrace::Backtrace`](https://doc.rust-lang.org/stable/std/backtrace/struct.Backtrace.html)
- [`Bound::as_ref`](https://doc.rust-lang.org/stable/std/ops/enum.Bound.html#method.as_ref)
- [`std::io::read_to_string`](https://doc.rust-lang.org/stable/std/io/fn.read_to_string.html)
- [`<*const T>::cast_mut`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.cast_mut)
- [`<*mut T>::cast_const`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.cast_const)

Of particular note, the `Backtrace` API allows capturing a stack backtrace at
any time, using the same platform-specific implementation that usually serves
panic backtraces. This may be useful for adding runtime context to error types,
for example.

These APIs are now usable in const contexts:

- [`<*const T>::offset_from`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from)
- [`<*mut T>::offset_from`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from)

### Compatibility notes

- As the final step of the [RLS
  deprecation](https://blog.rust-lang.org/2022/07/01/RLS-deprecation.html),
  this release has replaced RLS with a small LSP server showing a deprecation
  warning, advising users to migrate to `rust-analyzer`.

### Other changes

There are other changes in the Rust 1.65 release, including:

- MIR inlining is now enabled for optimized compilations. This provides a 3-10%
  improvement in compiletimes for real world crates.
- When scheduling builds, Cargo now sorts the queue of pending jobs to improve performance.

Check out everything that changed in
[Rust](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1650-2022-11-03),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-165-2022-11-03),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-165).

### Contributors to 1.65.0

Many people came together to create Rust 1.65.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.65.0/)
