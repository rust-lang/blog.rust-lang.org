+++
path = "2024/04/09/Rust-1.77.2"
title = "Announcing Rust 1.77.2"
authors = ["The Rust Security Response WG"]
aliases = ["2024/04/09/Rust-1.77.2.html"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.77.2. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.77.2 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.77.2

This release includes a fix for [CVE-2024-24576].

Before this release, the Rust standard library did not properly escape
arguments when invoking batch files (with the `bat` and `cmd` extensions) on
Windows using the [`Command`] API. An attacker able to control the arguments
passed to the spawned process could execute arbitrary shell commands by
bypassing the escaping.

This vulnerability is **CRITICAL** if you are invoking batch files on Windows
with untrusted arguments. No other platform or use is affected.

[You can learn more about the vulnerability in the dedicated
advisory.][advisory]

[CVE-2024-24576]: https://www.cve.org/CVERecord?id=CVE-2024-24576
[advisory]: https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html
[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

### Contributors to 1.77.2

Many people came together to create Rust 1.77.2. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.77.2/)
