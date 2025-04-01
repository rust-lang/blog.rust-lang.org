+++
path = "2023/08/29/committing-lockfiles"
title = "Change in Guidance on Committing Lockfiles"
authors = ["Ed Page"]
aliases = ["2023/08/29/committing-lockfiles.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

For years, the Cargo team has encouraged Rust developers to
[commit their `Cargo.lock` file for packages with binaries but not libraries](https://doc.rust-lang.org/1.71.1/cargo/faq.html#why-do-binaries-have-cargolock-in-version-control-but-not-libraries).
We now recommend people
[do what is best for their project](https://doc.rust-lang.org/nightly/cargo/faq.html#why-have-cargolock-in-version-control).
To help people make a decision, we do include some considerations and suggest
committing `Cargo.lock` as a starting point in their decision making.
To align with that starting point, `cargo new` will no longer ignore
`Cargo.lock` for libraries as of nightly-2023-08-24.
Regardless of what decision projects make, we encourage regular
[testing against their latest dependencies](https://doc.rust-lang.org/nightly/cargo/guide/continuous-integration.html#verifying-latest-dependencies).

## Background

The old guidelines ensured libraries tested their latest dependencies which
helped us keep quality high within Rust's package ecosystem by ensuring issues,
especially backwards compatibility issues,
were quickly found and addressed.
While this extra testing was not exhaustive,
We believe it helped foster a culture of quality in this nascent ecosystem.

This hasn't been without its downsides though.
This has removed an important piece of history from code bases,
making bisecting to find the root cause of a bug harder for maintainers.
For contributors,
especially newer ones,
this is another potential source of confusion and frustration from an unreliable CI whenever a
dependency is yanked or a new release contains a bug.

## Why the change

A lot has changed for Rust since the guideline was written.
Rust has shifted from being a language for early adopters to being more mainstream,
and we need to be mindful of the on-boarding experience of these new-to-Rust developers.
Also with this wider adoption, it isn't always practical to assume everyone is using
the latest Rust release and the community has been working through how to
manage support for minimum-supported Rust versions (MSRV).
Part of this is maintaining an instance of your dependency tree that can build
with your MSRV.
A lockfile is an appropriate way to pin versions for your project so you
can validate your MSRV but we found people were instead putting upperbounds on their
version requirements due to the strength of our prior guideline despite
[likely being a worse solution](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#multiple-requirements).

The wider software development ecosystem has also changed a lot in the
intervening time.
CI has become easier to setup and maintain.
We also have products like
[Dependabot](https://docs.github.com/en/code-security/dependabot/working-with-dependabot)
and
[Renovate](https://docs.renovatebot.com/).
This has opened up options besides having version control ignore `Cargo.lock` to test newer dependencies.
Developers could have a scheduled job that first runs `cargo update`.
They could also have bots regularly update their `Cargo.lock` in PRs, ensuring
they pass CI before being merged.

Since there isn't a universal answer to these situations,
we felt it was best to leave the choice to developers and give them information they need in making a decision.
For feedback on this policy change,
see [rust-lang/cargo#8728](https://github.com/rust-lang/cargo/issues/8728).
You can also reach out the the Cargo team more generally on
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo).
