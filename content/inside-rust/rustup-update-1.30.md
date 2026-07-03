+++
path = "inside-rust/2026/07/03/rustup-update-1.30"
title = "Rustup update: our plans for the 1.30 release cycle"
authors = ["rami3l"]

[extra]
team = "The Rustup Team"
team_url = "https://rust-lang.org/governance/teams/dev-tools/#team-rustup"
+++

This blog post is `t-rustup`'s first attempt of providing a more transparent
view of the team's current work.

Here, we summarize the focus areas of `t-rustup` in the ongoing release cycle
leading to the future 1.30.0 release, hoping to inform you of our future plans,
and ask for your feedback to help us make rustup better.

## Ongoing Changes

### [Refining the implicit installation behavior]

In rustup 1.27 or earlier, rustup would implicitly install the active toolchain
once found missing whenever rustup is invoked, including proxy invocations such
as `rustc` and `cargo`. There wasn't a way to disable this behavior, and it had
become a source of confusion from time to time.

Back when rustup 1.28.0 was released, implicit installation has been entirely
removed, which was a significant change in behavior that wasn't well
communicated to users, and we would like to sincerely apologize for the
frustration this has caused.

In response to this breakage, implicit installation has been
[re-enabled](https://blog.rust-lang.org/2025/03/04/Rustup-1.28.1), and you can
control it with the `RUSTUP_AUTO_INSTALL` environment variable or the `rustup
set auto-install` command.

However, it does seem that this behavior would be clearly unnecessary in
certain places, such as in the following example with rustup 1.27.1, where
rustup must install the active toolchain before removing a target while it
should have bailed out:

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

To make sure that rustup is not doing unnecessary work, we would like to
[disable](https://github.com/rust-lang/rustup/pull/4840) implicit installation
of the active toolchain in the `rustup-init` and `rustup` invocations when
unnecessary starting from 1.30. You can refer to the
[issue][Refining the implicit installation behavior] for a complete list of
impacted subcommands.

With the new behavior, the above example would simply fail, which should feel
more natural:

```console
> rustup +beta target remove aarch64-apple-darwin
error: toolchain 'beta-aarch64-unknown-linux-musl' is not installed
...
```

To avoid further confusions, we would like to stress a few points regarding
this change and its potential impact:

- Since we're PRESERVING implicit installation on proxy invocations like
  `rustc` and `cargo`, this should not cause widespread breakage as seen when
  1.28.0 was released.

- Most cases where `rustup-init` or `rustup` are invoked don't require the
  active toolchain to be implicitly installed, so we expect the disruption caused
  by this change to be minimal.

- Instead of using commands like `rustup show` to trigger toolchain installation,
  we recommend using `rustup install` instead.

- In future 1.29 releases, upon triggering implicit installation in any of the
  relevant rustup subcommands, rustup will print a deprecation warning to inform
  you of the upcoming change.

If you still have any concerns regarding the above change, please let us know
in the [issue thread][Refining the implicit installation behavior].

[Refining the implicit installation behavior]: https://github.com/rust-lang/rustup/issues/4836

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
removing the `curl` download backend. This change has recently been merged and
is planned to be shipped in the 1.30 release which is expected to happen in 4
to 8 months.

In addition, the `native` TLS backend has also been a difficult subject for the
team especially due to the difficulties of building and/or linking OpenSSL
consistently across multiple platforms. There are currently no plans to remove
it notably due to Linux packaging policies, but it might be downgraded to
"guaranteed to build" support from a certain point on in the future.

We encourage you to switch to the `reqwest` download backend and the `rustls`
TLS backend if you haven't already done so (if you haven't seen any related
deprecation warnings, you have nothing to worry about!). If there are specific
reasons preventing you from making the switch, please let us know via an issue
and link to the previous
[CfT](https://github.com/rust-lang/rustup/issues/3806).

## To Be Worked on Shortly

Below is a list of the things that the team plans to work on in the near
future:

1. [XDG Paths Support](https://github.com/rust-lang/rustup/issues/247): This is our GSoC 2026 Project.
1. [Notify users of possible updates](https://github.com/rust-lang/rustup/issues/3688)
1. [Signing & mirroring support](https://github.com/rust-lang/rustup/issues/2029)

## Focus Areas Without Progress

Below is a list of objectives on the team's radar where progress has been limited:

1. [Artifact signing](https://github.com/rust-lang/rustup/issues/2027)
1. [Process safety](https://github.com/rust-lang/rustup/issues/988)
1. [Toolchain identification & deduplication](https://github.com/rust-lang/rustup/issues/4663)
1. [Toolchain content verification & repair](https://github.com/rust-lang/rustup/issues/3940)
1. [Enhanced concurrency in toolchain installation](https://github.com/rust-lang/rustup/issues/731)
1. [MSVC environment setup improvements](https://github.com/rust-lang/rustup/issues/4446)
1. [JSON output mode](https://github.com/rust-lang/rustup/issues/3434)
1. [Trusted directories](https://github.com/rust-lang/rustup/issues/3935)

## Contributions Welcome

### [Restriction of toolchain names]

In the past, rustup used to put nearly no restrictions on toolchain names,
which led to a lot of unnecessary complexities. Notably, this has caused some
[phishing concerns](https://github.com/rust-lang/rustup/issues/4053) and has
also prevented us from [allowing certain toolchain names to bear a special
meaning](https://github.com/rust-lang/rustup/issues/4391#issuecomment-4274125954).

To improve the current situation, we are planning to restrict toolchain names
so that they should appear less surprising in the future, and that some
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
