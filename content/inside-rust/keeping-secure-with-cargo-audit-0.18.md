+++
path = "inside-rust/2023/09/04/keeping-secure-with-cargo-audit-0.18"
title = "Keeping Rust projects secure with cargo-audit 0.18: performance, compatibility and security improvements"
authors = ['Sergey "Shnatsel" Davidoff']
description = "A look at the new features in cargo-audit 0.18 for ensuring dependencies are free of known vulnerabilities"
aliases = ["inside-rust/2023/09/04/keeping-secure-with-cargo-audit-0.18.html"]

[extra]
team = "the Secure Code WG"
team_url = "https://www.rust-lang.org/governance/wgs/wg-secure-code"
+++

[`cargo audit`](https://crates.io/crates/cargo-audit) checks your project's dependencies for known security vulnerabilites.

By default `cargo audit` checks on your `Cargo.lock` file, but it can also [scan compiled binaries](https://github.com/rustsec/rustsec/tree/main/cargo-audit#cargo-audit-bin-subcommand). You can install `cargo-audit` and run it against your project with the following commands:

```
$ cargo install cargo-audit
$ cargo audit
```

Both `cargo audit` and the [RustSec](https://rustsec.org/) advisory database that powers it are maintained by the [Rust Secure Code working group](https://www.rust-lang.org/governance/wgs/wg-secure-code).

## What's new in this release

### Performance

`cargo audit` now uses the [sparse crates.io index](https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html) when checking for yanked crates. This dramatically speeds up the scanning process because `cargo audit` no longer needs to download the entire crates.io index, which could take up to several minutes.

Sparse index is used by default if you are running Rust 1.70 or later, same as in Cargo itself. `cargo audit` honors [the Cargo settings for the use of sparse index](https://doc.rust-lang.org/cargo/reference/config.html#registriescrates-ioprotocol), should you need to opt out for any reason.

### Security

`cargo audit` previously relied on [OpenSSL](https://en.wikipedia.org/wiki/OpenSSL) on all platforms. In this release we have switched to [rustls](https://crates.io/crates/rustls) - a high-quality, memory-safe TLS implementation in Rust.

In contrast to OpenSSL's [history](https://www.openssl.org/news/vulnerabilities.html) of high-severity vulnerabilities, rustls has a stellar track record and eliminates entire classes vulnerabilities by construction. It has passed an independent audit with flying colors; the auditors even [noted](https://github.com/rustls/rustls/blob/main/audit/TLS-01-report.pdf) that they "had the rare pleasure of being impressed with the exceptional quality of the presented software".

Since TLS constitutes the vast majority of the attack surface of `cargo audit`, we are very excited to use a more secure TLS implementation!

### Compatibility

A number of long-standing issues are resolved thanks to switching from [libgit2](https://github.com/libgit2/libgit2) to [gitoxide](https://github.com/Byron/gitoxide) as our git implementation:

 1. [`cargo audit` can now run in Alpine Linux containers](https://github.com/rustsec/rustsec/issues/466).
 1. [Several instances of `cargo audit` running in parallel can now fetch Git repositories without issue](https://github.com/rustsec/rustsec/issues/490).
 4. [Accessing Git repositories over SSH is now supported](https://github.com/rustsec/rustsec/issues/292).
 3. [Credential helpers to access private repositories are now supported](https://github.com/rustsec/rustsec/issues/555).

## Known issues

### Limited CPU architecture support

CPU architectures other than x86 and ARM are not supported by this release. This is due to [ring](https://github.com/briansmith/ring), the cryptographic library used by rustls, not supporting other CPU architectures yet.

rustls is [in the process](https://github.com/rustls/rustls/issues/521) of adding support for other cryptographic libraries. We will consider adding support for another TLS implementation if no portable cryptographic library for rustls materializes in the near future.

In the meantime we recommend using the previous release on uncommon CPU architectures. You may also consider other tools that read `Cargo.lock` files and the RustSec advisory database, such as [Trivy](https://github.com/aquasecurity/trivy), [osv-scanner](https://github.com/google/osv-scanner) or [Dependabot](https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts).

### `cargo audit fix` is not converted

The experimental subcommand `cargo audit fix` to automatically upgrade vulnerable dependencies has existed for a while but has been disabled by default. It has **not** been converted to use gitoxide and rustls in this release, and has not benefited from any of these improvements.

We will likely [rewrite this feature from the ground up](https://github.com/rustsec/rustsec/issues/938) before enabling it by default in subsequent releases.

## Reporting issues

Due to the sweeping changes to the libraries `cargo audit` relies on for git protocol and networking there are bound to be subtle differences in behavior compared to previous versions.

If you encounter issues with this latest release, please [report it to us on Github](https://github.com/rustsec/rustsec/issues/new). Thank you!

## Acknowledgements

Thanks to [Jake Shadle](https://github.com/Jake-Shadle) who did most of the work in this release, as well as for creating the [`tame-index`](https://github.com/EmbarkStudios/tame-index) crate that enabled sparse registry support in `cargo audit`.

Thanks to [Sebastian Thiel](https://github.com/Byron) for creating [`gitoxide`](https://github.com/Byron/gitoxide) and improving it to accommodate the `cargo audit` requirements, as well as helping review the changes.
