+++
path = "2025/10/16/docsrs-changed-default-targets"
title = "docs.rs: changed default targets"
authors = ["Denis Cornehl"]

[extra]
team = "the docs.rs team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-docs-rs"
+++

# Changes to default build targets on docs.rs

This post announces two changes to the list of default targets used to build
documentation on docs.rs.

Crate authors can specify a custom list of targets using
[docs.rs metadata in `Cargo.toml`](https://docs.rs/about/metadata). If this
metadata is not provided, docs.rs falls back to a default list. We are updating
this list to better reflect the current state of the Rust ecosystem.

## Apple silicon (ARM64) replaces x86_64

Reflecting Apple's transition from x86_64 to its own ARM64 silicon, the Rust
project has updated its platform support tiers. The `aarch64-apple-darwin`
target is now Tier 1, while `x86_64-apple-darwin` has moved to Tier 2. You can
read more about this in [RFC 3671](https://github.com/rust-lang/rfcs/pull/3671)
and [RFC 3841](https://github.com/rust-lang/rfcs/pull/3841).

To align with this, docs.rs will now use `aarch64-apple-darwin` as the default
target for Apple platforms instead of `x86_64-apple-darwin`.

## Linux ARM64 replaces 32-bit x86

Support for 32-bit `i686` architectures is declining, and major Linux
distributions have begun to phase it out.

Consequently, we are replacing the `i686-unknown-linux-gnu` target with
`aarch64-unknown-linux-gnu` in our default set.

## New default target list

The updated list of default targets is:

- `x86_64-unknown-linux-gnu`
- `aarch64-apple-darwin` (replaces `x86_64-apple-darwin`)
- `x86_64-pc-windows-msvc`
- `aarch64-unknown-linux-gnu` (replaces `i686-unknown-linux-gnu`)
- `i686-pc-windows-msvc`

## Opting out

If your crate requires the previous default target list, you can explicitly
define it in your `Cargo.toml`:

```toml
[package.metadata.docs.rs]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "i686-unknown-linux-gnu",
    "i686-pc-windows-msvc"
]
```

Note that docs.rs continues to support any target available in the Rust
toolchain; only the _default_ list has changed.
