+++
path = "2016/11/10/Rust-1.13"
title = "Announcing Rust 1.13"
authors = ["The Rust Core Team"]
aliases = ["2016/11/10/Rust-1.13.html"]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.13.0. Rust is
a systems programming language focused on safety, speed, and concurrency.

As always, you can [install Rust 1.13.0][install] from the appropriate page on
our website, and check out the [detailed release notes for 1.13.0][notes] on
GitHub. 1448 patches were landed in this release.

[install]: https://www.rust-lang.org/downloads.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1130-2016-11-10

It's been a busy season in Rust. We enjoyed three Rust conferences, [RustConf],
[RustFest], and [Rust Belt Rust], in short succession. It was great to see so
many Rustaceans in person, some for the first time! We've been [thinking a lot]
about the future, developing a [roadmap for 2017], and [building the tools] our
users [tell us] they need.

And even with all that going on, we put together a new release filled with fun
new toys.

[RustConf]: https://rustconf.com/
[RustFest]: https://www.rustfest.eu/
[Rust Belt Rust]: https://www.rust-belt-rust.com/
[thinking a lot]: https://internals.rust-lang.org/t/setting-our-vision-for-the-2017-cycle/3958/47
[roadmap for 2017]: https://github.com/rust-lang/rfcs/pull/1774
[building the tools]: https://internals.rust-lang.org/t/introducing-rust-language-server-source-release/4209
[tell us]: https://internals.rust-lang.org/t/2016-rust-commercial-user-survey-results/4317

### What's in 1.13 stable

The 1.13 release includes several extensions to the language, including the
long-awaited `?` operator, improvements to compile times, minor feature
additions to cargo and the standard library. This release also includes many
small enhancements to documentation and error reporting, by many contributors,
that are not individually mentioned in the release notes.

This release contains important security updates to Cargo, which depends on curl
and OpenSSL, which both published security updates recently. For more
information see the respective announcements for [curl 7.51.0] and [OpenSSL
1.0.2j].

[curl 7.51.0]: https://curl.haxx.se/changes.html
[OpenSSL 1.0.2j]: https://www.openssl.org/news/secadv/20160922.txt

#### The `?` operator

Rust has gained a new operator, `?`, that makes error handling more pleasant by
reducing the visual noise involved. It does this by solving one simple
problem. To illustrate, imagine we had some code to read some data from a file:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

This code has two paths that can fail, opening the file and reading the data
from it. If either of these fail to work, we'd like to return an error from
`read_username_from_file`. Doing so involves `match`ing on the result of the I/O
operations. In simple cases like this though, where we are only propagating
errors up the call stack, the matching is just boilerplate - seeing it written
out, in the same pattern every time, doesn't provide the reader with a great
deal of useful information.

With `?`, the above code looks like this:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
```

The `?` is shorthand for the entire match statements we wrote earlier. In other
words, `?` applies to a `Result` value, and if it was an `Ok`, it unwraps it and
gives the inner value. If it was an `Err`, it returns from the function you're
currently in.  Visually, it is much more straightforward. Instead of an entire
match statement, now we are just using the single "?" character to indicate that
here we are handling errors in the standard way, by passing them up the
call stack.

Seasoned Rustaceans may recognize that this is the same as the `try!` macro
that's been available since Rust `1.0`. And indeed, they are the same. Before
1.13, `read_username_from_file` could have been implemented like this:

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = try!(File::open("username.txt"));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));

    Ok(s)
}
```

So why extend the language when we already have a macro? There are multiple
reasons. First, `try!` has proved to be extremely useful, and is used often in
idiomatic Rust. It is used so often that we think it's worth having a sweet
syntax. This sort of evolution is one of the great advantages of a powerful
macro system: speculative extensions to the language syntax can be prototyped
and iterated on without modifying the language itself, and in return, macros that
turn out to be especially useful can indicate missing language features. This
evolution, from `try!` to `?` is a great example.

One of the reasons `try!` needs a sweeter syntax is that it is quite
unattractive when multiple invocations of `try!` are used in
succession. Consider:

```rust
try!(try!(try!(foo()).bar()).baz())
```

as opposed to

```rust
foo()?.bar()?.baz()?
```

