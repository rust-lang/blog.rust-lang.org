+++
path = "inside-rust/9999/12/31/clippys-feature-warming-up"
title = "What we learned with Clippy's feature freeze"
authors = ["blyxyas"]

[extra]
team = "the Clippy team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-clippy"
+++


The [feature freeze affecting Clippy][feature-freeze] has reached its end date. This means that the Clippy project has been accepting pull
requests that add new features (and new lints) for a while.

The Clippy has reviewed the results, and we can say that it's been a success 🎉. It has been so great that we managed
to create an all-time peak in the number of pull requests opened in a week.

In this plot we can observe that peak (the red lines mark the start and end of the feature freeze)

![Plot of PRs per week, we can see an all-time peak when the feature freeze happens](/inside-rust/clippy-warming-up/prs_per_week.png)

After some data crunching of those pull requests opened after June 26th, we had 18 pull requests open in that period
that added lints. With 326 pull requests open by both and old contributors, we want to highlight all the new people
that started contributing to Clippy, 47 new contributors who opened a total of 195 pull requests.

On a team meeting, we concluded that the results of the feature freeze were positive enough to warrant a future one.
The cadence and duration of these is still being decided.

58% of all pull requests by new contributors were opened by the same person. This single contributor opened 114 pull requests of the
total 195. We held a meeting with her ([ada4a] on Github) to iron out some of the details of
the codebase and our procedures for new contributors. We are currently analyzing that meeting in order to improve
the experience for future and current contributors.

We also asked some questions to Ada (ada4a) to include as a mini Q&A, here are some of the answers.

## Ada's Q&A

> Why do you like contributing to Clippy? What's so fun about it?

I came to enjoy improving the suggestions that Clippy makes: they are
fascinating in how they point out of the exact part of code that is
problematic, and show the (sometimes pretty complex) manipulations required to
fix it. Also, It has been empowering to learn the machinery behind the
compiler, and use that knowledge to refine these diagnostics even further.

> Would you recommend contributing to Clippy? Why or why not?

Yes, for multiple reasons:
- Rust is (in)famously hard to learn, and to me, Clippy is one of the parts of the
toolchain most helpful for beginners, as it teaches idiomatic style and helps
discover helpful functions from std -- thus, contributing to it helps reduce the
entry barrier of the language.
- If you, like me, wanted to learn the inner workings of the compiler and contribute
to it some day, but were intimidated by the [development setup required for that][rustc-dev-guide],
Clippy could be a nice stepping stone, as hacking on it doesn't require compiling
rustc, but it interacts with a lot of compiler's data structures and APIs.

[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html
[ada4a]: https://github.com/ada4a
[feature-freeze]: https://blog.rust-lang.org/inside-rust/2025/06/21/announcing-the-clippy-feature-freeze/
