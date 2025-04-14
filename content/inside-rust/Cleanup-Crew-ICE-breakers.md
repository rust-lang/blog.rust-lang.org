+++
path = "inside-rust/2020/02/06/Cleanup-Crew-ICE-breakers"
title = "Announcing the Cleanup Crew ICE-breaker group"
authors = ["Santiago Pastorino"]
description = "A new blog where the Rust team can post updates on the latest developments"
aliases = ["inside-rust/2020/02/06/Cleanup-Crew-ICE-breakers.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

Following Niko Matsakis' announcement of the [**LLVM ICE-breaker
group**](https://blog.rust-lang.org/inside-rust/2019/10/22/LLVM-ICE-breakers.html),
I'm pleased to announce the **Cleanup Crew ICE-breaker group**. It
follows the same principle, if you know Rust and would like to
contribute to rustc -- but without knowing about the compiler or taking
on a large commitment -- then the Cleanup Crew ICE-breaker group might
well be for you!

### What is the Cleanup Crew ICE-breaker group?

The "Cleanup Crew" are focused on improving bug reports. Specifically,
the goal is to try to ensure that every bug report has all the
information that will be needed for someone to fix it:

- a minimal, standalone example that shows the problem
- links to duplicates or related bugs
- if the bug is a regression (something that used to work, but no
  longer does), then a bisection to the PR or nightly that caused
  the regression

This kind of cleanup is invaluable in getting bugs fixed.

### Who should join?

It can be done by anybody who knows Rust, without any particularly deep
knowledge of the compiler.  If you want to be part of it and be notified
about things to do, just [add yourself to the list][instructions here]! When we come across a suitable
bug, we'll [write a message][tag syntax] that `@`-mentions every Github user on that
team. If you have some time, maybe you can provide some useful
information.

[instructions here]: https://rustc-dev-guide.rust-lang.org/notification-groups/about.html#join

[tag syntax]: https://rustc-dev-guide.rust-lang.org/notification-groups/about.html#tagging-an-issue-for-a-notification-group

You can find more information about the group on it's [rustc-dev-guide
section](https://rustc-dev-guide.rust-lang.org/notification-groups/cleanup-crew.html).

### Update

Since this blog post was written, the "ICE-breaker" groups have been renamed to "notification groups".
