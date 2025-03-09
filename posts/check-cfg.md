+++
layout = "post"
date = 2024-05-06
title = "Automatic checking of cfgs at compile-time"
author = "Urgau"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

The Cargo and Compiler team are delighted to announce that starting with Rust 1.80 (or nightly-2024-05-05) every _reachable_ `#[cfg]` will be **automatically checked** that they match the **expected config names and values**.

This can help with verifying that the crate is correctly handling conditional compilation for different target platforms or features. It ensures that the cfg settings are consistent between what is intended and what is used, helping to catch potential bugs or errors early in the development process.

This addresses a common pitfall for new and advanced users.

This is another step to our commitment to provide user-focused tooling and we are eager and excited to finally see it fixed, after more than two years since the original [RFC 3013](https://github.com/rust-lang/rfcs/pull/3013)[^1].

[^1]: The stabilized implementation and RFC 3013 diverge significantly, in particular there is only one form for `--check-cfg`: `cfg()` (instead of `values()` and `names()` being incomplete and subtlety incompatible with each other).

## A look at the feature 

Every time a Cargo feature is declared that feature is transformed into a config that is passed to `rustc` (the Rust compiler) so it can verify with it along with [well known cfgs](https://doc.rust-lang.org/nightly/rustc/check-cfg.html#well-known-names-and-values) if any of the `#[cfg]`, `#![cfg_attr]` and `cfg!` have unexpected configs and report a warning with the `unexpected_cfgs` lint.

*`Cargo.toml`*:
    
```toml
[package]
name = "foo"

[features]
lasers = []
zapping = []
```

*`src/lib.rs`:*

```rust
#[cfg(feature = "lasers")]  // This condition is expected
                            // as "lasers" is an expected value
                            // of the `feature` cfg
fn shoot_lasers() {}

#[cfg(feature = "monkeys")] // This condition is UNEXPECTED
                            // as "monkeys" is NOT an expected
                            // value of the `feature` cfg
fn write_shakespeare() {}

#[cfg(windosw)]             // This condition is UNEXPECTED
                            // it's supposed to be `windows`
fn win() {}
```

*`cargo check`*:

![cargo-check](../../../../images/2024-05-06-check-cfg/cargo-check.svg)

## Expecting custom cfgs

*UPDATE: This section was added with the release of nightly-2024-05-19.*

> In Cargo point-of-view: a custom cfg is one that is neither defined by `rustc` nor by a Cargo feature. Think of `tokio_unstable`, `has_foo`, ... but not `feature = "lasers"`, `unix` or `debug_assertions`

Some crates might use custom cfgs, like `loom`, `fuzzing` or `tokio_unstable` that they expected from the environment (`RUSTFLAGS` or other means) and which are always statically known at compile time. For those cases, Cargo provides via the `[lints]` table a way to statically declare those cfgs as expected.

Defining those custom cfgs as expected is done through the special `check-cfg` config under `[lints.rust.unexpected_cfgs]`:

*`Cargo.toml`*
```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(loom)', 'cfg(fuzzing)'] }
```

## Custom cfgs in build scripts

On the other hand some crates use custom cfgs that are enabled by some logic in the crate `build.rs`. For those crates Cargo provides a new instruction: [`cargo::rustc-check-cfg`](https://doc.rust-lang.org/nightly/cargo/reference/build-scripts.html#rustc-check-cfg)[^2] (or `cargo:rustc-check-cfg` for older Cargo version).

[^2]: `cargo::rustc-check-cfg` will start working in Rust 1.80 (or nightly-2024-05-05). From Rust 1.77 to Rust 1.79 *(inclusive)* it is silently ignored. In Rust 1.76 and below a warning is emitted when used without the unstable Cargo flag `-Zcheck-cfg`.

The syntax to use is described in the [rustc book](https://doc.rust-lang.org/nightly/rustc/) section [checking configuration](https://doc.rust-lang.org/nightly/rustc/check-cfg.html), but in a nutshell the basic syntax of `--check-cfg` is:

```
cfg(name, values("value1", "value2", ..., "valueN"))
```

Note that every custom cfgs must always be expected, regardless if the cfg is active or not!

### `build.rs` example

`build.rs`:
```rust
fn main() {
    println!("cargo::rustc-check-cfg=cfg(has_foo)");
    //        ^^^^^^^^^^^^^^^^^^^^^^ new with Cargo 1.80
    if has_foo() {
        println!("cargo::rustc-cfg=has_foo");
    }
}
```

> Each `cargo::rustc-cfg` should have an accompanying **unconditional** `cargo::rustc-check-cfg` directive to avoid warnings like this: `unexpected cfg condition name: has_foo`.

### Equivalence table

| `cargo::rustc-cfg`      | `cargo::rustc-check-cfg`                       |
|-------------------------|------------------------------------------------|
| `foo`                   | `cfg(foo)` or `cfg(foo, values(none()))`       |
| `foo=""`                | `cfg(foo, values(""))`                         |
| `foo="bar"`             | `cfg(foo, values("bar"))`                      |
| `foo="1"` and `foo="2"` | `cfg(foo, values("1", "2"))`                   |
| `foo="1"` and `bar="2"` | `cfg(foo, values("1"))` and `cfg(bar, values("2"))` |
| `foo` and `foo="bar"`   | `cfg(foo, values(none(), "bar"))`              |

More details can be found in the [`rustc` book](https://doc.rust-lang.org/nightly/rustc/check-cfg.html).

## Frequently asked questions

### Can it be disabled?

For Cargo users, the feature is **always on** and _cannot_ be disabled, but like any other lints it can be controlled: `#![warn(unexpected_cfgs)]`.

### Does the lint affect dependencies?

No, like most lints, `unexpected_cfgs` will only be reported for local packages thanks to [cap-lints](https://doc.rust-lang.org/rustc/lints/levels.html#capping-lints).

### How does it interact with the `RUSTFLAGS` env?

You should be able to use the `RUSTFLAGS` environment variable like it was before.
*Currently `--cfg` arguments are not checked, only usage in code are.*

This means that doing `RUSTFLAGS="--cfg tokio_unstable" cargo check` will not report any warnings, unless `tokio_unstable` is used within your local crates, in which case crate author will need to make sure that that custom cfg is expected with `cargo::rustc-check-cfg` in the `build.rs` of that crate.

### How to expect custom cfgs without a `build.rs`?

*UPDATE: Cargo with nightly-2024-05-19 now provides the `[lints.rust.unexpected_cfgs.check-cfg]` config to address the statically known custom cfgs.*

~~There is **currently no way** to expect a custom cfg other than with `cargo::rustc-check-cfg` in a `build.rs`.~~

Crate authors that don't want to use a `build.rs` and cannot use `[lints.rust.unexpected_cfgs.check-cfg]`, are encouraged to use Cargo features instead.

### How does it interact with other build systems?

Non-Cargo based build systems are not affected by the lint by default. Build system authors that wish to have the same functionality should look at the `rustc` documentation for the [`--check-cfg`](https://doc.rust-lang.org/nightly/rustc/check-cfg.html) flag for a detailed explanation of how to achieve the same functionality.
