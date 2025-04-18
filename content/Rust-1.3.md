+++
path = "2015/09/17/Rust-1.3"
title = "Announcing Rust 1.3"
authors = ["The Rust Core Team"]
aliases = [
    "2015/09/17/Rust-1.3.html",
    "releases/1.3.0",
]

[extra]
release = true
+++

The gear keeps turning: we're releasing Rust 1.3 stable today! As always, read
on for the highlights and check the [release notes][notes] for more detail.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-130-september-2015

### What's in 1.3 stable

This is our first release shipping with the
[Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/), a new book covering
"The Dark Arts of Advanced and Unsafe Rust Programming". While it's still in
draft form, this book already provides deep coverage of some of Rust's darker
corners.

On the library front, we saw a fair amount of API stabilization, including the
new `Duration` API and enhancements to `Error` and `Hash`/`Hasher`. We expect to
see further growth of the `std::time` module in the 1.5 timeframe.

The 1.3 cycle also saw continuing focus on performance. Most wins here are
within the standard library:

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

We're continuing to invest in Windows, with
[preliminary support for targeting Windows XP](https://github.com/rust-lang/rust/pull/26601). While
we do not intend to treat Windows XP as a "first tier" platform, it is now
feasible to build Rust code for XP as long as you avoid certain parts of the
standard library. Work on MSVC toolchain integration is ongoing, with full
support (on 64-bit) shipping in the 1.4 beta today.

On the Cargo front, we have landed support for
[lint capping](https://github.com/rust-lang/rust/pull/27260) as specified by an
[earlier RFC](https://github.com/rust-lang/rfcs/pull/1193). The idea is that
lints in your dependencies should not affect your ability to compile cleanly,
which in turn makes it easier to tweak the way lints work without undue hassle
in the ecosystem.

### Contributors to 1.3

Rust is a community-driven language, and we're delighted to thank the 131
contributors to this release:

- Aaron Turon
- Adam Heins
- Agoston Szepessy
- Aidan Hobson Sayers
- Akos Kiss
- Alex Crichton
- Alex Newman
- Alexis Beingessner
- Alisdair Owens
- Andreas Tolfsen
- Andrew Kuchev
- Andrew Paseltiner
- Andy Caldwell
- Andy Grover
- Antti Keränen
- Ariel Ben-Yehuda
- Barosl Lee
- Benjamin Herr
- Björn Steinbrink
- Blake Loring
- Brian Anderson
- Brody Holden
- Chris Morgan
- Christian Persson
- Christian Weinz
- Cole Reynolds
- Corey Farwell
- Corey Richardson
- Cristian Kubis
- Cruz Julian Bishop
- Daniel Albert
- Dave Huseby
- Dirkjan Ochtman
- Eduard Burtescu
- Eli Friedman
- Eljay
- Esption
- Felix S. Klock II
- Florian Hartwig
- Frank McSherry
- FuGangqiang
- Geoffrey Thomas
- Georg Brandl
- Guillaume Gomez
- Huon Wilson
- Ivan Ukhov
- Jan Likar
- Jared Roesch
- Jashank Jeremy
- Jason Schein
- Jeehoon Kang
- Jesús Espino
- Johannes Oertel
- John Hodge
- Jonathan Hansford
- Jonathan Reem
- Jose Narvaez
- Josh Triplett
- Joshua Landau
- Kagami Sascha Rosylight
- Kelvin Ly
- Ken Tossell
- Kevin Ballard
- Kevin Butler
- Kieran Hunt
- Kornel Lesiński
- Kristof Söderström
- Lee Jeffery
- Leif Arne Storset
- Liigo Zhuang
- Makoto Kato
- Manish Goregaokar
- Marcus Klaas
- Mark Buer
- Mathieu David
- Mathijs van de Nes
- Matt Friedman
- Michael Sproul
- Michael Woerister
- Ms2ger
- Nick Cameron
- Nick Hamann
- Nick Howell
- Nicolette Verlinden
- Niko Matsakis
- OGINO Masanori
- Oliver Schneider
- P1start
- Paolo Falabella
- Pascal Hertleif
- Patrick Walton
- Pavel Pravosud
- Peter Atashian
- Peter Elmers
- Ralf Jung
- Remi Rampin
- Richo Healey
- Ryan Pendleton
- Scott Olson
- Sean Patrick Santos
- Seo Sanghyeon
- Simon Sapin
- Simonas Kazlauskas
- Steve Klabnik
- Steven Allen
- Steven Fackler
- Steven Stewart-Gallus
- Sébastien Marie
- Taliesin Beynon
- Tamir Duberstein
- Theo Belaire
- Ticki
- Tobias Bucher
- Tshepang Lekhonkhobe
- Ulrik Sverdrup
- Vadim Chugunov
- Vadim Petrochenkov
- Vincent Bernat
- Vladimir Rutsky
- Wei-Ming Yang
- Wesley Wiser
- William Throwe
- arthurprs
- bors
- diaphore
- eternaleye
- jethrogb
- krumelmonster
- mdinger
- midinastasurazz
- mitaa
