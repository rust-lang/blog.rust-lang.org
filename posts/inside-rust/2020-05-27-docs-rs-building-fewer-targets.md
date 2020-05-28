---
layout: post
title: "docs.rs will start building fewer targets"
author: Joshua Nelson
team: the docs.rs team <https://www.rust-lang.org/governance/teams/dev-tools#docs-rs>
---

Starting soon, docs.rs will by default only build one target for crates published to crates.io.
This is a breaking change, and if you own a crate that is different on different platforms,
you should consider specifying your targets explicitly.

## What does this change?

Currently, docs.rs builds documentation for all [tier 1 platforms][platform support] by default.
After this change, docs.rs will only build one platform by default.
Usually the platform is `x86_64-unknown-linux-gnu`, but this can be configured.
This only changes the default, you can still opt-in to more targets if you choose.

## Why is this change being made?

Building all 5 tier-one targets means that builds take 5x as long to finish.
Additionally, it means that docs.rs stores 5x as much documentation, increasing our fixed costs.
For most crates, the documentation is the same on every platform, so there is no need to build it many times.

## Who will this affect?

This will affect Rust developers who maintain or use a crate which is different on different platforms
(i.e. uses [conditional compilation]) and also does not explicitly specify the targets it should be built on.
For example, [winapi](https://docs.rs/winapi/0.3.8/) only has documentation on windows platforms,
so it needs to specify all windows targets in order for them to show up.

As a quick rule of thumb, if you go to a docs.rs page that looks like `docs.rs/:crate/:version/:target`,
where `:target` is a tier 1 platform, that page may be affected.

## Will old documentation be deleted?

No, any existing documentation for different targets will remain up.
There is no plan to delete docs.rs documentation that has already been built.

## How do I make sure my crate doesn't break?

If you are a maintainer of a crate that uses conditional compilation,
you can explicitly specify your targets in `Cargo.toml` like so:

```toml
[package.metadata.docs.rs]
# replace these with the targets you actually want to build
targets = ["x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"]
```

Any target supported by `rustup add target` is also supported by docs.rs.
See [the metadata documentation](https://docs.rs/about#metadata) for more details.

If you develop but do not maintain a crate that uses conditional compilation,
you can open an issue or PR to add these targets.

## When is this change being made?

There is no set time frame yet.
We can guarantee at least a month (July 1st) before we will implement the change.

## How can I learn more?

More information on targets is available in the [platform support] page.
You can learn more about the change in [the issue proposing it][docs.rs#343] and [the PR with the implementation][docs.rs#532].
If you want to discuss the change with the docs.rs team,
feel free to join the [docs.rs discord channel] or [open an issue][issue link].


[platform support]: https://forge.rust-lang.org/release/platform-support.html
[docs.rs#343]: https://github.com/rust-lang/docs.rs/issues/343
[docs.rs#532]: https://github.com/rust-lang/docs.rs/issues/532
[docs.rs discord channel]: https://discord.gg/f7mTXPW
[conditional compilation]: https://doc.rust-lang.org/reference/conditional-compilation.html#forms-of-conditional-compilation
[issue link]: (https://github.com/rust-lang/docs.rs/issues/new?title=Changing%20default%20targets)
