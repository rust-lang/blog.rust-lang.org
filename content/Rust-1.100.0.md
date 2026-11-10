+++
path = "2026/11/12/Rust-1.100.0"
title = "Announcing Rust 1.100.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.100.0"]

[taxonomies]
tags = ["release"]
+++

The Rust team is happy to announce a new version of Rust, 1.100.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.100.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can get [`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.100.0](https://doc.rust-lang.org/stable/releases.html#version-11000-2026-11-12).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.100.0 stable

### Never type

After 10 years and 5 failed stabilization attempts, **the [never type] is finally stable**!

[never type]: https://doc.rust-lang.org/stable/std/primitive.never.html

The never type (spelled `!` in code) is the canonical uninhabited type, it represents the type of diverging computations – computations which never resolve to any value. It is impossible to create a value of the never type, as such, it is a marker of unreachable code. It is the type of expressions that terminate control flow, such as `return`, `break`, and [`std::process::exit`].

[`std::process::exit`]: https://doc.rust-lang.org/stable/std/process/fn.exit.html

Since having a value of the never type is impossible, Rust permits a never-to-any coercion, allowing to turn `!` into any other type (this is what distinguishes it from any other uninhabited types). This can be used, for example, to not return a value from one of the branches of a `match` or an `if`:

```rust
fn mrrrow(option: Option<u32>) {
    let value = match option {
        // `x` has type `u32`
        Some(x) => x,
        // `return` has type `!`, which is then coerced to `u32`,
        // allowing the `match` to pass type checking.
        None => return,
    };
    // ...
}
```

The never type could already have been "observed" on stable before – it was permitted as the return type of functions (`-> !`) and it was already the type of some built-in constructs (`return`, `break`, etc). What this release allows, is to name the never type anywhere:

```rust
fn infallible_unwrap(res: Result<T, !>) -> T {
    // ^^ writing `Result<_, !>` was not allowed before
    let Ok(v) = res;
    v
}
```

Additionally this release changes [`Infallible`] to be an alias to the never type. That makes all old APIs that used `Infallible` be compatible with using `!`. This was the plan ever since the introduction of `Infallible` and is one of the reasons why the never type stabilization was so hard (waffle gave a [talk](https://www.youtube.com/watch?v=3jM4cnEVrLc) about some of the troubles of the stabilization).

[`Infallible`]: https://doc.rust-lang.org/stable/std/convert/type.Infallible.html

Unfortunately, to allow changing `Infallible`, we had to introduce a breaking change. This release changes the "never type fallback" from `()` to `!` in editions **before** 2024 (i.e. 2015, 2018, and 2021), similarly to what has [already been done in edition 2024]. This should largely be unnoticeable, but can affect code in some rare cases (for example when ignoring a result of a function returning a generic parameter). Note that this change might also make the compiler detect more code as unreachable.

[already been done in edition 2024]: https://doc.rust-lang.org/edition-guide/rust-2024/never-type-fallback.html

We tried to minimize the impact of this breaking change as much as possible, by providing future compatibility warnings far in advance and releasing fixes to public crates that have been broken by this. If a dependency is failing due to issues with the never type, consider updating it, as a fix might have already been released for it.

### `Allocator` API

This release brings about the stabilization of the core parts of [`allocator_api`](https://github.com/rust-lang/rust/issues/32838), enabling library authors to define custom allocators that can be used together with certain standard library types - currently `Vec` and `Box`. These are enabled through the newly-stabilized `Allocator` trait.

A lot remains to be done still, including expanding the API surface usable with custom allocators and stabilizing their use on more collections and pointer types; with the core trait and design out of the way though, expect to see more being stabilized soon!

### `DropGuard`: run a closure on every exit path

The `Drop` trait in Rust makes it possible to run code when a variable goes out of scope, regardless of whether we `return`, `?`, or panic. `Drop` being a part of types means that [in Rust you never have to close a socket](https://blog.skylight.io/rust-means-never-having-to-close-a-socket/).

Implementing `Drop` can however feel a little inconvenient when all you want is to run some code unconditionally when a function exits. This is why we’re introducing `core::mem::DropGuard`: a new helper type that runs a closure when dropped.

`DropGuard` takes two arguments: a variable and a closure. It implements `Deref` and `DerefMut` so you can keep working with the variable as normal. But when dropped it runs the closure and consumes the variable.

`DropGuard` also implements a `dismiss` method which returns the variable from the guard without running the destructor. This makes it possible to quickly implement all-or-nothing transaction-like APIs with a rollback. Here is an example implementing a parser which uses a guard to roll back mutations in case of failure:

```rust
use std::mem::DropGuard;
use std::num::ParseIntError;

/// Attempt to parse all strings in `items` and push them into `nums`.
/// If parsing any of the strings fails, `nums` remains unchanged.
fn parse_all(nums: &mut Vec<u32>, strings: &[&str]) -> Result<(), ParseIntError> {
    let len = nums.len();
    let mut nums = DropGuard::new(nums, move |nums| {
        nums.truncate(len);    // ← 1. Reset the vec on failure
    });

    for s in strings {
        nums.push(s.parse()?); // ← 2. If `?` returns an error, roll back
    }

    DropGuard::dismiss(nums);  // ← 3. Parsing successful, dismiss the guard
    Ok(())
}
```

### Conjuring zero-sized types

As Rust's [unsafe code guidelines](https://github.com/rust-lang/unsafe-code-guidelines) are still being decided, it became clear that it would be very useful to decide what exactly it means to create zero-sized types in an unsafe way, and what we should recommend for people to do this.

This question is answered in Rust 1.100 with a single function: [`std::mem::conjure_zst`](https://doc.rust-lang.org/stable/std/mem/fn.conjure_zst.html).

Instead of transmuting `()`, `MaybeUninit::uninit().assume_init()`, or various other incantations, this is the dedicated way to create a value of a zero-sized type when you know that doing so is safe to do. If the type has no size, we know exactly what value it must be: nothing.

Of course, APIs often rely on the inability to construct such types for safety, and that's why this function is unsafe. For example, take this contrived example:

```rust
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

// look, I said the example was contrived, okay?
static HANDLE: Cell<*const AtomicUsize> = std::ptr::null();
static COUNTER: AtomicUsize = AtomicUsize::new(0);

// outside this module, you can't make your own `Handle`
pub struct Handle(());
impl Handle {
    pub fn setup() -> Handle {
        HANDLE.set(&raw const COUNTER);
        Handle(())
    }

    pub fn count(&self) -> usize {
        // SAFETY: We initialized `HANDLE` in the constructor, so, this is a valid pointer.
        let atomic = unsafe { &*HANDLE.get() };
        atomic.fetch_add(1, Ordering::Relaxed)
    }
}
```

Here, the module exploits the fact that `Handle` *cannot be used* without first setting up `HANDLE`, to make the `count` function safe. If we could just `conjure_zst::<Handle>()` in safe code, this is no longer sound, which is why it's unsafe to call `conjure_zst`.

### Reporting unused dependencies

Building dependencies that are no longer used or are only needed for specific platforms can slow down builds.
Rustc has the `unused_crate_dependencies` lint for this but it has false positives when used with Cargo because dependencies may be shared between multiple build targets, e.g. `dependencies.tracing-subscriber` being available for every `bin` in a package when only some may use it.

Cargo now reports unused dependencies to users, e.g.

```
warning: unused dependency `tracing-subscriber`
  --> Cargo.toml:23:1
   |
23 | tracing-subscriber.workspace = true
   | ^^^^^^^^^^^^^^^^^^
   |
   = note: `cargo::unused_dependencies` is set to `warn` by default
help: consider removing the dependency on `tracing-subscriber`
```

A limitation of this warning is that it will only appear when building every build target that may use an entry in a dependencies table.
For example, running `cargo check --bin foo` won't show warnings for unused `[dependencies]` because another `bin` may be present that uses them.
No warning will be shown for unused `dev-dependencies` because there isn't a way to ensure every development build target is built.

This warning is being introduced as part of a new linting system for Cargo, controlled by the `[lints.cargo]` table in `Cargo.toml`. See also the [other lints that have been introduced](https://doc.rust-lang.org/stable/cargo/reference/lints.html).

### Reducing Cargo cache collisions

Cargo builds all libraries and binaries in the `build-dir` and then uplifts final artifacts to the `target-dir. At this time, these share a common parent directory.
Different build configurations, like `--features`, can lead to different copies of libraries and binaries in the `build-dir`.
These copies are kept unique by a build-configuration hash but this hash was not always applied due to platform constraints.
This can lead to extra rebuilds, missed rebuilds, confusion over what binaries or libraries are used, and so on.
These problems get exacerbated when sharing a `build-dir` across git worktrees.

The `build-dir` now consistently applies the build-configuration hash to all content by organizing the directories around their build step, rather than their file type, and moving the hash from a file's name to the parent directory.

This also unblocks future improvements such as cache garbage collection, cross-workspace cache de-duplication, cross-workspace caching, and fine grain locking.

While this is an internal change, this may break build processes and tests that had workarounds for missing or incomplete features.
In some cases, the workaround is no longer needed.
See [cargo#17287](https://github.com/rust-lang/cargo/issues/17287) for possible solutions and to add any new use cases.

### Cargo min-publish-age a.k.a. dependency cooldowns

Malicious versions of packages are often caught within hours or daysafter being published, by either automated scanners or by the community.
Rust 1.100 stabilizes Cargo's min-publish-age aware dependency resolution, which lets you wait for that window before it picks up newly published versions.
This practice is also known as dependency cooldowns.

You can opt in via [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html#registryglobal-min-publish-age):

```toml
[registry]
global-min-publish-age = "3 days"
```

When selecting versions, Cargo now skips any version published more recently than that, and tells you what it held back:

```console
$ cargo update
    Updating crates.io index
     Locking 2 packages to highest compatible version as of min-publish-age
      Adding cc v1.11.0 (available: v1.18.0, published 3 hours ago)
      Adding clap 4.6.7 (available: 4.6.9, published 2 days ago)
```

When a new version is needed right away, you can override the check for a single update:

```console
$ CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow cargo update clap --precise 4.6.9
    Updating crates.io index
    Updating clap 4.6.7 -> clap 4.6.9 (published 2 days ago, minimum age 3 days)
```

The newer version will be recorded in `Cargo.lock` and kept from then on until a later `cargo update` re-resolves it.

A few things to keep in mind:

* This depends on the registry recording when each version was published.
  Versions without a `pubtime` field in the index are never held back.

* `cargo install` ignores the minimum publish age, for both the installed crate and its dependencies.
  Pin a version with `cargo install foo@1.2.3` if needed.

* Cooldowns reduce the risk from some supply chain attacks but are not a complete defense on their own.
  Please keep reviewing dependency updates as you otherwise would.

### Stabilized APIs

- [`impl PartialEq<Cow<'_, [U]>> for Vec<T, A>`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#impl-PartialEq%3CCow%3C'_,+%5BU%5D%3E%3E-for-Vec%3CT,+A%3E)
- [`impl PartialEq<Cow<'_, [U]>> for &[T]`](https://doc.rust-lang.org/stable/std/primitive.slice.html#impl-PartialEq%3CCow%3C'_,+%5BU%5D%3E%3E-for-%26%5BT%5D)
- [`impl PartialEq<Cow<'_, [U]>> for &mut [T]`](https://doc.rust-lang.org/stable/std/primitive.slice.html#impl-PartialEq%3CCow%3C'_,+%5BU%5D%3E%3E-for-%26mut+%5BT%5D)
- [`std::alloc::Allocator`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html)
- [`std::alloc::Global`](https://doc.rust-lang.org/stable/std/alloc/struct.Global.html)
- [`std::alloc::System`](https://doc.rust-lang.org/stable/std/alloc/struct.System.html)
- [`impl Allocator for Global`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-Global)
- [`impl Allocator for System`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-System)
- [`impl<A: Allocator> Allocator for &A`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-%26A)
- [`impl<T: Allocator, A: Allocator> Allocator for Box<T, A>`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-Box%3CT,+A%3E)
- [`impl<A: Allocator> Allocator for &mut A`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-%26mut+A)
- [`impl<T: Allocator, A: Allocator> Allocator for Arc<T, A>`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-Arc%3CT,+A%3E)
- [`impl<T: Allocator, A: Allocator> Allocator for Rc<T, A>`](https://doc.rust-lang.org/stable/std/alloc/trait.Allocator.html#impl-Allocator-for-Rc%3CT,+A%3E)
- [`Box::new_in`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.new_in)
- [`Box::from_raw_in`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.from_raw_in)
- [`Box::from_non_null_in`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.from_non_null_in)
- [`Box::into_raw_with_allocator`](https://doc.rust-lang.org/std/std/boxed/struct.Box.html#method.into_raw_with_allocator)
- [`Box::into_non_null_with_allocator`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.into_non_null_with_allocator)
- [`Box::allocator`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.allocator)
- [`Vec::new_in`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.new_in)
- [`Vec::with_capacity_in`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.with_capacity_in)
- [`Vec::from_raw_parts_in`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.from_raw_parts_in)
- [`Vec::from_parts_in`](https://doc.rust-lang.org/stable/stable/vec/struct.Vec.html#method.from_parts_in)
- [`Vec::into_raw_parts_with_allocator`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.into_raw_parts_with_allocator)
- [`Vec::into_parts_with_allocator`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.into_parts_with_allocator)
- [`Vec::allocator`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.allocator)
- [`Child::main_thread_handle`](https://doc.rust-lang.org/stable/std/os/windows/process/trait.ChildExt.html#tymethod.main_thread_handle)
- [`impl Default for std::sync::Once`](https://doc.rust-lang.org/stable/std/sync/struct.Once.html#impl-Default-for-Once)
- [`bool::toggle`](https://doc.rust-lang.org/stable/std/primitive.bool.html#method.toggle)
- [`Box::take`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.take)
- [`Box::map`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.map)
- [`Rc::map`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.map)
- [`Arc::map`](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#method.map)
- [`impl AsRef<T> for !`](https://doc.rust-lang.org/stable/std/primitive.never.html#impl-AsRef<T>-for-!)
- [`impl AsMut<T> for !`](https://doc.rust-lang.org/stable/std/primitive.never.html#impl-AsMut<T>-for-!)
- [`core::mem::DropGuard`](https://doc.rust-lang.org/stable/core/mem/struct.DropGuard.html)
- [`core::mem::conjure_zst`](https://doc.rust-lang.org/stable/core/mem/fn.conjure_zst.html)
- [`UnsafeCell::replace`](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html#method.replace)
- [`UnsafeCell::as_ref_unchecked`](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html#method.as_ref_unchecked)
- [`UnsafeCell::as_mut_unchecked`](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html#method.as_mut_unchecked)
- [`str::trim_prefix`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.trim_prefix)
- [`str::trim_suffix`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.trim_suffix)
- [`[T]::trim_prefix`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.trim_prefix)
- [`[T]::trim_suffix`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.trim_suffix)
- [`Path::trim_prefix`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.trim_prefix)
- [`Vec::from_fn`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.from_fn)
- [`os::windows::process::CommandExt::show_window`](https://doc.rust-lang.org/stable/std/os/windows/process/trait.CommandExt.html#tymethod.show_window)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.100.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-1100-2026-11-12), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-1100).

## Contributors to 1.100.0

Many people came together to create Rust 1.100.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.100.0/)
