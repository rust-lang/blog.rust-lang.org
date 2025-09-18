+++
path = "2025/09/18/Rust-1.90.0"
title = "Announcing Rust 1.90.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.90.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.90.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.90.0 with:

```console
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.90.0](https://doc.rust-lang.org/stable/releases.html#version-1900-2025-09-18).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.90.0 stable

# LLD is now the default linker on `x86_64-unknown-linux-gnu`

The `x86_64-unknown-linux-gnu` target will now use the LLD linker for linking Rust crates by default. This should result in improved linking performance vs the default Linux linker (BFD), particularly for large binaries, binaries with a lot of debug information, and for incremental rebuilds.

In the vast majority of cases, LLD should be backwards compatible with BFD, and you should not see any difference other than reduced compilation time. However, if you do run into any new linker issues, you can always opt out using the `-C linker-features=-lld` compiler flag. Either by adding it to the usual `RUSTFLAGS` environment variable, or to a project's [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) configuration file,
like so:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-Clinker-features=-lld"]
```

If you encounter any issues with the LLD linker, please [let us know](https://github.com/rust-lang/rust/issues/new/choose). You can read more about the switch to LLD, some benchmark numbers and the opt out mechanism [here](https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/).

### Cargo adds native support for workspace publishing

`cargo publish --workspace` is now supported, automatically publishing all of
the crates in a workspace in the right order (following any dependencies
between them).

This has long been possible with external tooling or manual ordering of
individual publishes, but this brings the functionality into Cargo itself.

Native integration allows Cargo's publish verification to run a build across
the full set of to-be-published crates *as if* they were published, including
during dry-runs. Note that publishes are still not atomic -- network errors or
server-side failures can still lead to a partially published workspace.

### Demoting `x86_64-apple-darwin` to Tier 2 with host tools

GitHub will soon [discontinue][gha-sunset] providing free macOS x86\_64 runners for public repositories. Apple has also announced their [plans][apple] for discontinuing support for the x86\_64 architecture.

In accordance with these changes, as of Rust 1.90, we have [demoted the `x86_64-apple-darwin` target][rfc] from [Tier 1 with host tools](https://doc.rust-lang.org/stable/rustc/platform-support.html#tier-1-with-host-tools) to [Tier 2 with host tools](https://doc.rust-lang.org/stable/rustc/platform-support.html#tier-2-with-host-tools). This means that the target, including tools like `rustc` and `cargo`, will be guaranteed to build but is not guaranteed to pass our automated test suite.

For users, this change will not immediately cause impact. Builds of both the standard library and the compiler will still be distributed by the Rust Project for use via `rustup` or alternative installation methods while the target remains at Tier 2. Over time, it's likely that reduced test coverage for this target will cause things to break or fall out of compatibility with no further announcements.

[apple]: https://en.wikipedia.org/wiki/Mac_transition_to_Apple_silicon#Timeline
[gha-sunset]: https://github.blog/changelog/2025-07-11-upcoming-changes-to-macos-hosted-runners-macos-latest-migration-and-xcode-support-policy-updates/#macos-13-is-closing-down
[rfc]: https://github.com/rust-lang/rfcs/pull/3841

### Stabilized APIs

- [`u{n}::checked_sub_signed`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.checked_sub_signed)
- [`u{n}::overflowing_sub_signed`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.overflowing_sub_signed)
- [`u{n}::saturating_sub_signed`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.saturating_sub_signed)
- [`u{n}::wrapping_sub_signed`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.wrapping_sub_signed)
- [`impl Copy for IntErrorKind`](https://doc.rust-lang.org/stable/std/num/enum.IntErrorKind.html#impl-Copy-for-IntErrorKind)
- [`impl Hash for IntErrorKind`](https://doc.rust-lang.org/stable/std/num/enum.IntErrorKind.html#impl-Hash-for-IntErrorKind)
- [`impl PartialEq<&CStr> for CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html#impl-PartialEq%3C%26CStr%3E-for-CStr)
- [`impl PartialEq<CString> for CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html#impl-PartialEq%3CCString%3E-for-CStr)
- [`impl PartialEq<Cow<CStr>> for CStr`](https://doc.rust-lang.org/stable/std/ffi/struct.CStr.html#impl-PartialEq%3CCow%3C'_,+CStr%3E%3E-for-CStr)
- [`impl PartialEq<&CStr> for CString`](https://doc.rust-lang.org/stable/std/ffi/struct.CString.html#impl-PartialEq%3C%26CStr%3E-for-CString)
- [`impl PartialEq<CStr> for CString`](https://doc.rust-lang.org/stable/std/ffi/struct.CString.html#impl-PartialEq%3CCStr%3E-for-CString)
- [`impl PartialEq<Cow<CStr>> for CString`](https://doc.rust-lang.org/stable/std/ffi/struct.CString.html#impl-PartialEq%3CCow%3C'_,+CStr%3E%3E-for-CString)
- [`impl PartialEq<&CStr> for Cow<CStr>`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html#impl-PartialEq%3C%26CStr%3E-for-Cow%3C'_,+CStr%3E)
- [`impl PartialEq<CStr> for Cow<CStr>`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html#impl-PartialEq%3CCStr%3E-for-Cow%3C'_,+CStr%3E)
- [`impl PartialEq<CString> for Cow<CStr>`](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html#impl-PartialEq%3CCString%3E-for-Cow%3C'_,+CStr%3E)

These previously stable APIs are now stable in const contexts:

- [`<[T]>::reverse`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.reverse)
- [`f32::floor`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.floor)
- [`f32::ceil`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.ceil)
- [`f32::trunc`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.trunc)
- [`f32::fract`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.fract)
- [`f32::round`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.round)
- [`f32::round_ties_even`](https://doc.rust-lang.org/stable/std/primitive.f32.html#method.round_ties_even)
- [`f64::floor`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.floor)
- [`f64::ceil`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.ceil)
- [`f64::trunc`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.trunc)
- [`f64::fract`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.fract)
- [`f64::round`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.round)
- [`f64::round_ties_even`](https://doc.rust-lang.org/stable/std/primitive.f64.html#method.round_ties_even)

### Platform Support

- `x86_64-apple-darwin` is now a tier 2 target

Refer to Rust’s [platform support page][platform_support_page] for more information on Rust’s tiered platform support.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.90.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-190-2025-09-18), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-190).

## Contributors to 1.90.0

Many people came together to create Rust 1.90.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.90.0/)

[platform_support_page]: https://doc.rust-lang.org/rustc/platform-support.html