The first is quite difficult to scan visually, and each layer of error handling
prefixes the expression with an additional call to `try!`. This brings undue
attention to the trivial error propagation, obscuring the main code path, in
this example the calls to `foo`, `bar` and `baz`. This sort of method chaining
with error handling occurs in situations like the builder pattern.

Finally, the dedicated syntax will make it easier in the future to produce nicer
error messages tailored specifically to `?`, whereas it is difficult to produce
nice errors for macro-expanded code generally (in this release, though, the `?`
error messages could use improvement).

Though this is a small feature, in our experience so far, `?` feels like a solid
ergonomic improvement to the old `try!` macro. This is a good example of the
kinds of incremental, quality-of-life improvements Rust will continue to
receive, polishing off the rough corners of our already-powerful base language.

Read more about `?` in [RFC 243].

#### Performance improvements

There has been a lot of focus on compiler performance lately. There's good news
in this release, and more to come.

Mark Simulacrum and Nick Cameron have been refining [perf.rust-lang.org], our
tool for tracking compiler performance. It runs the [rustc-benchmarks] suite
regularly, on dedicated hardware, and tracks the results over time. This tool
records the results for each pass in the compiler and is used by the compiler
developers to narrow commit ranges of performance regressions. It's an important
part of our toolbox!

We can use this tool to look at a [graph] of performance over the 1.13
development cycle, shown below. This cycle covered the dates from August 16
through September 29 (the graph begins from Augest 25th though and is filtered
in a few ways to eliminate bogus, incomplete, or confusing results). There
appear to be some big reductions, which are quantified on the corresponding
[statistics] page.

<br/>

![Performance graph](graph.png)

<br/>

The big improvement demonstrated in the graphs, on September 1, is from an
optimization from Niko to [cache normalized projections during
translation][cache]. That is to say, during generation of LLVM IR, the compiler
no longer recomputes concrete instances of associated types each time they are
needed, but instead reuses previously-computed values. This optimization doesn't
affect all code bases, but in code bases that exhibit certain patterns, like
[futures-rs], where [debug mode build-time improved by up to 40%][ev1], you'll notice
the difference.

Another such optimization, that doesn't affect every crate but does affect some
in a big way, came from Michael Woerister, and improves compile time for crates
that export many [inline] functions. When a function is marked `#[inline]`, in
addition to translating that function for use by the current crate, the compiler
stores its MIR representation in the crate rlib, and translates the function to
LLVM IR in every crate that calls it. The optimization Michael did is obvious in
retrospect: there are some cases where inline functions are only for the
consumption of other crates, and never called from the crate in which they are
defined; so the compiler doesn't need to translate code for inline functions in
the crate they are defined _unless_ they are called directly. This saves the
cost of rustc converting the function to LLVM IR and LLVM optimizing and
converting the function to machine code.

In some cases this results in dramatic improvements. Build times for the ndarray
crate [improved by 50%][ev2], and in the (unreleased) [winapi 0.3] crate, rustc
now emits no machine code at all.

But wait, there's more still! Nick Nethercote has [turned his focus to compiler
performance as well][speed], focusing on profiling and micro-optimizations. This
release contains [several fruits of his work][fruit], and there are more in the
pipeline for 1.14.

[fruit]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#compile-time-optimizations
[speed]: https://blog.mozilla.org/nnethercote/2016/10/14/how-to-speed-up-the-rust-compiler
[winapi 0.3]: https://github.com/retep998/winapi-rs
[ev1]: https://github.com/rust-lang/rust/pull/37600#issuecomment-258696690
[ev2]: https://github.com/rust-lang/rust/pull/37600#issuecomment-258706020
[futures-rs]: https://github.com/alexcrichton/futures-rs
[cache]: https://github.com/rust-lang/rust/pull/35761
[graph]: https://goo.gl/6T69T2
[statistics]: https://goo.gl/CLIAhi
[perf.rust-lang.org]: https://perf.rust-lang.org
[rustc-benchmarks]: https://github.com/rust-lang-nursery/rustc-benchmarks
[inline]: https://github.com/rust-lang/rust/pull/36524

### Other notable changes

