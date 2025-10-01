+++
path = "inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90"
title = "This Development-cycle in Cargo: 1.90"
authors = ["Ed Page"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

# This Development-cycle in Cargo: 1.90

This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.90 as well as highlights from 1.87, 1.88, 1.89.

<!-- time period: 2025-02-20 through 2025-09-18 -->

- [Plugin of the cycle](#plugin-of-the-cycle)
- [Implementation](#implementation)
  - [GSoC: Alternative design for `cargo fix`](#gsoc-alternative-design-for-cargo-fix)
  - [GSoC: Prototype a new set of Cargo "plumbing" commands](#gsoc-prototype-a-new-set-of-cargo-plumbing-commands)
  - [GSoC: Build script delegation](#gsoc-build-script-delegation)
  - [`--target host`](#target-host)
  - [annotate-snippets](#annotate-snippets)
- [Design discussions](#design-discussions)
  - [All hands: XDG paths](#all-hands-xdg-paths)
  - [All hands: cargo linting](#all-hands-cargo-linting)
  - [All hands: doctests](#all-hands-doctests)
  - [All hands: code-gen settings](#all-hands-code-gen-settings)
  - [All hands: `cargo explain`](#all-hands-cargo-explain)
  - [`cargo doc --serve`](#cargo-doc-serve)
  - [Multi-line messages from build scripts](#multi-line-messages-from-build-scripts)
  - [Forgetting `cargo fmt` after `cargo fix`](#forgetting-cargo-fmt-after-cargo-fix)
  - [Recursively find dependencies at a `path`](#recursively-find-dependencies-at-a-path)
  - [Include workspace license files with `cargo new`](#include-workspace-license-files-with-cargo-new)
- [Misc](#misc)
- [Focus areas without progress](#focus-areas-without-progress)

## Plugin of the cycle

Cargo can't be everything to everyone,
if for no other reason than the compatibility guarantees it must uphold.
Plugins play an important part of the Cargo ecosystem and we want to celebrate them.

Our plugin for this cycle is [`cargo-deny`](https://crates.io/crates/cargo-deny),
a linter for Cargo.
A builtin linter for Cargo is being tracked in
[#12235](https://github.com/rust-lang/cargo/issues/12235)
along with [ideas for lints](https://github.com/rust-lang/cargo/issues?q=is%3Aissue%20state%3Aopen%20label%3AA-new-lint).

Thanks to [krpeid](https://github.com/kpreid) for the suggestion!

[Please submit your suggestions for the next post.](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/Plugin.20of.20the.20Dev.20Cycle/near/420703211)

## Implementation

### GSoC: Alternative design for `cargo fix`

`cargo fix` will apply suggested fixes for lints
and has worked well for cleaning up sloppy code,
reducing the annoyance of toolchain upgrades where lints may change,
helping with Edition migrations,
and adopting of new lints in a code base.

However, `cargo fix`
- Can be slow ([#13214](https://github.com/rust-lang/cargo/issues/13214))
- Only applies a subset of possible lints
- You can't be selective of which lints are fixed without a lot of mucking with `RUSTFLAGS` which was important with the 2024 Edition migration because some lints still had a lot of false positives at first

A problem with addressing these is the current architecture.
`cargo fix` is implemented as a variant of `cargo check` that replaces `rustc` with `cargo` being run in a special mode that will call `rustc` in a loop, applying fixes until there are none.
While this special `rustc`-proxy mode is running,
a cross-process lock is held to force only one build target to be fixed at a time to avoid race conditions.
This ensures correctness at the cost of performance and difficulty in making the `rustc`-proxy interactive.

[cargo-fixit](https://github.com/crate-ci/cargo-fixit)
is a Proof of Concept for an alternative design,
developed by [Pyr0de](https://github.com/Pyr0de).
With this design,
`cargo fixit` spawns `cargo check` in a loop,
determining which build targets are safe to fix in this pass,
and applying the suggestions.
This puts the top-level program in charge of what fixes get applied,
making it easier to coordinate, allowing the locking to be removed and opening the door to an interactive mode.
This comes at the cost that fixes in packages lower in the dependency tree can cause later packages to rebuild multiple times,
slowing things down.

Regarding performance,
cargo-fixit is [showing promising results](https://github.com/crate-ci/cargo-fixit/blob/main/benchsuite/runs/2025-07-31-af6627c.md).

There remains
- Investigations into whether other optimizations are safe ([cargo-fixit#52](https://github.com/crate-ci/cargo-fixit/issues/52), [cargo-fixit#21](https://github.com/crate-ci/cargo-fixit/issues/21))
- Open questions on how the `cargo fix` interface should be maintained with this new design ([stabilization issues](https://github.com/crate-ci/cargo-fixit/milestone/5))
- Finish handling all of the remaining functionality

### GSoC: Prototype a new set of Cargo "plumbing" commands

We've had a [project goal for plumbing commands](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html)
for a while and
[secona](https://github.com/secona) wrote [cargo-plumbing](https://github.com/crate-ci/cargo-plumbing)
as a prototype.
The focus was on better understanding of what the plumbing commands can look like
and what is needed from Cargo.
Compromises had to be made in the actual result to not be blocked on what the Cargo Rust APIs currently allow
([cargo-plumbing#82](https://github.com/crate-ci/cargo-plumbing/issues/82)).
For example, instead of solely relying on the manifests that the user passed in,
the plumbing commands will re-read the manifests within each command,
preventing callers from being able to edit them to get specific behavior out of Cargo, e.g. dropping all workspace members to allow resolving dependencies on a per-package basis.

`cargo-plumbing` currently covers
- `locate-manifest`
- `read-manifest`
- `read-lockfile`
- `lock-dependencies`
- `write-lockfile`
- `resolve-features`
- `plan-build`

### GSoC: Build script delegation

Build scripts come at a compile time cost.
Even with `cargo check`,
they must be built as if you ran `cargo build` so they can be run.
While we need to identify ways to abstract common build script patterns
([#14948](https://github.com/rust-lang/cargo/issues/14948)),
that may not always be doable.
However, if we can shift build scripts from being defined in every package that needs the functionality into a couple of core build script packages,
we can reduce the number of build scripts that need to be built and linked.

The first step in being able to delegate build scripts to packages
is to be able to have multiple build scripts which is what
[namanlp](https://github.com/namanlp) has developed so far.

You can have
```toml
[package]
build = ["windows-manifest.rs", "release-info.rs"]
```
and your package can access their `OUT_DIR`s as `env!("windows-manifest_OUT_DIR")` and `env!("release-info_OUTDIR")`
([documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#metabuild)).

The next phase is static parameters being defined in `Cargo.toml`
and then specifying dependencies using [artifact-dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies).

### `--target host`

The following two commands are not equivalent:
```console
$ cargo build
$ cargo build --target `rustc --print host-tuple`
```

While they both build for the host's platform tuple,
the first is run in "host mode"
while the second is run in "cross-compilation mode".
In "host mode",
build scripts and proc-macros are built the same as binaries and tests, including `RUSTFLAGS` being applied, and everything is output to `target/<profile>`.
In "cross-compilation mode" with the host's tuple,
everything still builds for the host's tuple but `RUSTFLAGS` are not passed to build scripts and proc-macros and everything is output to `target/<tuple>/<profile>`.

Some challenges with this:
- Naively, someone may run `cargo build` for packaging artifacts for their current platform and `cargo --target <tuple>` for packaging artifacts for other platforms and the difference in the artifact-dir causes confusion and frustration in automation.
- In some cross-compilation scenarios,
users also need to specify `RUSTFLAGS` for their build scripts and proc-macros.
- When setting [`build.target`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) to a specific platform tuple,
  there wouldn't be a way to set it back to the host without hard coding a specific host, whether host mode or cross-compilation mode

[`target-applies-to-host`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#target-applies-to-host)
and
[`host-config`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#host-config)
are unstable features that have tried to address this.

In [#13051](https://github.com/rust-lang/cargo/issues/13051),
[kpreid](https://github.com/kpreid)
suggested a related feature to `target-applies-to-host`,
a platform-tuple placeholder where the following mean the same thing:
```console
$ cargo build --target host
$ cargo build --target `rustc --print host-tuple`
```

A user could then opt-in to always being in "cross-compilation" mode by setting in `~/.cargo/config.toml`:
```toml
[build]
target = "host"
```

<!-- 2025-08-11 -->
When talking about this as a team,
one concern was where all this could apply.
For example, the following might not make sense:
```toml
[target.host.dependencies]
regex = "1"
```
We decided the alias would only exist for `--target` / `build.target` for now.
We can evaluate where it makes sense to expand from there.

Another issue was whether `host` would work as a placeholder.
As we'd be carving out `host` from possible values,
we'd need to know whether that is safe.
We checked with T-compiler at [#t-compiler > &#96;--target=host&#96; as alias to host triple in cargo @ 💬](https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/.60--target.3Dhost.60.20as.20alias.20to.20host.20triple.20in.20cargo/near/534227313) and they were fine with it.
There is the question of whether `host` is clear that this is a placeholder for the host tuple or if it could be confused with "host mode" and the unstable `[host]` table for host builds.
One idea was to name it `{host}`,
to make this an explicit variable substitution but weren't thrilled with that, especially on the command-line.
For now, we've settled on `host-tuple` though the stabilization discussion is still on-going.

### annotate-snippets

*Update from [1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78/#user-controlled-cargo-diagnostics)*

[Muscraft](https://github.com/Muscraft/) released annotate-snippets
[0.12](https://github.com/rust-lang/annotate-snippets-rs/blob/master/CHANGELOG.md#0120---2025-08-28)
which includes
- swapping in rustc's renderer and fixing many bugs
- API re-design

This release is in preparation for replacing rustc's renderer with annotate-snippets.
At this point,
it appears that annotate-snippets is flexible enough and complete enough to handle all of rustc's needs and the remaining issues are with rustc itself and the adapter between rustc and annotate-snippets.

With annotate-snippets v0.12 out,
[epage](https://github.com/epage) started the effort to switch Cargo's user messages over to it which is being tracked in [#15944](https://github.com/rust-lang/cargo/issues/15944).

## Design discussions

### All hands: XDG paths

<!-- https://hackmd.io/oA1Ek1fBQxiMkql6osYq6A -->
Cargo stores all of its user-wide content under `CARGO_HOME`, usually `~/.cargo`,
including configuration, caches, and locally built binaries.
There has been a long standing request to use the OS native paths for these,
particularly [XDG Base Directories](https://specifications.freedesktop.org/basedir-spec/latest/)
([#1734](https://github.com/rust-lang/cargo/issues/1734)).

While it can be difficult to take an existing application and migrate data to a new location without breaking compatibility,
what makes this more difficult includes:
- Different versions of Cargo are expected to run side-by-side
- [`rustup`](https://rustup.rs/) integrates with `CARGO_HOME` and is versioned independent of Cargo and supports interacting with every version of Cargo
  - They both use `$CARGO_HOME/bin` for installing binaries
  - Rustup sets up `$CARGO_HOME/env` for users to source for adding `$CARGO_HOME/bin` to their `PATH` to access both Rustup and Cargo installed binaries
  - Rustup cleans up rustup and cargo content, including `$CARGO_HOME/bin` on uninstall
  - Cargo needs to be able to find third-party subcommands installed by both Cargo and Rustup
  - Rustup's `cargo` proxy always sets `CARGO_HOME` on the real `cargo`

Working through the Cargo / Rustup interactions has been a problem for moving this forward
(including the existing [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747))
and we were able to get the Cargo and Rustup teams together at the All Hands to talk through these problems.

The most immediate problem is Rustup setting `CARGO_HOME`.
To not break compatibility,
Cargo should always respect it if `CARGO_HOME` is set.
However, if Rustup always sets it,
then Cargo will never use OS native paths.
The intention behind Rustup setting `CARGO_HOME` is to make sure Rustup and Cargo use the same `CARGO_HOME` for `bin/` and `env`.
This was particularly a problem at some point in the past
when they did not agree on a definition.
Because new versions of Rustup work with old versions of Cargo,
this remains an issue.

Both teams suspect it will be acceptable at this point
for Rustup to stop setting `CARGO_HOME`.
However, we first need to characterize their diverging definitions of `CARGO_HOME` and  see if there is anything we can do to mitigate user problems.
This is being tracked in [rustup#4502](https://github.com/rust-lang/rustup/issues/4502).

As Cargo then moves to OS native paths,
does Rustup need to continue to match behavior and use the same paths?
This boiled down to whether Rustup should continue to cleanup `CARGO_HOME/bin`, caches, and config on uninstall.
If a user sets `~/.local/bin` as their install path,
Rustup could end up deleting user binaries.
In fact, Rustup does it today because some applications have chosen to install their binaries into `~/.cargo/bin`,
even if they weren't installed by `cargo install`.
It's also likely not a good idea to be deleting the users configuration.
We decided that Rustup should only remove content it manages
([rustup#285](https://github.com/rust-lang/rustup/issues/285)).

With that resolved,
we can consider on its own merits whether to use the same paths.
We leaned towards them being configured separately
but first we need to test the user experience for this to see how well it will work in practice.
We should at least coordinate on whether to use application or XDG paths on Mac.

As for `$CARGO_HOME/.env`, our expectation is that it will contain the default bin path for both Rustup and Cargo.

The rest of the mechanics are more program specific.
We talked a bit about Cargo's transition.
We still plan to do this in two phases, (1) make the more granular paths configurable and then (2) change the defaults to OS native paths.
The change to OS native paths might not even need to happen in one go so long as we've solve the policy questions (particularly for Mac).
For config, Cargo can read from both locations.
For caches, Cargo can abandon the old location though there is a question of how to handle the cache garbage collection database,
whether to have them be distinct or not.
There is a question on how to transition the bin path.

### All hands: cargo linting

*Update from [1.83](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83/#linting)*

<!-- 2025-04-22 -->
In preparing for the All Hands,
[epage](https://github.com/epage) ran a vibe check for
[previously raised questions](https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83/#linting), including:
- What is the intent for the Rustc/Clippy divide and should Cargo mirror it?
- What namespace should Cargo lints live in?
- If Cargo share a namespace with Rustc or Clippy, should `RUSTFLAGS` affect cargo?
  - e.g. given `RUSTFLAGS=-Ddeprecated cargo check`, should Cargo `deprecated` warnings also error?

We mostly focused the discussion on `RUSTFLAGS` behavior.
This would likely be implemented by Cargo passing the `RUSTFLAGS` to Rustc and asking it to print the effective lint level for the lints.
This felt convoluted to the team and `RUSTFLAGS` is intended as a low-level escape hatch and we should not be elevating its use in this way.

<!-- https://hackmd.io/bRs_A-SgSoWKGqc4YCSJIA -->
At the All Hands,
the above questions were re-visited with members of the Cargo team,
Compiler team,
and Clippy team.
While there is a quality divide between Rustc and Clippy, including performance,
the general sentiment is to avoid the divide unless you absolutely have to.

One difference between Rustc and Clippy lints that was called out is that Clippy is more strict.
When a Clippy lint is uplifted into Rustc, the severity is lowered.
A lint that is a `deny` for Clippy would be a `warn` for Rustc.

The Clippy team also recommended adopting organizing lints by semantic groups that have the same lint level for the entire group,
like [Clippy's groups](https://doc.rust-lang.org/clippy/).
This still leaves figuring out how this intersects with the lint levels having different meanings between Rustc and Clippy.
If Cargo calibrates to Rustc's lint levels,
should Cargo lower the severity of the groups or be more selective about what goes into each group?

When discussing the namespace for Cargo lints,
the recommendation is that Cargo only use the `cargo::` namespace,
keeping the focus on the linter,
at the cost of requiring users to common lints in two different places,
even if it has extra configuration like [`unexpected_cfgs`](https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html).
Another reason for Cargo to focus on the `cargo::` namespace is to make it easier for users to identify correctly where to open issues.

With these discussions out of the way,
we can continue to work towards stabilizing Cargo's linter
([#12235](https://github.com/rust-lang/cargo/issues/12235)).

In preparation for that, we improved our tracking of new lints, including:
- Adopted a clippy-like template for requesting lints
- Created Issues for [existing](https://rust-lang.github.io/rust-clippy/stable/index.html?groups=cargo) and [requested](https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue%20state%3Aopen%20label%3AT-cargo) Clippy Cargo lints
- Add an [`A-new-lint`](https://github.com/rust-lang/cargo/issues?q=is%3Aissue%20state%3Aopen%20label%3AA-new-lint) tag

### All hands: doctests

*Update from [1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82/#all-targets-and-doc-tests)*

When Cargo runs doctests,
it invokes `rustdoc` against the library to build and run the tests.
This runs counter to the way the rest of Cargo works which creates warts in behavior, requiring special cases to workaround, and frustrating users 
(e.g. a recent [reddit thread](https://www.reddit.com/r/rust/comments/1noj73x/are_doctests_painful_to_write_or_is_it_just_me/)).

Some examples of problems with doctests are:
- inability to run `cargo check` or `cargo clippy` on them
- `cargo test --workspace` rebuilding doctests when there was no change
- cargo can't collect "unused depednency" messages from rustc to identify which dependencies are unused across all dev-dependencies

This also affects future plans including:
- Coverage reporting ([#13040](https://github.com/rust-lang/cargo/issues/13040))
- Cargo providing improved output (e.g. [#2832](https://github.com/rust-lang/cargo/issues/2832))

The Testing DevEx team has done some brainstorming on this problem
([testing-devex-team#5](https://github.com/rust-lang/testing-devex-team/issues/5)), including:
- Building doctest support into the compiler,
  parsing `#[doc]` attributes and generating `#[test]` functions for them
  - Allows doctests on internal items and binary targets
  - Doesn't allow for the "public interface" testing unless Rustc also links to the original lib or do some import path hackery
  - Has issues with features like `compile_fail` and per-doctest Editions
- Using `rustdoc --output-format=doctest` to extract doctests, generate test files, build, and then run those ([rust#134529](https://github.com/rust-lang/rust/issues/134529))

<!-- https://hackmd.io/68nIlljST5y-h0xZa9Gt4g -->
<!-- https://hackmd.io/kQttcFQpQJa7WrQ3J5uLAg -->
<!-- https://hackmd.io/@fmease/inject-doctests-into-host-crate -->
<!-- 2025-05-20 -->
[epage](https://github.com/epage) took this to the Rustdoc team at the All Hands.
They had a variant of rustc having built-in knowledge of doctests:
run `rustdoc` as an alternative compiler driver that will do the translation.
This idea was recorded in [rust#141083](https://github.com/rust-lang/rust/issues/141083).
One problem that came up later on the issue is `cargo clippy` support as that is a separate compiler driver.
One idea is for the logic to be in rustc itself any drivers that link against it can provide a `--doctest` flag like the `--test` flag.

This is still left as an open area to further explore.

### All hands: code-gen settings

<!-- 2025-04-01 -->
The Cargo team discussed a request for exposing `-Cforce-frame-pointers` in
[`[profiles]`](https://doc.rust-lang.org/cargo/reference/profiles.html#custom-profiles)
([#15333](https://github.com/rust-lang/cargo/issues/15333)).
This seemed like a reasonable idea.
We decided against changing the `bench` profile to have this enabled
so that `cargo bench` and `cargo build --release` do the same thing by default.

This does tie into the broader question about deciding what compiler settings should be set in `[profiles]` and if we can smooth out the process.
The burden for adding new settings across both rustc and cargo was highlighted by
[kornelski](https://github.com/kornelski)
on
[Internals](https://internals.rust-lang.org/t/the-burden-of-creating-new-compiler-options-and-exposing-them-in-cargo/22101)
this is made harder by the fact that the Cargo team is generally disconnected from the development of these requests and it can be difficult to have the context for how these should be exposed in a higher-level, opinionated tool like Cargo.

<!-- https://hackmd.io/KbOUdlszRgCpMyWb1-YjVA -->
The Cargo team followed up with the Compiler team at the All Hands.
An idea was discussed of a `profile.*.codegen` field that would accept a list of
[`-C` code-gen settings](https://doc.rust-lang.org/nightly/rustc/codegen-options/index.html)
in a format Cargo could check against an allowlist, normalize, and de-duplicate.
Some `-Z` flags, like `Z fmt-debug`, may need to be turned into `-Zunstable-options -C fmg-debug` to work with this.

The allowlist is important because Cargo's abstractions over rustc are intended to not be able to create unsound programs
(e.g. having different floating point settings between compilation units)
and to prevent arbitrary injection through response files.
Updating the allowlist could be a step in the stabilization report
and provide a sync point between the Compiler and Cargo team meetings which is currently lacking.

<!-- 2025-05-20 -->
We then followed up on this in a Cargo team meeting as not everyone was at the All Hands or even at that session.

Unlike `RUSTFLAGS`,
this is something Cargo can reasonably parse and validate.
However, both have the problem of potentially interacting poorly with
built-in Cargo behavior.
The allowlist could help with this as well.

We did not come to any definitive answer in the discussion.

### All hands: `cargo explain`

<!-- https://hackmd.io/KbOUdlszRgCpMyWb1-YjVA -->
At the All Hands,
[timclicks](https://github.com/timclicks) attended an open time with the Cargo team to discuss ways to improve the experience with compiler diagnostics.
The core of the idea is to provide one place for users to go to get more detailed information on an error code or lint name.
Currently, you can view error code descriptions with `rustc --explain` or `cargo --explain` and clippy lints with `cargo clippy --explain`.
There isn't a local view for rustc lints.
These views dump markdown to the screen without any styling.

[timclicks](https://github.com/timclicks) proposed there to be a `cargo explain` subcommand that could take in an error code or lint name and stylize the markdown output.
There is the question of how information for third-party lints could be found once [rust#66079](https://github.com/rust-lang/rust/issues/66079) is stabilized.

This could be well served by having a built-in pager
([#11213](https://github.com/rust-lang/cargo/issues/11213)).
This led [epage](https://github.com/epage) to floating an idea that he has been considering for a bit: supporting a compilation watch mode
([#9339](https://github.com/rust-lang/cargo/issues/9339))
but in the style of [bacon](https://dystroy.org/bacon/).
Combined with this idea of `cargo explain`,
maybe it could be an interactive mode (e.g. `cargo check -i`)
that allowed you to select a diagnostic and have it pop up an "explain" view.
In response, someone brought up that the compiler knows more than is currently shown and maybe this could be included in the json output so that the user could drill further into a diagnostic in interactive mode.
Like with many other subcommands, this would likely best be experimented with as a
[custom subcommand](https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands)
before integrating it into Cargo.

### `cargo doc --serve`

<!-- 2025-06-03 -->
The Cargo team reviewed an old requested for `cargo doc` to get a `--serve` flag
([#4966](https://github.com/rust-lang/cargo/issues/4966)).
The given use case is for web browsers that limit plugin access to `file://` URLs.
This also seems relevant for
[WSL](https://learn.microsoft.com/en-us/windows/wsl/),
dev containers,
and remote development.
However, we would be concerned about anyone using this in production.

If the `cargo doc` process is staying alive to serve files,
it seems a natural extension to rebuild the documentation on changes
([#9339](https://github.com/rust-lang/cargo/issues/9339)).
That can then lead to considerations for live reloading
or broader ideas of watch functionality as an "interactive mode"
([example](https://github.com/rust-lang/cargo/issues/9339#issuecomment-3340096325))
as [discussed earlier](#all-hands-cargo-explain).
Presenting serve/watch as an "interactive mode" can help set expectations
when it comes to production use.

A challenge with this is the support burden this could put on the Cargo team,
which includes supporting users through unmet expectations and feature creep.
For example, mdbook has [several issues](https://github.com/rust-lang/mdBook/issues/?q=is%3Aissue%20state%3Aopen%20label%3ACommand-serve) open related to its `serve` functionality.

`rustup doc` also has similar needs and it would be good for this effort to be coordinated with them
(
[rustup#2151](https://github.com/rust-lang/rustup/issues/2151),
[rustup#3112](https://github.com/rust-lang/rustup/issues/3112)
).

Possible paths forward include experimenting with this as a nightly-only feature
or creating a third-party subcommand to experiment with the full blown interactive mode.

### Multi-line messages from build scripts

[kornelski](https://github.com/kornelski)
and [ratmice](https://github.com/ratmice)
started back up the conversation on multi-line build script errors and warnings.

Build scripts communicate to cargo through [directives](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script)
of the format `cargo::<name>=<params>\n`,
including `cargo::warning=<message>\n` and `cargo::error=<message>\n`.
The only way to report multiple lines of text for the same message is to split the message across multiple of these directives,
causing each line to render as `error: <name>@<ver>: <line>`.
[epage](https://github.com/epage) wrote up a
[summary](https://github.com/rust-lang/cargo/issues/13233#issuecomment-3185855839),
got input, and brought this before the Cargo team.

The options includes:
```
# Concatenation
cargo::error=header
cargo::error+=second line

# Block start/end
cargo::error_start=
header
second line
cargo::error_end=

# Line-count prefix
cargo::error_start=2
header
second line

# Block start/end + line prefix
cargo::error_start=
cargo::error=header
cargo::error=second line
cargo::error_end=
```

<!-- 2025-08-18 -->
When the team stepped through the options,
we determined that our values for a solution include:
- Every line having a `cargo::` prefix to match with our previous communication that cargo ignores everything else
- Explicit end-of-message, rather than being inferred
  - Some felt uncomfortable with a blank line being sufficient to end buffering
  - Some felt uncomfortable with cargo::error and cargo::warning changing their semantics from non-buffered to buffered
- Graceful handling of nested content or multi-threaded environments

These values excluded "block start/end" and "line-count prefix" solutions.
"Concatenation" and "block start/end + line prefix" have points both for and against them.
In the end, we favored the value of "explicit end-of-message" and preferred "block start/end + line prefix".

There are still
[open questions](https://github.com/rust-lang/cargo/issues/13233#issuecomment-3201200381),
though we didn't feel strongly whether those needed to be resolved before implementation or during the stabilization process.

### Forgetting `cargo fmt` after `cargo fix`

<!-- team meeting 2025-03-11 -->
`cargo fix` and `cargo clippy --fix` will apply suggested fixes from lints.
However, the output is not always pretty and making it so would be difficult.
If you are like me, it's easy to not notice that `cargo fmt` is needed until you see CI fail.
It would help if `cargo fix` also handled formatting
([#12608](https://github.com/rust-lang/cargo/issues/12608)).

One problem is users may not be using `cargo fmt`
and having `cargo fix` apply it for the first time could be unwanted on its own and hide the actual fix that went in.

A way to reduce unwanted edits is if we had an interface to tell `cargo fmt` to only re-format the parts changed by `cargo fix`.
The exact interaction for these would have to be carefully considered and may still produce unwanted reformatting.

At the next level, Cargo could have a [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) field
```
[fix]
rustfmt = true
```
If we did this, we may want to have Cargo run `cargo fmt --check` first,
similar to our "VCS dirty" check.

At a minimum, `cargo fix` could recommend running `cargo fmt` if changes were made.

Our discussion was inconclusive and we moved it to [Internals](https://internals.rust-lang.org/t/properly-formatted-cargo-fix-code/22539).

### Recursively find dependencies at a `path`

<!-- team meeting 2025-03-11 -->
Cargo will automatically find a package within its git repo, like:
```toml
[dependencies]
bevy_math = { git = "https://github.com/bevyengine/bevy.git" }
```
However, the same can't be said for `path` dependencies where an exact path is needed.
Say you cloned Bevy into `/Users/epage/Development/bevy`, you'd have to depend on it as:
```toml
[dependencies]
bevy_math = { path = "/Users/epage/Development/bevy/crates/bevy_math/" }
```

For `git`, the recursive path search has run into several issues, including:
- A performance hit for walking the filesystem on every run and parsing the manifests ([#14395](https://github.com/rust-lang/cargo/issues/14395))
- Ambiguous package names ([#11858](https://github.com/rust-lang/cargo/issues/11858))
- ~~Non-determinism in package selection when multiple share a name~~, fixed in [#14239](https://github.com/rust-lang/cargo/commit/6c0f14c245406e09c1ed7bda8ff770bdd8c3c54e)) 
- ~~Warning about ambiguous package names even when not referenced ([#10752](https://github.com/rust-lang/cargo/issues/10752))~~, fixed in [#14239](https://github.com/rust-lang/cargo/pull/14239)
- Non-fatal error messages for unreferenced packages ([#13724](https://github.com/rust-lang/cargo/issues/13724))

At minimum, we can improve the error message from:
```
error: no matching package named `definitely_not_bar` found
location searched: /Users/eric/Temp/foo/crates/bar
required by package `foo v0.1.0 (/Users/eric/Temp/foo)`
```
to something like:
```
error: no matching package named `definitely_not_bar` found at `bar/`
note: required by package `foo v0.1.0 (/Users/eric/Temp/foo)`

help: package `definitely_not_bar` exists at `bar/definitely_not_bar/`
```

Due to the problems we've had with `git`,
we were hesitant to extend the recursive path search to `path`.
This likely wouldn't be reconsidered until at least
[#14395](https://github.com/rust-lang/cargo/issues/14395)
is addressed.
We talked about the idea of caching the relative path inside of the `Cargo.lock`.
If the package is no longer at that path (update of `git` dependency, user edited the layout at `path`),
Cargo would re-scan for the package.
The big hurdle would be plumbing this advisory information from one lockfile, through the resolve, to the next lockfile.

### Include workspace license files with `cargo new`

<!-- team meeting 2025-03-11 -->
When you run `cargo new` in a workspace,
it will automatically inherit `workspace.package` fields
and the `workspace.lints` table.
Commonly, license files need to be copied into a package for distribution via `cargo publish`
and it would help if symlinked any license files Cargo found in the workspace root into the package
([13328](https://github.com/rust-lang/cargo/issues/13328)).

On a mechanics side,
there are downsides to symlinking on Windows that
could make it a bad policy by default,
regardless of the platform `cargo new` is running on.
Copying, always or as a fallback to symlinking failing,
has its own drawbacks.

We could have a `workspace.new-files` that `cargo new` will copy in.
It's hard to evaluate a solution like this without better understanding
where we might go with templateing ([#5151](https://github.com/rust-lang/cargo/issues/5151)).

We could have a `package.extra-files` which `cargo publish` will copy into the `.crate` file like it does `package.readme` when it's outside of the package root.
If we made it this general,
we'd need to also allow users to specify where the files would go.

There is `package.license-file` which `cargo publish` will already copy into the `.crate` file if it is outside the package root, like with `package.readme`.
However, only one file can be specified ([#5933](https://github.com/rust-lang/cargo/issues/5933))
because this is intended for a custom license when `package.license` is insufficient.
See also
[#8537](https://github.com/rust-lang/cargo/issues/8537),
[#9908](https://github.com/rust-lang/cargo/issues/9908),
[#9972](https://github.com/rust-lang/cargo/issues/9972).
We vaguely discussed a way to map license names,
including a custom license identifier,
to specific files.
`cargo publish` would then copy these into the package root inside the `.crate` file.
We didn't end up reaching any specific conclusions.


## Misc

- [ranger-ross](https://github.com/ranger-ross) is experimenting with new `build-dir` layouts in [#15947](https://github.com/rust-lang/cargo/pull/15947) while working to improve how we test it in [#15874](https://github.com/rust-lang/cargo/pull/15874)

## Focus areas without progress

These are areas of interest for Cargo team members with no reportable progress for this development-cycle.

Project goals in need of owners
- [Prototype Cargo build analysis](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-analysis.html)
- [Open namespaces](https://rust-lang.github.io/rust-project-goals/2025h2/open-namespaces.html)
- [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h2/pub-priv.html)
<!--
- [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)
- [Rework Cargo Build Dir Layout](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html)
- [Prototype a new set of Cargo "plumbing" commands](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html)
- [Finish the libtest json output experiment](https://rust-lang.github.io/rust-project-goals/2025h2/libtest-json.html)
-->

Ready-to-develop:
<!--
- [Open namespaces](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#open-namespaces)
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
<!--
- [OS-native config/cache directories (ie XDG support)](https://github.com/rust-lang/cargo/issues/1734)
  - [Phase 1 Pre-RFC](https://internals.rust-lang.org/t/pre-rfc-split-cargo-home/19747)
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
