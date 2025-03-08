+++
layout = "post"
date = 2025-01-10
title = "This Month in Our Test Infra: December 2024"
author = "Jieyou Xu"
team = "the Bootstrap Team <https://www.rust-lang.org/governance/teams/infra#team-bootstrap>"
+++

# This Month in Our Test Infra: December 2024

<!-- time period: 2024-12-04 through 2025-01-05 -->

Happy new year, dear reader! This is the last *This Month in Our Test Infra* issue for 2024.

This is a quick summary of the changes in the test infrastructure for the [rust-lang/rust] repository[^scope] for **December 2024**.

[^scope]: The test infra here refers to the test harness [compiletest] and supporting components in our build system [bootstrap]. This test infra is used mainly by rustc and rustdoc. Other tools like cargo, miri or rustfmt maintain their own test infra.

As usual, if you encounter bugs or UX issues when using our test infrastructure, please [file an issue][new-issue]. Bugs and papercuts can't be fixed if we don't know about them!

**Thanks to everyone who contributed to our test infra!**

## Highlights

### `rustc-dev-guide` is now a `josh` subtree!

Previously, [rustc-dev-guide] was a submodule inside [rust-lang/rust], and updates to [rustc-dev-guide] had to be done in a separate PR against the [rustc-dev-guide] repository.

