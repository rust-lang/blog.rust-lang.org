+++
path = "2026/07/09/Rust-1.97.0"
title = "Announcing Rust 1.97.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.97.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.97.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.97.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.97.0](https://doc.rust-lang.org/stable/releases.html#version-1970-2026-07-09).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.97.0 stable

### Symbol mangling v0 enabled by default

When Rust is compiled into object files and binaries, each item (functions,
statics, etc) must have a globally unique "symbol" identifying it. To avoid
conflicts when linking together different Rust programs, Rust mangles the
original name of items to include additional context such as the module path,
defining crate, generics, and more. Historically, this mangling was based on
the [Itanium ABI](https://refspecs.linuxbase.org/cxxabi-1.86.html#mangling),
also (sometimes) used by C++.

The new mangling scheme resolves a number of drawbacks from the previous one:

* Generic parameter instantiations preserve their values, rather than being tracked solely behind a hash
* Inconsistencies: not all parts used the Itanium ABI, meaning that custom demangling was still necessary

Since Rust 1.59, the compiler has supported opting into a Rust-specific
mangling scheme via `-Csymbol-mangling-version=v0`. Since November 2025, this
scheme has been enabled by default on nightly, and 1.97 is now enabling it on
stable Rust. The legacy mangling scheme can only be enabled on nightly, and the
current plan is to fully remove it.

See the previous [blog post](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/) for more details.

### Cargo support for denying warnings

It's common practice to deny warnings in CI. Historically, doing so is
typically done through `RUSTFLAGS=-Dwarnings`. With Rust 1.97, Cargo controls
how warnings interact with build success: either silencing them (via `allow`
level), rendering without failing (default, `warn`), or denying them (via `deny`).

As a  result of Cargo configuration determining the behavior, using this
feature doesn't invalidate the underlying build cache, meaning that it's easy
to temporarily opt-in. For example, if warnings are adding unwanted noise while
working through fixing errors after a refactor, you can run
`CARGO_BUILD_WARNINGS=allow cargo check`, temporarily silencing them.

In CI, jobs can instead set `CARGO_BUILD_WARNINGS=deny` to deny warnings. This
can be combined with `--keep-going` to collect all errors and warnings rather
than stopping on the first failing package.

See the [documentation](https://doc.rust-lang.org/cargo/reference/config.html#buildwarnings) for more details.

### Linker output no longer hidden by default

rustc invokes a linker on behalf of users. Historically, rustc has silenced
linker output by default if the link completes successfully. This can mask real
problems, though, so in Rust 1.97 we are enabling linker messages by default.
These are emitted as a warning lint, for example:

```
warning: linker stderr: ignoring deprecated linker optimization setting '1'
  |
  = note: `#[warn(linker_messages)]` on by default
```

Common linker messages that have been diagnosed as false positives or intentional behavior
are filtered out by rustc. Several defects have already been fixed as a result
of no longer hiding this output on nightly.

Note that currently, `linker_messages` is a special lint that is *not* affected
by the `warnings` lint group. This is intentional as rustc generally doesn't
control linker output as precisely, and it's not uncommon for output to only
appear on some platforms. If you are seeing what you think is a false positive
output from the linker, please [file an issue].

To silence the warning in the mean time, you can configure the lint level to
allow. This can be done through `Cargo.toml` by adding a [lints section] like this:

```toml
[lints.rust]
linker_messages = "allow"
```

[file an issue]: https://github.com/rust-lang/rust/issues/new/choose
[lints section]: https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-lints-section

### Stabilized APIs

- [`Default for RepeatN`](https://doc.rust-lang.org/stable/std/iter/struct.RepeatN.html#impl-Default-for-RepeatN%3CA%3E)
- [`Copy for ffi::FromBytesUntilNulError`](https://doc.rust-lang.org/stable/std/ffi/struct.FromBytesUntilNulError.html#impl-Copy-for-FromBytesUntilNulError)
- [`Send for std::fs::File` on UEFI](https://github.com/rust-lang/rust/pull/154003)
- [`<{integer}>::isolate_highest_one`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.isolate_highest_one)
- [`<{integer}>::isolate_lowest_one`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.isolate_lowest_one)
- [`<{integer}>::highest_one`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.highest_one)
- [`<{integer}>::lowest_one`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.lowest_one)
- [`<{uN}>::bit_width`](https://doc.rust-lang.org/stable/std/primitive.u32.html#method.bit_width)
- [`NonZero<{integer}>::isolate_highest_one`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.isolate_highest_one)
- [`NonZero<{integer}>::isolate_lowest_one`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.isolate_lowest_one)
- [`NonZero<{integer}>::highest_one`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.highest_one)
- [`NonZero<{integer}>::lowest_one`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.lowest_one)
- [`NonZero<{uN}>::bit_width`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.bit_width)

These previously stable APIs are now stable in const contexts:

- [`char::is_control`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_control)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.97.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-197-2026-07-09), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-197).

## Contributors to 1.97.0

Many people came together to create Rust 1.97.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.97.0/)
