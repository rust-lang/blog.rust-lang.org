+++
path = "2018/09/13/Rust-1.29"
title = "Announcing Rust 1.29"
authors = ["The Rust Core Team"]
aliases = [
    "2018/09/13/Rust-1.29.html",
    "releases/1.29.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.29.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.29.0 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.29.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1290-2018-09-13

## What's in 1.29.0 stable

The 1.29 release is fairly small; Rust 1.30 and 1.31 are going to have a lot
in them, and so much of the 1.29 cycle was spent preparing for those
releases. The two most significant things in this release aren't even language
features: they're new abilities that Cargo has grown, and they're both about lints.

* `cargo fix` can automatically fix your code that has warnings
* `cargo clippy` is a bunch of lints to catch common mistakes and improve your Rust code

### `cargo fix`

With the release of Rust 1.29, Cargo has a new subcommand: `cargo fix`. If you've written
code in Rust before, you've probably seen a compiler warning before. For example, consider
this code:

```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```

Here, we're calling `do_something` a hundred times. But we never use the variable `i`.
And so Rust warns:

```
$ cargo build
   Compiling myprogram v0.1.0 (file:///path/to/myprogram)
warning: unused variable: `i`
 --> src\main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

See how it suggests that we use `_i` as a name instead? We can automatically
apply that suggestion with `cargo fix`:

```
$ cargo fix
    Checking myprogram v0.1.0 (file:///C:/Users/steve/tmp/fix)
      Fixing src\main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

If we look at `src\main.rs` again, we'll see that the code has changed:

```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```

We're now using `_i`, and the warning will no longer appear.

This initial release of `cargo fix` only fixes up a small number of warnings.
The compiler has an API for this, and it only suggests fixing lints that
we're confident recommend correct code. Over time, as our suggestions
improve, we'll be expanding this to automatically fix more warnings.

if you find a compiler suggestion and want to help make it fixable, please
leave a comment on [this
issue](https://github.com/rust-lang/rust/issues/50723).

### `cargo clippy`

Speaking of warnings, you can now check out a preview of `cargo clippy` through Rustup.
Clippy is a large number of additional warnings that you can run against your Rust code.

For example:

```rust
let mut lock_guard = mutex.lock();

std::mem::drop(&lock_guard)

operation_that_requires_mutex_to_be_unlocked();
```

This code is syntactically correct, but may have a deadlock! You see, we
dropped *a reference to `lock_guard`*, not the guard itself. Dropping
a reference is a no-op, and so this is almost certainly a bug.

We can get the preview of Clippy from Rustup:

```
$ rustup component add clippy-preview
```

and then run it:

```
$ cargo clippy
error: calls to `std::mem::drop` with a reference instead of an owned value. Dropping a reference does nothing.
 --> src\main.rs:5:5
  |
5 |     std::mem::drop(&lock_guard);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[deny(drop_ref)] on by default
note: argument has type &std::result::Result<std::sync::MutexGuard<'_, i32>, std::sync::PoisonError<std::sync::MutexGuard<'_, i32>>>
 --> src\main.rs:5:20
  |
5 |     std::mem::drop(&lock_guard);
  |                    ^^^^^^^^^^^
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.212/index.html#drop_ref
```

As you can see from that help message, you can view all of the lints that
clippy offers on the web.

Please note that this is a preview; clippy has not yet reached 1.0. As such,
its lints may change. We'll release a `clippy` component once it has stabilized;
please give the preview a try and let us know how it goes.

Oh, and one more thing: [you can't use clippy with `cargo-fix` yet,
really](https://github.com/rust-lang-nursery/rustfix/issues/130). It's in the works!

See the [detailed release notes][notes] for more.

### Library stabilizations

Three APIs were stabilized this release:

* [`Arc<T>::downcast`](https://doc.rust-lang.org/std/sync/struct.Arc.html#method.downcast)
* [`Rc<T>::downcast`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.downcast)
* [`Iterator::flatten`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten)

Additionally, you can [now compare `&str` and
`OsString`](https://github.com/rust-lang/rust/pull/51178/).

See the [detailed release notes][notes] for more.

### Cargo features

We covered the two new subcommands to Cargo above, but additionally, [Cargo
will now try to fix up lockfiles that have been corrupted by a `git
merge`](https://github.com/rust-lang/cargo/pull/5831/). You can pass
`--locked` to disable this behavior.

`cargo doc` has also grown a new flag:
[`--document-private-items`](https://github.com/rust-lang/cargo/pull/5543).  By
default, `cargo doc` only documents public things, as the docs it produces are
intended for end-users. But if you're working on your own crate, and you have
internal documentation for yourself to refer to, `--document-private-items`
will generate docs for all items, not just public ones.

See the [detailed release notes][notes] for more.

## Contributors to 1.29.0

Many people came together to create Rust 1.29. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.29.0)
