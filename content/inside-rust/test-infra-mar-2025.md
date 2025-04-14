+++
path = "inside-rust/2025/04/13/test-infra-mar-2025"
title = "This Month in Our Test Infra: March 2025 issue"
authors = ["Jieyou Xu"]
aliases = ["inside-rust/2025/04/13/test-infra-mar-2025.html"]

[extra]
team = "the Bootstrap Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-bootstrap"
+++

# This Month in Our Test Infra: March 2025 issue

<!-- time period: 2025-03-11 through 2025-04-13 -->

This is a quick summary of the changes in the test infrastructure for the [rust-lang/rust] repository[^scope]
between **2025-03-11** and **2025-04-13**.

During this period, there are some significant changes to [compiletest] and [bootstrap] that contributors should be aware of.

[^scope]: The test infra here refers to the test harness [compiletest] and supporting components in our build system [bootstrap]. This test infra is used mainly by rustc and rustdoc. Other tools like cargo, miri or rustfmt maintain their own test infra.

As usual, if you encounter bugs or UX issues when using our test infrastructure, please [file an issue][new-issue]. Bugs and papercuts can't be fixed if we don't know about them!

**Thanks to everyone who contributed to our test infra!**

## Highlights

### `compiletest` now supports matching diagnostics on lines below

UI error annotations previously could match against diagnostics emitted for previous lines (`//~^`) or same line (`//~`), but couldn't express matching against diagnostics emitted for subsequent lines. Following [#139100][pr-139100], test writers can now use `//~v` to match against diagnostics emitted for subsequent lines. This is particularly useful for e.g. parser tests where the test file may be exercising lexer/parser diagnostics and the test file itself is not syntactically valid (so can't use comments *afterwards* to match against *previous* lines).

Example:

```rs
// tests/ui/parser/issues/issue-103451.rs

struct R { }
//~vv ERROR this file contains an unclosed delimiter
struct S {
    x: [u8; R
```

[pr-139100]: https://github.com/rust-lang/rust/pull/139100

### `compiletest` now supports precise and exhaustive matching on diagnostics without line information associated with the main test file

