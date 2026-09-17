+++
path = "9999/12/31/demoting-i686-windows-targets-to-std-only"
title = "Demoting i686 Windows targets to std-only"
authors = ["Mateusz Mikuła"]

[extra]
team = "the Compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler#team-compiler"
+++

With Rust 1.100.0, the Tier 1 with host tools target `i686-pc-windows-msvc` will be demoted to Tier 1 without host tools. Respectively, the Tier 2 with host tools target `i686-pc-windows-gnu` will be demoted to Tier 2 without host tools.

Builds of the standard library will continue to be distributed, but host tools such as the compiler will be no longer available. `i686-pc-windows-msvc` as a Tier 1 target still undergoes CI testing.

## Background

Desktop and Server 32-bit only x86 CPUs are no longer sold for over 15 years, and general 32-bit Windows support has ended in October 2025. This means that the development platforms these targets are meant for hardly exist these days, and even if they do exist they typically aren't capable enough for development.

Even on the modern x86_64 hardware, building i686 Windows toolchains has proven to be problematic. We have encountered compiler binaries crashing when built with the i686 MSVC target, and the GNU C++ toolchain failing with OOMs during LLVM build.

Considering all these things, cross-compiling these targets from a better supported one is what we have found to be the best solution forward. As part of that, we stopped producing host tools for these targets. For the time being, the prebuilt standard library is still available, and in case of `i686-pc-windows-msvc` still tested on CI.

## What Changes?

After Rust 1.100, it will no longer be possible to install toolchains on 32-bit Windows hosts. We recommend cross-compiling from a 64-bit Windows host instead. Other 32-bit platforms that we already provide host tools for are not impacted by this change.

For more details about these demotions, see [RFC 3999](https://github.com/rust-lang/rfcs/blob/51783df9a76c355de7ceebeae101cba47f8ca463/text/3999-std-only-i686-msvc.md) for `i686-pc-windows-msvc` demotion, and [MCP 1020](https://github.com/rust-lang/compiler-team/issues/1020) for `i686-pc-windows-gnu` demotion.
