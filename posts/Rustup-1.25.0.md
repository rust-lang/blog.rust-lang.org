+++
layout = "post"
date = 2022-07-11
title = "Announcing Rustup 1.25.0"
author = "The Rustup Working Group"
+++

The rustup working group is happy to announce the release of rustup version 1.25.0. [Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.25.0 is as easy as stopping any programs which may be using Rustup (e.g. closing your IDE) and running:

```
rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

[rust]: https://www.rust-lang.org
[install]: https://rustup.rs

## What's new in rustup 1.25.0

This version of Rustup involves a significant number of internal cleanups, both in terms
of the Rustup code and its documentation. In addition to a lot of work on the codebase
itself, due to the length of time since the last release this one has a record number
of contributors and we thank you all for your efforts and time.

One of the biggest changes in 1.25.0 is the new offer on Windows installs to auto-install
the Visual Studio 2022 compilers which should simplify the process of getting started for
people not used to developing on Windows with the MSVC-compatible toolchains.

A second important change for 1.25.0 is a number of PRs focussed around startup performance
for Rustup. While it may not seem all that important to many, Rustup's startup time is
a factor in the time it takes to do builds which involve large numbers of crates on systems
which do not have large numbers of CPU cores. Hopefully the people for whom this is a common
activity will notice an improvement; though there's more opportunity to speed things up still available.

Some, but by no means all, of the rest of this release's highlights includes support for
`rustup default none` to unset the default toolchain, support for Windows arm64, inclusion
of `rust-gdbgui` as a proxy so that platforms which support it can use GDB's gui mode with Rust,
and some improvements to `rustup-init.sh`.

Full details are available in the [changelog]!

Rustup's documentation is also available in [the rustup book][book].

[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made rustup 1.25.0 possible!

- 二手掉包工程师 (hi-rustin)
- Brian Bowman (Seeker14491)
- Jon Gjengset (jonho)
- pierwill
- Daniel Silverstone (kinnison)
- Robert Collins (rbtcollins)
- Alan Somers (asomers)
- Brennan Vincent (umanwizard)
- Jynn Nelson (jyn514)
- Eric Huss (ehuss)
- Will Bush (willbush)
- Thad Guidry (thadguidry)
- Alexander Lovchin (alovchin91)
- zoodirector
- Takayuki Nakata (giraffate)
- Yusuke Abe (chansuke)
- Wyatt Carss (wcarss)
- Sondre Aasemoen (sondr3)
- facklambda
- Chad Dougherty (crd477)
- Noritada Kobayashi (noritada)
- Milan (mdaverde)
- Pat Sier (pjsier)
- Matt Keeter (mkeeter)
- Alex Macleod (alexendoo)
- Sathwik Matsa (sathwikmatsa)
- Kushal Das (kushaldas)
- Justus Winter (teythoon)
- k900
- Nicolas Ambram (nico-abram)
- Connor Slade (basicprogrammer10)
- Yerkebulan Tulibergenov (yerke)
- Caleb Cartwright (calebcartwright)
- Matthias Beyer (matthiasbeyer)
- spacemaniac
- Alex Touchet (atouchet)
- Guillaume Gomez (guillaumegomez)
- Chris Denton (chrisdenton)
- Thomas Orozco (krallin)
- cui fliter (cuishuang)
- Martin Nordholts (enselic)
- Emil Gardström (emilgardis)
- Arlo Siemsen (arlosi)
