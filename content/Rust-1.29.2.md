+++
path = "2018/10/12/Rust-1.29.2"
title = "Announcing Rust 1.29.2"
authors = ["The Rust Release Team"]
aliases = [
    "2018/10/12/Rust-1.29.2.html",
    "releases/1.29.2",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.29.2. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.29.2 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.29.2][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1292-2018-10-11

## What's in 1.29.2 stable

This patch release introduces a workaround to a [miscompilation bug][54462]
introduced in Rust 1.29.0. We haven't found the root cause of the bug yet, but
it showed up after a LLVM version upgrade, and it's caused by an optimization.
We disabled that optimization until the root cause is fixed.

This release also includes the `rls-preview` rustup component for Windows GNU
users, which wasn't included in the 1.29.0 release due to a build failure. We
also added safeguards in the release infrastructure to prevent stable and beta
releases with missing components for Tier 1 platform in the future.

[54462]: https://github.com/rust-lang/rust/issues/54462
