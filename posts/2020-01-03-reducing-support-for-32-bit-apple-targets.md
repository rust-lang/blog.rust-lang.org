---
layout: post
title: "Reducing support for 32-bit Apple targets"
author: Pietro Albini
---

The Rust team regrets to announce that Rust 1.41.0 (to be released on January
30th, 2020) will be the last release with the current level of support for
32-bit Apple targets. Starting from Rust 1.42.0, those targets will be demoted
to Tier 3.

The decision was made on [RFC 2837], and was accepted by the compiler and
release teams. This post explains what the change means, why we did it, and how
your project is affected.

[RFC 2837]: https://github.com/rust-lang/rfcs/pull/2837

# What’s a support tier?

The Rust compiler can build code targeting [a lot of
platforms][platform-support] (also called “targets”), but the team doesn't have
the resources or manpower to provide the same level of support and testing for
each of them.
To make our commitments clear, we follow a tiered support policy (currently
being formalized and revised in [RFC 2803]), explaining what we guarantee:

- Tier 1 targets can be downloaded through rustup and are fully tested
  during the project’s automated builds. A bug or a regression affecting one of
  these targets is usually prioritized more than bugs only affecting platforms
  in other tiers.

- Tier 2 targets can also be downloaded through rustup, but our
  automated builds don’t execute the test suite for them. While we guarantee a
  standard library build (and for some of them a full compiler build) will be
  available, we don’t ensure it will actually work without bugs (or even work
  at all).

- Tier 3 targets are not available for download through rustup, and are
  ignored during our automated builds. You can still build their standard
  library for cross-compiling (or the full compiler in some cases) from source
  on your own, but you might encounter build errors, bugs, or missing features.

[platform-support]: https://forge.rust-lang.org/release/platform-support.html
[RFC 2803]: https://github.com/rust-lang/rfcs/pull/2803

# Which targets are affected?

The main target affected by this change is 32-bit macOS (`i686-apple-darwin`),
which will be demoted from Tier 1 to Tier 3. This will affect both using the
compiler on 32-bit Mac hardware, and cross-compiling 32-bit macOS binaries from
any other platform.

Additionally, the following 32-bit iOS targets will be demoted from Tier 2 to
Tier 3:

* `armv7-apple-ios`
* `armv7s-apple-ios`
* `i386-apple-ios`

We will continue to provide the current level of support for all Apple 64bit
targets.

# Why are those targets being demoted?

Apple dropped support for running 32-bit binaries starting from [macOS
10.15][deprecate-macos] and [iOS 11][deprecate-ios]. They also prevented all
developers from cross-compiling 32-bit programs and apps starting from Xcode 10
(the platform’s IDE, containing the SDKs).

Due to those decisions from Apple, the targets are no longer useful to our users,
and their choice to prevent cross-compiling makes it hard for the
project to continue supporting the 32-bit platform in the long term.

[deprecate-macos]: https://support.apple.com/en-us/HT208436
[deprecate-ios]: https://developer.apple.com/documentation/uikit/app_and_environment/updating_your_app_from_32-bit_to_64-bit_architecture

# How will this affect my project?

If you don’t build 32-bit Apple binaries this change won’t affect you at all.

If you still need to build them, you’ll be able to continue using Rust 1.41.0
without issues. As usual the Rust project will provide critical bugfixes and
security patches until the next stable version is released (on March 12th,
2020), and we plan to keep the release available for download for the
foreseeable future (as we do with all the releases shipped so far).

The code implementing the targets won’t be removed from the compiler codebase,
so you’ll also be able to build future releases from source on your own
(keeping in mind they might have bugs or be broken, as that code will be
completly untested).

# What about the nightly channel?

We will demote the targets on the nightly channel soon, but we don't have an
exact date for when that will happen. We recommend pinning a nightly version
beforehand though, to prevent `rustup toolchain install` from failing once we
apply the demotion.

To pin a nightly version you need to use "nightly" followed by the day the
nightly was released, as the toolchain name. For example, to install the nightly
released on December 1st, 2019 and to use it you can run:

```plain
rustup toolchain install nightly-2019-12-01

# Default to this nightly system-wide...
rustup default nightly-2019-12-01

# ...or use this nightly for a single build
cargo +nightly-2019-12-01 build
```
