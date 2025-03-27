+++
layout = "post"
date = 2024-05-06
title = "Announcing Rustup 1.27.1"
author = "The Rustup Team"
+++

The Rustup team is happy to announce the release of Rustup version 1.27.1.
[Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rustup installed, getting Rustup 1.27.1 is as easy as stopping any programs which may be using Rustup (e.g. closing your IDE) and running:

```console
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```console
$ rustup update
```

If you don't have it already, you can [get Rustup][install] from the appropriate page on our website.

[rust]: https://www.rust-lang.org
[install]: https://rustup.rs

## What's new in Rustup 1.27.1

This new Rustup release involves some minor bug fixes.

The headlines for this release are:

1. Prebuilt Rustup binaries should be working on older macOS versions again.
2. `rustup-init` will no longer fail when `fish` is installed but `~/.config/fish/conf.d` hasn't been created.
3. Regressions regarding symlinked `RUSTUP_HOME/(toolchains|downloads|tmp)` have been addressed.

Full details are available in the [changelog]!

Rustup's documentation is also available in [the Rustup Book][book].

[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made Rustup 1.27.1 possible!

- Anas (0x61nas)
- cuiyourong (cuiyourong)
- Dirkjan Ochtman (djc)
- Eric Huss (ehuss)
- eth3lbert (eth3lbert)
- hev (heiher)
- klensy (klensy)
- Chih Wang (ongchi)
- Adam (pie-flavor)
- rami3l (rami3l)
- Robert (rben01)
- Robert Collins (rbtcollins)
- Sun Bin (shandongbinzhou)
- Samuel Moelius (smoelius)
- vpochapuis (vpochapuis)
- Renovate Bot (renovate)

