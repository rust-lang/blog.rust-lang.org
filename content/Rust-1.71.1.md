+++
path = "2023/08/03/Rust-1.71.1"
title = "Announcing Rust 1.71.1"
authors = ["The Rust Release Team"]
aliases = ["2023/08/03/Rust-1.71.1.html"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.71.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.71.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.71.1 stable

Rust 1.71.1 fixes Cargo not respecting the umask when extracting dependencies,
which could allow a local attacker to edit the cache of extracted source code
belonging to another local user, potentially executing code as another user.
This security vulnerability is tracked as [CVE-2023-38497], and you can read
more about it [on the advisory we published earlier today][advisory]. We
recommend all users to update their toolchain as soon as possible.

Rust 1.71.1 also addresses several regressions introduced in Rust 1.71.0,
including bash completion being broken for users of Rustup, and the
`suspicious_double_ref_op` being emitted when calling `borrow()` even though it
shouldn't.

You can find more detailed information on the specific regressions, and other
minor fixes, in the [release notes].

[CVE-2023-38497]: https://www.cve.org/CVERecord?id=CVE-2023-38497
[advisory]: https://blog.rust-lang.org/2023/08/03/cve-2023-38497.html
[release notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1711-2023-08-03

### Contributors to 1.71.1

Many people came together to create Rust 1.71.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.71.1/)
