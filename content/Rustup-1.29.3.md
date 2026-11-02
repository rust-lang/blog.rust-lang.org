+++
path = "2026/11/03/Rustup-1.29.3"
title = "Announcing rustup 1.29.3"
authors = ["The Rustup Team"]

[taxonomies]
tags = ["rustup"]
+++

The rustup team has published a new point release of rustup, 1.29.3.

[Rustup][install] is the recommended tool to install [Rust][rust], a
programming language that empowers everyone to build reliable and efficient
software.

## What's new in rustup 1.29.3

This new patch release has fixed some minor regressions over the previous one:

- Fixed "Illegal instruction" error on certain x86_64 CPUs without SSE4.1.
  [pr#5162]

- Fixed "`lock()` not supported" error on Solaris. [pr#5167]

[1.29.3]: https://github.com/rust-lang/rustup/releases/tag/1.29.3
[pr#5162]: https://github.com/rust-lang/rustup/pull/5162
[pr#5167]: https://github.com/rust-lang/rustup/pull/5167

Further details are available in the [changelog]!

## How to update

If you have a previous version of rustup installed, getting the new one is as easy as stopping
any programs which may be using rustup (e.g. closing your IDE) and running:

```
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
$ rustup update
```

For this particular release, if you have encountered any issues during
self-update, you may try to update rustup manually by following the steps below:

1. [Download][manual-installation] the corresponding `rustup-init` binary for
   your platform.
2. Make double sure that no other `rustup` processes are running.
3. Replace your existing `rustup` binary with the downloaded `rustup-init`
   renamed to `rustup`.
   - As a reminder, this binary is often located at `$HOME/.cargo/bin/rustup`, or
     `$CARGO_HOME/bin/rustup` if you have overridden the `$CARGO_HOME` environment
     variable.

[manual-installation]: https://rust-lang.github.io/rustup/installation/other.html#manual-installation

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

Rustup's documentation is also available in [the rustup book][book].

## Caveats

Rustup releases can come with problems not caused by rustup itself but just due to having a new release.

In particular, anti-malware scanners might block rustup or stop it from creating or copying
files, especially when installing `rust-docs` which contains many small files.

Issues like this should be automatically resolved in a few weeks when the anti-malware scanners are updated
to be aware of the new rustup release.

## Thanks

Thanks again to all the [contributors] who made this rustup release possible!

[issues]: https://github.com/rust-lang/rustup/issues
[book]: https://rust-lang.github.io/rustup/
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[contributors]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md#detailed-changes
[install]: https://rustup.rs
[rust]: https://www.rust-lang.org
