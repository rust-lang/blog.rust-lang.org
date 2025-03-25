+++
layout = "post"
date = 2025-04-03
title = "Announcing Rust 1.86.0"
author = "The Rust Release Team"
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.86.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.86.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.86.0](https://doc.rust-lang.org/stable/releases.html#version-1860-2025-04-03).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.86.0 stable

### Trait upcasting

This release includes a long awaited feature — the ability to upcast trait objects.
If a trait has a [supertrait](https://doc.rust-lang.org/reference/items/traits.html#supertraits) you can coerce a reference to said trait object to a reference to a trait object of the super trait:

```rust
trait Trait: Supertrait {}
trait Supertrait {}

fn upcast(x: &dyn Trait) -> &dyn Supertrait {
    x
}
```

The same would work with any other kind of (smart)-pointer, like `Arc<dyn Trait> -> Arc<dyn Supertrait>` or `*const dyn Trait -> *const dyn Supertrait`.

Previously this would require a workaround in the form of an `upcast` method in the `Trait` itself (and that would work only for one kind of reference/pointer). Such workarounds are not necessary anymore. 

Note that this adds a new _safety invariant_  to raw pointers — if a pointer points to a trait object, vtable must be valid for that trait. This means that "leaking" a raw pointer to a trait object with an invalid vtable into safe code may lead to undefined behavior.

Trait upcasting may be especially useful with the `Any` trait, as it allows upcasting your trait object to `dyn Any` to call the downcast methods. Whereas before you'd have to write workarounds for this or use external crates.

```rust
use std::any::Any;

trait MyAny: Any {}

impl dyn MyAny {
    fn downcast_ref<T>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}
```

You can [learn more about trait upcasting in the Rust reference](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions).

### `HashMap`s and slices now support indexing multiple elements mutably

The borrow checker will prevent simaltaneous usage of references returned from `get_mut` methods. To get around this the standard library now provides a `get_disjoint_mut` helper on slices and `HashMap` to retrieve mutable references to multiple elements simaltaneously. See the following example taken from the API docs of [`slice::get_disjoint_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get_disjoint_mut):
```rust
let v = &mut [1, 2, 3];
if let Ok([a, b]) = v.get_disjoint_mut([0, 2]) {
    *a = 413;
    *b = 612;
}
assert_eq!(v, &[413, 2, 612]);

if let Ok([a, b]) = v.get_disjoint_mut([0..1, 1..3]) {
    a[0] = 8;
    b[0] = 88;
    b[1] = 888;
}
assert_eq!(v, &[8, 88, 888]);

if let Ok([a, b]) = v.get_disjoint_mut([1..=2, 0..=0]) {
    a[0] = 11;
    a[1] = 111;
    b[0] = 1;
}
assert_eq!(v, &[1, 11, 111]);
```

### Allow safe functions to be marked with the #[target_feature] attribute.

Functions marked with `#[target_feature]` are generally considered as unsafe functions: they are unsafe to call, cannot *generally* be assigned to safe function pointers, and don't implement the `Fn*` traits.

However, calling them from other `#[target_feature]` functions with a superset of features is safe.

```rust
// Demonstration function
#[target_feature(enable = "avx2")]
fn avx2() {}

fn foo() {
    // Calling `avx2` here is unsafe, as we must ensure
    // that AVX is available first.
    unsafe {
        avx2();
    }
}

#[target_feature(enable = "avx2")]
fn bar() {
    // Calling `avx2` here is safe.
    avx2();
}
```

### Debug assert pointers are non-null when required for soundness

The compiler will now insert checks that a pointer is not null upon non-zero-sized reads and writes, and also when the pointer is reborrowed into a reference. For example, the following code will now produce a non-unwinding panic:
```rust
let _x = *std::ptr::null::<u8>();
let _x = &*std::ptr::null::<u8>();
```
Trivial examples like this have produced a warning since Rust 1.53.0, the new runtime check will detect these scenarios regardless of complexity.

### Make `missing_abi` lint warn by default

Omitting the ABI in extern blocks and functions (e.g. `extern {}` and `extern fn`) will now result in a warning (via the `missing_abi` lint). Omitting the ABI after the `extern` keyword has always implicitly resulted in the `"C"` ABI. It is now recommended to explicitly specify the `"C"` ABI (e.g. `extern "C" {}` and `extern "C" fn`).

### Target deprecation warning for 1.87.0

The target `i586-pc-windows-msvc` will be removed in the next version of Rust, 1.87.0. This is because this target is effectively useless. Its difference to the much more popular `i686-pc-windows-msvc` is that it does not require SSE2 instruction support, but Windows 10, the minimum required OS version of all `windows` targets (except the `win7` targets), requires SSE2 instructions itself, so this target does not fulfill a purpose.

You can check the [Major Change Proposal](https://github.com/rust-lang/compiler-team/issues/840) for more information.

### Stabilized APIs

- [`{float}::next_down`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.next_down)
- [`{float}::next_up`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.next_up)
- [`HashMap::get_disjoint_mut`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.get_disjoint_mut)
- [`HashMap::get_disjoint_unchecked_mut`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#method.get_disjoint_unchecked_mut)
- [`NonZero::count_ones`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.count_ones)
- [`Vec::pop_if`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop_if)
- [`sync::Once::wait`](https://doc.rust-lang.org/stable/std/sync/struct.Once.html#method.wait)
- [`sync::Once::wait_force`](https://doc.rust-lang.org/stable/std/sync/struct.Once.html#method.wait_force)
- [`sync::OnceLock::wait`](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html#method.wait)
- [`<[_]>::get_disjoint_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get_disjoint_mut)
- [`<[_]>::get_disjoint_unchecked_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.get_disjoint_unchecked_mut)
- [`slice::GetDisjointMutError`](https://doc.rust-lang.org/stable/std/slice/enum.GetDisjointMutError.html)


These APIs are now stable in const contexts:

- [`io::Cursor::get_mut`](https://doc.rust-lang.org/stable/std/io/struct.Cursor.html#method.get_mut)
- [`io::Cursor::set_position`](https://doc.rust-lang.org/stable/std/io/struct.Cursor.html#method.set_position)
- [`str::is_char_boundary`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.is_char_boundary)
- [`str::split_at`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at)
- [`str::split_at_checked`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at_checked)
- [`str::split_at_mut`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at_mut)
- [`str::split_at_mut_checked`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at_mut_checked)
- [`hint::black_box`](https://doc.rust-lang.org/stable/std/hint/fn.black_box.html)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.86.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-186-2025-04-03), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-186).

## Contributors to 1.86.0

Many people came together to create Rust 1.86.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.86.0/)