Previously, test writers had to resort to `//@ error-pattern` to match against diagnostics which do not have associated line information in the main test file. Now, test writers may use `//~? DIAGNOSTIC_KIND diagnostic_message` to match against a diagnostic with diagnostic kind `DIAGNOSTIC_KIND` (e.g. `ERROR` or `HELP`) with message `diagnostic_message`. See [compiletest: Support matching on diagnostics without a span #138865][pr-138865]. `//~?` may appear on any line of the test, but are conventionally placed at the end.

Example:

```rs
// tests/ui/invalid-module-declaration/invalid-module-declaration.rs

mod auxiliary {
    mod foo;
}

fn main() {}

//~? ERROR file not found for module `baz`
```

Compared to `//@ error-pattern`, `//~?` error annotations are **precise** and **exhaustive**.

- **Precise**: `//~?` annotations will not match against diagnostics that do have line information.
- **Exhaustive**: if multiple diagnostics lacking line information in the main test file are present, then all of them need to be annotated with `//~?`.

Combined, `//~?` helps to prevent accidentally blessing away diagnostics that don't have associated line information on the main test file.

`//@ error-pattern` are still useful for e.g. runtime error messages that do not have specific spans, or compile time messages that require imprecise matching due to multiline platform-specific diagnostics. `//~?` should be preferred where suitable.

[pr-138865]: https://github.com/rust-lang/rust/pull/138865


## Notable changes

This section is intended to be like "compatibility notes" but for human test writers.

### In preparation of stage 0 std redesign, it is now possible to use stage 0 libtest for `compiletest`

The [stage 0 std redesign](https://github.com/rust-lang/rust/pull/119899) means that building the library tree would require a stage 1 compiler. `compiletest` is somewhat strange in that it currently depends on in-tree libtest -- meaning that with the stage 0 std redesign, changes to the compiler tree would necessitate rebuilding `compiletest`, causing more friction for compiler development cycle. To mitigate this, bootstrap [now supports a new configuration option to switch `compiletest` to depend on stage 0 libtest instead of in-tree libtest][pr-139386], so that changes to `compiler/` tree would not cause `compiletest` to rebuild.

```toml
[build]
compiletest-use-stage0-libtest = true
```

Note that as a trade-off, we do check this force-stage0-libtest configuration in PR CI. This means that there are possible windows of time where libtest may be modified before bumping stage 0 compiler, which may require the PR author modifying libtest to add `cfg(bootstrap)`/`cfg(not(bootstrap))` to compiletest bits depending libtest programmatic API as suitable. This is done under the assumption that libtest programmatic API changes are comparatively much less frequent. If this proves to be a significant burden, the bootstrap team would be open to revisiting this scheme.

Note in over the long term, we'd like to migrate away from depending on in-tree libtest (both to render this hack unnecessary and to have more control over test execution).

[pr-139386]: https://github.com/rust-lang/rust/pull/139386

### bootstrap's `config.toml` has been renamed `bootstrap.toml`

Previously, [bootstrap] used `config.toml` as the configuration file under checkout root. This is potentially confusing since Cargo also uses `config.toml` as its configuration file name. Now, [bootstrap instead uses `bootstrap.toml` as the configuration file name][pr-137081]. The example config file `config.example.toml` has also been renamed to `bootstrap.example.toml`.

`config.toml` is temporarily accepted as fallback config file to mitigate migration churn across different checkouts, but will be phased out in the future.

[pr-137081]: https://github.com/rust-lang/rust/pull/137081

### `compiletest` now enforces stricter parsing of diagnostics kinds

Previously, `compiletest` was very lax in the casing of diagnostic kinds (i.e. the `ERROR`/`HELP` portion of error annotations). This mean that annotations such as

```rs
//~ Error xxx
//~ Error: xxx
```

were also accepted compared to the more common `//~ ERROR` or `//~ error` forms.

Eventually, we would like to canonicalize the error annotations into one form. To make this transition less abrupt, as an intermediate step, compiletest will now:

- Enforce that error annotation diagnostic kinds must either be full caps (e.g. `ERROR`) or lowercase (e.g. `error`). Mixed cases like `//~ Error` will now be rejected.
- Empty diagnostics (such as empty `//~ NOTE`) will no longer be silently ignored. These indicate silly empty diagnostic notes emitted by rustc which should be fixed.

### `compiletest` now enforces that error annotations are required even if `//@ error-pattern` directives are used

Related to previous `//~?` improvements, compiletest will now also [check for error annotations (`//~`) in tests that use `//@ error-pattern` to minimize risk of accidentally blessing away diagnostics, including diagnostics without associated line information for the main test file.][pr-139137].

In *exceptional* cases, the test writer may opt out of the error annotation checks via `//@ compile-flags: --error-format=human`.

[pr-139137]: https://github.com/rust-lang/rust/pull/139137

### `compiletest` now allows opting in to non-exhaustive matching of a specific diagnostic kind via new directive `//@ dont-require-annotations: DIAGNOSTIC_KIND`

`compiletest` now has a [new directive `//@ dont-require-annotations: DIAGNOSTIC_KIND`][pr-139489] (where `DIAGNOSTIC_KIND` is e.g. `ERROR`/`HELP`/`SUGGESTION`) to allow opting into non-exhaustive matching for the specified `DIAGNOSTIC_KIND`. This includes the `ERROR` diagnostic kind, where it was not possible to opt-out of exhaustive matching previously.

This directive should be used with caution and sparingly.

[pr-139489]: https://github.com/rust-lang/rust/pull/139489

### `compiletest` no longer accepts `{true, false}` as revision names

In test suites that support revisions, previously `//@ revisions: true` and `//@ revisions: false` were accepted as revision names. However, this is confusing because they will be automatically registered as `--cfg={true,false}` to `rustc`, but the test writer would have to use `cfg(r#true)` and `cfg(r#false)` in the test. Hence, [the `{true,false}` revision names are no longer permitted by compiletest](https://github.com/rust-lang/rust/pull/138692).

### `compiletest` now supports a `//@ needs-crate-type` directive

Test writers can now use a [`//@ needs-crate-type` directive](https://github.com/rust-lang/rust/pull/139469) to guard test execution based on whether the target platform supports all of the required crate types.

The directive accepts a comma-delimited list of valid crate types that are accepted by the `rustc --crate-type` flag. E.g. `//@ needs-crate-type: dylib, proc-macro`.

Example:

```rs
// This test would be ignored on targets e.g. `wasm32-unknown-unknown` that
// do not (currently) support `dylib` crate type.

//@ needs-crate-type: dylib
//@ compile-flags: --crate-type=dylib

fn foo() {}
```

### `compiletest` now trims whitespace from the env var name passed to `//@ {unset,}-{exec,rustc}-env` directives

`compiletest` has a [long-standing quirk](https://github.com/rust-lang/rust/issues/132990) where

```rs
//@ rustc-env: RUSTC_BOOTSTRAP=1
```

was not the same as

```rs
//@ rustc-env:RUSTC_BOOTSTRAP=1
```

where the former is treated as `⌴RUSTC_BOOTSTRAP`. The same was true for `//@ exec-env`, and the `//@ unset-{exec,rustc}-env` directives.

This has now been fixed. `compiletest` will now [trim whitespace from the env var name](https://github.com/rust-lang/rust/pull/139507), so that the aforementioned forms are now equivalent:

```rs
//@ rustc-env: RUSTC_BOOTSTRAP=1
//@ rustc-env:RUSTC_BOOTSTRAP=1
```

both correspond to env var name `RUSTC_BOOTSTRAP`.

### `compiletest` now enforces that `//@ edition` directive must be used instead of `//@ compile-flags: --edition=xxx`

`compiletest` has an `--edition` flag that permits changing the default edition that tests are run with. However, no test suites currently pass with that set to non-2015-edition, but in the future we would like to be possible to run the test suites across all editions to ensure we have good test coverage over all editions (subject to further design concerns).

To make this remotely possible, we first need to ensure `compiletest`'s `--edition` override flag functions. This means that now, `compiletest` will reject rustc `--edition` flags passed via `//@ compile-flags` in favor of the `//@ edition` directive, to ensure that `compiletest` can override the edition of the test.

If the test author needs to exercise the behavior of the `--edition` rustc flag *itself*, a `run-make` test should be used.

## Upcoming changes

The following changes have not merged yet, but will merge in the near future or we are actively working towards them.

### `compiletest` will soon also treat `SUGGESTION` error annotations as viral

Previously, if the test writer specifies `//~ HELP` or `//~ NOTE`, then `compiletest` treated them as *viral* -- if one is specified, then all `HELP`/`NOTE` diagnostics of the same diagnostic kind must be exhaustively specified. For consistency and to be more strict in checking what diagnostics the compiler emits, we [plan to make `//~ SUGGESTION` annotations also viral](https://github.com/rust-lang/rust/pull/139618).

This viral exhaustive-matching behavior can be opt-out through the newly introduced `//@ dont-require-annotations: SUGGESTION` (see the *Notable changes* section for the description of this new directive).

### `run-make` tests (and the `run-make-support` library) will soon cross-compile for target platform by default

We're working on fixing the `run-make` test suite and the `run-make-support` library to properly test cross-compile target platform by default. Test authors would still be able to use `//@ ignore-cross-compile` if the test can only work on host.

See <https://github.com/rust-lang/rust/pull/138066> and related work-in-progress efforts.

## PR listing

### Improvements

- [bootstrap]: [Change `config.toml` to `bootstrap.toml`](https://github.com/rust-lang/rust/pull/137081)
- [bootstrap]: [Make it possible to use stage0 libtest on `compiletest`](https://github.com/rust-lang/rust/pull/139386)
- [citool]: [Add bootstrap step diff to CI job analysis](https://github.com/rust-lang/rust/pull/138930)
- [citool]: [Add job summary links to post-merge report](https://github.com/rust-lang/rust/pull/139481)
- [citool]: [Improve post-merge workflow](https://github.com/rust-lang/rust/pull/138454)
- [compiletest]: [Say which test failed the `COMPILETEST_REQUIRE_ALL_LLVM_COMPONENTS` assertion](https://github.com/rust-lang/rust/pull/138858)
- [compiletest]: [Support matching on diagnostics without a span](https://github.com/rust-lang/rust/pull/138865)
- [compiletest]: [Report compiletest pass mode if forced](https://github.com/rust-lang/rust/pull/138999)
- [compiletest]: [Support matching diagnostics on lines below](https://github.com/rust-lang/rust/pull/139100)
- [compiletest]: [Require `//~` annotations even if `error-pattern` is specified](https://github.com/rust-lang/rust/pull/139137)
- [compiletest]: [Introduce a `//@ needs-crate-type` `compiletest` directive](https://github.com/rust-lang/rust/pull/139469)
- [compiletest]: [Stricter parsing for diagnostic kinds](https://github.com/rust-lang/rust/pull/139485)
- [compiletest]: [Add directive `//@ dont-require-annotations`](https://github.com/rust-lang/rust/pull/139489)

### Fixes

- [bootstrap]: [Only build `rust_test_helpers` for `{incremental,ui}` test suites](https://github.com/rust-lang/rust/pull/139347)
- [compiletest]: [Update `compiletest`'s `has_asm_support` to match `rustc`](https://github.com/rust-lang/rust/pull/138371)
- [compiletest]: [Update error message](https://github.com/rust-lang/rust/pull/138441)
- [compiletest]: [Reject `{true,false}` as revision names](https://github.com/rust-lang/rust/pull/138692)
- [compiletest]: [Trim whitespace from environment variable names](github.com/rust-lang/rust/pull/139507)
- [compiletest]: [Fix breakage when running `compiletest` with `--test-args=--edition=2015`](https://github.com/rust-lang/rust/pull/139578)
- [run-make]: [Reintroduce remote-test support in `run-make` tests](https://github.com/rust-lang/rust/pull/138652)
- [run-make-support]: [Calculate artifact names for target platform, not host platform](https://github.com/rust-lang/rust/pull/139242)

### Cleanups

- [compiletest]: [Encapsulate all of the code that touches libtest](https://github.com/rust-lang/rust/pull/139317)
- [compiletest]: [Sort deps and drop dep on `anyhow`](https://github.com/rust-lang/rust/pull/139520)
- [compiletest]: [Remove the `--logfile` flag](https://github.com/rust-lang/rust/pull/139528)
- [compiletest]: [Update compiletest to Edition 2024](https://github.com/rust-lang/rust/pull/139606)
- [compiletest]: [Don't use stringly paths for `compose_and_run`](https://github.com/rust-lang/rust/pull/139609)

### Documentation updates

Note that since [rustc-dev-guide] became a josh subtree in [rust-lang/rust], some doc updates are made alongside the [rust-lang/rust] PR themselves (which are not redundantly listed here).

- [rustc-dev-guide]: [Test directive can appear anywhere in the file](https://github.com/rust-lang/rustc-dev-guide/pull/2314)
- [rustc-dev-guide]: [Add some links about the `rustdoc-gui` test suite](https://github.com/rust-lang/rustc-dev-guide/pull/2315)
- [rustc-dev-guide]: [Slightly reorganize ecosystem tests, stub out codegen backend test pages](https://github.com/rust-lang/rustc-dev-guide/pull/2299)
- [rustc-dev-guide]: [Add new section on the `rustdoc` test suite](https://github.com/rust-lang/rustc-dev-guide/pull/2295)
- [rustc-dev-guide]: [Add Fuchsia ping group page and mention Fuchsia and RfL ping groups in integration test pages](https://github.com/rust-lang/rustc-dev-guide/pull/2294)
- [rustc-dev-guide]: [Mention that `//@ known-bug` test directive takes arguments](github.com/rust-lang/rustc-dev-guide/pull/2288)
- [rustc-dev-guide]: ["Running tests" chapter cleaning](https://github.com/rust-lang/rustc-dev-guide/pull/2287)
- [rustc-dev-guide]: [Remove the doc for `no-system-llvm`](https://github.com/rust-lang/rustc-dev-guide/pull/2284)
- [rustc-dev-guide]: [Fix link to `rustc_*` test attributes in `ui.md`](https://github.com/rust-lang/rustc-dev-guide/pull/2324)


[rust-lang/rust]: https://github.com/rust-lang/rust
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
[compiletest]: https://github.com/rust-lang/rust/tree/master/src/tools/compiletest
[run-make-support]: https://github.com/rust-lang/rust/tree/master/src/tools/run-make-support
[bootstrap]: https://github.com/rust-lang/rust/tree/master/src/bootstrap
[libtest]: https://github.com/rust-lang/rust/tree/master/library/test
[new-issue]: https://github.com/rust-lang/rust/issues/new
[filecheck]: https://llvm.org/docs/CommandGuide/FileCheck.html
[run-make]: https://github.com/rust-lang/rust/tree/master/tests/run-make
[tidy]: https://github.com/rust-lang/rust/tree/master/src/tools/tidy
[library]: https://github.com/rust-lang/rust/tree/master/library
[citool]: https://github.com/rust-lang/rust/tree/master/src/ci/citool
