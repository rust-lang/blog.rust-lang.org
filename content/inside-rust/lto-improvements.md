+++
path = "inside-rust/2020/06/29/lto-improvements"
title = "Disk space and LTO improvements"
authors = ["Eric Huss"]
description = "Disk space and LTO improvements"
aliases = ["inside-rust/2020/06/29/lto-improvements.html"]

[extra]
team = "the Cargo team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

Thanks to the work of [Nicholas Nethercote] and [Alex Crichton], there have been some recent improvements that reduce the size of compiled libraries, and improves the compile-time performance, particularly when using LTO. This post dives into some of the details of what changed, and an estimation of the benefits.

These changes have been added incrementally over the past three months, with the latest changes landing just a few days ago on the nightly channel. The bulk of the improvements will be found in the 1.46 stable release (available on 2020-08-27). It would be great for any projects that use LTO to test it out on the nightly channel (starting from the 2020-06-13 release) and report any issues that arise.

[Nicholas Nethercote]: https://github.com/nnethercote
[Alex Crichton]: https://github.com/alexcrichton/

## Background

When compiling a library, `rustc` saves the output in an `rlib` file which is an [archive file]. This has historically contained the following:

* Object code, which is the result of code generation. This is used during regular linking.
* [LLVM bitcode], which is a binary representation of LLVM's intermediate representation. This can be used for [Link Time Optimization] (LTO).
* Rust-specific metadata, which covers [a wide range of data][metadata] about the crate.

LTO is an optimization technique that can perform whole-program analysis. It analyzes all of the bitcode from every library at once, and performs optimizations and code generation. `rustc` supports several forms of LTO:

* Fat LTO. This performs "full" LTO, which can take a long time to complete and may require a significant amount of memory.
* [Thin LTO]. This LTO variant supports much better parallelism than fat LTO. It can achieve similar performance improvements as fat LTO (sometimes even better!), while taking much less total time by taking advantage of more CPUs.
* Thin-local LTO. By default, `rustc` will split a crate into multiple "codegen units" so that they can be processed in parallel by LLVM. But this prevents some optimizations as code is separated into different codegen units, and is handled independently. Thin-local LTO will perform thin LTO across the codegen units within a single crate, bringing back some optimizations that would otherwise be lost by the separation. This is `rustc`'s default behavior if opt-level is greater than 0.

## What has changed

Changes have been made to both `rustc` and Cargo to control which libraries should include object code and which should include bitcode based on the project's [profile] LTO settings. If the project is not using LTO, then Cargo will instruct `rustc` to not place bitcode in the rlib files, which should reduce the amount of disk space used. This may have a small improvement in performance since `rustc` no longer needs to compress and write out the bitcode.

If the project is using LTO, then Cargo will instruct `rustc` to not place object code in the rlib files, avoiding the expensive code generation step. This should improve the build time when building from scratch, and reduce the amount of disk space used.

Two `rustc` flags are now available to control how the rlib is constructed:

* [`-C linker-plugin-lto`] causes `rustc` to only place bitcode in the `.o` files, and skips code generation. This flag was [originally added][linker-plugin-lto-track] to support cross-language LTO. Cargo now uses this when the rlib is only intended for use with LTO.
* [`-C embed-bitcode=no`] causes `rustc` to avoid placing bitcode in the rlib altogether. Cargo uses this when LTO is not being used, which reduces some disk space usage.

Additionally, the method in which bitcode is embedded in the rlib has changed. Previously, `rustc` would place compressed bitcode as a `.bc.z` file in the rlib archive. Now, the bitcode is placed as an uncompressed section within each `.o` [object file] in the rlib archive. This can sometimes be a small performance benefit, because it avoids cost of compressing the bitcode, and sometimes can be slower due to needing to write more data to disk. This change helped simplify the implementation, and also matches the behavior of clang's `-fembed-bitcode` option (typically used with Apple's iOS-based operating systems).

## Improvements

The following is a summary of improvements observed on a small number of real-world projects of small and medium size. Improvements of a project will depend heavily on the code, optimization settings, operating system, environment, and hardware. These were recorded with the 2020-06-21 nightly release on Linux with parallel job settings between 2 and 32.

