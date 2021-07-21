---
layout: post
title: "Rust 2021 public testing period"
author: Niko Matsakis
team: the Edition 2021 Project Group <https://www.rust-lang.org/governance/teams/core#project-edition-2021>
---

# Rust 2021 public testing period

We are happy to announce that the Rust 2021 edition is entering its **public testing period**. All of the planned features for the edition are now available on nightly builds along with migrations that should move your code from Rust 2018 to Rust 2021. If you'd like to learn more about the changes that are part of Rust 2021, check out the [nightly version of the Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2021/index.html).

### Public testing period

As we enter the public testing period, **we are encouraging adventurous users to test migrating their crates over to Rust 2021.** As always, we expect this to be a largely automated process. The steps to try out the Rust 2021 Edition as follows ([more detailed directions can be found here](https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html)):

1. Install the most recent nightly: `rustup update nightly`.
2. Run `cargo +nightly fix --edition`.
3. Edit `Cargo.toml` and place `cargo-features = ["edition2021"]` at the top (above `[package]`), and change the edition field to say `edition = "2021"`.
4. Run `cargo +nightly check` to verify it now works in the new edition.

**Note that Rust 2021 is still unstable, so you can expect bugs and other changes!** We recommend migrating your crates in a temporary copy of your code versus your main branch. If you do encounter problems, or find areas where quality could be improved (missing documentation, confusing error messages, etc) please [file an issue](https://github.com/rust-lang/rust/issues/new/choose) and tell us about it! Thank you!

### What comes next

We are targeting stabilization of all Rust 2021 for Rust 1.56, which will be released on October 21st, 2021. Per the [Rust train release model](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html), that means all features and work must be landed on nightly by September 7th.
