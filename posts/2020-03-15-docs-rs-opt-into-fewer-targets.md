---
layout: post
title: "docs.rs now allows you to choose your build targets"
author: Joshua Nelson
team: the docs.rs team <https://www.rust-lang.org/governance/teams/dev-tools#docs-rs>
---

Recently, [docs.rs] added a feature that allows crates to opt-out of building on all targets.
If you don't need to build on all targets, you can enable this feature
to allow reduce your queue times and storage costs.

## What does the feature do?

By default, docs.rs builds all crates published to [crates.io] for [every tier one target][metadata].
However, most crates have the same content on all target.
Of the platform-dependent crates, almost all target a single platform,
and do not need to be built on other targets.
For example, [`winapi`] only has documentation on the `x86_64-pc-windows-msvc`
and `i686-pc-windows-msvc` targets, and is blank on all others.

This feature allows you to request building only on specific targets.
For example, [`winapi`] could opt into only building windows targets
by putting the following in its `Cargo.toml`:

```toml
[package.metadata.docs.rs]
# This also sets the default target to `x86_64-pc-windows-msvc`
targets = ["x86_64-pc-windows-msvc", "i686-pc-windows-msvc"]
```

If you only need a single target, it's even simpler:

```toml
[package.metadata.docs.rs]
# This builds only the default target (usually `x86_64-unknown-linux-gnu`)
targets = []
```

See the [docs.rs documentation][metadata] for more details about how to opt-in.

## How does this help my crate?

Instead of building for every tier-one target, you can build for only a single target,
reducing your documentation times by a factor of 6.
This can especially help large crates or projects with many crates
that take several hours to document.

## How does this help docs.rs?

Building all crates from crates.io can take a long time!
Building fewer targets will allow us to reduce wait times for every crate.
Additionally, this will decrease our fixed storage costs, allowing docs.rs to be sustainable into the future.

## Possible future changes

We are considering turning this on by default in the future;
i.e. only building for one target unless multiple targets are specifically requested.
However, we do not want to break anyone's documentation, so in the meantime the feature is opt-in instead of opt-out.

This change will also make it easier for docs.rs to build
for targets that are not tier one, such as embedded targets.

## How can I learn more?

For more details about targets and what it means to be a tier-one target, see [platform support].
For details about the change, see the PR: [docs.rs#632].
For details about the motivation, see [docs.rs#343].
For details about non-tier-one targets, see [docs.rs#563].

[docs.rs]: https://docs.rs/
[crates.io]: https://crates.io/
[platform support]: https://forge.rust-lang.org/release/platform-support.html
[metadata]: https://docs.rs/about#metadata
[`winapi`]: https://docs.rs/winapi/
[docs.rs#343]: https://github.com/rust-lang/docs.rs/issues/343
[docs.rs#563]: https://github.com/rust-lang/docs.rs/issues/563#issuecomment-573321498
[docs.rs#632]: https://github.com/rust-lang/docs.rs/pull/632
