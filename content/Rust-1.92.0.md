+++
path = "2025/12/11/Rust-1.92.0"
title = "Announcing Rust 1.92.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.92.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.92.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.92.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.92.0](https://doc.rust-lang.org/stable/releases.html#version-1920-2025-12-11).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.92.0 stable

### Deny-by-default never type lints

The language and compiler teams continue to work on stabilization of the [never type](https://doc.rust-lang.org/stable/std/primitive.never.html). In this release the [`never_type_fallback_flowing_into_unsafe`](https://doc.rust-lang.org/beta/rustc/lints/listing/deny-by-default.html#dependency-on-unit-never-type-fallback) and [`dependency_on_unit_never_type_fallback`](https://doc.rust-lang.org/beta/rustc/lints/listing/deny-by-default.html#dependency-on-unit-never-type-fallback) future compatibility lints were made deny-by-default, meaning they will cause a compilation error when detected.

These lints detect code which is likely to be broken by the never type stabilization. It is highly advised to fix them if they are reported in your crate.

### `unused_must_use` no longer warns about `Result<(), UninhabitedType>`

Rust's `unused_must_use` lint warns when ignoring the return value of a function, if the function or its return type is annotated with `#[must_use]`. For instance, this warns if ignoring a return type of `Result`, to remind you to use `?`, or something like `.expect("...")`.

However, some functions return `Result`, but the error type they use is not actually "inhabited", meaning it can never exist in real code (e.g. the [`!`](https://doc.rust-lang.org/std/primitive.never.html) or [`Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html) types).

The `unused_must_use` lint now no longer warns on `Result<(), UninhabitedType>`, or on `ControlFlow<UninhabitedType, ()>`. For instance, it will not warn on `Result<(), !>`. This avoids having to check for an error that can never happen.

```rust
fn can_never_fail() -> Result<(), !> {
    // ...
    Ok(())
}

fn main() {
    can_never_fail()
}
```

This is particularly useful with the common pattern of a trait with an associated error type, where the error type may *sometimes* be infallible:

```rust
trait UsesAssocErrorType {
    type Error;
    fn method(&self) -> Result<(), Self::Error>;
}

struct CannotFail;
impl UsesAssocErrorType for CannotFail {
    type Error = !;
    fn method(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct CanFail;
impl UsesAssocErrorType for CanFail {
    type Error = std::io::Error;
    fn method(&self) -> Result<(), Self::Error> {
        Err(std::io::Error::other("something went wrong"))
    }
}

fn main() {
    CannotFail.method(); // No error
    CanFail.method(); // Error: unused `Result` that must be used
}
```

### Emit unwind tables even when `-Cpanic=abort` is enabled on linux

Backtraces with `-Cpanic=abort` previously worked in Rust 1.22 but were broken in Rust 1.23, as we stopped emitting unwind tables with `-Cpanic=abort`. In Rust 1.45 a workaround in the form of `-Cforce-unwind-tables=yes` was stabilized.

In Rust 1.92 unwind tables will be emitted by default even when `-Cpanic=abort` is specified, allowing for backtraces to work properly. If unwind tables are not desired then users should use `-Cforce-unwind-tables=no` to explicitly disable them being emitted.

### Validate input to `#[macro_export]`

Over the past few releases, many changes were made to the way built-in attributes are processed in the compiler. This should greatly improve the error messages and warnings rust gives for built-in attributes and especially make these diagnostics more consistent among all of the over 100 built-in attributes.

To give a small example, in this release specifically, Rust became stricter in checking what arguments are allowed to `macro_export` by [upgrading that check to a "deny-by-default lint" that will be reported in dependencies.](https://github.com/rust-lang/rust/pull/143857). 

### Restrictions on user code impls of `DerefMut` for `Pin`

A soundness issue with `Pin` has been solved by preventing third-party crates from implementing `DerefMut` for `Pin<T>` when `T` is a local type that doesn't implement `DerefMut<Target: Unpin>`.

### Stabilized APIs

...

These previously stable APIs are now stable in const contexts:

...

### Platform Support

Refer to Rust’s [platform support page][platform-support] for more information on Rust’s tiered platform support.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.92.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-192-2025-12-11), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-192).

## Contributors to 1.92.0

Many people came together to create Rust 1.92.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.92.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
