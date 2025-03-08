+++
layout = "post"
title = "This Development-cycle in Cargo: 1.84"
author = "Ed Page"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

# This Development-cycle in Cargo: 1.84

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.84.

<!-- time period: 2024-10-18 through 2024-11-28 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Simple english in documentation](#simple-english-in-documentation)
  - [Build Script API](#build-script-api)
  - [Replacing mtimes with checksums](#replacing-mtimes-with-checksums)
  - [Rustflags and caching](#rustflags-and-caching)
  - [Snapshot testing](#snapshot-testing)
  - [JSON schema files](#json-schema-files)
- [Design discussions](#design-discussions)
  - [Improving the built-in profiles](#improving-the-built-in-profiles)
  - [Avoid building production code when changing tests](#avoid-building-production-code-when-changing-tests)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-hack](https://crates.io/crates/cargo-hack) makes it easy to verify different feature combinations work together and that you can build for all supported Rust versions.

Thanks to [epage](https://github.com/epage) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### Simple english in documentation

trot approached the Cargo team on
[zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Difficult.20ESL.20words.20Cargo.20Book/near/482282496)
about making the
[Cargo book](https://doc.rust-lang.org/cargo/)
more approachable for people for whom English is a second language.
After some discussion, we decided to start with simplifying the language used in the Cargo book.

[KGrewal1](https://github.com/KGrewal1) took the lead on this and posted [#14825](https://github.com/rust-lang/cargo/pull/14825).
They also made the language more consistent in [#14829](https://github.com/rust-lang/cargo/pull/14829).

### Build Script API

*Update from [1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html#build-script-api)*

With the Cargo team approving owning `build-rs`,
[epage](https://github.com/epage)
worked with
[CAD97](https://github.com/CAD97/)
and
[pietroalbini](https://github.com/pietroalbini)
to transfer publish rights for [build-rs](https://crates.io/crates/build-rs) to the Rust Project.

CAD97 then did a first-pass review and update to `build-rs` and epage merging it into cargo ([#14786](https://github.com/rust-lang/cargo/pull/14786)).
epage then did a pass to update `build-rs` in [#14817](https://github.com/rust-lang/cargo/pull/14817).

On [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Where.20.60build-rs.60.20shoud.20live/near/480798285),
[Turbo87](https://github.com/Turbo87) raised concernd about `build-rs` (and Cargo-maintained crates more generally) being in the Cargo repo and tied to the Cargo release process.
This means that there is a 6-12 week delay between a bug fix being merged and bein released, projects that need access to unstable functionality must use a git dependency, the MSRV is infectious which puts pressure on the Cargo team to bump it regularly, and the issues are mixed together.
On the other hand, Cargo support, documentation, and APIs are able to developed hand-in-hand.
It would be great if we could improve the release process within the Cargo repo (e.g. [#14538](https://github.com/rust-lang/cargo/issues/14538))
but keeping that in sync with 3 parallel release channels (stable, beta, nightly), including leaving space to patch an arbitrary number of crate releases for each release channel,
makes this difficult.

### Replacing mtimes with checksums

*Update from [1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html#misc)*

<!-- 2024-10-22 -->
With unstable support for using [checksums, rather than mtime, to determine when a build is dirty](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#checksum-freshness),
the Cargo team discussed the path for stabilization.

One hole in the current design is that Cargo doesn't checksum inputs to build scripts.
If Cargo did the checksum on the user's behalf,
then it might see a different version of the file than the build script.
However, requiring build scripts to checksum the files and report that to Cargo adds a significant complexity to build scripts, including coordinating with Cargo on what hash algorithm is used.
This would also require build scripts to pull in a hasher dependency, increasing their build times.
However, it is unclear if checksumming for build scripts is a requirement.
Also, if we could develop a plan to reduce the need for build scripts, we reduce the scope of the problem.

Another concern is with performance.
The overhead of checksumming will be most noticeable on builds without any changes as otherwise compile times will dominate.
We are further tracking this in [#14722](https://github.com/rust-lang/cargo/issues/14722).

There is the question of what this will look like.
Do we exclusively switch to checksums or give people the choice?
At minimum, we'd need to give people a temporary escape hatch as we transition Cargo to checksums in case they are somehow relying on the mtime behavior.
Whether Cargo would need a permanent config field is unclear.

We reached out to some build system owners on
[zulip](https://rust-lang.zulipchat.com/#narrow/channel/334885-t-cargo.2Fbuild-integration/topic/Call.20for.20testing.3A.20use.20of.20checksums.20over.20mtime/near/478311654)
to do a call-for-testing.
So far, we've only heard from the Rust Project itself where this made build time testing more difficult because touching files to force rebuilds is a much easier option than trying to carefully make repeated edits to files.

### Rustflags and caching

Cargo's [build fingerprinting](https://doc.rust-lang.org/nightly/nightly-rustc/cargo/core/compiler/fingerprint/index.html) has to satisfy several needs, including
- Detecting when the existing build cache has to be thrown away and re-built, called the fingerprint
- Segregating the build cache so it gets preserved across unrelated commands (e.g. alternating `cargo check` and `cargo test` without rebuilding dependencies), called [`-Cextra-filename`](https://doc.rust-lang.org/rustc/codegen-options/index.html#extra-filename)
- Making symbol names unique so you can't unintentionally use a type in an ABI-incompatible context, called [`-Cmetadata`](https://doc.rust-lang.org/rustc/codegen-options/index.html#metadata)

`RUSTFLAGS` is a way to bypass Cargo's abstractions and directly control the behavior of `rustc`.
Cargo includes `RUSTFLAGS` in the fingerprint hash but not in the `-Cextra-filename` hash,
causing a full rebuild when they change.
This can be especially problematic when `RUSTFLAGS` differs between the user and their editor running `cargo`.
For example, some users report they set `--cfg test` in their editor so all `#[cfg(test)]`s are enabled in rust-analyzer.

A previous attempt was made to segregate the cache for `RUSTFLAGS` in
[#6503](https://github.com/rust-lang/cargo/pull/6503).
However, Cargo uses the same hash for `-Cextra-filename` and `-Cmetadata`
so by segregating the cache, the symbol names also become unique.
In theory, this is a good thing as `RUSTFLAGS` can affect the ABI.
However, not all `RUSTFLAGS` affect the ABI.

Take [`--remap-path-prefix`](https://doc.rust-lang.org/rustc/command-line-arguments.html#--remap-path-prefix-remap-source-names-in-output) which is suppose to make builds of binaries more reproducible by stripping information specific to a specific build.
By including this in `-Cmetadata`,
the binary changes ([#6914](https://github.com/rust-lang/cargo/issues/6914)).
A special case for this was added [#6966](https://github.com/rust-lang/cargo/pull/6966).

Another case we ran into was [PGO](https://doc.rust-lang.org/rustc/profile-guided-optimization.html).
With PGO, you create a build with
[`-Cprofile-generate`](https://doc.rust-lang.org/rustc/codegen-options/index.html#profile-generate)
and then run it against a benchmark.
You then feed this back into the build with
[`-Cprofile-use`](https://doc.rust-lang.org/rustc/codegen-options/index.html#profile-use)
to improve the optimizations the compiler performs.
At this point, we reverted 
[#6503](https://github.com/rust-lang/cargo/pull/6503)
in
[#7417](https://github.com/rust-lang/cargo/pull/7417).

In [#8716](https://github.com/rust-lang/cargo/issues/8716),
[ehuss](https://github.com/ehuss) proposed Cargo track `-Cextra-filename` and `-Cmetadata` separately and only include `RUSTFLAGS` in `-Cextra-filename`.

After some refactoring
([#14826](https://github.com/rust-lang/cargo/pull/14826))
and test improvements
(
[#14848](https://github.com/rust-lang/cargo/pull/14848),
[#14846](https://github.com/rust-lang/cargo/pull/14846),
[#14859](https://github.com/rust-lang/cargo/pull/14859)
)
by [epage](https://github.com/epage) and [weihanglo](https://github.com/weihanglo/),
epage posted [#14830](https://github.com/rust-lang/cargo/pull/14830).
However, weihanglo found there are still problems with `--remap-path-prefix`: even when using `profile.dev.split-debuginfo="packed"`, the binaries are different because the binary includes `DW_AT_GNU_dwo_name` which points to the debug file which exists per-rlib with `-Cextra-filename` included.

Merging of [#14830](https://github.com/rust-lang/cargo/pull/14830) is blocked until the problem with `--remap-path-prefix` is resolved.

### Snapshot testing

*Update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html#snapshot-testing)*

[epage](https://github.com/epage) finalized the work for moving off of Cargo's custom assertions.

In removing the core of the custom assertions,
we were relying on `dead_code` warnings as we removed assertions
that were no longer used.
However, we missed removing an assertion and epage removed it in [#14759](https://github.com/rust-lang/cargo/pull/14759).

[#14781](https://github.com/rust-lang/cargo/pull/14781) and
[#14785](https://github.com/rust-lang/cargo/pull/14785) saw us migrate the last of our "unordered lines" assertion tests.
[#14785](https://github.com/rust-lang/cargo/pull/14785) took some investigation to figure out the best way to migrate.
Cargo's custom assertions redacted fewer values and allowed a test author to ignore a value redaction by using the raw value in the expected result.
`snapbox` applies redactions earlier in the process, requiring them to always be used.
This made it so Cargo would lose test coverage in switching to snapbox as we wouldn't be verifying as much of `cargo`s output.
However, in consulting with the test author, coverage of those redacted values was not intended by them, bypassing this problem for now.

This still left "contains", "does not contain", and "contains x but not y" assertions.
Rather than trying to design out how these should fit into snapbox,
epage left them after switching to `snapbox`'s redactions in [#14790](https://github.com/rust-lang/cargo/pull/14790).

As this point, epage documented lessons learned through this effort in [#14793](https://github.com/rust-lang/cargo/pull/14793) and we now consider this migration complete, closing out [#14039](https://github.com/rust-lang/cargo/issues/14039).

### JSON schema files

In [#12883](https://github.com/rust-lang/cargo/issues/12883),
we got a request for JSON schema support for `.cargo/config.toml` files.
We already have to duplicate the schema between the source and the documentation, we didn't want to duplicate it with a hand-maintained JSON schema representation.
Thankfully there is [schemars](https://crates.io/crates/schemars) to generate JSON schema from serde types.

To experiment with JSON schema generation, 
[dacianpascu06](https://github.com/dacianpascu06) added support for JSON schema generation for `Cargo.toml` in
[#14683](https://github.com/rust-lang/cargo/pull/14683),
see [manifest.schema.json](https://github.com/rust-lang/cargo/blob/master/crates/cargo-util-schemas/manifest.schema.json).

Generating a JSON schema for `.cargo/config.toml` will take a bit more investigation.
`Cargo.toml` has a single top-level definition with specific extension points within the schema.
`.cargo/config.toml` does not have a single top-level definition but instead the schema is defined per table or field.
This is because config layering operates on the specific path that is looked up.
The types for the schema are scattered throughout the Cargo code base and will take work to collect them all together to create a top-level definition solely for JSON schema generation.

## Design discussions

### Improving the built-in profiles

<!-- 2024-10-29 -->
Hand-in-hand with benchmarking is profiling yet the `bench` profile does not include the relevant debug information for profiling,
requiring users to tweak their profiles in every repo (or in their home directory).
[CraftSpider](https://github.com/CraftSpider) proposed in
[#14032](https://github.com/rust-lang/cargo/issues/14032)
that we update the `bench` profile to make this easier.
However, benchmarks also need fidelity with `release` builds to ensure your numbers match what users will see.
We decided we should keep the `bench` profile matching `release` though we recognize there is room to explore improving user workflows for profiling.

[foxtran](https://github.com/foxtran) restarted the conversation on changing the defaults for `release` to improve runtime performance in
[#11298](https://github.com/rust-lang/cargo/issues/11298).

Potential changes include
- Enabling LTO, whether thin or fat
- Reducing the codegen-units
- Increasing the opt-level

While `release` builds aren't focused on fast compile-times,
there is still a point of diminishing returns in trading off compile-time for runtime performance.
While `release` is generally geared towards production builds,
there are cases where `dev` is too slow for development.
[weihanglo](https://github.com/weihanglo) ran the numbers of LTO and codegen-units for Cargo in
[#14719](https://github.com/rust-lang/cargo/issues/14719).
From those numbers, it seems like thin LTO is an easy win.

One option is for a `release-fast` or `release-heavy` to be made.
Adding new profiles may be a breaking change though and we'd have to carefully approach doing so.
We also already have discoverability problems with `release` and it has a dedicated flag (`--release`).
Without some kind of built-in integration, it seems like these policies would be best left for users.

Whatever profile is used, one problem with LTO is that there are miscompilations which might prevent it from being a safe default (e.g. [#115344](https://github.com/rust-lang/rust/issues/115344)).

<!-- 2024-11-05 -->
On the other end of the spectrum is the `dev` profile.
This profile serves two roles
- Fast iteration time
- Running code through a debugger

It turns out that these can be at odds with each other.
When running through a debugger, you often want the binary to behave like the source code and optimizations can get in the way.
However, optimizations can reduce the amount of IR being processed, speeding up codegen.
They can then also speed up proc macros, build scripts, and test runs.
Maybe we can even design a optimization level focused on improving compile times at the cost of the debugger experience.
Similarly, how much debug information you carry around in your binary can affect your build times.

Looking at the [Rust 2023 survey results](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html#challenges),
improving compilation times and the debugging experience is neck and neck.
The question is which debugging experience are they referring to?
Those on the call mostly used "printf"-style debugging and would get benefit out of improving compilation time.
Even if we surveyed people and found this representative of the Rust community (which [davidlattimore](https://github.com/davidlattimore/) did for a subset of the community on [Fediverse](https://mas.to/@davidlattimore/113484821980790635)),
how much of this is survivorship bias from the quality of the debugger experience?
How much would even existing community members behavior change with an improved debugger experience?

However, this may not be all-or-nothing.
We could split the `dev` profile into separate iteration-time and debugger profiles so there is a low friction way of access the non-default workflow.
There would still be friction.
If iteration-time were the default and enough people use debuggers through their IDEs and those IDEs are pre-configured,
then working with IDE vendors to change their defaults would reduce a lot of the friction.
This would likely require a long transition period.

We could split one of the two workflows out into a whole new profile which runs into the same problems as `release-fast` and `release-heavy`.

One idea for address the potential breakage is that we move the built-in profiles into a `cargo::` namespace and make them immutable.
We would switch the reserved profiles to just inheriting a namespaced profile by default.
There are open questions on whether this would be a breaking change and more analysis would be needed.

Instead of reserving a new profile name, what if Cargo used the reserved `debug` name?
`debug` is already a reserved profile name and in several user-facing locations the `dev` profile is referred to as `debug` (`--debug`, `target/debug`).
We could make `dev` (`--dev`) focused on iteration time and `debug` (`--debug`) on debuggers.
There is the question of `target/debug` as changing users to `target/dev` might be too disruptive.

It will take work to finish a plan and figuring out if its too disruptive.
If can move forward with it, it will likely require a long transition time and support across multiple projects.

Is this change worth it?
[joshtriplett](https://github.com/joshtriplett) ran a survey on
[Internals](https://internals.rust-lang.org/t/feedback-request-performance-improvements-from-reducing-debug-info/21825)
on the affect of just
[`CARGO_PROFILE_DEV_DEBUG=line-tables-only`](https://doc.rust-lang.org/cargo/reference/profiles.html#debug)
on compilation time
with some follow up conversation on [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Debug.20information.20and.20build.20time/near/481165988).

Another angle for improving iteration time for `dev` is to make it easier to speed up dependencies in the hot path.
Cargo allows you to set different optimization levels for different dependencies and some projects encourage this, like [sqlx](https://github.com/launchbadge/sqlx/tree/42ce24dab87aad98f041cafb35cf9a7d5b2b09a7?tab=readme-ov-file#compile-time-verification):
```toml
[profile.dev.package.sqlx-macros]
opt-level = 3
```
What if packages could provide a
[package override](https://doc.rust-lang.org/cargo/reference/profiles.html#overrides)
for when they are used as a dependency?

Another potential use case for dependency-specified profile overrides is for mir-only rlibs.
Cargo performs codegen for each rlib in your dependency tree and relies on the linker to remove anything unused.
Mir-only rlibs would defer all codegen to the very end, allowing less codegen to be performed, potentially speeding up builds.
This has the potential to replace the need for `[features]` for a large number of use cases.
One problem is if there is a lot of shared mir between test binaries as that will lead to redundant codegen, slowing down builds.
One way to experiment with this is to allow enabling mir-only rlibs on a per-package basis through profiles.
With dependency-specified profile overrides, large packages like
[windows-sys](https://crates.io/crates/windows-sys)
could opt-in to being a mir-only rlib.

Dependency-specified profile overrides would be a hidden interaction that would need careful consideration.

### Avoid building production code when changing tests

[milianw](https://github.com/milianw) posted on
[zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/upstream.20test-only.20code.20change.20triggers.20downstream.20rebuilds/near/478396625) 
about their library and all dependents rebuild when changing a unit test.

When a `#[test]` inside of a library changes,
the timestamp for the file changes and Cargo rebuilds the file.
One way to avoid that is by
[moving tests to dedicated files](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html#Assorted-Tricks).
The rust repo does this with [a tool](https://github.com/rust-lang/rust/blob/HEAD/src/tools/tidy/src/unit_tests.rs) to enforce the practice.
[epage](github.com/epage) proposed a clippy lint for this in [rust-clippy#13589](https://github.com/rust-lang/rust-clippy/issues/13589).

When a library changes, Cargo always rebuilds dependents.
Previously, [Osiewicz](https://github.com/Osiewicz) proposed on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Dynamically.20pruning.20jobs.20from.20the.20work.20queue)
that rustc hash the API of a crate, allowing Cargo to only rebuild dependents when the API hash changes.
This is being tracked in
[#14604](https://github.com/rust-lang/cargo/issues/14604).

## Misc

- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PugGrub version solving algorithm
- Building on [epage](github.com/epage)'s work in [#14750](https://github.com/rust-lang/cargo/pull/14750), [linyihai](https://github.com/linyihai) diagnostics with extraneous details in [#14497](https://github.com/rust-lang/cargo/pull/14497).
- [Rustin170506](https://github.com/Rustin170506) updated how config files are loaded for cargo script in [#14749](https://github.com/rust-lang/cargo/pull/14749)
- [epage](github.com/epage) updated frontmatter parsing for cargo script in [#14792](https://github.com/rust-lang/cargo/pull/14792) and got manifest-editing commands updated to support cargo script in [#14857](https://github.com/rust-lang/cargo/pull/14857) and [#14864](https://github.com/rust-lang/cargo/pull/14864)
- [arlosi](https://github.com/arlosi) wrapped up work on `CARGO_BUILD_WARNINGS=deny` in [#14388](https://github.com/rust-lang/cargo/pull/14388) (update from [1.81](https://blog.rust-lang.org/inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81.html#turn-all-warnings-into-errors))

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Config control over feature unification](https://github.com/rust-lang/cargo/issues/14774)
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
- [Split CARGO_TARGET_DIR](https://github.com/rust-lang/cargo/issues/14125)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
<!--
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
