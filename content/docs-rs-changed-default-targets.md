+++
path = "2025/10/15/docsrs-changed-default-targets"
title = "docs.rs: changed default targets"
authors = ["Denis Cornehl"]

[extra]
team = "the docs.rs team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-docs-rs"
+++

With Apple's transition from x86_64 to its own silicon (ARM64), the Rust project
has updated its platform support tiers.

Consequently, the `aarch64-apple-darwin` target has been promoted to Tier 1,
while the `x86_64-apple-darwin` target has been demoted to Tier 2. You can read
more about this change in
[RFC 3671](https://github.com/rust-lang/rfcs/pull/3671) and
[RFC 3841](https://github.com/rust-lang/rfcs/pull/3841).

To align with this ecosystem shift, docs.rs is updating its default build
targets.

Crates can provide [docs.rs metadata](https://docs.rs/about/metadata) to specify
which targets to build for. If this metadata is not provided, docs.rs uses a
default list of targets. We are now switching the default macOS target in this
list from `x86_64-apple-darwin` to `aarch64-apple-darwin`.

The new default target list is:

- `x86_64-unknown-linux-gnu`
- `aarch64-apple-darwin` (previously: `x86_64-apple-darwin`)
- `x86_64-pc-windows-msvc`
- `i686-unknown-linux-gnu`
- `i686-pc-windows-msvc`

## If your crate requires the old target list

If your crate needs to be built for the previous default target list, you can
explicitly define it using [docs.rs metadata](https://docs.rs/about/metadata) in
your `Cargo.toml`:

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
