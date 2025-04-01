+++
path = "inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78"
title = "This Development-cycle in Cargo: 1.78"
authors = ["Ed Page"]
aliases = ["inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.78

We wanted to share what has been happening for the last 6 weeks to better keep the community informed and involved.
For work that was merged before the beta branch was made at the end of the cycle, it will be in the Beta channel for the next 6 weeks after which it will be generally available.

This is distinct from [This Week in Rust](https://this-week-in-rust.org/) in that it tries to focus more on the big picture, rather than individual PRs, and pulls from more sources, like Cargo Team meetings and [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo).

This is an experiment in finding better ways to be engaged with the community and we'll see how well it works and how well we can keep up on it.

<!-- time period: 2024-02-09 through 2024-03-21 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [Terminal styling](#terminal-styling)
  - [User-controlled diagnostics](#user-controlled-cargo-diagnostics)
  - [Performance](#performance)
  - [MSRV-aware Cargo](#msrv-aware-cargo)
  - [Registry authentication](#registry-authentication)
  - [Git extensions](#git-extensions)
  - [Garbage collection](#garbage-collection)
  - [Default Edition](#default-edition)
  - [Open namespaces](#open-namespaces)
- [Design discussions](#design-discussions)
  - [Deprecated `Cargo.toml` fields](#deprecated-cargotoml-fields)
  - [RFC #3452: Nested packages](#rfc-3452-nested-packages)
  - [Why is this yanked?](#why-is-this-yanked)
  - [Weak feature syntax](#weak-feature-syntax)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [cargo-sweep](https://crates.io/crates/cargo-sweep) which removes unused build files.
See also [cargo-cache](https://crates.io/crates/cargo-cache).
For a related work inside of Cargo,
see [#12633](https://github.com/rust-lang/cargo/issues/12633).

Thanks to [LukeMathWalker](https://github.com/LukeMathWalker) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

##### Terminal styling

<!-- team meeting: 2024-02-20 -->

While Cargo has UI tests, they have not verified the terminal styling, like colors.
Rustc manages this by writing the ANSI escape codes to text files which are hard to visualize outside of `cat stdout.log`.
In [#13461](https://github.com/rust-lang/cargo/pull/13461),
[epage](https://github.com/epage) ported Cargo's UI snapshots from text to SVG, allowing terminal styling to be captured.
To accomplish this, they created [`anstyle-svg`](https://docs.rs/anstyle-svg/latest/anstyle_svg/)
to render ANSI escape codes as styles in an SVG
(credit goes to [`term-transcript` for the original idea](https://crates.io/crates/term-transcript))
and integrated that into snapbox
([trycmd#256](https://github.com/assert-rs/trycmd/pull/256))
which we use for snapshoting our UI tests.

![rendering of cargo-add's output using SVG](../../../../images/inside-rust/2024-03-26-this-development-cycle-in-cargo-1.78/stderr.term.svg)
*(not a screenshot but generated from cargo's output)*

While this verified most of Cargo's terminal styling, we couldn't force styling on within `--help` to snapshot it.
While we added styling to `--help` in
[#12578](https://github.com/rust-lang/cargo/pull/12578),
we overlooked this being controlled by
[term.color](https://doc.rust-lang.org/cargo/reference/config.html#termcolor)
as this all happens before the config is initialized.
In [#13463](https://github.com/rust-lang/cargo/pull/13463),
we refactored Cargo's initialization so at least some config is available before parsing command-line arguments,
allowing `--help` to be controlled by config.
This still leaves `cargo --color=never --help` as unsupported ([#9012](https://github.com/rust-lang/cargo/issues/9012)).

In reviewing the SVG snapshots, we identified some CLI help output that was overlooked in [#12578](https://github.com/rust-lang/cargo/pull/12578)
and addressed it in [#13479](https://github.com/rust-lang/cargo/pull/13479)

Since then,
rustc (thanks to [estebank](https://github.com/estebank) in [rust#121877](https://github.com/rust-lang/rust/pull/121877))
and annotate-snippets (thanks to [Muscraft](https://github.com/Muscraft) in [annotate-snippets-rs#86](https://github.com/rust-lang/annotate-snippets-rs/pull/86))
have adopted SVG snapshot testing of terminal styling

##### User-controlled cargo diagnostics

*[Update from 1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#user-controlled-cargo-diagnostics). In summary, this aims to add [user-controlled lints](https://github.com/rust-lang/cargo/issues/12235) that look like rustc and are controlled through the [`[lints]` table](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section)*

One problem we had with the SVG snapshot tests was with annotate-snippets,
the rustc-like diagnostic renderer that Cargo is using.
Rustc, and by extension annotate-snippets, specializes the colors for each platform for maximum compatibility with the [default colors used by each platform's most common terminals](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit).
To workaround this, we had to put in snapshot wildcards in place of the style names,
making the SVGs render different than what you'd get on the terminal.
Muscraft added the `testing-colors` feature to `annotate-snippets` to force consistent colors across platforms for testing
([annotate-snippets-rs#82](https://github.com/rust-lang/annotate-snippets-rs/pull/82)),
allowing us to have the SVGs better match the terminal while working on all platforms.

In preparation to shift our focus from `annotate-snippets` to Cargo's diagnostic system,
we reviewed Cargo's code for generating messages for TOML parsing errors for any cleanup we should first apply to Cargo and/or `annotate-snippets`.
`annotate-snippets` requires callers to deal with columns but that is a UX concern that is dependent on the medium you are rendering to so Muscraft shifted the API to focus on byte indices
([annotate-snippets-rs#90](https://github.com/rust-lang/annotate-snippets-rs/pull/90)).
There is still a lot of complexity left to extract the lines for the message and translating the document-relative spans to be line-relative.
We had wondered if we could use `annotate-snippets`'s "`fold` unannotated lines" mechanism to pass in the entire file and let `annotate-snippets` do it for us.
There was some inconsistency in how it folded the start and end of the file so in [annotate-snippets-rs#109](https://github.com/rust-lang/annotate-snippets-rs/pull/109),
we erred on the side that made it easy for callers like Cargo.
In removing the line extraction from Cargo, we found that there was a hack in Cargo for how `annotate-snippets` highlights EOF and so we merged [annotate-snippets-rs#107](https://github.com/rust-lang/annotate-snippets-rs/pull/107).

Muscraft was going to focus on Cargo's adoption of `annotate-snippets` before looking to rustc's.
However, some people are discussing working on rustc for GSoC 
([zulip](https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc/topic/Idea.3A.20extend.20annotate-snippets)).
In the hope to keep breaking changes down,
epage re-examined the API with an eye towards rustc and how to allow it to evolve for anything we missed (mainly by using the builder pattern).
See [annotate-snippets-rs#94](https://github.com/rust-lang/annotate-snippets-rs/pull/94).
We also found some implementation details being exposed in the API that we had overlooked when we previously abstracted them away
([annotate-snippets-rs#67](https://github.com/rust-lang/annotate-snippets-rs/pull/67))
which Muscraft fixed in [annotate-snippets-rs#105](https://github.com/rust-lang/annotate-snippets-rs/pull/105).

To see how these changes simplify the caller, see
- [#13609](https://github.com/rust-lang/cargo/pull/13609)
- [#13619](https://github.com/rust-lang/cargo/pull/13619)

`annotate-snippets` was first introduced into Cargo for rendering TOML errors.
This was straight forward to implement because `toml` exposes [byte spans on `Error`](https://docs.rs/toml/latest/toml/de/struct.Error.html#method.span).
For lints, we were going to need to look up spans for arbitrary keys and values on the document.
`toml` exposes spans during deserialization but this has some impedance mismatches with serde and requires us to explicit track and forward throughout cargo any spans we care about.
As an alternative, we were planning to rely on a truly terribly great [serde hack](https://play.rust-lang.org/?version=stable&edition=2021&gist=0d457da235449046bd30932a91e45d96)
that [dtolnay](https://github.com/dtolnay)
[pointed out](https://github.com/toml-rs/toml/issues/571#issuecomment-1782050097)
despite the performance overhead of re-parsing the TOML to look up each span.
When considering how to improve the performance,
epage came up with an API design for `toml_edit` to allow looking up the span for a node in a document which was implemented in
[toml-rs#698](https://github.com/toml-rs/toml/pull/698).
To ensure this information is available for where lints will be added,
we flattened the code for parsing manifests
([#13589](https://github.com/rust-lang/cargo/pull/13589))
so we could attach the source and spans to the data structures used throughout cargo
([#13593](https://github.com/rust-lang/cargo/pull/13593)).

With these building blocks in place, we are ready to start on Cargo's diagnostic system.

As an aside, in the hopes that we can one day use fancier unicode characters in diagnostics (and progress updates), we've generalized `cargo tree --charset` into the config [`term.unicode`](https://doc.rust-lang.org/nightly/cargo/reference/config.html#termunicode) in [#13337](https://github.com/rust-lang/cargo/pull/13337).

##### Performance

At the tail end of the 1.78 development cycle,
[davidlattimore](https://github.com/davidlattimore/)
posted on
[Speeding up the Rust edit-build-run cycle](https://davidlattimore.github.io/posts/2024/02/04/speeding-up-the-rust-edit-build-run-cycle.html).
This got epage curious about where Cargo's time is going and wanting to make it easier to give users insight into that.
Cargo has [`--timings`](https://doc.rust-lang.org/cargo/reference/timings.html?highlight=timings#reporting-build-timings)
but that doesn't include Cargo's overhead.
There was also a `CARGO_PROFILE` environment variable to cause Cargo to capture and dump a couple of specific stages.
Inspired by [git-branchless](https://github.com/arxanas/git-branchless),
epage decided to experiment with support for
[tracing-chrome](https://crates.io/crates/tracing-chrome)
in Cargo which was merged in
[#13399](https://github.com/rust-lang/cargo/pull/13399)
behind the 
[`CARGO_LOG_PROFILE` environment variable](https://doc.crates.io/contrib/tests/profiling.html).

![rendering of traces for building cargo](../../../../images/inside-rust/2024-03-26-this-development-cycle-in-cargo-1.78/cargo-profile.png)
*(rendering of traces for building `cargo`)*

epage tried this out on
[cargo-nextest](https://crates.io/crates/cargo-nextest)
and took notes on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Chrome.20tracing.20for.20cargo/near/424965726).
Its important to note that Cargo's overhead is either in small fixed costs per run or even smaller per-package costs.
These will likely be dwarfed by Rustc (if there are situations you know of otherwise, let us know on that
[zulip thread](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Chrome.20tracing.20for.20cargo/near/424965726)!).
Because of this, epage is mostly focusing on the [cargo script](https://github.com/rust-lang/rfcs/pull/3502) use case,
especially since the third-party predecessors went through the trouble of 
[implementing their own caching scheme on top of Cargo](https://github.com/fornwall/rust-script/blob/fb4e6276ae15c338e075d56fe97fd1090fe9c368/src/main.rs#L386-L423)
to avoid Cargo's overhead.

The single longest operation is related to
[git2](https://crates.io/crates/git2).
Since there is active work on replacing it with
[gitoxide](https://crates.io/crates/gix)
([progress report](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.60gitoxide.60.20integration.20updates/near/420960494)), 
we lean towards punting on this rather than adding complexity and risk by deferring that initialization work.

Another major source of overhead is in parsing dependencies, particularly:
1. Parsing `Cargo.toml` files
2. Enumerating inferred build targets (particularly tests)
3. Linting inferred build targets (particularly tests)

Building on the refactor from
[User-controlled diagnostics](#user-controlled-cargo-diagnostics)
for accessing spans, epage is working on explicitly enumerating inferred build targets in the published `Cargo.toml` for a package.
In addition to removing the overhead from inferring targets,
this will improve errors for maintainers
([#13456](https://github.com/rust-lang/cargo/issues/13456))
and make it easier for crates.io to add more features to their frontend
(e.g. [crates.io#5882](https://github.com/rust-lang/crates.io/issues/5882) 
and [crates.io#814](https://github.com/rust-lang/crates.io/issues/814)).

We hope to be able to build on that work to defer lints out of manifest parsing, allowing us to skip the lint analysis when its for a dependency
(thanks to [cap-lints](https://doc.rust-lang.org/rustc/lints/levels.html#capping-lints)).

##### MSRV-aware Cargo

*[Update from 1.77](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#rfc-3537-make-cargo-respect-minimum-supported-rust-version-msrv-when-selecting-dependencies)*

<!-- team meeting: 2024-02-27 -->
[RFC #3537](https://github.com/rust-lang/rfcs/pull/3537) went through
[FCP](https://github.com/rust-lang/rfcs/pull/3537#issuecomment-1946381890)
at the start of this development cycle.
This was a much debated RFC with many, widely different opinions on where the RFC should go.
To help work through this debate, we held extended
[Office Hours](https://github.com/rust-lang/cargo/wiki/Office-Hours)
to allow higher-throughput communication on this topic.
In the end, the Cargo team felt we should move forward with the RFC as-is.
The Cargo team [posted](https://github.com/rust-lang/rfcs/pull/3537#issuecomment-1968172897):

> Thank you everyone for your feedback!
> 
> Your participation has helped us gain a better understanding of the different ways people use Cargo and what people's needs are. We recognize that there are a lot of competing opinions on how to meet user needs.
> 
> Whichever way we go, there comes a point where we need to move forward. However, it is important to remember that RFCs are not a final specification. This RFC in particular will be stabilized a piece at a time (with `cargo new` changes likely made last). In preparing to stabilize a feature, we will take into account changes in the ecosystem and feedback from testing unstable features. Based on that evaluation, we may make changes from what this RFC says. Whether we make changes or not, stabilization will then require approval of the cargo team to merge (explicit acknowledgement from all but 2 members with no concerns from any member) followed by a 10 days Final Comment Period (FCP) for the remaining 2 team members and the wider community. Cargo FCPs are now tracked in This Week in Rust to ensure the community is aware when this happens and can participate. Even then, a change like what is proposed for `cargo new` can be reverted without an RFC, likely only needing to follow the FCP process.

Soon after, epage followed up by fleshing out `cargo add`'s auto-selection of version requirements so it could be stabilized in [#13608](https://github.com/rust-lang/cargo/pull/13608)
- [#13516](https://github.com/rust-lang/cargo/pull/13516) added a fallback to `rustc -V` when `package.rust-version` is not set
- [#13537](https://github.com/rust-lang/cargo/pull/13537) fixed inconsistencies with how we compare Rust versions, reducing the risk for bugs

<!-- team meeting: 2024-03-12 -->
A first step with the resolver work is helping users know that a dependency has been held back.
This isn't just an MSRV-aware resolver problem but a SemVer-aware resolver problem.
Being cautious about overwhelming users with information,
epage broke this out into a separate issue
([#13539](https://github.com/rust-lang/cargo/issues/13539))
for a more focused conversation and started a discussion on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/How.20to.20report.20.22held.20back.22.20dependencies.20from.20MSRV.20resolver).
In talking about this in a Cargo team meeting,
we decided to move forward and this was merged in [#13561](https://github.com/rust-lang/cargo/pull/13561).

The next area of potential bike shedding is how to organize and name the config fields for controlling the resolver.
This is being tracked in [#13540](https://github.com/rust-lang/cargo/issues/13540).

##### Registry Authentication

When [support for alternative forms of registry authentication](https://doc.rust-lang.org/cargo/reference/registry-authentication.html)
was added, the default of plain-text credential storage was not carried over to alternative registries.
This discrepancy was confusing to at least one user
([#13343](https://github.com/rust-lang/cargo/issues/13343)).
In reflecting on this, it seems appropriate to deprecate implicit use of `cargo:token` built-in credential provider.
Users could suppress the deprecation warning by opting in explicitly.

<!-- team meeting: 2024-03-05 -->
In preparing to deprecate this, epage decided to dog food the documentation for credential providers.
The first thing is the documentation recommends credential providers based on the users platform.
Having a machine-agnostic config is a lot easier for users to maintain,
so epage tried merging all of the entries, relying on each provider declaring itself as unsupported when unavailable (like `cargo:wincred` on non-Windows platforms).
However, `cargo:libsecret` will error, rather than be skipped, if `libsecret` is not installed.
After some discussion on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/reg.20auth.20and.20libsecret)
and in a team meeting, [#13558](https://github.com/rust-lang/cargo/pull/13558) was created.

##### Git extensions

<!-- team meeting: 2024-03-05 --> 
[arlosi](https://github.com/arlosi) brought up in a meeting that they can't build with Cargo if its in a git repo that uses features unsupported by libgit2.
In this specific case, the problem is [Split Index](https://github.com/rust-lang/cargo/issues/10150).
In particular, this is causing problems with vendoring packages with build scripts because the 
[default behavior for build scripts is to re-run if any source has changed unless `cargo::rerun-if-changed` is emitted](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed).
They are currently working around this by modifying vendored packages to have a `package.include` field which disables Cargo's git walking.

This will also affect `cargo package`.
In discussing this, another scenario that can come up is any `cargo doc` invocation because `rustdoc`, unlike `rustc`, doesn't tell `cargo doc` what files were looked at, so `cargo doc` has to guess.

One option is to walk the directory manually using the [`ignore`](https://crates.io/crates/ignore) package.
However, this isn't just about respecting `.gitignore` but this also checks the stage.

That left us with:
- Switch the directory scanning to [gitoxide](https://crates.io/crates/gix) as that supports Split Index
- Wrap the `git` CLI and either fallback implicitly or create a config much like [`net.git-fetch-with-cli`](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli) which would not just support Split Index but any git extension not currently supported by a re-implementation like libgit2 or gitoxide.
- Attempt to phase out the implicit "scan all" in build scripts, limiting the fix to just this specific use case.  This would be done with a new Edition.  We've been hesitant to change build scripts with Editions because a lot of times they rely on a library to emit the instructions which can be on a different Edition.

[Byron](https://github.com/Byron) stepped in and provided a gitoxide implementation in [#13592](https://github.com/rust-lang/cargo/pull/13592).
Discussions are on-going for stabilizing this work on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/.60gitoxide.60.20integration.20updates/near/428383923).

##### Garbage collection

We're working on automatic cleanup of on-disk caches.
Initially, we are starting with global state.
This effort is being tracked in [#12633](https://github.com/rust-lang/cargo/issues/12633).

<!-- team meeting: 2024-02-27 -->
As a small step forward for,
[ehuss](https://github.com/ehuss) proposed we stabilize global cache tracking in
[#13492](https://github.com/rust-lang/cargo/pull/13492).
This will ensure your machine has the historical data it needs to determine what caches to garbage collect once we stabilize that part of this.

##### Default Edition

[kpreid](https://github.com/kpreid) proposed we deprecate relying on default Editions on [Internals](https://internals.rust-lang.org/t/idea-rustc-cargo-should-warn-on-unspecified-edition/20309).
Today, if you create a `Cargo.toml` without setting [package.edition](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field),
Cargo will default to the 2015 Edition.
The same is true if you directly run `rustc` without passing `--edition` which people do for "quick experiments".
Similarly, some people don't realize that `rustfmt` is more like `rustc`, needing the `--edition` flag, when they likely need `cargo fmt` to respect their `Cargo.toml` edition.

If we deprecated relying on the default Edition, it would likely reduce user confusion.
This also would help with [RFC #3502: cargo script](https://github.com/rust-lang/rfcs/pull/3502) because that defines the default for embedded manifest differently: use the current edition but warn.
Having both warn and users being used to explicitly setting the Edition will help gloss over the difference in their defaults.

<!-- team meeting: 2024-02-20 -->
The Cargo team discussed this and was in favor of moving forward and merged this in [#13505](https://github.com/rust-lang/cargo/pull/13505).

While it might be reasonable for the Compiler team to come to a different conclusion,
we didn't want Cargo omitting `--edition` when it calls `rustc` to block them, so we made sure we always pass it in [#13499](https://github.com/rust-lang/cargo/pull/13499).

Sometimes it can be easy to overlook why an existing project is slower to evolve compared to new projects.
One challenge is the weight of the existing features.
In this case, it was the tests for those features.
To get an idea of what that weight is,
consider the manual test updates done in
[#13504](https://github.com/rust-lang/cargo/pull/13504) to unblock this work.

##### Open namespaces

Recently, [RFC #3243](https://github.com/rust-lang/rfcs/pull/3243) was approved which is a major shift in Rust.
Previously, library namespaces were closed to extension.
With this RFC, we are moving closer to Python which allows restricted extension of a library's namespace.
You will be able to name a package `foo::bar`,
making your package be part of the `foo` namespace.
A major restriction on this is that crates.io will put the owners of `foo` in control of who can publish `foo::*` packages.
This will be useful for projects like Clap, Bevy, or Gitoxide that have a large collection of libraries with independent versioning that act as a cohesive whole.
Technically, this could be used as registry namespacing (naming all packages `my-org::*`) but they will likely run into impedance mismatches as this feature was not design for that use case.

As a first step,
epage implemented rudimentary support this in Cargo in [#13591](https://github.com/rust-lang/cargo/pull/13591).
You can run `cargo metadata` but `cargo check` will fail.
Discussions on the cargo/compiler interactions are happening in the
[rustc tracking issue](https://github.com/rust-lang/rust/issues/122349).
The unstable feature was named [open-namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces) with the hope to be more semantically specific to reduce people inadverently thinking this was registry namespacing.

## Design discussions

##### Deprecated `Cargo.toml` fields

<!-- team meeting: 2024-03-19 -->

In reviewing a PR, epage observed that the contributor accessed 

[`manifest.dev_dependencies`](https://docs.rs/cargo-util-schemas/latest/cargo_util_schemas/manifest/struct.TomlManifest.html#structfield.dev_dependencies)
(for `[dev-dependencies]`),
overlooking [`manifest.dev_dependencies2`](https://docs.rs/cargo-util-schemas/latest/cargo_util_schemas/manifest/struct.TomlManifest.html#structfield.dev_dependencies2)
(for `[dev_dependencies]`).
Considering the obvious name of the `manifest.dev_dependencies` field and lack of awareness of `[dev_dependencies]` (not even the other `Cargo.toml` parsers surveyed support it),
this was understandable.

The reminder that these fields exist led to a discussion within the Cargo team of what we should do about them.

A quick overview:

| Expected             | Alt                  | If alt used | If both used |
|----------------------|----------------------|-------------|--------------|
| `package`            | `project`            | deprecated, planned removal | warn        |
| `build-dependencies` | `build_dependencies` | nothing | warn and say alt is deprecated |
| `dev-dependencies`   | `dev_dependencies`   | nothing | warn and say alt is deprecated |
| `proc-macro`         | `proc_macro`         | nothing | warn and say alt is deprecated |
| `crate-type`         | `crate_type`         | nothing | warn and say alt is deprecated |

Our plan is to research the use of all of our deprecated functionality, including
- When it was introduced?
- When it was superseded?
- How common is the use on crates.io?
- How common the use is within the ecosystem (Cargo may normalize some of this on publish)?

Our options include:
- Warn that it is deprecated but keep it
- Warn that it is deprecated on existing Editions and disallow it on future Editions
  - As most alternatives date far enough back, we are assuming we don't need to restrict the warning based on a package's declared minimum-supported Rust version (MSRV)
- Warn and once a sufficient amount of time has passed, remove the functionality (restricted for only what we consider to be outside our compatibility guarantees like when we removed support for parsing invalid manifests in [#9932](https://github.com/rust-lang/cargo/pull/9932))

This is being tracked in
[#13629](https://github.com/rust-lang/cargo/issues/13629)
and discussed on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Next.20step.20for.20deprecations.20in.20Cargo/near/428407231).

##### RFC #3452: Nested packages

[RFC #3452](https://github.com/rust-lang/rfcs/pull/3452)
would allow `cargo publish` to bundle select
[path dependencies](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#specifying-path-dependencies)
within a package's published `.crate` file.
This could remove the need for publishing two packages for proc-macros or allow splitting up a larger package into smaller compilation units for faster incremental rebuilds.
A similar idea was posted as [RFC #2224](https://github.com/rust-lang/rfcs/pull/2224) in 2017 but it was postponed.
In 2022, [yoshuawuyts](https://github.com/yoshuawuyts) approached this problem from the language side in their post [Inline Crates](https://blog.yoshuawuyts.com/inline-crates/).

kpreid worked through the remaining feedback on their RFC.
Threads were opened with
[T-cargo](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/RFC.20.233452.3A.20Nested.20Cargo.20packages/near/427540788)
and [T-crates-io](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/RFC.20.233452.3A.20Nested.20Cargo.20packages/near/427541267)
in the hopes to uncover additional fundamental areas that need addressing in the lead up for an FCP.

<!-- team meeting: 2024-03-19 -->

The Cargo team had a high level discussion on RFC #3452 to gauge general interest for moving forward with this.

One concern raised was the complexity in documenting this,
especially when giving users guidance on when to use a build targets, packages, nested packages, or workspaces
(see also [When to use packages or workspaces?](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#when-to-use-packages-or-workspaces)).

There is also the potential for unintended side effects.
If we don't restrict what dependencies can be nested,
it could make supply chain traceability more difficult, like with [SBOMS](https://github.com/rust-lang/rfcs/pull/3553),
and could make working around problems with dependencies the happy path, rather than encouraging people to keep the quality of the ecosystem high.

##### Why is this yanked?

There has long been a request for allowing a message to be included when running `cargo yank`
([#2608](https://github.com/rust-lang/cargo/issues/2608)).
This could become more important as we allow yanked packages to be used in more places
(see [`cargo update --precise <yanked>`](https://blog.rust-lang.org/inside-rust/2024/02/13/this-development-cycle-in-cargo-1-77.html#cargo-update---precise-yanked) from 1.77).

<!-- team meeting: 2024-03-05 -->
[hi-rustin](https://github.com/hi-rustin/cargo-information)
brought this up in a crates.io team meeting.
It turns out that they are considering something similar for their admin management feature.
So how should Cargo get and report this information?

The first tool to reach for when getting information from crates.io is the 
[Index](https://doc.rust-lang.org/cargo/reference/registry-index.html)
which we use for dependency resolution.
We also have a well-paved path for extending Cargo's registry support in this way without negatively impacting third-party registries.
However, we normally restrict the Index to content needed for dependency resolution.
This is mostly done for performance / disk space reasons.
With the Git Index, you have to download the entire thing.
This is improved with the Sparse Index, where you download only the packages being considered but its still all versions.
We then have to parse these entries to find the relevant versions.

Creating an additional database for this side-band, more mutable metadata,
would require more upfront work but this might offer us other benefits.
Some other ways we could use this database include:
- Unmaintained status (overlaps with rustsec)
- Deprecation status ([crates.io#7146](https://github.com/rust-lang/crates.io/issues/7146)), especially if you can point to a replacement (like rustsec's "unmaintained"), e.g. helping `structopt` users discover that their upgrade path is switching to `clap`, similar for `rlua` to `mlua`
- Prepare for broken builds due to bug-compatibility hacks being removed ([rust#106060](https://github.com/rust-lang/rust/pull/106060))
- Maybe even allow third-party registries to distribute rules for [dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)

For now, we were leaning towards `cargo yank` being able to provide this information to a registry and crates.io storing this and reporting it to users.
Later on, we can explore how we'd want Cargo to consume this information.
At that time, we can backfill whatever database Cargo uses with crates.io's database.

##### Linter for Cargo

Last year on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Cargo.20Lints/near/421280492),
we discussed where Cargo lints should live,
whether all in cargo and run as part of every command or if some should live in a dedicated linter command.
One idea that came up was for some of these lints to live in `cargo clippy`, 
specifically the cargo subcommand and not `clippy-driver` which is where all clippy lints live today
(including some [cargo ones](https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo)).

<!-- team meeting: 2024-02-13 -->
This came up again at the start of 1.78's development when a contributor was looking to implement another Cargo lint in clippy ([clippy#10306](https://github.com/rust-lang/rust-clippy/issues/10306)).
As discussed on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Adding.20more.20information.20to.20.60cargo.20metadata.60/near/419342414),
one of the challenges was in getting access to the information the lint needed.
`cargo metadata` isn't really meant for exposing these lower level details so this would require re-implementing parts of Cargo in `clippy-driver`.
The existence of [`cargo-util-schema`](https://docs.rs/cargo-util-schemas) helps but doesn't alleviate all of the problem.
If the lint could be implemented inside of `cargo clippy` and either `cargo clippy` depended on `cargo` as a library or was baked into Cargo then it would have access to all of the existing machinery, making it easier to keep up-to-date as Cargo evolves.

For lists of potential lints, without consideration for whether they'd live in cargo or an explicit lint command, see
- [clippy's cargo group](https://rust-lang.github.io/rust-clippy/master/index.html#?groups=cargo)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [lints blocked on the diagnostic work](https://github.com/rust-lang/cargo/issues/12235)

Baking `cargo-clippy` directly into `cargo` came up when clippy went out of "preview" and was rejected by the Cargo team at that time
(from what people remember).
Besides having to define the semantics for when `clippy-driver` isn't installed,
the cargo team would be taking ownership of another team's command
and has us doing less dog-fooding of first-class, complex external subcommands.

There is also the question of why a lint should run every time vs be in an explicit lint action.
As discussed in [Performance](#performance),
there can be a noticeable overhead to lint analysis.
This also offers a nursery for lints and the opportunity to be more opinionated by default.

Digging into the
[rustc dev guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html)
and the [clippy book](https://doc.rust-lang.org/nightly/clippy/index.html),
provided a lot of useful information for this discussion and as we add lints to cargo, even if the "why" isn't always explicitly laid out.
In particular, there is the guidance on
[rustc lints, clippy lints, and transition clippy lints to rustc lints](https://github.com/rust-lang/rfcs/blob/master/text/2476-clippy-uno.md#compiler-uplift).

We still need to get more background from the clippy team before we can continue our discussions on where things belong.

##### Weak feature syntax

[RFC #3491](https://github.com/rust-lang/rfcs/pull/3491) plans to transition out implicit features in the next Edition.
Another feature change that has been proposed in [#10556](https://github.com/rust-lang/cargo/issues/10556) was to transition out the weak dependency syntax (`dep?/feature`) by making `dep/feature` always be weak.
This was recently discussed on [zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Weak.20features.20syntax).

When you want a feature to activate a dependency's feature, you use `dep/feature` syntax.
If the dependency is also optional,
this will activate the dependency as well.
The weak feature syntax (`dep?/feature`) allows you to only activate the feature *if* the dependency is activated another way.
A common use case for this is if you have a `serde` feature and you want to enable `serde` features in your optional dependencies.
To put this another way, `"foo/serde"` is the same as `"dep:foo", "foo?/serde"`.

We suspect this might be confusing and it would be more elegant to reduce the amount of syntax but its unclear how much of a problem this is for users in practice which is important to weigh out against the transition costs.

We could also phase this out by first deprecating `foo/serde` syntax.
This would better telegraph the change and extend the window for soliciting feedback.
We could tie this deprecation to a package's MSRV so they will only see if i they have the option to change.

In discussion confusing syntax, one point of confusion that came up was that `dep:foo/serde` is unsupported.

## Misc

- [baby230211](https://github.com/baby230211) fixed `cargo publish` so that when it strips dev-dependencies, it will strip activations of those dependencies in [#13518](https://github.com/rust-lang/cargo/pull/13518).
- Muscraft put in heoric work renaming `Config` to `GlobalContext` in [#13409](https://github.com/rust-lang/cargo/pull/13409).
- epage improved clap's error output to help users know how to pass arguments to wrapped commands, like tests, in [#13448](https://github.com/rust-lang/cargo/pull/13448)

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Ready-to-develop:
- [Merge `cargo upgrade` into `cargo update`](https://github.com/rust-lang/cargo/issues/12425)
- [`cargo publish` for workspaces](https://github.com/rust-lang/cargo/issues/1169)
- [Auto-generate completions](https://github.com/rust-lang/cargo/issues/6645)
  - See [clap-rs/clap#3166](https://github.com/clap-rs/clap/issues/3166)
- Generalize cargo's test assertion code
  - [Add `CARGO_WORKSPACE_DIR`](https://github.com/rust-lang/cargo/issues/3946)
  - [Structured assertions in snapbox](https://github.com/assert-rs/trycmd/issues/92)
  - [Find a solution for order-independent assertions between cargo and snapbox](https://github.com/assert-rs/trycmd/issues/151)
- [`cargo update --precise` with pre-release deps](https://github.com/rust-lang/cargo/issues/13290)

Needs design and/or experimentation:
<!-- - [GC](https://github.com/rust-lang/cargo/issues/12633) -->
- [cargo info](https://github.com/rust-lang/cargo/issues/948)
- [Per-user artifact cache](https://github.com/rust-lang/cargo/issues/5931)
- [Dependency resolution hooks](https://github.com/rust-lang/cargo/issues/7193)
- [A way to report why crates were rebuilt](https://github.com/rust-lang/cargo/issues/2904)

Planning:
- Cargo script ([RFC #3502](https://github.com/rust-lang/rfcs/pull/3502), [RFC #3503](https://github.com/rust-lang/rfcs/pull/3503))
- [Disabling of default features](https://github.com/rust-lang/cargo/issues/3126)
- [RFC #3416: `features` metadata](https://github.com/rust-lang/rfcs/pull/3416)
  - [RFC #3485: descriptions](https://github.com/rust-lang/rfcs/pull/3485) (descriptions)
  - [RFC #3487: visibility](https://github.com/rust-lang/rfcs/pull/3487) (visibility)
  - [RFC #3486: deprecation](https://github.com/rust-lang/rfcs/pull/3486)
  - [Unstable features](https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features)
<!-- - [RFC #3452: Nested packages](https://github.com/rust-lang/rfcs/pull/3452) -->
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
- [RFC #3553: Cargo SBOM Fragment](https://github.com/rust-lang/rfcs/pull/3553)
- [RFC #3371: CARGO_TARGET_BASE_DIR](https://github.com/rust-lang/rfcs/pull/3371)
<!-- - [RFC #3243: Packages as optional namespaces](https://github.com/rust-lang/rfcs/pull/3243) -->
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
