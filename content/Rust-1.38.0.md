+++
path = "2019/09/26/Rust-1.38.0"
title = "Announcing Rust 1.38.0"
authors = ["The Rust Release Team"]
aliases = [
    "2019/09/26/Rust-1.38.0.html",
    "releases/1.38.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.38.0. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.38.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the appropriate page on our website.

[install]: https://www.rust-lang.org/install.html

## What's in 1.38.0 stable

The highlight of this release is pipelined compilation.

### Pipelined compilation

[internals-pipelined]: https://internals.rust-lang.org/t/evaluating-pipelined-rustc-compilation/10199

To compile a crate, the compiler doesn't need the dependencies to be fully built. Instead, it just needs their "metadata" (i.e. the list of types, dependencies, exports...). This metadata is produced early in the compilation process. Starting with Rust 1.38.0, Cargo will take advantage of this by automatically starting to build dependent crates as soon as metadata is ready.

While the change doesn't have any effect on builds for a single crate, during testing [we got reports][internals-pipelined] of 10-20% compilation speed increases for optimized, clean builds of some crate graphs. Other ones did not improve much, and the speedup depends on the hardware running the build, so your mileage might vary. No code changes are needed to benefit from this.

### Linting some incorrect uses of `mem::{uninitialized, zeroed}`

As [previously announced](https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html#maybeuninitt%3E-instead-of-mem::uninitialized), `std::mem::uninitialized` is essentially impossible to use safely. Instead, `MaybeUninit<T>` should be used.

We have not yet deprecated `mem::uninitialized`; this will be done in a future release. Starting in 1.38.0, however, `rustc` will provide a lint for a narrow class of incorrect initializations using `mem::uninitialized` or `mem::zeroed`.

It is undefined behavior for some types, such as `&T` and `Box<T>`, to ever contain an all-`0` bit pattern, because they represent pointer-like objects that cannot be `null`. It is therefore an error to use `mem::uninitialized` or `mem::zeroed` to initialize one of these types, so the new lint will attempt to warn whenever one of those functions is used to initialize one of them, either directly or as a member of a larger `struct`. The check is recursive, so the following code will emit a warning:

```rust
struct Wrap<T>(T);
struct Outer(Wrap<Wrap<Wrap<Box<i32>>>>);
struct CannotBeZero {
    outer: Outer,
    foo: i32,
    bar: f32
}

...

let bad_value: CannotBeZero = unsafe { std::mem::uninitialized() };
```

Astute readers may note that Rust has more types that cannot be zero, notably `NonNull<T>` and `NonZero<T>`. For now, initialization of these structs with `mem::uninitialized` or `mem::zeroed` is *not* linted against.

These checks do not cover all cases of unsound use of `mem::uninitialized` or `mem::zeroed`, they merely help identify code that is definitely wrong. All code should still be moved to use `MaybeUninit` instead.

### `#[deprecated]` macros

The `#[deprecated]` attribute, first introduced in Rust 1.9.0, allows crate authors to notify their users an item of their crate is deprecated and will be removed in a future release. Rust 1.38.0 extends the attribute, allowing it to be applied to macros as well.

### `std::any::type_name`

For debugging, it is sometimes useful to get the name of a type. For instance, in generic code, you may want to see, at run-time, what concrete types a function's type parameters has been instantiated with. This can now be done using `std::any::type_name`:

```rust
fn gen_value<T: Default>() -> T {
    println!("Initializing an instance of {}", std::any::type_name::<T>());
    Default::default()
}

fn main() {
    let _: i32 = gen_value();
    let _: String = gen_value();
}
```

This prints:

```
Initializing an instance of i32
Initializing an instance of alloc::string::String
```

Like all standard library functions intended only for debugging, the exact contents and format of the string are not guaranteed. The value returned is only a best-effort description of the type; multiple types may share the same `type_name` value, and the value may change in future compiler releases.

### Library changes

- [`slice::{concat, connect, join}` now accepts `&[T]` in addition to `&T`.][62528]
- [`*const T` and `*mut T` now implement `marker::Unpin`.][62583]
- [`Arc<[T]>` and `Rc<[T]>` now implement `FromIterator<T>`.][61953]
- [`iter::{StepBy, Peekable, Take}` now implement `DoubleEndedIterator`.][61457]

Additionally, these functions have been stabilized:

- [`<*const T>::cast`] and [`<*mut T>::cast`]
- [`Duration::as_secs_f32`] and [`Duration::as_secs_f64`]
- [`Duration::div_f32`] and [`Duration::div_f64`]
- [`Duration::from_secs_f32`] and [`Duration::from_secs_f64`]
- [`Duration::mul_f32`] and [`Duration::mul_f64`]
- Euclidean remainder and division operations -- [`div_euclid`],
  [`rem_euclid`] -- for all integer primitives. `checked`,
  `overflowing`, and `wrapping` versions are also available.

[`<*const T>::cast`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast
[`<*mut T>::cast`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.cast-1
[`Duration::as_secs_f32`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs_f32
[`Duration::as_secs_f64`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs_f64
[`Duration::div_f32`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.div_f32
[`Duration::div_f64`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.div_f64
[`Duration::from_secs_f32`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.from_secs_f32
[`Duration::from_secs_f64`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.from_secs_f64
[`Duration::mul_f32`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.mul_f32
[`Duration::mul_f64`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.mul_f64
[`div_euclid`]: https://doc.rust-lang.org/std/primitive.i32.html#method.div_euclid
[`rem_euclid`]: https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid


[62528]: https://github.com/rust-lang/rust/pull/62528/
[62583]: https://github.com/rust-lang/rust/pull/62583/
[61953]: https://github.com/rust-lang/rust/pull/61953/
[61884]: https://github.com/rust-lang/rust/pull/61884/
[61457]: https://github.com/rust-lang/rust/pull/61457/

### Other changes

[relnotes-rust]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1380-2019-09-26
[relnotes-cargo]: https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-138-2019-09-26
[relnotes-clippy]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-138

There are other changes in the Rust 1.38 release: check out what changed in [Rust][relnotes-rust], [Cargo][relnotes-cargo], and [Clippy][relnotes-clippy].

### Corrections
A Previous version of this post mistakenly marked these functions as stable. They are not yet stable.
[`Duration::div_duration_f32`] and [`Duration::div_duration_f64`].

[`Duration::div_duration_f32`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.div_duration_f32
[`Duration::div_duration_f64`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.div_duration_f64

## Contributors to 1.38.0

Many people came together to create Rust 1.38.0. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.38.0/)
