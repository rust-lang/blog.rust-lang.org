+++
path = "2025/09/01/rust-lld-on-1.90.0-stable"
title = "Faster linking times with 1.90.0 stable on Linux using the LLD linker"
authors = ["Rémy Rakic"]

[extra]
team = "the compiler performance working group"
team_url = "https://www.rust-lang.org/governance/teams/compiler#team-wg-compiler-performance"
+++

TL;DR: rustc will start using the LLD linker by default on the `x86_64-unknown-linux-gnu` target starting with the next stable release (1.90.0, scheduled for 2025-09-18), which should significantly reduce linking times. Test it out on beta now, and please report any encountered issues.

#### Some context

Linking time is often a big part of compilation time. When rustc needs to build a binary or a shared library, it will usually call the default linker installed on the system to do that (this can be changed on the command-line or by the target for which the code is compiled).

The linkers do an important job, with concerns about stability, backwards-compatibility and so on. For these and other reasons, on the most popular operating systems they usually are older programs, designed when computers only had a single core. So, they usually tend to be slow on a modern machine. For example, when building ripgrep 13 in debug mode on Linux, roughly half of the time is actually spent in the linker.

There are different linkers, however, and the usual advice to improve linking times is to use one of these newer and faster linkers, like LLVM's [`lld`](https://lld.llvm.org/) or Rui Ueyama's [`mold`](https://github.com/rui314/mold).

Some of Rust's wasm and aarch64 targets already use `lld` by default. When using rustup, rustc ships with a version of `lld` for this purpose. When CI builds LLVM to use in the compiler, it also builds the linker and packages it. It's referred to as `rust-lld` to avoid colliding with any `lld` already installed on the user's machine.

Since improvements to linking times are substantial, it would be a good default to use in the most popular targets. This has been discussed for a long time, for example in issues [#39915](https://github.com/rust-lang/rust/issues/39915) and [#71515](https://github.com/rust-lang/rust/issues/71515).

To expand our testing, we have enabled rustc to use `rust-lld` by default on nightly, [in May 2024](https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html). No major issues have been reported since then.

We believe we've done all the internal testing that we could, on CI, crater, on our benchmarking infrastructure and on nightly, and plan to enable `rust-lld` to be the linker used by default on `x86_64-unknown-linux-gnu` for stable builds in 1.90.0.

#### Benefits

While this also enables the compiler to use more linker features in the future, the most immediate benefit is much improved linking times.

Here are more details from the ripgrep example mentioned above: for an incremental rebuild, linking is reduced 7x, resulting in a 40% reduction in end-to-end compilation times. For a from-scratch debug build, it is a 20% improvement.

![Before/after comparison of a `ripgrep` incremental debug build](ripgrep-comparison.png)

Most binaries should see some improvements here, but it's especially significant with e.g. bigger binaries, or for incremental rebuilds, or when involving debuginfo. These usually see bottlenecks in the linker.

Here's [a link](https://perf.rust-lang.org/compare.html?start=b3e117044c7f707293edc040edb93e7ec5f7040a&end=baed03c51a68376c1789cc373581eea0daf89967&stat=instructions%3Au&tab=compile) to the complete results from our benchmarks.

#### Possible drawbacks

From our prior testing, we don't really expect issues to happen in practice. It is a drop-in replacement for the vast majority of cases, but `lld` is not _bug-for-bug_ compatible with GNU ld.

In any case, using `rust-lld` can be disabled if any problem occurs: use the `-C linker-features=-lld` flag to revert to using the system's default linker.

Some crates somehow relying on these differences could need additional link args, though we also expect this to be quite rare. Let us know if you encounter problems, by [opening an issue](https://github.com/rust-lang/rust/issues/new/choose) on GitHub.

Some of the big gains in performance come from parallelism, which could be undesirable in resource-constrained environments, or for heavy projects that are already reaching hardware limits.

#### Summary, and call for testing

rustc will use `rust-lld` on `x86_64-unknown-linux-gnu`, starting with the 1.90.0 stable release, for much improved linking times. Rust 1.90.0 will be released next month, on the 18th of September 2025.

This linker change is already available on the current beta (`1.90.0-beta.6`). To help everyone prepare for this landing on stable, please test your projects on beta and let us know if you encounter problems, by [opening an issue](https://github.com/rust-lang/rust/issues/new/choose) on GitHub.

If that happens, you can revert to the default linker with the `-C linker-features=-lld` flag. Either by adding it to the usual `RUSTFLAGS` environment variable, or to a project's [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) configuration file,
like so:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-Clinker-features=-lld"]
```
