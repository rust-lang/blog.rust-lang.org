---
layout: post
title: "March Sprint for rustc: Shrink Memory Usage"
author: Felix Klock
team: The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>
---

I am very excited about the compiler team's upcoming sprint, and
I want to share that excitement with all of you.

The Rust Compiler Team decided over a series of recent meetings
([4 dec 2020][mtg-2020-12-04], [15 jan 2021][mtg-2021-01-15],
[29 jan 2021][mtg-2021-01-29]) that our focus for 2021 will be improving the compiler's
performance from the perspective of the new contributor experience bootstrapping
the compiler and developing code for the compiler.

The team's first sprint for 2021 will be during the first week of March, and its
focus will be on reducing the memory footprint of the compiler during bootstrap.

During our most recent planning meeting ([12 feb 2021][mtg-2021-02-12]),
there were no new significant steering meeting proposals, so we decided to
do some extra sprint planning during that time. We established:

 * we should settle on a common set of tools for measuring memory usage (for now we expect it to be Valgrind DHAT on Linux).
 * we need some centralized documentation for how to best use those tools to measure rustc's memory usage
 * we need concrete proposals for tasks that will drive activity during the sprint; we started up
   the [shrinkmem-sprint zulip][] to host conversations related to that.

One detail about this sprint that differs a little from previous rust hacking sessions like "impl days":
we are not actively trying to solicit new contributors during this time. (Of course new contributors are
always welcome to join in the fun, but the compiler team recognized that we do not have as much mentoring
bandwwidth for this first sprint compared to past "impl days" events.)

[mtg-2020-12-04]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/99285steeringmeeting20201204PerformanceGoalsfor2020.html

[mtg-2021-01-15]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/79391planningmeeting20210115.html

[mtg-2021-01-29]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/95022steeringmeeting20210129.html

[mtg-2021-02-12]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/44925planningmeeting20210212.html

[shrinkmem-sprint zulip]: https://zulip-archive.rust-lang.org/276895tcompilershrinkmemsprint/index.html
