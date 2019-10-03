---
layout: post
title: "Keeping Rust projects secure with cargo-audit 0.9: dependency trees, core advisories, unmaintained crates"
author: Tony Arcieri
description: "A look at the new features in cargo-audit 0.9 for ensuring dependencies are free of security advisories"
team: the Secure Code WG <https://www.rust-lang.org/governance/wgs/wg-secure-code>
---

[cargo-audit](https://github.com/rustsec/cargo-audit) is a command-line utility which inspects `Cargo.lock` files and compares them against the [RustSec Advisory Database](https://rustsec.org), a community database of security vulnerabilities maintained by the [Rust Secure Code Working Group](https://github.com/rust-secure-code/wg).

You can install `cargo-audit` and run it against your project with the following commands:

```
$ cargo install cargo-audit
$ cargo audit
```

The 0.9 release of `cargo-audit` includes a number of new features we hope make it more useful, including:

- UI improvements for understanding how vulnerabilities apply to your project
- Warnings for unmaintained crate dependencies (with an option to hard fail)
- Tracking of vulnerabilities in the Rust language itself

## UI improvements: dependency trees

Previously, when `cargo-audit` found a vulnerability in a project, it would display advisory information without any context as to how a particular vulnerable dependency is included in your project, making it difficult to determine what action needs to be taken to resolve the vulnerability.

The latest release prints an inverse dependency tree (ala the excellent [cargo-tree](https://github.com/sfackler/cargo-tree) crate) for each advisory showing how a vulnerable dependency is included in your project:

![cargo audit with dependency tree](https://raw.githubusercontent.com/RustSec/cargo-audit/a840f7b/screenshot.png)

In future versions of `cargo-audit` we [hope to add a `cargo audit fix` command](https://github.com/RustSec/cargo-audit/issues/23) ala `npm audit fix` which can either automatically update the necessary dependencies or provide instructions on how to do so. If that feature interests you and you'd like to contribute, [we're looking for help](https://github.com/RustSec/cargo-audit/issues/23)!

## New feature: unmaintained crate warnings

This release added the notion of [informational advisories](https://github.com/RustSec/rustsec-crate/pull/75) - advisories which don't directly represent a security vulnerability, but may contain potentially security-relevant information. The primary intended use for this feature is [providing warnings for unmaintained crates](https://github.com/RustSec/advisory-db/issues/173).

A recent study, [Small World with High Risks: A Study of Security Threats in the npm Ecosystem](https://www.usenix.org/system/files/sec19-zimmermann.pdf), showed that unmaintained npm packages pose a high risk to that ecosystem:

> Our results provide evidence that npm suffers from single points of failure and that unmaintained packages threaten large code bases

Rust is in a similar boat with some high profile crates, [such as the `term` crate](https://github.com/Stebalien/term/issues/93) (downloaded 8,000 times a day), are unmaintained. By [tracking information about unmaintained crates in the RustSec Advisory Database](https://github.com/RustSec/advisory-db/issues/173), we hope to improve visibility on these crates, either by helping people discover "successor" crates they should switch to, or putting potential volunteer maintainers in touch with authors interested in handing crates off. When those handoffs happen, we can mark unmaintained crate advisories as obsolete (while still giving interested security researchers a list of crates to keep an eye on for potential [software supply chain attacks](https://blog.npmjs.org/post/180565383195/details-about-the-event-stream-incident)).

If you have an unmaintained crate you'd like us to create an advisory for, or know of a third party unmaintained crate and would like to provide information about potential alternatives, please [leave a comment in the RustSec/advisory-db#173 GitHub issue](https://github.com/RustSec/advisory-db/issues/173) and we can file an advisory for it.

For now, unmaintained crates are surfaced as warnings, with some brief information available about each one and a link to click to read more. If you'd like for unmaintained crate advisories to be considered errors (so `cargo-audit` exits with a non-zero status for e.g. CI purposes), run:

```
$ cargo audit -D
```

or if you prefer to be more explicit:

```
$ cargo audit --deny-warnings
```

## Tracking Rust language vulnerabilities

Previously the [RustSec Advisory Database](https://rustsec.org) only tracked information about vulnerable crates published through [crates.io](https://crates.io). Starting with this release, however, we are also indexing advisories for vulnerabilities in the Rust language's core ecosystem components, including `std`, `cargo`, and `rustdoc`. We've now indexed the following vulnerabilities:

- [CVE-2018-1000622: rustdoc: Uncontrolled search path element vulnerability in rustdoc plugins](https://rustsec.org/advisories/CVE-2018-1000622.html)
- [CVE-2018-1000657: std: Buffer overflow vulnenrability in `VecDeque::reserve()`](https://rustsec.org/advisories/CVE-2018-1000657.html)
- [CVE-2018-1000810: std: Buffer overflow vulnerability in `str::repeat()`](https://rustsec.org/advisories/CVE-2018-1000810.html)
- [CVE-2019-12083: std: Memory safety vulnerabilities arising from `Error::type_id`](https://rustsec.org/advisories/CVE-2019-12083.html)
- [CVE-2019-16760: cargo: Cargo prior to Rust 1.26.0 may download the wrong dependency](https://rustsec.org/advisories/CVE-2019-16760.html)

We are [interested in potentially surfacing information about these advisories via cargo-audit](https://github.com/RustSec/cargo-audit/issues/140), e.g. optionally detecting if the currently active Rust toolchain is vulnerable. If that interests you, we're also looking for help on this issue!

Thanks for reading, and we hope you enjoy `cargo-audit` 0.9!

