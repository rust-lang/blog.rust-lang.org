+++
path = "2022/07/19/Rust-1.62.1"
title = "Announcing Rust 1.62.1"
authors = ["The Rust Release Team"]
aliases = ["2022/07/19/Rust-1.62.1.html"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.62.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.62.1 with:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.62.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1621-2022-07-19

## What's in 1.62.1 stable

Rust 1.62.1 addresses a few recent regressions in the compiler and standard
library, and also mitigates a CPU vulnerability on Intel SGX.

* [The compiler fixed unsound function coercions involving `impl Trait` return types.][98608]
* [The compiler fixed an incremental compilation bug with `async fn` lifetimes.][98890]
* [Windows added a fallback for overlapped I/O in synchronous reads and writes.][98950]
* [The `x86_64-fortanix-unknown-sgx` target added a mitigation for the
  MMIO stale data vulnerability][98126], advisory [INTEL-SA-00615].

[98608]: https://github.com/rust-lang/rust/issues/98608
[98890]: https://github.com/rust-lang/rust/issues/98890
[98950]: https://github.com/rust-lang/rust/pull/98950
[98126]: https://github.com/rust-lang/rust/pull/98126
[INTEL-SA-00615]: https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00615.html

### Contributors to 1.62.1

Many people came together to create Rust 1.62.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.62.1/)
