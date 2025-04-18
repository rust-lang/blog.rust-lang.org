+++
path = "2017/10/12/Rust-1.21"
title = "Announcing Rust 1.21"
authors = ["The Rust Core Team"]
aliases = [
    "2017/10/12/Rust-1.21.html",
    "releases/1.21.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.21.0. Rust
is a systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.21 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.21.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1210-2017-10-12

### What's in 1.21.0 stable

This release contains some very minor, but nice-to-have features, as well as
some new documentation.

First up, a small change to literals. Consider code like this:

```rust
let x = &5;
```

In Rust, this code is synonymous with:

```rust
let _x = 5;
let x = &_x;
```

That is, the `5` here will be stored on the stack, or possibly in registers.
`x` will be a reference to it.

However, given that it's a literal integer, there's no reason that it *has*
to be local like this. Imagine we had a function that took a `'static` argument,
like `std::thread::spawn`. You might use `x` like this:


```rust
use std::thread;

fn main() {
    let x = &5;

    thread::spawn(move || {
        println!("{}", x);
    });

}
```

In previous versions of Rust, this would fail to compile:

```
error[E0597]: borrowed value does not live long enough
  --> src/main.rs:4:14
   |
4  |     let x = &5;
   |              ^ does not live long enough
...
10 | }
   | - temporary value only lives until here
   |
   = note: borrowed value must be valid for the static lifetime...
```

Because the `5` is local, so is its borrow, which doesn't satisfy the
requirements for `spawn`.

However, if you compile this on Rust 1.21, it will work. Why? Well,
if the thing being referred to is okay to put into a `static`, we could
instead de-sugar `let x = &5;` like this:

```rust
static FIVE: i32 = 5;

let x = &FIVE;
```

Here, since the `FIVE` is `static`, `x` is a `&'static i32`. And so this
is what Rust will now do in this kind of case. For full details, see [RFC 1414],
which was accepted in January, but started in December of 2015!

[RFC 1414]: https://github.com/rust-lang/rfcs/blob/master/text/1414-rvalue_static_promotion.md

We [now run LLVM in parallel while generating
code](https://github.com/rust-lang/rust/pull/43506), which should reduce peak
memory usage.

The [RLS](https://github.com/rust-lang-nursery/rls/) can now be installed
[through rustup](https://github.com/rust-lang/rust/pull/44204) by invoking
`rustup component add rls-preview`. In general, many useful Rust developer
tools such as the RLS, Clippy, and `rustfmt` need nightly Rust; this is the
first steps toward having them work on stable Rust. Please check out the
preview, and you'll hear more about these plans in the future.

Finally, a few documentation improvements. First up, if you visit [the docs
for `std::os`](https://doc.rust-lang.org/stable/std/os/), which contains
operating system specific functionality, you'll now see more than just `linux`,
the platform we build the documentation on. We've long regretted that the hosted
version of the documentation has been Linux-specific; this is a first step towards
rectifying that. This is [specific to the standard
library](https://github.com/rust-lang/rust/pull/43348) and not for general use;
we hope to improve this further in the future.

Next, [Cargo's docs are moving!](https://github.com/rust-lang/rust/pull/43916)
Historically, Cargo's docs were hosted on doc.crates.io, which doesn't follow
the release train model, even though Cargo itself does. This led to situations
where a feature would land in Cargo nightly, the docs would be updated, and
then for up to twelve weeks, users would *think* that it should work, but it
wouldn't yet. [https://doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo)
will be the new home of Cargo's docs, though for now, that URL is a redirect to
doc.crates.io. Future releases will move Cargo's docs over, and at that point,
doc.crates.io will redirect to doc.rust-lang.org/cargo. Cargo's docs have long
needed a refreshing, so expect to hear more news about Cargo's docs generally
in the future!

Finally, until now, `rustdoc` did not have any documentation. This is now
[fixed](https://github.com/rust-lang/rust/pull/43863), with a new "`rustdoc`
Book," located at
[https://doc.rust-lang.org/rustdoc](https://doc.rust-lang.org/rustdoc). These
docs are fairly bare-bones at the moment, but we'll be improving them over
time.

See the [detailed release notes][notes] for more.

#### Library stabilizations

Not too many stabilizations this release, but there's one really great
quality of life change: due to the lack of type-level integers, arrays only
supported various traits up to size 32. This [has now been fixed for the
`Clone` trait](https://github.com/rust-lang/rust/pull/43690), which also
caused a lot of ICEs at times, when a type would be `Copy` but not `Clone`.
For other traits, [an RFC for type-level integers was accepted
recently](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md),
which may help with this situation. That change has yet to be implemented, however,
though pre-requisite work is ongoing at the moment.

Next, [`Iterator::for_each`](https://github.com/rust-lang/rust/pull/44567) has
been stabilized, letting you consume an iterator for side effects without needing
a `for` loop:

```rust
// old
for i in 0..10 {
    println!("{}", i);
}

// new
(0..10).for_each(|i| println!("{}", i));
```

The correct one to use depends on your situation; in the sample above, the `for` loop
is pretty straightforward. But when you're chaining a number of iterators together,
the `for_each` version is sometimes clearer. Consider this:

```rust
// old
for i in (0..100).map(|x| x + 1).filter(|x| x % 2 == 0) {
    println!("{}", i);
}

// new
(0..100)
    .map(|x| x + 1)
    .filter(|x| x % 2 == 0)
    .for_each(|i| println!("{}", i));
```

[`Rc<T>` and `Arc<T>` now implement `From<&[T]> where T: Clone`, `From<str>`,
`From<String>`, `From<Box<T>> where T: ?Sized`, and
`From<Vec<T>>`.](https://github.com/rust-lang/rust/pull/42565)

The [`max` and `min` functions on the `Ord`
trait](https://github.com/rust-lang/rust/pull/44593) are now stable.

The [`needs_drop` intrinsic](https://github.com/rust-lang/rust/pull/44639)
is now stable.

Finally, [`std::mem::discriminant` has been
stabilized](https://doc.rust-lang.org/std/mem/fn.discriminant.html), allowing
you to see what variant an `enum` instance is without a `match` statement.

See the [detailed release notes][notes] for more.

#### Cargo features

Beyond the documentation features listed above, Cargo is gaining one major
feature in this release:
[`[patch]`](https://github.com/rust-lang/cargo/pull/4123). Designed in [RFC
1969](https://github.com/rust-lang/rfcs/blob/master/text/1969-cargo-prepublish.md),
the `[patch]` section of your `Cargo.toml` can be used when you want to
override certain parts of your dependency graph. We also have a feature,
`[replace]` that has similar functionality. In many ways, `[patch]` is the new
`[replace]`, and while we have no plans to deprecate or remove `[replace]`,
at this point, you should use `[patch]` instead of `[replace]`.

So what's it look like? Let's say we have a `Cargo.toml` that looks like this:

```toml
[dependencies]
foo = "1.2.3"
```

In addition, our `foo` crate depends on a `bar` crate, and we find a bug in
`bar`. To test this out, we'd download the source code for `bar`, and then
update our `Cargo.toml`:

```toml
[dependencies]
foo = "1.2.3"

[patch.crates-io]
bar = { path = '/path/to/bar' }
```

Now, when you `cargo build`, it will use the local version of `bar`, rather
than the one from `crates.io` that `foo` depends on.

For more details, see the
[documentation](http://doc.crates.io/manifest.html#the-patch-section).

Additionally:

* [you can now `cargo install` multiple crates at
  once](https://github.com/rust-lang/cargo/pull/4216)
* [If you're in a virtual workspace, `--all` is now
  applied automatically](https://github.com/rust-lang/cargo/pull/4335).
* [`include` and `exclude` fields in your `Cargo.toml` accepts patterns similar
  to a `.gitignore`](https://github.com/rust-lang/cargo/pull/4270).

See the [detailed release notes][notes] for more.

### Contributors to 1.21.0

Many people came together to create Rust 1.21. We couldn't have done it without
all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.21.0)
