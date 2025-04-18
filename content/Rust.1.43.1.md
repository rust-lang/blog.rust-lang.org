+++
path = "2020/05/07/Rust.1.43.1"
title = "Announcing Rust 1.43.1"
authors = ["The Rust Release Team"]
aliases = [
    "2020/05/07/Rust.1.43.1.html",
    "releases/1.43.1",
]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.43.1.
Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.43.1 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.43.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1431-2020-05-07


## What's in Rust 1.43.1

Rust 1.43.1 addresses two regressions introduced in the 1.43.0 stable release, and updates the OpenSSL version used by Cargo.

### Fixed undetectable CPU features

Rust 1.27.0 introduced support for detecting x86 CPU features in the standard library, thanks to the [`is_x86_feature_detected!`][feat-detect] macro. Due to an internal refactoring, Rust 1.43.0 prevented the detection of features that can't be used on stable yet (such as AVX-512), even though detecting them was allowed in the past. Rust 1.43.1 fixes this regression. More information on the regression in available in [issue #71473][rust/71473].

[feat-detect]: https://doc.rust-lang.org/stable/std/macro.is_x86_feature_detected.html
[rust/71473]: https://github.com/rust-lang/rust/issues/71473

### Fixed broken `cargo package --list`

Rust 1.43.0 broke support for listing the files included in packages published with Cargo, when inside a workspace with path dependencies or unpublished versions. A fix for the issue is included in Rust 1.43.1. More information on the bug is available in [Cargo issue #8151][cargo/8151].

[cargo/8151]: https://github.com/rust-lang/cargo/issues/8151

### OpenSSL updated to 1.1.1g

OpenSSL, one of the dependencies of Cargo, recently released a [security advisory][CVE-2020-1967]. Unfortunately we were not able to include the fix in time for Rust 1.43.0, so we upgraded OpenSSL in Rust 1.43.1. We have no evidence this vulnerability could compromise the security of Cargo users (if you do, [please follow our security policy][security]).

[CVE-2020-1967]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2020-1967
[security]: https://www.rust-lang.org/policies/security

## Contributors to 1.43.1

Many people came together to create Rust 1.43.1.
We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.43.1/)
