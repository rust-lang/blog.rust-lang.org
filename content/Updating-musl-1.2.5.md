+++
path = "2025/12/05/Updating-musl-1.2.5"
title = "Updating Rust's Linux musl targets to 1.2.5"
authors = ["Aria Desires"]
description = "musl targets will soon ship with musl 1.2.5"

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

# Updating Rust's Linux musl targets to 1.2.5

Beginning with Rust 1.93 (slated for stable release on 2026-01-22), the various `*-linux-musl` targets will all [ship](https://github.com/rust-lang/rust/pull/142682) with musl 1.2.5. This primarily affects static musl builds for `x86_64`, `aarch64`, and `powerpc64le` which bundled musl 1.2.3. This update comes with [several fixes and improvements](https://musl.libc.org/releases.html), and a breaking change that affects the Rust ecosystem.

For the Rust ecosystem, the primary motivation for this update is to receive major improvements to musl's DNS resolver which shipped in 1.2.4 and received bug fixes in 1.2.5. When using `musl` targets for static linking, this should make portable linux binaries that do networking more reliable, particularly in the face of large DNS records and recursive nameservers.

However, 1.2.4 also comes with a breaking change: [the removal of several legacy compatibility symbols that the Rust libc crate was using](https://github.com/rust-lang/libc/issues/2934). A fix for this [was shipped in libc 0.2.146 in June 2023 (2 years ago)](https://github.com/rust-lang/libc/pull/2935), and we have been waiting for newer versions of the libc crate to propagate throughout the ecosystem before shipping the musl update.

A crater run in July 2024 found only about 2.4% of Rust projects were still affected. [A crater run in June 2025 found 1.5% of Rust projects were affected](https://github.com/rust-lang/rust/pull/142682#issuecomment-3002514461). Most of that change is from crater analyzing More Rust Projects. The absolute amount of broken projects went down by 15% while the absolute amount of analyzed projects went up by 35%.

At this point we expect there will be minimal breakage, and most breakage should be resolved by a `cargo update`. We believe this update shouldn't be held back any longer, as it contains critical fixes for the musl target.

Manual inspection of some of the affected projects indicates they largely haven't run `cargo update` in 2 years, often because they haven't had any changes in 2 years. Analysis suggests this Rust release will be a devastating blow to the thriving ecosystem of Rust gamedev projects that you started 3 years ago and will definitely get back to this year.

Build failures from this change will typically look like "some \`extern\` functions couldn't be found; some native libraries may need to be installed or have their path specified", often specifically for "undefined reference to \`open64'", often while trying to build very old versions of the `getrandom` crate (hence the decimation of abandoned gamedev projects in particular):

<details><summary>Example Build Failure</summary>

```
[INFO] [stderr]    Compiling guess_the_number v0.1.0 (/opt/rustwide/workdir)
[INFO] [stdout] error: linking with `cc` failed: exit status: 1
[INFO] [stdout]   |
[INFO] [stdout]   = note:  "cc" "-m64" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/rcrt1.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crti.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtbeginS.o" "/tmp/rustcMZMWZW/symbols.o" "<2 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "/opt/rustwide/target/x86_64-unknown-linux-musl/debug/deps/{librand-bff7d8317cf08aa0.rlib,librand_chacha-612027a3597e9138.rlib,libppv_lite86-742ade976f63ace4.rlib,librand_core-be9c132a0f2b7897.rlib,libgetrandom-dc7f0d82f4cb384d.rlib,liblibc-abed7616303a3e0d.rlib,libcfg_if-66d55f6b302e88c8.rlib}.rlib" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*}.rlib" "-lunwind" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{libcfg_if-*,liblibc-*}.rlib" "-lc" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/{librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "-L" "/tmp/rustcMZMWZW/raw-dylibs" "-Wl,-Bdynamic" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-nostartfiles" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib" "-o" "/opt/rustwide/target/x86_64-unknown-linux-musl/debug/deps/guess_the_number-41a068792b5f051e" "-Wl,--gc-sections" "-static-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtendS.o" "<sysroot>/lib/rustlib/x86_64-unknown-linux-musl/lib/self-contained/crtn.o"
[INFO] [stdout]   = note: some arguments are omitted. use `--verbose` to show all linker arguments
[INFO] [stdout]   = note: /usr/bin/ld: /opt/rustwide/target/x86_64-unknown-linux-musl/debug/deps/libgetrandom-dc7f0d82f4cb384d.rlib(getrandom-dc7f0d82f4cb384d.getrandom.828c5c30a8428cf4-cgu.0.rcgu.o): in function `getrandom::util_libc::open_readonly':
[INFO] [stdout]           /opt/rustwide/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.8/src/util_libc.rs:150:(.text._ZN9getrandom9util_libc13open_readonly17hdc55d6ead142a889E+0xbc): undefined reference to `open64'
[INFO] [stdout]           collect2: error: ld returned 1 exit status
[INFO] [stdout]           
[INFO] [stdout]   = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
[INFO] [stdout]   = note: use the `-l` flag to specify native libraries to link
[INFO] [stdout]   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stderr] error: could not compile `guess_the_number` (bin "guess_the_number") due to 1 previous error
```

</details>



# Updated targets

All Rust musl targets that bundle a copy of musl now bundle 1.2.5. All Rust musl targets now require musl 1.2.5 at a minimum.

The mostly only actually impacts the three "Tier 2 With Host Tools" musl targets which were pinned to musl 1.2.3:
* `aarch64-unknown-linux-musl`
* `x86_64-unknown-linux-musl`
* `powerpc64le-unknown-linux-musl`

The fourth target at this level of support, `loongarch64-unknown-linux-musl`, is so new that it was always on musl 1.2.5.

[Due to an apparent configuration oversight with `crosstool-ng`](https://github.com/rust-lang/rust/pull/142682#issuecomment-3358724267), all other targets were already bundling musl 1.2.5. These targets were [silently upgraded to musl 1.2.4 in Rust 1.74.0](https://github.com/rust-lang/rust/commit/f99fdac3df5f97287857786bcce9660250009cbf) and [silently upgraded to musl 1.2.5 in Rust 1.86](https://github.com/rust-lang/rust/commit/ed2823c6efec9845d6e35ee7717f2e65099ab1c2). This oversight has been rectified and all targets have been pinned to musl 1.2.5 to prevent future silent upgrades (but hey, no one noticing bodes well for the ecosystem impact of this change). Their documentation has now been updated to reflect the fact that bundling 1.2.5 is actually intentional, and that 1.2.5 is now considered a minimum requirement.

Here are all the updated definitions:

## Tier 2 with Host Tools

target | notes
-------|-------
[`aarch64-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/aarch64-unknown-linux-musl.html) | ARM64 Linux with musl 1.2.5
[`powerpc64le-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/powerpc64le-unknown-linux-musl.html) | PPC64LE Linux (kernel 4.19, musl 1.2.5)
`x86_64-unknown-linux-musl` | 64-bit Linux with musl 1.2.5


## Tier 2 without Host Tools


target | std | notes
-------|:---:|-------
`arm-unknown-linux-musleabi` | ✓ | Armv6 Linux with musl 1.2.5
`arm-unknown-linux-musleabihf` | ✓ | Armv6 Linux with musl 1.2.5, hardfloat
`armv5te-unknown-linux-musleabi` | ✓ | Armv5TE Linux with musl 1.2.5
`armv7-unknown-linux-musleabi` | ✓ | Armv7-A Linux with musl 1.2.5
`armv7-unknown-linux-musleabihf` | ✓ | Armv7-A Linux with musl 1.2.5, hardfloat
`i586-unknown-linux-musl` | ✓ | 32-bit Linux (musl 1.2.5, original Pentium)
`i686-unknown-linux-musl` | ✓ | 32-bit Linux with musl 1.2.5 (Pentium 4)
[`riscv64gc-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/riscv64gc-unknown-linux-musl.html) | ✓ | RISC-V Linux (kernel 4.20+, musl 1.2.5)

## Tier 3


target | std | host | notes
-------|:---:|:----:|-------
[`hexagon-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/hexagon-unknown-linux-musl.html) | ✓ | | Hexagon Linux with musl 1.2.5
`mips-unknown-linux-musl` | ✓ |  | MIPS Linux with musl 1.2.5
[`mips64-openwrt-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/mips64-openwrt-linux-musl.html) | ? |  | MIPS64 for OpenWrt Linux musl 1.2.5
`mips64-unknown-linux-muslabi64` | ✓ |  | MIPS64 Linux, N64 ABI, musl 1.2.5
`mips64el-unknown-linux-muslabi64` | ✓ |  | MIPS64 (little endian) Linux, N64 ABI, musl 1.2.5
`mipsel-unknown-linux-musl` | ✓ |  | MIPS (little endian) Linux with musl 1.2.5
`powerpc-unknown-linux-musl` | ? |  | PowerPC Linux with musl 1.2.5
[`powerpc-unknown-linux-muslspe`](https://doc.rust-lang.org/rustc/platform-support/powerpc-unknown-linux-muslspe.html) | ? |  | PowerPC SPE Linux with musl 1.2.5
[`powerpc64-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/powerpc64-unknown-linux-musl.html) | ✓ | ✓ | PPC64 Linux (kernel 4.19, musl 1.2.5)
`riscv32gc-unknown-linux-musl` | ? |   | RISC-V Linux (kernel 5.4, musl 1.2.5 + RISCV32 support patches)
[`s390x-unknown-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/s390x-unknown-linux-musl.html) | ✓ |  | S390x Linux (kernel 3.2, musl 1.2.5)
`thumbv7neon-unknown-linux-musleabihf` | ? |  | Thumb2-mode Armv7-A Linux with NEON, musl 1.2.5
[`x86_64-unikraft-linux-musl`](https://doc.rust-lang.org/rustc/platform-support/unikraft-linux-musl.html) | ✓ |   | 64-bit Unikraft with musl 1.2.5
