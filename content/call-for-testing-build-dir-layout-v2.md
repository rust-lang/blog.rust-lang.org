+++
path = "2026/03/13/call-for-testing-build-dir-layout-v2"
title = "Call for Testing: Build Dir Layout v2"
authors = ["Ed Page"]
+++

We would welcome people to try and report issues with the nightly-only
`cargo -Zbuild-dir-new-layout`.
While the layout of the [build dir](https://doc.rust-lang.org/cargo/reference/build-cache.html)
is internal-only,
many projects need to rely on the unspecified details due to missing features within Cargo.
While we've performed a [crater run](https://github.com/rust-lang/rust/pull/149852),
that won't cover everything and we need help identifying tools and process that rely on the details,
reporting issues to these projects so they can update to the new layout or support them both.

## How to test this?

With at least nightly 2026-03-10,
run your tests, release processes, and anything else that may touch build-dir/target-dir
with the `-Zbuild-dir-new-layout` flag and `CARGO_BUILD_BUILD_DIR=build` environment variable, separately.

For example:
```console
$ cargo test -Zbuild-dir-new-layout
$ CARGO_BUILD_BUILD_DIR=build cargo test
```

Outcomes may include:
- Fixing local problems
- Reporting problems in upstream tools with a note on the [the tracking issue](https://github.com/rust-lang/cargo/issues/15010) for others
- Providing feedback on the [the tracking issue](https://github.com/rust-lang/cargo/issues/15010)

While this Call for Testing is for `-Zbuild-dir-new-layout`,
we ask to test with `CARGO_BUILD_BUILD_DIR=build` to help identify cause and relative impact.

Known failure modes:
- Inferring a `[[bin]]`s path from a `[[test]]`s path:
  - Use [`std::env::var_os("CARGO_BIN_EXE_*")`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-cargo-test) for Cargo 1.94+, maybe keeping the inference as a fallback for older Cargo versions
  - Use [`env!("CARGO_BIN_EXE_*")`](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates)
- Build scripts looking up target-dir from their binary or `OUT_DIR`: see [Issue #13663](https://github.com/rust-lang/cargo/issues/13663)
  - Update current workarounds to support the new layout
- Looking up user-requested artifacts from rustc, see [Issue #13672](https://github.com/rust-lang/cargo/issues/13672)
  - Update current workarounds to support the new layout

Library support status as of publish time:
- [assert_cmd](https://crates.io/crates/assert_cmd): fixed
- [cli_test_dir](https://crates.io/crates/cli_test_dir): [Issue #65](https://github.com/emk/subtitles-rs/issues/65)
- [compiletest_rs](https://crates.io/crates/compiletest_rs): [Issue #309](https://github.com/Manishearth/compiletest-rs/issues/309)
- [executable-path](https://crates.io/crates/executable-path): fixed
- [snapbox](https://crates.io/crates/snapbox): fixed
- [term-transcript](https://crates.io/crates/term-transcript): [Issue #269](https://github.com/slowli/term-transcript/issues/269)
- [test_bin](https://crates.io/crates/test_bin): [Issue #13](https://github.com/MichaelMcDonnell/test_bin/issues/13)
- [trycmd](https://crates.io/crates/trycmd): fixed

## What is not changing?

The layout of final artifacts within [target dir](https://doc.rust-lang.org/cargo/reference/build-cache.html).

Nesting of build artifacts under the profile and the target tuple, if specified.

## What is changing?

We are switching from organizing by content type to scoping the content by the package name and a hash of the build unit and its inputs.

Here is an example of the current layout, assuming you have a package named `lib` and a package named `bin`, and both have a build script:
```
build-dir/
├── CACHEDIR.TAG
└── debug/
    ├── .cargo-lock                       # file lock protecting access to this location
    ├── .fingerprint/                     # build cache tracking
    │   ├── bin-[BUILD_SCRIPT_RUN_HASH]/*
    │   ├── bin-[BUILD_SCRIPT_BIN_HASH]/*
    │   ├── bin-[HASH]/*
    │   ├── lib-[BUILD_SCRIPT_RUN_HASH]/*
    │   ├── lib-[BUILD_SCRIPT_BIN_HASH]/*
    │   └── lib-[HASH]/*
    ├── build/
    │    ├── bin-[BIN_HASH]/*             # build script binary
    │    ├── bin-[RUN_HASH]/out/          # build script run OUT_DIR
    │    ├── bin-[RUN_HASH]/*             # build script run cache
    │    ├── lib-[BIN_HASH]/*             # build script binary
    │    ├── lib-[RUN_HASH]/out/          # build script run OUT_DIR
    │    └── bin-[RUN_HASH]/*             # build script run cache
    ├── deps/
    │   ├── bin-[HASH]*                   # binary and debug information
    │   ├── lib-[HASH]*                   # library and debug information
    │   └── liblib-[HASH]*                # library and debug information
    ├── examples/                         # unused in this case
    └── incremental/...                   # managed by rustc
```

The proposed layout:
```
build-dir/
├── CACHEDIR.TAG
└── debug/
    ├── .cargo-lock                       # file lock protecting access to this location
    ├── build/
    │   ├── bin/                          # package name
    │   │   ├── [BUILD_SCRIPT_BIN_HASH]/
    │   │   │   ├── fingerprint/*         # build cache tracking
    │   │   │   └── out/*                 # build script binary
    │   │   ├── [BUILD_SCRIPT_RUN_HASH]/
    │   │   │   ├── fingerprint/*         # build cache tracking
    │   │   │   ├── out/*                 # build script run OUT_DIR
    │   │   │   └── run/*                 # build script run cache
    │   │   └── [HASH]/
    │   │       ├── fingerprint/*         # build cache tracking
    │   │       └── out/*                 # binary and debug information
    │   └── lib/                          # package name
    │       ├── [BUILD_SCRIPT_BIN_HASH]/
    │       │   ├── fingerprint/*         # build cache tracking
    │       │   └── out/*                 # build script binary
    │       ├── [BUILD_SCRIPT_RUN_HASH]/
    │       │   ├── fingerprint/*         # build cache tracking
    │       │   ├── out/*                 # build script run OUT_DIR
    │       │   └── run/*                 # build script run cache
    │       └── [HASH]/
    │           ├── fingerprint/*         # build cache tracking
    │           └── out/*                 # library and debug information
    └── incremental/...                   # managed by rustc
```

For more information on these Cargo internals, see the [`mod layout` documentation](https://doc.rust-lang.org/nightly/nightly-rustc/cargo/core/compiler/layout/index.html).

## Why is this being done?

[ranger-ross](https://github.com/ranger-ross/) has worked tirelessly on this as a stepping stone to [cross-workspace caching](https://github.com/rust-lang/cargo/issues/5931)
which will be easier when we can track each cacheable unit in a self-contained directory.

This also unblocks work on:
- [Automatic cleanup of stale build units](https://github.com/rust-lang/cargo/issues/5026) to keep disks space use constant over time
- [More granular locking](https://github.com/rust-lang/cargo/issues/4282) so `cargo test` and rust-analyzer don't block on each other

Along the way, we found this helps with:
- [Build performance](https://github.com/rust-lang/cargo/issues/16665) as the intermediate artifacts accumulate in `deps/`
- [Content of `deps/` polluting `PATH` during builds on Windows](https://github.com/rust-lang/cargo/issues/7919)
- [Avoiding file collisions among intermediate artifacts](https://github.com/rust-lang/cargo/issues/16673)

While the Cargo team does not officially endorse sharing a `build-dir` across workspaces,
that last item should reduce the chance of encountering problems for those who choose to.

## Future work

We will use the experience of this layout change to help guide how and when to perform any future layout changes, including:
- Efforts to reduce path lengths to reduce risks for errors for developers on Windows
- Experimenting with moving artifacts out of the `--profile` and `--target` directories, allowing sharing of more artifacts where possible

In addition to narrowing scope,
we did not do all of the layout changes now because some are blocked on the lock change which is blocked on this layout change.

We would also like to work to decouple projects from the unspecified details of build-dir.