The performance wins for debug builds were anywhere from 0% to 4.7% faster. Larger binary crates tended to fare better than smaller library crates.

LTO builds were recorded anywhere from 4% to 20% faster. Thin LTO fared consistently better than fat LTO.

The number of parallel jobs also had a large impact on the amount of improvement. Lower parallel job counts saw substantially more benefit than higher ones. A project built with `-j2` can be 20% faster, whereas the same project at `-j32` would only be 1% faster. Presumably this is because the code-generation phase benefits from higher concurrency, so it was taking a relatively smaller total percentage of time.

The overall target directory size is typically 20% to 30% smaller for debug builds. LTO builds did not see as much of an improvement, ranging from 11% to 19% smaller.

## More details

Nicholas Nethercote wrote about the journey to implement these changes at <https://blog.mozilla.org/nnethercote/2020/04/24/how-to-speed-up-the-rust-compiler-in-2020/>. It took several PRs across `rustc` and Cargo to make this happen:

- [#66598](https://github.com/rust-lang/rust/pull/66598) — The original approach, that was decided to be too simplistic.
- [#66961](https://github.com/rust-lang/rust/issues/66961) — The issue outlining the strategy that was employed.
- [#70289](https://github.com/rust-lang/rust/pull/70289)
  [#70297](https://github.com/rust-lang/rust/pull/70297)
  [#70345](https://github.com/rust-lang/rust/pull/70345)
  [#70384](https://github.com/rust-lang/rust/pull/70384)
  [#70644](https://github.com/rust-lang/rust/pull/70644)
  [#70729](https://github.com/rust-lang/rust/pull/70729)
  [#71374](https://github.com/rust-lang/rust/pull/71374)
  [#71716](https://github.com/rust-lang/rust/pull/71716)
  [#71754](https://github.com/rust-lang/rust/pull/71754) — A series of refactorings to prepare for the new behavior and do some cleanup.
- [#71323](https://github.com/rust-lang/rust/pull/71323) — Introduced a new flag to control whether or not bitcode is embedded.
- [#70458](https://github.com/rust-lang/rust/pull/70458) [#71528](https://github.com/rust-lang/rust/pull/71528) — Switched how LLVM bitcode is embedded.
- [#8066](https://github.com/rust-lang/cargo/pull/8066)
  [#8192](https://github.com/rust-lang/cargo/pull/8192)
  [#8204](https://github.com/rust-lang/cargo/pull/8204)
  [#8226](https://github.com/rust-lang/cargo/pull/8226)
  [#8254](https://github.com/rust-lang/cargo/pull/8254)
  [#8349](https://github.com/rust-lang/cargo/pull/8349) — The series of Cargo changes to implement the new functionality.

## Conclusion

Although this is a conceptually simple change (LTO=bitcode, non-LTO=object code), it took quite a bit of preparation and work to make it happen. There were many edge cases and platform-specific behaviors to consider, and testing to perform. And, of course, the obligatory bike-shedding over the names of new command-line flags. This resulted in quite a substantial improvement in performance, particularly for LTO builds, and a huge improvement in disk space usage. Thanks to all of those that helped to make this happen!

[archive file]: https://en.wikipedia.org/wiki/Ar_(Unix)
[LLVM bitcode]: https://llvm.org/docs/BitCodeFormat.html
[Link Time Optimization]: https://llvm.org/docs/LinkTimeOptimization.html
[Thin LTO]: http://blog.llvm.org/2016/06/thinlto-scalable-and-incremental-lto.html
[profile]: https://doc.rust-lang.org/cargo/reference/profiles.html
[object file]: https://en.wikipedia.org/wiki/Object_file
[`-C linker-plugin-lto`]: https://doc.rust-lang.org/nightly/rustc/codegen-options/#linker-plugin-lto
[`-C embed-bitcode=no`]: https://doc.rust-lang.org/nightly/rustc/codegen-options/#embed-bitcode
[metadata]: https://github.com/rust-lang/rust/blob/0b66a89735305ebac93894461559576495ab920e/src/librustc_metadata/rmeta/mod.rs#L172-L214
[linker-plugin-lto-track]: https://github.com/rust-lang/rust/issues/49879
