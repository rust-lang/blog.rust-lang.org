+++
path = "2017/02/09/Rust-1.15.1"
title = "Announcing Rust 1.15.1"
authors = ["The Rust Core Team"]
aliases = [
    "2017/02/09/Rust-1.15.1.html",
    "releases/1.15.1",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.15.1. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.15.1 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [download Rust][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.15.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1151-2017-02-09

### What's in 1.15.1 stable

This release fixes two issues, a soundness bug in the new
`vec::IntoIter::as_mut_slice` method, and a regression wherein certain C
components of the Rust distribution were [not compiled with `-fPIC`][fpic].  The
latter results in the text section of executables being writable in some
configurations, including common Linux configurations, subverting an important
attack mitigation, and creating longer startup times by causing the linker to do
more work. For mostly-Rust codebases, the practical impact of losing read-only
text sections is relatively small (since Rust's type system is its first line of
defense), but for Rust linked into other codebases the impact could be
unexpectedly quite significant. [PIC] issues are well understood and not
Rust-specific, so the rest of this post focuses on the soundness bug.

[fpic]: https://github.com/rust-lang/rust/pull/39523
[PIC]: https://en.wikipedia.org/wiki/Position-independent_code

The problem with `as_mut_slice`, a three line function, was [discovered] just
minutes after publishing Rust 1.15.0, and is a reminder of the perils of writing
unsafe code.

[discovered]: https://www.reddit.com/r/rust/comments/5roiq7/announcing_rust_115/dd8vujs/

`as_mut_slice` is a method on the `IntoIter` iterator for the `Vec` type that
offers a mutable view into the buffer being iterated over. Conceptually it is
simple: just return a reference to the buffer; and indeed the implementation is
simple, but it's unsafe because `IntoIter` is implemented with an unsafe pointer
to the buffer:

```rust
pub fn as_mut_slice(&self) -> &mut [T] {
    unsafe {
        slice::from_raw_parts_mut(self.ptr as *mut T, self.len())
    }
}
```

It's just about the simplest unsafe method one could ask for. Can you spot the
error? Our reviewers didn't! This API slipped through the cracks because it is
such a standard and small one. It's a copy-paste bug that the reviewers glossed
over. This method takes a shared reference and unsafely derives from it a
mutable reference. That is totally bogus because it means `as_mut_slice` can be
used to produce multiple mutable references to the same buffer, which is the one
single thing you must not do in Rust.

Here's a simple example of what this bug would let you write, incorrectly:

```rust
fn main() {
    let v = vec![0];
    let v_iter = v.into_iter();
    let slice1: &mut [_] = v_iter.as_mut_slice();
    let slice2: &mut [_] = v_iter.as_mut_slice();
    slice1[0] = 1;
    slice2[0] = 2;
}
```

Here both `slice1` and `slice2` are referencing the same mutable slice. Also
notice that the iterator they are created from, `v_iter` is not declared
mutable, which is a good indication something fishy is going on.

The [solution] here is trivial, just change `&self` to `&mut self`:

```rust
pub fn as_mut_slice(&mut self) -> &mut [T] {
    unsafe {
        slice::from_raw_parts_mut(self.ptr as *mut T, self.len())
    }
}
```

[solution]: https://github.com/rust-lang/rust/pull/39466

With that, proper uniqueness invariants are maintained, only one mutable slice
can be created at a time, and `v_iter` must be declared mutable in order to pull
out a mutable slice.

So we made that change, and we're releasing a fix. In Rust we take pride in not
breaking APIs, but since this is a new, minor feature, and the present
implementation is spectacularly unsound, we decided to go ahead and release the
fix immediately, hopefully before too many codebases pick it up — that is, we
don't consider this a breaking change that requires a careful transition, but a
necessary bug fix. For more about Rust's approach to ensuring stability see the
["Stability as a Deliverable"][stab] blog post, [RFC 1122], on language
evolution, and [RFC 1105], on library evolution.

[stab]: https://blog.rust-lang.org/2014/10/30/Stability.html
[RFC 1122]: https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md
[RFC 1105]: https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md

We're still evaluating what to learn from this, but this is a good reminder of
the care that must be taken when writing unsafe code. We have some ideas for how
to catch this kind of problem earlier in the development process, but haven't
made any decisions yet.

We apologize for the inconvenience. Let's go hack.

### Contributors to 1.15.1

We had 2 individuals contribute to Rust 1.15.1.
[Thanks!](https://thanks.rust-lang.org/rust/1.15.1)
