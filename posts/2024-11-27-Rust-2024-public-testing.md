+++
layout = "post"
title = "Rust 2024 call for testing"
author = "Eric Huss & TC"
team = "the Edition 2024 Project Group <https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html>"
+++

# Rust 2024 call for testing

We've been hard at work on Rust 2024. We're thrilled about how it has turned out. It's going to be the largest edition since Rust 2015. It has a great many improvements that make the language more consistent and ergonomic, that further our relentless commitment to safety, and that will open the door to long-awaited features such as `gen` blocks, `let` chains, and the never (`!`) type. For more on the changes, see the nightly [Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html).

As planned, we recently merged the feature-complete Rust 2024 edition [to the release train](https://github.com/rust-lang/rust/pull/133349) for Rust 1.85. It has now entered **nightly beta**[^1].

You can help right now to make this edition a success by testing Rust 2024 on your own projects using nightly Rust. Migrating your projects to the new edition is straightforward and mostly automated. Here's how:

1. Install the most recent nightly with `rustup update nightly`.
2. In your project, run `cargo +nightly fix --edition`.
3. Edit `Cargo.toml` and change the edition field to say `edition = "2024"` and, if you have a `rust-version` specified, set `rust-version = "1.85"`.
4. Run `cargo +nightly check` to verify your project now works in the new edition.
5. Run some tests, and try out the new features!

(More details on how to migrate can be found [here](https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html) and within each of the [chapters](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html) describing the changes in Rust 2024.)

If you encounter any problems or see areas where we could make the experience better, tell us about it by [filing an issue](https://github.com/rust-lang/rust/issues/new/choose).

### Coming next

Rust 2024 will enter the beta channel on 2025-01-09, and will be released to stable Rust with Rust 1.85 on 2025-02-20.

[^1]: That is, it's still in nightly (not in the beta channel), but the edition items are frozen in a way similar to it being in the beta channel, and as with any beta, we'd like wide testing.
