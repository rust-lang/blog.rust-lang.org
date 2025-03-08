+++
layout = "post"
date = 2025-03-04
title = "Announcing rustup 1.28.1"
author = "The Rustup Team"
+++

The rustup team is happy to announce the release of rustup version 1.28.1.
[Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

## Challenges with rustup 1.28.0

rustup 1.28.0 was a significant release with many changes, and there was a quick response from many
folks that this release broke their processes. While we considered yanking the release, we worried
that this would cause problems for people who had already updated to adopt some of the changes.
Instead, we are rolling forward with 1.28.1 today and potentially further bugfix releases to
address the feedback that comes in.

We value all constructive feedback -- please keep it coming in the [issue tracker]. In particular,
the change with regard to implicit toolchain installation is being discussed in [this issue].

[this issue]: https://github.com/rust-lang/rustup/issues/4211
[issue tracker]: https://github.com/rust-lang/rustup/issues/

## What's new in rustup 1.28.1

This release contains the following fixes:

- Automatic install is enabled by default but can be opted out by setting `RUSTUP_AUTO_INSTALL`
  environment variable to `0`. [pr#4214] [pr#4227]
- `rustup show active-toolchain` will only print a single line, as it did in 1.27. [pr#4221]
- Fixed a bug in the reqwest backend that would erroneously timeout downloads after 30s. [pr#4218]
- Use relative symlinks for proxies. [pr#4226]

[1.28.1]: https://github.com/rust-lang/rustup/releases/tag/1.28.1
[pr#4214]: https://github.com/rust-lang/rustup/pull/4214
[pr#4221]: https://github.com/rust-lang/rustup/pull/4221
[pr#4218]: https://github.com/rust-lang/rustup/pull/4218
[pr#4226]: https://github.com/rust-lang/rustup/pull/4226
[pr#4227]: https://github.com/rust-lang/rustup/pull/4227

## How to update

If you have a previous version of rustup installed, getting rustup 1.28.1 is as easy as stopping
any programs which may be using Rustup (e.g. closing your IDE) and running:

```console
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```console
$ rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

Rustup's documentation is also available in [the rustup book][book].

## Caveats

Rustup releases can come with problems not caused by rustup itself but just due to having a new release.
As such, we recommend paying attention to the following potential issues in particular:

- Anti-malware scanners might be blocking rustup or stopping it from creating or copying files
  (especially when installing `rust-docs`, since it contains many small files).

- In your CI environment, rustup might fail when trying to perform a self-update.

  This is a [known issue](https://github.com/rust-lang/rustup/issues/3709),
  and in the case where this issue does occur, we recommend applying the following workaround at the beginning of your workflow:

  ```console
  $ rustup set auto-self-update disable
  ```

  Also, starting from 1.28.0, rustup will no longer attempt to self-update in CI environments,
  so this workaround should not be necessary in the future.

These issues should be automatically resolved in a few weeks when the anti-malware scanners are updated to be aware of the new rustup release,
and the hosted version is updated across all CI runners.

## Thanks

Thanks to the rustup and t-release team members who came together to quickly address these issues.

[book]: https://rust-lang.github.io/rustup/
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[contributors]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md#detailed-changes
[install]: https://rustup.rs
[rust]: https://www.rust-lang.org
