+++
path = "2024/03/11/Rustup-1.27.0"
title = "Announcing Rustup 1.27.0"
authors = ["The Rustup Team"]
aliases = ["2024/03/11/Rustup-1.27.0.html"]
+++

The rustup team is happy to announce the release of rustup version 1.27.0.
[Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.27.0 is as easy as stopping any programs which may be using Rustup (e.g. closing your IDE) and running:

```console
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```console
$ rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

[rust]: https://www.rust-lang.org
[install]: https://rustup.rs

## What's new in rustup 1.27.0

This long-awaited Rustup release has gathered all the new features and fixes since April 2023. These changes include improvements in Rustup's maintainability, user experience, compatibility and documentation quality.

Also, it's worth mentioning that Dirkjan Ochtman (djc) and rami3l (rami3l) have joined the team and are coordinating this new release.

At the same time, we have granted Daniel Silverstone (kinnison) and <span lang=zh>二手掉包工程师</span> (hi-rustin) their well-deserved alumni status in this release cycle.
Kudos for your contributions over the years and your continuous guidance on maintaining the project!

The headlines for this release are:

1. Basic support for the fish shell has been added.
   If you're using `fish`, PATH configs for your Rustup installation will be added automatically from now on.

   _Please note that this will only take effect on installation, so if you have already installed Rustup on your machine, you will need to reinstall it.
   For example, if you have installed Rustup via [rustup.rs][install], simply follow [rustup.rs][install]'s instructions again;
   if you have installed Rustup using [some other method][other installation methods], you might want to reinstall it using that same method._

2. Rustup support for `loongarch64-unknown-linux-gnu` as a _host platform_ has been added.
   This means you should be able to install Rustup via [rustup.rs][install] and no longer have to rely on [loongnix.cn] or self-compiled installations.

   _Please note that as of March 2024, `loongarch64-unknown-linux-gnu` is a ["tier 2 platform with host tools"], so Rustup is guaranteed to build for this platform.
   According to Rust's [target tier policy], this does not imply that these builds are also guaranteed to work, but they often work to quite a good degree and patches are always welcome!_


Full details are available in the [changelog]!

Rustup's documentation is also available in [the rustup book][book].

[other installation methods]: https://rust-lang.github.io/rustup/installation/other.html
[loongnix.cn]: https://rust-lang.loongnix.cn
["tier 2 platform with host tools"]: https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-with-host-tools
[target tier policy]: https://doc.rust-lang.org/nightly/rustc/target-tier-policy.html
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made rustup 1.27.0 possible!

- Anthony Perkins (acperkins)
- Tianqi (airstone42)
- Alex Gaynor (alex)
- Alex Hudspith (alexhudspith)
- Alan Somers (asomers)
- Brett (brettearle)
- Burak Emir (burakemir)
- Chris Denton (ChrisDenton)
- cui fliter (cuishuang)
- Dirkjan Ochtman (djc)
- Dezhi Wu (dzvon)
- Eric Swanson (ericswanson-dfinity)
- Prikshit Gautam (gautamprikshit1)
- hev (heiher)
- <span lang=zh>二手掉包工程师</span> (hi-rustin)
- Kamila Borowska (KamilaBorowska)
- klensy (klensy)
- Jakub Beránek (Kobzol)
- Kornel (kornelski)
- Matt Harding (majaha)
- Mathias Brossard (mbrossard)
- Christian Thackston (nan60)
- Ruohui Wang (noirgif)
- Olivier Lemasle (olivierlemasle)
- Chih Wang (ongchi)
- Pavel Roskin (proski)
- rami3l (rami3l)
- Robert Collins (rbtcollins)
- Sandesh  Pyakurel (Sandesh-Pyakurel)
- Waffle Maybe (WaffleLapkin)
- Jubilee (workingjubilee)
- WÁNG Xuěruì (xen0n)
- Yerkebulan Tulibergenov (yerke)
- Renovate Bot (renovate)
