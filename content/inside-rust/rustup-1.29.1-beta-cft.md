+++
path = "inside-rust/2026/07/14/rustup-1.29.1-beta-cft"
title = "Rustup 1.29.1 beta: call for testing!"
authors = ["rami3l"]

[extra]
team = "The Rustup Team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-rustup"
+++

We are excited to announce that rustup 1.29.1 beta is now available for testing
and we are currently looking for testers.

## What's new

The headlines of this release are:

- Concurrency in certain `rustup` operations has been improved:
  - When running `rustup update`, rustup will first check for possible updates in parallel. [pr#4752]
  - When running `rustup component add` with multiple components, they will be installed concurrently. [pr#4790]

- Implicit installation of the active toolchain in `rustup-init` and `rustup` invocations has
  been deprecated where deemed unnecessary and will now produce a warning. [pr#4840]
  - Please see our
    [blog post](https://blog.rust-lang.org/inside-rust/2026/07/03/rustup-update-1.30/#refining-the-implicit-installation-behavior)
    for more details regarding this change.

- Installing `i686-pc-windows-*` host toolchains on 64-bit Windows now requires `--force-non-host`.
  [pr#4935]

- A bug has been fixed which might cause Windows installation to fail when using `rustup-init.sh`.
  [pr#4756]

- "Target **triple**" has been renamed to "target **tuple**" across the project to reflect the
  [new terminology](https://github.com/rust-lang/rust/pull/125579/changes/a26450cf81d67d68d3c6157579f8d968349129e7).
  [pr#4743] [pr#4827] [pr#4834]
  - Please note that this is not a breaking change in the CLI since the existing
    options such as `--target` are not using this terminology.

In addition, rustup now officially supports `aarch64-pc-windows-gnullvm` as a host platform. [pr#4523]

[pr#4523]: https://github.com/rust-lang/rustup/pull/4523
[pr#4743]: https://github.com/rust-lang/rustup/pull/4743
[pr#4752]: https://github.com/rust-lang/rustup/pull/4752
[pr#4756]: https://github.com/rust-lang/rustup/pull/4756
[pr#4790]: https://github.com/rust-lang/rustup/pull/4790
[pr#4827]: https://github.com/rust-lang/rustup/pull/4827
[pr#4834]: https://github.com/rust-lang/rustup/pull/4834
[pr#4840]: https://github.com/rust-lang/rustup/pull/4840
[pr#4935]: https://github.com/rust-lang/rustup/pull/4935

Further details are available in the [changelog]!

As usual, we would be happy to [receive][issues] regression/breakage reports of
any kind.

## How to test

To begin testing this new version, all you need to do is simply switching to
the dev environment by setting the following environment variable when updating
or installing rustup:

```sh
RUSTUP_UPDATE_ROOT=https://dev-static.rust-lang.org/rustup
```

To switch out of the dev environment, just remove that environment variable and
do a `rustup self update`.

[issues]: https://github.com/rust-lang/rustup/issues
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
