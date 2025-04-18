+++
path = "2015/08/06/Rust-1.2"
title = "Announcing Rust 1.2"
authors = ["The Rust Core Team"]
aliases = [
    "2015/08/06/Rust-1.2.html",
    "releases/1.2.0",
]

[extra]
release = true
+++

Today marks the [completion][install] of the Rust 1.2 stable and 1.3 beta
release cycles! Read on for the highlight, or check the [release notes][notes]
for more detail.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-120-august-2015

### What's in 1.2 stable

As we
[previously announced](https://blog.rust-lang.org/2015/06/25/Rust-1.1.html), Rust
1.2 comes with two major performance improvements for the compiler:

- **An across-the-board improvement** to real-world compiler
  performance. Representative crates include [hyper][hyper] (compiles 1.16x
  faster), [html5ever][html5ever] (1.62x faster), [regex][regex] (1.32x faster)
  and [rust-encoding][rust-encoding] (1.35x faster). You can explore some of this
  performance data at Nick Cameron's
  [preliminary tracking site](https://www.ncameron.org/perf-rustc/), using dates
  2015-05-15 to 2015-06-25.

- **Parallel codegen** is
  [now working](https://github.com/rust-lang/rust/pull/26018), and produces a
  33% speedup when bootstrapping on a 4 core machine. Parallel codegen is
  particularly useful for debug builds, since it prevents some optimizations;
  but it can also be used with optimizations as an effective `-O1` flag. It can
  be activated by passing `-C codegen-units=N` to `rustc`, where `N` is the
  desired number of threads.

Cargo's performance has also improved dramatically:

- Builds that do not require any recompilation ("no-op builds") for large
  projects are much faster: for Servo, build time went from 5 seconds to 0.5
  seconds.

- Cargo now supports shared target directories that cache dependencies across
  multiple packages, which results in significant build-time reduction for
  complex projects.

The 1.2 release also
[introduces support](https://github.com/rust-lang/rust/pull/25350) for the MSVC
(Microsoft Visual C) toolchain, as opposed to GNU variants. The upshot is that
Rust code is now directly linkable against code built using the native Windows
toolchain. The compiler bootstraps on MSVC, we have preliminary nightlies, and
we are testing all rust-lang crates against MSVC. Unwinding support is not yet
available (the process aborts on panic), but work is underway to land it.

On the language side, Rust 1.2 marks the completion of the
[dynamically-sized type (DST)](https://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/)
work, allowing smart pointers like `Rc` to seamless apply to arrays and trait
objects, so that `Rc<[T]>` is fully usable. This final enhancement applies to
all smart pointers in the standard library. Support for external smart pointer
types is available in nightlies, and will be stabilized soon.

[hyper]: https://crates.io/crates/hyper
[html5ever]: https://crates.io/crates/html5ever
[regex]: https://crates.io/crates/regex
[rust-encoding]: https://crates.io/crates/encoding

### What's in 1.3 beta

One of the most exciting developments during the 1.3 cycle was the introduction
of the [Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/), a new book
covering "The Dark Arts of Advanced and Unsafe Rust Programming". While it's
still in its early days, this book already provides indispensable coverage of
some of Rust's more subtle aspects.

The 1.3 cycle also saw additional focus on performance, though most wins here
are within the standard library:

- The substring matcher now uses a
  [more efficient algorithm](https://github.com/rust-lang/rust/pull/26327).
- There were
  [improvements to zero filling](https://github.com/rust-lang/rust/pull/26849)
  that speed up `Vec::resize` and `Read::read_to_end`.
- The implementation of `Read::read_to_end` has been
  [specialized for `stdin` and `File`](https://github.com/rust-lang/rust/pull/26950),
  resulting in additional speedups.
- The `PartialEq` implementation on slices is now
  [much faster](https://github.com/rust-lang/rust/pull/26884).

We have also made strides in our Windows support, landing
[preliminary support for targeting Windows XP](https://github.com/rust-lang/rust/pull/26601). While
we do not intend to treat Windows XP as a "first tier" platform, it is now
feasible to build Rust code for XP as long as you avoid certain parts of the
standard library.

On the Cargo front, we have landed support for
[lint capping](https://github.com/rust-lang/rust/pull/27260) as specified by an
[earlier RFC](https://github.com/rust-lang/rfcs/pull/1193). The idea is that
lints in your dependencies should not affect your ability to compile cleanly,
which in turn makes it easier to tweak the way lints work without undue hassle
in the ecosystem.

### Contributors to 1.2

The 1.2 stable release represents the hard work of 180 fine folks:

- Aaron Turon
- Abhishek Chanda
- Adolfo Ochagavía
- Aidan Hobson Sayers
- Akshay Chiwhane
- Alex Burka
- Alex Crichton
- Alex Stokes
- Alexander Artemenko
- Alexis Beingessner
- Andrea Canciani
- Andrew Foote
- Andrew Kensler
- Andrew Straw
- Ariel Ben-Yehuda
- Austin Hellyer
- Barosl Lee
- Ben Striegel
- Björn Steinbrink
- Brian Anderson
- Brian Campbell
- Brian Leibig
- Brian Quinlan
- Carol (Nichols || Goulding)
- Chris Hellmuth
- Christian Stadelmann
- Chuck Bassett
- Corey Farwell
- Cornel Punga
- Cruz Julian Bishop
- Dave Huseby
- David Campbell
- David Stygstra
- David Voit
- Eduard Bopp
- Eduard Burtescu
- Eli Friedman
- Emilio Cobos Álvarez
- Emily Dunham
- Eric Ye
- Erik Michaels-Ober
- Falco Hirschenberger
- Felix S. Klock II
- FuGangqiang
- Geoffrey Thomas
- Gleb Kozyrev
- Guillaume Gomez
- Gulshan Singh
- Heejong Ahn
- Huachao Huang
- Huon Wilson
- Ivan Ukhov
- Iven Hsu
- Jake Goulding
- Jake Hickey
- James Miller
- Jared Roesch
- Jeremy Schlatter
- Jexell
- Jim Blandy
- Johann Tuffe
- Johannes Hoff
- Johannes Oertel
- John Hodge
- Jonathan Reem
- Joshua Landau
- Kevin Ballard
- Kubilay Kocak
- Lee Jeffery
- Leo Correa
- Liigo Zhuang
- Lorenz
- Luca Bruno
- Luqman Aden
- Manish Goregaokar
- Marcel Müller
- Marcus Klaas
- Marin Atanasov Nikolov
- Markus Westerlind
- Martin Pool
- Marvin Löbel
- Matej Lach
- Mathieu David
- Matt Brubeck
- Matthew Astley
- Max Jacobson
- Maximilian Haack
- Michael Layzell
- Michael Macias
- Michael Rosenberg
- Michael Sproul
- Michael Woerister
- Mihnea Dobrescu-Balaur
- Mikhail Zabaluev
- Mohammed Attia
- Ms2ger
- Murarth
- Mário Feroldi
- Nathan Long
- Nathaniel Theis
- Nick Cameron
- Nick Desaulniers
- Nick Fitzgerald
- Nick Hamann
- Nick Howell
- Niko Matsakis
- Nils Liberg
- OlegTsyba
- Oliver 'ker' Schneider
- Oliver Schneider
- P1start
- Parker Moore
- Pascal Hertleif
- Paul Faria
- Paul Oliver
- Peer Aramillo Irizar
- Peter Atashian
- Peter Elmers
- Philip Munksgaard
- Ralph Giles
- Rein Henrichs
- Ricardo Martins
- Richo Healey
- Ricky Taylor
- Russell Johnston
- Russell McClellan
- Ryan Pendleton
- Ryman
- Rémi Audebert
- Sae-bom Kim
- Sean Collins
- Sean Gillespie
- Sean Patrick Santos
- Seo Sanghyeon
- Simon Sapin
- Simonas Kazlauskas
- Steve Gury
- Steve Klabnik
- Steven Allen
- Steven Fackler
- Steven Walter
- Sébastien Marie
- Tamir Duberstein
- Thomas Karpiniec
- Tim Ringenbach
- Tshepang Lekhonkhobe
- Ulrik Sverdrup
- Vadim Petrochenkov
- Wei-Ming Yang
- Wesley Wiser
- Wilfred Hughes
- Will Andrews
- Will Engler
- Xuefeng Wu
- XuefengWu
- Yongqian Li
- York Xiang
- Z1
- ben fleis
- benaryorg
- bluss
- bors
- clatour
- diwic
- dmgawel
- econoplas
- frankamp
- funkill
- inrustwetrust
- joliv
- klutzy
- marcell
- mdinger
- olombard
- peferron
- ray glover
- saml
- simplex
- sumito3478
- webmobster
