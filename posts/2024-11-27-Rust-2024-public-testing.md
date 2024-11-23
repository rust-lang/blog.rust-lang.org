---
layout: post
title: "Rust 2024 call for testing"
author: Eric Huss
team: the Edition 2024 Project Group <https://github.com/rust-lang/team/blob/f4a9c02d725256b064a18003ffd3e496dfed4b1b/teams/project-edition-2024.toml>
---

# Rust 2024 call for testing

We are happy to announce that the Rust 2024 edition is entering its **public testing period**. All of the planned features for the edition are now available on nightly builds along with migrations that should move your code from Rust 2021 to Rust 2024. If you'd like to learn more about the changes that are part of Rust 2024, check out the [nightly version of the Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2024/index.html).

### Public testing period

We are asking for the public to help test the 2024 edition by trying out the migration in your projects using the nightly channel. As always, we expect this to be a largely automated process. The steps to try out the Rust 2024 Edition as follows ([more detailed directions can be found here](https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html)):

1. Install the most recent nightly: `rustup update nightly`.
2. Run `cargo +nightly fix --edition`.
3. Edit `Cargo.toml` and change the edition field to say `edition = "2024"`.
4. Run `cargo +nightly check` to verify it now works in the new edition.
5. Run some tests and kick the tires on the new features!

If you encounter any problems or find areas where quality could be improved (missing documentation, confusing error messages, etc) please [file an issue](https://github.com/rust-lang/rust/issues/new/choose) and tell us about it! Thank you!

### What comes next

Rust 2024 is scheduled to hit stable in Rust 1.85, which will be released on February 20th, 2025.
