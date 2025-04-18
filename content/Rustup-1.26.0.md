+++
path = "2023/04/25/Rustup-1.26.0"
title = "Announcing Rustup 1.26.0"
authors = ["The Rustup Working Group"]
aliases = ["2023/04/25/Rustup-1.26.0.html"]
+++

The rustup working group is happy to announce the release of rustup version 1.26.0. [Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.26.0 is as easy as stopping any programs which may be using Rustup (e.g. closing your IDE) and running:

```
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
$ rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

[rust]: https://www.rust-lang.org
[install]: https://rustup.rs

## What's new in rustup 1.26.0

This version of Rustup involves a significant number of internal cleanups, both in terms
of the Rustup code and its tests. In addition to a lot of work on the codebase
itself, due to the length of time since the last release this one has a record number
of contributors and we thank you all for your efforts and time.

The headlines for this release are:

1. Add rust-analyzer as a proxy of rustup. Now you can call rust-analyzer and it will be proxied to the rust-analyzer component for the current toolchain.

2. Bump the clap dependency from 2.x to 3.x. It's a major version bump, so there are some help text changes, but the command line interface is unchanged.

3. Remove experimental GPG signature validation and the rustup show keys command. Due to its experimental status, validating the integrity of downloaded binaries did not rely on it, and there was no option to abort the installation if a signature mismatch happened. Multiple problems with its implementation were discovered in the recent months, which led to the decision to remove the experimental code. The team is working on the design of a new signature validation scheme, which will be implemented in the future.

Full details are available in the [changelog]!

Rustup's documentation is also available in [the rustup book][book].

[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made rustup 1.26.0 possible!

- Daniel Silverstone (kinnison)
- Sabrina Jewson (SabrinaJewson)
- Robert Collins (rbtcollins)
- chansuke (chansuke)
- Shamil (shamilsan)
- Oli Lalonde (olalonde)
- 二手掉包工程师 (hi-rustin)
- Eric Huss (ehuss)
- J Balint BIRO (jbalintbiro)
- Easton Pillay (jedieaston)
- zhaixiaojuan (zhaixiaojuan)
- Chris Denton (ChrisDenton)
- Martin Geisler (mgeisler)
- Lucio Franco (LucioFranco)
- Nicholas Bishop (nicholasbishop)
- SADIK KUZU (sadikkuzu)
- darkyshiny (darkyshiny)
- René Dudfield (illume)
- Noritada Kobayashi (noritada)
- Mohammad AlSaleh (MoSal)
- Dustin Martin (dmartin)
- Ville Skyttä (scop)
- Tshepang Mbambo (tshepang)
- Illia Bobyr (ilya-bobyr)
- Vincent Rischmann (vrischmann)
- Alexander (Alovchin91)
- Daniel Brotsky (brotskydotcom)
- zohnannor (zohnannor)
- Jynn Nelson (jyn514)
- Prikshit Gautam (gautamprikshit1)
- Dylan Thacker-Smith (dylanahsmith)
- Jan David (jdno)
- Aurora (lilith13666)
- Pietro Albini (pietroalbini)
- Renovate Bot (renovate-bot)
