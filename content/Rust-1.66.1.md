+++
path = "2023/01/10/Rust-1.66.1"
title = "Announcing Rust 1.66.1"
authors = ["The Rust Release Team"]
aliases = [
    "2023/01/10/Rust-1.66.1.html",
    "releases/1.66.1",
]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.66.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.66.1 with:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.66.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1661-2023-01-10

## What's in 1.66.1 stable

Rust 1.66.1 fixes Cargo not verifying SSH host keys when cloning dependencies
or registry indexes with SSH. This security vulnerability is tracked as
[CVE-2022-46176], and you [can find more details in the advisory][advisory].

[CVE-2022-46176]: https://www.cve.org/CVERecord?id=CVE-2022-46176
[advisory]: https://blog.rust-lang.org/2023/01/10/cve-2022-46176.html

### Contributors to 1.66.1

Many people came together to create Rust 1.66.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.66.1/)
