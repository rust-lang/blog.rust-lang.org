+++
path = "inside-rust/2021/02/15/shrinkmem-rustc-sprint"
title = "March Sprint for rustc: Shrink Memory Usage"
authors = ["Felix Klock"]
aliases = ["inside-rust/2021/02/15/shrinkmem-rustc-sprint.html"]

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

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

 * we should settle on a common set of tools for measuring memory usage,
 * we need some centralized documentation for how to best use those tools to measure rustc's memory usage, and
 * we need concrete proposals for tasks that will drive activity during the sprint; we started up
   the [shrinkmem-sprint zulip][] ([public archive][shrinkmem-sprint archive]) to host conversations related to that.

Given the more narrow focus of this sprint, the Rust Compiler Team is interested
in input from people with strong experience using Valgrind DHAT or Windows
Performance Analyzer (or other tools) to help with identifying opportunities to
reduce memory usage. If this sounds like you please drop by the
[shrinkmem-sprint zulip][] and say hello!

[mtg-2020-12-04]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/99285steeringmeeting20201204PerformanceGoalsfor2020.html

[mtg-2021-01-15]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/79391planningmeeting20210115.html

[mtg-2021-01-29]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/95022steeringmeeting20210129.html

[mtg-2021-02-12]: https://zulip-archive.rust-lang.org/238009tcompilermeetings/44925planningmeeting20210212.html

[shrinkmem-sprint zulip]: https://rust-lang.zulipchat.com/#narrow/stream/276895-t-compiler.2Fshrinkmem-sprint
[shrinkmem-sprint archive]: https://zulip-archive.rust-lang.org/276895tcompilershrinkmemsprint/index.html
