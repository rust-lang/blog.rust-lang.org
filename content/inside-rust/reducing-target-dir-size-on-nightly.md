+++
path = "inside-rust/2026/08/18/reducing-target-dir-size-on-nightly"
title = "Reducing target directory size on nightly"
authors = ["Jakub Beránek"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

TL;DR: Soon, Cargo will enable the `-Zembed-metadata=no` feature on the nightly channel by default, which can help reduce the size of the `target` directory somewhat. This is an experiment designed to gather feedback about this feature. See [here](#how-to-opt-out) for ways to opt out, in case you run into any issues.

## What is this about?

High disk usage of Rust compilation artifacts is frequently cited as one of the biggest annoyances of Rust users. In our 2025 State of Rust survey, it was actually the second most commonly reported [problem][binary-size-challenge], right after compilation speed.

There are various reasons why the `target` directory can become quite large, such as:
- Cargo compiles the whole crate graph from scratch by default, which produces a lot of build artifacts.
- Debug information takes a lot of disk space.
- Incremental compilation artifacts take a lot of disk space.

While you can disable debug information or incremental compilation to reduce the `target` directory size, that of course comes with severe trade-offs in compilation speed and debuggability of your program. However, there is one source of data in the `target` directory that currently takes too much size even though it doesn't really have to. It is the "crate metadata", which can be duplicated across multiple files. We will focus on that in this blog post.

## What causes duplicated (meta)data

For years, Cargo has been using [pipelined compilation][pipelined-compilation] to speed up building of crate graphs. When compiling a library crate, it tells the compiler to produce an `.rmeta` file (which contains all the crate metadata required to use this library) as soon as possible, even before having the final executable code available. This enables dependent crates to start compiling sooner. However, once the library does finish compiling, the final produced `.rlib` file will contain *both* the executable code and the Rust-specific metadata.

Which means that after the compilation finishes, the metadata of each library crate will be stored on disk twice: in the `.rmeta` file and also in the `.rlib` file. This can cause a non-trivial increase of the `target` directory size.

## Avoiding duplicated metadata

In order to reduce unnecessary data duplication in the built artifacts, last year we introduced an unstable compiler [flag][rust-pr] called `-Zembed-metadata`, together with a corresponding [Cargo flag](https://github.com/rust-lang/cargo/pull/15378) with the same name. When using `-Zembed-metadata=no`, the compiler will stop putting the metadata into the `.rlib` files, thus reducing their file size.

Soon, Cargo will start defaulting to using `-Zembed-metadata=no` when *both* Cargo and the used `rustc` have the `nightly` channel. Note that this is an experiment, the feature is not currently headed for stabilization. We want to evaluate how does this change fare in practice, and gather feedback from nightly users.

## How much does it help?

We compiled two crates (`serde` and `cargo` itself) on the `x86_64-unknown-linux-gnu` target using a recent nightly compiler (`rustc 1.100.0-nightly (67854e511 2026-08-15)`) in several configurations:
- `dev` profile with/without incremental compilation and with/without debuginfo
- `release` profile without incremental compilation and without debuginfo

We can see the results in the table below. The `Before` column shows the size of the `target` directory when using the previous default (`-Zembed-metadata=yes`), while the `After` column shows the `target` disk size with `-Zembed-metadata=no`, which will soon be the default on the nightly channel.

| Profile   | Incremental | Debuginfo | Before [MiB] | After [MiB] | Reduction |
|-----------|-------------|-----------|--------------|-------------|-----------|
| `dev`     | Yes         | Yes       | 179.70       | 171.29      | -4.7%     |
| `dev`     | Yes         | No        | 96.84        | 88.44       | -8.7%     |
| `dev`     | No          | Yes       | 81.12        | 72.68       | -10.4%    |
| `dev`     | No          | No        | 31.06        | 22.62       | -27.2%    |
| `release` | No          | No        | 40.88        | 28.68       | -29.8%    |

As expected, the disk size wins depend a lot on how much of the `target` directory is taken up by the crate metadata itself. If there are no incremental artifacts and no debuginfo, such as when using the default configuration of the `release` profile, the wins are substantial, in this case almost 30%. On the other hand, if we use the defaults of the `dev` profile, which enables both incremental compilation and generation of debug information, then the wins are more modest (in this case around 5%), because the debug information and incremental build artifacts dwarf the size of the crate metadata.

Below are the results for compiling Cargo itself, which are slightly better than for serde. In the best case, the new default reduced the size of the `target` directory by approximately 33%, almost 300 MiB in absolute terms!

| Profile   | Incremental | Debuginfo | Before [MiB] | After [MiB] | Reduction |
|-----------|-------------|-----------|--------------|-------------|-----------|
| `dev`     | Yes         | Yes       | 3151.16      | 2910.88     | -7.6%     |
| `dev`     | Yes         | No        | 1476.00      | 1235.71     | -16.3%    |
| `dev`     | No          | Yes       | 2065.75      | 1825.45     | -11.6%    |
| `dev`     | No          | No        | 999.71       | 759.41      | -24.0%    |
| `release` | No          | Yes       | 807.05       | 537.15      | -33.4%    |

This does not fully solve the problem of large `target` directories, but it can help at least a little bit.

## Does this affect me?

In the vast majority of situations, you should not notice anything different when `-Zembed-metadata=no` is used, other than the `target` directory becoming smaller. However, if you for some reason link to `.rlib` files manually, you might now also have to pass the corresponding `.rmeta` file to the compiler using the `--extern` flag to get access to the crate's metadata.

If you do not do that correctly, you might be greeted with an error similar to this one:

```
error: only metadata stub found for `rlib` dependency `foo`
please provide path to the corresponding .rmeta file with full metadata
```

Note that Cargo does not currently ["uplift"][rmeta-uplift] the corresponding `.rmeta` file, even when using `-Zembed-metadata=no`. That means that the `.rmeta` file (unlike the `.rlib` file) will not appear as [a final build artifact](https://doc.rust-lang.org/cargo/reference/build-cache.html) in the `target/<profile>` directory after the build of a leaf library completes, and so you must find its location in the nested build directories, e.g. using Cargo's [JSON output](https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages), if you want to locate it.

If you are using a different build system than Cargo, then this change will not affect you.

## How to opt out?

If you want to opt out, you can either pass the `-Zembed-metadata=yes` command-line argument to Cargo (note that this is passed as a Cargo flag, not as a compiler flag in `RUSTFLAGS`, because Cargo must be aware of the flag's value) or set the environment variable `CARGO_UNSTABLE_EMBED_METADATA=true`.

You can also opt out using a `.cargo/config.toml` file:
```toml
[unstable]
embed-metadata = true
```

## Future work

By enabling this feature on the `nightly` channel by default, we want to figure out how will it work in practice, and if Cargo users run into any unexpected issues with it. If everything goes smoothly, we would like to stabilize the compiler side of this feature, and then make the `embed-metadata=no` behavior be used by default also on the stable toolchain. However, this is not set in stone; it is possible that the feature and/or the flags might change before stabilization, or that they will not be stabilized at all, if we run into significant issues.

Therefore, if you run into any issues with this new default, please [open an issue][open-an-issue] in the Cargo repository, so that we get to know about them. Thank you!

And if you are interested in how is this feature moving forward, you can observe its progress in its [tracking issue][tracking-issue].

[binary-size-challenge]: https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/#challenges-and-wishes-about-rust
[rmeta-not-uplifted]: https://github.com/rust-lang/cargo/issues/17359
[tracking-issue]: https://github.com/rust-lang/cargo/issues/15495
[cargo-pr]: https://github.com/rust-lang/cargo/pull/15378
[rust-pr]: https://github.com/rust-lang/rust/pull/137535
[pipelined-compilation]: https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/#pipelined-compilation
[rmeta-uplift]: https://github.com/rust-lang/cargo/issues/17359
[open-an-issue]: https://github.com/rust-lang/cargo/issues
