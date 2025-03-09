+++
layout = "post"
date = 2023-08-24
title = "Cargo changes how arrays in config are merged"
author = "Arlo Siemsen"
team = "the Cargo team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

Cargo will be changing the order of merged configuration arrays, and we are looking for people to help test this change and provide feedback.

When an array in Cargo's configuration is defined in multiple places, the elements are merged together. The order that the elements were merged was inconsistent with how other configuration types were merged together, and was not working as expected.

The new merging order will match the precedence order of non-array [configuration], with higher-precedence configuration being placed later in the array. 

In the case of `build.rustflags`, this resolves the confusing situation where higher precedence flags (in a project's `config.toml`) are overridden with lower precedence flags (in the global `$CARGO_HOME`, for example).

This may result in behavior changes if a project depends on the existing merging order. If you have an environment that involves merging configuration arrays, please consider testing your project with nightly to ensure it will continue to work once this change stabilizes. If you encounter a problem, please file an [issue].

The change is included in Cargo starting with `nightly-2023-08-24` toolchain.

## Merging Order

The previous merging order was unspecified, but in practice it was the following, with earlier entries appearing first in the array:
* `config.toml` in the current directory
* `config.toml` in a parent directory
* `config.toml` in `$CARGO_HOME`
* command-line (`--config`)
* environment variable (`CARGO_*`)

The new merging order is:
* `config.toml` in `$CARGO_HOME`
* `config.toml` in a parent directory
* `config.toml` in the current directory
* environment variable (`CARGO_*`)
* command-line (`--config`)

The implementation is in [cargo#12515].

## Impacted configuration settings

The following configuration settings have arrays of strings that will be impacted by this change:

* [`alias`] entries
* [`build.rustflags`]
* [`build.rustdocflags`]
* [`target.<triple>.rustflags`]
* [`net.ssh.known-hosts`]

[`alias`]: https://doc.rust-lang.org/nightly/cargo/reference/config.html#alias
[`build.rustflags`]: https://doc.rust-lang.org/nightly/cargo/reference/config.html#buildrustflags
[`build.rustdocflags`]: https://doc.rust-lang.org/nightly/cargo/reference/config.html#buildrustdocflags
[`target.<triple>.rustflags`]: https://doc.rust-lang.org/nightly/cargo/reference/config.html#targettriplerustflags
[`net.ssh.known-hosts`]: https://doc.rust-lang.org/nightly/cargo/reference/config.html#netsshknown-hosts

## Example

The following example shows how this change may impact you and why we are making this change.

With the following in your Cargo home directory (usually `~/.cargo/config.toml`):

```toml
[build]
rustflags = ["-C", "target-cpu=x86-64-v2"]
```

and then inside a project directory there is a `.cargo/config.toml` configuration file with:

```toml
[build]
rustflags = ["-C", "target-cpu=x86-64-v3"]
```

when you run `cargo build` within that project, cargo would previously merge these so that it passes `-C target-cpu=x86-64-v3 -C target-cpu=x86-64-v2` to `rustc`. Because `rustc` ignores options earlier on the command-line and only honors the last one, the result would end up using `x86-64-v2`. This would effectively cause the merged config settings in the current directory to be ignored. 

This was not the intended behavior, since config merging should always start with the lowest precedence (things in the Cargo home directory) and have more-specific config locations override those.

[cargo#12515]: https://github.com/rust-lang/cargo/pull/12515
[configuration]: https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure
[issue]: https://github.com/rust-lang/cargo/issues/new/choose
