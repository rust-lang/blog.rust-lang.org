+++
path = "2020/04/23/Rust-1.43.0"
title = "Announcing Rust 1.43.0"
authors = ["The Rust Release Team"]
aliases = [
    "2020/04/23/Rust-1.43.0.html",
    "releases/1.43.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.43.0. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.43.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.43.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1430-2020-04-23

## What's in 1.43.0 stable

This release is fairly minor. There are no new major features. We have some
new stabilized APIs, some compiler performance improvements, and a small
macro-related feature. See the [detailed release notes][notes] to learn about
other changes not covered by this post.

### `item` fragments

In macros, you can use `item` fragments to interpolate items into the body of traits,
impls, and extern blocks. For example:

```rust
macro_rules! mac_trait {
    ($i:item) => {
        trait T { $i }
    }
}
mac_trait! {
    fn foo() {}
}
```

This will generate:

```rust
trait T {
    fn foo() {}
}
```

### Type inference around primitives

The type inference around primitives, references, and binary operations was
improved. A code sample makes this easier to understand: this code fails to
compile on Rust 1.42, but compiles in Rust 1.43.

```rust
let n: f32 = 0.0 + &0.0;
```

In Rust 1.42, you would get an error that would say "hey, I don't know how to add
an `f64` and an `&f64` with a result of `f32`." The algorithm now correctly decides
that both `0.0` and `&0.0` should be `f32`s instead.

### New Cargo environment variable for tests

In a move to help integration testing, [Cargo will set some new environment
variables](https://github.com/rust-lang/cargo/pull/7697).

This is easiest to explain by example: let's say we're working on a command
line project, simply named "cli". If we're writing an integration test, we want
to invoke that `cli` binary and see what it does. When running tests and
benchmarks, Cargo will set an environment variable named `CARGO_BIN_EXE_cli`,
and I can use it inside my test:

```rust
let exe = env!("CARGO_BIN_EXE_cli");
```

This makes it easier to invoke `cli`, as we now have a path to it directly.

### Library changes

[You can now use associated constants on floats and integers directly][consts], rather
than having to import the module. That is, you can now write `u32::MAX` or `f32::NAN`
with no `use std::u32;` or `use std::f32;`.

[consts]: https://github.com/rust-lang/rust/pull/68952/

There is a [new `primitive`
module](https://github.com/rust-lang/rust/pull/67637/) that re-exports Rust's
primitive types. This can be useful when you're writing a macro and want to make
sure that the types aren't shadowed.

Additionally, we stabilized six new APIs:

- [`Once::is_completed`]
- [`f32::LOG10_2`]
- [`f32::LOG2_10`]
- [`f64::LOG10_2`]
- [`f64::LOG2_10`]
- [`iter::once_with`]

[`Once::is_completed`]: https://doc.rust-lang.org/std/sync/struct.Once.html#method.is_completed
[`f32::LOG10_2`]: https://doc.rust-lang.org/std/f32/consts/constant.LOG10_2.html
[`f32::LOG2_10`]: https://doc.rust-lang.org/std/f32/consts/constant.LOG2_10.html
[`f64::LOG10_2`]: https://doc.rust-lang.org/std/f64/consts/constant.LOG10_2.html
[`f64::LOG2_10`]: https://doc.rust-lang.org/std/f64/consts/constant.LOG2_10.html
[`iter::once_with`]: https://doc.rust-lang.org/std/iter/fn.once_with.html

### Other changes

[relnotes-cargo]: https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-143-2020-04-23
[relnotes-clippy]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-143

There are other changes in the Rust 1.43.0 release: check out what changed in
[Rust][notes], [Cargo][relnotes-cargo], and [Clippy][relnotes-clippy].

## Contributors to 1.43.0

Many people came together to create Rust 1.43.0. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.43.0/)
