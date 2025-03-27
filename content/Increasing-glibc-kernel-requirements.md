+++
layout = "post"
date = 2022-08-01
title = "Increasing the glibc and Linux kernel requirements"
author = "Nikita Popov"
+++

The minimum requirements for Rust toolchains targeting Linux will [increase][PR] with the
Rust 1.64.0 release (slated for September 22nd, 2022). The new minimum requirements are:

* glibc >= 2.17 (previously glibc >= 2.11)
* kernel >= 3.2 (previously kernel >= 2.6.32)

These requirements apply both to running the Rust compiler itself (and other Rust tooling like
Cargo or Rustup), and to running binaries produced by Rust, if they use the libstd.

If you are not targeting an old long-term-support distribution, or embedded hardware running
an old Linux version, this change is unlikely to affect you. Otherwise, read on!

# Affected targets

In principle, the new kernel requirements affect all `*-linux-*` targets, while the glibc
requirements affect all `*-linux-gnu*` targets. In practice, many targets were already requiring
newer kernel or glibc versions. The requirements for such targets do not change.

Among targets for which a Rust host toolchain is distributed, the following *are* affected:

* `i686-unknown-linux-gnu` (Tier 1)
* `x86_64-unknown-linux-gnu` (Tier 1)
* `x86_64-unknown-linux-musl` (Tier 2 with host tools)
* `powerpc-unknown-linux-gnu` (Tier 2 with host tools)
* `powerpc64-unknown-linux-gnu` (Tier 2 with host tools)
* `s390x-unknown-linux-gnu` (Tier 2 with host tools)

The following are *not* affected, because they already had higher glibc/kernel requirements:

* `aarch64-unknown-linux-gnu` (Tier 1)
* `aarch64-unknown-linux-musl` (Tier 2 with host tools)
* `arm-unknown-linux-gnueabi` (Tier 2 with host tools)
* `arm-unknown-linux-gnueabihf` (Tier 2 with host tools)
* `armv7-unknown-linux-gnueabihf` (Tier 2 with host tools)
* `mips-unknown-linux-gnueabihf` (Tier 2 with host tools)
* `powerpc64le-unknown-linux-gnueabihf` (Tier 2 with host tools)
* `riscv64gc-unknown-linux-gnueabihf` (Tier 2 with host tools)

For other tier 2 or tier 3 targets, for which no Rust toolchain is distributed, we do not
accurately track minimum requirements, and they may or may not be affected by this change.
`*-linux-musl*` targets are only affected by the kernel requirements, not the glibc requirements.
Targets which only use libcore and not libstd are unaffected.

A list of supported targets and their requirements can be found on the
[platform support page][platform-support].

# Affected systems

The glibc and kernel versions used for the new baseline requirements are already close to a decade
old. As such, this change should only affect users that either target old long-term-support Linux
distributions, or embedded hardware running old versions of Linux.

The following Linux distributions *are* still supported under the new requirements:

* RHEL 7 (glibc 2.17, kernel 3.10)
* SLES 12-SP5 (glibc 2.22, kernel 4.12.14)
* Debian 8 (glibc 2.19, kernel 3.16.7)
* Ubuntu 14.04 (glibc 2.19, kernel 3.13)

The following distributions are *not* supported under the new requirements:

* RHEL 6 (glibc 2.12, kernel 2.6.32)
* SLES 11-SP4 (glibc 2.11.3, kernel 3.0.101)
* Debian 6 (glibc 2.11, kernel 2.6.32), Debian 7 (glibc 2.13, kernel 3.2.41)
* Ubuntu 12.04 (glibc 2.15, kernel 3.2)

Out of the distributions in the second list, only RHEL 6 still has limited vendor support (ELS).

# Why increase the requirements?

We want Rust, and binaries produced by Rust, to be as widely usable as possible. At the same time,
the Rust project only has limited resources to maintain compatibility with old environments.

There are two parts to the toolchain requirements: The minimum requirements for running the Rust
compiler on a host system, and the minimum requirements for cross-compiled binaries.

The minimum requirements for host toolchains affect our build system. Rust CI produces binary
artifacts for dozens of different targets. Creating binaries that support old glibc versions
requires either building on an operating system with old glibc (for native builds) or using a
buildroot with an old glibc version (for cross-compiled builds).

At the same time, Rust relies on LLVM for optimization and code generation, which regularly
increases its toolchain requirements. LLVM 16 will require GCC 7.1 or newer (and LLVM 15 supports
GCC 5.1 in name only). Creating a build environment that has both a very old glibc and a recent
compiler becomes increasingly hard over time. crosstool-ng (which we use for most cross-compilation
needs) does not support targeting both glibc 2.11, and using a compiler that satisfies the new LLVM
requirements.

The requirements for cross-compiled binaries have a different motivation: They affect which kernel
versions need to be supported by libstd. Increasing the kernel requirements allows libstd to use
newer syscalls, without having to maintain and test compatibility with kernels that do not support
them.

The new baseline requirements were picked as the least common denominator among long-term-support
distributions that still have active support. This is currently RHEL 7 with glibc 2.17 and
kernel 3.10. The kernel requirement is picked as 3.2 instead, because this is the minimum
requirement of glibc itself, and there is little relevant API difference between these versions.

# What should I do?

If you or your organization are affected by this change, there are a number of viable options
depending on your situation:

* Upgrade your target system, or raise the minimum requirements of your software, to satisfy the
  new constraints.
* If you are running the Rust compiler on an old host, consider cross-compiling from a newer host
  instead.
* If you are targeting an old glibc version, consider targeting musl instead.
* If you are targeting an old kernel version and use libstd, you may be out of luck: In this case
  you may have to either freeze your current Rust version, or maintain a fork of libstd that
  supports older kernels.

[PR]: https://github.com/rust-lang/rust/pull/95026
[platform-support]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
