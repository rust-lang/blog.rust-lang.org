+++
path = "2026/02/02/rust-debugging-survey-2026"
title = "Rust debugging survey 2026"
authors = ["Jakub Beránek"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

We're launching a [Rust Debugging Survey][survey-link].

Various issues with debugging Rust code are often mentioned as one of the biggest [challenges][annual-survey] that annoy Rust developers. While it is definitely possible to debug Rust code today, there are situations where it does not work well enough, and the quality of debugging support also varies a lot across different debuggers and operating systems.

In order for Rust to have truly stellar debugging support, it should ideally:
- Support (several versions!) of different debuggers (such as GDB, LLDB or CDB) across multiple operating systems.
- Implement debugger visualizers that are able to produce quality presentation of most Rust types.
- Provide first-class support for debugging `async` code.
- Allow evaluating Rust expressions in the debugger.

Rust is not quite there yet, and it will take a lot of work to reach that level of debugger support. Furthermore, it is also challenging to ensure that debugging Rust code *keeps* working well, across newly released debugger versions, changes to internal representation of Rust data structures in the standard library and other things that can break the debugging experience.

We already have some [plans][debug-test-suite-gsoc] to start improving debugging support in Rust, but it would also be useful to understand the current debugging struggles of Rust developers. That is why we have prepared the [Rust Debugging Survey][survey-link], which should help us find specific challenges with debugging Rust code.

**You can fill out the survey [here][survey-link].**

Filling the survey should take you approximately 5 minutes, and the survey is fully anonymous. We will accept submissions until Friday, February 20th, 2026. After the survey ends, we will evaluate the results and post key insights on this blog.

We would like to thank [@hashcatHitman](https://github.com/hashcatHitman), who helped prepare the survey.

We invite you to fill the survey, as your responses will help us improve the Rust debugging experience. Thank you!

[annual-survey]: https://blog.rust-lang.org/2025/02/13/2024-State-Of-Rust-Survey-results/#challenges
[survey-link]: TODO
[debug-test-suite-gsoc]: https://github.com/rust-lang/google-summer-of-code?tab=readme-ov-file#improve-rust-compiler-debuginfo-test-suite
