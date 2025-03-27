+++
path = "inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77"
title = "This Development-cycle in Cargo: 1.77"
authors = ["Ed Page"]
aliases = ["inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.77

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.77.

<!-- time period: 2023-12-28 through 2024-02-08 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Polishing `cargo new`](#polishing-cargo-new)
  - [Merging `cargo upgrade` into `cargo update`](#merging-cargo-upgrade-into-cargo-update)
  - [`cargo update --precise <yanked>`](#cargo-update-precise-yanked)
  - [`-Zcheck-cfg`](#zcheck-cfg)
  - [User-controlled diagnostics](#user-controlled-cargo-diagnostics)
  - [Strip `std`'s debuginfo when debuginfo is not requested](#strip-std-s-debuginfo-when-debuginfo-is-not-requested)
  - [Stabilizing `cargo metadata`'s `id` field](#stabilizing-cargo-metadata-s-id-field)
- [Design discussions](#design-discussions)
  - [Being-less-surprising-when-people-benchmark-debug-builds](#being-less-surprising-when-people-benchmark-debug-builds)
  - [Cargo script](#cargo-script)
  - [When to use packages or workspaces?](#when-to-use-packages-or-workspaces)
  - [RFC #3537: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](#rfc-3537-make-cargo-respect-minimum-supported-rust-version-msrv-when-selecting-dependencies)
  - [RFC #3516 (public/private dependencies)](#rfc-3516-public-private-dependencies)
  - [Fallback dependencies](#fallback-dependencies)
  - [Build script directives](#build-script-directives)
  - [Cargo and rustup](#cargo-and-rustup)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)
  - [How you can help](#how-you-can-help)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our featured plugin for this cycle is [cargo-watch](https://crates.io/crates/cargo-watch), which will re-run cargo commands on source changes.
For a discussion on this being merged into cargo,
see [#9339](https://github.com/rust-lang/cargo/issues/9339).

Thanks to [LukeMathWalker](https://github.com/LukeMathWalker) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

##### Polishing `cargo new`

`cargo new` gained the ability to detect workspaces and automatically inherit their fields in Cargo 1.71 and update `workspace.members` in Cargo 1.75.
These were implemented separately and the field inheritance didn't take into account workspace member excludes which was addressed by
[hi-rustin](https://github.com/hi-rustin)
in [#13261](https://github.com/rust-lang/cargo/pull/13261).
[linyihai](https://github.com/linyihai)
then limited the logic for workspace inclusion to whether the discovered package already had a `[workspace]` table in
[#13391](https://github.com/rust-lang/cargo/pull/13391).
[linyihai](https://github.com/linyihai)
also added a `note:` to users if we edited `workspace.members` in
[#13411](https://github.com/rust-lang/cargo/pull/13411).

Whenever you run `cargo new`, you get a comment giving you next steps for filing out your `Cargo.toml`:
```rust
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```
While this helps new Rust programmers,
this adds boilerplate that existing Rust programmers have to remove on every invocation.
In trying to keep both sets of users in mind,
we are trying this out as a `note:` instead ([#13371](https://github.com/rust-lang/cargo/pull/13371).
For myself, I felt it odd to see context for the note (created a package) after the note,
so in [#13367](https://github.com/rust-lang/cargo/pull/13367)
we switched from printing a `Created` status at the end to a `Creating` status at the beginning.

With the previous `Created`:
```console
$ cargo new foo
      Adding `foo` as member of workspace at `/home/epage/src/personal/cargo`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
     Created binary (application) `foo` package
```
With the new `Creating`:
```console
$ cargo new foo
    Creating binary (application) `foo` package
      Adding `foo` as member of workspace at `/home/epage/src/personal/cargo`
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

##### Merging `cargo upgrade` into `cargo update`

With `cargo add` and `cargo remove` merged into cargo,
the last major tool to merge from `cargo-edit` is `cargo upgrade`.
For now, we are focusing only on incompatible upgrades
([#12425](https://github.com/rust-lang/cargo/issues/12425)),
deferring out consideration of modifying version requirements under other circumstances
([#10498](https://github.com/rust-lang/cargo/issues/10498)).

So far, the focus has been on polishing up `cargo update`, including
- [Replace `--package <spec>` with a positional argument](https://github.com/rust-lang/cargo/pull/12545)
- [Clarify `<spec> --aggressive` as `<spec> --recursive`](https://github.com/rust-lang/cargo/pull/12544)
- [Allowing a version shorthand when `<spec>` is ambiguous](https://github.com/rust-lang/cargo/pull/12614)

In this development-cycle,
we added highlighting of dependencies that are behind in [#13372](https://github.com/rust-lang/cargo/pull/13372),
providing a subset of [cargo-outdated](https://crates.io/crates/cargo-outdated) to all cargo users (see also [#4309](https://github.com/rust-lang/cargo/issues/4309)).

During review,
the PR was called out for not following our
[console output style guide](https://doc.crates.io/contrib/implementation/console.html#style).
This was a case of "copying the style of existing code".
To reduce the chance of this happening in the future,
[#13410](https://github.com/rust-lang/cargo/pull/13410)
aligns more of our console output with our style guide.

The remaining tasks are to add a `--breaking` flag and to extend `--precise <breaking>` so that version requirements get modified.

##### `cargo update --precise <yanked>`

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#cargo-update---precise-yanked)

Previously, the cargo team approved selecting yanked packages.
[weihanglo](https://github.com/weihanglo/) provided an implementation in [#13333](https://github.com/rust-lang/cargo/pull/13333) which was merged.
It is going through [a round of testing](https://github.com/rust-lang/cargo/issues/4225#issuecomment-1930204063)
before being stabilized.

This is of interest for [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks/issues/275).
The current solution doesn't fully solve their [need](https://github.com/rust-lang/cargo/issues/4225#issuecomment-1930353693).
We'd like need to expand this from `--precise` opting in to yanked packages to Cargo consider yanked packages but with the lowest precedence.
This opens up the door quite wide on yanked packages and
we want to further evaluate the remaining use cases after `--precise` support is merged to see if that is worth it.

##### `-Zcheck-cfg`

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#-zcheck-cfg)

[Urgau](https://github.com/Urgau) and I discussed some inconsistent syntax for the `rustc --check-cfg` parameter.
The syntax for defining a set of values for a `--cfg` was overloaded so that the empty set was treated as valueless.
In practice what this meant was that if you had `#[cfg(feature = "foo")]` with an empty `features` table,
you would get a warning about `features` being undefined, rather than about the value `foo` being undefined.
This was fixed in [rust-lang/rust#119473](https://github.com/rust-lang/rust/pull/119473), [rust-lang/rust#119930](https://github.com/rust-lang/rust/pull/119930), and [#13316](https://github.com/rust-lang/cargo/pull/13316).
See [Urgau's comment](https://github.com/rust-lang/rust/issues/82450#issuecomment-1898975197) for more details.

An unfortunate false positive from this lint was with crates using `#[cfg_attr(docsrs, ...)]` to enable nightly features on [docs.rs](https://docs.rs/).
The warning for this could only be resolved by either adding a `build.rs` to define `docsrs` or to disable this feature completely with an `#![allow]`.
`rustc` maintains a hand-written list of "well known" `--cfg`s but this was done by convention, rather than officially supported.
So we decided to see if it could be
[officially supported](https://rust-lang.zulipchat.com/#narrow/stream/356853-t-docs-rs/topic/.E2.9C.94.20.60--cfg.20docsrs.60/near/417280521)
by having docs.rs pass `--cfg docsrs` to rustdoc on behalf of users.
There seemed interest, so I opened
[rust-lang/docs.rs#2389](https://github.com/rust-lang/docs.rs/issues/2389)
and Urgau closed it with [rust-lang/docs.rs#2390](https://github.com/rust-lang/docs.rs/pull/2390).
`--cfg docsrs` was then added to a
[Cargo "well known" list](https://github.com/rust-lang/cargo/pull/13383).
Cargo seemed a more appropriate home as docs.rs is generally tied into [crates.io](https://crates.io/) which is generally tied to Cargo while rustc can be used with other build systems.

<!-- team meeting: 2024-02-06 -->
The cargo team had a preliminary conversation on stabilizing the feature.
A concern was raised about performance, especially when there are a large number of features, like with [windows](https://docs.rs/crate/windows/latest/features).
We've asked for `-Zcheck-cfg` to be benchmarked against `windows` to verify the impact.
We are also leaning towards limiting this feature to "local" packages.
This means only workspace members and path dependencies would be checked,
leaving git and registry dependencies alone.
Already cargo and rustc have the concept of "cap lints" to hide warnings from non-local dependencies.

A [calling for testing](https://github.com/rust-lang/rfcs/pull/3013) is up.

##### User-controlled cargo diagnostics

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#user-controlled-cargo-diagnostics)

As was mentioned in the 1.76 post,
the Cargo team is working on updating
[annotate-snippets](https://github.com/rust-lang/annotate-snippets-rs)
to look like rustc's messages.
The original intention was for all Rust project diagnostic renderers to use this crate for a unified look and feel.
The effort stalled out on rustc's side which came up during a
[cleanup of rustc](https://github.com/rust-lang/rust/issues/59346#issuecomment-1877780379)
where it was suggested to remove the code.
This revived the discussion again on having a unified renderer.
In the end, the decision was to let Cargo be the test bed for this effort as its use cases are simpler
as there aren't existing expectations for richer error messages.
This would help close the gap for rustc's needs.

Speaking of being like rustc,
[Muscraft](https://github.com/Muscraft)'s
[PR was merged](https://github.com/rust-lang/annotate-snippets-rs/pull/73)
for using the same color scheme as rustc.

The first phase of adding rustc-like messages to cargo was merged in
[#13172](https://github.com/rust-lang/cargo/pull/13172).
We got a report of a panic
(fixed in [#13375](https://github.com/rust-lang/cargo/pull/13375))
which highlighted a poor TOML parse message so that was fixed as well 
([#13376](https://github.com/rust-lang/cargo/pull/13376)).

##### Strip `std`'s debuginfo when debuginfo is not requested 

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#meta-2024-edition)

Previously, we discussed implicitly setting `strip = "debuginfo"` when `debug=0`.
A formal [proposal](https://github.com/rust-lang/cargo/issues/4122#issuecomment-1868318860) from [Kobzol](https://github.com/Kobzol) was accepted and implemented in [#13257](https://github.com/rust-lang/cargo/pull/13257).
With this change, debug symbols for `std` would be stripped in the default `release` profile build. This is closer to what users expect for `debug=0` and also upholds our promise from [the Cargo documentation](https://doc.rust-lang.org/cargo/reference/profiles.html#debug): _no debug info at all_.
It was observed the release binaries are
[smaller by ~3-4 MiB](https://perf.rust-lang.org/compare.html?start=e004adb5561b724ac18f5b24584648ca4e42b6ad&end=9d280f70157edca19af117734c1223f5dd0dcd52&stat=size%3Alinked_artifact&tab=compile),
and on Linux the compilations are [slightly faster](https://perf.rust-lang.org/compare.html?start=e004adb5561b724ac18f5b24584648ca4e42b6ad&end=9d280f70157edca19af117734c1223f5dd0dcd52&stat=instructions%3Au&tab=compile).
However, the compilation on macOS might be a bit slower
([~1% for building cargo](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Setting.20.60strip.3Ddebuginfo.60.20by.20default.20when.20.60debug.3D0.60/near/408984829))
as it needs to invoke the external `strip` command.
The other known issue ([#11641](https://github.com/rust-lang/cargo/issues/11641))
is that on macOS it relies on the system's `strip`, which might fail if the `strip` command is shadowed by an incompatible `strip` binary.
We'll continue monitoring if it becomes a burden to either Rust maintainers or users.
See [Kobzol's post](https://kobzol.github.io/rust/cargo/2024/01/23/making-rust-binaries-smaller-by-default.html) for more details.

##### Stabilizing `cargo metadata`'s `id` field

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#stabilizing-cargo-metadatas-id-field)

The FCP completed and the [stabilization PR](https://github.com/rust-lang/cargo/pull/12914) was merged.

Thanks to nightly testing, we found that we had overlooked that people were correlating the output from `cargo metadata` with `cargo build --message-format=json`, so we extended this stabilization to `--message-format=json` as well in [#13311](https://github.com/rust-lang/cargo/pull/13311) and added tests to make sure their output is interoperable in [#13322](https://github.com/rust-lang/cargo/pull/13322).

## Design discussions

##### Being less surprising when people benchmark debug builds

A common pitfall for users new to Rust is that they benchmark their code and find its surprisingly slow when the answer is as simple as passing `--release`.
[jackh726](https://github.com/jackh726) started a 
[discussion](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/cargo.20build.20default.20profile),
exploring ways to help the user avoid this pitfall (see also [#9446](https://github.com/rust-lang/cargo/issues/9446).

The default profile, `dev`, is optimized for fast feedback that makes debugging easier (by including debug-info and activating `debug_assertions`).
The assumption being that debugging will be part of the inner development loop with only occasional releases.
The need for speed is slightly reduced with the introduction of `cargo check`.

Users that aren't expecting this must notice and decipher `dev [unoptimized + debuginfo]` among all of their compiler output.

Brainstorming is on going but ideas include
- Requiring `--profile`
- Tweaking the status line's text
- Adding emoji or styling to the status line
- Support per-command default profiles in config and warn when unset
- Changing the default profile for commands
- Reducing other output (somewhat discussed in [#8889](https://github.com/rust-lang/cargo/issues/8889))

In solving this, we'll need to carefully weigh the needs of all users, including our commitment to backwards compatibility.
Discussion is on-going.

##### Cargo script

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#cargo-script)

As of 1.76, there were two issues on the [syntax side](https://github.com/rust-lang/rfcs/pull/3503):
- Whether the meaning of the infostring was owned by rustc or by the tools using it
- The use of backticks made nesting cargo scripts in markdown, like in Issues, confusing

The discussion on infostrings goes back to the purpose of this.
Rustc already has `#[attributes]` to work and doesn't need this new syntax.
If anything, the focus should be on improving attributes.
This new syntax is designed around the needs for external tools which can't easily work with attributes.
With this context in mind, it was proposed to let external tools define it.

If we agree on that, then our stopgap of requiring an infostring is gone,
reducing the minimum syntax and making it easier to shift away from markdown code fences and avoiding the nesting problem.
In brainstorming with T-lang,
[several syntaxes were considered](https://github.com/rust-lang/cargo/issues/12207#issuecomment-1877652079).
At this time, each of those is supported in cargo for people to give them a try
([#13241](https://github.com/rust-lang/cargo/pull/13241),
[#13247](https://github.com/rust-lang/cargo/pull/13247)).

After discussing them and evaluating user reports, including [timClicks](https://github.com/timClicks)'s [reaction video](https://www.youtube.com/watch?v=S8MLYZv_54w),
the following syntax was proposed:
```rust
#!/usr/bin/env cargo

---
[dependencies]
clap = { version = "4.2", features = ["derive"] }
---

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
```

The syntax RFC has been [proposed for merging](https://github.com/rust-lang/rfcs/pull/3503#issuecomment-1930765966).

On Cargo's side, there is still the question of how to deal with profiles.

##### When to use packages or workspaces?

<!-- team meeting: 2024-01-09 -->
Cargo makes it easy enough to mix binaries and a library together in a package: you just create the files.
The problem is that people quickly hit limitations with the design of `Cargo.toml`.
For example, by doing `cargo add pulldown-cmark`, you pull in a CLI parser that slows down your builds and you should add `--no-default-features`.

Issues people have opened around this include:
- [#1982](https://github.com/rust-lang/cargo/issues/1982)
- [#1430](https://github.com/rust-lang/cargo/issues/1430)
- [#4273](https://github.com/rust-lang/cargo/issues/4273)
- [#5881](https://github.com/rust-lang/cargo/issues/5881)
- [#12848](https://github.com/rust-lang/cargo/issues/12848)
- [#12980](https://github.com/rust-lang/cargo/issues/12980)

When working to improve one of those areas with
[RFC #3374](https://github.com/rust-lang/rfcs/pull/3374),
we found that it would cause
[more confusion on how feature unification works which is already a topic that causes confusion](https://github.com/rust-lang/rfcs/pull/3374#discussion_r1235768792).

Are we pushing a square peg through a round hole?
As a team member put it: "There is a deadzone between 'just add a bin' and 'add a new package'".
Maybe we can look at improving the workspace side of this as well.
To that end, a thought experiment was proposed: what if we only supported one built output per package?  Where would be the pain points?

One gap is with newer users understanding how to adopt workspaces (see also [#5656](https://github.com/rust-lang/cargo/issues/5656)).
One idea proposed was a tool to convert a package to a workspace+package.
This is similar to an idea proposed to convert a cargo script to a multi-file package.
Maybe that similarity can help guide us in what this tool should look like.
This would likely best be experimented with as a third-party plugin.

There is overhead in managing metadata in all of the package but workspace inheritance with the recent `cargo new` work has helped reduce that.

There is still overhead in each package using multiple files and directories by default.
Supporting cargo scripts as workspace members could help with this.

A big gap in all of this is that you can only publish a package at a time
([#1169](https://github.com/rust-lang/cargo/issues/1169)).
We call this out below as one of our "Focus areas" and have proposed it for GSoC.
Releases are more than just publishing and people likely will need to adopt tools like [cargo release](https://github.com/crate-ci/cargo-release).
We have tried to raise awareness of these tools by calling the, out in our 
[publish documentation](https://doc.rust-lang.org/nightly/cargo/reference/publishing.html?highlight=smart#publishing-a-new-version-of-an-existing-crate).
[Nested packages](https://github.com/rust-lang/rfcs/pull/3452) would also reduce some of the release overhead.

There is also the issue that sharing a package name between a binary and a library is more convenient.
For example, compare
```console
$ cargo add pulldown-cmark
cargo add typos
```
with
```console
$ cargo install pulldown-cmark
cargo install typos-cli
```
[RFC #3383](https://github.com/rust-lang/rfcs/pull/3383) is an attempt at improving this.

While we didn't come to any particular conclusions,
we at least have a better understanding of the different challenges involved.

##### RFC #3537: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#rfc-3537-make-cargo-respect-minimum-supported-rust-version-msrv-when-selecting-dependencies)

In processing the feedback on this RFC,
the author came back with a [major update](https://github.com/rust-lang/rfcs/pull/3537).
Part of the goal is to reframe the conversations around different use cases,
and working out how we prioritize these different use cases.
While doing this re-framing,
more rough edges in the workflow were observed and addressed.

<!-- team meeting: 2024-01-02 -->

This RFC calls for a change in behavior to the resolver.
We had considered a new field to control this but that makes behavior more static than is intended.
For example, we'd likely want different behavior between a local `cargo check`, certain CI jobs, and `cargo install`.
If we had this, we could tie this to the Edition.
Because we had started down this route,
`package.resolver` was overlooked.
The RFC has been updated to allow controlling the default with `package.resolver` with the default for that field changing with the next Edition.

<!-- team meeting: 2024-01-23 -->
In stabilizing [`Cargo.lock` v4](https://github.com/rust-lang/cargo/pull/12852),
the question came up about respecting MSRV when generating lockfiles.
When reviewing that in [#12861](https://github.com/rust-lang/cargo/pull/12861),
the question came up of whether we should *not* do this if `--ignore-rust-version` is passed in.
Today it means "ignore the MSRV incompatible error".
With the RFC, it also means "don't resolve based on MSRV".
Lockfiles would add a third meaning.
Is this too much?
When evaluating it, most people likely won't be passing `--ignore-rust-version` to build commands because they predict a dependency tree change and would instead use that more with lockfile commands like `cargo update`.
Similarly, we expect the need for `cargo build --ignore-rust-version`  to diminish because the RFC calls for the error to be turned into a deny-by-default lint.
We likely could deprecate the flag on build commands, reducing this overloading.
We decided there wasn't a reason to hold up the RFC for this and that we can address this for lockfiles when the RFC is merged.

<!-- team meeting: 2024-01-16 -->
On the Pre-RFC, A [user pointed out](https://internals.rust-lang.org/t/pre-rfc-msrv-aware-resolver/19871/65?u=epage)
that their `cargo publish` fails when run from their MSRV toolchain.  This is because Cargo only reuses your lockfile if you have a bin, causing the latest dependencies to be selected. We created [#13306](https://github.com/rust-lang/cargo/issues/13306) from this, deferring any decisions.

##### RFC #3516 (public/private dependencies)

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#rfc-3516-publicprivate-dependencies)

<!-- team meeting: 2024-01-16 -->
A concern was raised on the tracking issue about
[public dependencies requiring an MSRV bump when stabilized](https://github.com/rust-lang/rust/issues/44663#issuecomment-1878029660) 
which would slow down the adoption of the feature.
So far our process has been focused on requiring MSRV bumps to adopt new features as this the a safe default to ensure the users intentions are preserved.
For example, with [`different-binary-name`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#different-binary-name),
ignoring the `filename` field, rather than erroring, would product unexpected results.
The first time I'm aware of Cargo treating an unstable `Cargo.toml` field as an unused key on stable was `package.rust-version` as it was only used for diagnostic purposes.
This was then repeated for the [`[lints]` table](https://github.com/rust-lang/rfcs/pull/3389).
We've [clarified our unstable feature docs](https://github.com/rust-lang/cargo/pull/13307) to make it easier to evaluate alternatives to requiring an MSRV bump.
For public dependencies,
we decided to go ahead and warn on stable rather than error
([#13340](https://github.com/rust-lang/cargo/pull/13340)).  
While we can't change the past, some compiler issues
([rust-lang/rust#71043](https://github.com/rust-lang/rust/issues/71043), 
[rust-lang/rust#119428](https://github.com/rust-lang/rust/issues/119428)) 
make it unclear when this feature will be stabilized and so we might have a sufficient gap to justify this work.
We decided to support enabling the feature through both `Cargo.toml`'s `cargo-features` for those who always need it and `-Z` for those that want to build on stable.

In reviewing [RFC #3560](https://github.com/rust-lang/rfcs/pull/3560),
there was a note about
[preferring warnings to be the same across all Editions](https://github.com/rust-lang/rfcs/pull/3560#issuecomment-1919437187).
In [RFC #3516](https://rust-lang.github.io/rfcs/3516-public-private-dependencies.html#rustc),
we erred on side of changing the level with the Edition to keep noise down.
In [discussing this on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/RFC.20.233516.20-.20RFC.3A.20Superseding.20public.2Fprivate.20dependencies/near/419166799),
we'll need to re-evaluate this decision before stabilization.

##### Fallback dependencies

<!-- team meeting: 2024-01-02 -->
Optional dependencies allow a caller to opt-in to more specialized implementations,
like [winnow](https://crates.io/crates/winnow) having a feature for replacing hand implemented string searches with [memchr](https://crates.io/crates/memchr).
Sometimes you want to reuse an existing fallback implementation from a crate (see also [#1839](https://github.com/rust-lang/cargo/issues/1839)).
The example used in our discussion was `flate2` and the compression library it uses under the hood.
If two backends are enabled, `flate2` prioritizes one and the other is ignored but slowing down user builds.

This would be solved by [mutually-exclusive global features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618) but is there a smaller solution we can work with until then?

For example, could we support `target."cfg(not(feature = "miniz_oxide"))".dependencies` (see also [#8170](https://github.com/rust-lang/cargo/issues/8170))?
We can't handle these as we are resolving features because we are building up the set of features incrementally without a place to say "this is complete, let's evaluate `not(features)`".
We could resolve features normally and then check for `not(features)` and add those in.
This falls apart because these new dependencies would not have feature resolution performed.
We would instead need to loop over running the feature resolving,
checking `not(features)`, and adding them to the set we evaluate next time.
This is complex to implement, algorithmically complex, and may run into cycles with dev-dependencies.

Could we have a `build.rs` ask for features to be enabled?
Like above, this runs into problems with implementation and algorithmic complexity.
This also runs into issues with divergent resolutions where a later package enables a feature that changes the resolution of an earlier package that was already built.

For when the fallback is for compatibility with old versions of Rust, what might work is to instead allow dependencies like
`target."cfg(accessible(std::io::IsTerminal))".dependencies`
([rust-lang/rust#64797](https://github.com/rust-lang/rust/issues/64797))
or
`target."cfg(version(1.70.0))".dependencies` ([rust-lang/rust#64796](https://github.com/rust-lang/rust/issues/64796)).

##### Build Script directives

[*Update from 1.76*](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#misc)

Build scripts communicate to cargo via [special commands that get printed](https://doc.rust-lang.org/nightly/cargo/reference/build-scripts.html#outputs-of-the-build-script).
We found that it was difficult to add new directives because we shared a namespace with users in defining their link metadata.
We resolved this by migrating the directive prefix from `cargo:` to `cargo::` which separates our namespace from the users namespace (`cargo::metadata`)

<!-- team meeting: 2024-01-02 -->

In doing this, we overlooked that [`target.<triple>.<links>`](https://doc.rust-lang.org/nightly/cargo/reference/config.html?highlight=rustc-env#targettriplelinks)
had a similar problem (see also [#12201](https://github.com/rust-lang/cargo/pull/12201#issuecomment-1868539358)).
As the new syntax was stabilized for 1.76 which was in beta, the pressing question is if we needed to revert that and do these together.
After discussion, we did not see a hard requirement for them to be in lock step though consistency is nice.
We are now tracking the config side of this in [#13211](https://github.com/rust-lang/cargo/issues/13211).

##### Cargo and rustup

<!-- team meeting: 2024-01-16 -->
When [GuillaumeGomez](https://github.com/GuillaumeGomez)
was preparing their [blog post on custom linters](https://blog.guillaume-gomez.fr/articles/2024-01-18+Writing+your+own+Rust+linter),
they ran into a problem because they expected `cargo install --path <foo>` to use the `rust-toolchain.toml` file discovered at `<foo>`,
rather than from their current directory ([#11036](https://github.com/rust-lang/cargo/issues/11036)).
Like `.cargo/config.toml`, `rust-toolchain.toml` is an "environment configuration" and doesn't respect flags like `--manifest-path`.
However, cargo makes an exception for `.cargo/config.toml` for `cargo install` (and soon cargo script).
Could we do similar for `rust-toolchain.toml`?

Rustup is an optional toolchain manager that by its nature is versioned and distributed independently of Cargo.
We do have some special casing in Cargo for it but its more focused on error messages and performance.
We'd be breaking an abstraction if we had Cargo take on some of Rustup's role in identifying toolchain versions to use.
We'd also have to tread carefully because of there are needs for isolated toolchains,
like with Linux distributions.
Worse is that we could run into behavior mismatches when mixing old Cargo with new Rustup or new Rustup with old Cargo where Cargo does the wrong thing.

Likely a first step is providing a warning to users that the toolchain is being ignored.

## Misc

- [RFC #3553](https://github.com/rust-lang/rfcs/pull/3553) was posted for SBOMs
- Like the [feature limit](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html), crates.io now has a [dependency limit](https://github.com/rust-lang/crates.io/pull/7916)
- [`cargo fix`](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.E2.9C.94.20Performance.20of.20.60cargo.20fix.60) can be dramatically slower than `cargo check`.  [#13243](https://github.com/rust-lang/cargo/pull/13243) speeds it up some.
- In a follow up to [RFC #3529](https://github.com/rust-lang/rfcs/pull/3529), [Internals: Integration with mono-repos via intermediate directories](https://internals.rust-lang.org/t/integration-with-mono-repos-via-intermediate-directories/20160?u=epage) was posted.

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Inline snapshotting in snapbox](https://github.com/assert-rs/trycmd/issues/221)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)
- [`cargo update --precise` with pre-release deps](https://github.com/rust-lang/cargo/issues/13290)

Needs design and/or experimentation:
- GC
- [cargo info](https://github.com/rust-lang/cargo/issues/948)
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3485: descriptions](https://github.com/rust-lang/rfcs/pull/3485) (descriptions)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [RFC #3452: Nested packages](https://github.com/rust-lang/rfcs/pull/3452)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
<!-- - [RFC #3553](https://github.com/rust-lang/rfcs/pull/3553) -->
- [RFC #3371: CARGO_TARGET_BASE_DIR](https://github.com/rust-lang/rfcs/pull/3371)
- [RFC #3243: Packages as optional namespaces](https://github.com/rust-lang/rfcs/pull/3243)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)

##### How you can help

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
