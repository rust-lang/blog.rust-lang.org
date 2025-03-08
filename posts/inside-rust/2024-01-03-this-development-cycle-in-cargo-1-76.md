+++
layout = "post"
date = 2024-01-03
title = "This Development-cycle in Cargo: 1.76"
author = "Ed Page"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

We wanted to share what has been happening for the last 6 weeks to better keep the community informed and involved.
For work that was merged before branching for 1.76 beta, it will be in the Beta channel for the next 6 weeks after which it will be generally available.

This is distinct from [This Week in Rust](https://this-week-in-rust.org/) in that it tries to focus more on the big picture, rather than individual PRs, and pulls from more sources, like Cargo Team meetings and [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo).

This is an experiment in finding better ways to be engaged with the community and we'll see how well it works and how well we can keep up on it.

<!-- time period: 2023-11-16 through 2023-12-28 -->

## Merged

##### Managing growth of Cargo

The Cargo team has been working to scale our processes to allow the number of packages in the workspace to grow
- For [1.74](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html#cargo-registry-authentication), we provided an [API](https://crates.io/crates/cargo-credential) for third-party [credential providers](https://doc.rust-lang.org/cargo/reference/credential-provider-protocol.html)
- We are interested in providing more first-party APIs interacting with cargo like for [build scripts](https://github.com/rust-lang/cargo/issues/12432) and [env variables set during the build](https://github.com/rust-lang/cargo/issues/12431)
- Other projects need access to some of cargo's logic without wanting to pull in a large, monolithic library, like for [crates.io verifying `Cargo.toml` files on `cargo publish`](https://github.com/rust-lang/crates.io/pull/6914)
- We are hoping that we can improve the contributor experience with smaller packages (faster to build and test) with clearer boundaries (easier to reason about)

<!-- source: 2023-11-21 meeting -->
We've had a couple of breakages affecting people over the last year while we reflected on some recent regressions.  Examples include:
- When making future-incompatible updates to the output of `cargo metadata`, not coordinating with the third-party `cargo_metadata` API ([oli-obk/cargo_metadata#240](https://github.com/oli-obk/cargo_metadata/issues/240))
- Confusion over dependencies on `cargo-credential` causing non-working dependency trees when building these packages from the crates.io ([rust-lang/cargo#13004](https://github.com/rust-lang/cargo/pull/13004))

Some potential improvements include
- Once we support [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169), we could explore verifying published packages in CI (related: [taiki-e/cargo-hack#216](https://github.com/taiki-e/cargo-hack/issues/216))
- Explore having more first-party APIs for interacting with cargo so the feedback loop is tighter

As a first step in stretching our ability to scale,
we split out [`Cargo.toml` serde types](https://docs.rs/cargo-util-schemas)
from cargo itself
([rust-lang/cargo#12801](https://github.com/rust-lang/cargo/issues/12801)).

Other areas for potentially splitting out of the `cargo` library include:
- Move serde and CLI types into `cargo-util-schemas`
- Console output
- Parsing and layer merging for `.cargo/config.toml`
- Reading from different package sources (git, path, git registry, sparse registry)

##### Long-path support

A user ran into path-length issues on Windows with `cargo install --git`
([rust-lang/cargo#13020](https://github.com/rust-lang/cargo/issues/13020))
which led to [ChrisDenton](https://github.com/ChrisDenton) to post a PR for 
[embedding a Windows manifest into the cargo binary](https://github.com/rust-lang/cargo/pull/13131), modeled after `rustc`.
After some exploration on that PR, it was merged with
[rust-lang/cargo#13141](https://github.com/rust-lang/cargo/issues/13141)
being created to track some of the remaining work
(which is in tandem with [rust-lang/cargo#9770](https://github.com/rust-lang/cargo/issues/9770)).

When interacting with git, there are some notes in
[rust-lang/cargo#13020](https://github.com/rust-lang/cargo/issues/13020)
on some additional config settings to workaround problems.

##### Stabilizing `cargo metadata`'s `id` field

Currently, `cargo metadata`'s package `id` field is defined to be [opaque](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html#compatibility).
The problem is you can't take a package from the output and pass it to `cargo <cmd> --package <value>`.
You could use the `name` field but that can be ambiguous when there are multiple, incompatible versions in your `Cargo.lock`.
`name` is a subset of the [Package ID Specs](https://doc.rust-lang.org/cargo/reference/pkgid-spec.html) format which most `--package` parameters accept.
[rust-lang/cargo#12914](https://github.com/rust-lang/cargo/pull/12914) proposes we switch `id` to be Package ID Spec and declare it as non-opaque in `cargo metadata`'s output,
allowing a caller to take the `id` and pass it cargo via the `--package` parameters.

We did find one hurdle: Package ID Specs can sometimes be ambiguous.
That was resolved in [rust-lang/cargo#12933](https://github.com/rust-lang/cargo/pull/12933).

This is waiting on input from the Cargo team.

##### `-Ztrim-paths`

[`-Ztrim-paths`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#profile-trim-paths-option) is an unstable feature that provides different options to sanitize paths embedded in final binaries.
This improves privacy and reproducibility when shipping and sharing artifacts without sacrificing the debugging experience.

`-Ztrim-paths` is generally usable and [weihanglo](https://github.com/weihanglo/) has been driving the effort to get this stable.
Recently, they fixed an issue when sanitizing the package in the current working directory ([rust-lang/cargo#13114](https://github.com/rust-lang/cargo/pull/13114)) and added end-to-end tests to ensure the debugging experience does not regress ([rust-lang/cargo#13091](https://github.com/rust-lang/cargo/pull/13091) and [rust-lang/cargo#13118](https://github.com/rust-lang/cargo/pull/13118)).

There are some symbols that are not sanitized yet, for example `N_SO` and `N_OSO` symbols on macOS or `DW_AT_comp_dir` on Linux when split-debuginfo enabled.
This is tracked in [rust-lang/rust#117652](https://github.com/rust-lang/rust/issues/117652). 

When sanitizing paths, we remap the start of the path to an identifier.
The current remap rules make it difficult to configure a debugger to [remap to the source on their system](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Source-Path.html#set-substitute_002dpath).
Alternative remap rules are being discussed in [rust-lang/cargo#13171](https://github.com/rust-lang/cargo/issues/13171).
An important consideration being raised is that the users can successfully remap in their debugger regardless of endianness or bit-width which is important for cross-platform debugging.

##### `-Zcheck-cfg`

[`-Zcheck-cfg`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#check-cfg)
is an unstable feature that will cause `rustc` to warn on undefined conditional compilation, like `#[cfg(unknown)]` or `#[cfg(feature = "unknown")]`.

[Urgau](https://github.com/Urgau) has been working across `rustc` and `cargo` to polish up this feature for stabilization.
Recently, they:
- Stopped checking names/values on the `rustc --cfg` CLI flag ([rust-lang/rust#117522](https://github.com/rust-lang/rust/pull/117522)) after a [zulip thread](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/check-cfg.20and.20RUSTFLAGS.20interaction) and some [discussion in the tracking issue](https://github.com/rust-lang/rust/issues/82450#issuecomment-1813687060)
- Fixed cargo so that it will re-compile packages when features change, avoiding a stale warning status ([rust-lang/cargo#13012](https://github.com/rust-lang/cargo/pull/13012))
- Fixed `--check-cfg` when no features are present ([rust-lang/cargo#13011](https://github.com/rust-lang/cargo/pull/13011))
- Added all known`target_feature` values ([rust-lang/rust#118908](https://github.com/rust-lang/rust/pull/118908))
- Add more suggestions for the user to the `--check-cfg` diagnostic ([rust-lang/rust#118213](https://github.com/rust-lang/rust/pull/118213))

[Urgau](https://github.com/Urgau) is hoping to start stabilization discussion during the 1.77 development-cycle.

##### RFC #3516 (public/private dependencies)

[RFC #3516 (public/private dependencies)](https://github.com/rust-lang/rfcs/pull/3516)
was merged which will help users identify when they leak their dependencies in their public API,
helping prevent unintentional breaking changes.  This is behind [`cargo-features = ["public-dependency"]`](https://doc.rust-lang.org/cargo/reference/unstable.html#public-dependency).
A good amount of the implementation was done as part of the superseded
[RFC #1977](https://github.com/rust-lang/rfcs/pull/1977).

[linyihai](https://github.com/linyihai) has stepped in to help implement the remaining Cargo work, including:
- Configuring the `public` field on dependencies via `cargo add` ([rust-lang/cargo#13046](https://github.com/rust-lang/cargo/pull/13046))
- Limiting the lints to libraries ([rust-lang/cargo#13135](https://github.com/rust-lang/cargo/pull/13135))
- Verifying the behavior with recursive public dependencies ([rust-lang/cargo#13183](https://github.com/rust-lang/cargo/pull/13183))

Other Cargo work includes
- Deciding against use of the `public` field in `workspace.dependencies` ([rust-lang/cargo#13125](https://github.com/rust-lang/cargo/pull/13125))
- Removing code left behind from RFC 1977 ([rust-lang/cargo#13036](https://github.com/rust-lang/cargo/pull/13036))
- Confirming how we can help people migrate with `cargo fix` ([rust-lang/cargo#13095](https://github.com/rust-lang/cargo/issues/13095))
- Exploring how we can further improve the lint ([rust-lang/cargo#13096](https://github.com/rust-lang/cargo/issues/13096))

The hope is to have this ready for 2024 Edition.
The [tracking issue](https://github.com/rust-lang/rust/issues/44663) enumerates what work is remaining.
The biggest risks are likely:
- Cases where `rustc` should warn but doesn't
([rust-lang/rust#71043](https://github.com/rust-lang/rust/issues/71043)
- `rustc` not correctly tracking the transitive nature of a dependency being `public` ([rust-lang/rust#119428](https://github.com/rust-lang/rust/issues/119428))

##### User-controlled cargo diagnostics

The Cargo Team is very cautious about adding warnings to Cargo because there is nothing like `rustc`s `--allow ...` / `#[allow(...)]` to suppress them when needed.
This changed with the introduction of the [`[lints]` table](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html#lint-configuration-through-cargo).
We are tracking cargo warning control (and the lints it can unblock) in
[rust-lang/cargo#12235](https://github.com/rust-lang/cargo/issues/12235).

The first milestone is for TOML parse errors to match `rustc`'s error style, going from
```
error: failed to parse manifest at `[..]`

Caused by:
  TOML parse error at line 6, column 25
    |
  6 | build = 3
    | ^
  invalid type: integer `3`, expected a boolean or string
```
to
```
error: invalid type: integer `3`, expected a boolean or string
--> Cargo.toml:6:25
  |
6 | build = 3
  | ^
```
Rather than writing our own error message renderer that imitates `rustc`,
[Muscraft](https://github.com/Muscraft) resurrected the
[`annotate-snippets` project](https://github.com/rust-lang/annotate-snippets-rs/)
with the intention of making it work for `cargo` and then migrating `rustc` to it.
They released annotate-snippets [v0.10](https://github.com/rust-lang/annotate-snippets-rs/blob/master/CHANGELOG.md#0100---december-12-2023)
and created
[rust-lang/cargo#13172](https://github.com/rust-lang/cargo/pull/13172)
for integrating it into cargo when parsing `Cargo.toml` files.

We will also need to decide what to do about
[the differences in colors between `rustc` and `cargo`](https://github.com/rust-lang/cargo/issues/12740).
[Muscraft](https://github.com/Muscraft) has been looking into why `rustc`'s colors were chosen and are preparing a proposal for what both programs should use.

##### `cargo info`

We've had a request for a [`cargo info` command](https://github.com/rust-lang/cargo/issues/948) for close to a decade.
[hi-rustin](https://github.com/hi-rustin/), a regular Cargo contributor,
has taken on designing such a command.

You can try it out by running
```
$ cargo install cargo-information
$ cargo info clap
```
Ideas and feedback are welcome!
See the [Issue tracker](https://github.com/hi-rustin/cargo-information/issues).

## Postponing RFCs

<!-- source: 2023-12-05 meeting -->
The Cargo team is looking to clean up the backlog of open RFCs.

**[RFC #3379 (`os_version` predicates for `#[cfg]`)](https://github.com/rust-lang/rfcs/pull/3379):**
I'll defer to the summary on the RFC:

> I'm going to propose to postpone this RFC. I think we all agree that this would be a great thing to have, but I think there are some big questions, particularly around how version support of pre-built std works, how it might tie into supporting [target requirements](https://github.com/rust-lang/cargo/issues/6179), how the version information is determined, etc. Primarily, there isn't anyone on the team who has the capacity at this time to champion this feature.

**[RFC #3177 (`[patch]` dependencies using unidiff patchfiles)](https://github.com/rust-lang/rfcs/pull/3177):**
We think this would be very useful but there are a lot of details to work out and no one on the team is able to help shepherd this effort.
Taking a lesson from other teams and from the
[cargo script eRFC](https://github.com/rust-lang/rfcs/pull/3424),
we felt the best way for this to move forward is for someone to sketch out a rough proposal and then implement it as an unstable feature as an experiment.
This experiment would be focused on learning the things we need to figure out what should be in the RFC.

**[RFC #3287 (native code coverage support)](https://github.com/rust-lang/rfcs/pull/3287):**
We are very much interested in improving the developer experience around coverage but unclear if that RFC is the right approach
(e.g. [rust-lang/cargo#13040](https://github.com/rust-lang/cargo/issues/13040) includes an alternative).
Like with RFC #3177, we need to run an experiment to flesh out the design for this.

<!-- source: 2023-12-12 meeting -->
**[RFC #3263 (don't treat pre-releases as compatible with each other)](https://github.com/rust-lang/rfcs/pull/3263):**
We fully recognize that this is a problem.
For example, Clap ran into this with the 3.0 pre-releases and is the reason Clap stopped using pre-releases.
However, there were a lot of questions that have remained unresolved within the RFC for the last year and no one on the Cargo team has the availability to help drive these conversations.
A viable short-term solution would be to use the proposed [cargo linting system](https://github.com/rust-lang/cargo/issues/12235)
to warn users that don't pin their pre-release version requirements with `=` in their `Cargo.toml` file.
As an alternative for short-term testing of pre-releases, users can `[patch]` in the dependency's git repo.
For more extensive use of immature APIs or behavior, Clap has been exposing them through the convention of `unstable-` prefixed features that are documented as having no semver guarantees.
[rust-lang/cargo#10881](https://github.com/rust-lang/cargo/issues/10881) proposes native support for unstable features.
We recognize this does not help when a library is going through a more extensive breaking change and there is still a place for pre-releases.

<!-- source: 2023-12-05 meeting -->
**Action item:** We do need to go back and document the experiment process so we can more easily point people to the expectations for running one.

## Design discussions

##### Meta: 2024 Edition

<!-- source: 2023-12-19 meeting -->

With [the window soon closing for the 2024 Edition](https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html),
we explored whether there is anything else we should attempt to slip in.

Currently, we have planned:
- [RFC #3516 (public/private dependencies)](https://github.com/rust-lang/rfcs/pull/3516)
- [RFC #3491 (remove implicit features)](https://github.com/rust-lang/rfcs/pull/3491)

with the possibility of [RFC #3537 (MSRV-aware resolver)](https://github.com/rust-lang/rfcs/pull/3537) being tied to an Edition.

We brainstormed other ideas including:

**[Disabling of default features](https://github.com/rust-lang/cargo/issues/3126):**
We see this as a part of the bigger picture for making it easier to evolve features,
particularly taking built-in functionality and putting it behind a feature without it being a breaking change.
This likely doesn't need an Edition on its own but we also talked about *if* we want to remove `default-features = false` that that would require an Edition.
However, we aren't sure if that is what we want,
we would not want to rush that design,
and we should have a large deprecation window before the switch.

<!-- source: also https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/doctest-xcompile -->
**[Cross-compile Doctests](https://github.com/rust-lang/cargo/issues/7040):**
Currently, we skip doctests when using `--target` and this feature changes it so we start running them.
Switching to this behavior is likely to break people.
Testing out `-Zdoctest-xcompile` on [rust-lang/rust](https://github.com/rust-lang/rust/) saw no errors with `--target=armhf-gnu` and `--target=arm-android` had a few in std.
From a user surprise perspective, we feel like people would be more surprised to find out we are silently skipping doctests rather than being surprised to see compilation errors.
Maybe having doctests run (and fail) on an Edition change would be viewed as a bug fix.
If we move forward with this, the decision is likely not ours alone as we'd need to stabilize flags in other tools as well.
The conversation is ongoing on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/doctest-xcompile/).

<!-- source: also https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Setting.20.60strip.3Ddebuginfo.60.20by.20default.20when.20.60debug.3D0.60 -->
**[Have `profile.*.debug=0` imply `profile.*.strip = "debuginfo"`](https://github.com/rust-lang/cargo/issues/4122):**
When a user disables debug info with [`debug=0`](https://doc.rust-lang.org/cargo/reference/profiles.html#debug),
they will still have debug symbols from `std` as it is pre-built.
By implicitly setting [`strip = "debuginfo"`](https://doc.rust-lang.org/cargo/reference/profiles.html#strip) when `debug=0`,
we'd be closer to what the user actually requested.
According to the [zulip thread](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Setting.20.60strip.3Ddebuginfo.60.20by.20default.20when.20.60debug.3D0.60),
this speeds up builds on Linux and shrinks binaries.
The downsides when `debug=0` is set are slightly slower builds on Mac and backtraces will be less informative.
To put the backtraces in perspective, this will make backtraces through `std` consistent with user code and user code is likely to be the majority of an application.
We felt this could move forward as-is without an Edition and asked for a more formal proposal which can be found [on the issue](https://github.com/rust-lang/cargo/issues/4122#issuecomment-1868318860).

<!-- also https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/separating.20debuginfo.20on.20linux.20in.20debug.20mode -->
**[Make `profile.*.split-debuginfo` the default](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/separating.20debuginfo.20on.20linux.20in.20debug.20mode):**
We previously changed the default for Mac
([rust-lang/cargo#9298](https://github.com/rust-lang/cargo/pull/9298))
to improve compile times and reduce target-dir size.
If we make this change, we realized it can't be tied to an Edition because
[`[profile]`](https://doc.rust-lang.org/cargo/reference/profiles.html#profiles)
is a workspace-level field and Cargo only has the concept of an Edition for packages and not workspace.
For further discussion, see the [zulip thread](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/separating.20debuginfo.20on.20linux.20in.20debug.20mode).

##### `[lints]` redux

[1.74](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html#lint-configuration-through-cargo) saw the introduction of [`[lints]` in `Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section).

The primary points of concern as more people tried this out were
- [Confusion over the interaction between groups and lints](https://github.com/rust-lang/cargo/issues/12918)
- [Wanting to both inherit lints from the workspace and add more](https://github.com/rust-lang/cargo/issues/13157)
- [Concern over forgetting to inherit workspace lints](https://github.com/rust-lang/cargo/issues/12208)

<!-- source: 2023-11-21 meeting -->
For the last point, we discussed implicit workspace inheritance for all fields.
The main concern we focused on was about the challenges of this being too magical, making it harder for people to reason about what cargo is doing.
One piece of prior art is [`[profile]`](https://doc.rust-lang.org/cargo/reference/profiles.html).
While Cargo implicitly layers `Cargo.toml` over `.cargo/config.toml`, this is involving configuration which is already a little off the beaten path and might not be the best model to follow.
However, it also supports profiles explicitly saying they inherit from another profile.
We could have a manifest-wide opt-in for inheriting fields that were not explicitly set.
Based on feedback, we could then explore changing the default for this on an edition.
One awkward case is dependencies.
We shouldn't automatically add all dependencies.
We also probably shouldn't allow dependencies without a source (i.e. allow skipping `workspace = true`).
But not having dependencies participate would be inconsistent.

Discussing the prior art of `[profile]` also led to a discussion of having more than one set of values you can inherit from.
We discussed a couple of ideas, including
- Having named sets of fields that you inherit all-or-nothing (`inherits = "public-members"`)
- Having named fields that you can inherit per-field (`rust-version.workspace = "public-members"`)
- Naming other packages you can inherit from, either whole or per field

<!-- source: https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/warnings.20control.20option.20bikeshed -->
<!-- this was actually in October
With `[lints]` replacing the need for project-specific `RUSTFLAGS` to control warnings,
that still leaves runtime setting of warnings.
Most commonly, you'll see this with CI jobs that do `RUSTFLAGS=-Dwarnings cargo clippy` or local development that does `RUSTFLAGS=-Awarnings`.
Both of these flush cargo's caches ([rust-lang/cargo#8716](https://github.com/rust-lang/cargo/issues/8716))
and require remember to switch to `RUSTDOCFLAGS` for `cargo doc`.
Cargo intercepts every warning before showing it to the user.
These then get recorded so they can be replayed for packages that aren't rebuilt.
[We could have flag(s) on Cargo commands to control what should happen to these warnings](https://github.com/rust-lang/cargo/issues/8424).
The simplest form of this would be `--warnings <allow|deny>`.
The advantage of keeping this simple is that it avoids broken expectations from people who are used to `rustc`s flags as these can't behave the same.
They can only affect existing warnings and only turn them into an error or silence them.
We also have no knowledge of warning groups.

There remain open questions for how to treat `cargo`s "warnings" which aren't always proper diagnostics, like `rustc`
and if this should be a flag or a config setting.
For more discussion, see [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/warnings.20control.20option.20bikeshed).
-->

##### cargo script

Progress on the [cargo script RFC](https://github.com/rust-lang/rfcs/pull/3502) and implementation has stalled while the [RFC for embedding manifests](https://github.com/rust-lang/rfcs/pull/3503) is figured out.

The current proposal is:
````rust
#!/usr/bin/env cargo
```cargo
[dependencies]
clap = { version = "4.2", features = ["derive"] }
```

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "Path to config")]
    config: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
````

<!-- source: 2023-11-21 meeting -->
The manifest is embedded in what we are calling a code-fence frontmatter, modeled off of markdown.
The `cargo` identifier is referred to as an infostring.

There are two directions we can take the infostring in the long run:
- Does the parent tool (in this case, cargo) own the definition of the infostring and is allowed whatever identifiers it wants
- Does `rustc` own the meaning of the infostring, allowing the Rust Project to add additional types of metadata without concern for breaking tools that rely on custom identifiers

The embedded manifest syntax RFC was updated with a [new section](https://github.com/epage/rfcs/blob/frontmatter/text/3503-frontmatter.md#optional-or-additional-infostrings),
side-stepping this discussion by suggesting we hard code support for `cargo` right now and leave the decision to the future when we have more context for how this might be used.

We also recognize that using three backticks would likely trip users up when they try to put these into markdown code fences as users might not be aware of how to escape it or forget to escape it, causing frustration.

There is on-going discussion on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/213817-t-lang/topic/Syntax.20for.20embedded.20tooling.20metadata).

##### SBOM

Supply-chain security is getting a lot of attention lately.
One element of this is being able to trace what all was pulled into a binary and how it was built.
This is referred to as a Software Bill of Materials, or SBOM.

Previously, [arlosi](https://github.com/arlosi) created a
[Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-cargo-sbom/19842) on this topic.
The Pre-RFC has continued to garner discussion, including
- Enumerating the limitations of third-party solutions ([link](https://internals.rust-lang.org/t/pre-rfc-cargo-sbom/19842/31?u=epage))
- On the role of Cargo's SBOM format, whether it should be an intermediate format that gives integrators the information they need to create their final SBOM or whether it should be self-contained enough to just do a format conversion to the format of choice.  This affects fields like
  - Author (can be pulled from manifests)
  - Hashes (which comes with questions of which artifacts using which algorithms)
  - Timestamps (which is not reproducible which is a requirement for some SBOM use cases)

[arlosi](https://github.com/arlosi) is planning to incorporate the feedback into the Pre-RFC, do a last call for feedback, and move onto a full RFC.

##### RFC #3537: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies

One frustration point with Cargo and the crates.io ecosystem is when a dependency is added and the build fails because it uses newer features than your version of Rust supports.
We've been tracking this in [rust-lang/cargo#9930](https://github.com/rust-lang/cargo/issues/9930).
Historically, we've deferred work on this as we expect the errors when no compatible package is present to cause more confusion and user frustration than the problem being solved.
We were hopeful that a [PubGrub](https://github.com/pubgrub-rs/pubgrub) dependency resolver would fix this but there is a lot of work remaining before we can switch dependency resolvers.

During the 1.74 development-cycle, we merged
[rust-lang/cargo#12560](https://github.com/rust-lang/cargo/pull/12560)
which added a perma-unstable implementation so people could at least use nightly for one-off dependency resolution.
Just before the 1.76 development-cycle,
we came up with a way to side-step the resolver error messages by only preferring stable versions which we merged in
[rust-lang/cargo#12950](https://github.com/rust-lang/cargo/pull/12950).

This side-step was written up as
[Pre-RFC: MSRV Aware Resolver](https://internals.rust-lang.org/t/pre-rfc-msrv-aware-resolver/19871/58)
which led into
[RFC #3537: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537).

<!-- source: 2023-11-28 meeting -->
<!-- source: 2023-12-12 meeting -->
One of the main conversations is about whether we should respect MSRV by default or require an opt-in.
After some discussion during cargo team meetings and office hours,
the plan moving forward is to re-focus the document on workflows,
what behavior we want to drive (e.g. avoiding stagnation),
and how different possible solutions affect the workflows and user behavior.

We are waiting on the RFC author for integrating this new approach into the RFC.

##### RFC #3371: CARGO_TARGET_BASE_DIR support

<!-- source: 2023-12-05 meeting -->
[RFC #3371](https://github.com/rust-lang/rfcs/pull/3371)
allows users to move all of their target-dirs to be under a common root directory without a lot of bookkeeping on the users end.
It was proposed for merge back in June but it fell off our radar and we were finally able to talk over it.
We clarified that this proposal is independent of
[per-user caching](https://github.com/rust-lang/cargo/issues/5931)
and both efforts are mostly independent and worthwhile.
Per-user caching would reduce how much we put in target-dir but workspaces would still need a target-dir for uncacheable builds and final artifacts.

While there are other solutions that cover the motivations for `CARGO_TARGET_BASE_DIR`,
we felt `CARGO_TARGET_BASE_DIR` is a principled shortcut that can get us most of those benefits sooner.

One concern raised in the RFC is how can people find their target-dir (e.g. packaging their built `[[bin]]`s).
This becomes more relevant if we were to consider switching the workspace's target-dir to a central location.
In the RFC, the idea of symlinking `target/` to target-dir was brought up.
It is unclear whether the benefits to those that need it outweigh the annoyance for those that don't.
Users can get the location of target-dir via `cargo metadata`.
Stabilizing [`--out-dir`](https://github.com/rust-lang/cargo/issues/6790) would bypass most uses for accessing target-dir.
Those might be sufficient.
If not, maybe we could explore having a config field to control the creation of a symlink.

We then explored the design space a little bit,
taking inspiration from
[the index's `dl` field](https://doc.rust-lang.org/cargo/reference/registry-index.html#index-configuration).
For example, having placeholders for `{home}` or `{cargo_home}` would make it easier to copy configs from account to account.
What if we extend `CARGO_TARGET_DIR` with placeholder support instead of adding `CARGO_TARGET_BASE_DIR`, allowing `CARGO_TARGET_DIR={cargo_home}/target/{manifest_path_hash}`?
This seems like this would simplify the design quite a bit.

This is now back on the RFC author's plate to process this feedback and update the proposal as they see fit.

##### RFC #3493: `cargo update --precise <prerelease>`

To use a [pre-release](https://semver.org/#spec-item-9) today,
users have to opt-in with their version requirements.
[RFC #3493](https://github.com/rust-lang/rfcs/pull/3493) changes cargo's dependency resolver so that users can opt-in to a pre-release in their `Cargo.lock`.
This works well if users want to test for regressions in a pre-release dependency or want access to functionality early but don't require it (e.g. performance improvements).
If a package requires something from a pre-release, like a new API, that should instead be specified in the version requirement which
is more of the focus of [RFC #3263](https://github.com/rust-lang/rfcs/pull/3263).

<!-- source: 2023-12-12 meeting -->
Since we were already discussing postponing #3263,
we discussed whether we should also postpone #3493.
While we want to improve pre-releases,
no one on the team is available to help shepherd this and
the proposal would involve an invasive change to cargo that would likely be brittle.
For how much time we do take to address pre-release,
it was unclear if this was the most important.

As we discussed it,
we realized there was a solid precedence to base the design off of,
[yanked packages](https://doc.rust-lang.org/cargo/commands/cargo-yank.html).
We could also minimize risk by suggesting that the
[`semver` package](https://crates.io/crates/semver)
keep the existing version matching logic and expose the new behavior under a different function name.
We also realized that this RFC is a prerequisite for RFC #3263 so that cargo could correctly unify pre-release and regular release version requirements.

For bookkeeping purposes,
there was a concern this would be require `Cargo.lock` changes.
If so, then it would likely be complex enough that we'd need an experimental implementation first.
However, we found `Cargo.lock` changes are unlikely to be needed after some further discussion.

We updated the RFC and this is now waiting on author to wrap up the discussion.

##### `cargo update --precise <yanked>`

<!-- source: 2023-12-19 meeting -->
Between [RFC #3493](https://github.com/rust-lang/rfcs/pull/3493) letting users force pre-releases through `--precise` and
[rust-lang/cargo#12425](https://github.com/rust-lang/cargo/issues/12425) doing the same for breaking changes,
it seemed fitting to extend this concept to yanked packages,
resolving [rust-lang/cargo#4225](https://github.com/rust-lang/cargo/issues/4225).
We felt we need to trust users in these scenarios.
Users might have valid reasons to access yanked packages,
whether its short-term to test something out
or long term and they accept the risks.
We considered an additional flag for this but pre-release and breaking changes don't need a flag.
For breaking changes, the flag would be for use outside `--precise` for opting in for all packages.
There is the possibility we'd want to extend that concept to pre-release and yanked packages.
For a preponderance of caution, we reached out to a prior cargo team member in case there was context we were missing and they gave the greenlight.

[rust-lang/cargo#4225](https://github.com/rust-lang/cargo/issues/4225) is marked as [accepted](https://doc.crates.io/contrib/issues.html#issue-status-labels) and we welcome people to contribute a PR for this.

## Misc

Other relevant topics of interest:
- [Rust Blog: Cargo Cache Cleaning](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html)
- [Github: Fix color handling in cargo for legacy Windows consoles](https://github.com/rust-lang/cargo/issues/13104)
- [Github: Transition `build.rs` directives from `cargo:` to `cargo::` to allow evolution without breaking compatibility](https://github.com/rust-lang/cargo/pull/12201)
- [Internals: verify docs build as part of `cargo publish`?](https://internals.rust-lang.org/t/how-about-cargo-publish-expect-doc/20011)
- [Internals: default `--features` via `.cargo/config.toml`](https://internals.rust-lang.org/t/pre-rfc-configure-default-used-features-in-config-toml-make-rust-analyzer-and-cargo-use-it-as-default/20007)
- [Zulip: Feedback on Earthly's design for Rust CI](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Incremental.20Rust.20builds.20in.20CI)
- [Zulip: Status update on using gitoxide in cargo](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.60gitoxide.60.20integration.20updates)
- [Zulip: Allow `[profile]`s to enable `--features` ](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Profile.20specific.20features)
- [PackagingCon videos are up](https://www.youtube.com/@packagingcon_org/videos) with a [presentation on Namespaces in Rust](https://www.youtube.com/watch?v=1IINSW7IG-k)

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - Blocked on [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Inline snapshotting in snapbox](https://github.com/assert-rs/trycmd/issues/221)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)

Needs design and/or experimentation:
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)

Planning:
- [RFC #3243: Package as (optional) namespaces](https://github.com/rust-lang/rfcs/pull/3243)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3485: description field](https://github.com/rust-lang/rfcs/pull/3485)
  - [RFC #3487: visibility control](https://github.com/rust-lang/rfcs/pull/3487)
  - [RFC #3486: deprecation field](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [RFC #3452: Nested packages](https://github.com/rust-lang/rfcs/pull/3452)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)

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
