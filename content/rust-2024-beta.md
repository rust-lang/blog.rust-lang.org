+++
path = "2025/01/22/rust-2024-beta"
title = "Rust 2024 in beta channel"
authors = ["TC & Eric Huss"]
aliases = ["2025/01/22/rust-2024-beta.html"]

[extra]
team = "the Edition 2024 Project Group"
team_url = "https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html"
+++

# Rust 2024 in beta channel

The next edition, Rust 2024, has entered the beta channel.  It will live there until 2025-02-20, when Rust 1.85 and Rust 2024 will be released as stable.

We're really happy with how Rust 2024 has turned out, and we're looking forward to putting it in your hands.

You can get a head start in preparing your code for the new edition, and simultaneously help us with final testing of Rust 2024, by following these steps within a project:

1. Run `rustup update beta`.
2. Run `cargo update`.
3. Run `cargo +beta fix --edition`.
4. Set `edition = "2024"` and, if needed, `rust-version = "1.85"`, in `Cargo.toml`.
5. Run `cargo +beta check`, address any remaining warnings, and then run other tests.

More details on how to migrate can be found [here](https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html) and within each of the [chapters](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/) describing the changes in Rust 2024.  For more on the changes themselves, see the [Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/).

If you encounter any problems or see areas where we could make the experience better, tell us about it by [filing an issue](https://github.com/rust-lang/rust/issues/new/choose).
