+++
layout = "post"
title = "cargo-audit v0.11: Introducing the `fix` feature, yanked crate detection, and more"
author = "Tony Arcieri"
description = "Release announcement for cargo-audit v0.11 describing the new features"
team = "the Secure Code WG <https://www.rust-lang.org/governance/wgs/wg-secure-code>"
+++

[cargo-audit](https://github.com/rustsec/cargo-audit) is a command-line utility which inspects `Cargo.lock` files and compares them against the [RustSec Advisory Database](https://rustsec.org), a community database of security vulnerabilities maintained by the [Rust Secure Code Working Group](https://github.com/rust-secure-code/wg).

This post describes the new features in the 0.11 release of `cargo-audit`.

## `cargo audit fix`: automatically update vulnerable dependencies

One of our [most requested features](https://github.com/RustSec/cargo-audit/issues/23) (especially by fans of the similar [`npm audit fix`](https://docs.npmjs.com/cli/audit) command), the [new `cargo audit fix` subcommand](https://github.com/RustSec/cargo-audit#cargo-audit-fix-subcommand) will attempt to automatically update version requirements for vulnerable dependencies to non-vulnerable versions.

Note that this is an experimental new feature which isn't enabled by default. To try it out, install `cargo-audit` with the following:

```
$ cargo install cargo-audit --features=fix
```

This will perform the same audit process as `cargo audit` initially, and then attempt to apply fixes to your `Cargo.toml`:

![cargo audit fix screenshot](../../../../images/inside-rust/2020-01-23-Introducing-cargo-audit-fix-and-more/cargo-audit-fix.png)

Under the hood, it uses [cargo-edit](https://github.com/killercup/cargo-edit) (as a library) to perform modifications to your `Cargo.toml` file, using the fixed version requirements listed in the advisory to try to perform an automatic upgrade to a non-vulnerable version of a dependency for each advisory.

Note once more that this is a *new, experimental feature* and as such it's bound to have bugs. If you're worried, you can use `cargo audit fix --dry-run` to perform a dry run only. And if you do encounter bugs, please [file a bug report](https://github.com/rustsec/cargo-audit/issues).

We'd like to thank Reza Fatahi and Hanif Ariffin for their work in contributing this feature.

## Warnings for yanked crates

As you can see in the screenshot above, `cargo audit` now checks each of the crates in your `Cargo.lock` file against the [crates.io](https://crates.io) index to determine if any of them have been yanked. If they have, it will emit a warning as per above.

If you'd like for yanked crates to be a hard failure, you can run `cargo audit` with the `-D` command-line argument:

```
$ cargo audit -D
```

or if you prefer to be more explicit:

```
$ cargo audit --deny-warnings
```

## Compatibility with the new "V2" format for `Cargo.lock`

Rust 1.39 shipped support for a new [merge-friendly `Cargo.lock` format](https://github.com/rust-lang/cargo/pull/7070).

`cargo audit` consumes `Cargo.lock` directly, and while the V2 format change didn't break the core vulnerability-auditing functionality of `cargo audit`, several minor features regressed because of this, such as displaying dependency trees for vulnerable dependencies.

This release also upgrades to version 4.0 of the [`cargo-lock` crate](https://github.com/RustSec/cargo-lock), which includes full support for the V2 `Cargo.lock` format and constructs a representation of a lockfile which is the same across the V1 and V2 formats.

If you noticed dependency trees failing to display after upgrading to the V2 `Cargo.lock` format, they should now be working again!

Thanks for reading, and we hope you enjoy `cargo-audit` 0.11!

