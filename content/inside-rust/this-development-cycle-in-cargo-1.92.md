+++
path = "inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92"
title = "This Development-cycle in Cargo: 1.92"
authors = ["Ed Page"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.92

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.92.

<!-- time period: 2025-09-18 through 2025-10-30 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Build performance guide](#build-performance-guide)
  - [Cargo Script](#cargo-script)
  - [Public dependencies](#public-dependencies)
  - [Build-dir layout](#build-dir-layout)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-wizard](https://github.com/Kobzol/cargo-wizard) which can optimize your project for build times, runtime performance, or binary size.

Thanks to [Kobzol](https://github.com/Kobzol) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### Build performance guide

On [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/Build.20performance.20section.20in.20the.20Cargo.20book/near/537596927),
[Kobzol](https://github.com/Kobzol)
floated the idea of a build performance guide being added to the
[Cargo book](https://doc.rust-lang.org/cargo).
The first thing we needed to work out was how to handle having small reviewable chunks while having enough content to justify the document.
We decided to hold off on merging anything until the start of this development cycle.
The guide was introduced in
[#15970](https://github.com/rust-lang/cargo/pull/15970).

Ideally, this guide wouldn't be needed.
In some cases,
there are steps we can take to obsolete a section,
like providing a meaningful unused dependency warning
([#15813](https://github.com/rust-lang/cargo/issues/15813))
rather than suggesting tools that try to guess what dependencies are unused.
In some cases,
builds are slower by default as we try to balance several competing needs.
However, even in those cases, we can evaluate whether we have the right balance or if there is another way to meet multiple needs
(e.g. [#15931](https://github.com/rust-lang/cargo/issues/15931)).
We decided to link out to this content to help raise awareness of these efforts to track them or participate.

Going forward, we are going to need to figure out how to balance what optimizations to include and how to talk about them.
How do we vet that an optimization is actually beneficial?
How much of an improvement is worth mentioning?
How niche or tricky of an optimization is worth including?
We dealt a little bit with this when adding documentation about linkers ([#15991](https://github.com/rust-lang/cargo/pull/15991)) because some platforms already have fast linkers and making linking slightly faster than that is very different than switching from a slow linker to a faster one.

We're tracking further progress on this effort at
[#16119](https://github.com/rust-lang/cargo/issues/16119).

<!--
https://github.com/rust-lang/cargo/pull/16107
https://github.com/rust-lang/cargo/pull/16108
https://github.com/rust-lang/cargo/pull/16085
https://github.com/rust-lang/cargo/pull/16078
https://github.com/rust-lang/cargo/pull/15970
https://github.com/rust-lang/cargo/pull/15924
https://github.com/rust-lang/cargo/pull/15991
-->

### Cargo Script

*Update from [1.86](https://blog.rust-lang.org/inside-rust/2025/02/27/this-development-cycle-in-cargo-1.86/#cargo-script)*

[epage](https://github.com/epage/) posted the [stabilization report](https://github.com/rust-lang/rust/pull/148051)
for the Rust frontmatter syntax,
the first step towards stabilizing Cargo script.
Cargo's frontmatter parser was also updated to better match rustc's include functionality
([#15975](https://github.com/rust-lang/cargo/pull/15975))
and error messages (
[#15952](https://github.com/rust-lang/cargo/pull/15952),
[#15972](https://github.com/rust-lang/cargo/pull/15972)
).

`build-dir` ([docs](https://doc.rust-lang.org/cargo/reference/build-cache.html)),
split out of `target-dir` in Cargo 1.91,
was modeled off of Cargo script but implemented independently.
In [#16073](https://github.com/rust-lang/cargo/pull/16073),
Cargo script switched to using `build-dir = "{cargo-cache-home}/build/{workspace-path-hash}"`
which is proposed to be the new `build-dir` default eventually
([#16147](https://github.com/rust-lang/cargo/issues/16147)).
However, this did lead to issues with `memfd`
([#16110](https://github.com/rust-lang/cargo/issues/16110))
which still needs discussion.
To match the semantics of `build-dir` being for internals,
Cargo script's `Cargo.lock` was moved into `build-dir`
([#16087](https://github.com/rust-lang/cargo/pull/16087)).

In preparing to stabilize Cargo script,
the Cargo team talked through some of the open questions.

<!-- team meeting 2025-09-30 for arg0 -->
In [#12870](https://github.com/rust-lang/cargo/issues/12870),
[novafacing](https://github.com/novafacing) requested a way to get the script's original path.
`CARGO_MANIFEST_PATH` was previously added but didn't meet everyone's needs.
[Nemo157](https://github.com/Nemo157) pointed out that ideally CLI parsers report the script, not the binary, in usage examples.
There isn't really a way for libraries like `clap` to detect and workaround this,
requiring hacks on the user end.
They suggested Cargo override `arg[0]` which is what CLI parsers use for usage examples.
When we discussed this as a team, we were interested in people being able to get both pieces of information, the binary and the source.
We were also concerned about platform support for setting `arg[0]` and [`current_exe`](https://doc.rust-lang.org/std/env/fn.current_exe.html).
Granted, shebang support is also not supported on every platform.
Python and Ruby report `arg[0]` as the script but they have more control over the behavior.
In the end, we decided on setting `arg[0]` and it being a best-effort.
We will leave `current_exe` alone to be the way to access the binary path.
We would be open to people contributing support for more platforms,
likely through contributing it to `std`.
Setting of `arg[0]` was implemented in
[#16027](https://github.com/rust-lang/cargo/pull/16027).

<!-- team meeting 2025-09-30 for manifest fields -->
Cargo scripts do not support every manifest field,
especially for the initial release.
A long standing open question has been whether the manifest fields
should be validated with an allowlist or a denylist.
The concern is if a new field gets added,
should we err on the side of it being supported or not?
Forgetting to update the Cargo script allowlist on the release of a new feature is a poor experience.
On the other hand, forgetting to update the denylist could mean we commit to a feature that we shouldn't support.
The ideal solution is to rely on the type system to ensure we exhausitvely the manifest fields.
If that isn't possible, we erred on the side of an allowlist.
Thankfully, the implementation had already been updated to make it easy to rely on the type system for this.
The validation logic was changed in
[#16026](https://github.com/rust-lang/cargo/pull/16026).

<!-- team meeting 2025-10-07 for escaping package.name -->
A cargo script's file name gets turned into a
[`package.name`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field)
but not every script name is a valid `package.name`.
So far, Cargo has sanitized the file name into being a valid `package.name`.
But valid according to whom?
General Cargo commands,
`cargo new`,
or crates.io?
So far, the `cargo new` rules had been implemented.
This is important to decide upfront because the sanitization results are visible through the binary's name, `cargo metadata`, and `--message-format json`.
As we stepped through each `cargo new` rule,
we found they were becoming less relevant through other efforts in Cargo, changes in Windows, etc.
We decided to do the bare minimum sanitization needed for general Cargo commands.
During the implementation of
[#16120](https://github.com/rust-lang/cargo/pull/16120),
[epage](https://github.com/epage/)
felt it was too premature to freely allow names that match directory names inside the binary's directory.
Users can just now move some of those directories out in Rust 1.91
([#15833](https://github.com/rust-lang/cargo/pull/15833)).
Changing this to be the default in Cargo is still under discussion
([#16147](https://github.com/rust-lang/cargo/issues/16147))
and users could still move it back.
Instead of sanitizing,
epage let this fall back to existing validation rules that will error.

### Public dependencies

*Update from [1.76](https://blog.rust-lang.org/inside-rust/2024/01/03/this-development-cycle-in-cargo-1-76/#rfc-3516-public-private-dependencies)*

While this feature is largely blocked on the lint within rustc,
this was further refined in Cargo.

[jneem](https://github.com/jneem) experimented with Cargo rewriting the lint to provide Cargo-specific context in [#16002](https://github.com/rust-lang/cargo/pull/16002).

[sadmac7000](https://github.com/sadmac7000) changed `cargo add`s version auto-selection to evaluate public dependencies in case the user intends to use them together
([#15966](https://github.com/rust-lang/cargo/pull/15966)).

<!-- team meeting 2025-10-07 -->
[JohnScience](https://github.com/JohnScience) proposed `cargo tree --edges no-external` as a way to see only local packages
([#16043](https://github.com/rust-lang/cargo/issues/16043)).
We have this today in `--depth workspace` though maybe we could improve parts of our documentation about this.
However, this got us to re-evaluate `--depth public` which walks through all public public dependencies and no further
(inspired by `--depth workspace`).
Would this be better served as `--edges public`?
The flag was originally added to help in analysing the lints current behavior
([rust#119428](https://github.com/rust-lang/rust/issues/119428#issuecomment-2686384070)).
Most `--edges` opt-in specific edge types,
while this would instead be applying a filter across edge types.
The only other exception is `no-proc-macros`.
We decided that we were comfortable adding more edge filters and decided to change this ([#16081](https://github.com/rust-lang/cargo/pull/16081)).

### Build-dir layout

*Update from [1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/#misc)*

Cargo's caches have traditionally been organized around the role they fulfil
with `.fingerprint/` housing the state for rebuild-detection for all packages
while `deps/` stores the build artifacts.
This makes calling rustc easy, just pass it `deps/` and it will figure out what files need to be loaded.

By mixing intermediate artifacts together like this,
- if we were to GC the content, we'd need to track individual files for a build unit ([#5026](https://github.com/rust-lang/cargo/issues/5026))
- it is difficult to coordinate more granular locks ([#4282](https://github.com/rust-lang/cargo/issues/4282))
- it is more difficult to cache build unit artifacts across projects ([#5931](https://github.com/rust-lang/cargo/issues/5931)).
- requires Cargo to make the file names unique (except on Windows) ([#8332](https://github.com/rust-lang/cargo/issues/8332))
  - and file collisions on Windows ([#8794](https://github.com/rust-lang/cargo/issues/8794))
- leads to bugs where project binaries can shadow system or Rust toolchain binaries on Windows because we have to put `deps/` in `PATH` for linking ([#7919](https://github.com/rust-lang/cargo/issues/7919))

The layout for intermediate build artifacts is an implementation detail which we can change.
[#15010](https://github.com/rust-lang/cargo/issues/15010)
proposes changing the layout to be centered on the build unit the files belong to,
rather than the role of the files.
We have a single folder to track for GC, locking, and caching.
A unique hash will be kept in the parent directory's name,
allowing us to reduce collisions of files and shadowing of binaries on Windows.
This new layout was implemented in [#15947](https://github.com/rust-lang/cargo/pull/15947).

There is a catch: many tools in the ecosystem depend on the layout.
The reason [ranger-ross](https://github.com/ranger-ross) added support for the new 
[`build-dir`](https://doc.rust-lang.org/cargo/reference/build-cache.html)
was to serve as an easy for projects to test if they rely on internals of Cargo.

We can punt on finding alternative solutions to these projects,
but that means each time we change the layout of the `build-dir`,
there is an ecosystem cost.
Turns out, we might want to change it multiple times.
The `build-dir` is subdivided by `<profile>/<platform>/`
but that is mostly beneficial for locking purposes.
If we had a new locking scheme
([#4282](https://github.com/rust-lang/cargo/issues/4282)),
we could reduce path lengths on Windows
and allow intermediate artifact reuse between profiles and even platforms (e.g. build script builds).
As I said earlier, the locking scheme is also blocked on the new layout.
We either have to do implement and stabilize them together or have two transitions.
It doesn't stop there.
A new locking scheme may be benefited by us moving away from mutable intermediate artifacts
which could balloon disk usage as each build for each edit of your source would have a distinct artifact.
This would be benefitted by aggressive GC of the intermediate artifacts
which is also blocked on the new layout.

<!-- team meeting 2025-10-21 -->
As a team, we discussed this tricky path towards stabilization of the new layout.

After talking through the interaction between these different features,
we leaned towards doing one layout change without blocking on any other work and evaluating how that goes to see how we should handle further layout changes.

It would be great if [crater](https://rustc-dev-guide.rust-lang.org/tests/crater.html)
could identify projects impacted by changing the layout.
It may not help us when it is a build process extracting `build.rs` generated artifacts or when running the tool being built.
There may be some `-sys` crate situations it might identify.
Later,
[ehuss](https://github.com/ehuss/)
posted on 
[Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build-dir.20tool.20compatibility/near/546267015)
some preliminary investigations into what projects might be doing relying on the `build-dir` layout.
In addition to this type of inspection,
we could change the layout on nightly-only to help identify impacted projects.

We are using `build-dir` as an opt-in for people to evaluate both changing it itself and as a smoke test for a new layout.
Even once we change the `build-dir` location
([#16147](https://github.com/rust-lang/cargo/issues/16147)),
users will be able to opt-out.
Should we do similar for the new layout itself?
If we made the flag a proper config,
this would give the `build-dir` layout more of a semblance of stability than is meant.
This is also a maintenance burden.
Supporting the two layouts already complicates things and has limited our changes to the new layout.
Supporting the old layout for any period of time will likely require all features built on top of it to be conditioned on it until we are able to drop the old layout.
A temporary environment variable to toggle the behavior may work.

At this point,
it is on
[epage](https://github.com/epage/) and
[ranger-ross](https://github.com/ranger-ross)
to come up with a concrete transition plan.

## Misc

- [epage](https://github.com/epage/) continued their work on migrating existing Cargo messages to annotate-snippets ([#15944](https://github.com/rust-lang/cargo/issues/15944)) in
[#15942](https://github.com/rust-lang/cargo/pull/15942),
[#15943](https://github.com/rust-lang/cargo/pull/15943),
[#15945](https://github.com/rust-lang/cargo/pull/15945).
[jneem](https://github.com/jneem) joined in,
posting
[#16035](https://github.com/rust-lang/cargo/pull/16035),
[#16066](https://github.com/rust-lang/cargo/pull/16066),
[#16113](https://github.com/rust-lang/cargo/pull/16113),
[#16126](https://github.com/rust-lang/cargo/pull/16126) (*Update from [1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/#annotate-snippets)*).
- [weihanglo](https://github.com/weihanglo) posted initial support for structured, persistent logging for Cargo for performing build analysis ([#16150](https://github.com/rust-lang/cargo/pull/16150))
- [weihanglo](https://github.com/weihanglo) cleaned up the config logic in preparation for [config-include](https://doc.rust-lang.org/cargo/reference/unstable.html#config-include) to have an `optional` key for not erroring on not-present configs.
- [Muscraft](https://github.com/Muscraft) proposed adopting `typos` for spell checking Cargo ([#16122](https://github.com/rust-lang/cargo/pull/16122))

<!-- team meeting 2025-10-21 -->
<!-- config-include
https://github.com/rust-lang/cargo/pull/16109
https://github.com/rust-lang/cargo/pull/16105
https://github.com/rust-lang/cargo/pull/16094
https://github.com/rust-lang/cargo/pull/16100
https://github.com/rust-lang/cargo/pull/16067
https://github.com/rust-lang/cargo/pull/16084
https://github.com/rust-lang/cargo/pull/16004
-->

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

<!--
Project goals in need of owners
- [Prototype Cargo build analysis](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-analysis.html)
- [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h2/pub-priv.html)
- [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)
- [Rework Cargo Build Dir Layout](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html)
- [Prototype a new set of Cargo "plumbing" commands](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html)
- [Finish the libtest json output experiment](https://rust-lang.github.io/rust-project-goals/2025h2/libtest-json.html)
-->

Ready-to-develop:
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
<!--
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
-->

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
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)

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
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo)
and you can talk to us in real-time during
[Contributor Office Hours](https://github.com/rust-lang/cargo/wiki/Office-Hours).
If you are looking to help with one of the bigger projects mentioned here and are just starting out,
[fixing some issues](https://doc.crates.io/contrib/process/index.html#working-on-issues)
will help familiarize yourself with the process and expectations,
making things go more smoothly.
If you'd like to tackle something
[without a mentor](https://doc.crates.io/contrib/issues.html#issue-status-labels),
the expectations will be higher on what you'll need to do on your own.