Now, thanks to [@Kobzol](https://github.com/Kobzol)'s efforts (which included overcoming many unforeseen obstacles), [`rustc-dev-guide` is now a `josh` subtree][pr-134907]. This is a significant improvement for contribution workflows because it means that documentation updates to [rustc-dev-guide] can accompany the implementation change in [rust-lang/rust] in the same PR. The reduction in contribution friction also encourages [rustc-dev-guide] updates because you no longer have to author and maintain two separate PRs.

[pr-134907]: https://github.com/rust-lang/rust/pull/134907

### compiletest will now show the difference between normalized output and actual output for differing lines

Previously, it can be very difficult to tell when a ui test fails what the actual output is pre-normalization, as you would have to resort to `--nocapture` or similar.

Now, [compiletest will also show the pre-normalization mismatched output lines on failure to make this easier to tell][pr-133733]. Example output:

```text
failures:

---- [ui] tests/ui/layout/enum.rs stdout ----
diff of stderr:

-	error: align: AbiAndPrefAlign { abi: Align(2 bytes), pref: $PREF_ALIGN }
+	error: align: AbiAndPrefAlign { abi: Align(2 bytes), pref: $PREF_ALIN }
2	  --> $DIR/enum.rs:9:1
3	   |
4	LL | enum UninhabitedVariantAlign {

Note: some mismatched output was normalized before being compared
-	error: align: AbiAndPrefAlign { abi: Align(2 bytes), pref: Align(8 bytes) }
-	  --> /home/jyn/src/rust2/tests/ui/layout/enum.rs:9:1
+	error: align: AbiAndPrefAlign { abi: Align(2 bytes), pref: $PREF_ALIN }
```

[pr-133733]: https://github.com/rust-lang/rust/pull/133733

### compiletest now allows using a specific debugger when running debuginfo tests

For a long time, the `tests/debuginfo` test suite could only be successfully run if you had all the tested debuggers (being `lldb`, `gdb`, `cdb`). This is very annoying locally if:

- One or more of these debuggers are not available or don't work locally[^lldb].
- You just wanted to look at the test failures for a given debugger.

Now, you are able to run the `tests/debuginfo` test suite with a *specific* debugger, to only run the tests against that specified debugger. Example usage:

```bash
$ ./x test tests/debuginfo -- --debugger gdb
```

### `ui` tests now support `//@ forbid-output`

`ui` tests can now use the `//@ forbid-output: REGEX` directive to check for a pattern which must not appear in the stderr. If the `REGEX` pattern is matched, then the `ui` test will fail.

Please consult [rustc-dev-guide] for more details.

### `./x test` now accepts a `--no-capture` flag which will be forwarded to compiletest (and thus libtest)

Previously, if you wanted to pass the `--nocapture` flag through [bootstrap], through [compiletest], to the underlying [libtest] runner, on Linux you have to write:

```bash
$ ./x test tests/ui -- --nocapture
```

On native Windows msvc it's even worse, I recall having to write

```powershell
PS> ./x test tests/ui -- -- --nocapture
```

Which is hard to discover and a contributor papercut[^nocapture].

Now, you can just write

```bash
$ ./x test tests/ui --no-capture
```

and bootstrap will forward this flag as `--nocapture` to the underlying [libtest].


## Notable changes

This section is intended to be like a "compatibility notes" but for human test writers.

### `FileCheck`-based test suites no longer predefine `MSVC` and `NONMSVC` `FileCheck` prefixes

In the *current* setup, compiletest will register a [`FileCheck`][filecheck] custom prefix for each compiletest `//@ revision`. However, for historical reasons compiletest also predefined `MSVC` and `NONMSVC` `FileCheck` prefixes depending on the *host*. But this is very surprising for two reasons:

1. It's "spooky action in a distance" because the user never declared these custom prefixes, and these prefixes are conditionally set based on the host. It's hard to debug too.
2. If the user also wants to add their own `//@ revision: MSVC NONMSVC` revisions, because compiletest would pass `--check-prefix` for those names twice, this will cause `FileCheck` to report an error about duplicate prefixes.

Therefore, in [compiletest: don't register predefined `MSVC`/`NONMSVC` `FileCheck` prefixes #134463](https://github.com/rust-lang/rust/pull/134463) we no longer predefine these two `FileCheck` prefixes.

If you want the previous `MSVC` vs `NONMSVC` behavior, you will need to explicitly write out

```rs
//@ revisions: MSVC NONMSVC
//@[MSVC] only-windows-msvc
//@[NONMSVC] ignore-windows-msvc
```

### `normalize-{stderr,stdout}-test` directives are renamed to `normalize-{stderr,stdout}`

Mostly a cleanup, the `-test` suffixes provide no additionally useful information, and only make these two `normalize-*` directives hard to discover.

`normalize-{stderr,stdout}-test` directives are now [renamed to `normalize-{stderr,stdout}`][pr-134759]. `normalize-{stderr,stdout}-{32,64}bit` directives remain unaffected.

[pr-134759]: https://github.com/rust-lang/rust/pull/134759

### compiletest will now deny usages of builtin `FileCheck` suffixes as revision names (for `FileCheck`-based test suites)

For [`FileCheck`][filecheck]-based test suites (`assembly`, `codegen`, `mir-opt`), compiletest will register a custom `FileCheck` prefix for each compiletest `//@ revision`. However, `FileCheck` also has some [builtin suffixes][filecheck-suffixes] such as:

```rust
// COM:
// CHECK:
// CHECK-NEXT:
// CHECK-SAME:
// CHECK-EMPTY:
// CHECK-NOT:
// CHECK-COUNT:
// CHECK-DAG:
// CHECK-LABEL:
```

When combined, this previously meant that the compiletest revision + builtin `FileCheck` suffix constructions such as

```rust
// next:
// same-SAME:
// not-NOT:
// next-NEXT:
// not-SAME:
```

are permitted by compiletest, which are incredibly confusing.

As such, compiletest will now reject `CHECK`, `COM`, `NEXT`, `SAME`, `EMPTY`, `NOT`, `COUNT`, `DAG`, `LABEL` as revision names in `FileCheck`-based test suites.

[filecheck-suffixes]: https://llvm.org/docs/CommandGuide/FileCheck.html#the-check-next-directive


## PR listing

### Improvements

- [rustc-dev-guide]: [Turn `rustc-dev-guide` into a `josh` subtree #134907](https://github.com/rust-lang/rust/pull/134907)
- [compiletest]: [Show the difference between the normalized output and the actual output for lines which didn't match #133733](https://github.com/rust-lang/rust/pull/133733)
- [compiletest]: [Explain that UI tests are expected not to compile by default #133813](https://github.com/rust-lang/rust/pull/133813)
- [compiletest]: [Allow using a specific debugger when running `debuginfo` tests #134629](https://github.com/rust-lang/rust/pull/134629)
- [compiletest], [run-make-support]: [Improve `compiletest` and `run-make-support` symlink handling](https://github.com/rust-lang/rust/pull/134659)
- [compiletest]: [Support `forbid-output` in UI tests #134738](https://github.com/rust-lang/rust/pull/134738)
- [bootstrap]: [Add `--no-capture`/`--nocapture` as bootstrap arguments #134809](https://github.com/rust-lang/rust/pull/134809)
- [bootstrap]: [Allow `./x check compiletest` #134848](https://github.com/rust-lang/rust/pull/134848)
- [compiletest]: [Deny usage of special `FileCheck` suffixes as revision names #134925](https://github.com/rust-lang/rust/pull/134925)
- [tidy]: [Run Python formatting check in `tidy` on CI #134964](https://github.com/rust-lang/rust/pull/134964)
- [tidy]: [Print how to rebless Python formatting in `tidy` #134968](https://github.com/rust-lang/rust/pull/134968)

### Fixes

- [compiletest]: [Fix `--nocapture` for `run-make` tests #134111](https://github.com/rust-lang/rust/pull/134111)
- [compiletest]: [Remove empty 'expected' files when blessing #134808](https://github.com/rust-lang/rust/pull/134808)
- [run-make]: [Fix `assert_stderr_not_contains_regex` #134113](https://github.com/rust-lang/rust/pull/134113)[^oops]

### Cleanups

- [compiletest]: [Don't register predefined `MSVC`/`NONMSVC` `FileCheck` prefixes](https://github.com/rust-lang/rust/pull/134463)
- [compiletest]: [Remove the `-test` suffix from `normalize-*` directives #134759](https://github.com/rust-lang/rust/pull/134759)
- [compiletest]: [Only pass the post-colon value to `parse_normalize_rule` #134840](https://github.com/rust-lang/rust/pull/134840)
- [compiletest]: [Slightly simplify the handling of debugger directive prefixes #134849](https://github.com/rust-lang/rust/pull/134849)
- [bootstrap]: [Consolidate the macros for declaring compiletest test suites #134876](https://github.com/rust-lang/rust/pull/134876)
- [tidy]: [Replace `black` with `ruff` in `tidy` #133821](https://github.com/rust-lang/rust/pull/133821)

### Documentation updates

- [Document how to run the split Docker pipelines #134894](https://github.com/rust-lang/rust/pull/134894)
- [Document the `--dev` flag for `src/ci/docker/run.sh` #134669](https://github.com/rust-lang/rust/pull/134669)
- [rustc-dev-guide]: [`compiletest`: Document the `--debugger` flag #2170](https://github.com/rust-lang/rustc-dev-guide/pull/2170)
- [rustc-dev-guide]: [Document `forbid-output` for UI tests #2171](https://github.com/rust-lang/rustc-dev-guide/pull/2171)
- [rustc-dev-guide]: [Remove the `-test` suffix from normalize directives #2172](https://github.com/rust-lang/rustc-dev-guide/pull/2172)
- [rustc-dev-guide]: [Document `x test --no-capture` #2174](https://github.com/rust-lang/rustc-dev-guide/pull/2174)
- [rustc-dev-guide]: [Describe how to use rust-analyzer with `rmake.rs` #2191](https://github.com/rust-lang/rustc-dev-guide/pull/2191)


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


[^lldb]: For example, I keep having to debug the debugger like `lldb` when the older `lldb` versions keep complaining about `ModuleNotFoundError: No module named '_lldb'`.
[^nocapture]: I don't know about you, but I can never for the life of me remember what the flag is called. I keep thinking it's `--no-capture` when currently libtest only accepts `--nocapture`. Thankfully T-testing-devex FCP'd to add the more conventional version `--no-capture`, looking forward to that. I can also never figure out how many `--` dashes I need, I keep finding myself having to add more `--` until it starts working.
[^oops]: Nyehehehe
