+++
path = "2018/05/29/Rust-1.26.1"
title = "Announcing Rust 1.26.1"
authors = ["The Rust Core Team"]
aliases = [
    "2018/05/29/Rust-1.26.1.html",
    "releases/1.26.1",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.26.1. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.26.1 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.26.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1261-2018-05-29

## What's in 1.26.1 stable

A couple of issues were found in 1.26.0 which were deemed sufficient for a patch release.

A quick summary of the changes:

* RLS no longer interferes with command line builds
* Rustfmt stopped badly formatting text in some cases
* Returning from main via `impl Trait` where the Trait is not `Termination` is no longer permitted
* `::<>` (turbofish) no longer works for method arguments whose type is `impl Trait`
* `NaN > NaN` no longer returns true in const contexts
* rustup should no longer fail due to missing documentation on some platforms

If your code continues to compile, only the change to floating point
comparisons may alter behavior.

### RLS no longer interferes with command line builds

The version of RLS shipped with 1.26.0 utilized the same target directory as
Cargo from the command line, which meant that switching between the two would
lead to everything being recompiled. This problem was made worse for Windows
users due to a filesystem lock being left unreleased by either RLS or the
compiler, leading to an increased error rate. This latter bug is not yet
fixed, but it happens much less frequently with the first bug fixed.

### Rustfmt bad formatting

Previously, rustfmt would overindent multi-line string literals, which is now
fixed.

### Returning from main with `impl Trait` no longer works when Trait isn't Termination

Previously, we only checked that the underlying type implemented the
`Termination` trait. It is now only possible to return concrete types on
stable, as nothing except for `impl Termination` will work, but that trait is
currently unstable to import.

For example, this will no longer work on 1.26.1:

```rust
fn main() -> impl Copy {}
```

But this will keep working, as it doesn't attempt to return any hidden types
via `impl Trait`, but rather names types concretely.

```rust
fn main() -> Result<(), std::io::Error> {
    Ok(())
}
```

### Turbofish no longer works for method arguments with `impl Trait`

Previously, we accidentally permitted code to specify the type of method
arguments which use `impl Trait`. On 1.26.0, the code below would work, but
how exactly turbofish (`::<u32>` below) should interact with `impl Trait`
hasn't yet been decided, so we're preventing turbofish use until we can be
sure the semantics are as we desire.

```rust
struct Foo;

impl Foo {
    fn bar(&self, _arg: impl Copy) {}
}

fn main() {
    Foo.bar::<u32>(0);
}
```

### Floating point comparisons changed in constant contexts

Previously, comparing `NaN` as greater than other floating point numbers in a constant
context would return true, which is a bug; now, this comparison returns false.
In some cases that may mean that the behavior of code will change, but we
expect this to be relatively unlikely.

```rust
use std::f64::NAN;
const FOO: bool = ::std::f64::NAN >= ::std::f64::NAN;
// On 1.26.0
assert_eq!(FOO, true);
// On 1.26.1
assert_eq!(FOO, false);
```

### rustup should now work to install stable on platforms with missing docs

During the development cycle for 1.26, a change was made to how we build the
documentation for the standard library, which made it so that we stopped
producing the documentation component for a variety of tier 2 platforms.  This
led to breakage when running `rustup update` on those platforms, as rustup
refused to partially install Rust. Some users will need to run `rustup install
stable` instead of `rustup update` to make rustup avoid the missing docs
component, but this should be a one-time problem.

This was unfortunately fixed too late to make it into 1.26 stable, so we added
the patch for 1.26.1 to permit users to install Rust on these platforms.

```
$ rustup update
info: syncing channel updates for 'stable-x86_64-unknown-freebsd'
info: latest update on 2018-05-10, rust version 1.26.0 (a77568041 2018-05-07)
error: component 'rust-docs' for 'x86_64-unknown-freebsd' is unavailable for download
```
