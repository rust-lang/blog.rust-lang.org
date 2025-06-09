+++
path = "2025/06/16/rust-compiler-performance-survey-2025"
title = "Rust compiler performance survey 2025"
authors = ["Jakub Beránek"]

[extra]
team = "the Compiler Performance Working Group"
team_url = "https://www.rust-lang.org/governance/teams/compiler#team-wg-compiler-performance"
+++

Long compile times of Rust code are frequently being cited as one of the biggest [challenges][annual-survey] limiting the productivity of Rust developers. Rust compiler contributors are of course aware of that, and they are continuously working to improve the situation, by finding [new ways][how-to-speed-up-the-compiler] of speeding up the compiler, [triaging performance regressions][performance-triage] and [measuring][perf-dashboard] our long-term performance improvements. Recently, we also made progress on some [large changes][stabilize-lld] that have been in the making for a long time, which could significantly improve compiler performance by default.

When we talk about compilation performance, it is important to note that it is not always so simple as determining how long does it take `rustc` to compile a crate. There are many diverse development workflows that might have competing trade-offs, and that can be bottlenecked by various factors, such as the integration of the compiler with the used build system.

In order to better understand these workflows, we have prepared a [Rust Compiler Performance Survey][survey-link]. This survey is focused specifically on compilation performance, which allows us to get more detailed data than what we usually get from the annual State of Rust survey. The data from this survey will help us find areas where we should focus our efforts on improving the productivity of Rust developers.

**You can fill out the survey [here][survey-link].**

Filling the survey will likely take you approximately 10 minutes, and the survey is fully anonymous. We will accept submissions until Monday, July 7th, 2025. After the survey ends, we will evaluate the results and post key insights on this blog.

We invite you to fill the survey, as your responses will help us improve Rust compilation performance. Thank you!

[annual-survey]: https://blog.rust-lang.org/2025/02/13/2024-State-Of-Rust-Survey-results/#challenges
[survey-link]: TODO
[how-to-speed-up-the-compiler]: https://nnethercote.github.io/2025/03/19/how-to-speed-up-the-rust-compiler-in-march-2025.html
[performance-triage]: https://github.com/rust-lang/rustc-perf/blob/master/triage/README.md
[perf-dashboard]: https://perf.rust-lang.org/dashboard.html
[stabilize-lld]: https://github.com/rust-lang/rust/pull/140525
