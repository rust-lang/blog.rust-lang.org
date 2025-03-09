+++
layout = "post"
date = 2023-05-09
title = "Updating Rust's Linux musl targets"
author = "Wesley Wiser"
description = "musl targets will soon ship with musl 1.2"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++

Beginning with Rust 1.71 (slated for stable release on 2023-07-13), the various `*-linux-musl` targets will [ship][PR] with musl 1.2.3.
These targets currently use musl 1.1.24.
While musl 1.2.3 introduces some [new features][musl features], most notably [64-bit time][time64] on all platforms, it is ABI compatible with earlier musl versions.

As such, this change is unlikely to affect you.

# Updated targets

The following targets will be updated:

| Target | [Support Tier][tiers] |
| - | - |
| `aarch64-unknown-linux-musl` | Tier 2 with Host Tools |
| `x86_64-unknown-linux-musl` | Tier 2 with Host Tools |
| `arm-unknown-linux-musleabi` | Tier 2 |
| `arm-unknown-linux-musleabihf` | Tier 2 |
| `armv5te-unknown-linux-musleabi` | Tier 2 |
| `armv7-unknown-linux-musleabi` | Tier 2 |
| `armv7-unknown-linux-musleabihf` | Tier 2 |
| `i586-unknown-linux-musl` | Tier 2 |
| `i686-unknown-linux-musl` | Tier 2 |
| `mips-unknown-linux-musl` | Tier 2 |
| `mips64-unknown-linux-muslabi64` | Tier 2 |
| `mips64el-unknown-linux-muslabi64` | Tier 2 |
| `mipsel-unknown-linux-musl` | Tier 2 |
| `hexagon-unknown-linux-musl` | Tier 3 |
| `mips64-openwrt-linux-musl` | Tier 3 |
| `powerpc-unknown-linux-musl` | Tier 3 |
| `powerpc64-unknown-linux-musl` | Tier 3 |
| `powerpc64le-unknown-linux-musl` | Tier 3 |
| `riscv32gc-unknown-linux-musl` | Tier 3 |
| `riscv64gc-unknown-linux-musl` | Tier 3 |
| `s390x-unknown-linux-musl` | Tier 3 |
| `thumbv7neon-unknown-linux-musleabihf` | Tier 3 |

Note: musl 1.2.3 does not raise the minimum required Linux kernel version for any target.

# Will 64-bit time break the `libc` crate on 32-bit targets?

No, the musl project made this change carefully preserving ABI compatibility.
The `libc` crate will continue to function correctly without modification.

A future version of the `libc` crate will [update][libc PR] the definitions of time-related structures and functions to be 64-bit on all musl targets however this is blocked on the musl targets themselves first being updated.
At present, there is no anticipated date when this change will take place and care will be taken to help the Rust ecosystem transition successfully to the updated time-related definitions.

[libc PR]: https://github.com/rust-lang/libc/pull/3068
[musl features]: https://musl.libc.org/releases.html
[PR]: https://github.com/rust-lang/rust/pull/107129
[tiers]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
[time64]: https://musl.libc.org/time64.html
