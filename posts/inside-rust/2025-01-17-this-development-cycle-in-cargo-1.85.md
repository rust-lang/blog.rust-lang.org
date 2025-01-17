---
layout: post
title: "This Development-cycle in Cargo: 1.85"
author: Ed Page
team: The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>
---

# This Development-cycle in Cargo: 1.85

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.85.

<!-- time period: 2024-11-29 through 2025-01-09 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Rustflags and caching](#rustflags-and-caching)
  - [Report progress in the taskbar](#report-progress-in-the-taskbar)
  - [`cargo publish` dirty-check performance](#cargo-publish-dirty-check-performance)
  - [Future-proofing the Index](#future-proofing-the-index)
- [Design discussions](#design-discussions)
  - [Project goals](#project-goals)
  - [Automatic retry for `cargo publish`](#automatic-retry-for-cargo-publish)
  - [`cargo publish` verify behavior](#cargo-publish-verify-behavior)
  - [Codifying build script patterns](#codifying-build-script-patterns)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our choice for this cycle is [clippy-sarif](https://crates.io/crates/clippy-sarif).  While not a plugin, it wraps `cargo clippy`, converting the lint output into [Static Analysis Results Interchange Format (SARIF)](https://sarifweb.azurewebsites.net) which GitHub can use to post comments on a PR review for lints, making them easier to access than reading a log file.

Thanks to [epage](https://github.com/epage) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### Rustflags and caching

*Update from [1.84](https://blog.rust-lang.org/inside-rust/2024/12/13/this-development-cycle-in-cargo-1.84.html#rustflags-and-caching)*

As a recap, Cargo throws out the cache on changes to `RUSTFLAGS`,
instead of creating separate entries in the cache.
The problem is `RUSTFLAGS` is opaque to Cargo and different flags have different caching needs.
The remaining problem is `--remap-path-prefix` which is used to make two different builds look the same.
The parameters to the flag are unique to each build and the cache key ends up in the final binary,
preventing `--remap-path-prefix` from serving its purpose.

<!-- 2024-12-03 -->
We talked about this a Cargo team meeting.
Once we have built-in support for `--remap-path-prefix` through
[trim-paths](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#profile-trim-paths-option),
we could just point people to that instead.
However, there are open issues with trim-paths and its unclear how long that will take to become available.
We also could go back to the fix for this from the previous attempt and parse `RUSTFLAGS`
(see [#6966](https://github.com/rust-lang/cargo/pull/6966))
but we're uncomfortable with that because parsing of command-lines is context-sensitive.
We ended up with a middle-ground where we apply a rough heuristic where if something looks like `--remap-path-prefix`,
we do not include `RUSTFLAGS` in our cache key.

This was merged in [#14830](https://github.com/rust-lang/cargo/pull/14830)

### Report progress in the taskbar

Previously, [Gordon01](https://github.com/Gordon01)
picked up work on [ConEmu's](https://conemu.github.io/en/AnsiEscapeCodes.html#ConEmu_specific_OSC) OSC 9;4 which is an ANSI escape code they defined to report progress through the Windows taskbar icon
([#14615](https://github.com/rust-lang/cargo/pull/14615)).
Support for reporting this has extended to applications like systemd
([systemd#34929](https://github.com/systemd/systemd/pull/34929))
while support for reporting this has extended to applications like
[Ptyxis](https://blogs.gnome.org/chergert/2024/12/03/ptyxis-progress-support/).

<!-- 2024-12-17 -->
With an FCP started, the team briefly discussed whether this needs to be added to `.cargo/config.toml`.
We previously added configuration control for other terminal features,
like unicode and hyperlinks.
We don't exclusively turn on these features because not all terminals gracefully degrade in their presence.
In particular for the Taskbar escape code, 
Kitty had made their own feature using OSC 9,
causing a poor experience with OSC 9;4
([kitty#8011](https://github.com/kovidgoyal/kitty/issues/8011)).
This gives us a way for users to test the feature in case their terminal isn't in our allow list or to turn it off if they don't like the results.

The discussion then moved to
[#14615](https://github.com/rust-lang/cargo/pull/14615#discussion_r1888730634),
particularly focusing on what to name the config field.
Currently, the field is proposed to go in the `term.progress` table.
As these are de facto standards, there is no officially defined name for this feature.
We could name it for the escape code (`term.progress.osc9_4`),
the original intent (`term.progress.taskbar`),
how progress is being reported (e.g. `term.progress.ansi`),
or one of a myriad of options.

Ideally, the config field would be:
- Easy discover for people wanting to change this behavior
- Clear meaning when looking at a config or config docs
- Consistent with the wider community

At this time, we have not come to a consensus on a name.

### `cargo publish` dirty-check performance

When running `cargo package` or `cargo publish`,
Cargo records the git commit hash and path within the repo into a file in the `.crate` file.
This was skipped if `--allow-dirty` was used which changed in Cargo 1.81
to help with efforts in auditing published packages
([#13960](https://github.com/rust-lang/cargo/pull/13960)).

[landonxjames](https://github.com/landonxjames) reported that this change caused a significant perfornce regression when publishing the
[aws-sdk-rust repo](https://github.com/awslabs/aws-sdk-rust).
Cargo went from skipping its dirty file check with `--allow-dirty` to always running it.
The check has not been optimized for publishing over 400 packages at once.
For example, Cargo checks if any file in the repo is dirty.
[weihanglo](https://github.com/weihanglo) reduced the scope of this check to just the package directory in
[#14962](https://github.com/rust-lang/cargo/pull/14962)
but we held off on that change as this drew attention to shortcomings in the dirty check
([#14967](https://github.com/rust-lang/cargo/issues/14967))
and we were concerned that fixing those problems would conflict with the performance improvement.

weihanglo set out to test that theory by fixing several of the problems in
[#14962](https://github.com/rust-lang/cargo/pull/14962),
particularly for files that Cargo copies into the package root
(
[#14981](https://github.com/rust-lang/cargo/pull/14981),
[#14966](https://github.com/rust-lang/cargo/pull/14966)
).
They tried to further optimize this in
[#14985](https://github.com/rust-lang/cargo/pull/14985)
though it didn't seem worth the complexity.

With the approach taken above,
path filtering isn't an issue so we moved forward with it in 
[#14997](https://github.com/rust-lang/cargo/pull/14997).

We are waiting to hear back to see if more work is needed for optimizing this.

### Future-proofing the Index

When discussing new features for Cargo,
changes to the Index come up.
The Index is a collection of basic metadata (called Summaries)
for every package version in a registry to allow dependency resolution without downloading a `.crate` file and parsing the `Cargo.toml`.
Purely additive changes to a Summary are fine,
like adding `package.rust-version`.
We can extend existing fields or change the interpretation of them but the experience isn't ideal for old versions of Cargo.

Back when Cargo 1.60 was released,
users on older versions of Cargo started to see errors like
```
error: failed to select a version for the requirement `libgit2-sys = "^0.13.3"`
candidate versions found which didn't match: 0.13.2+1.4.2, 0.13.1+1.4.2, 0.13.0+1.4.1, ...
location searched: crates.io index
required by package `git2 v0.14.3`
    ... which satisfies dependency `git2 = "^0.14"` (locked to 0.14.3) of package `repor v0.1.0 (/home/epage/src/personal/repro)`
```
When you go on crates.io, version `0.13.3` exists.
The problem is `0.13.3` used the new [weak dependency feature syntax](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features) (`dep?/feature`) which older Cargos do not understand.
This error message is a poor user experience for weak dependency features and any future extensions of the Summary schema.
We've been tracking this usability problem in [#10623](https://github.com/rust-lang/cargo/issues/10623).

[epage](https://github.com/epage) did some recent work (
[#14897](https://github.com/rust-lang/cargo/pull/14897),
[#14921](https://github.com/rust-lang/cargo/pull/14921),
[#14923](https://github.com/rust-lang/cargo/pull/14923),
[#14927](https://github.com/rust-lang/cargo/pull/14927)
) so Cargo can extract as much information as it can from unsupported Summaries, without losing this information through caching, and report it to the user.

The following output shows three different ways we might report an unsupported Summary, depending on how much context we have:
```
[UPDATING] `dummy-registry` index
[ERROR] failed to select a version for the requirement `foo = "^0.1.1"`
  version 0.1.3 requires cargo 1.2345
  version 0.1.4 requires a Cargo version that supports index version 1000000000
  version 0.1.5's index entry is invalid
location searched: `dummy-registry` index (which is replacing registry `crates-io`)
required by package `a v0.5.0 ([ROOT]/foo)`
```
We also report yanked versions now:
```
[UPDATING] `dummy-registry` index
[ERROR] failed to select a version for the requirement `baz = "=0.0.2"`
  version 0.0.2 is yanked
location searched: `dummy-registry` index (which is replacing registry `crates-io`)
required by package `bar v0.0.1`
    ... which satisfies dependency `bar = "*"` of package `foo v0.0.1 ([ROOT]/foo)`
```

With the improved error reporting,
we are more free to design Index related features.

## Design discussions

### Project goals

Cargo-related goals that have been proposed for 2025h1 include
- [build-std](https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html): led by [AdamGemmell](https://github.com/AdamGemmell) and the Rust team at Arm
- [Continue resolving cargo-semver-checks blockers for merging into cargo](https://rust-lang.github.io/rust-project-goals/2025h1/cargo-semver-checks.html): led by [obi1kenobi](https://github.com/obi1kenobi) 
- [Extend pubgrub to match cargo's dependency resolution](https://rust-lang.github.io/rust-project-goals/2025h1/pubgrub-in-cargo.html): led by [eh2406](https://github.com/eh2406)
- [Finish the libtest json output experiment](https://rust-lang.github.io/rust-project-goals/2025h1/libtest-json.html): led by [epage](https://github.com/epage)
- [Prototype a new set of Cargo "plumbing" commands](https://rust-lang.github.io/rust-project-goals/2025h1/cargo-plumbing.html): in need of a task owner for Cargo work
- [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h1/pub-priv.html): in need of a task owner for compiler work
- [Implement Open API Namespace Support](https://rust-lang.github.io/rust-project-goals/2025h1/open-namespaces.html): in need of a task owner for compiler work

### Automatic retry for `cargo publish`

[ijackson](https://github.com/ijackson) proposed that cargo automatically backoff and retry when hitting crates.io rate limits in
[#13530](https://github.com/rust-lang/cargo/issues/13530).

<!-- 2024-12-10 -->
The two main concerns we had were:

Does this follow the intent of crates.io in setting a rate limit? 
We reached out and they said they are fine with it from a technical perspective.

Is this the user experience we want?
The retry times are quite long and users might be unhappy with `cargo publish` unexpectedly "hanging" for several minutes,
even worse if they are publishing a workspace
([#1169](https://github.com/rust-lang/cargo/issues/1169)).
If the user decided to cancel a workspace publish with ctrl-c,
they could be left in a half-published state.
Having an `--idempotent` flag to skip versions already published
([#13397](https://github.com/rust-lang/cargo/issues/13397))
could help.
If this is part of a larger release process,
like with [`cargo-release`](https://crates.io/crates/cargo-release),
then recovery on failure can be messy.
In `cargo-release`'s case,
it guesses whether an operation will be rate limited and blocks the user,
requiring them to override it.
We did not look into what canceling a CI-based release,
like with [`release-plz`](https://github.com/MarcoIeni/release-plz),
would look like.
Instead of `cargo-release` guessing whether the rate limit would be hit,
crates.io could provide an API to query this
however the crates.io team is worried about the compatibility burden this may put on them to further evolve the rate limits.

When crates.io came back with their feedback on this,
they did bring up the idea of adjusting the crate-version rate limit from being scoped per-user to being per-crate.
This is being tracked in [crates.io#10396](https://github.com/rust-lang/crates.io/issues/10396).

### `cargo publish` verify behavior

When testing `cargo publish --workspace` ([#1169](https://github.com/rust-lang/cargo/issues/1169)),
[epage](https://github.com/epage) realized that the verify step is running `cargo build`, rather than `cargo check`.
While this likely doesn't matter for publishing,
it can have a dramatic effect when iterating on a release process with the `--dry-run` flag.
They proposed this get changed in [#14930](https://github.com/rust-lang/cargo/pull/14930)
but after some back and forth,
it turned out there might be more nuance in this and the discussion shifted to
[#14941](https://github.com/rust-lang/cargo/issues/14941).

Something to keep in mind is that `cargo check` is not guaranteed to find every compiler error that `cargo build` may find (see [RFC #3477](https://github.com/rust-lang/rfcs/pull/3477)).
More specifically, these cover post-monomorphizaton errors which includes linker errors.

<!-- 2024-12-17 -->
We discussed this in the recent Cargo team meeting.
Already someone has asked for verification support to be configurable so it can run tests
([#14685](https://github.com/rust-lang/cargo/issues/14685)).
If cargo had that, this only becomes a question of defaults.
Without it, we would be locking people into an answer which puts more pressure on the decision.
However, there are several questions to work out for
[#14685](https://github.com/rust-lang/cargo/issues/14685)
that could slow down this issue if we block on it.

An important question for us to answer is what the default purpose of the verify step should be
- Sanity check packaging (e.g. were all needed files included)
  - *If* you consider testing of a `.crate` file important, then running tests is the only way to sanity check it because they can rely on data files, see #14685 for more details
  - For `-sys` crates, there is the possibility of a missing file causing linker errors
  - Only checking one combination of features or one profile could also mask problems with missing files
- Last-ditch baseline quality check
  - In case no CI
  - In case changes were made locally before publishing, particularly with `--allow-dirty`

If its a last-ditch baseline quality check, then running `build` adds extra assurances in light of
[RFC #3477](https://github.com/rust-lang/rfcs/pull/3477).
The question then becomes, what is the right default baseline?
- A fast "most build problems" check
- Current: The default feature on a `dev` profile on the current platform
- Different feature combinations?
- Different platform combinations?
- Different profile combinations?
- Linting, testing, unsafe checking, or formal proofs

Whether the differences between `cargo check` and `cargo build` are relevant depends on factors like:
- Our general assumption of what users do before publishing (`cargo test`? run all of CI?)
- The likelihood that a step from above is missed before release
- The likelihood of post-monomorphization errors being introduced in that gap
- The number of affected users if a problem does slip through (are more popular crates likely to have more rigorous processes?)

The scariest part of rust-lang/rfcs#3477 is that a crate that builds with `cargo check` is not subject to Rust's stability guarantees, only `cargo build`.  However, a crate that can only be checked and not built is of little use and is unlikely people could depend on that.

We decided to reach out on
[Internals](https://internals.rust-lang.org/t/performance-of-cargo-package-cargo-publish-and-the-purpose-of-the-verify-step/22135)
to get more feedback on the role of verify and the impact of its performance.

### Codifying build script patterns

Inspired by our interest in finding abstractions for `RUSTFLAGS` 
([#12739](https://github.com/rust-lang/cargo/issues/12739)),
[epage](https://github.com/epage) started tracking uses of `build.rs`
in [#14948](https://github.com/rust-lang/cargo/issues/14948)
so we can explore what are gaps in Rust and Cargo that need filling

If nothing else, we can use
[metabuild](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#metabuild)
for custom declarative build scripts.
To simplify the design and to align with other features,
epage [posted a counter proposal](https://github.com/rust-lang/cargo/issues/14903#issuecomment-2523803041) that builds on 
[artifact dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies).
Compared to the current design, this avoids duplicate build and linking of build script binaries
and allows finer grained tracking of when to re-run them.
This might even allow the running of build scripts to be conditioned on features.
[joshtriplett](https://github.com/joshtriplett) suggested consolidating the ways to pass data to build scripts, which epage agreed with.
Whats left unclear is whether the data should be low level (literal environment variables) or abstracted (some configuration that gets converted to environment variables).

## Misc

- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PugGrub version solving algorithm
- [ranger-ross](https://github.com/ranger-ross) improved build detection fingerprinting in [#14973](https://github.com/rust-lang/cargo/pull/14973)
- [ranger-ross](https://github.com/ranger-ross) improved diagnostics with Index update failures in [#14973](https://github.com/rust-lang/cargo/pull/15014)
- [epage](https://github.com/epage) added `CARGO_CFG_FEATURE` to build scripts in [#14902](https://github.com/rust-lang/cargo/pull/14902)
- [carloskiki](https://github.com/carloskiki) posted [RFC #3759](https://github.com/rust-lang/rfcs/pull/3759) for specifying targets required for building a package after hosting a [Pre-RFC on Internals](https://internals.rust-lang.org/t/pre-rfc-allow-packages-to-specify-a-set-of-supported-targets/21979/39)

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