This release contains important security updates to Cargo, which depends on curl
and OpenSSL, which both published security updates recently. For more
information see the respective announcements for [curl 7.51.0] and [OpenSSL
1.0.2j].

[curl 7.51.0]: https://curl.haxx.se/changes.html
[OpenSSL 1.0.2j]: https://www.openssl.org/news/secadv/20160922.txt

Macros can now be used in type position ([RFC 873]), and
attributes can be applied to statements ([RFC 16]):

```rust
// Use a macro to name a type
macro_rules! Tuple {
    { $A:ty,$B:ty } => { ($A, $B) }
}

let x: Tuple!(i32, i32) = (1, 2);
```

```rust
// Apply a lint attribute to a single statement
#[allow(non_snake_case)]
let BAD_STYLE = List::new();
```

Inline drop flags have been removed. Previously, in case of a conditional move,
the compiler would store a "drop flag" inline in a struct (increasing its size)
to keep track of whether or not it needs to be dropped. This means that some
structs take up some unexpected extra space, which interfered with things like
passing types with destructors over FFI. It also was a waste of space for
code that didn't have conditional moves. In 1.12,
[MIR became the default][1.12], which laid the groundwork for many improvements,
including [getting rid of these inline drop flags][35764]. Now, drop flags are
stored in an extra slot on the stack frames of functions that need them.

1.13 contains a [serious bug in code generation][arm] for ARM targets using
hardware floats (which is most ARM targets). ARM targets in Rust are presently
in our 2nd support tier, so this bug was not determined to block the
release. Because 1.13 contains a security update, users that must target ARM are
encouraged to use the 1.14 betas, which will soon get a fix for ARM.

[arm]: https://github.com/rust-lang/rust/issues/37630

#### Language stabilizations

* The [`Reflect`] trait is deprecated. See the [explanation] of what this means
  for parametricity in Rust.
* [Stabilize macros in type position][36014]. [RFC 873].
* [Stabilize attributes on statements][36995]. [RFC 16].

#### Library stabilizations

* [`checked_abs`], [`wrapping_abs`], and [`overflowing_abs`]
* [`RefCell::try_borrow`], and [`RefCell::try_borrow_mut`]
* [Add `assert_ne!` and `debug_assert_ne!`][35074]
* [Implement `AsRef<[T]>` for `std::slice::Iter`][35559]
* [Implement `CoerceUnsized` for `{Cell, RefCell, UnsafeCell}`][35627]
* [Implement `Debug` for `std::path::{Components,Iter}`][36101]
* [Implement conversion traits for `char`][35755]
* [`SipHasher`] is deprecated. Use [`DefaultHasher`].
* [Implement more traits for `std::io::ErrorKind`][35911]

#### Cargo features

* [cargo: Add --all-features flag to cargo][cargo/3038]
* [cargo: Add --message-format flag][cargo/3000]

