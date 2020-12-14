---
layout: post
title: "Rustdoc call for new contributors"
author: The Rustdoc Team
release: false
---

Hello community!

The `rustdoc` project is looking for new contributions. The project has since long time grown up and the current members can barely cope with the maintainership and evolution. We are looking forward to mentoring and growing the amount of contributors with the possible future prospect to funnel some of them into a pathway to a stable membership.

If you:

- have some coding skills
- want to help one of the cornerstone project of the Rust ecosystem
- want to get engaged in the free/open source community

then this call is for you!


## What is `rustdoc`?

Rustdoc builds the documentation for a Rust crate. It ships with your Rust compiler and it is invoked with `cargo doc`.

rustdoc is not to be confused with docs.rs (even though docs.rs is using rustdoc!): docs.rs is the website hosting the documentation for Rust crates and it's a separate entity with different stewardship (some contributors share their work among these two projects).

## Why is `rustdoc` important?

One of the most useful tools in the toolbelt of Rust developers is the ability to automatically create the documentation in a standardized form. We all love *reading* (and expect to find) documentation when we use a library but we don't like *writing* it. This tension is solved by using `rustdoc`. Everytime you:

- open up docs.rs and search for the documentation of a crate
- compile the documentation for your new shiny library with [`cargo doc`](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate)
- visit [the documentation for the standard library](https://doc.rust-lang.org/std/)
- run [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html#target-selection)

... you are using the work of the [rustdoc team](https://www.rust-lang.org/governance/teams/dev-tools#rustdoc)!

## How can you contribute

There is a fairly [long backlog](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-rustdoc) of open issues with an ongoing effort to label some of them with [E-mentor](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AT-rustdoc+label%3AE-mentor) to provide immediate feedback for people curious to contribute.

In order to give concrete expectations, it's worth mentioning that at its current stage the project does have a medium/high level bar to new contributors and some time might be needed to get the proper context to land your first PR.

## How to get started

The code for rustdoc is in [the main Rust repository](https://github.com/rust-lang/rust/tree/master/src/librustdoc). The rustc-dev-guide has an [an introduction to rustdoc](https://rustc-dev-guide.rust-lang.org/rustdoc.html), as well as a detailed dive into [how rustdoc works internally](https://rustc-dev-guide.rust-lang.org/rustdoc-internals.html).

## Where to get in contact

The official channel for day to day conversations is on Zulip at the [#rustdoc stream](https://rust-lang.zulipchat.com/#narrow/stream/266220-rustdoc). If you happen to be on [Discord](https://discord.gg/4yEYPuT), there is also the channel there.

The team is looking forward to welcoming new contributors, so roll up your sleeves, get involved and dive straight into the center of the Rust ecosystem!
