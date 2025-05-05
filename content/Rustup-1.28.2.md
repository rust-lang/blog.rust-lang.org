+++
path = "2025/05/05/Rustup-1.28.2"
title = "Announcing rustup 1.28.2"
authors = ["The Rustup Team"]
aliases = ["2025/05/05/Rustup-1.28.2.html"]
+++

The rustup team is happy to announce the release of rustup version 1.28.2.
[Rustup][install] is the recommended tool to install [Rust][rust], a programming language that
empowers everyone to build reliable and efficient software.

## What's new in rustup 1.28.2

The headlines of this release are:

- The cURL download backend and the native-tls TLS backend are now officially deprecated and
  a warning will start to show up when they are used. [pr#4277]

  - While rustup predates reqwest and rustls, the rustup team has long wanted to standardize on
    an HTTP + TLS stack with more components in Rust, which should increase security, potentially
    improve performance, and simplify maintenance of the project.
    With the default download backend already switched to reqwest since [2019][pr#1660], the team
    thinks it is time to focus maintenance on the default stack powered by these two libraries.

  - For people who have set `RUSTUP_USE_CURL=1` or `RUSTUP_USE_RUSTLS=0` in their environment to
    work around issues with rustup, please try to unset these after upgrading to 1.28.2 and file
    [an issue][issue tracker] if you still encounter problems.

- The version of `rustup` can be pinned when installing via `rustup-init.sh`, and
  `rustup self update` can be used to upgrade/downgrade rustup 1.28.2+ to a given version.
  To do so, set the `RUSTUP_VERSION` environment variable to the desired version (for example `1.28.2`).
  [pr#4259]

- `rustup set auto-install disable` can now be used to disable automatic installation of the toolchain.
  This is similar to the `RUSTUP_AUTO_INSTALL` environment variable introduced in 1.28.1 but with a
  lower priority. [pr#4254]

- Fixed a bug in Nushell integration that might generate invalid commands in the shell configuration.
  Reinstalling rustup might be required for the fix to work. [pr#4265]

[pr#1660]: https://github.com/rust-lang/rustup/pull/1660
[pr#4254]: https://github.com/rust-lang/rustup/pull/4254
[pr#4259]: https://github.com/rust-lang/rustup/pull/4259
[pr#4265]: https://github.com/rust-lang/rustup/pull/4265
[pr#4277]: https://github.com/rust-lang/rustup/pull/4277
[issue tracker]: https://github.com/rust-lang/rustup/issues/

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

[book]: https://rust-lang.github.io/rustup/
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[contributors]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md#detailed-changes
[install]: https://rustup.rs
[rust]: https://www.rust-lang.org
