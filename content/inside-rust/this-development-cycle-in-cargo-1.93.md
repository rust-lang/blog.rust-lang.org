+++
path = "inside-rust/2026/01/07/this-development-cycle-in-cargo-1.93"
title = "This Development-cycle in Cargo: 1.93"
authors = ["Ed Page"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.93

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.93.

<!-- time period: 2025-10-31 through 2025-12-11 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Rendering diagnostics](#rendering-diagnostics)
  - [Linting](#linting)
  - [Turn all warnings into errors](#turn-all-warnings-into-errors)
  - [Shell completions](#shell-completions)
  - [Build dir layout](#build-dir-layout)
  - [Custom final artifacts](#custom-final-artifacts)
  - [Target dir locking](#target-dir-locking)
  - [Structured logging](#structured-logging)
  - [Config include](#config-include)
  - [`pubtime`](#pubtime)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-override](https://github.com/eopb/cargo-override) which helps manage your [`patch` table](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section).

Thanks to [eopb](https://github.com/eopb) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### Rendering diagnostics

*Update from [1.92](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/#misc)*

[jneem](https://github.com/jneem) continued the work of migrating to [annotate-snippets](https://github.com/rust-lang/annotate-snippets-rs/) in [#16143](https://github.com/rust-lang/cargo/pull/16143)).
This effort is being tracked in [#15944](https://github.com/rust-lang/cargo/issues/15944).

A compiler major change proposal (or MCP) was posted to switch nightly rustc over to using
[annotate-snippets](https://github.com/rust-lang/annotate-snippets-rs/) ([compiler-team#937](https://github.com/rust-lang/compiler-team/issues/937)).
That experiment went well enough that it has now been approved for general use by rustc ([compiler-team#947](https://github.com/rust-lang/compiler-team/issues/947)).
The relevance to Cargo is that this lowers the barrier for enabling the unicode renderer for all diagnostics.
[Muscraft](https://github.com/Muscraft) started this off by adding to Cargo `-Zrustc-unicode` ([#16243](https://github.com/rust-lang/cargo/pull/16243)).

Before:
```
warning: used import from `std` instead of `core`
  --> src/error.rs:25:5
   |
25 | use std::num::NonZeroUsize;
   |     ^^^ help: consider importing the item from `core`: `core`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.92.0/index.html#std_instead_of
_core
note: the lint level is defined here
  --> src/lib.rs:53:9
   |
53 | #![warn(clippy::std_instead_of_core)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
After:
```
warning: used import from `std` instead of `core`
   ╭▸ src/error.rs:25:5
   │
25 │ use std::num::NonZeroUsize;
   │     ━━━ help: consider importing the item from `core`: `core`
   │
   ╰ help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#std_instead_of_core
note: the lint level is defined here
   ╭▸ src/lib.rs:53:9
   │
53 │ #![warn(clippy::std_instead_of_core)]
   ╰╴        ━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

You can enable this selectively for nightly by adding it to your `~/.cargo/config.toml` file:
```toml
[unstable]
rustc-unicode = true
```

Stabilization is being tracked in [rust#148607](https://github.com/rust-lang/rust/issues/148607)

### Improving diagnostics

*Update from [1.92](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/#misc)*

We wanted to give thanks to all the people who helped improve the quality of Cargo's error messages:
- [0xPoe](https://github.com/0xPoe) ([#16125](https://github.com/rust-lang/cargo/pull/16125))
- [Amberley-Sz](https://github.com/Amberley-Sz) ([#16241](https://github.com/rust-lang/cargo/pull/16241))
- [epage](https://github.com/epage) ([#16216](https://github.com/rust-lang/cargo/pull/16216), [#16225](https://github.com/rust-lang/cargo/pull/16225), [#16227](https://github.com/rust-lang/cargo/pull/16227), [#16233](https://github.com/rust-lang/cargo/pull/16233), [#16256](https://github.com/rust-lang/cargo/pull/16256))
- [motorailgun](https://github.com/motorailgun) ([#16207](https://github.com/rust-lang/cargo/pull/16207), [#16268](https://github.com/rust-lang/cargo/pull/16268))
- [TanmayArya-1p](https://github.com/TanmayArya-1p) ([#16338](https://github.com/rust-lang/cargo/pull/16338))

<!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-11-11.md for 16268 -->

### Linting

*Update from [1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/#all-hands-cargo-linting)*

[weihanglo](https://github.com/weihanglo) added the lint `implicit_minimum_version_req`
([#16321](https://github.com/rust-lang/cargo/pull/16321)),
improving the linting system along the way (
[#16320](https://github.com/rust-lang/cargo/pull/16320),
[#16324](https://github.com/rust-lang/cargo/pull/16324),
[#16364](https://github.com/rust-lang/cargo/pull/16364)
).

<!-- https://github.com/rust-lang/cargo/pull/16321#discussion_r2578707514 -->

Something that came up in #16321 is whether we should lint dependencies in `workspace.dependencies` or on every time that one of those is inherited.

Or put another way, given `./Cargo.toml`:
```toml
[workspace]
members = ["a"]

[workspace.dependencies]
clap = "4"
```

and `./a/Cargo.toml`:
```toml
[package]
name = "a"

[dependencies]
clap.workspace = true
```

Should you see the following each time you inherit a dependency:
```
[WARNING] dependency version requirement lacks full precision
 --> Cargo.toml:7:7
  |
7 | dep = "1"
  |       ^^^
  |
[NOTE] dependency `dep` was inherited
 --> a/Cargo.toml:8:5
  |
8 | dep.workspace = true
  |     ----------------
  = [NOTE] `cargo::imprecise_version_requirements` is set to `warn` in `[lints]`
```

Or see the following only once, regardless of whether you inherit:
```
[WARNING] dependency version requirement without an explicit minimum version
 --> Cargo.toml:7:7
  |
7 | dep = "1"
  |       ^^^ missing full version components
  |
  = [NOTE] `cargo::implicit_minimum_version_req` is set to `warn` in `[lints]`
```

To liken this to Rust code,
should we treat inherited dependency sources as macros,
injecting their content into the caller,
or like functions which act in isolation.
Macros emit lints for each invocation and are subject to the caller's lint level.
Functions emit lints only for where they are defined, not used, and have their own lint level controlling them.

When linting `workspace.dependencies`,
what controls the lint level?
We already have the need for "workspace lints" for linting
virtual workspaces (e.g. [#13723](https://github.com/rust-lang/cargo/issues/13723)) and
the dependency tree (e.g. [#9930](https://github.com/rust-lang/cargo/issues/9930)),
so this problem isn't unique.
The current thought is we'd use `workspace.lints` not just for inheriting lints for for controlling workspace lints.
We may need to do a fallback to `lints` for implicit workspaces,
like we do for `package.resolver`.

Another problem is what `edition` is applicable?
In a previous issue with dependency inheritance,
we framed the problem around the act of inheriting and put the responsibility on the package.
That worked well for that case but won't work for all of these other cases.
We have talked for a while of a need for a workspace `edition`.
Luckily, we can punt on this until we need to make a change across editions.

In the end, we decided to lint the `workspace.dependencies` entry
though we have flexibility to change this in the future,
especially with this feature being unstable.

In response, [weihanglo](https://github.com/weihanglo) posted [#16367](https://github.com/rust-lang/cargo/pull/16367)
to update the `unknown_lints` lint to match what we did for `implicit_minimum_version_req`.

### Turn all warnings into errors

*Update from [1.81](https://blog.rust-lang.org/inside-rust/2024/08/15/this-development-cycle-in-cargo-1.81/#turn-all-warnings-into-errors)*

[jyn514](https://github.com/jyn514) 
posted on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Environment.20variable.20for.20.5Blints.5D.20table.3F/near/548176767)
looking at updating the Rust bootstrap process to not require a full rebuild when denying or allowing warnings.
The [unstable `build.warnings`](https://doc.rust-lang.org/cargo/reference/unstable.html#warnings) seemed like a perfect fit.

When jyn514 tried to apply this to bootstrap in
[#148332](https://github.com/rust-lang/rust/pull/148332),
they ran into problems with hard warnings ([#14802-comment](https://github.com/rust-lang/cargo/issues/14802#issuecomment-3487140567)).
In the end, we decided to ignore hard warnings which
[epage](https://github.com/epage) addressed in [#16213](https://github.com/rust-lang/cargo/pull/16213).

jyn514 also pointed out on Zulip that this does not abort the build after the first package with warnings,
unlike `RUSTFLAGS=-Dwarnings`.
epage suggested have it key off of `--keep-going` for whether it should error after the first set of warnings or show all warnings.
This was not discussed further.

### Shell completions

*Update from [1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83/#shell-completions)*

We'd like to thank the people who helped move forward our new auto completion system ([#14520](https://github.com/rust-lang/cargo/issues/14520)):
- [charmitro](https://github.com/charmitro) ([#16322](https://github.com/rust-lang/cargo/pull/16322))
- [epage](https://github.com/epage) ([#16327](https://github.com/rust-lang/cargo/pull/16327), [#16296](https://github.com/rust-lang/cargo/pull/16296))
- [jakobhellermann](https://github.com/jakobhellermann) ([#16210](https://github.com/rust-lang/cargo/pull/16210), [#16215](https://github.com/rust-lang/cargo/pull/16215))

### Build dir layout

*Update from [1.92](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/#build-dir-layout)*

[epage](https://github.com/epage/) proposed a transition plan on [#15010](https://github.com/rust-lang/cargo/issues/15010#issuecomment-3618689353) with the current focus being on testing, including:
- [crater](https://rustc-dev-guide.rust-lang.org/tests/crater.html) ([rust#149852](https://github.com/rust-lang/rust/pull/149852))
- cargo's test suite ([#16375](https://github.com/rust-lang/cargo/pull/16375))
- manual testing

This led to several fixes (
[#16300](https://github.com/rust-lang/cargo/pull/16300),
[#16335](https://github.com/rust-lang/cargo/pull/16335),
[#16348](https://github.com/rust-lang/cargo/pull/16348),
).

[epage](https://github.com/epage/) also did an audit of the feature for any potential changes that could impact testing.
This led to:
- Slipping in more layout changes ([#16351](https://github.com/rust-lang/cargo/pull/16351),
[#16345](https://github.com/rust-lang/cargo/pull/16345))
- Finalizing the names of the directories on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/.E2.9C.94.20directory.20naming.20with.20new.20build.20layout/near/562137402)
- Double checking any other open questions

Next will be working through the test results and opening this up for wider testing by doing a Call for Testing.

### Custom final artifacts

While the build-dir layout is an implementation detail,
there are legitimate reasons for relying on it and it would help with future transitions to provide solutions for people who are relying on it today.

One such use case is accessing final artifacts created by build scripts
([#13663](https://github.com/rust-lang/cargo/issues/13663)).
In considering this, we need to keep in mind that:
- there is the potential for any build script in the dependency tree to create such an artifact
- Cargo tries to avoid collisions in intermediate and final artifacts, see also [#6313](https://github.com/rust-lang/cargo/issues/6313)
- Cargo tries to be safe under concurrent access between multiple `cargo` invocations and between multiple build scripts within a single `cargo` invocation.

Cargo can't just provide build scripts with direct access to the
[artifact-dir](https://github.com/rust-lang/cargo/issues/6790).
This would allow any build script to participate but Cargo couldn't report collisions and this would not be safe for concurrent access.

Build scripts could have a special directory to stage final artifacts.
Cargo would then need to walk all of the staging directories,
check for collisions, and then copy them over to the artifact-dir.
This does make predicting and resolving collisions by users more difficult as a user needs to track what every build script in their dependency tree does.
This may also have a performance impact if the build script can directly generate to the staging directory and needs to copy them over but they are unused by the user.

When discussing expanding the ability for build scripts to communicate with dependents
([#3544](https://github.com/rust-lang/cargo/issues/3544)),
another idea came up.
We can introduce a new build script directive for specifying an artifact,
both where it comes from and where it should be placed, relative to the artifact-dir.
These artifacts would only be uplifted into the artifact-dir for selected packages, the same rule that applies to existing artifacts.
A build script could read `cargo::metadata` output from a dependency's build script and generate an artifact build script directive for it.

So going back to our criteria
- Any build script will be able to generate a final artifact though it does require some manual work
- Cargo could check the build script directives and report an error for collisions.  Users will have full visibility into collisions and autonomy for responding to these errors as they own all of the affected build scripts.  The errors would come at the end of the build which is unfortunate.
- Cargo will be doing the uplifting and can ensure the locations it will access are locked

<!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-11-04.md -->
This can serve as a polyfill for
[RFC #3035](https://github.com/rust-lang/rfcs/pull/3035) (postponed),
providing a workaround for it and allowing for further experimentation.
One way this is lacking as a polyfill is
[artifact dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) support.
We could name the directive `artifact` and automatically create artifact dependency variables when you depend on the package, rather than requiring [#3544](https://github.com/rust-lang/cargo/issues/3544).
The artifact could be named for the destination but if we allow directories as destinations,
we don't have a unique name.
Depending on artifacts from a package would also be implicit in the dependency specification unlike regular artifact dependencies.

Some open questions on this include
- what to name it? `copy`, `artifact`, `copy-artifact`, or something else?
- what should the copy semantics be when source or destination is a directory without a trailing `/`?
- what separator should be used between the source and destination?
- when copying directories, do we check whether directories collide or their content?
- if and how we tie this to the artifacts dependency system

One potential route forward is to defer a connection to artifact dependencies and either make sure the design can handle adding it later or make the design low level and build an artifact solution on top of it later.

### Target dir locking

One thing that slows down builds is being blocked on another build,
particularly when running `cargo check` on the command-line while rust-analyzer is also running `cargo check`.
This happens because the entire target directory underneath `target/<platform>/<profile>` is managed by a single lock to avoid concurrent builds from negatively impacting each other.

[ranger-ross](https://github.com/ranger-ross) is working towards a cross-project cache
([#5931](https://github.com/rust-lang/cargo/issues/5931))
with [build dir layout changes](#build-dir-layout) being one stepping stone.
They decided to explore a finer-grained locking scheme for the target directory as a user-facing deliverable on top of the build dir layout changes as well as to explore caching schemes for the cross-project cache, assuming we can get reuse.

A target dir is usually made up of two concepts, a build dir for intermediate artifacts (e.g. rlibs) and an artifacts dir for final artifacts (e.g. binaries).
We've organize the build dir around build units for Cargo to lock them individually.
Before any build unit locking can be meaningful, we need to adjust how we lock the artifacts dir.
As most of the concern is about `cargo check` vs `cargo check` / `cargo clippy` / `cargo run` / `cargo test`,
[ranger-ross](https://github.com/ranger-ross) took the simple approach and split the build dir and artifacts dir locks,
avoiding grabbing the artifacts dir lock when no artifacts are produced.
This won't help against `cargo build`, `cargo run`, `cargo test`, or `cargo bench` running in parallel which is likely fine as these are less likely to need to run in parallel.

As for locking intermediate build unit artifacts individually,
things got complicated.
The straight forward way to start is to grab any needed locks around the operation that needs protection.
This means each build unit will grab an exclusive lock for itself and a shared lock for all dependencies.
This runs into several complications:
- prevents [pipelined compilation](https://blog.rust-lang.org/2019/09/26/Rust-1.38.0/#pipelined-compilation) as later build units need to wait for the rlib to be generated when they might have only needed the rmeta.
- performance impact of each build unit grabbing a lock for every dependency
- quickly hitting ulimit's, especially with each build unit grabbing a lock for every dependency

We had hoped to avoid some of this by using a single `(read or append) xor exclusive-write` lock for all of the build dir.
The first problem is that build units are mutable and we don't know ahead of time if we'll be mutating them.
You can see what build inputs can mutate cache entries by [comparing Fingerprints to `Metadata::c_metadata`](https://doc.rust-lang.org/nightly/nightly-rustc/cargo/core/compiler/fingerprint/index.html#fingerprints-and-unithashs).
If we made cache entries immutable, we'd see an explosion of cache entries as one will exist for every `cargo check` run done by `rust-analyzer` when editing your source.
This would block the locking effort on build-dir GC ([#5026](https://github.com/rust-lang/cargo/issues/5026))
and likely need different reclamation times for different types of entries so Cargo preserves the most relevant ones.
This would have the benefit of not rebuilding when switching back and forth between branches.
However, an even bigger problem is present: we need a build to discover the source files to find out if a cache entry exists.
For the cross-project cache, that won't be an issue because we expect Cargo will only be storing entries in that cache for immutable packages.

When exploring the per-build-unit lock design,
[ranger-ross](https://github.com/ranger-ross) optimized it by caching locks for dependencies.
This led to the idea of having the top-level build operation own the locks that are lazily acquired, ensuring we'd need at most one lock per build unit.

This design is being further explored on
[Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Build.20cache.20and.20locking.20design/near/548098785)
and in [#16155](https://github.com/rust-lang/cargo/pull/16155).

### Structured logging

This is a [project goal](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-analysis.html) to add structured logging to Cargo so users can view [timings](https://doc.rust-lang.org/cargo/reference/timings.html) and rebuild reasons for past builds,
avoiding the frustration of "why was that slow" or "why did that rebuild" particularly when the build result is not reproducible.

[weihanglo](https://github.com/weihanglo) added structured logging support to cargo
([#16150](https://github.com/rust-lang/cargo/pull/16150))
fleshed it out (
[#16179](https://github.com/rust-lang/cargo/pull/16179),
[#16203](https://github.com/rust-lang/cargo/pull/16203),
[#16282](https://github.com/rust-lang/cargo/pull/16282),
[#16303](https://github.com/rust-lang/cargo/pull/16303),
[#16346](https://github.com/rust-lang/cargo/pull/16346),
[#16350](https://github.com/rust-lang/cargo/pull/16350),
[#16378](https://github.com/rust-lang/cargo/pull/16378),
)
and added `cargo report timings` ([#16377](https://github.com/rust-lang/cargo/pull/16377)).

You can enable this for nightly toolchains by adding to your `~/.cargo/config.toml`
```toml
[build.analysis]
enabled = true
```

There is still a lot to figure out on the path to stabilization, including:
- The data schema, especially if we want to work towards unifying this schema with Cargo's json output which would improve Cargo's json output ([#8283](https://github.com/rust-lang/cargo/issues/8283)) which would unblock a new, faster, more flexible architecture for `cargo fix` (e.g. [cargo-fixit#68](https://github.com/crate-ci/cargo-fixit/issues/68)).  See also [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build.20analysis.20log.20format/with/563521641)
- The interface for the new `cargo report` commands

### Config include

[`-Zconfig-include`](https://doc.rust-lang.org/cargo/reference/unstable.html#config-include) allows for a `.cargo/config.toml` file to include others, e.g.
```toml
include = ["frodo.toml", "samwise.toml"]
```

This has been stuck in unstable limbo since 2019
with [various concerns](https://github.com/rust-lang/cargo/issues/7723#issuecomment-1595887492).
There was an attempt to stabilize it in 2024
with the intent of expanding it in the future.
There was [a desire](https://github.com/rust-lang/cargo/issues/7723#issuecomment-2386278512) to ensure we could support those future possibilities before preceding.

[weihanglo](https://github.com/weihanglo) picked up the work on this, starting with merging of arrays of tables across layers
([#16103](https://github.com/rust-lang/cargo/pull/16103)),
having only supported arrays of strings before.
They then added support for `include` being specified by an array of tables ([#16174](https://github.com/rust-lang/cargo/pull/16174)).
This will allow for adding fields in the future to customize the include behavior,
like declaring an include optional
([#16180](https://github.com/rust-lang/cargo/pull/16180)).

As for allowing templates and globs in the future,
the original thought was to opt-in via fields on the config table.
However, other parts of Cargo directly support templates or globs.
We deferred the decision on this by erroring if glob or template syntax is present
([#16285](https://github.com/rust-lang/cargo/pull/16285))
which will allow us to go down either route in the future.

<!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-11-25.md -->
Another question that came up during stabilization is what syntax forms are allowed and encouraged.
At this point, the following were supported:
```toml
include = "a.toml"

include = [
  "a.toml",
  "b.toml",

]

include = [
  { path = "a.toml" },
  { path = "b.toml" },
]

[[include]]
path = "a.toml"
[[include]]
path = "b.toml"
```
In TOML, there is no logical difference between an array of inline-tables and an Array-of-Tables but which syntax is used can affect both the user and the design.
If Array-of-Tables is our encouraged syntax,
then we should likely drop `include = ""` and `include = [""]`
as those are shorthands that require transforming the code to a different structure when new features are needed.
An array of inline-tables has the problem that it is a top-level key which can be confusing in TOML because the root table is order dependent (has to be at the top) unlike any other table.
In this case, that can be a strength because `include` is special and changes how the file is processed, so it likely should be at the top to help call attention to it.
We decided to prefer an array of inline-tables in our documentation and in any future `.cargo/config.toml` style guide.
[weihanglo](https://github.com/weihanglo) updated the docs in [#16301](https://github.com/rust-lang/cargo/pull/16301).
We decided to drop the `include = ""` form as too specialized.
[weihanglo](https://github.com/weihanglo) did this in [#16298](https://github.com/rust-lang/cargo/pull/16298).

<!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-11-18.md -->
One workflow annoyance with `.cargo/config.toml` is config files specific to both a project and a user ([#14565](https://github.com/rust-lang/cargo/issues/14565)).
If a project commits their config file,
the only option for the user is to move the repo into a subdirectory that they put their own config file in.
If a project does not commit a config file, a user can create one but it won't be ignored by default in git and users frequently commit any file that shows up in `git status`.
With `-Zconfig-include`, projects could gitignore a `.cargo/config.user.toml` and have:
```toml
include = [
    { path = "config.user.toml", optional = true }
]
```
However, this requires projects to update their MSRV and choose to adopt this convention.
Improving this now is worth considering for any impact on the design of config-include that we can't change once stabilized.
For example, `.cargo/` has very little use today but that can change after `-Zconfig-include` is stabilized.
What if we reserved a name like `.cargo/config.user.toml` before stabilizing so we could auto-load that file and have `cargo init` gitignore it?
In addition to designing enough to know what to reserve,
this overlooks the existence of `--config PATH` for which users could have their own files in `.cargo/`, increasing the chance someone would be negatively impacted by reserving the name.
Blocking stabilizing `-Zconfig-include` on researching the answers to these questions did not seem worth it.
There are still other options, for example `.cargo/config/` is reserved by happenstance of Cargo always assuming `.cargo/config` is a TOML file and erroring if it is a directory.

[weihanglo](https://github.com/weihanglo) stabilized this feature in [#16284](https://github.com/rust-lang/cargo/pull/16284).

### `pubtime`

Recently, there has been an increased interest in supporting gradual rollouts of new package versions by giving users control over how old a package's version must be before being allowed to upgrade to it.

<!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-12-09.md -->
The first step for Cargo to support this is to know the publish time of registry packages.
Cargo's [dependency resolution](https://doc.rust-lang.org/cargo/reference/resolver.html) operates off a Summary from the Registry Index.
We need to extend that format with the publish time ([#15491](https://github.com/rust-lang/cargo/issues/15491))
and have crates.io do a backfill while gracefully handling registries that don't provide this information.
After discussion with the crates.io team,
[epage](https://github.com/epage) implemented support for this within Cargo as well as "time traveling dependency resolution" ([#5221](https://github.com/rust-lang/cargo/issues/5221)) as an end-user way of exploring this functionality in [#16265](https://github.com/rust-lang/cargo/pull/16265)
[Turbo87](https://github.com/Turbo87) implemented support in the staging instance of crates.io in [crates.io#12315](https://github.com/rust-lang/crates.io/pull/12315).
In preparation for stabilization of this Summary field,
the Cargo team discussed the design and
[epage](https://github.com/epage) applied the feedback in [#16369](https://github.com/rust-lang/cargo/pull/16369).
They then posted the stabilization request in [#16372](https://github.com/rust-lang/cargo/pull/16372).

While "time traveling dependency resolution" via `cargo generate-lockfile --publish-time 2025-12-01T01:01:01Z` is straightforward to implement,
it isn't as easy to stabilize.
This only works for registry packages where the registry provides the needed information.
Even then it is incomplete as we only track the latest `yanked` state, rather than the history of yanking and unyanking.
We'd need to make sure we don't over-promise to the user what this can accomplish.
We also need to clearly communicate what is happening to the user when the feature is active.
These unresolved issues are being tracked in [#16271](https://github.com/rust-lang/cargo/issues/16271).

On [#15973](https://github.com/rust-lang/cargo/issues/15973),
there is on-going exploration of requirements, prior art, and designs for `minimumReleaseAge`.

<!-- ## Design discussions -->

## Misc

- [osiewicz](https://github.com/osiewicz) added `--workspace` to `cargo clean` ([#16263](https://github.com/rust-lang/cargo/pull/16263))
- [osiewicz](https://github.com/osiewicz) sped up `cargo clean -p` and `cargo clean --workspace` ([#16264](https://github.com/rust-lang/cargo/pull/16264))
- [weihanglo](https://github.com/weihanglo) added `net.git-fetch-with-cli` support to [git shallow clones](https://doc.rust-lang.org/cargo/reference/unstable.html#git) ([#16156](https://github.com/rust-lang/cargo/pull/16156))
- [Muscraft](https://github.com/Muscraft) added spell checking to cargo's source and documentation([#16122](https://github.com/rust-lang/cargo/pull/16122))
- [*(Update from 1.90)*](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/#all-hands-cargo-explain) [orhun](https://github.com/orhun) is exploring an interactive pager-like TUI for `cargo tree` at [cargo-tree-tui](https://github.com/orhun/cargo-tree-tui/) as a testing ground for extending other commands with them, see also [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/TUI.20for.20cargo/near/556994881).
- [*(Update from 1.92)*](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/#cargo-script) [epage](https://github.com/epage) further polished Cargo Script ([#16334](https://github.com/rust-lang/cargo/pull/16334), [#16248](https://github.com/rust-lang/cargo/pull/16248), [#16169](https://github.com/rust-lang/cargo/pull/16169)) <!-- https://github.com/rust-lang/cargo-team/blob/main/meetings/sync-meeting/2025-11-04.md -->
- [weihanglo](https://github.com/weihanglo) posted [#16309](https://github.com/rust-lang/cargo/pull/16309) for an unstable feature for speeding up rustdoc builds.

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Project goals in need of owners
- [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h2/pub-priv.html)
<!--
- [Prototype Cargo build analysis](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-analysis.html)
- [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)
- [Rework Cargo Build Dir Layout](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html)
- [Prototype a new set of Cargo "plumbing" commands](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html)
- [Finish the libtest json output experiment](https://rust-lang.github.io/rust-project-goals/2025h2/libtest-json.html)
-->

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/14520)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)

<!--
Needs design and/or experimentation:
-->

Planning:
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
- [Pre-RFC: Global, mutually exclusive features](https://internals.rust-lang.org/t/pre-rfc-mutually-excusive-global-features/19618)
- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553)
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)

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
