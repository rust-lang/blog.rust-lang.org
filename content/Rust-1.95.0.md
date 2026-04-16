+++
path = "2026/04/16/Rust-1.95.0"
title = "Announcing Rust 1.95.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.95.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.95.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.95.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.95.0](https://doc.rust-lang.org/stable/releases.html#version-1950-2026-04-16).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.95.0 stable

### `cfg_select!`

Rust 1.95 introduces a
[`cfg_select!`](https://doc.rust-lang.org/stable/std/macro.cfg_select.html)
macro that acts roughly similar to a compile-time `match` on `cfg`s. This
fulfills the same purpose as the popular
[`cfg-if`](https://crates.io/crates/cfg-if) crate, although with a different
syntax. `cfg_select!` expands to the right-hand side of the first arm whose
configuration predicate evaluates to `true`. Some examples:

```rust
cfg_select! {
    unix => {
        fn foo() { /* unix specific functionality */ }
    }
    target_pointer_width = "32" => {
        fn foo() { /* non-unix, 32-bit functionality */ }
    }
    _ => {
        fn foo() { /* fallback implementation */ }
    }
}

let is_windows_str = cfg_select! {
    windows => "windows",
    _ => "not windows",
};
```

### if-let guards in matches

Rust 1.88 stabilized [let chains](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/#let-chains). Rust
1.95 brings that capability into match expressions, allowing for conditionals
based on pattern matching.

```rust
match value {
    Some(x) if let Ok(y) = compute(x) => {
        // Both `x` and `y` are available here
        println!("{}, {}", x, y);
    }
    _ => {}
}
```

Note that the compiler will not currently consider the patterns matched in `if
let` guards as part of the exhaustiveness evaluation of the overall match, just
like `if` guards.

### Stabilized APIs

- [`MaybeUninit<[T; N]>: From<[MaybeUninit<T>; N]>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-From%3CMaybeUninit%3C%5BT;+N%5D%3E%3E-for-%5BMaybeUninit%3CT%3E;+N%5D)
- [`MaybeUninit<[T; N]>: AsRef<[MaybeUninit<T>; N]>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-AsRef%3C%5BMaybeUninit%3CT%3E;+N%5D%3E-for-MaybeUninit%3C%5BT;+N%5D%3E)
- [`MaybeUninit<[T; N]>: AsRef<[MaybeUninit<T>]>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-AsRef%3C%5BMaybeUninit%3CT%3E%5D%3E-for-MaybeUninit%3C%5BT;+N%5D%3E)
- [`MaybeUninit<[T; N]>: AsMut<[MaybeUninit<T>; N]>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-AsMut%3C%5BMaybeUninit%3CT%3E;+N%5D%3E-for-MaybeUninit%3C%5BT;+N%5D%3E)
- [`MaybeUninit<[T; N]>: AsMut<[MaybeUninit<T>]>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-AsMut%3C%5BMaybeUninit%3CT%3E%5D%3E-for-MaybeUninit%3C%5BT;+N%5D%3E)
- [`[MaybeUninit<T>; N]: From<MaybeUninit<[T; N]>>`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#impl-From%3C%5BMaybeUninit%3CT%3E;+N%5D%3E-for-MaybeUninit%3C%5BT;+N%5D%3E)
- [`Cell<[T; N]>: AsRef<[Cell<T>; N]>`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#impl-AsRef%3C%5BCell%3CT%3E;+N%5D%3E-for-Cell%3C%5BT;+N%5D%3E)
- [`Cell<[T; N]>: AsRef<[Cell<T>]>`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#impl-AsRef%3C%5BCell%3CT%3E%5D%3E-for-Cell%3C%5BT;+N%5D%3E)
- [`Cell<[T]>: AsRef<[Cell<T>]>`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#impl-AsRef%3C%5BCell%3CT%3E%5D%3E-for-Cell%3C%5BT%5D%3E)
- [`bool: TryFrom<{integer}>`](https://doc.rust-lang.org/stable/std/primitive.bool.html#impl-TryFrom%3Cu128%3E-for-bool)
- [`AtomicPtr::update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.update)
- [`AtomicPtr::try_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicPtr.html#method.try_update)
- [`AtomicBool::update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicBool.html#method.update)
- [`AtomicBool::try_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicBool.html#method.try_update)
- [`AtomicIn::update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicIsize.html#method.update)
- [`AtomicIn::try_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicIsize.html#method.try_update)
- [`AtomicUn::update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicUsize.html#method.update)
- [`AtomicUn::try_update`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicUsize.html#method.try_update)
- [`cfg_select!`](https://doc.rust-lang.org/stable/std/macro.cfg_select.html)
- [`mod core::range`](https://doc.rust-lang.org/stable/core/range/index.html)
- [`core::range::RangeInclusive`](https://doc.rust-lang.org/stable/core/range/struct.RangeInclusive.html)
- [`core::range::RangeInclusiveIter`](https://doc.rust-lang.org/stable/core/range/struct.RangeInclusiveIter.html)
- [`core::hint::cold_path`](https://doc.rust-lang.org/stable/core/hint/fn.cold_path.html)
- [`<*const T>::as_ref_unchecked`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_ref_unchecked)
- [`<*mut T>::as_ref_unchecked`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_ref_unchecked-1)
- [`<*mut T>::as_mut_unchecked`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.as_mut_unchecked)
- [`Vec::push_mut`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.push_mut)
- [`Vec::insert_mut`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.insert_mut)
- [`VecDeque::push_front_mut`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.push_front_mut)
- [`VecDeque::push_back_mut`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.push_back_mut)
- [`VecDeque::insert_mut`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.insert_mut)
- [`LinkedList::push_front_mut`](https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html#method.push_front_mut)
- [`LinkedList::push_back_mut`](https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html#method.push_back_mut)
- [`Layout::dangling_ptr`](https://doc.rust-lang.org/stable/std/alloc/struct.Layout.html#method.dangling_ptr)
- [`Layout::repeat`](https://doc.rust-lang.org/stable/std/alloc/struct.Layout.html#method.repeat)
- [`Layout::repeat_packed`](https://doc.rust-lang.org/stable/std/alloc/struct.Layout.html#method.repeat_packed)
- [`Layout::extend_packed`](https://doc.rust-lang.org/stable/std/alloc/struct.Layout.html#method.extend_packed)

These previously stable APIs are now stable in const contexts:

- [`fmt::from_fn`](https://doc.rust-lang.org/stable/std/fmt/fn.from_fn.html)
- [`ControlFlow::is_break`](https://doc.rust-lang.org/stable/core/ops/enum.ControlFlow.html#method.is_break)
- [`ControlFlow::is_continue`](https://doc.rust-lang.org/stable/core/ops/enum.ControlFlow.html#method.is_continue)

### Destabilized JSON target specs

Rust 1.95 removes support on stable for passing a custom target specification
to `rustc`. This should **not** affect any Rust users using a fully stable
toolchain, as building the standard library (including just `core`) already
required using nightly-only features.

We're also gathering use cases for custom targets on the [tracking issue](https://github.com/rust-lang/rust/issues/151528)
as we consider whether some form of this feature should eventually be stabilized.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.95.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-195-2026-04-16), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-195).

## Contributors to 1.95.0

Many people came together to create Rust 1.95.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.95.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
