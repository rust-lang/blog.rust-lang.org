+++
layout = "post"
date = 2024-05-17
title = "Faster linking times on nightly on Linux using `rust-lld`"
author = "Rémy Rakic"
team = "the compiler performance working group <https://www.rust-lang.org/governance/teams/compiler#team-wg-compiler-performance>"
+++

TL;DR: rustc will use `rust-lld` by default on `x86_64-unknown-linux-gnu` on nightly to
significantly reduce linking times.

#### Some context

Linking time is often a big part of compilation time. When rustc needs to build a binary or a shared
library, it will usually call the default linker installed on the system to do that (this can be
changed on the command-line or by the target for which the code is compiled).

The linkers do an important job, with concerns about stability, backwards-compatibility and so on.
For these and other reasons, on the most popular operating systems they usually are older programs,
designed when computers only had a single core. So, they usually tend to be slow on a modern
machine. For example, when building ripgrep 13 in debug mode on Linux, roughly half of the time is
actually spent in the linker.

There are different linkers, however, and the usual advice to improve linking times is to use one of
these newer and faster linkers, like LLVM's [`lld`](https://lld.llvm.org/) or Rui Ueyama's
[`mold`](https://github.com/rui314/mold).

Some of Rust's wasm and aarch64 targets already use `lld` by default. When using rustup, rustc ships
with a version of `lld` for this purpose. When CI builds LLVM to use in the compiler, it also builds
the linker and packages it. It's referred to as `rust-lld` to avoid colliding with any `lld` already
installed on the user's machine.

Since improvements to linking times are substantial, it would be a good default to use in the most
popular targets. This has been discussed for a long time, for example in issues
[#39915](https://github.com/rust-lang/rust/issues/39915) and
[#71515](https://github.com/rust-lang/rust/issues/71515), and rustc already offers nightly flags to
use `rust-lld`.

By now, we believe we've done all the internal testing that we could, on CI, crater, and our
benchmarking infrastructure. We would now like to expand testing and gather real-world feedback and
use-cases. Therefore, we will enable `rust-lld` to be the linker used by default on
`x86_64-unknown-linux-gnu` for nightly builds.

#### Benefits

While this also enables the compiler to use more linker features in the future, the most immediate
benefit is much improved linking times.

Here are more details from the ripgrep example mentioned above: linking is reduced 7x, resulting in
a 40% reduction in end-to-end compilation times.

![Before/after comparison of a `ripgrep` debug build](../../../../images/2024-05-17-enabling-rust-lld-on-linux/ripgrep-comparison.png)

Most binaries should see some improvements here, but it's especially significant with e.g. bigger
binaries, or when involving debuginfo. These usually see bottlenecks in the linker.

Here's [a
link](https://perf.rust-lang.org/compare.html?start=b3e117044c7f707293edc040edb93e7ec5f7040a&end=baed03c51a68376c1789cc373581eea0daf89967&stat=instructions%3Au&tab=compile)
to the complete results from our benchmarks.

If testing goes well, we can then stabilize using this faster linker by default for
`x86_64-unknown-linux-gnu` users, before maybe looking at other targets.

#### Possible drawbacks

From our prior testing, we don't really expect issues to happen in practice. It is a drop-in
replacement for the vast majority of cases, but `lld` is not _bug-for-bug_ compatible with GNU ld.

In any case, using `rust-lld` can be disabled if any problem occurs: use the `-Z
linker-features=-lld` flag to revert to using the system's default linker.

Some crates somehow relying on these differences could need additional link args. For example, we
saw <20 crates in the crater run failing to link because of a different default about [encapsulation
symbols](https://lld.llvm.org/ELF/start-stop-gc): these could require
`-Clink-arg=-Wl,-z,nostart-stop-gc` to match the legacy GNU ld behavior.

Some of the big gains in performance come from parallelism, which could be undesirable in
resource-constrained environments.

#### Summary

rustc will use `rust-lld` on `x86_64-unknown-linux-gnu` nightlies, for much improved linking times,
starting in tomorrow's rustup nightly (`nightly-2024-05-18`).
Let us know if you encounter problems, by [opening an
issue](https://github.com/rust-lang/rust/issues/new/choose) on GitHub.

If that happens, you can revert to the default linker with the `-Z linker-features=-lld` flag.
Either by adding it to the usual `RUSTFLAGS` environment variable, or to a project's
[`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) configuration file,
like so:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-Zlinker-features=-lld"]
```
