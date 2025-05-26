+++
path = "2025/05/26/demoting-i686-pc-windows-gnu"
title = "Demoting i686-pc-windows-gnu to Tier 2"
authors = ["Noratrieb"]
aliases = []

[extra]
team = "Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

In Rust 1.88.0, the Tier 1 target `i686-pc-windows-gnu` will be demoted to Tier 2.
As a Tier 2 Target, builds will continue to be distributed for both the standard library and the compiler.

## Background

Rust has supported Windows for a long time, with two different flavors of Windows targets: MSVC-based and GNU-based. MSVC-based targets (for example the most popular Windows target `x86_64-pc-windows-msvc`) use Microsoft’s native linker and libraries, while GNU-based targets (like `i686-pc-windows-gnu`) are built entirely from free software components like `gcc`, `ld`, and mingw-w64.

The major reason to use a GNU-based toolchain instead of the native MSVC-based one is cross-compilation and licensing. `link.exe` only runs on Windows (barring Wine hacks) and requires a license for commercial usage.

`x86_64-pc-windows-gnu` and `i686-pc-windows-gnu` are currently both Tier 1 with host tools.
The [Target Tier Policy] contains more details on what this entails, but the most important part is that tests for these targets are being run on every merged PR.
This is the highest level of support we have, and is only used for the most high value targets (the most popular Linux, Windows, and Apple targets).

The `*-windows-gnu` targets currently do not have any dedicated target maintainers.
We do not have a lot of expertise for this toolchain, and issues often aren't fixed and cause problems in CI that we have a hard time to debug.

The 32-bit version of this target is especially problematic and has significantly less usage than `x86_64-pc-windows-gnu`, which is why `i686-pc-windows-gnu` is being demoted to Tier 2.

## What changes?

After Rust 1.88.0, `i686-pc-windows-gnu` will now be Tier 2 with host tools.
For users, nothing will change immediately. Builds of both the standard library and the compiler will still be distributed by the Rust Project for use via `rustup` or alternative installation methods.

This does mean that this target will likely accumulate bugs faster in the future because of the reduced testing.

## Future

If no maintainers are found and the `*-windows-gnu` targets continue causing problems, they may be demoted further.
No concrete plans about this have been made yet.

If you rely on the `*-windows-gnu` targets and have expertise in this area, we would be very happy to have you as a target maintainer. You can check the [Target Tier Policy] for what exactly that would entail.

For more details on the motivation of the demotion, see [RFC 3771](https://rust-lang.github.io/rfcs/3771-demote-i686-pc-windows-gnu.html) which proposed this change.

[Target Tier Policy]: https://doc.rust-lang.org/nightly/rustc/target-tier-policy.html
