+++
path = "2026/09/01/Rustup-1.29.1"
title = "Announcing rustup 1.29.1"
authors = ["The Rustup Team"]
+++

The rustup team is happy to announce the release of rustup version 1.29.1.

[Rustup][install] is the recommended tool to install [Rust][rust], a
programming language that empowers everyone to build reliable and efficient
software.

## What's new in rustup 1.29.1

The headlines of this release are:

- Concurrency in certain `rustup` operations has been improved:
  - When running `rustup update`, rustup will first check for possible updates in parallel. [pr#4752]
  - When running `rustup component add` with multiple components, they will be installed concurrently. [pr#4790]

- Implicit installation of the active toolchain in `rustup-init` and `rustup` invocations has
  been deprecated where deemed unnecessary and will now produce a warning. [pr#4840]
  - Please see our
    [blog post](https://blog.rust-lang.org/inside-rust/2026/07/03/rustup-update-1.30/#refining-the-implicit-installation-behavior)
    for more details regarding this change.

- `rustup doc` now supports the `--serve` flag which allows serving the docs over local HTTP.
  This should help users with containerized browser and/or rustup setups. [pr#4986]

- Installing `i686-pc-windows-*` host toolchains on 64-bit Windows now requires `--force-non-host`.
  [pr#4935]

- `rustup-init` will no longer leave unexpected files on disk after cancelled installations. [pr#4996]

- A bug has been fixed which might cause Windows installation to fail when using `rustup-init.sh`.
  [pr#4756]

- "Target **triple**" has been renamed to "target **tuple**" across the project to reflect the
  [new terminology](https://github.com/rust-lang/rust/pull/125579/changes/a26450cf81d67d68d3c6157579f8d968349129e7).
  [pr#4743] [pr#4827] [pr#4834]
  - Please note that this is not a breaking change in the CLI since the existing
    options such as `--target` are not using this terminology.

In addition, rustup now officially supports `aarch64-pc-windows-gnullvm` as a host platform. [pr#4523]

[1.29.1]: https://github.com/rust-lang/rustup/releases/tag/1.29.1
[pr#4523]: https://github.com/rust-lang/rustup/pull/4523
[pr#4743]: https://github.com/rust-lang/rustup/pull/4743
[pr#4752]: https://github.com/rust-lang/rustup/pull/4752
[pr#4756]: https://github.com/rust-lang/rustup/pull/4756
[pr#4790]: https://github.com/rust-lang/rustup/pull/4790
[pr#4827]: https://github.com/rust-lang/rustup/pull/4827
[pr#4834]: https://github.com/rust-lang/rustup/pull/4834
[pr#4840]: https://github.com/rust-lang/rustup/pull/4840
[pr#4935]: https://github.com/rust-lang/rustup/pull/4935
[pr#4986]: https://github.com/rust-lang/rustup/pull/4986
[pr#4996]: https://github.com/rust-lang/rustup/pull/4996

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
