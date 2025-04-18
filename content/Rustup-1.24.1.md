+++
path = "2021/04/29/Rustup-1.24.1"
title = "Announcing Rustup 1.24.1"
authors = ["The Rustup Working Group"]
aliases = ["2021/04/29/Rustup-1.24.1.html"]
+++

The rustup working group is happy to announce the release of rustup version 1.24.1. [Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.24.1 is as easy as closing your IDE and running:

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

## What's new in rustup 1.24.1

Firstly, if you have not read the [previous announcement][1.24.0] then in brief, 1.24
introduces better support for low memory systems, installs itself into the Add/Remove programs
list on Windows, and now supports using `rust-toolchain.toml` files.

[1.24.0]: https://blog.rust-lang.org/2021/04/27/Rustup-1.24.0.html

Shortly after publishing the 1.24.0 release of Rustup, we got reports of [a regression][2737]
preventing users from running `rustfmt` and `cargo fmt` after upgrading to
Rustup 1.24.0. To limit the damage we **reverted** the release to version
1.23.1. The only substantive change between 1.24.0 and 1.24.1 is to correct this regression.

[2737]: https://github.com/rust-lang/rustup/issues/2737

### Other changes

You can check out all the changes to Rustup for 1.24.0 and 1.24.1 in the [changelog]!

Rustup's documentation is also available in [the rustup book][book].

[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made rustup 1.24.0 and 1.24.1 possible!

- Alex Chan
- Aloïs Micard
- Andrew Norton
- Avery Harnish
- chansuke
- Daniel Alley
- Daniel Silverstone
- Eduard Miller
- Eric Huss
- est31
- Gareth Hubball
- Gurkenglas
- Jakub Stasiak
- Jynn Nelson
- Jubilee (workingjubilee)
- kellda
- Michael Cooper
- Philipp Oppermann
- Robert Collins
- SHA Miao
- skim (sl4m)
- Tudor Brindus
- Vasili (3point2)
- наб (nabijaczleweli)
- 二手掉包工程师 (hi-rustin)
