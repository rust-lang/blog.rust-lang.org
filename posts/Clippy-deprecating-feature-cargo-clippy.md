+++
layout = "post"
date = 2024-02-28
title = 'Clippy: Deprecating `feature = "cargo-clippy"`'
author = "The Clippy Team"
+++

Since Clippy [`v0.0.97`] and before it was shipped with `rustup`, Clippy
implicitly added a `feature = "cargo-clippy"` config[^1] when linting your code
with `cargo clippy`.

[^1]: It's likely that you didn't even know that Clippy implicitly sets this
    config (which was not a Cargo feature). This is intentional, as we stopped
    advertising and documenting this a long time ago.

Back in the day (2016) this was necessary to allow, warn or deny Clippy lints
using attributes:

```rust
#[cfg_attr(feature = "cargo-clippy", allow(clippy_lint_name))]
```

Doing this hasn't been necessary for a long time. Today, Clippy users will set
lint levels with tool lint attributes using the `clippy::` prefix:

```rust
#[allow(clippy::lint_name)]
```

The implicit `feature = "cargo-clippy"` has only been kept for backwards
compatibility, but will be deprecated in upcoming nightlies and later in
`1.78.0`.

## Alternative

As there is a rare [use case] for conditional compilation depending on Clippy,
we will provide an alternative. So in the future (`1.78.0`) you will be able to
use:

```rust
#[cfg(clippy)]
```

## Transitioning

> Should you only use stable toolchains, you can wait with the transition until
> Rust `1.78.0` (2024-05-02) is released.

Should you have instances of `feature = "cargo-clippy"` in your code base, you
will see a warning from the new Clippy lint
[`clippy::deprecated_clippy_cfg_attr`] available in the latest nightly Clippy.
This lint can automatically fix your code. So if you should see this lint
triggering, just run:

```
cargo clippy --fix -- -Aclippy::all -Wclippy::deprecated_clippy_cfg_attr
```

This will fix all instances in your code.

In addition, check your `.cargo/config` file for:

```toml
[target.'cfg(feature = "cargo-clippy")']
rustflags = ["-Aclippy::..."]
```

If you have this config, you will have to update it yourself, by either changing
it to `cfg(clippy)` or taking this opportunity to transition to [setting lint
levels in `Cargo.toml`][cargo-lints] directly.

## Motivation for Deprecation

Currently, there's a [call for testing], in order to stabilize [checking
conditional compilation at compile time][rfc-3013], aka `cargo check
-Zcheck-cfg`. If we were to keep the `feature = "cargo-clippy"` config, users
would start seeing a lot of warnings on their `feature = "cargo-clippy"`
conditions. To work around this, they would either need to allow the lint or
have to add a dummy feature to their `Cargo.toml` in order to silence those
warnings:

```toml
[features]
cargo-clippy = []
```

We didn't think this would be user friendly, and decided that instead we want to
deprecate the implicit `feature = "cargo-clippy"` config and replace it with the
`clippy` config.

[`v0.0.97`]: https://github.com/rust-lang/rust-clippy/blob/61daf674eaf17f3b504c51f01b4ee63fac47dfcf/CHANGELOG.md?plain=0#0097--2016-11-03
[rfc-3013]: https://github.com/rust-lang/rfcs/pull/3013
[use case]: https://doc.rust-lang.org/clippy/configuration.html#disabling-evaluation-of-certain-code
[`clippy::deprecated_clippy_cfg_attr`]: https://rust-lang.github.io/rust-clippy/master/index.html#/deprecated_clippy_cfg_attr
[cargo-lints]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section
[call for testing]: https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479
