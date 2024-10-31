---
layout: post
title: "This Development-cycle in Cargo: 1.83"
author: Ed Page
team: The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>
---

# This Development-cycle in Cargo: 1.83

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.83.

<!-- time period: 2024-09-06 through 2024-10-17 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [MSRV-aware Cargo](#msrv-aware-cargo)
  - [Shell completions](#shell-completions)
  - [Public/private dependencies](#publicprivate-dependencies)
  - [Optimizing cargo](#optimizing-cargo)
- [Design discussions](#design-discussions)
  - [Target and target](#target-and-target)
  - [Linting](#linting)
  - [Build Script API](#build-script-api)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [`cargo-bloat`](https://crates.io/crates/cargo-bloat) what is taking up space in your binary.

Thanks to [epage](https://github.com/epage) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### MSRV-aware Cargo

*Update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html#msrv-aware-cargo)*

In preparation for documenting the MSRV-aware dependency resolver,
[epage](https://github.com/epage) revamped the dependency resolution documentation
(
[#14620](https://github.com/rust-lang/cargo/pull/14620),
[#14662](https://github.com/rust-lang/cargo/pull/14662),
)
and wrote down the lessons learned on choosing an MSRV from the RFC process
([#14636](https://github.com/rust-lang/cargo/pull/14636)).

Previous, we discussed suboptimal results when resolving dependencies with multiple distinct MSRVs in a workspace.
We were resolving for the lowest MSRV in the workspace.
For a workspace member with any other MSRV,
they would either get a version older than needed for too new.
We proposed resolving for compatibility with the most MSRVs in the workspace.
We made reporting improvements to better communicate this.
However, there was still concerns over newer users unintentionally ending up in this situation and being confused.
With the MSRV policy recommendations we made in
[#14636](https://github.com/rust-lang/cargo/pull/14636),
we are directing users away from this situation.
In comparing with the current state and the remaining problems with the proposed state,
we felt the balance shifted in favor of the proposed state and we merged it in
[#14569](https://github.com/rust-lang/cargo/pull/14569).

At this point, epage posted [#14639](https://github.com/rust-lang/cargo/pull/14639) to stabilize and document the MSRV-aware resolver.
FCP did not close before the end of 1.83,
so expect this for 1.84.

<!-- 2024-09-17 -->
We also discussed and further refined the dependency change output in 
[14568](https://github.com/rust-lang/cargo/pull/14568).

### Shell completions

*Update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html#shell-completions)*

With clap's new completion system integrated into nightly Cargo in
[#14493](https://github.com/rust-lang/cargo/pull/14493),
[shannmu](https://github.com/shannmu) added custom completers in an effort to get parity with the hand-maintained completion scripts, including:
- `--example <TAB>` ([#14531](https://github.com/rust-lang/cargo/pull/14531))
- `--bench <TAB>` ([#14532](https://github.com/rust-lang/cargo/pull/14532))
- `--bin <TAB>` ([#14533](https://github.com/rust-lang/cargo/pull/14533))
- `--test <TAB>` ([#14548](https://github.com/rust-lang/cargo/pull/14548))
- `--target <TAB>` ([#14535](https://github.com/rust-lang/cargo/pull/14535))
- `--registry <TAB>` ([#14656](https://github.com/rust-lang/cargo/pull/14656))
- `cargo update <TAB>` ([#14552](https://github.com/rust-lang/cargo/pull/14552))
- `cargo uninstall <TAB>` ([#14534](https://github.com/rust-lang/cargo/pull/14534))
- `-Z <TAB>` ([#14536](https://github.com/rust-lang/cargo/pull/14536))
- `cargo help <TAB>` ([#14557](https://github.com/rust-lang/cargo/pull/14557))

In testing this out, [epage](https://github.com/epage)
- Fixed a bug with bash ([#5731](https://github.com/clap-rs/clap/pull/5731))
- Made this work with `cargo +nightly` ([clap#5733](https://github.com/clap-rs/clap/pull/5733)) and subsequently fixed `--target` completion for rustup which that broke ([#14564](https://github.com/rust-lang/cargo/pull/14564))
- Made completions err on the side of giving no completions rather than completing file names ([clap#5763](https://github.com/clap-rs/clap/pull/5763))
- Tweaked the display order ([clap#5739](https://github.com/clap-rs/clap/pull/5739), [clap#5741](https://github.com/clap-rs/clap/pull/5741), [clap#5743](https://github.com/clap-rs/clap/pull/5743))
- Removed redundant completions ([clap#5740](https://github.com/clap-rs/clap/pull/5740))

<!-- 2024-10-08 -->
One issue that came up is that `cargo check --bin <TAB>` shows both binary names as well as all `check` options.
Options like `--bin` are defined to optionally take a value (i.e. `--bin [NAME]` or [`num_args(0..=1)`](https://docs.rs/clap/latest/clap/struct.Arg.html#method.num_args)) so that a user without completions can run `cargo check --bin<ENTER>` and see the list of names they can choose from.
Since the new completer is working off of the definitions of the CLI,
it completes `--bin` both for taking a value and not, not knowing that the value is required in practice and it should only complete values.
The ideal solution would be to delegate the "possible values" reporting to clap so users get both the nicer error reporting and high quality completions.
We are tracking this in [#14606](https://github.com/rust-lang/cargo/issues/14606).

In [#14592](https://github.com/rust-lang/cargo/pull/14592), epage made it easier to profile custom completers as Cargo's overhead is important for the usability of this feature.

**For how to try this feature out, see the [documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#native-completions).**

### Public/private dependencies

*Update from [1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#rfc-3516-publicprivate-dependencies)*

While some improvements have been made to public/private dependency support
(
[#14504](https://github.com/rust-lang/cargo/pull/14504),
[#14507](https://github.com/rust-lang/cargo/pull/14507)
),
this feature has been blocked on bugs in the Rustc lint
(
[rust#71043](https://github.com/rust-lang/rust/issues/71043),
[rust#119428](https://github.com/rust-lang/rust/issues/119428),
).
This is blocking other improvements that can build off of knowing which dependencies are private or not, including:
- [`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) being able to identify when a public dependency's major version is bumped which becomes important for inheriting workspace dependencies because action-at-a-distance can be a breaking change
- Speeding up `cargo doc` by skipping unreachable transitive dependencies ([#2025](https://github.com/rust-lang/cargo/issues/2025))
- An alternative to `cargo doc --no-deps` that will generate documentation for all packages relevant to the API
- Smarter heuristics for `cargo add` in picking a version requirements for dependencies
- [A way to keep version requirements in-sync](https://rust-lang.github.io/rfcs/3516-public-private-dependencies.html#caller-declared-relations)

On [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/public.2Fprivate.20deps.20without.20rustc.3F),
[epage](https://github.com/epage) floated the idea of stabilizing the support in Cargo without the Rustc lints.
Alternatively, we could limit the scope of the Rustc lints to err on the side of false negatives over false positives.
This needs more planning and coordination with the compiler team.

### Optimizing cargo

*Update from [1.81](https://blog.rust-lang.org/inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81.html#optimizing-git-sources)*

On [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/.60cargo.20metadata.60.20performance),
some tool authors were discussing with the Cargo team how to optimize `cargo metadata`.

One of the concerns raised was with the serialization / deserialization time of json.
In considering supporting another format, there has to be enough of a difference from json to be worth the effort, it must have a good compatibility story, and we should consider the interoperability of the format.
For example, CBOR was brought up.
While it has an IETF RFC,
the Rust implementations have had some incompatibilities between them and the deserialization time seems to be on par with json
([source](https://github.com/djkoloski/rust_serialization_benchmark)) though real world benchmarks with `cargo metadata` would be needed to tell if it will make a difference.

In other performance news,
[x-hgg-x](https://github.com/x-hgg-x) has been optimizing the dependency resolver
(
[#14663](https://github.com/rust-lang/cargo/pull/14663),
[#14665](https://github.com/rust-lang/cargo/pull/14665),
[#14692](https://github.com/rust-lang/cargo/pull/14692)
).
They also proposed we change the allocator used in Cargo ([#14691](https://github.com/rust-lang/cargo/pull/14691)).
This is being discussed on [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Replace.20system.20allocator.20with.20jemalloc.2Fmimalloc/near/477108091).

## Design discussions

### Target and target

*Update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html#--all-targets-and-doc-tests)*

<!-- 2024-09-24 -->
We previously discussed wanting a `--all-targets-and-doctests` flag.
While that name is good enough for development,
we might not want it when stabilized.
If we had an alternative name for "targets" we could call this `--all-<something>` and deprecate `--all-targets`.
This isn't the first time renaming "cargo target" or "build target" has come up.

In Cargo, the word ["target"](https://doc.rust-lang.org/cargo/appendix/glossary.html#target) can refer to
- An independent build artifact
- The definition and source for a build artifact (see [Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html))
- [`build.target-dir`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget-dir) where build artifacts and their build state are stored, organized by platform triple when cross-compiling
- [`build.target`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) which is the platform triple being built for
- [Manifest `[target]`](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies) and [config `[target]`](https://doc.rust-lang.org/cargo/reference/config.html#target) tables which are ostensibly for a platform triple but allow almost any `cfg` expression

On the CLI, this looks like:
- `--all-targets`
- `--target-dir <PATH>`
- `--target <TRIPLE>`

The language gets muddier because a "build target" is a top-level build artifact for a given `Cargo.toml` file
while a "build unit" or a
["crate"](https://doc.rust-lang.org/cargo/appendix/glossary.html#crate)
includes top-level build artifacts, associated build artifacts (e.g. build scripts, doctests, and unit tests), any of their dependencies needed for building.
To make this even more confusing, while Cargo and Rustc use that definition of "crate", the more commonly known definition is that associated with a 
[`[package]`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section) as used by `cargo publish` and crates.io.

While some users may carefully check the documentation or already be very familiar with these corners of Cargo,
we should still consider the impact on users who are new to Cargo or not had a reason to dive into these corners yet.
Just talking about these topics,
it takes care to disambiguate how the terms are being used.

Focusing specifically on disambiguating "target", `build.target` and `[target]` (roughly platform triple) are heavily ingrained in Cargo and the community and would have a high cost to try to transition away from.
In 1.82, we talked about replacing `build.target-dir` 
([#14125](https://github.com/rust-lang/cargo/issues/14125))
and `--all-targets`
([#6669](https://github.com/rust-lang/cargo/issues/6669))
for other reasons.
That only leaves documentation for where "build target" is used (actually "Cargo target"),
making it the easiest use of the word "target" to change.

In [#14125](https://github.com/rust-lang/cargo/issues/14125),
we were looking at making one of the replacements for `build.target-dir` be `build.artifacts-dir`.
Similarly, when overriding which "build target" is accessed within a dependency's `Cargo.toml`, we were calling that an [artifact dependency](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies).
Our natural inclination is to then use the term "artifact" for "build target".
However, this showed a slight disconnect between how each of these terms is used:
- Cargo only copies into the `build.artifacts-dir` binaries, dynamic libraries, and explicitly-requested-on-the-command-line static libraries and not all "build targets" though there is a [request](https://github.com/rust-lang/cargo/issues/6100#issuecomment-1250332307) for such behavior
- A single "build target" can have multiple output files, e.g. debug files.  We can likely gloss over this.
- "build target" can also refer to the definition and source though this is a bit of a misnomer built from using "build target" as a metonym for the definition

Other names that have come up include
- component
- product
- output

The conversation for this is continuing on [Internals](https://internals.rust-lang.org/t/renaming-cargo-target/21585)

### Linting

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#user-controlled-cargo-diagnostics)*

[Urgau](https://github.com/Urgau) is looking at extending the check-cfg linting system in rustc to Cargo to apply to `target.<cfg>` tables
([#14581](https://github.com/rust-lang/cargo/pull/14581)).
While there are several new-to-Cargo aspects to work out (or defer),
one in particular we discussed was what namespace the lint level and configuration should live under.

The current proposal is to reuse the Rustc lint in Cargo, e.g.
```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(foo)'] }
```

However, this potentially applies to other Rust-like lints and groups we may want in Cargo, including
- [`unknown_lints`](https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#unknown-lints)
- [`non_snake_case`](https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#non-snake-case) (though rustc lacks a `non_kebab_case` for use with binaries, see [#14439](https://github.com/rust-lang/cargo/pull/14439))
- [`deprecated`](https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#deprecated)
- [`warnings`](https://doc.rust-lang.org/nightly/rustc/lints/listing/warn-by-default.html#warnings)

By putting these in `lints.cargo`, users will need to duplicate their `unexpected_cfgs` configuration.
However, other lints like `deprecated` or `non_snake_case` they may want to control separately between Cargo and Rust.

By putting these in `lints.rust`,
- We need to figure out what is responsible for linting for `unknown_lints` that both knows about both Cargo and Rust `rust::` lints and ensures no more than one message is sent per instance
- We need to figure out a process for Cargo extending the namespace with new lints (e.g. any buy-in from T-compiler about adding `rust::non_kebab_case`)
- Cargo will need a way to calculate effective lint levels after processing of lint groups
- We need to decide if `RUSTFLAGS` can also affect Cargo's linting as users are used to it applying to the `rust` namespace

This isn't just for the `rust` namespace.
There is still an open question on whether Cargo should have an explicit lint command that fills a role like `clippy` and if that should be a part of `cargo clippy` somehow.
If so, then Cargo will also have lints in the `clippy` namespace.

This is being discussed on [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Sharing.20of.20lints).

### Build Script API

The Cargo team has been discussing making more official APIs for interacting with Cargo,
for instance an API for build script authors
([#12432](https://github.com/rust-lang/cargo/issues/12432)).

In these discussions, we had overlooked
[RFC 3119](https://rust-lang.github.io/rfcs/3119-rust-crate-ownership.html)
which established policies around packages owned by the Rust Project.
In [#14599](https://github.com/rust-lang/cargo/pull/14599),
we used our [team charter](https://doc.crates.io/contrib/team.html) to override the default process for adding new packages.
We then used that new process in [14600](https://github.com/rust-lang/cargo/pull/14600) to officially declare the support policy for each package in the Cargo repo.

With that out of the way,
we used the process in
[#12432](https://github.com/rust-lang/cargo/issues/12432)
to officially accept a build script API package.

## Misc

- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PugGrub version solving algorithm
- [Flowrey](https://github.com/Flowrey) finished their work on a `--dry-run` flag for `cargo install` ([#14280](https://github.com/rust-lang/cargo/pull/14280))
- [ahaoboy](https://github.com/ahaoboy) added dark mode to `--timings` ([#14588](https://github.com/rust-lang/cargo/pull/14588))
- [Xaeroxe](https://github.com/Xaeroxe) added unstable support using checksums, instead of mtimes, to determine when to rebuild ([#14137](https://github.com/rust-lang/cargo/pull/14137))
- [epage](https://github.com/epage) posted [RFC 3692](https://github.com/rust-lang/rfcs/pull/3692) which came about from several conversations people had with the Cargo team at RustConf
- In response to [#14555](https://github.com/rust-lang/cargo/issues/14555), the Cargo team decided to put the responsibility for mixing programmatic and human output on wrapper tools for now <!-- 2024-10-08 -->

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- [Split CARGO_TARGET_DIR](https://github.com/rust-lang/cargo/issues/14125)
<!--
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
- Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503))
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
- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553)

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
