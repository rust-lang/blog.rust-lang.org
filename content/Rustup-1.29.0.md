+++
path = "2025/02/01/Rustup-1.29.0"
title = "Announcing rustup 1.29.0"
authors = ["The Rustup Team"]
+++

The rustup team is happy to announce the release of rustup version 1.29.0.

[Rustup][install] is the recommended tool to install [Rust][rust], a
programming language that empowers everyone to build reliable and efficient
software.

## What's new in rustup 1.29.0

Following the footsteps of many package managers in the pursuit of better
toolchain installation performance, the headline of this release is that rustup
has been enabled to **download components concurrently** and **unpack during
downloads** in operations such as `rustup update` or `rustup toolchain` and to
concurrently check for updates in `rustup check`, thanks to a [GSoC 2025
project][concurrent-rustup].
This is by no means a trivial change so a long
tail of issues might occur, please [report][issues] them if you have found any!

[concurrent-rustup]: https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#make-rustup-concurrent

Furthermore, rustup now officially supports the following host platforms:

- `sparcv9-sun-solaris`
- `x86_64-pc-solaris`

Also, rustup will start automatically inserting the right `$PATH` entries
during `rustup-init` for the following shells, in addition to those already
supported:

- `tcsh`
- `xonsh`

This release also comes with other quality-of-life improvements, to name a few:

- When running rust-analyzer via a proxy, rustup will consider the
  `rust-analyzer` binary from `PATH` when the rustup-managed one is not found.
  - This should be particularly useful if you would like to bring your own
    `rust-analyzer` binary, e.g. if you use Neovim, Helix, etc. or are
    developing rust-analyzer itself.

- Empty environment variables are now treated as unset. This should help with
  resetting configuration values to default when an override is present.

- `rustup check` will use different exit codes based on whether new updates
  have been found: it will exit with `100` on any updates or `0` for no
  updates.

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
