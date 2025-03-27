+++
path = "inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80"
title = "This Development-cycle in Cargo: 1.80"
authors = ["Ed Page"]
aliases = ["inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.80

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.80.

<!-- time period: 2024-05-03 through 2024-06-13 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [`-Zcheck-cfg`](#zcheck-cfg)
  - [User-controlled cargo diagnostics](#user-controlled-cargo-diagnostics)
  - [`-Ztrim-paths`](#ztrim-paths)
  - [MSRV-aware Cargo](#msrv-aware-cargo)
  - [Removing implicit features](#removing-implicit-features)
  - [Normalizing published manifest files](#normalizing-published-manifest-files)
  - [Merging `cargo upgrade` into `cargo update`](#merging-cargo-upgrade-into-cargo-update)
  - [`.crate` provenance](#crate-provenance)
  - [`cargo publish --workspace`](#cargo-publish-workspace)
  - [Snapshot testing](#snapshot-testing)
- [Design discussions](#design-discussions)
  - [RFC triage](#rfc-triage)
  - [Custom test harnesses and `panic = "abort"`](#custom-test-harnesses-and-panic-abort)
  - [Short-hand manifest syntaxes](#short-hand-manifest-syntaxes)
  - [Leaky abstractions of rustc](#leaky-abstractions-of-rustc)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-expand](https://crates.io/crates/cargo-expand) a more convenient and easier to remember wrapper around `cargo rustc` for seeing all macros expanded within a crate.
For macro authors, this is a big debugging help.
For macro users, this can help in breaking the opacity of what a macro invocation does.

Thanks to [LukeMathWalker](https://github.com/LukeMathWalker) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

##### `-Zcheck-cfg`

*Update from [1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#-zcheck-cfg)*

As a refresher, this is a rustc feature that checks `#[cfg]`s against a list of known names and values.
When used with Cargo, the names and values come from:
- rustc's list of "well known" cfgs (generally first party compilation toolchains)
- cargo's list of "well known" cfgs
- [`[features]`](https://doc.rust-lang.org/cargo/reference/features.html)
- [`cargo::rustc-check-cfg` build.rs directive](https://doc.rust-lang.org/nightly/cargo/reference/build-scripts.html#rustc-check-cfg)
- Passing `--check-cfg` through `RUSTFLAGS`

At the beginning of May, Cargo's support for `--check-cfg` was stabilized ([#13571](https://github.com/rust-lang/cargo/pull/13571)).
When stabilizing, the team weighed
the [crater results](https://github.com/rust-lang/rust/issues/120701#issuecomment-1937010106)
and feedback from the multiple rounds of [Call for Testing](https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479)
(which included [tweaking the layout](https://github.com/rust-lang/this-week-in-rust/pull/5195) for This Week in Rust to improve visibility).
Hand-in-hand with stabilization,
[urgau](https://github.com/urgau) published a
[blog post](https://blog.rust-lang.org/2024/05/06/check-cfg.html)
to help people first exposed to this by upgrading nightly.

Soon after this hit nightly, [rust-lang/rust#124800](https://github.com/rust-lang/rust/issues/124800) (and related issues) were open.
People's concerns included:
- The false positive rate
- Having to use a `build.rs` for custom `--cfg`s where one wasn't used before
- `#[cfg]`s in `build.rs` itself which has no way of specifying `--check-cfg` for itself
- Some confusion over the lint message and documentation

A lot of discussion was spent on trying to get everyone involved in the conversation (including us) onto the same page in terms of what this feature was, what the options were for working with it, and the impact of each of those.
This is understandably a frustrating process because people who are negatively impacted are feeling the pain now.
However, we need to make sure we find the right solutions rather than the first.

<!-- 2024-05-07 -->
<!-- https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Adding.20kani.20to.20list.20of.20known.20configurations -->
<!-- https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/rustc-check-cfg.20build.2Ers -->
A positive of this was that nightly was doing its job in helping to collect critical feedback well before we hit stable!
To give us plenty of time for feedback, we intentionally held back the stabilization PR until after the previous nightly was branched to beta, so we'd have a full 12 weeks to collect feedback and improve this.
As this was a lint (non-blocking) and we felt confident in doing any needed polish in the 12 weeks before release,
we decided to keep this in nightly, rolling back only if we were running up to the deadline for release.

We recognize that the documentation was an issue.
We worked together to find ways to improve it (e.g.
[#13869](https://github.com/rust-lang/cargo/pull/13869)
[#13937](https://github.com/rust-lang/cargo/pull/13937)
[rust-lang/rust#124209](https://github.com/rust-lang/rust/pull/124209)
).
One challenge that limited this work was finding a place for Cargo's documentation to live as there isn't a user-focused Cargo feature for this to center the documentation around.

As for improving how people interact with this feature,
longer term we feel
[private features](https://github.com/rust-lang/rfcs/pull/3487) and 
[mutually-exclusive, global features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
would help replace a lot of cases for custom `cfg`s.
That still leaves the short term.

An straightforward answer that came up multiple times was to add to `Cargo.toml` a `[cfg]` table ([#11631](https://github.com/rust-lang/cargo/pull/11631)).
This was proposed during the development of the feature but was
[rejected by the Cargo team](https://github.com/rust-lang/cargo/pull/11631#issuecomment-1487424886)
as designing a whole new system that didn't align with the rest of cargo (see also [Leaky abstractions of rustc](#leaky-abstractions-of-rustc)).

<!-- 2024-05-14 -->
Through a mixture of Github, Cargo Office Hours, and in-person conversations at a conference,
we settled on the solution of using lint configuration from the "Future Possibilities" of
[RFC 3389: `[lints]`](https://github.com/rust-lang/rfcs/pull/3389).
Shout out to [wesleywiser](https://github.com/wesleywiser)
for the [idea](https://github.com/rust-lang/rust/issues/124800#issuecomment-2099448673)
and [urgau](https://github.com/urgau)
for the implementation in [#13913](https://github.com/rust-lang/cargo/pull/13913).

e.g.
```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(has_foo)'] }
```
*(see [Cargo Specifics - Checking Conditional Configurations](https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html#check-cfg-in-lintsrust-table) for more details)*

<!-- 2024-05-21 -->
We weren't necessarily done and had to address questions like:
- [Should we prevent people from extending Cargo check-cfg values?](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.60check-cfg.60.20and.20custom.20.60feature.60)
- Should we reduce warning noise in 1.79 as part of the transition to this feature?  Yes, in [#13925](https://github.com/rust-lang/cargo/pull/13925).
- Are we okay with the inconsistent dashes in `lints.rust.unexpected_cfgs.check-cfg`?  Yes, its not too different from dependency names and fields.
- Should we allow the `level` to be optional?  As its an MSRV bump either way, we decided to defer making a decision on this.  Some lean towards preferring being explicit.

One lesson learned from this is that the Cargo and Compiler teams should better coordinate on changes like this.
In particular,
[public-private dependencies](https://github.com/rust-lang/rust/issues/44663)
seems like it will have a similar effect on the ecosystem.

##### User-controlled cargo diagnostics

*Update from [1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html#user-controlled-cargo-diagnostics)*

This development-cycle, diagnostics mostly saw polish.

<!-- 2024-05-14 -->
We did have a forward-looking conversation among the Cargo team: when should lints be evaluated?

Most manifest errors and warnings today are processed when the document is parsed.
Warnings get captured and reported later only in some commands and only if  we don't otherwise cap the lints for that package.
Or put another way, we analyze warnings for every package in your dependency tree and then throw away almost all of that work.

If we were to move some of these errors and warnings to when we run our new diagnostics,
we could stop doing throw-away work and maybe improve some aspects of code organization.
As a side effect,
some errors that get reported today might not get reported at all.
Could we change this later without breaking our expectation for compatibility or, if we can't, are we ok with being stuck with this new behavior?
After some discussion, we felt that we could move in this direction but we'll need to look at this on a case by case basis.

##### `-Ztrim-paths`

*Update from [1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#-ztrim-paths)*

<!-- https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/trim-paths.20testing -->
<!-- https://github.com/rust-lang/cargo/issues/12137 -->

[rust-lang/rust#107099](https://github.com/rust-lang/rust/pull/107099) was merged, allowing `rustdoc` to participate in trim-paths.
Cargo still needs to be updated.

Another challenge Cargo has had is access to a hash algorithm that is stable across platforms and not just run-to-run.
[Urgau](https://github.com/Urgau) is working to pull out rustc's stable hasher for Cargo to reuse ([rust-lang/rustc-stable-hash#1](https://github.com/rust-lang/rustc-stable-hash/pull/1)).

##### MSRV-aware Cargo

*Update from [1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html#msrv-aware-cargo)*

The main focus at this point is the MSRV-aware resolver.

We ran a [Call for Testing](https://github.com/rust-lang/cargo/issues/13873) and so far we haven't gotten any actionable feedback.
Separate from that Call for Testing, we did receive feedback about the
`latest` annotations in `cargo update` ([#13908](https://github.com/rust-lang/cargo/issues/13908)).
Discussion is still on-going.

<!-- 2024-06-04 -->
As far as we are aware, the last remaining step before stabilization is
renaming the configuration ([#13540](https://github.com/rust-lang/cargo/issues/13540)):
```toml=
[resolver]
something-like-precedence = "something-like-rust-version"
```

We want to keep in mind how this might align with future possibilities like:
- [#5657 Minimal version resolution](https://github.com/rust-lang/cargo/issues/5657)
- [#4225 Control of yanked](https://github.com/rust-lang/cargo/issues/4225)
- [#13290 Control of prerelease](https://github.com/rust-lang/cargo/issues/13290)

For minimal vs maximal version resolution, we see that is likely an enumerate value under an unnamed key, so we can set that aside.

For the rest, we are talking about the following modes:
| mode                                 | MSRV     | yanked   | prerelease |
|--------------------------------------|----------|----------|----------|
| this is yet another candidate        | Required | Never?   | Never?   |
| de-priorize this over other versions | Required | Likely   | Likely   |
| don't resolve to if already in use   | Likely   | Required | Required |

This helped to show that we probably want to name the field `incompatible-rust-version` to clarify that we are talking about how we are handling those packages and to leave room for `resolver.rust-version` to override what version is used when resolving dependencies.

The challenge is providing a clear way to communicate the "mode".
For de-prioritizing, we considered:
- discourage
- eschew
- avoid
- fallback

Of those, `fallback` most captures the nuance but that is a noun while we had been considering `allow` and `deny` for the other modes which are verbs.

We had considered a short-term solution of a "precedence policy" field that would take a policy name, much like we have profile names that encompass many compiler settings.
This would be short term and could run into its own issues with being confusing, so we decided to focus on the "right" solution.

##### Removing implicit features

*Update from [1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#weak-feature-syntax) and [1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html#user-controlled-cargo-diagnostics)*

A crater run exposed a couple of bugs with `cargo fix` when migrating implicit features to explicit features ([#14010](https://github.com/rust-lang/cargo/issues/14010)).

<!-- 2024-06-11 -->
This exposed a misunderstanding that `["dep_name/feature_name"]` is not always the same as `["dep:dep_name", "dep_name?/feature_name"]` because the former won't suppress the creation of an implicit feature.
As a side effect of how we prevent implicit features from being created in Edition 2024,
there are corner cases where the `["dep_name/feature_name"]` syntax will error
([#14016](https://github.com/rust-lang/cargo/issues/14016))
.
We considered whether we should deprecate the syntax (which we had previously deferred) or whether to implicitly inject `"dep:dep_name"`.
After some discussion, we settled on the latter.

##### Normalizing published manifest files

*Update from [1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html#normalizing-published-package-files)*

Now that published `Cargo.toml` files list all targets,
we can avoid reading the filesystem with [#13849](https://github.com/rust-lang/cargo/pull/13849).

Thanks for [stormshield-guillaumed](https://github.com/stormshield-guillaumed) testing nightlies,
we found out that these changes made the vendored `Cargo.toml` non-deterministic and fixed that in [#14004](https://github.com/rust-lang/cargo/pull/14004).

##### Merging `cargo upgrade` into `cargo update`

*Update from [1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#merging-cargo-upgrade-into-cargo-update)*

In [#13979](https://github.com/rust-lang/cargo/pull/13979), [torhovland](https://github.com/torhovland) added support for `cargo update --breaking` with the following policy:
1. In-memory, upgrades certain workspace member dependencies to the latest breaking version
  - Must use `^` version requirement operator (the default)
  - Must not be renamed as those tend to be used to allow multiple versions of the same package and the user likely doesn't want the version changed
  - Can further limit to specific dependencies by naming them
2. Reconcile the lockfile with these new version requirements
3. Write out the new version requirements, preserving the precision used in the original requirement (e.g. `1.0` would be upgraded to `2.0`, not `2.0.5`)

This is available to use as of nightly 2024-06-09.

##### `.crate` provenance

With the xz backdoor, there has been an increased interest in verifying that a published `.crate` matches the repo.
Cargo already includes a `cargo_vcs_info.json` file in the `.crate` to identify what `cargo publish` was run against.
One problem is that this file wasn't being generated if `--allow-dirty` was used
([#13695](https://github.com/rust-lang/cargo/issues/13695)).

After some discussion, [torhovland](https://github.com/torhovland) created [#13960](https://github.com/rust-lang/cargo/pull/13960) for us to always generate `cargo_vcs_info.json` but to include a `dirty: "true"` field when `--allow-dirty` was used.

For more investigation on using this file, see
[999 crates of Rust on the wall](https://lawngno.me/blog/2024/06/10/divine-provenance.html).

##### `cargo publish --workspace`

[torhovland](https://github.com/torhovland) and
[jneem](https://github.com/jneem) stepped up to break down what it would take
to add `--workspace`  in
[#1169](https://github.com/rust-lang/cargo/issues/1169) and
[#10948](https://github.com/rust-lang/cargo/issues/10948).
They started on the milestone of support for `cargo package --workspace`.

The first problem to address is how will we generate a valid lockfile for packages that haven't been uploaded.
We discussed this in Office Hours and decided to implement an internal-only package-source overlay system in
[#13926](https://github.com/rust-lang/cargo/pull/13926).
This would allow local packages to pretend to be on crates.io.
This might sound like a very useful future *but* we intentionally decided to keep this internal-only and only implemented it with great hesitance because a generalized version of this would be a source of 
[dependency-confusion attacks](https://medium.com/@alex.birsan/dependency-confusion-4a5d60fec610).

Now that the overlay-source is implemented, work continues on [#13947](https://github.com/rust-lang/cargo/pull/13947).

##### Snapshot testing

*Update from [1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#terminal-styling)*

Cargo has had a homegrown CLI assertion framework with support for features like
- Redactions
- Jsonlines
- Unordered text and jsonlines
- Pretty diffs on failure

As there is less of a community around this,
the knowledge is more specialized,
its less likely to be documented,
and we are on our own for feature development.

[epage](https://github.com/epage) and
[Muscraft](https://github.com/Muscraft)
have been generalizing the concepts from `cargo-test-support` into
[snapbox](https://crates.io/crates/snapbox).
This has been used for UI testing in cargo since `cargo add`.

Out of frustration with hand-editing nearly every test multiple times this year, epage dove into closing the functionality gaps with `cargo-test-support` and release this in the
[0.5.11 and 0.6.0 releases](https://github.com/assert-rs/snapbox/blob/main/crates/snapbox/CHANGELOG.md#060---2024-05-23).

We started the porting effort with non-CLI assertions in
[#13980](https://github.com/rust-lang/cargo/pull/13980)
and then CLI assertions in
[#14031](https://github.com/rust-lang/cargo/pull/14031).

Immediate benefits for cargo contributors
- Update test results by adding `env SNAPSHOTS=overwrite`
- Auto-redact non-deterministic snapshotted values (e.g. [`[ELAPSED]`](https://github.com/rust-lang/cargo/pull/13973)) or platform-specific snapshotted values (e.g. [`[BROKEN_PIPE]`](https://github.com/rust-lang/cargo/commit/eee105338420c68105678a6b883514a190ee5070))

We've documented the process and opened the porting work for anyone to contribute in
[#14039](https://github.com/rust-lang/cargo/issues/14039),
like [weihanglo](https://github.com/weihanglo) did in [#14041](https://github.com/rust-lang/cargo/pull/14041).

## Design discussions

##### RFC triage

<!-- 2024-05-21 -->
When discussing the 2024 Edition,
it came up that an RFC was partly delayed because the author and the reviewing team each thought they were waiting on the other.
RFC authors have a real need driving them to write a proposal and put a lot of time into writing the RFC and driving the discussion.
Out of respect for investment that RFC authors have put in,
[epage](https://github.com/epage) wanted to drive another pass in trying to make sure there is a clear owner for the next steps for each RFC.
This might even include closing the RFC which can be a sensitive topic.

For our first, epage reminded us that [templating CARGO_TARGET_DIR](https://github.com/rust-lang/rfcs/pull/3371#issuecomment-1602959262) has been proposed to merge and is waiting on team members to review it.

<!-- 2024-05-28 -->
[RFC #3383: recommended-bins](https://github.com/rust-lang/rfcs/pull/3383):
This was briefly touched on when we last discussed
[when to use a package or a workspace](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#when-to-use-packages-or-workspaces).
Since then, the `#[diagnostic]` attribute was stabilized in [1.78](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html#diagnostic-attributes).
Mirroring the concept in Cargo would be a lot lower of a barrier to entry than a fully designed feature.
The Cargo team got behind this idea and shared this on the RFC.

<!-- 2024-06-11 -->
[RFC 3310: root rustflags](https://github.com/rust-lang/rfcs/pull/3310):
This was originally written for coverage reporting and later the author wrote
[#3287](https://github.com/rust-lang/rfcs/pull/3287).
That said, the feature seems generally useful, even if dangerous (see the Drawbacks).
In the end, we decided to postpone this as the author hasn't been responsive.
We included next steps for someone to pick it up.

[RFC 3416: Features as table, not just array](https://github.com/rust-lang/rfcs/pull/3416): Overall, we were in favor of this moving forward.  The main concerns were naming (which we worked out) and ensuring third-parties have a heads up of the potential change to `Cargo.toml` ([announced on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/334885-t-cargo.2Fbuild-integration/topic/.60Cargo.2Etoml.60.20schema.20changes/near/444030709)).
As a reminder, we provide the definition of [`TomlManifest`](https://docs.rs/cargo-util-schemas/latest/cargo_util_schemas/manifest/struct.TomlManifest.html) for others to pull in and use.

##### Custom test harnesses and `panic = "abort"`

<!-- 2024-05-28 -->
[#11214](https://github.com/rust-lang/cargo/issues/11214) is looking for a way to allow [`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic) in tests and benches.
For example, ideally benches reflect what the production code will do but if the production code is built with `panic = "abort"` but the tests are built with `panic = "unwind"`, then they can have different performance characteristics.

`panic` is a profile setting and gets applied to all build targets.
We don't want someone changing this value for their bins to affect their tests unexpectedly.

We also need to keep in mind backwards compatibility.
Changing this setting would immediately break people.

We didn't come to any immediate conclusion on how to move forward with this.

##### Short-hand manifest syntaxes

[RFC #3502](https://github.com/rust-lang/rfcs/pull/3502) was recently merged.
Throughout its life, we've occasionally brainstormed ways to smooth out the feature more, like [supporting `package.edition` as a number and not just a string](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/edition.20as.20number).

<!-- 2024-05-21 -->
**Removing the need for `[package]` header:**
Instead each `package` field would work in the top-level of the manifest.
For cargo-script, this would mean you could do
```toml
edition = "2024"
```
instead of
```toml
package.edition = "2024"
```
or
```toml
[package]
edition = "2024"
```
The main concerns raised were
- Detecting if the package is present when using `[workspace]`
- Potential implementation complexities, especially around good error reporting
- If we simplified this by limiting it to cargo-script, it would add more steps to by-hand convert a cargo-script to a multi-file package

Overall, this is something that doesn't have to be decided now.
Instead, we can see how people use cargo-script and decide what improvements are needed.

<!-- 2024-05-28 -->
**Embedded `build.rs`:**
The following `Cargo.toml` alternatives to a standalone `build.rs` were proposed:
```toml
build.rs = '''
fn main() {
  ...
}
'''
# or
build.rs.output = '''
cargo::directive=...
'''
```
While `build.rs.output` might have helped with check-cfg before we added lint configuration,
it seems pretty rare that it would be of use.

Embedding rust source in the manifest would allow a cargo-script to have a `build.rs`.
While having a fully separate `-sys` package is reasonable for general cases, there can be one-off "I just want a lib for a little bit" use cases.
We had previously decided that embedding other content in a cargo-script (build script, config, proc-macros, addition source and packages),
was an anti-feature and we'd need to revisit a lot more than just this.
Instead, we would want to encourage multi-file packages in these cases.

As an alternative, we discussed [metabuild](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#metabuild) which has been stuck in unstable limbo due to the lack of real world use and polyfills like [system-deps](https://crates.io/crates/system-deps).

Again, this is likely something we should wait until cargo-script has been stabilized and in use for a while.

##### Leaky abstractions of rustc

<!-- 2024-06-04 -->
Cargo is dependent on the behavior of rustc but frequently users regularly need access to rustc features that haven't been abstracted yet ([#12739](https://github.com/rust-lang/cargo/issues/12739)), leading to `RUSTFLAGS` and `cargo rustc`.

Here be dragons though. A recent example of this causing user confusion is that `RUSTFLAGS=-Copt-level=3 cargo test` will disable debug assertions while `profile.test.opt-level = 3` does not ([#14033](https://github.com/rust-lang/cargo/issues/14033)).

Similarly, `cargo build --message-format=json` does not report back all json messages from the compiler.
This makes life more difficult for [cargo show-asm](https://crates.io/crates/cargo-show-asm) as they want to tell the compiler to emit the assembly and having the compiler tell them where to find the assembly files, rather than trying to guess ([#13672](https://github.com/rust-lang/cargo/issues/13672)).
Cargo filters out messages related to implementation details
(e.g. emitted files and unstable aspects of target-dir)
but doesn't know when a message came back because its tied to an implementation detail or a user requested it via `RUSTFLAGS`.
While people can find ways to rely on those same implementation details,
there is a difference between the user knowingly going off the beaten path and us endorsing it in terms of the burden it places on us for evolving things in the future.
During the team meeting, we couldn't come up with a solution that satisfied us and we reported back our unsatisfactory ideas on the issue.
However, in Office Hour, one idea for a rustc feature for this would be to permit specifying a root directory, and not just a file, to emit to.

## Misc

- zstd `.crate` files were briefly discussed on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/zstd.20crate.20files), see also [#2526](https://github.com/rust-lang/cargo/issues/2526).
- [Daily reports](https://rust-lang.zulipchat.com/#narrow/stream/260232-t-cargo.2FPubGrub/topic/Progress.20report) by [Eh2406](https://github.com/Eh2406) on the progress of the Rust implementation of the PubGrub version solving algorithm
- More progress has been made on [#13709](https://github.com/rust-lang/cargo/pull/13709) for [RFC 3553: SBOM](https://github.com/rust-lang/rfcs/pull/3553)
- We've identified more corner cases for `cargo update --precise <prerelease>` ([#13290](https://github.com/rust-lang/cargo/issues/13290)) <!-- 2024-05-28 -->

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
<!--
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)
-->

Needs design and/or experimentation:
- [GC](https://github.com/rust-lang/cargo/issues/12633)
- [cargo info](https://github.com/rust-lang/cargo/issues/948)
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [RFC #3371: CARGO_TARGET_BASE_DIR](https://github.com/rust-lang/rfcs/pull/3371)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
- Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503))
<!--
- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3485: descriptions](https://github.com/rust-lang/rfcs/pull/3485) (descriptions)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
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
