+++
path = "2026/04/04/docsrs-only-default-targets"
title = "docs.rs: building fewer targets by default"
authors = ["Denis Cornehl"]

[extra]
team = "the docs.rs team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-docs-rs"
+++

# Building fewer targets by default

On **2026-05-01**, docs.rs will make a **breaking** change to its build
behavior.

Today, if a crate does not define a `targets` list in its
[docs.rs metadata][about-metadata], docs.rs builds documentation for a default
list of five targets.

Starting on **2026-05-01**, docs.rs will instead build documentation for only
the default target unless additional targets are requested explicitly.

This is the next step in a change we first introduced in 2020, when docs.rs
added support for opting into fewer build targets. Most crates do not compile
different code for different targets, so building fewer targets by default is a
better fit for most releases. It also reduces build times and saves resources on
docs.rs.

This change only affects:

1. new releases
2. rebuilds of old releases

## How is the default target chosen?

If you do not set `default-target`, docs.rs uses the target of its build
servers: `x86_64-unknown-linux-gnu`.

You can override that by setting `default-target` in your
[docs.rs metadata][about-metadata]:

```toml
[package.metadata.docs.rs]
default-target = "x86_64-apple-darwin"
```

## How do I build documentation for additional targets?

If your crate needs documentation to be built for more than the default target,
define the full list explicitly in your `Cargo.toml`:

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

When `targets` is set, docs.rs will build documentation for exactly those
targets.

docs.rs still supports any target available in the Rust toolchain. Only the
default behavior is changing.

[about-metadata]: https://docs.rs/about/metadata
