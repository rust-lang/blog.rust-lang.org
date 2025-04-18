+++
path = "2017/06/08/Rust-1.18"
title = "Announcing Rust 1.18"
authors = ["The Rust Core Team"]
aliases = [
    "2017/06/08/Rust-1.18.html",
    "releases/1.18.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.18.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.18 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.18.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1180-2017-06-08

### What's in 1.18.0 stable

As usual, Rust 1.18.0 is a collection of improvements, cleanups, and new
features.

One of the largest changes is a long time coming: core team members Carol
Nichols and Steve Klabnik have been writing a new edition of "The Rust
Programming Language", the official book about Rust. It's being [written openly
on GitHub](https://github.com/rust-lang/book), and has over a hundred
contributors in total. This release [includes the first draft of the second
edition in our online documentation](https://doc.rust-lang.org/stable/book/).
19 out of 20 chapters have a draft; the draft of chapter 20 will land in Rust
1.19. When the book is done, a print version will be made available through [No
Starch Press](https://www.nostarch.com/Rust), if you'd like a paper copy. We're
still working with the editors at No Starch to improve the text, but we wanted
to start getting a wider audience now.

The new edition is a complete re-write from the ground up, using the last two
years of knowledge we've gained from teaching people Rust. You'll find
brand-new explanations for a lot of Rust's core concepts, new projects to
build, and all kinds of other good stuff. Please check it out and [let us know
what you think](https://github.com/rust-lang/book/issues/new)!

As for the language itself, an old feature has learned some new tricks: the
`pub` keyword has been expanded a bit. Experienced Rustaceans will know that
items are private by default in Rust, and you can use the `pub` keyword to make
them public. In Rust 1.18.0, `pub` has [gained a new
form](https://github.com/rust-lang/rust/pull/40556):

```rust
pub(crate) bar;
```

The bit inside of `()` is a 'restriction', which refines the notion of how this
is made public. Using the `crate` keyword like the example above means that
`bar` would be public to the entire crate, but not outside of it. This makes it
easier to declare APIs that are "public to your crate", but not exposed to your
users. This was *possible* with the existing module system, but often very
awkward.

You can also specify a path, like this:

```rust
pub(in a::b::c) foo;
```

This means "usable within the hierarchy of `a::b::c`, but not elsewhere." This
feature was defined in [RFC
1422](https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md)
and [is documented in the
reference](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself).

For our Windows users, Rust 1.18.0 has [a new attribute,
`#![windows_subsystem]`](https://github.com/rust-lang/rust/pull/40870). It
works like this:

```rust
#![windows_subsystem = "console"]
#![windows_subsystem = "windows"]
```

These control the [`/SUBSYSTEM` flag](https://msdn.microsoft.com/en-us/library/fcc1zstk.aspx)
in the linker. For now, only `"console"` and `"windows"` are supported.

When is this useful? In the simplest terms, if you're developing a graphical
application, and do not specify `"windows"`, a console window would flash up upon
your application's start. With this flag, it won't.

Finally, Rust's tuples, enum variant fields, and structs (without `#[repr]`) have
always had an unspecified layout. [We've turned on automatic re-ordering](https://github.com/rust-lang/rust/pull/40377), which can result in smaller sizes
through reducing padding. Consider a struct like this:

```rust
struct Suboptimal(u8, u16, u8);
```

In previous versions of Rust on the x86_64 platform, this struct would have the
size of six bytes. But looking at the source, you'd expect it to have four. The
extra two bytes come from padding; given that we have a `u16` here, it should be
aligned to two bytes. But in this case, it's at offset one. To move it to offset
two, another byte of padding is placed after the first `u8`. To give the whole struct
a proper alignment, another byte is added after the second `u8` as well, giving us
`1 + 1 (padding) + 2 + 1 + 1 (padding) = 6 bytes`.

But what if our struct looked like this?

```rust
struct Optimal(u8, u8, u16);
```

This struct is properly aligned; the `u16` lies on a two byte boundary, and so
does the entire struct. No padding is needed. This gives us `1 + 1 + 2 = 4
bytes`.

When designing Rust, we left the details of memory layout undefined for just
this reason. Because we didn't commit to a particular layout, we can make
improvements to it, such as in this case where the compiler can optimize
`Suboptimal` into `Optimal` automatically. And if you check the sizes of
`Suboptimal` and `Optimal` on Rust 1.18.0, you'll see that they both have a
size of four bytes.

We've been planning this change for a while; previous versions of Rust included
this optimization on the nightly channel, but some people wrote unsafe code
that assumed the exact details of the representation. We rolled it back while
we fixed all instances of this that we know about, but if you find some code
breaks due to this, please let us know so we can help fix it! Structs used
for FFI can be given the `#[repr(C)]` annotation to prevent reordering, in
addition to C-compatible field layout.

We've been planning on moving `rustdoc` to use a CommonMark compliant markdown
parser for a long time now. However, just switching over can introduce
regressions where the CommonMark spec differs from our existing parser,
Hoedown. As part of the transition plan, [a new flag has been added to
`rustdoc`](https://github.com/rust-lang/rust/pull/40338),
`--enable-commonmark`. This will use the new parser instead of the old one.
Please give it a try! As far as we know, both parsers will produce identical
results, but we'd be interested in knowing if you find a scenario where the
rendered results differ!

Finally, compiling `rustc` itself is now [15%-20%
faster](https://github.com/rust-lang/rust/pull/41469). Each commit message in
this PR goes over the details; there were some inefficiencies, and now they've
been cleaned up.

See the [detailed release notes][notes] for more.

#### Library stabilizations

Seven new APIs were stabilized this release:

- [`Child::try_wait`] is a non-blocking form of `Child::wait`.
- [`HashMap::retain`] and  [`HashSet::retain`] bring the `retain` API `Vec<T>` has to these two hash data structures.
- [`PeekMut::pop`] lets you pop the top element from a `BinaryHeap<T>` after you've already peeked at it without needing to reorder the heap a second time.
- [`TcpStream::peek`], [`UdpSocket::peek`], [`UdpSocket::peek_from`] let you peek at a stream or socket.

[`Child::try_wait`]: https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait
[`HashMap::retain`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.retain
[`HashSet::retain`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.retain
[`PeekMut::pop`]: https://doc.rust-lang.org/std/collections/binary_heap/struct.PeekMut.html#method.pop
[`TcpStream::peek`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.peek
[`UdpSocket::peek_from`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek_from
[`UdpSocket::peek`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peek

See the [detailed release notes][notes] for more.

#### Cargo features

Cargo has [added support](https://github.com/rust-lang/cargo/pull/3842) for the Pijul VCS,
which is written in Rust. `cargo new my-awesome-project --vcs=pijul` will get you going!

To supplement the `--all` flag, Cargo now has [several new
flags](https://github.com/rust-lang/cargo/pull/3901) such as `--bins`,
`--examples`, `--tests`, and `--benches`, which will let you build all programs of
that type.

Finally, Cargo now supports [Haiku](https://github.com/rust-lang/cargo/pull/3952) and
[Android](https://github.com/rust-lang/cargo/pull/3885)!

See the [detailed release notes][notes] for more.

### Contributors to 1.18.0

Many people came together to create Rust 1.18. We couldn't have done it without
all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.18.0)
