+++
path = "2022/06/30/Rust-1.62.0"
title = "Announcing Rust 1.62.0"
authors = ["The Rust Release Team"]
aliases = [
    "2022/06/30/Rust-1.62.0.html",
    "releases/1.62.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.62.0. Rust is a programming language
empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.62.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.62.0][notes] on GitHub.

If you'd like to help us out by testing future releases, you might consider updating locally to use
the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`).
Please [report] any bugs you might come across!

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1620-2022-06-30
[report]: https://github.com/rust-lang/rust/issues/new/choose

## What's in 1.62.0 stable

### `cargo add`

You can now add new dependencies directly from the command line using `cargo add`. This command supports specifying features and versions. It can also be used to modify existing dependencies.

For example:

```
cargo add log
cargo add serde --features derive
cargo add nom@5
```

See the [cargo documentation](https://doc.rust-lang.org/nightly/cargo/commands/cargo-add.html) for more.

### `#[default]` enum variants

You can now use `#[derive(Default)]` on enums if you specify a default variant. For example, until now you would have to manually write a `Default` impl for this enum:


```rust
#[derive(Default)]
enum Maybe<T> {
    #[default]
    Nothing,

    Something(T),
}
```

As of now only "unit" variants (variants that have no fields) are allowed to be marked `#[default]`. More information is available in the [RFC](https://rust-lang.github.io/rfcs/3107-derive-default-enum.html) for this feature.

### Thinner, faster mutexes on Linux

Previously, `Mutex`, `Condvar`, and `RwLock` were backed by the pthreads library on Linux. The pthreads locks support more features than the Rust APIs themselves do, including runtime configuration, and are designed to be used in languages with fewer static guarantees than Rust provides.

The mutex implementation, for example, is 40 bytes and cannot be moved. This forced the standard library to allocate a `Box` behind the scenes for each new mutex for platforms that use pthreads.

Rust's standard library now ships with a raw futex-based implementation of these locks on Linux, which is very lightweight and doesn't require extra allocation. In 1.62.0 `Mutex` only needs 5 bytes for its internal state on Linux, though this may change in future versions.

This is part of a long effort to improve the efficiency of Rust's lock types, which includes previous improvements on Windows such as unboxing its primitives. You can read more about that effort in the [tracking issue](https://github.com/rust-lang/rust/issues/93740).

### Bare metal `x86_64` target

It's now easier to build OS-less binaries for `x86_64`, for example when writing a kernel. The [`x86_64-unknown-none` target](https://doc.rust-lang.org/beta/rustc/platform-support/x86_64-unknown-none.html) has been promoted to [Tier 2](https://doc.rust-lang.org/rustc/platform-support.html#tier-2) and can be installed with rustup.

```
$ rustup target add x86_64-unknown-none
$ rustc --target x86_64-unknown-none my_no_std_program.rs
```

You can read more about development using `no_std` in the [Embedded Rust book](https://docs.rust-embedded.org/book/intro/no-std.html).

### Stabilized APIs

The following methods and trait implementations are now stabilized:

- [`bool::then_some`]
- [`f32::total_cmp`]
- [`f64::total_cmp`]
- [`Stdin::lines`]
- [`windows::CommandExt::raw_arg`]
- [`impl<T: Default> Default for AssertUnwindSafe<T>`]
- [`From<Rc<str>> for Rc<[u8]>`][rc-u8-from-str]
- [`From<Arc<str>> for Arc<[u8]>`][arc-u8-from-str]
- [`FusedIterator for EncodeWide`]
- [RDM intrinsics on aarch64][stdarch/1285]

### Other changes

There are other changes in the Rust 1.62.0 release. Check out what changed in
[Rust](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1620-2022-06-30),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-162-2022-06-30),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-162).

### Contributors to 1.62.0

Many people came together to create Rust 1.62.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.62.0/)

[`bool::then_some`]: https://doc.rust-lang.org/stable/std/primitive.bool.html#method.then_some
[`f32::total_cmp`]: https://doc.rust-lang.org/stable/std/primitive.f32.html#method.total_cmp
[`f64::total_cmp`]: https://doc.rust-lang.org/stable/std/primitive.f64.html#method.total_cmp
[`Stdin::lines`]: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.lines
[`impl<T: Default> Default for AssertUnwindSafe<T>`]: https://doc.rust-lang.org/stable/std/panic/struct.AssertUnwindSafe.html#impl-Default
[rc-u8-from-str]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#impl-From%3CRc%3Cstr%3E%3E
[arc-u8-from-str]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#impl-From%3CArc%3Cstr%3E%3E
[stdarch/1285]: https://github.com/rust-lang/stdarch/pull/1285
[`windows::CommandExt::raw_arg`]: https://doc.rust-lang.org/stable/std/os/windows/process/trait.CommandExt.html#tymethod.raw_arg
[`FusedIterator for EncodeWide`]: https://doc.rust-lang.org/stable/std/os/windows/ffi/struct.EncodeWide.html#impl-FusedIterator
