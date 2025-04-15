+++
path = "2022/04/07/Rust-1.60.0"
title = "Announcing Rust 1.60.0"
authors = ["The Rust Release Team"]
aliases = ["2022/04/07/Rust-1.60.0.html"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.60.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.60.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website, and check out the
[detailed release notes for 1.60.0][notes] on GitHub.
If you'd like to help us out by testing future releases, you might consider updating locally to use
the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report] any bugs you might come across!

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1600-2022-04-07
[report]: https://github.com/rust-lang/rust/issues/new/choose

## What's in 1.60.0 stable

### Source-based Code Coverage

Support for LLVM-based coverage instrumentation has been stabilized in rustc. You can try this out on your code by rebuilding your code with `-Cinstrument-coverage`, for example like this:

```sh
RUSTFLAGS="-C instrument-coverage" cargo build
```

After that, you can run the resulting binary, which will produce a
`default.profraw` file in the current directory. (The path and filename can be
overriden by an environment variable; see
[documentation](https://doc.rust-lang.org/stable/rustc/instrument-coverage.html#running-the-instrumented-binary-to-generate-raw-coverage-profiling-data)
for details).

The `llvm-tools-preview` component includes `llvm-profdata` for processing and
merging raw profile output (coverage region execution counts); and `llvm-cov`
for report generation. `llvm-cov` combines the processed output, from
`llvm-profdata`, and the binary itself, because the binary embeds a mapping from
counters to actual source code regions.

```sh
rustup component add llvm-tools-preview
$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse default.profraw -o default.profdata
$(rustc --print sysroot)/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov show -Xdemangler=rustfilt target/debug/coverage-testing \
    -instr-profile=default.profdata \
    -show-line-counts-or-regions \
    -show-instantiations
```

The above commands on a simple helloworld binary produce this annotated report, showing that each line of the input was covered.

```
    1|      1|fn main() {
    2|      1|    println!("Hello, world!");
    3|      1|}
```

For more details, please read the
[documentation](https://doc.rust-lang.org/rustc/instrument-coverage.html) in the
rustc book. The baseline functionality is stable and will exist in some form
in all future Rust releases, but the specific output format and LLVM tooling which
produces it are subject to change. For this reason, it is important to make
sure that you use the same version for both the `llvm-tools-preview` and the
rustc binary used to compile your code.

### `cargo --timings`

Cargo has stabilized support for collecting information on build with the `--timings` flag.

```sh
$ cargo build --timings
   Compiling hello-world v0.1.0 (hello-world)
      Timing report saved to target/cargo-timings/cargo-timing-20220318T174818Z.html
    Finished dev [unoptimized + debuginfo] target(s) in 0.98s
```

The report is also copied to `target/cargo-timings/cargo-timing.html`. A report on the release build of Cargo has been put up [here](2022-04-07-timing.html). These reports can be useful for improving build performance.
More information about the timing reports may be found in the [documentation](https://doc.rust-lang.org/nightly/cargo/reference/timings.html).

### New syntax for Cargo features

This release introduces two new changes to improve support for [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) and how they interact with [optional dependencies](https://doc.rust-lang.org/cargo/reference/features.html#optional-dependencies): Namespaced dependencies and weak dependency features.

Cargo has long supported features along with optional dependencies, as illustrated by the snippet below.

```toml
[dependencies]
jpeg-decoder = { version = "0.1.20", default-features = false, optional = true }

[features]
# Enables parallel processing support by enabling the "rayon" feature of jpeg-decoder.
parallel = ["jpeg-decoder/rayon"]
```

There are two things to note in this example:
* The optional dependency `jpeg-decoder` implicitly defines a feature of the same name. Enabling the `jpeg-decoder` feature will enable the `jpeg-decoder` dependency.
* The `"jpeg-decoder/rayon"` syntax enables the `jpeg-decoder` dependency *and* enables the `jpeg-decoder` dependency's `rayon` feature.

Namespaced features tackles the first issue. You can now use the `dep:` prefix in the `[features]` table to explicitly refer to an optional dependency without implicitly exposing it as a feature. This gives you more control on how to define the feature corresponding to the optional dependency including hiding optional dependencies behind more descriptive feature names.

Weak dependency features tackle the second issue where the `"optional-dependency/feature-name"` syntax would always enable `optional-dependency`. However, often you want to enable the feature on the optional dependency *only* if some other feature has enabled the optional dependency. Starting in 1.60, you can add a ? as in `"package-name?/feature-name"` which will only enable the given feature if something else has enabled the optional dependency.

For example, let's say we have added some serialization support to our library, and it requires enabling a corresponding feature in some optional dependencies. That can be done like this:

```toml
[dependencies]
serde = { version = "1.0.133", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
serde = ["dep:serde", "rgb?/serde"]
```

In this example, enabling the serde feature will enable the serde dependency. It will also enable the serde feature for the rgb dependency, but only if something else has enabled the rgb dependency.

### Incremental compilation status

Incremental compilation is re-enabled for the 1.60 release. The Rust team continues to work on fixing bugs in incremental, but no problems causing widespread breakage are known at this time, so we have chosen to reenable incremental compilation. Additionally, the compiler team is continuing to work on long-term strategy to avoid future problems of this kind. That process is in relatively early days, so we don't have anything to share yet on that front.

### `Instant` monotonicity guarantees

On all platforms `Instant` will try to use an OS API that guarantees monotonic
behavior if available (which is the case on all tier 1 platforms). In practice
such guarantees are -- under rare circumstances -- broken by hardware,
virtualization, or operating system bugs. To work around these bugs and platforms
not offering monotonic clocks, `Instant::duration_since`, `Instant::elapsed` and
`Instant::sub` now saturate to zero. In older Rust versions this led to a panic
instead. `Instant::checked_duration_since` can be used to detect and handle
situations where monotonicity is violated, or `Instant`s are subtracted in the
wrong order.

This workaround obscures programming errors where earlier and later instants are
accidentally swapped. For this reason future Rust versions may reintroduce
panics in at least those cases, if possible and efficient.

Prior to 1.60, the monotonicity guarantees were provided through mutexes or
atomics in std, which can introduce large performance overheads to
`Instant::now()`. Additionally, the panicking behavior meant that Rust software
could panic in a subset of environments, which was largely undesirable, as the
authors of that software may not be able to fix or upgrade the operating system,
hardware, or virtualization system they are running on. Further, introducing
unexpected panics into these environments made Rust software less reliable and
portable, which is of higher concern than exposing typically uninteresting
platform bugs in monotonic clock handling to end users.

### Stabilized APIs

The following methods and trait implementations are now stabilized:

- [`Arc::new_cyclic`][arc_new_cyclic]
- [`Rc::new_cyclic`][rc_new_cyclic]
- [`slice::EscapeAscii`][slice_escape_ascii]
- [`<[u8]>::escape_ascii`][slice_u8_escape_ascii]
- [`u8::escape_ascii`][u8_escape_ascii]
- [`Vec::spare_capacity_mut`][vec_spare_capacity_mut]
- [`MaybeUninit::assume_init_drop`][assume_init_drop]
- [`MaybeUninit::assume_init_read`][assume_init_read]
- [`i8::abs_diff`][i8_abs_diff]
- [`i16::abs_diff`][i16_abs_diff]
- [`i32::abs_diff`][i32_abs_diff]
- [`i64::abs_diff`][i64_abs_diff]
- [`i128::abs_diff`][i128_abs_diff]
- [`isize::abs_diff`][isize_abs_diff]
- [`u8::abs_diff`][u8_abs_diff]
- [`u16::abs_diff`][u16_abs_diff]
- [`u32::abs_diff`][u32_abs_diff]
- [`u64::abs_diff`][u64_abs_diff]
- [`u128::abs_diff`][u128_abs_diff]
- [`usize::abs_diff`][usize_abs_diff]
- [`Display for io::ErrorKind`][display_error_kind]
- [`From<u8> for ExitCode`][from_u8_exit_code]
- [`Not for !` (the "never" type)][not_never]
- [_Op_`Assign<$t> for Wrapping<$t>`][wrapping_assign_ops]
- [`arch::is_aarch64_feature_detected!`][is_aarch64_feature_detected]

### Other changes

There are other changes in the Rust 1.60.0 release. Check out what changed in
[Rust](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1600-2022-04-07),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-160-2022-04-07),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-160).

### Contributors to 1.60.0

Many people came together to create Rust 1.60.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.60.0/)

[arc_new_cyclic]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#method.new_cyclic
[rc_new_cyclic]: https://doc.rust-lang.org/stable/std/rc/struct.Rc.html#method.new_cyclic
[slice_escape_ascii]: https://doc.rust-lang.org/stable/std/slice/struct.EscapeAscii.html
[slice_u8_escape_ascii]: https://doc.rust-lang.org/stable/std/primitive.slice.html#method.escape_ascii
[u8_escape_ascii]: https://doc.rust-lang.org/stable/std/primitive.u8.html#method.escape_ascii
[vec_spare_capacity_mut]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.spare_capacity_mut
[assume_init_drop]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.assume_init_drop
[assume_init_read]: https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.assume_init_read
[i8_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.i8.html#method.abs_diff
[i16_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.i16.html#method.abs_diff
[i32_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.i32.html#method.abs_diff
[i64_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.i64.html#method.abs_diff
[i128_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.i128.html#method.abs_diff
[isize_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.isize.html#method.abs_diff
[u8_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.u8.html#method.abs_diff
[u16_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.u16.html#method.abs_diff
[u32_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.u32.html#method.abs_diff
[u64_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.u64.html#method.abs_diff
[u128_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.u128.html#method.abs_diff
[usize_abs_diff]: https://doc.rust-lang.org/stable/std/primitive.usize.html#method.abs_diff
[display_error_kind]: https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#impl-Display
[from_u8_exit_code]: https://doc.rust-lang.org/stable/std/process/struct.ExitCode.html#impl-From%3Cu8%3E
[not_never]: https://doc.rust-lang.org/stable/std/primitive.never.html#impl-Not
[wrapping_assign_ops]: https://doc.rust-lang.org/stable/std/num/struct.Wrapping.html#trait-implementations
[is_aarch64_feature_detected]: https://doc.rust-lang.org/stable/std/arch/macro.is_aarch64_feature_detected.html