[cargo/3000]: https://github.com/rust-lang/cargo/pull/3000
[cargo/3038]: https://github.com/rust-lang/cargo/pull/3038
[`checked_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.checked_abs
[`wrapping_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_abs
[`overflowing_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_abs
[`RefCell::try_borrow`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow
[`RefCell::try_borrow_mut`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow_mut
[`SipHasher`]: https://doc.rust-lang.org/std/hash/struct.SipHasher.html
[`DefaultHasher`]: https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html
[35074]: https://github.com/rust-lang/rust/pull/35074
[35559]: https://github.com/rust-lang/rust/pull/35559
[35627]: https://github.com/rust-lang/rust/pull/35627
[35755]: https://github.com/rust-lang/rust/pull/35755
[35911]: https://github.com/rust-lang/rust/pull/35911
[36014]: https://github.com/rust-lang/rust/pull/36014
[36995]: https://github.com/rust-lang/rust/pull/36995
[36101]: https://github.com/rust-lang/rust/pull/36101
[35764]: https://github.com/rust-lang/rust/pull/35764
[`Reflect`]: https://doc.rust-lang.org/std/marker/trait.Reflect.html
[explanation]: https://github.com/rust-lang/rust/issues/27749#issuecomment-244489589
[RFC 16]: https://github.com/rust-lang/rfcs/blob/master/text/0016-more-attributes.md
[RFC 873]: https://github.com/rust-lang/rfcs/blob/master/text/0873-type-macros.md
[RFC 243]: https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md
[1.12]: https://blog.rust-lang.org/2016/09/29/Rust-1.12.html

See the [detailed release notes][notes] for more.

### Contributors to 1.13.0

We had 155 individuals contribute to 1.13.0. Thank you so much!

* Aaron Gallagher
* Abhishek Kumar
* aclarry
* Adam Medziński
* Ahmed Charles
* Aleksey Kladov
* Alexander von Gluck IV
* Alexandre Oliveira
* Alex Burka
* Alex Crichton
* Amanieu d'Antras
* Amit Levy
* Andrea Corradi
* Andre Bogus
* Andrew Cann
* Andrew Cantino
* Andrew Lygin
* Andrew Paseltiner
* Andy Russell
* Ariel Ben-Yehuda
* arthurprs
* Ashley Williams
* athulappadan
* Austin Hicks
* bors
* Brian Anderson
* c4rlo
* Caleb Jones
* CensoredUsername
* cgswords
* changchun.fan
* Chiu-Hsiang Hsu
* Chris Stankus
* Christopher Serr
* Chris Wong
* clementmiao
* Cobrand
* Corey Farwell
* Cristi Cobzarenco
* crypto-universe
* dangcheng
* Daniele Baracchi
* DarkEld3r
* David Tolnay
* Dustin Bensing
* Eduard Burtescu
* Eduard-Mihai Burtescu
* Eitan Adler
* Erik Uggeldahl
* Esteban Küber
* Eugene Bulkin
* Eugene R Gonzalez
* Fabian Zaiser
* Federico Ravasio
* Felix S. Klock II
* Florian Gilcher
* Gavin Baker
* Georg Brandl
* ggomez
* Gianni Ciccarelli
* Guillaume Gomez
* Jacob
* jacobpadkins
* Jake Goldsborough
* Jake Goulding
* Jakob Demler
* James Duley
* James Miller
* Jared Roesch
* Jared Wyles
* Jeffrey Seyfried
* JessRudder
* Joe Neeman
* Johannes Löthberg
* John Firebaugh
* johnthagen
* Jonas Schievink
* Jonathan Turner
* Jorge Aparicio
* Joseph Dunne
* Josh Triplett
* Justin LeFebvre
* Keegan McAllister
* Keith Yeung
* Keunhong Lee
* king6cong
* Knight
* knight42
* Kylo Ginsberg
* Liigo
* Manish Goregaokar
* Mark-Simulacrum
* Matthew Piziak
* Matt Ickstadt
* mcarton
* Michael Layne
* Michael Woerister
* Mikhail Modin
* Mohit Agarwal
* Nazım Can Altınova
* Neil Williams
* Nicholas Nethercote
* Nick Cameron
* Nick Platt
* Niels Sascha Reedijk
* Nikita Baksalyar
* Niko Matsakis
* Oliver Middleton
* Oliver Schneider
* orbea
* Panashe M. Fundira
* Patrick Walton
* Paul Fanelli
* philipp
* Phil Ruffwind
* Piotr Jawniak
* pliniker
* QuietMisdreavus
* Rahul Sharma
* Richard Janis Goldschmidt
* Scott A Carr
* Scott Olson
* Sean McArthur
* Sebastian Ullrich
* Sébastien Marie
* Seo Sanghyeon
* Sergio Benitez
* Shyam Sundar B
* silenuss
* Simonas Kazlauskas
* Simon Sapin
* Srinivas Reddy Thatiparthy
* Stefan Schindler
* Stephan Hügel
* Steve Klabnik
* Steven Allen
* Steven Fackler
* Terry Sun
* Thomas Garcia
* Tim Neumann
* Tobias Bucher
* Tomasz Miąsko
* trixnz
* Tshepang Lekhonkhobe
* Ulrich Weigand
* Ulrik Sverdrup
* Vadim Chugunov
* Vadim Petrochenkov
* Vanja Cosic
* Vincent Esche
* Wesley Wiser
* William Lee
* Ximin Luo
* Yossi Konstantinovsky
* zjhmale
