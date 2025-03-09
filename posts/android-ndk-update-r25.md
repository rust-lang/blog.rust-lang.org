+++
layout = "post"
date = 2023-01-09
title = "Updating the Android NDK in Rust 1.68"
author = "Android Platform Team"
description = "Modernizing Android support in Rust"
+++

We are pleased to announce that Android platform support in Rust will be
modernized in Rust 1.68 as we update the target NDK from r17 to r25.  As a
consequence the minimum supported API level will increase from 15 (Ice Cream
Sandwich) to 19 (KitKat).

In NDK r23 Android switched to using LLVM's `libunwind` for all architectures.
This meant that
1. If a project were to target NDK r23 or newer with previous versions of Rust
   [a workaround](https://github.com/rust-lang/rust/pull/85806#issuecomment-1096266946)
   would be required to redirect attempts to link against `libgcc` to instead
   link against `libunwind`.  Following this update this workaround will no
   longer be necessary.
2. If a project uses NDK r22 or older it will need to be updated to use [r23 or
   newer](https://developer.android.com/ndk/downloads).  Information about the
   layout of the NDK's toolchain can be found
   [here](https://developer.android.com/ndk/guides/other_build_systems).

Going forward the Android platform will target the most recent LTS NDK, allowing
Rust developers to access platform features sooner.  These updates should occur
yearly and will be announced in release notes.

