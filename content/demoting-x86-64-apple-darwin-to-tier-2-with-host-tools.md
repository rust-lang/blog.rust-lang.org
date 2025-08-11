+++
path = "2025/08/19/demoting-x86-64-apple-darwin-to-tier-2-with-host-tools"
title = "Demoting x86_64-apple-darwin to Tier 2 with host tools"
authors = ["Jake Goulding"]

[extra]
team = "the Infrastructure team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-infra"
+++

In Rust 1.90.0, the target `x86_64-apple-darwin` will be demoted to Tier 2 with host tools.
The standard library and the compiler will continue to be built and distributed,
but automated tests of these components are no longer guaranteed to be run.

## Background

Rust has supported macOS for a long time,
with some amount of support dating back to Rust 0.1 and likely before that.
During that time period,
Apple has changed CPU architectures from x86 to x86\_64 and now to Apple silicon,
ultimately announcing the [end of support][timeline] for the x86\_64 architecture.

Similarly,
[GitHub has announced][gha] that they will no longer provide free macOS x86\_64 runners for public repositories.
The Rust Project uses these runners to execute automated tests for the `x86_64-apple-darwin` target.
Since the [target tier policy][policy] requires that Tier 1 platforms must run tests in CI,
the `x86_64-apple-darwin` target must be demoted to Tier 2.

## What changes?

Starting with Rust 1.90.0, `x86_64-apple-darwin` will be Tier 2 with host tools.
For users,
nothing will change immediately;
builds of both the standard library and the compiler will still be distributed by the Rust Project for use via `rustup` or alternative installation methods.

Over time,
this target will likely accumulate bugs faster due to reduced testing.

## Future

If the `x86_64-apple-darwin` target causes concrete problems,
it may be demoted further.
No plans for further demotion have been made yet.

For more details on the motivation of the demotion, see [RFC 3841][].

[policy]: https://doc.rust-lang.org/stable/rustc/target-tier-policy.html
[timeline]: https://en.wikipedia.org/wiki/Mac_transition_to_Apple_silicon#Timeline
[gha]: https://github.blog/changelog/2025-07-11-upcoming-changes-to-macos-hosted-runners-macos-latest-migration-and-xcode-support-policy-updates/#macos-13-is-closing-down
[RFC 3841]: https://rust-lang.github.io/rfcs/3841-demote-x86_64-apple-darwin.html
