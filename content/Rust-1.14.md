+++
path = "2016/12/22/Rust-1.14"
title = "Announcing Rust 1.14"
authors = ["The Rust Core Team"]
aliases = [
    "2016/12/22/Rust-1.14.html",
    "releases/1.14.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.14.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

As always, you can [install Rust 1.14.0][install] from the appropriate page on our
website, and check out the [detailed release notes for 1.14.0][notes] on GitHub.
1230 patches were landed in this release.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1140-2016-12-22

### What's in 1.14.0 stable

One of the biggest features in Rust 1.14 isn't actually in the language or
compiler: the [rustup tool has reached a 1.0 release][rustup], and is now the
recommended way to install Rust from the project directly. Rustup does a bit
more than just install Rust:

> rustup installs The Rust Programming Language from the official release
> channels, enabling you to easily switch between stable, beta, and nightly
> compilers and keep them updated. It makes cross-compiling simpler with binary
> builds of the standard library for common platforms. And it runs on all
> platforms Rust supports, including Windows.

[rustup]: https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/203

We had [a previous post about Rustup][prev] back in May. You can learn more
about it there, or by checking it out [on
GitHub](https://github.com/rust-lang-nursery/rustup.rs).

[prev]: https://blog.rust-lang.org/2016/05/13/rustup.html

Another exciting feature is [experimental support for WebAssembly][wasm] as a
target, `wasm32-unknown-emscripten`. It is still early days, and there's a lot
of bugs to shake out, so please give it a try and report them! To give you a
small taste of how it works, once you have [emscripten] installed, compiling
some Rust code to WebAssembly is as easy as:

```bash
$ rustup target add wasm32-unknown-emscripten
$ echo 'fn main() { println!("Hello, Emscripten!"); }' > hello.rs
$ rustc --target=wasm32-unknown-emscripten hello.rs
$ node hello.js
```

[wasm]: https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627
[emscripten]: https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html

The community has been doing interesting, experimental work in this area: see
[Jan-Erik's slides] for the workshop he ran at [Rust Belt Rust] for some
examples, or check out [Tim's example of the classic TodoMVC project][todomvc].
This implementation builds off of his [webplatform
crate](https://crates.io/crates/webplatform), which exposes the DOM to Rust.

[Jan-Erik's slides]: https://www.hellorust.com/emscripten/
[Rust Belt Rust]: https://www.rust-belt-rust.com/sessions/
[todomvc]: https://github.com/rust-webplatform/rust-todomvc

Speaking of platforms, a large number of platforms have gained additional
support:

For `rustc`:

* `mips-unknown-linux-gnu`
* `mipsel-unknown-linux-gnu`
* `mips64-unknown-linux-gnuabi64`
* `mips64el-unknown-linux-gnuabi64`
* `powerpc-unknown-linux-gnu`
* `powerpc64-unknown-linux-gnu`
* `powerpc64le-unknown-linux-gnu`
* `s390x-unknown-linux-gnu`

And for `std`:

* `arm-unknown-linux-musleabi`
* `arm-unknown-linux-musleabihf`
* `armv7-unknown-linux-musleabihf`

If you're using one of these platforms, follow the instructions on the website
to install, or add the targets to an existing installation with
`rustup target add`.

These platforms are all 'tier 2', please see our page on [platform support] for
more details.

[platform support]: https://forge.rust-lang.org/platform-support.html

Just like how the community is doing interesting work on the WebAssembly
target, there's also neat things going on with increasing Rust's target support
beyond what's listed above. [xargo] allows for easy cross-compilation of Rust
to bare-metal targets. If you're writing an operating system in Rust, or doing
something interesting on a microcontroller, xargo can make your life a lot
simpler.

[xargo]: https://github.com/japaric/xargo


The landing of MIR over the last few releases means that a [number of
improvements to compile times] have landed, with more coming in the future.

[number of improvements to compile times]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#compile-time-optimizations

In the language, one small improvement has landed: support for [RFC 1492]. This small
addition lets you use `..` in more places. Previously, say you had a struct like this:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
```

In any context where you're doing a pattern match, you could use `..` to ignore the
parts you don't care about. For example:


```rust
let p = Point { x: 0, y: 1, z: 2 };

match p {
    Point { x, .. } => println!("x is {}", x),
}
```

The `..` ignores `y` and `z`.

Consider a similar `Point`, but as a tuple struct:

```rust
struct Point(i32, i32, i32);
```

Previously, you could use `..` to ignore all three elements:

```rust
let p = Point(0, 1, 2);

match p {
    Point(..) => println!("found a point"),
}
```

But you could not use it to only ignore parts of the tuple:

```rust
let p = Point(0, 1, 2);

match p {
    Point(x, ..) => println!("x is {}", x),
}
```

This was an inconsistency, and so with RFC 1492 stabilized, compiles fine as of
this release. This applies to more situations than tuples; please see [the
RFC][RFC 1492] for more details.

[RFC 1492]: https://github.com/rust-lang/rfcs/blob/master/text/1492-dotdot-in-patterns.md

#### Library stabilizations

There have been a number of additions to the standard library, but they don't
fit into particularly nice categories for this release. Here's the highlights:

* [`println!()`, with no arguments, prints newline][36825].
  Previously, an empty string was required to achieve the same.
* [`Wrapping` impls standard binary and unary operators on references, as well
  as the `Sum` and `Product` iterators][37356], making references to these
  types easier to use.
* [Implement `From<Cow<str>> for String` and `From<Cow<[T]>> for
  Vec<T>`][37326]. These implementations make sense, but were not yet added.
* [Expand `.zip()` specialization to `.map()` and `.cloned()`][37230] for improved performance.
* [Implement `RefUnwindSafe` for atomic types][37178], as these types are
  "unwind safe," though that wasn't obvious at first.
* [Specialize `Vec::extend` to `Vec::extend_from_slice`][37094] for performance gains.
* [Don't reuse `HashMap` random seeds][37470]. This helps to mitigate one type
  of DDoS attack.
* [The internal memory layout of `HashMap` is more cache-friendly, for
  significant improvements in some operations][36692]
* [Impl `Add<{str, Cow<str>}>` for `Cow<str>`][36430]. We already support `Add`
  for other string types, so not having it on `Cow` is inconsistent.

[36825]: https://github.com/rust-lang/rust/issues/36825
[37356]: https://github.com/rust-lang/rust/issues/37356
[37326]: https://github.com/rust-lang/rust/issues/37326
[37230]: https://github.com/rust-lang/rust/issues/37230
[37178]: https://github.com/rust-lang/rust/issues/37178
[37094]: https://github.com/rust-lang/rust/issues/37094
[37470]: https://github.com/rust-lang/rust/issues/37470
[36692]: https://github.com/rust-lang/rust/issues/36692
[36430]: https://github.com/rust-lang/rust/issues/36430

See the [detailed release notes][notes] for more.

#### Cargo features

As for Cargo, [RFC 1721] has been implemented. Cargo will now pass along the
values printed by `rustc --print cfg` to build scripts. The motivation for this
feature is that Cargo can now compile objects for statically linking against
the msvcrt on the MSVC platform.

[RFC 1721]: https://github.com/rust-lang/rfcs/blob/master/text/1721-crt-static.md

Cargo now works properly [with a read-only `CARGO_HOME`][3259].

Finally, Cargo will [ignore the `panic` configuration for the `test` and
`bench` profiles][3175]. This is important because the test runner relies on
panics counting as failing tests, and so with `panic=abort`, a failing test
would abort the entire test suite.

[3259]: https://github.com/rust-lang/cargo/issues/3259
[3175]: https://github.com/rust-lang/cargo/issues/3175

See the [detailed release notes][notes] for more.

### Contributors to 1.14.0

We had 144 individuals contribute to 1.14.0. Thank you so much!

* Abhishek Chanda
* Adam Perry
* Ahmed Charles
* Aidan Hobson Sayers
* Aleksey Kladov
* Alexander von Gluck IV
* Alex Burka
* Alex Crichton
* Alex von Gluck IV
* Amanieu d'Antras
* Andrea Corradi
* Andrea Pretto
* Andreas Sommer
* Andre Bogus
* Andrew Paseltiner
* angelsl
* Anthony Ramine
* Ariel Ben-Yehuda
* arthurprs
* Austin Hicks
* bors
* Brian Anderson
* Bunts Thy Unholy
* CensoredUsername
* Chris McDonald
* Christopher
* christopherdumas
* Christopher Serr
* Cobrand
* Corey Farwell
* Cristi Cobzarenco
* Daan Sprenkels
* Danny Hua
* David Henningsson
* Devon Hollowood
* Dmitry Gritsay
* Dominik Inführ
* Duncan
* Eduard Burtescu
* Eduard-Mihai Burtescu
* Eric Roshan-Eisner
* est31
* Fabian Frei
* Federico Mena Quintero
* Felix S. Klock II
* Florian Diebold
* Florian Hartwig
* Florian Zeitz
* Frank Rehberger
* Gavin Baker
* Geoffry Song
* Guillaume Gomez
* iirelu
* James Miller
* Jan-Erik Rediger
* Jared Roesch
* Jeffrey Seyfried
* Jesus Garlea
* Jethro Beekman
* Joe Neeman
* Johannes Muenzel
* John Firebaugh
* John Hodge
* johnthagen
* Jonas Schievink
* Jonathan Turner
* Jorge Aparicio
* Josh Stone
* Josh Triplett
* Keegan McAllister
* Keith Yeung
* KillTheMule
* Konrad Borowski
* leonardo.yvens
* Liigo Zhuang
* loggerhead
* Manish Goregaokar
* Marcin Fatyga
* Mark-Simulacrum
* Martin Glagla
* Martin Thoresen
* Mathieu Borderé
* Mathieu Poumeyrol
* Matt Brubeck
* Matthew Piziak
* Matwey V. Kornilov
* mcarton
* Michael Woerister
* Mikhail Modin
* Mikko Rantanen
* msiglreith
* Nabeel Omer
* Nathan Musoke
* Nicholas Nethercote
* Nick Cameron
* Nick Fitzgerald
* Nick Stevens
* Nikhil Shagrithaya
* Niko Matsakis
* Oliver Middleton
* p512
* ParkHanbum
* Paul Lange
* Paulo Matos
* Paul Osborne
* Peter Atashian
* Peter N
* Philip Davis
* Pieter Frenssen
* Pweaver (Paul Weaver)
* pweyck
* QuietMisdreavus
* Raph Levien
* Razican
* Robin Stocker
* Ross Schulman
* Ryan Senior
* Scott Olson
* Seo Sanghyeon
* Simonas Kazlauskas
* Simon Sapin
* Srinivas Reddy Thatiparthy
* Stefan Schindler
* Stephen M. Coakley
* Steve Klabnik
* Steven Fackler
* Tamir Duberstein
* Taylor Cramer
* Tim Neumann
* Tobias Bucher
* Tomasz Miąsko
* tormol
* Tshepang Lekhonkhobe
* Ulrik Sverdrup
* Vadim Chugunov
* Vadim Petrochenkov
* Vadzim Dambrouski
* Vangelis Katsikaros
* Wang Xuerui
* Wesley Wiser
* Zack M. Davis
* Zoffix Znet
* Артём Павлов [Artyom Pavlov]
* 石博文
