+++
path = "inside-rust/2026/05/22/rustup-update-1.30"
title = "Rustup Update: Our Plans for the 1.30 Release Cycle"
authors = ["rami3l"]

[extra]
team = "The Rustup Team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-rustup"
+++

With this post, we summarize the centers of interest of `t-rustup` in the
ongoing release cycle leading to the future 1.30.0 release, hoping to make it
more visible to the community of what is going on with the rustup project's
recent development, as well as how you can potentially help making rustup
better.

## Ongoing Changes

### [Simplification of download & TLS backends](https://github.com/rust-lang/rustup/issues/3790)

Since a long time ago, the simplification of the rustup codebase has been a
recurring topic for the team. One particular area of interest is the downloader
subsystem, since supporting the combination of different download and TLS
backends has been a source of significant maintenance burden.

So far, we have made significant progress towards establishing the new
defaults: the `reqwest` download backend and the `rustls` TLS backend have been
the default and recommended choice since 2019 and 2025 respectively, and
starting from 2025, the uses of the deprecated `curl` download backend and the
`native` TLS backend have been triggering deprecation warnings.

In this release cycle, we have decided to take the next step in this effort by
removing the `curl` download backend, which has recently been merged and is
planned to be shipped in the 1.30 release which is expected to happen in 2027.

In addition, a downgrade of our support for the `native` TLS backend is also on
our roadmap. The current idea is that while its complete removal might be
impractical notably due to Linux packaging policies, it will start to get
"guaranteed to build" support like some Tier 2 targets starting from a certain
point on in the future.

We encourage every user to switch to the `reqwest` download backend and the
`rustls` TLS backend if they haven't already done so (if you haven't seen any
related deprecation warnings, everything is good already!). If there are
specific reasons preventing you from making the switch, please let us know
immediately via an issue and link to the previous
[CfT](https://github.com/rust-lang/rustup/issues/3806).

### [Refining the auto installation behavior]

Back when rustup 1.28.0 was released, the auto installation of active
toolchains has been disabled both in `rustup` and proxy (such as `rustc` and
`cargo`) invocations, which was a significant change in behavior that wasn't
well communicated to users, and we would like to sincerely apologize for having
caused a lot of frustration because of that.

In response to this breakage, the old default behavior has been [re-enabled in
1.28.1](https://blog.rust-lang.org/2025/03/04/Rustup-1.28.1). However, it does
seem that old behavior can be clearly unnecessary in certain places, such as in
the following example with rustup 1.27.1, where "the active toolchain must be
first installed in order to remove a target", while rustup should have simply
bailed out:

```console
> rustup --version
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.81.0 (eeb90cda1 2024-09-04)`

> rustup +beta target remove aarch64-apple-darwin
info: syncing channel updates for 'beta-aarch64-unknown-linux-musl'
1007.6 KiB / 1007.6 KiB (100 %) 567.4 KiB/s in  1s ETA:  0s
info: latest update on 2025-09-28, rust version 1.91.0-beta.4 (aa7859c0d 2025-09-27)
info: downloading component 'cargo'
^C
```

This is why we are planning to re-disable it in some scenarios in the hope of
being able to do this in a more intuitive and far less disrpuptive way.

Recently, a change has been recently
[merged](https://github.com/rust-lang/rustup/pull/4840) that disables the
auto-installation of the active toolchain in `rustup-init` and `rustup`
invocations (with `rustup run`, `rustup doc` and `rustup man` as the only
exceptions); it will be in effect starting from rustup 1.30.

If you have any concerns about the above change, please let us know in the
[issue thread][Refining the auto installation behavior]. In the meantime, extra
deprecation warnings are planned for the upcoming releases of 1.29, which
should provide users with more time to prepare.

[Refining the auto installation behavior]: https://github.com/rust-lang/rustup/issues/4836

## To Be Worked on Shortly

Below is a list of the things that the team plans to be work on in the near
future:

1. [XDG Paths Support](https://github.com/rust-lang/rustup/issues/247): This is our GSoC 2026 Project.
1. [Notify users of possible updates](https://github.com/rust-lang/rustup/issues/3688)
1. [Signing & mirroring support](https://github.com/rust-lang/rustup/issues/2029)

## Focus Areas Without Progress

Below is a list of the things that are on the team's radar but the progress
having been made on them remains limited:

1. [MSVC installation improvements](https://github.com/rust-lang/rustup/issues/4446)
1. Refactoring/Simplification of the installer
   1. [Toolchain content verify & repair wizard](https://github.com/rust-lang/rustup/issues/3940)
   1. [Refactoring of the unpacking logic to async Rust](https://github.com/rust-lang/rustup/issues/4159)
1. [Process safety](https://github.com/rust-lang/rustup/issues/988)
1. [Toolchain identification & deduplication](https://github.com/rust-lang/rustup/issues/4663)
1. [Trusted Directories](https://github.com/rust-lang/rustup/issues/3935)
1. [Proxy mode cold start time improvements](https://github.com/rust-lang/rustup/issues/2626)
1. [JSON output mode](https://github.com/rust-lang/rustup/issues/3434)

## Contributions Welcome

### [Restriction of toolchain names]

In the past, rustup used to put nearly no restrictions on legal toolchain
names, which led to a lot of unnecessary complexities. Notably, this has caused
some [phishing concerns](https://github.com/rust-lang/rustup/issues/4053) and
has also prevented us from [allowing certain toolchain names to bear a special
meaning](https://github.com/rust-lang/rustup/issues/4391#issuecomment-4274125954).

To improve the current situation, we are planning to restrict the toolchain
names so that the regular name should appear less surprising, and that some
particular formats can be reserved for special purposes.

In fact, the team has made the first step in 1.29 by [disallowing toolchain
names that start with `+`](https://github.com/rust-lang/rustup/pull/4650) and
the outcome seems to be positive so far.

If you are interested in helping us out with this effort, please reach out at
the [issue][Restriction of toolchain names] thread so that we can work out the
details and the next steps together.

[Restriction of toolchain names]: https://github.com/rust-lang/rustup/issues/4059

## How You Can Help

You can help us out by participating in the project's development in different
ways, such as providing feedback on the [ongoing changes](#ongoing-changes) or
helping us out with the issues we think would be a fit for interested
contributors in the ["Contributions Welcome" section](#contributions-welcome).

In addition, if there is a specific issue not discussed here that you are
interested in addressing but you are not sure of what to do, please don't
hesitate to reach out to our [Zulip channel][Zulip channel].

[Zulip channel]: https://rust-lang.zulipchat.com/#narrow/channel/490103-t-rustup
