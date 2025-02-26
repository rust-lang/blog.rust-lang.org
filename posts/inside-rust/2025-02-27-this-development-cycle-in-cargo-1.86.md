---
layout: post
title: "This Development-cycle in Cargo: 1.86"
author: Ed Page
team: The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>
---

# This Development-cycle in Cargo: 1.86

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.86.

<!-- time period: 2025-01-09 through 2025-02-20 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Polishing diagnoxtics](#polishing-diagnostics)
  - [`cargo package` VCS dirty checks](#cargo-package-vcs-dirty-checks)
  - [Cargo script](#cargo-script)
  - [Identifying unused `#[test]`s](#identifying-unused-tests)
- [Design discussions](#design-discussions)
  - [`CARGO` environment variable](#cargo-environment-variable)
  - [Specifying supported platforms in packages](#specifying-supported-platforms-in-packages)
  - [Implicitly insert workspace members into `workspace.dependencies`](#implicitly-insert-workspace-members-into-workspacedependencies)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-update](https://crates.io/crates/cargo-update) which checks for and applies updates for `cargo install`ed binaries.  Built-in support for this is being tracked in [#4101](https://github.com/rust-lang/cargo/issues/4101).

Thanks to [Muscraft](https://github.com/Muscraft) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### Polishing diagnostics

Cargo's errors and warnings saw a surprising number of improvements recently.

New diagnostics include:
- [#15014](https://github.com/rust-lang/cargo/pull/15014), by [ranger-ross](https://github.com/ranger-ross), which warns users that something is preventing the caching of the registry index.
- [#15071](https://github.com/rust-lang/cargo/pull/15071), by [linyihai](https://github.com/linyihai), ensures `cargo check --workspace --package invalid` errors just like `cargo check --package invalid`.

Improvements to existing diagnostics:
- [#15093](https://github.com/rust-lang/cargo/pull/15093), by [kornelski](https://github.com/kornelski), which extends workspace-member-load errors with which glob was being expanded.
- [#15133](https://github.com/rust-lang/cargo/pull/15133), by [DJMcNab](https://github.com/DJMcNab), which extends invalid-feature errors with a suggestion of a similar looking feature name.
- [#15185](https://github.com/rust-lang/cargo/pull/15185), by [bk2204](https://github.com/bk2204), which detects a common invalid git URL and suggests a correction.
- [#15138](https://github.com/rust-lang/cargo/pull/15138), by [epage](https://github.com/epage), switches the wording on similar suggestions to more closely match rustc.
- [#15199](https://github.com/rust-lang/cargo/pull/15199), by [linyihai](https://github.com/linyihai), will extend invalid-bin errors from `cargo run` with additional context to understand how to fix them.

### `cargo package` VCS dirty checks

*Update from [1.85](https://blog.rust-lang.org/inside-rust/2025/01/17/this-development-cycle-in-cargo-1.85.html#cargo-publish-dirty-check-performance)*

When investigating the performance regression in `cargo package`,
we found corner cases where a file packaged into a `.crate` could have uncommitted changes while being overlooked in the VCS dirty check.
After [weihanglo](https://github.com/weihanglo)
filled several holes last development cycle,
the remaining piece of
[#14967](https://github.com/rust-lang/cargo/issues/14967)
was workspace inheritance and related features.

<!-- 2025-02-11 -->
If you have the workspace:
```toml
[workspace]
resolver = "3"

[workspace.package]
version = "10.0.0"

[profile]
codegen-units = 1
lto = true
debug = "line-tables-only"
```
and a package
```toml
[package]
name = "foo"
version.workspace = true
```
then the published version will be
```toml
[package]
name = "foo"
version = "10.0.0"
resolver = "3"
```
(see [#8264](https://github.com/rust-lang/cargo/issues/8264) for a discussion on whether `profile` should be copied over)

However, the workspace `Cargo.toml` is not checked for dirty status.
There are a wide range of approaches we can take:
- Check every inheritable field in the package in case workspace inheritance and `workspace.resolver` copying will happen
- Re-generate the packaged `Cargo.toml` from a previous commit in git and diff the two ([#15089](https://github.com/rust-lang/cargo/pull/15089))
- Always consider the workspace `Cargo.toml` to be relevant
- Always consider every file in the repo to be relevant, reducing the chance we'll miss other files as well in the future (e.g. `Cargo.lock`)

After explaining these options to the team,
we decided to start with taking the middle road and checking if the workspace `Cargo.toml` or `Cargo.lock` files are dirty
as this balances complexity and brittleness against maintaining precision in what is checked.

### Cargo script

*Update from [1.84](https://blog.rust-lang.org/inside-rust/2024/12/13/this-development-cycle-in-cargo-1.84.html#misc)*

[Rustin170506](https://github.com/Rustin170506)
finished designing and implementing support for
[Package ID Specifications](https://doc.rust-lang.org/cargo/reference/pkgid-spec.html)
for cargo scripts.
Package ID Specifications aren't relevant for cargo scripts in most places they show up.
For example, cargo scripts can't be workspace members yet where the `--package` flag becomes important.
However, implementing support for this now is important because a package's Package ID Specification can show up in the output of `cargo metadata`
and we need to have the format defined before stabilization.

In a prior PR, we found a gap between rustc and cargo's shebang parsing.
Shebang's are ambiguous with attributes.
A normal shebang may look like:
```rust
#!/usr/bin/env cargo
#![allow(dead_code)]
```
Rustc considers a `#!` at the start of a file that is followed by a `[` to be an attribute, rather than a shebang.

However, rustc allows "whitespace" between `#!` and `[allow(dead_code)]`, like:
```rust
#!/usr/bin/env cargo
#! [allow(dead_code)]
```
So more precisely, the `#!` can have whitespace before the `[` and still be considered an attribute instead of a shebang

What we overlooked is that the comments in rustc's lexer describing this meant to include comments with whitespace, so the following is a valid attribute and not a shebang followed by invalid syntax:
```rust
#!//usr/bin/env cargo
[allow(dead_code)]
```

Following rustc's rules for shebangs would require every frontmatter parser to also understand Rust's grammar for comments.
As this is a new feature, we felt we had some flexibility in how closely we followed rustc and decided to punt on parsing comments which we documented in
[#15173](https://github.com/rust-lang/cargo/pull/15173).
Note that decisions like this are not finalized, just implemented proposals, until stabilization.

[epage](https://github.com/epage) posted [rust#137193](https://github.com/rust-lang/rust/pull/137193)
which adds frontmatter syntax support to `rustc` thanks to the guidance of
[bjorn3](https://github.com/bjorn3),
[jyn514](https://github.com/jyn514),
[Noratrieb](https://github.com/Noratrieb),
and [ytmimi](https://github.com/ytmimi).

Once that PR is merged, the remaining tasks before stabilization are:
1. Add frontmatter syntax support to rust-analyzer
2. Remove cargo's hack around rustc's lack of frontmatter support
3. Improve error reporting in cargo's frontmatter parser
4. Call of testing

Along the way,
epage refactored some code
in [#15168](https://github.com/rust-lang/cargo/pull/15168)
and [#15172](https://github.com/rust-lang/cargo/pull/15172)
to make it less likely for contributors to make mistakes in the future.

epage also started a discussion on [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/rustc.20frontmatter.20extraction.20crate.3F/near/500516745) on making Cargo's frontmatter parser reusable.

### Identifying unused `#[test]`s

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#-zcheck-cfg)*

With `check-cfg` stable,
[Urgau](https://github.com/Urgau) started investigating a gap in the lint reported on
[Users](https://users.rust-lang.org/t/cargo-what-is-the-purpose-of-lib-test-false/102361).
[jplatte](https://github.com/jplatte) wanted to exclusively use integration tests over unit tests.
They set `lib.test = false` and hoped that rustc would report any unused
tests in their `[lib]` through the `check-cfg` lint.
However, `test` is considered an always-present, built-in cfg by rustc.

Several solutions were discussed,
including allowing people to mark built-in cfg's as unknown
[rust#117778](https://github.com/rust-lang/rust/issues/117778).
Urgau proposed in [compiler-team#785](https://github.com/rust-lang/compiler-team/issues/785) for the responsibility of marking `test` as a known config to be on the caller.
When rustc receives `--test`, it implies `--cfg test`.
This enables tests to run as `#[test]` expands to include `#[cfg(test)]`.
In cargo when [`lib.harness = false`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-harness-field),
`--test` is not passed to rustc and so cargo is responsible to pass `--cfg test`.
So the responsibility for `--cfg test` is shared by rustc and the caller.
This was implemented in [rust#131729](https://github.com/rust-lang/rust/pull/131729) and [#15007](https://github.com/rust-lang/cargo/pull/15007).

This lead to false positives in [`core`](https://doc.rust-lang.org/core/) because it sets `lib.test = false` but includes code via `#[path]` with `#[test]`s ([zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/.60test.60.20well.20known.20cfg.20error/near/497171227)).
This can be worked around by setting `lints.rust.unexpected_cfgs.check-cfg = ['cfg(test)']`.

Later on that thread and in [#15131](https://github.com/rust-lang/cargo/issues/15131),
[kpreid](https://github.com/kpreid) called out more significant false positives:
[`lib.test`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-test-field) doesn't mean there are never tests but that tests aren't built and run by default.
In this case, it is fully valid to use `#[test]` or `#[cfg(test)]`.

To give us more time, we decided to revert the change in
[#15132](https://github.com/rust-lang/cargo/pull/15132).
This is being further discussed in [#15131](https://github.com/rust-lang/cargo/issues/15131).

## Design discussions

### `CARGO` environment variable

Cargo passes the path for itself to child processes via `CARGO` in case they need to launch the same version of Cargo
(e.g. in a `build.rs` or an [xtask](https://github.com/matklad/cargo-xtask)).
Several years ago,
[jonhoo](https://github.com/jonhoo) ran into some corner cases with this:

A binary that uses `cargo`-the-library to launch child processes would set `CARGO` to this custom binary that may not be meant to act as a substitute for `cargo`-the-binary
([#10119](https://github.com/rust-lang/cargo/issues/10119)).
Depending on the binary's intent,
there is either no correct answer for what `CARGO` should be for child processes or it should be the value of `CARGO` set on the binary.

When invoking `cargo`-the-binary through `ld` for tight control of the shared library path without propagating it, Cargo looks up its path and gets back the path to `ld`
([#10113](https://github.com/rust-lang/cargo/issues/10113)).
The correct answer is in `argv[0]` but that has lower precedence than
[`current_exe`](https://doc.rust-lang.org/std/env/fn.current_exe.html).
Otherwise, there is no correct answer as the `CARGO` set on `cargo`-the-binary may or may not be related.

In [#11285](https://github.com/rust-lang/cargo/pull/11285),
we gave higher precedence to passing along `CARGO` set on the current process over discovering the current process' path.
This is rightfully causing problems because
a process spawned by one toolchain that itself was spawned by another toolchain
will have `CARGO` set to the outer-most toolchain path,
rather than the inner-most ([#15099](https://github.com/rust-lang/cargo/issues/15099)).

<!-- 2025-02-18 -->
Some options that were discussed on [#15099](https://github.com/rust-lang/cargo/issues/15099) and among the team include:
- `cargo`-the-binary opting in to always override `CARGO` but this misses the `ld` case
- The `cargo` rustup proxy overriding `CARGO` to the path it is about to launch but this does not affect situations outside of rustup, like a cargo script invoking a deb build that invokes the Debian-built `cargo`
- Callers are responsible for unsetting `CARGO` when calling into a different toolchain version but this is non-obvious to do ahead of time and difficult to debug and determine to do when someone runs into this problem.

The proposal we came away with is for `cargo`-the-binary to opt-in to overriding `CARGO` *if* `current_exe` is `cargo{EXE_SUFFIX}`.
While there are still corner cases this can run into problems with
(e.g. custom names for `cargo`-the-binary directly or through symlinks),
this at least shrinks the window for people to be hitting the corner cases.

### Specifying supported platforms in packages

*Update from [1.85](https://blog.rust-lang.org/inside-rust/2025/01/17/this-development-cycle-in-cargo-1.85.html#misc)*

<!-- 2025-01-28 -->
Conversation on [RFC #3759](https://github.com/rust-lang/rfcs/pull/3759) was settling down,
so we reviewed this as a team.

After some initial discussion on the [options for names for the field](https://github.com/rust-lang/rfcs/pull/3759#discussion_r1909195446),
the conversation focused on use cases:

**Build-target filtering (in-scope):**
like [`required-features`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-required-features-field),
automatically filter out any build-target that is not compatible with the selected platform `--target`.

In the Cargo repo, we have `cargo-credential-libsecret`, `cargo-credential-wincred`, and `cargo-credential-macos-keychain`.
To allow `cargo check --workspace`, we've `#[cfg]`ed the implementation of each package so they build on all platforms.
If we had a `required-target` field, we could remove the `#[cfg]`s.

This is in-scope for the RFC as it has the smallest scope to design and implement on top of the definition of the field.

**Error on incorrect use (deferred):**
Potential error cases include:
- I'm building for `--target x86_64-unknown-linux-gnu` but have a `dependencies.windows-sys` dependency
- I have `required-target = 'cfg(target_os="linux")'` but have a `dependencies.windows-sys` dependency
- I have a `target.cfg(target_os="linux").dependencies.windows-sys` dependency,
we could report this as an error.

Say `cargo-credential-wincred` only had an implementation for `#[cfg(windows)]`,
trying to use it on another platform would be reported with an error that `WindowsCredential` could not be found, with a hint that `cfg(windows)` is required.
That is not the friendliest error and would only be found if you validate every supported platform.
Instead, Cargo could report before building anything that this will fail on some of your supported platforms.

The first error case is easy to implement and has precedence for its behavior when building with a dependency with a `package.rust-version` that is older than the current toolchain.
The other two require some work to figure out the intersection of `cfg`s.
In either case, there has been less interest expressed in the use case and there is concern people will be over-zealous in applying the `required-target` field to their packages, preventing their use.
Deferring gives us more time to analyze that problem and more of a tradition of setting the field without extra connotations.

**Vendor only relevant packages (deferred):**
When you run `cargo vendor`, it saves the content of every `.crate` file in your `Cargo.lock`, even if its not needed for the platforms you build on.
When generating or validating `Cargo.lock`,
we could exclude any `target.*.dependencies` that is not compatible with your `required-target`, avoiding the need to vendor them.

Having `required-target` affect dependency resolution would allow a Linux application to not vendor `windows-sys` ([#7058](https://github.com/rust-lang/cargo/issues/7058)).
However, a new field would not be needed for a `target.cfg(target_os= "linux").dependencies`
to exclude any transitive dependencies in a `target.cfg(target_os="windows").dependencies`.
if we supported that,
someone could emulate `required-target` by putting all dependencies behind `target.cfg(...).dependencies`.

There has been a lot of interest in this idea but it adds a lot of complications,
so we initially deferred it out so we could focus on a subset of the conversation.

Tracking all of the `cfg`s that lead to a dependency could get complicated in the dependency resolver.
A 70% solution is we only compare `target.cfg(...).dependencies` against `required-target`,
ignoring any `target.cfg(...).dependencies` along the way that could further narrow things.

This feature could also run up against a general rule of Cargo:
running `cargo check` from a different toolchain version should not modify your `Cargo.lock`:
- Any improvement to the algorithm for evaluating `cfg`s, `Cargo.lock` can change it.  We'd need to keep every implementation and have it selected by either the `Cargo.lock` version field or `workspace.resolver`.
- Any change in the data for `cfg`s could change it.  There is not much we can do about this.

This use case is different than the others in that its mostly focused on application developers.
Vendoring of dependencies is specific to upstream pre-built binaries in which they know the full set of platform tuples being built for.
For downstream distributions,
they will already be decoupling the package from vendored dependencies and could disable this somehow.
If it is that specialized, maybe we should decouple `cargo vendor` / `Cargo.lock` filtering from `required-target` and have the user explicitly enumerate each platform tuple.
This bypasses the `cfg` set logic but still runs into changing definitions of platform tuples.
Presuming a more limited audience,
maybe that level of volatility will be acceptable to them.
This `resolver.targets` could be a config field.
The downside is that your `Cargo.lock` would be dependent on transient or context-sensitive settings but maybe that is fine with the more limited use case.
We might want to record the `resolver.targets` in `Cargo.lock` so that unexpected changes from missing or changed config are clear.
Maybe more important is that if this config were used when running `cargo publish`,
then `cargo install --locked` could end up failing.

Another direction we discussed was for `required-target` to be a subset of `cfg` functionality, like only `target_os`.
This might allow us to make some simplifying assumptions but we'd need to work closely with T-compiler to ensure those assumptions are upheld.

We didn't end up reaching a specific conclusion and will need to consider this further.

### Implicitly insert workspace members into `workspace.dependencies`

<!-- 2025-02-11 -->
We discussed a proposal by [CinchBlue](https://github.com/CinchBlue) to act as if workspace members were added to `workspace.dependencies`
([#13453](https://github.com/rust-lang/cargo/issues/13453)).
Since Cargo already needs to discover the location of packages,
this would remove the need to give Cargo information it already has,
reducing friction when moving packages in a repo.

In effect, this would treat:
```toml
# Cargo.toml
[workspace]
members = ["crates/*"]

# crates/foo/Cargo.toml
[package]
name = "foo"
[dependencies]
bar.workspace = true

# crates/bar/Cargo.toml
[package]
name = "bar"
```
like
```toml
# Cargo.toml
[workspace]
members = ["crates/*"]
[workspace.dependencies]
foo.path = "crates/foo"
bar.path = "crates/bar"

# crates/foo/Cargo.toml
[package]
name = "foo"
[dependencies]
bar.workspace = true

# crates/bar/Cargo.toml
[package]
name = "bar"
```

The first challenge is determining what the `workspace.dependencies` entry should look like.
For example, the `version` field is generally required to publish a package.
One exception is when user intentionally leave off `version` for dev-dependencies to workaround
[publish cycles](https://github.com/rust-lang/cargo/issues/4242).
However, we don't know if packages are intended to be published or not
because packages default to `package.publish = true`,
Tracking all of that to determine how to implicitly fill `workspace.dependencies` would also be complicated.
Likewise, we'd need to figure out how to handle `registry` and `default-features` fields.

Speaking of complicated designs,
consider the following example:
```toml
# Cargo.toml
[workspace]
members = ["crates/*"]
[workspace.package]
version = "2.0.0"
[package]
name = "foo"
version.workspace = true
[dependencies]
bar.workspace = true

# crates/bar/Cargo.toml
[package]
name = "bar"
version.workspace = true
```

To load `Cargo.toml`, we'd need to
1. Parse `Cargo.toml`
2. Discover and parse `crates/bar/Cargo.toml`
3. Apply `Cargo.toml`s `workspace.package` to `crates/bar/Cargo.toml` to ensure `version` is set if needed
4. Add `crates/bar/Cargo.toml` to `Cargo.toml`s `workspace.dependencies`
5. Apply `Cargo.toml`s `workspace.dependencies` to `Cargo.toml`

Or put another way,
we'd need to load the workspace members in multiple passes,
ensuring we are only operating on initialized data in each pass.

Already we feel that we are at the limits of our complexity budget for parsing `Cargo.toml` and this would exceed that.

Weighing all of that against the workaround of manually populating `workspace.dependencies`,
the latter doesn't seem so bad.
We could even smooth that out by having `cargo new` inject new workspace members into `workspace.dependencies` ([#15180](https://github.com/rust-lang/cargo/issues/15180)).
A user would then be free to edit the entries to suit their needs.

With `cargo new` automatically populating `workspace.dependencies` and `cargo add` automatically using it,
this would shift the ecosystem over to using it and the question came up on whether `workspace.dependencies` is mature enough for this.
There are caveats with the feature itself, like [known issues with `default-features`](https://github.com/rust-lang/cargo/issues/12162).
There are also caveats with the workflows around it, like tracking breaking changes.
Without `workspace.dependencies`,
you can look at every commit in a directory to look for breaking changes
(tools like [`cargo release changes`](https://github.com/crate-ci/cargo-release) help with that).
However, that won't be the case in the following scenario:
```toml
# Cargo.toml
[workspace]
members = ["crates/*"]
[workspace.dependencies]
dep = { path = "crates/dep", version = "1.0.0" }

# lib/Cargo.toml
cargo-features = ["public-dependency"]
[package]
name = "lib"
[dependencies]
dep = { workspace = true, public = true }
```
You could bump `dep`s version to `2.0.0`, breaking users of `lib` and not be able to tell by looking at `git log lib/`.
[cargo semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)
might be able to help with this today but stabilization of `public-dependency` would make this trivial to add such a check but that is blocked on some bugs in the rustc lint (
[rust#71043](https://github.com/rust-lang/rust/issues/71043),
[rust#119428](https://github.com/rust-lang/rust/issues/119428)
).

## Misc

- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PugGrub version solving algorithm
- Support for `resolver.feature-unification = "workspace"` was implemented in [#15157](https://github.com/rust-lang/cargo/pull/15157) by [aliu](https://github.com/aliu) *(update from [1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html#misc))*
- [epage](https://github.com/epage) posted [RFC #3772](https://github.com/rust-lang/rfcs/pull/3772) to deprecate non-package `edition` fields, like `lib.edition` <!-- 2025-02-11 -->
- [ranger-ross](https://github.com/ranger-ross) started work on splitting out `build-dir` from `target-dir` in [#15104](https://github.com/rust-lang/cargo/pull/15104) *(update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html#target-dir-and-artifact-dir))*
- [osiewicz](https://github.com/osiewicz)'s [MCP](https://github.com/rust-lang/compiler-team/issues/790) for avoiding rebuilds when a dependency's implementation changes was accepted and on [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Support.20for.20.22Relink.20Don't.20Rebuild.22.20in.20Cargo/near/495600409) there was some discussion on how to handle line number changes when diagnostics show output from dependencies.

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Project goals in need of owners
- [Open namespaces](https://rust-lang.github.io/rust-project-goals/2025h1/open-namespaces.html)
- [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h1/pub-priv.html)
- [Prototype a new set of plumbing commands](https://rust-lang.github.io/rust-project-goals/2025h1/cargo-plumbing.html)

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
<!--
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- [Split CARGO_TARGET_DIR](https://github.com/rust-lang/cargo/issues/14125)
- Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503))
- [Config control over feature unification](https://github.com/rust-lang/cargo/issues/14774)
-->

Needs design and/or experimentation:
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)
- [GC](https://github.com/rust-lang/cargo/issues/12633)

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
<!--
- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553)
-->

## How you can help

If you have ideas for improving cargo,
we recommend first checking [our backlog](https://github.com/rust-lang/cargo/issues/)
and then exploring the idea on [Internals](https://internals.rust-lang.org/c/tools-and-infrastructure/cargo/15).

If there is a particular issue that you are wanting resolved that wasn't discussed here,
some steps you can take to help move it along include:
- Summarizing the existing conversation (example:
  [Better support for docker layer caching](https://github.com/rust-lang/cargo/issues/2644#issuecomment-1489371226),
  [Change in `Cargo.lock` policy](https://github.com/rust-lang/cargo/issues/8728#issuecomment-1610265047),
  [MSRV-aware resolver](https://github.com/rust-lang/cargo/issues/9930#issuecomment-1489089277)
  )
- Document prior art from other ecosystems so we can build on the work others have done and make something familiar to users, where it makes sense
- Document related problems and solutions within Cargo so we see if we are solving to the right layer of abstraction
- Building on those posts, propose a solution that takes into account the above information and cargo's compatibility requirements ([example](https://github.com/rust-lang/cargo/issues/9930#issuecomment-1489269471))

We are available to help mentor people for
[S-accepted issues](https://doc.crates.io/contrib/issues.html#issue-status-labels)
on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo)
and you can talk to us in real-time during
[Contributor Office Hours](https://github.com/rust-lang/cargo/wiki/Office-Hours).
If you are looking to help with one of the bigger projects mentioned here and are just starting out,
[fixing some issues](https://doc.crates.io/contrib/process/index.html#working-on-issues)
will help familiarize yourself with the process and expectations,
making things go more smoothly.
If you'd like to tackle something
[without a mentor](https://doc.crates.io/contrib/issues.html#issue-status-labels),
the expectations will be higher on what you'll need to do on your own.
