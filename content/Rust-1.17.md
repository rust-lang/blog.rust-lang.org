+++
path = "2017/04/27/Rust-1.17"
title = "Announcing Rust 1.17"
authors = ["The Rust Core Team"]
aliases = [
    "2017/04/27/Rust-1.17.html",
    "releases/1.17.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.17.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.17 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.17.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1170-2017-04-27

### What's in 1.17.0 stable

The story of Rust 1.17.0 is mostly one of small, quality of life improvements. For example,
[the `'static` lifetime is now assumed in statics and consts](https://github.com/rust-lang/rust/pull/39265). When writing a const or static like this:

```rust
const NAME: &'static str = "Ferris";
static NAME: &'static str = "Ferris";
```

Rust 1.17 will allow you to elide the `'static`, since that's the only lifetime that makes
sense:

```rust
const NAME: &str = "Ferris";
static NAME: &str = "Ferris";
```

In some situations, this can remove lots of boilerplate:

```rust
// old
const NAMES: &'static [&'static str; 2] = &["Ferris", "Bors"];

// new
const NAMES: &[&str; 2] = &["Ferris", "Bors"];
```

Another similar improvement is "field init shorthand." Similar to ECMAScript 6,
which calls this "Object Literal Property Value Shorthand", duplication can be
removed when declaring structs, like this:

```rust
// definitions
struct Point {
    x: i32,
    y: i32,
}

let x = 5;
let y = 6;

// old
let p = Point {
    x: x,
    y: y,
};

// new
let p = Point {
    x,
    y,
};
```

That is, the `x, y` form will assume that its values are set to a variable
with the same name in its scope.

For another small quality of life improvement, it's common for new Rustaceans to
try to use `+` to add two `&str`s together. This doesn't work, you can only add
`String + &str`. As such, [a new error
message](https://github.com/rust-lang/rust/pull/39116) was added to help users
who make this mistake:

```rust
// code
"foo" + "bar"

// old
error[E0369]: binary operation `+` cannot be applied to type `&'static str`
 --> <anon>:2:5
  |
2 |     "foo" + "bar"
  |     ^^^^^
  |
note: an implementation of `std::ops::Add` might be missing for `&'static str`
 --> <anon>:2:5
  |
2 |     "foo" + "bar"
  |     ^^^^^

// new
error[E0369]: binary operation `+` cannot be applied to type `&'static str`
 --> <anon>:2:5
  |
2 |     "foo" + "bar"
  |     ^^^^^
  |
  = note: `+` can't be used to concatenate two `&str` strings
help: to_owned() can be used to create an owned `String` from a string
reference. String concatenation appends the string on the right to the string on
the left and may require reallocation. This requires ownership of the string on
the left.
  |     "foo".to_owned() + "bar"
```

When using Cargo's build scripts, you must set the location of the script in your
`Cargo.toml`. However, the vast majority of people wrote `build = "build.rs"`, using
a `build.rs` file in the root of their project. [This convention is now encoded
into Cargo](https://github.com/rust-lang/cargo/pull/3664), and will be assumed if
`build.rs` exists. We've been warning about this change for the past few releases,
and you can use `build = false` to opt out.

This release marks [the removal](https://github.com/rust-lang/rust/pull/39431)
of the old `Makefile` based build system. The new system, announced in Rust
1.15, is written in Rust and primarily uses Cargo to drive the build. It is now
mature enough to be the only build system.

As part of that change, packages from crates.io can now be used within Rust's
build system. The first one to be added was [mdBook](https://crates.io/crates/mdbook),
and [it's now being used](https://github.com/rust-lang/rust/pull/39633) to render
our various book-like documentation:

* [The book](https://doc.rust-lang.org/stable/book/) ([repo](https://github.com/rust-lang/book))
* [The reference](https://doc.rust-lang.org/stable/reference/) ([repo](https://github.com/rust-lang-nursery/reference))
* [The nomicon](https://doc.rust-lang.org/stable/nomicon/) ([repo](https://github.com/rust-lang-nursery/nomicon))

In addition, see those links to their respective repositories; they've been
moved out of tree. Also, we've added a fourth book, still in-tree: [The
Unstable Book](https://doc.rust-lang.org/stable/unstable-book/). This
provides an overview of unstable features by name, contains links to their
tracking issues, and may contain initial documentation.
If there's a feature you want to see stabilized, please get involved on
its tracking issue!

A few releases ago, `rustup` stopped installing documentation
by default. We made this change to save some bandwidth and because not
all users want a copy of the documentation locally. However, this created
a pitfall: some users did not realize that this changed, and would only
notice once they were no longer connected to the internet. In addition,
some users *did* want to have a local copy of the docs, regardless of
their connectivity. As such, we've [reverted the change](https://github.com/rust-lang/rust/pull/40526), and documentation is being
installed by default again.

Finally, while this release is full of improvements, there is one small
step back we want to regretfully inform you about. On Windows, Visual
Studio 2017 has been released, and Microsoft has changed the structure
of how the software is installed. [Rust cannot automatically detect this
location](https://github.com/rust-lang/rust/issues/38584), and while we
were working on the necessary changes, they did not make it in time for
this release. Until then, Visual Studio 2015 still works fine, or you
can run `vcvars.bat` on the command line. We hope to make this work
in a seamless fashion soon.

See the [detailed release notes][notes] for more.

#### Library stabilizations

19 new bits of API were stabilized this release:

* [`Arc::into_raw`] and [`Rc::into_raw`] let you consume an `Arc` or `Rc` and get a raw pointer.
* [`Arc::from_raw`] and [`Rc::from_raw`] let you take that raw pointer and get an `Arc` or `Rc`.
* [`Arc::ptr_eq`] and [`Rc::ptr_eq`] return true if the two `Arc`s or two `Rc`s point to the same value (not just values that compare as equal).
* [`Ordering::then`] lets you chain two `Ordering`s together, and [`Ordering::then_with`] lets you do it with a function.
* [`BTreeMap::range`] allows you to iterate over a portion of a `BTreeMap`, and [`BTreeMap::range_mut`] lets you do it mutably. [`collections::Bound`] can give you even more control.
* [`process::abort`] will completely terminate a process in an abnormal fashion.
* [`ptr::read_unaligned`] and [`ptr::write_unaligned`] are like `ptr::read` and `ptr::write`, but without alignment requirements.
* [`Result::expect_err`] mirrors `Result::expect`, but with the `Err` case rather than the `Ok` case.
* [`Cell::swap`] is similar to `std::mem::swap`, but lets you do it with `&Cell` instead of `&mut T`.
* [`Cell::replace`] is similar to `std::mem::replace`, but lets you do it with `&Cell` instead of `&mut T`.
* [`Cell::into_inner`] lets you consume the `Cell`, and extract its value.
* [`Cell::take`] lets you take the value out of a `Cell`, leaving its `Default::default` behind.

[`Arc::from_raw`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.from_raw
[`Arc::into_raw`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.into_raw
[`Arc::ptr_eq`]: https://doc.rust-lang.org/std/sync/struct.Arc.html#method.ptr_eq
[`BTreeMap::range_mut`]: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.range_mut
[`BTreeMap::range`]: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.range
[`Cell::into_inner`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.into_inner
[`Cell::replace`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.replace
[`Cell::swap`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.swap
[`Cell::take`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.take
[`Ordering::then_with`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
[`Ordering::then`]: https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then
[`Rc::from_raw`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.from_raw
[`Rc::into_raw`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.into_raw
[`Rc::ptr_eq`]: https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq
[`Result::expect_err`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err
[`collections::Bound`]: https://doc.rust-lang.org/std/collections/enum.Bound.html
[`process::abort`]: https://doc.rust-lang.org/std/process/fn.abort.html
[`ptr::read_unaligned`]: https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html
[`ptr::write_unaligned`]: https://doc.rust-lang.org/std/ptr/fn.write_unaligned.html

In other changes, `Cell<T>` used to require that `T: Copy` for many of its methods,
but [this has been relaxed significantly](https://github.com/rust-lang/rust/pull/39793).

`Box<T>` [now implements](https://github.com/rust-lang/rust/pull/40009) over a dozen new
conversions with `From`.

`SocketAddr` and `IpAddr` have [some new conversions](https://github.com/rust-lang/rust/pull/39372)
as well. Previously, you may have written code like this:

```rust
"127.0.0.1:3000".parse().unwrap()
```

Now, you can write

```rust
SocketAddr::from(([127, 0, 0, 1], 3000))
// or even
([127, 0, 0, 1], 3000).into()
```

This removes some unnecessary run-time parsing, and is roughly as readable, depending on
your preferences.

Backtraces [now have nicer formatting](https://github.com/rust-lang/rust/pull/38165), eliding
some things by default. For example, the full backtrace:

```
thread 'main' panicked at 'explicit panic', foo.rs:2
stack backtrace:
   1:     0x55c39a23372c - std::sys::imp::backtrace::tracing::imp::write::hf33ae72d0baa11ed
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x55c39a23571e - std::panicking::default_hook::{{closure}}::h59672b733cc6a455
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x55c39a235324 - std::panicking::default_hook::h1670459d2f3f8843
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:367
   4:     0x55c39a235afb - std::panicking::rust_panic_with_hook::hcf0ddb069e7beee7
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x55c39a22e866 - std::panicking::begin_panic::heb433e9aa28a7408
   6:     0x55c39a22e9bf - foo::main::hd216d4a160fcce19
   7:     0x55c39a23d44a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
   8:     0x55c39a236006 - std::rt::lang_start::hd7c880a37a646e81
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panicking.rs:436
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/panic.rs:361
                        at /buildslave/rust-buildbot/slave/stable-dist-rustc-linux/build/src/libstd/rt.rs:57
   9:     0x55c39a22e9e9 - main
  10:     0x7f5e5ed3382f - __libc_start_main
  11:     0x55c39a22e6b8 - _start
  12:                0x0 - <unknown>
```

is now instead

```
thread 'main' panicked at 'explicit panic', foo.rs:2
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:355
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:371
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:549
   5: std::panicking::begin_panic
   6: foo::main
   7: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:98
   8: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:433
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:57
   9: main
  10: __libc_start_main
  11: _start
```

By default. You can set the environment variable `RUST_BACKTRACE=full` to get the full
backtrace. We may be able to do more cleanup in the future; see [this bug](https://github.com/rust-lang/rust/pull/40264) for more.

See the [detailed release notes][notes] for more.

#### Cargo features

Other than the previously mentioned `build.rs` changes, Cargo has a few new improvements.
[`cargo check --all`](https://github.com/rust-lang/cargo/pull/3731) and
[`cargo run --package`](https://github.com/rust-lang/cargo/pull/3691) are two missing
flags that are now supported.

You can now [opt in to ignoring SSL revocation checks](https://github.com/rust-lang/cargo/pull/3699). The default is still to check, of course.

A new field in `Cargo.toml`, `required-features`, lets you [specify specific features
that must be set for a target to be built](https://github.com/rust-lang/cargo/pull/3667).
Here's an example: let's say that we are writing a crate that interacts with databases,
and that we support multiple databases. We might have this in our `Cargo.toml`:

```toml
[features]
# ...
postgres = []
sqlite = []
tools = []
```

The `tools` feature allows us to include extra tooling, and the `postgres` and `sqlite`
features control which databases we want to support.

Previously, `cargo build` would attempt to build all targets, which is normally what
you want. But what if we had a `src/bin/postgres-tool.rs`, that would only really
be relevant if the `postgres` and `tools` features would be enabled? Previously,
we would have to write something like this:

```rust
#[cfg(not(all(feature = "postgres", feature = "tools")))]
fn main() {
    println!("This tool requires the `postgres` and `tools` features to be enabled.");
}

#[cfg(all(feature = "postgres", feature = "tools"))]
fn main() {
    // real code
}
```

This is a lot of boilerplate to work around `cargo build`'s behavior. It's even
more unfortunate with `examples/`, which are supposed to show off how to use
your library, but this shenanigans is only relevant within the package, not if
you were to try to use the example on your own.

With the new `required-features` key, we can add this:

```toml
[[bin]]
# ...
required-features = ["postgres", "tools"]
```

Now, `cargo build` will only build our `postgres-tool` if we have the two features
set, and so we can write a normal `fn main` without all the `cfg` nonsense getting
in the way.

See the [detailed release notes][notes] for more.

### Contributors to 1.17.0

Many people came together to create Rust 1.17. We couldn't have done it without
all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.17.0)
