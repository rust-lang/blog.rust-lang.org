+++
path = "2021/06/08/Rustup-1.24.3"
title = "Announcing Rustup 1.24.3"
authors = ["The Rustup Working Group"]
aliases = ["2021/06/08/Rustup-1.24.3.html"]
+++

The rustup working group is happy to announce the release of rustup version 1.24.3. [Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.24.3 is as easy as closing your IDE and running:

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

## What's new in rustup 1.24.3

This patch release focuses around resolving some regressions in behaviour in
the 1.24.x series, in either low tier platforms, or unusual situations around
very old toolchains.

Full details are available in the [changelog]!

Rustup's documentation is also available in [the rustup book][book].

[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[book]: https://rust-lang.github.io/rustup/

## Thanks

Thanks again to all the contributors who made rustup 1.24.3 possible!

- Alexander (asv7c2)
- Ian Jackson
- pierwill
- 二手掉包工程师 (hi-rustin)
- Robert Collins
- Daniel Silverstone
