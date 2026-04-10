+++
path = "2026/04/16/Rust-1.95.0"
title = "Announcing Rust 1.95.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.95.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.95.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.95.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.95.0](https://doc.rust-lang.org/stable/releases.html#version-1950-2026-04-16).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.95.0 stable

### `cfg_select!`

Rust 1.95 introduces a
[`cfg_select!`](https://doc.rust-lang.org/stable/std/macro.cfg_select.html)
macro that acts roughly similar to a compile-time `match` on `cfg`s. This
fulfills the same purpose as the popular
[`cfg-if`](https://crates.io/crates/cfg-if) crate, although with a different
syntax. `cfg_select!` expands to the right-hand side of the first arm whose
configuration predicate evaluates to `true`. Some examples:

```rust
cfg_select! {
    unix => {
        fn foo() { /* unix specific functionality */ }
    }
    target_pointer_width = "32" => {
        fn foo() { /* non-unix, 32-bit functionality */ }
    }
    _ => {
        fn foo() { /* fallback implementation */ }
    }
}

let is_windows_str = cfg_select! {
    windows => "windows",
    _ => "not windows",
};
```

### if-let guards in matches

Rust 1.88 stabilized [let chains](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/#let-chains). Rust
1.95 brings that capability into match expressions, allowing for conditionals
based on pattern matching.

```rust
match value {
    Some(x) if let Ok(y) = compute(x) => {
        // Both `x` and `y` are available here
        println!("{}, {}", x, y);
    }
    _ => {}
}
```

Note that the compiler will not currently consider the patterns matched in `if
let` guards as part of the exhaustiveness evaluation of the overall match, just
like `if` guards.

### Stabilized APIs

See draft release notes, will get copied after those are non-draft:

<https://github.com/rust-lang/rust/issues/154711#:~:text=Stabilized%20APIs>

### Destabilized JSON target specs

Rust 1.95 removes support on stable for passing a custom target specification
to `rustc`. This should **not** affect any Rust users using a fully stable
toolchain, as building the standard library (including just `core`) already
required using nightly-only features.

We're also gathering use cases for custom targets on the [tracking issue](https://github.com/rust-lang/rust/issues/151528)
as we consider whether some form of this feature should eventually be stabilized.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.95.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-195-2026-04-16), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-195).

## Contributors to 1.95.0

Many people came together to create Rust 1.95.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.95.0/)

[platform-support]: https://doc.rust-lang.org/rustc/platform-support.html
