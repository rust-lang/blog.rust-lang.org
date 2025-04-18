+++
path = "2015/06/25/Rust-1.1"
title = "Rust 1.1 stable, the Community Subteam, and RustCamp"
authors = ["The Rust Core Team"]
aliases = [
    "2015/06/25/Rust-1.1.html",
    "releases/1.1.0",
]

[extra]
release = true
+++

We're happy to announce the completion of the first release cycle after Rust
1.0: today we are [releasing][install] Rust 1.1 stable, as well as 1.2 beta.

Read on for details the releases, as well as some exciting new developments
within the Rust community.

[install]: https://www.rust-lang.org/install.html

### What's in 1.1 Stable

One of the highest priorities for Rust after its 1.0 has been improving compile
times. Thanks to the hard work of a number of contributors, Rust 1.1 stable
provides a **32% improvement** in compilation time over Rust 1.0 (as measured by
bootstrapping).

Another major focus has been improving error messages throughout the
compiler. Again thanks to a number of contributors, a large portion of compiler
errors now include extended explanations accessible using the `--explain` flag.

Beyond these improvements, the 1.1 release includes a number of important new
features:

* *New `std::fs` APIs*. This release stabilizes a
  [large set of extensions](https://github.com/rust-lang/rfcs/pull/1044) to the
  filesystem APIs, making it possible, for example, to compile Cargo on stable Rust.
* *musl support*. It's
  [now possible](https://github.com/rust-lang/rust/pull/24777) to target
  [musl](https://www.musl-libc.org/) on Linux. Binaries built this way are
  statically linked and have zero dependencies. Nightlies are on the way.
* *`cargo rustc`*. It's now possible to build a Cargo package while passing
  arbitrary flags to the final `rustc` invocation.

More detail is available in the [release notes][rn].

[rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-110-june-2015

### What's in 1.2 Beta

Performance improvements didn't stop with 1.1 stable. Benchmark compilations are
showing an **additional 30%** improvement from 1.1 stable to 1.2 beta; Cargo's
main crate compiles 18% faster.

In addition, [parallel codegen](https://github.com/rust-lang/rust/pull/26018) is
working again, and can substantially speed up large builds in debug mode; it
gets another 33% speedup on bootstrapping on a 4 core machine. It's not yet on
by default, but will be in the near future.

Cargo has also seen some performance improvements, including a 10x speedup on
large "no-op" builds (from 5s to 0.5s on Servo), and shared target directories
that cache dependencies across multiple packages.

In addition to all of this, 1.2 beta includes our first support for MSVC
(Microsoft Visual C): the compiler is able to bootstrap, and we have preliminary
nightlies targeting the platform. This is a big step for our Windows support,
making it much easier to link Rust code against code built using the native
toolchain. Unwinding is not yet available -- code aborts on panic -- but the
implementation is otherwise complete, and all rust-lang crates are now testing
on MSVC as a first-tier platform.

Rust 1.2 stable will be released six weeks from now, together with 1.3 beta.

### Community news

In addition to the above technical work, there's some exciting news within the
Rust community.

In the past few weeks, we've [formed a new subteam][community] explicitly
devoted to supporting the Rust community. The team will have a number of
responsibilities, including aggregating resources for meetups and other events,
supporting diversity in the community through leadership in outreach, policies,
and awareness-raising, and working with our early production users and the core
team to help guide prioritization.

In addition, we'll soon be holding the first official Rust conference:
[RustCamp](https://rustcamp.com/), on August 1, 2015, in Berkeley, CA, USA. We've
received a number of excellent talk submissions, and are expecting a great
program.

[community]: https://internals.rust-lang.org/t/announcing-the-community-subteam/2248

### Contributors to 1.1

As with every release, 1.1 stable is the result of work from an amazing and
active community. Thanks to the 168 contributors to this release:

- Aaron Gallagher
- Aaron Turon
- Abhishek Chanda
- Adolfo Ochagavía
- Alex Burka
- Alex Crichton
- Alexander Polakov
- Alexis Beingessner
- Andreas Tolfsen
- Andrei Oprea
- Andrew Paseltiner
- Andrew Straw
- Andrzej Janik
- Aram Visser
- Ariel Ben-Yehuda
- Avdi Grimm
- Barosl Lee
- Ben Gesoff
- Björn Steinbrink
- Brad King
- Brendan Graetz
- Brian Anderson
- Brian Campbell
- Carol Nichols
- Chris Morgan
- Chris Wong
- Clark Gaebel
- Cole Reynolds
- Colin Walters
- Conrad Kleinespel
- Corey Farwell
- David Reid
- Diggory Hardy
- Dominic van Berkel
- Don Petersen
- Eduard Burtescu
- Eli Friedman
- Erick Tryzelaar
- Felix S. Klock II
- Florian Hahn
- Florian Hartwig
- Franziska Hinkelmann
- FuGangqiang
- Garming Sam
- Geoffrey Thomas
- Geoffry Song
- Graydon Hoare
- Guillaume Gomez
- Hech
- Heejong Ahn
- Hika Hibariya
- Huon Wilson
- Isaac Ge
- J Bailey
- Jake Goulding
- James Perry
- Jan Andersson
- Jan Bujak
- Jan-Erik Rediger
- Jannis Redmann
- Jason Yeo
- Johann
- Johann Hofmann
- Johannes Oertel
- John Gallagher
- John Van Enk
- Jordan Humphreys
- Joseph Crail
- Kang Seonghoon
- Kelvin Ly
- Kevin Ballard
- Kevin Mehall
- Krzysztof Drewniak
- Lee Aronson
- Lee Jeffery
- Liigo Zhuang
- Luke Gallagher
- Luqman Aden
- Manish Goregaokar
- Marin Atanasov Nikolov
- Mathieu Rochette
- Mathijs van de Nes
- Matt Brubeck
- Michael Park
- Michael Rosenberg
- Michael Sproul
- Michael Wu
- Michał Czardybon
- Mike Boutin
- Mike Sampson
- Ms2ger
- Nelo Onyiah
- Nicholas
- Nicholas Mazzuca
- Nick Cameron
- Nick Hamann
- Nick Platt
- Niko Matsakis
- Oliver Schneider
- P1start
- Pascal Hertleif
- Paul Banks
- Paul Faria
- Paul Quint
- Pete Hunt
- Peter Marheine
- Philip Munksgaard
- Piotr Czarnecki
- Poga Po
- Przemysław Wesołek
- Ralph Giles
- Raphael Speyer
- Ricardo Martins
- Richo Healey
- Rob Young
- Robin Kruppe
- Robin Stocker
- Rory O’Kane
- Ruud van Asseldonk
- Ryan Prichard
- Sean Bowe
- Sean McArthur
- Sean Patrick Santos
- Shmuale Mark
- Simon Kern
- Simon Sapin
- Simonas Kazlauskas
- Sindre Johansen
- Skyler
- Steve Klabnik
- Steven Allen
- Steven Fackler
- Swaroop C H
- Sébastien Marie
- Tamir Duberstein
- Theo Belaire
- Thomas Jespersen
- Tincan
- Ting-Yu Lin
- Tobias Bucher
- Toni Cárdenas
- Tshepang Lekhonkhobe
- Ulrik Sverdrup
- Vadim Chugunov
- Valerii Hiora
- Wangshan Lu
- Wei-Ming Yang
- Wojciech Ogrodowczyk
- Xuefeng Wu
- York Xiang
- Young Wu
- bors
- critiqjo
- diwic
- gareins
- inrustwetrust
- jooert
- klutzy
- kwantam
- leunggamciu
- mdinger
- nwin
- parir
- pez
- robertfoss
- sinkuu
- tynopex
- らいどっと
