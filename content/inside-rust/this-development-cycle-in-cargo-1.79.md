+++
path = "inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79"
title = "This Development-cycle in Cargo: 1.79"
authors = ["Ed Page"]
aliases = ["inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.79

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.79.

<!-- time period: 2024-03-22 through 2024-05-02 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Deprecations](#deprecations)
  - [User-controlled diagnostics](#user-controlled-cargo-diagnostics)
  - [MSRV-aware Cargo](#msrv-aware-cargo)
  - [Edition 2024](#edition-2024)
  - [Normalizing Published Package Files](#normalizing-published-package-files)
  - [`cargo info`](#cargo-info)
- [Design discussions](#design-discussions)
  - [Applying patch files to dependencies](#applying-patch-files-to-dependencies)
  - [Cargo script](#cargo-script)
  - [SBOM](#sbom)
  - [Nested packages](#nested-packages)
  - [Workspace inheritance of deps](#workspace-inheritance-of-deps)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-outdated](https://crates.io/crates/cargo-outdated) which gives an overview of out-of-date dependencies.
As of Cargo 1.78, we include some of this information in the `cargo-update` output
([#13372](https://github.com/rust-lang/cargo/pull/13372)).
Try giving `cargo update --dry-run --verbose` a try!
As for how we could further improve our reporting of outdated dependencies,
see [#4309](https://github.com/rust-lang/cargo/issues/4309).

Thanks to [LukeMathWalker](https://github.com/LukeMathWalker) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

##### Deprecations

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#deprecated-cargotoml-fields)*

[weihanglo](https://github.com/weihanglo/) dug into the cargo code base to enumerate official and unofficial deprecations and recorded them in
[#13629](https://github.com/rust-lang/cargo/issues/13629).

<!-- team meeting: 2024-04-16 -->
The deprecations ended up being divided into the following categories:

**Deprecate, remove on next Edition:** including [#13747](https://github.com/rust-lang/cargo/pull/13747), [#13804](https://github.com/rust-lang/cargo/pull/13804), and [#13839](https://github.com/rust-lang/cargo/pull/13839).

**Deprecate but never remove:**  This is targeted in areas like the CLI or `.cargo/config.toml` which don't have an Edition mechanism to evolve them.

**Remove, breaking compatibility:**  This is focused on bugs with minimized impact to users.

An easy example is `badges.workspace = true` allowing inheritance from `package.badges`.
This was not in the RFC, undocumented, and didn't follow the standard pattern for inheritance making it harder to discover.
We removed support for this in [#13788](https://github.com/rust-lang/cargo/pull/13788).

<!-- team meeting: 2024-04-30 -->
Cargo also allowed dependencies without a source (e.g. `dep = {}`).
This was originally removed 3 years ago in [#9686](https://github.com/rust-lang/cargo/pull/9686)
but was reverted after it was reported to have broken an old version of the `bit-set` crate which was used by `libusb` which has gone unmaintained ([see #9885](https://github.com/rust-lang/cargo/issues/9885)).
We revisited this and decided to remove support for it again
(see [#13775](https://github.com/rust-lang/cargo/pull/13775))
and soon after a user of libusb noticed again
([#13824](https://github.com/rust-lang/cargo/issues/13824)).
After looking at this more carefully, we decided to stick with our original decision.
We broke people 3 years ago, been warning since that it will be removed, and there are two maintained replacement packages
([rusb](https://crates.io/crates/rusb) and [nusb](https://crates.io/crates/nusb)).

**Re-evaluate in the future:**  In particular, for [#4797](https://github.com/rust-lang/cargo/pull/4797), we want to wait until there is a stable mechanism to replace it.

##### User-controlled cargo diagnostics

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#user-controlled-cargo-diagnostics). In summary, this aims to add [user-controlled cargo lints](https://github.com/rust-lang/cargo/issues/12235) that look like rustc and are controlled through the [`[lints]` table](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section)*

[Muscraft](https://github.com/Muscraft) started off this development cycle with a rough sketch of lint system ([#13621](https://github.com/rust-lang/cargo/pull/13621)) and fleshed it out and polished it up including
- Reporting why a lint is being shown ([#13801](https://github.com/rust-lang/cargo/pull/13801))
- Handling `forbid`'s special behavior ([#13797](https://github.com/rust-lang/cargo/pull/13797/commits))
- Support for unstable lints ([#13805](https://github.com/rust-lang/cargo/pull/13805))

<!-- team meeting: 2024-04-30 -->
Original lint  names were written using kebab-case.
In [#13635](https://github.com/rust-lang/cargo/pull/13635),
they were switched to also support snake_case to match rustc.
After we had to deal with deprecating snake_case fields in `Cargo.toml`,
[Muscraft](https://github.com/Muscraft) brought up whether we should initially only support one case.
A couple of the participants stylistically preferred kebab-case, especially to match the rest of the manifest.
However, rustc considers snake_case to be the canonical form and we decided that would be a good starting point
([#13837](https://github.com/rust-lang/cargo/pull/13837)).
We can always add a second style later, if we so wished.

<!-- team meeting: 2024-04-16 -->
Our test case for this functionality is deprecating implicit features in Edition 2024.
We modeled this as a deprecation warning for implicit features in existing Editions
while Edition 2024 will report the optional dependency as unused ([#13778](https://github.com/rust-lang/cargo/pull/13778)).
We discussed how we wanted to model unused optional dependemncies.
At a high level, the  most direct way is we change how we internally enumerate features to be based on the edition.
However, this doesn't play well with registry packages.
We resolve them off of the Index which doesn't have the full `Cargo.toml`, particularly the Edition,
and prior versions of Cargo would read these Index entries and generate implicit features, breaking on upgrade of Cargo without extra care.
Maybe we should work to support the Edition in the Index but we don't need to do that now.
We ended up stripping unused optional dependencies from the published `Cargo.toml` and the Index.
The way this was done also means they won't show up in `Cargo.lock` like unused `workspace.dependencies`.
As a side effect, some lints may not run against these dependencies.

![rendering of unused optional dependency lint](../../../../images/inside-rust/2024-05-07-this-development-cycle-in-cargo-1.79/lint.stdout.term.svg)

##### MSRV-aware Cargo

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#msrv-aware-cargo)*

The subset needed for Edition 2024 is effectively code complete!
Feel free to [try it out](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#msrv-aware-resolver)
and [leave feedback](https://github.com/rust-lang/cargo/issues/9930).

We continued to iterate on how we report lockfile changes, including
- Reporting that dependencies changed on any command, not just `update` ([#13561](https://github.com/rust-lang/cargo/pull/13561), [#13759](https://github.com/rust-lang/cargo/pull/13759))

We've continued to iterate on the MSRV resolver's behavior, including
- Defaulting to `rustc -V` when your `package.rust-version` is unset ([#13743](https://github.com/rust-lang/cargo/pull/13743))
- Tweaked the behavior when a dependency's `package.rust-version` is unset ([#13791](https://github.com/rust-lang/cargo/pull/13791))
- Avoiding it for `cargo install` ([#13790](https://github.com/rust-lang/cargo/pull/13790))

As for controlling the resolver policy, we've implemented:
- `--ignore-rust-version` disables MSRV dependency resolution ([#13738](https://github.com/rust-lang/cargo/pull/13738))
- We added `--ignore-rust-version` to `cargo update` and `cargo generate-lockfile` ([#13742](https://github.com/rust-lang/cargo/pull/13742))
- We added a placeholder config field so it can be forced on or off ([#13769](https://github.com/rust-lang/cargo/pull/13769)).  We still need final names for this, see [#13540](https://github.com/rust-lang/cargo/issues/13540).
- We added `package.resolver = "3"` ([#13776](https://github.com/rust-lang/cargo/pull/13776))
- We made this the default resolver for Edition 2024 ([#13785](https://github.com/rust-lang/cargo/pull/13785))

##### Edition 2024

*[Update from 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#meta-2024-edition)*

In addition to the above, work on Editions draws more attention to `cargo fix`.
This includes [#13728](https://github.com/rust-lang/cargo/pull/13728) and [#13792](https://github.com/rust-lang/cargo/pull/13792)
by [weihanglo](https://github.com/weihanglo/).

We also discussed [on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.60cargo.20fix.20--edition.60.20and.20.60Cargo.2Etoml.60.20changes) if there are `Cargo.toml` changes that should be made by `cargo fix --edition`, like updating the `package.edition` or the `package.rust-version`.

The challenge with updating `package.edition` is `cargo fix` only runs for one set of build targets, platforms, and feature combinations and so we don't know when an entire project is fully converted over to the new edition.
The user might need to make multiple calls to migrate and updating `package.edition` too early can get in the way of that.

##### Normalizing Published Package Files

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#performance)*

After much work (
[#13666](https://github.com/rust-lang/cargo/pull/13666)
[#13693](https://github.com/rust-lang/cargo/pull/13693)
[#13701](https://github.com/rust-lang/cargo/pull/13701)
[#13713](https://github.com/rust-lang/cargo/pull/13713)
[#13729](https://github.com/rust-lang/cargo/pull/13729)
), published and vendored `Cargo.toml` files will now include all build targets explicitly enumerated.

Benefits
- You cannot bypass the checksum in adding a build target to a vendored dependency by dropping a file
- When all build targets have an explicit path, you now get a warning if one is excluded when packing, helping to catch mistakes
- You can now intentionally exclude a build target from publishing without having to set the path
- It is easier to audit changes to the build targets across versions
- We hope this opens the door to more performance improvements when parsing large dependency trees

As a side effect, the output from `cargo vendor` will vary by Cargo version.
We try to minimize this kind of churn but felt it was justified in this case.

##### `cargo info`

*[Update from 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#cargo-info)*

There was some recent discussion on an issue for how `cargo add` should render features
([#10681](https://github.com/rust-lang/cargo/issues/10681)).
epage figured `cargo info` could be a good place to try out their proposal
([cargo-information#140](https://github.com/hi-rustin/cargo-information/pull/140)).
A question aspect of this was to apply the same rendering to dependencies to distinguish between required, activated-optional, and deactivated-optional dependencies.

epage also made the auto-selection of what version to show a little smarter.
Instead of showing the latest when a version is unspecified,
`cargo info` tries to be smart and show you a version that is relevant.
Before, that was a version from your lockfile or a MSRV-compatible version.
With [cargo-information#137](https://github.com/hi-rustin/cargo-information/pull/137),
we don't just check the lockfile but first check the direct dependencies of the package you are in and then the direct dependencies of all workspace members, making it more likely what will be shown is what you will be using.

![rendering of cargo-info's verbose output using SVG](../../../../images/inside-rust/2024-05-07-this-development-cycle-in-cargo-1.79/info.stdout.term.svg)
*(verbose output, normally dependencies are hidden)*

At this point, [`cargo-information`](https://crates.io/crates/cargo-information) feels like it could be ready to merge into cargo.
Please give it a try and [let us know what you think](https://github.com/hi-rustin/cargo-information/issues)!

## Design discussions

#### Applying patch files to dependencies

*[Update from 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#postponing-rfcs)*

Previously, we discussed closing this RFC, asking for an experimental implementation to help flesh out the design.
[weihanglo](https://github.com/weihanglo/) stepped in with a proof of concept in [#13779](https://github.com/rust-lang/cargo/pull/13779).
High level design discussions are on-going on that PR.

#### Cargo script

*[Update from 1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#cargo-script)*

T-lang has approved [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503) for the syntax of embedding manifests.
This still leaves [RFC #3502](https://github.com/rust-lang/rfcs/pull/3502).

<!-- team meeting: 2024-04-23 -->
While cargo script is primarily focused on exploration,
there will be times people want to do heavy analysis and want release builds (see [RFC comment](https://github.com/rust-lang/rfcs/pull/3502#discussion_r1337996703)).

We could add `cargo --release <script>`.
This runs into a common challenge of `#!` processing being limited in processing of interpreter arguments which can't be resolved in a fully cross-platform way.

Unfortunately, you can't just do:
```toml
[profile.dev]
inherits = "release"
```
as that is disallowed for built-in profiles.
This also infects anything else you do on the script, like `cargo test`.

We could introduce a `run` profile as that is reserved for cargo's use.
The name implies we should also update `cargo run` to use it which would cause the target directory to change from `cargo build`, causing extra builds and a larger `target/`.

We could add a config for [default-profile](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/cargo.20build.20default.20profile).
That would be stored in a separate config file that needs to go with the script.
It also would shift the emphasis to the user setting it, rather than the script author.
Likely, the script author knows best what to be done.
We'd likely want to put this in the manifest.

Knowing that there are ways to improve this in the future in a backwards compatible way, we were fine deferring this out of the RFC.

Another challenge is the nuance of how to interpret `cargo <something>`.
It could be a built-in command, an alias, a third-party command, or a script.
How do we decide which to try first?
This is the continuation of [this RFC thread](https://github.com/rust-lang/rfcs/pull/3502#discussion_r1337912575).
epage had more details on prior art in [epage/cargo-script-mvs#154](https://github.com/epage/cargo-script-mvs/issues/154).
In looking at the different cases, the main problem we found is that a script named `cargo test` would run `cargo-test`.
To run the script, you'd need to run `cargo ./test`.

An obvious way of solving this problem is a separate `cargoscript` binary.
The RFC goes into some trade offs on this.
Its particularly an issue if the binary name starts with `cargo-` as that would be picked up as a third-party command.

However, this only really happens with non-standard `PATH`s on Linux and Mac (Windows is a separate case) or having too-generic of script names in your `PATH`.
This lowered the priority for us.

We could possibly take advantage of `AT_EXECFN` where supported so cargo can detect its being run as a `#!` interpreter and bypass the precedence rules.

Talking about this brought up a new concern: `cargo <script>` loads the config file based on `<script>`, rather than your current working directory.
This is like `cargo install` and unlike all other cargo commands.
The big concern that was raised was with running a script out of a temp directory or a downloads directory and picking up an unexpected config file.
Now, the RFC specifies `cargo <script>` to load the config only from `CARGO_HOME`.

With these things resolved the RFC has been
[proposed for merging](https://github.com/rust-lang/rfcs/pull/3502#issuecomment-2072925354).

#### SBOM

*[Update from 1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#sbom)*

Though the RFC is not yet approved,
[justahero](https://github.com/justahero) has created a proof-of-concept of it in [#13709](https://github.com/rust-lang/cargo/pull/13709).

<!-- team meeting: 2024-04-30 -->
As a team, we recently discussed the configuration side of this RFC.
Currently, the RFC lists this as a `build.sbom = <bool>` config field.
When we previously discussed [RFC #2801 (cargo auditable)](https://github.com/rust-lang/rfcs/pull/2801) and shifted our focus to a full SBOM,
the core of the idea was that we could model SBOMs like we model debug info.
We could start it as a
["split"](https://doc.rust-lang.org/nightly/cargo/reference/profiles.html#split-debuginfo)
["full"](https://doc.rust-lang.org/nightly/cargo/reference/profiles.html#debug)
SBOM and evolve from there.
This would imply that this should be a profile setting, rather than a general build setting.
Especially if we eventually support embedding this in the binary,
it seems like something that is dependent on the type of build (release vs development) than something to do unconditionally.
However, the debuginfo analogy falls down in that a project might want both a final SBOM and `cargo auditable` embedded SBOM.

If we set aside the future possibilities,
it is assumed that this will mostly be set by wrapper tools that generate a proper SBOM out of the fragment we generate.
They will be setting it on a per-run basis and not expect users to enable this setting,
making the focus be on a environment variable, rather than config.
In this framing, we aren't tying it to a specific profile and requiring people to do so could add some extra hurdles.

We could explore having a build setting for now and allow for layering a profile setting in the future.
The main issue to be worked out is if there is a reasonable precedence policy to this.

During the discussion, we also came up with another framing.
This could be viewed as a build report that could be used for more than just SBOMs but for external tools to analyze builds, including what was actually built, is being fingerprinted as part of the build, etc.
In this case, its more of a transient build setting.

We were not yet able to reach a conclusion and will need to revisit this.

#### Nested packages

*[Update from 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html#rfc-3452-nested-packages)*

<!-- team meeting: 2024-04-02 -->
Last time the Cargo team discussed this,
we didn't come to any firm conclusion and felt we needed more time to dig into the RFC text.

The core question to the discussion is "do we expect the benefits to be worth the cost?".

An additional use case that came up during the discussion was to combine this with [artifact dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) so you could have build scripts that were full packages that didn't require the overhead of publishing a full workspace.

Nested packages would create a footgun when it comes to workspaces.
If you have a nested package that is a [public dependency](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#public-dependency) of multiple workspace members,
you could start passing types between the packages.
This will work locally because they all use the same instance of the nested package as a dependency.
When you publish, each package will have its own instance of the nested package and the types will become incompatible.

In general, there was unease that nested packages could encourage people to vendor dependencies more often in a way that is less traceable and allows the code to drift in uncontrolled ways while we want to have the workflow encourage people to work directly with upstream.

As a side note, during the discussion, the idea came up for a new `pub(scope)` to allow access to private APIs within a [namespace](https://rust-lang.github.io/rfcs/3243-packages-as-optional-namespaces.html#motivation).

#### Workspace inheritance of deps

[LukeMathWalker](https://github.com/LukeMathWalker) recently announced [cargo autoinherit](https://mainmatter.com/blog/2024/03/18/cargo-autoinherit/) which will consolidate your dependencies of workspace members sources to
[`[workspace.dependencies]`](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table).

While discussing the announcement,
a [footgun came up with regards to `default-features`](https://www.reddit.com/r/rust/comments/1bjdnne/cargoautoinherit_dry_up_your_workspace/kvr6iq1/).
If you set `default-features = true` in the workspace dependencies then `default-features = false` in your package dependencies is ignored
([#12162](https://github.com/rust-lang/cargo/issues/12162)).
This might not be too bad except `default-features = true` is the default including when using the `dep = "1.0"` syntax.
You'd then have to explicitly re-enable `default-features` when needed, for example:
```toml
[workspace.dependencies]
foo = { version = "1.0", default-features = false }

[dependencies]
foo = { workspace = true, default-features = true }
```

While discussing workspace inheritance for the
[`public` dependency field](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#public-dependency)
([#13125](https://github.com/rust-lang/cargo/pull/13125)),
one thought that epage had was that maybe dependency inheritance should focus exclusively on inheriting the source of the dependency and not anything else.
When trying to not repeat oneself,
it can be easy to go overboard and de-duplicate logic that reflects more of the current implementation, and not the requirements.
As the implementation changes, you then have to shift things in and out between the shared and specific state.

Applying that concept to `default-features`,
maybe we should disallow it in workspace dependencies (on a new Edition),
and have the package dependencies' `default-features` always win.
As workspaces don't have an Edition, we could frame this from the package's perspective as it inherits a field, making it so the package's Edition applies.

Going to the full extreme of removing `default-features` and `features` from workspace dependencies might be too large of departure from today.
For example, likely a maintainer would find it convenient to share `serde`'s `derive` feature throughout their workspace.
An incremental step could be to treat the lack of `default-features` in a  workspace dependency as unset.

<!-- team meeting: 2024-03-26 -->
We discussed this in the Cargo team meeting.
To better communicate this, we borrowed a table from a prior discussion on inheriting default-features

| Workspace | Member | 1.64 behaviour | 1.69 behavior | proposed 202X Edition behavior | Reasoning
|:---:|:---:|---|---|---|---
| *nothing* | df=false | Enabled | Enabled, warning that it is ignored | **Disabled** | Basically means let the package control default features when not specified 
| df=true | df=false  | Enabled | Enabled, warning | Enabled, warning | Should have been an error in the past since the field is ignored. May change to an error in the future.
| *nothing* | df=true  | Enabled |  Enabled | Enabled | There is no conflict about default being enabled.
| df=true | df=true | Enabled | Enabled | Enabled | ^Same
| *nothing* | *nothing*  | Enabled  | Enabled | Enabled | No ambiguity.
| df=true | *nothing*   | Enabled | Enabled | Enabled | Enabled | ^Same
| df=false | df=false  | Disabled | Disabled | Disabled | No ambiguity.
| df=false | df=true  | Disabled | **Enabled** | Enabled | This allows members to decide whether or not they want default features. The workaround of `features=["default"]` seems unnecessary.
| df=false | df=nothing  | Disabled | Disabled | Disabled | Natural way to write "give me whatever was written in workspace".  ||

Some points that were discussed
- Documenting this as this deviates from the intended mental model
- Ecosystem churn in switching to this
- If we discourage features in workspace dependencies, it might preclude people who are doing it as low effort [workspace hack](https://crates.io/crates/cargo-hakari).  We felt that we should exercise caution and not remove features, yet.

As this was after the cut off for the 2024 Edition,
this will likely be stuck waiting on the 2027 Edition.

## Misc

- Thank to [PaulDance](https://github.com/PaulDance), we are now publishing `cargo-test-support` and `cargo-test-macro` which can be useful for developing tools that integrate with cargo [#13418](https://github.com/rust-lang/cargo/pull/13418).  The focus is on Cargo's needs, making the APIs and documentation a little rough for others to adopt.
- Thanks for [linyihai](https://github.com/linyihai), we now have unstable support for `cargo update foo --precise <prerelease>` ([#13626](https://github.com/rust-lang/cargo/pull/13626)), last discussed in [1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76.html#rfc-3493-cargo-update---precise-prerelease).

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
<!-- - [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166) -->

Needs design and/or experimentation:
- [GC](https://github.com/rust-lang/cargo/issues/12633)
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)
<!-- - [cargo info](https://github.com/rust-lang/cargo/issues/948) -->

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3485: descriptions](https://github.com/rust-lang/rfcs/pull/3485) (descriptions)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [RFC #3371: CARGO_TARGET_BASE_DIR](https://github.com/rust-lang/rfcs/pull/3371)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
<!-- - [RFC #3452: Nested packages](https://github.com/rust-lang/rfcs/pull/3452) -->
<!-- - [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553) -->
<!-- - Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503)) -->

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
