+++
path = "inside-rust/2025/12/20/rustup-1.29.0-beta-cft"
title = "Rustup 1.29.0 beta: Call for Testing!"
authors = ["rami3l"]

[extra]
team = "The Rustup Team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-rustup"
+++

We are excited to announce that rustup 1.29.0 beta is now available for testing
and we are currently looking for testers.

## What's new

Following the footsteps of many package managers in the pursuit of better
toolchain installation performance, the headline of this release is that rustup
has been enabled to **download components concurrently** and **unpack during
downloads** in operations such as `rustup update` or `rustup toolchain` and to
concurrently check for updates in `rustup check`, thanks to a [GSoC 2025
project][concurrent-rustup]. This is by no means a trivial change so a long
tail of issues might occur, please report them if you have found any!
[pr#4388] [pr#4426] [pr#4436] [pr#4455] [pr#4471] [pr#4605]

[concurrent-rustup]: https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#make-rustup-concurrent

As usual, we would be happy to [receive][issues] regression/breakage reports of
any kind, especially regarding the installation and/or usage on the following
environments:

[issues]: https://github.com/rust-lang/rustup/issues

* New host platforms:
  - `sparcv9-sun-solaris` [pr#4380]
  - `x86_64-pc-solaris` [pr#4380]

* New shells:
  - `tcsh` [pr#4459]
  - `xonsh` [pr#4626]

* Environments where you would like to bring your own `rust-analyzer` binary
  (such as r-a developers and certain Neovim/Helix users): rustup will now
  consider the `rust-analyzer` binary from `PATH` when the rustup-managed one is
  not found. [pr#4324]

- Environments where you would like to override an environment variable back to
  the default: rustup now treats empty values as unset. [pr#4422]

[pr#4324]: https://github.com/rust-lang/rustup/pull/4324
[pr#4380]: https://github.com/rust-lang/rustup/pull/4380
[pr#4388]: https://github.com/rust-lang/rustup/pull/4388
[pr#4422]: https://github.com/rust-lang/rustup/pull/4422
[pr#4426]: https://github.com/rust-lang/rustup/pull/4426
[pr#4436]: https://github.com/rust-lang/rustup/pull/4436
[pr#4455]: https://github.com/rust-lang/rustup/pull/4455
[pr#4459]: https://github.com/rust-lang/rustup/pull/4459
[pr#4471]: https://github.com/rust-lang/rustup/pull/4471
[pr#4605]: https://github.com/rust-lang/rustup/pull/4605
[pr#4626]: https://github.com/rust-lang/rustup/pull/4626

## How to test

To begin testing this new version, all you need to do is simply switching to
the dev environment by setting the following environment variable when updating
or installing rustup:

```sh
RUSTUP_UPDATE_ROOT=https://dev-static.rust-lang.org/rustup
```

To switch out of the dev environment, just remove that environment variable and
do a `rustup self update`.

Finally, please don't forget to check out the corresponding section in our
[CHANGELOG.md](https://github.com/rust-lang/rustup/blob/master/CHANGELOG.md#1290---)
for the complete list of changes included in this version.

## Acknowledgements

A big thank you to:
- [@djc] for continuously polishing the codebase to get rid of a whole lot of
  historical burden and carefully shaping the final form of preliminary
  concurrency support we have today;
- [@ChrisDenton] for the careful handling of IO and Windows-related complexities;
- [@FranciscoTGouveia] for joining me in the GSoC event to investigate the
  possibilities and lay the foundation of concurrency in rustup;
- [@Kobzol] for enabling and organizing rustup's participation in GSoC 2025 and
  improving our CLI's cold start performance;
- ... and many other contributors who have made this new release possible!

Many thanks for everyone's continued support! Wishing you a magical holiday
season surrounded by love, peace, and laughter 🍀

[@djc]: https://github.com/djc
[@ChrisDenton]: https://github.com/chrisdenton
[@FranciscoTGouveia]: https://github.com/FranciscoTGouveia
[@Kobzol]: https://github.com/kobzol
