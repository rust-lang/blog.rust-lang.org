+++
path = "inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81"
title = "This Development-cycle in Cargo: 1.81"
authors = ["Ed Page"]
aliases = ["inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.81

This is a summary of what has been happening around Cargo development for the merge window for Rust 1.81.

<!-- time period: 2024-06-13 through 2024-07-25 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Snapshot testing](#snapshot-testing)
  - [Optimizing git sources](#optimizing-git-sources)
  - [Duplicate package name warning](#duplicate-package-name-warning)
  - [Removing implicit features](#removing-implicit-features)
  - [Garbage collection](#garbage-collection)
  - [Turn all warnings into errors](#turn-all-warnings-into-errors)
  - [Merging `cargo upgrade` into `cargo update`](#merging-cargo-upgrade-into-cargo-update)
  - [`cargo publish --workspace`](#cargo-publish-workspace)
  - [Fingerprinting builds](#fingerprinting-builds)
  - [`cargo info`](#cargo-info)
- [Design discussions](#design-discussions)
  - [`--lockfile-path`](#lockfile-path)
  - [`path-bases`](#path-bases)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo nextest](https://nexte.st/), an alternative to `cargo test`.
[T-testing-devex](https://rust-lang.github.io/rfcs/3455-t-test.html) has been formed to better focus in on improving the test workflow.
We hope to be able to learn from some of `cargo nextest`'s lessons and pull them into `cargo test`.

Thanks to [LukeMathWalker](https://github.com/LukeMathWalker) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

##### Snapshot testing

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#snapshot-testing)*

While there isn't a technical discussion to note, we felt it was important to call out the hard work of the people who participated in porting almost all of Cargo's 3000+ tests from the besoke assertions to `snapbox` as tracked in [#14039](https://github.com/rust-lang/cargo/issues/14039).

- [Muscraft](https://github.com/Muscraft)
- [choznerol](https://github.com/choznerol)
- [dieterplex](https://github.com/dieterplex)
- [eth3lbert](https://github.com/eth3lbert)
- [heisen-li](https://github.com/heisen-li)
- [henry40408](https://github.com/henry40408)
- [linyihai](https://github.com/linyihai)
- [rami3l](https://github.com/rami3l)
- [weihanglo](https://github.com/weihanglo)

We still have a few spots left for the initial pass and then to dig into the corner cases we deferred.

As part of this effort, we've been re-examining our contributor documentation.
[#14272](https://github.com/rust-lang/cargo/pull/14272) was the first step towards improving them.
Related to contributor documentation,
we improved Issue triage instructions ([#14052](https://github.com/rust-lang/cargo/pull/14052)) based on feedback in office hours
and we documented design considerations for RFCs ([#14222](https://github.com/rust-lang/cargo/pull/14222)) based on RFC discussions.

##### Optimizing git sources

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#performance)*

Cargo's no-build overhead came up again [recently](https://www.reddit.com/r/rust/comments/1e0hbc4/media_whats_up_with_cargo_execution_times/).
[Osiewicz](https://github.com/Osiewicz) highlighted [Zed](https://github.com/zed-industries/zed) as a large workspace that might be worth profiling as a representative case.

Zed hit cases we hadn't covered before with trace profiling,
requiring more traces to be added ([#14238](https://github.com/rust-lang/cargo/pull/14238)).
In particular, Zed has some git patches (2 at that time) and git dependencies (12 at that time).

One easy fix was to not calculate how much disk space a git repo filled for our upcoming
[cache garbage collection](https://github.com/rust-lang/cargo/issues/12633),
when the result was already in our database or if GC is disabled
([#14252](https://github.com/rust-lang/cargo/pull/14252)).

The other easily observed overhead for git dependencies will take more care to resolve.

The first is that we fully load all manifests inside a git repo to find the package for the specified dependency name.
We wouldn't need to do this if you could specify the path within a git repo (tracked in [#11858](https://github.com/rust-lang/cargo/issues/11858)).
Without getting into the design work that would entail,
we could likely optimize what we currently do.

There are three distinct expensive phases for loading a manifest
1. Parse the TOML
2. Deserialize the parsed TOML to our types
3. Misc post-processing

All we need is the name, so we could get away with only doing (1).
Except that isn't entirely true because "load all manifests" isn't entirely true.
Initially, we skip manifests in hidden directories and git submodules, only loading them if we follow a path dependency into them.
Likely, the only way to optimize this (without new opt-in features), is if we cache our index of `package.name` to manifest path.
Caching isn't trivial.
We need a place to store the cache.
In case a bug gets fixed related to this, we need to deal with cache invalidation.
If we do this by caching per Cargo version, we then need to deal with removing stale entries.
This isn't impossible but some care is needed.
You can follow progress on this on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Redundant.20code.20in.20.60GitSouce.60.3F/near/450547472).

The other observed slow down is in submodule updates.
We use a sentinel file to know whether we need to checkout the files for a given git commit.
However, that sentinel file only guards the root repo and not any of the submodules which take a significant amount of time to discover and determine that nothing needs to be done.
In resolving this, we need to make sure it works well with interrupted checkouts and works well across Cargo versions.
More discussion is needed on an exact solution.

##### Duplicate package name warning

As we just covered, users specify packages in a git repo by name.
When two packages have the same name,
cargo picks the first one found.
In 1.63 ([#10701](https://github.com/rust-lang/cargo/pull/10701)),
we added a warning when a package name was ambiguous.
However, that didn't take into account whether the user was depending on that package name in the first place.
For example, Cargo has a lot of repeated package names in test data that no one ever references
([#10752](https://github.com/rust-lang/cargo/issues/10752)).

The challenge in fixing this is we split loading a git source into two separate phases: loading all manifests (discarding duplicates), and looking up a manifest by name.
After a lot of refactors (
[#13993](https://github.com/rust-lang/cargo/pull/13993),
[#14169](https://github.com/rust-lang/cargo/pull/14169),
[#14231](https://github.com/rust-lang/cargo/pull/14231)
),
it was easier to identify the impact radius for changing the behavior and to make the change itself which we did in 
[#14239](https://github.com/rust-lang/cargo/pull/14239).

##### Removing implicit features

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#removing-implicit-features)*

During 1.80's development, we found a corner case that failed when it probably shouldn't ([#14016](https://github.com/rust-lang/cargo/issues/14016)).
This came from how easy it is to lose track of all of the meanings of `"dep_name/feature_name"`.
On the surface, it looks like `"dep:dep_name", "dep_name?/feature_name"`.
However, one difference is that `"dep:dep_name"` disables implicit features while `dep_name/feature_name` does not.
Last cycle, we decided to implicitly inject ``"dep:dep_name"`` and finally got to implementing that this cycle
([#14221](https://github.com/rust-lang/cargo/pull/14221)).

<!-- 2024-07-23 -->
However, we again overlooked another difference: `"dep:dep_name"` cannot be used with required dependencies while `"dep_name/feature_name"` can
([#14283](https://github.com/rust-lang/cargo/issues/14283)).

We could just make this conditional on whether the dependency is optional except normalizing features would become dependent on normalizing dependencies when normalizing dependencies is already dependent on normalizing of features.
There are several ways we could break this logical cycle but they rely on making assumptions that work for today and we could be laying a trap for our future selves.
Also, we would be stuck with this complicated design as anything we would do in the future, like re-working `dep_name/feature_name`, would simplify this.
As a team, we discussed whether we should cut removal of implicit features from the 2024 Edition or maybe even try to design out a subset that future `dep_name/feature_name` work.
Two "simple" changes we could make include allowing `dep:dep_name` and/or `dep_name?/feature_name` to be used with required dependencies.
There was some concern that `dep_name?/feature_name` would be confusing as the `?` is meant to imply "maybe enabled".
This will need further follow up to see if there is a way forward without new design work.
When we work on the long-term design for this, we also need to keep in mind that `"dep_name/feature_name"` is more like `"dep_name"?, dep:dep_name"?, "dep_name?/feature_name"`.

To unblock Edition 2024 testing, we reverted [#14221](https://github.com/rust-lang/cargo/pull/14221) in [#14295](https://github.com/rust-lang/cargo/pull/14295).

##### Garbage collection

*(Update from [1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#garbage-collection))*

<!-- 2024-07-16 -->
[ehuss](https://github.com/ehuss) proposed we stabilize a minimal amount of garbage collection: a single control to turn it on, off, or to control the frequency of collection.
Originally, the proposal was to stabilize with the frequency being 6 months and later switching downloaded content to 3 months and download-generated content to 1 month.
The idea being to offer more of a grace window for people working with old versions of Cargo that won't mark items in the cache as used.
However, we decided to keep expectation-setting simple and go with the current numbers.

While discussing the stabilization PR ([#14287](https://github.com/rust-lang/cargo/pull/14287)),
the [name of the config field came up (`gc.auto.frequency`)](https://github.com/rust-lang/cargo/pull/14287#issuecomment-2243770766).
Some users had previously reported that they found the term "GC" confusing in this context.
Technically the term is appropriate and other tools use it to refer to this purpose.
Even if we are fine with the name, we should make sure we've acknowledged the concern in approving the design.
We are further discussing the name in [#14292](https://github.com/rust-lang/cargo/issues/14292).

##### Turn all warnings into errors

A common pattern in projects is to be sloppy on warnings during local development but have CI turn any warnings into errors to prevent warnings from being merged.
Similarly, a developer may want to hide hundreds of warnings to better see the errors that are blocking them.

Currently, the way to fail a build with warnings is by setting `RUSTFLAGS=-Dwarnings` while to hide warnings it is `RUSTFLAGS=-Awarnings` but
- This doesn't help with `rustdoc` which needs `RUSTDOCFLAGS` (which isn't too obvious)
- This won't help with Cargo warnings which we are working to add
- Changing `RUSTFLAGS` between runs doesn't play well with build caching

Previously a `--deny-warnings` flag was proposed ([#8424](https://github.com/rust-lang/cargo/issues/8424)).
To handle the `RUSTFLAGS=-Awarnings` case,
we proposed to generalize this to `--warnings <allow|deny>`.
A first attempt at this was made in [#12875](https://github.com/rust-lang/cargo/pull/12875).

<!-- 2024-07-23 -->
This effort lost traction in working through some of the open questions which we recently discussed.

**Should this apply to all existing Cargo warnings?**
No, Cargo liberally uses the "warning" status and we'd need to audit when these would be reasonably to turn into errors.

**If this were on the CLI, would it be confusing with rustc's `--warn`, `--allow`, `--deny`, and `--forbid` flags?**
Likely not now but if we add them to `cargo` commands, maybe.
We'll start with config-only for now, bypassing this question.

**Where in `.cargo/config.toml` should this live?**
We agreed this shouldn't be in the `term` table.
For now, we are looking at `build.warnings` though some warning cases are outside of builds (e.g. `cargo publish`).

We also discussed the config values.
We need a "reset to default" (currently `warn`).
We need a "warning to error" (currently `deny`).
We were also discussing a "hide warnings" (currently `allow`).
We named these after the behavior of the lint levels.
A `build.warnings = "warn"` would look weird but so would `RUSTFLAGS=--warn=warnings`.

There was some question about the risks with "hide warnings".
Our intent with this is to hide them during development while errors are present.
Of course, having something like [bacon](https://dystroy.org/bacon/) built-in which sorts by level would help.
We aren't there yet.
The big risk is people being too permissive and extending this to their CI.
They can already do `RUSTFLAGS=-Awarnings`, so no real loss.

##### Merging `cargo upgrade` into `cargo update`

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#merging-cargo-upgrade-into-cargo-update)*

[torhovland](https://github.com/torhovland) has been polishing `cargo update --breaking`
([14259](https://github.com/rust-lang/cargo/pull/14259))
and working on adding `cargo update --precise <breaking>`
([#14140](https://github.com/rust-lang/cargo/pull/14140)).
This has uncovered a lot of UX decisions that were left unspecified in
[#12425](https://github.com/rust-lang/cargo/issues/12425)
that juxtapose different workflows against each other and against the non-breaking behavior of `cargo update`, including
- Should `cargo update --precise <breaking>` only apply to direct dependencies or also to transitive dependencies (which would error)?  This gets strange when you have both a direct and transitive path to a dependency.  Should we only update one of them or error?
- How free should cargo be to updating other dependencies to allow a breaking update to occur?  Currently, `--precise` only allows the dependency you specified to be updated unless you pass `--recursive`.  Its likely that a breaking change will have a cascading effect.

Some of these are being worked out on those PRs while others are being left to the [tracking issue](https://github.com/rust-lang/cargo/issues/12425#issuecomment-2186198258).

##### `cargo publish --workspace`

*Update from [1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#cargo-publish-workspace)*

[jneem](https://github.com/jneem) continued work on `cargo package --workspace`.
The first step was to switch `cargo package` to run in stages, first packaging the `.crate` files and then verifying them
([#14074](https://github.com/rust-lang/cargo/pull/14074)).
Once that was in place, they added building multiple `.crate` files
([#13947](https://github.com/rust-lang/cargo/pull/13947)).

##### Fingerprinting builds

[Xaeroxe](https://github.com/Xaeroxe) restarted the conversation around fingerprinting source using checksums, instead of mtimes, on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Implementing.20checksum.20based.20fingerprinting).
Specifically, they were starting work on this and wanted to avoid race conditions between when cargo checksummed the source and when rustc built.
Ideally, rustc would checksum the source to avoid this.
Some past work in this direction was shared.
From there, they created a compiler [MCP](https://github.com/rust-lang/compiler-team/issues/765)
and a [PR](https://github.com/rust-lang/rust/pull/126930) against the compiler.
However, to do this with `build.rs` would require `build.rs` files to opt-in and they would need to be coordinate on checksum algorithms.
We decided to punt on this and leave `build.rs` using mtime for now.

[Xaeroxe](https://github.com/Xaeroxe) posted [#14137](https://github.com/rust-lang/cargo/pull/14137) an initial version of checksum fingerprinting for Cargo.

##### `cargo info`

*Update from [1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html#cargo-info)*

After waiting some time to collect feedback,
[Rustin170506](https://github.com/Rustin170506)
proposed `cargo info` to be merged into cargo ([#14141](https://github.com/rust-lang/cargo/pull/14141)).
This included adapting the code from being written as a third-party to being written in a way Cargo expects, documentation was added, and completion scripts.

There was some discussion on whether to insta-stabilize.
As a third-party command, we are able to get the testing benefits of nightly-only features.
If we don't insta-stabilize it, you could only run it as `cargo +nightly -Zsubcommand info` or run the existing `cargo-info` not as `cargo info` but `cargo-info info`; `cargo info` would fail.
Like with `cargo add` and `cargo remove`, we decided to insta-stabilize.

FCP was started for this command.

## Design discussions

##### `--lockfile-path`

When you (or rust-analyzer) runs `cargo metadata`,
Cargo will ensure the lockfile is in-sync with the `Cargo.toml` file,
generating it if needed.
However, if you do this on a read-only filesystem, then it fails
([#10096](https://github.com/rust-lang/cargo/issues/10096)).

When discussing that problem, the main perspectives were

- `cargo metadata` should be consistent in its output which can only happen if we can write a `Cargo.lock` to the filesystem
- `cargo metadata` should always let me query information, even it means the information changes due to outside changes (e.g. a new version of a dependency is published so it gets selected)

The problem with the latter is user intention.
Today, you can run `cargo metadata --no-deps` and no `Cargo.lock` gets generated so there isn't a problem.
Once you ask for dependencies,
you are asking for dependencies of that instance of the project and an ephemeral state. 

<!-- 2024-07-16 -->
One proposed compromise was a
[`--lockfile-path`](https://github.com/rust-lang/cargo/issues/5707)
flag that allowed callers to override the lockfile location used by the project,
allowing callers to use a writeable location.
[Ifropc](https://github.com/Ifropc) drove the conversation on this and we discussed it further among the Cargo team.

For the design, we decided to be consistent in behavior with `--manifest-path`
- CLI flag, not in the manifest or in config
- `lockfile_path.file_name() == "Cargo.lock"` must be true

On top of that, we gave some pointers on getting starting implementing this feature.

##### `path-bases`

<!-- 2024-06-18 -->
Following in our efforts to
[triage RFCs](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#rfc-triage),
the Cargo team discussed
[RFC: 3529: Add named path bases to cargo (v2)](https://github.com/rust-lang/rfcs/pull/3529)
which tries to make it easier to manage large repos with multiple workspaces by centralizing the knowledge of the path to packages.

<!-- https://github.com/rust-lang/rfcs/pull/3529 -->
<!-- 2024-07-02 -->
<!-- 2024-07-09 -->
It can feel a bit strange for a `Cargo.toml` (project-specific configuration) to be dependent on `.cargo/config.toml` (environment-specific configuration), in this case to get the definition of `base`.
In particular, to better support these roles we've been wanting to find how to support some `.cargo/config.toml` workflows in `Cargo.toml`
([#12738](https://github.com/rust-lang/cargo/issues/12738)).
As a real-world consequence of this, a project using a `base` defined in `.cargo/config.toml` cannot be used as a git dependency or patch.

While the author's use case is focused on cross-workspace dependencies,
specifying `base`s within a large workspace could also be useful
(though sharing dependency specifications through workspace inheritance would also help).
We would want defining of `base`s to be supported in `Cargo.toml` at least.
After some discussion, we decided we were fine with also defining `base`s in `.cargo/config.toml`, layering them.
This is not too different from dependencies referencing a `registry` from `.cargo/config.toml`.

With `[path-bases]` being allowed in `Cargo.toml` comes working out the details, like
- Whether this is a package section, like `[dependencies]`, or a workspace section, like `[profile]`
- If a package section, would it support workspace inheritance and what would the semantics be?

After some discussion, we decided that it was better to build this feature up incrementally,
starting with `[path-bases]` in `.cargo/config.toml` and adding it to `Cargo.toml` at a future point.

<!-- 
##### [--compile-time-deps (for IDEs)](https://github.com/rust-lang/rfcs/pull/3344)

Discussed: 2024-06-18

Wasn't present and no disposition was posted to the RFC
Concerned about accurately representing where the conversation went

-->

## Misc

- [#14224: Note MSRV for features in documentation](https://github.com/rust-lang/cargo/pull/14224)
- On-going work on [#14058: `cargo:rerun-if-env-changed` doesn't work with env configuration](https://github.com/rust-lang/cargo/pull/14058)
- On-going work on [#14280: `cargo install --dry-run`](https://github.com/rust-lang/cargo/pull/14280)
- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PubGrub version solving algorithm

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
<!--
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
-->

Needs design and/or experimentation:
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)
<!--
- [cargo info](https://github.com/rust-lang/cargo/issues/948)
- [GC](https://github.com/rust-lang/cargo/issues/12633)
-->

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [RFC #3371: CARGO_TARGET_BASE_DIR](https://github.com/rust-lang/rfcs/pull/3371)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
- <!-- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553) -->- Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503))

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
