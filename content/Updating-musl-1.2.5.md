+++
path = "2025/07/01/Updating-musl-1.2.5"
title = "Updating Rust's Linux musl targets to 1.2.5"
authors = ["Aria Desires"]
description = "musl targets will soon ship with musl 1.2.5"

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

# Updating Rust's Linux musl targets to 1.2.5

Beginning with Rust 1.90 (slated for stable release on 2025-09-18), the various `*-linux-musl` targets will [ship](https://github.com/rust-lang/rust/pull/142682) with musl 1.2.5. These targets currently use musl 1.2.3. This update comes with [several fixes and improvements](https://musl.libc.org/releases.html), and a breaking change that affects the Rust ecosystem.

For the Rust ecosystem, the primary motivation for this update is to receive major improvements to musl's DNS resolver which shipped in 1.2.4 and received bug fixes in 1.2.5. When using `musl` targets for static linking, this should make portable linux binaries that do networking more reliable, particularly in the face of large DNS records and recursive nameservers.

However, 1.2.4 also comes with a breaking change: [the removal of several legacy compatibility symbols that the Rust libc crate was using](https://github.com/rust-lang/libc/issues/2934). A fix for this [was shipped in libc 0.2.146 in June 2023 (2 years ago)](https://github.com/rust-lang/libc/pull/2935), and we have been waiting for newer versions of the libc crate to propagate throughout the ecosystem before shipping the musl update.

A crater run in July 2024 (a year ago) [found only about 2% of rust projects were still affected](https://github.com/rust-lang/rust/pull/125692#issuecomment-2245182085). At this point we expect there will be minimal breakage, and most breakage should be resolved by a `cargo update`. We believe this update shouldn't be held back any longer, as it contains critical fixes for the musl target.

# Updated targets

The following targets will be updated (this is just "all musl targets except [`loongarch64-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/loongarch-linux.html) which already was on 1.2.5, and [WALI](https://github.com/arjunr2/WALI) which doesn't use our bundled musl"):


## Tier 2 with Host Tools

target | notes
-------|-------
`aarch64-unknown-linux-musl` | ARM64 Linux with musl 1.2.3
[`powerpc64le-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/powerpc64le-unknown-linux-musl.html) | PPC64LE Linux (kernel 4.19, musl 1.2.3)
[`riscv64gc-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/riscv64gc-unknown-linux-musl.html) | RISC-V Linux (kernel 4.20, musl 1.2.3)
`x86_64-unknown-linux-musl` | 64-bit Linux with musl 1.2.3


## Tier 2 without Host Tools


target | std | notes
-------|:---:|-------
`arm-unknown-linux-musleabi` | ✓ | Armv6 Linux with musl 1.2.3
`arm-unknown-linux-musleabihf` | ✓ | Armv6 Linux with musl 1.2.3, hardfloat
`armv5te-unknown-linux-musleabi` | ✓ | Armv5TE Linux with musl 1.2.3
`armv7-unknown-linux-musleabi` | ✓ | Armv7-A Linux with musl 1.2.3
`armv7-unknown-linux-musleabihf` | ✓ | Armv7-A Linux with musl 1.2.3, hardfloat
`i586-unknown-linux-musl` | ✓ | 32-bit Linux (musl 1.2.3, original Pentium)
`i686-unknown-linux-musl` | ✓ | 32-bit Linux with musl 1.2.3 (Pentium 4)

## Tier 3


target | std | host | notes
-------|:---:|:----:|-------
[`hexagon-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/hexagon-unknown-linux-musl.html) | ✓ | | Hexagon Linux with musl 1.2.3
`mips-unknown-linux-musl` | ✓ |  | MIPS Linux with musl 1.2.3
[`mips64-openwrt-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/mips64-openwrt-linux-musl.html) | ? |  | MIPS64 for OpenWrt Linux musl 1.2.3
`mips64-unknown-linux-muslabi64` | ✓ |  | MIPS64 Linux, N64 ABI, musl 1.2.3
`mips64el-unknown-linux-muslabi64` | ✓ |  | MIPS64 (little endian) Linux, N64 ABI, musl 1.2.3
`mipsel-unknown-linux-musl` | ✓ |  | MIPS (little endian) Linux with musl 1.2.3
`powerpc-unknown-linux-musl` | ? |  | PowerPC Linux with musl 1.2.3
[`powerpc-unknown-linux-muslspe`](https://doc.rust-lang.org/rustc/platform-support/powerpc-unknown-linux-muslspe.html) | ? |  | PowerPC SPE Linux with musl 1.2.3
[`powerpc64-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/powerpc64-unknown-linux-musl.html) | ✓ | ✓ | PPC64 Linux (kernel 4.19, musl 1.2.3)
`riscv32gc-unknown-linux-musl` | ? |   | RISC-V Linux (kernel 5.4, musl 1.2.3 + RISCV32 support patches)
[`s390x-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/s390x-unknown-linux-musl.html) | ✓ |  | S390x Linux (kernel 3.2, musl 1.2.3)
`thumbv7neon-unknown-linux-musleabihf` | ? |  | Thumb2-mode Armv7-A Linux with NEON, musl 1.2.3
[`x86_64-unikraft-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/unikraft-linux-musl.html) | ✓ |   | 64-bit Unikraft with musl 1.2.3
