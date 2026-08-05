+++
path = "2026/08/TODO/stabilizing-the-next-solver-trait-solver"
title = "Types Team Update and Roadmap"
authors = ["lcnr"]
aliases = ["2024/06/26/types-team-update.html"]

[extra]
team = "The Rustc Trait System Refactor Initiative"
team_url = "https://github.com/rust-lang/trait-system-refactor-initiative/"
+++

After nearly 4 years of active development [the next-generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113) is close to stabilization. We will enable it by default on nightly to surface any remaining issues and plan to stabilize it in the next months. This is the largest single change to the Rust compiler since its initial release. It completely replaces the existing type system components responsible for proving where-clauses, normalizing associated types, and much more.

This is an internal component of `rustc` and the main benefits will come in the future as the removal of the old implementation will unblock features such a [Type Alias Impl Trait and Return Type Notation][RTN], adding new implicit default trait bounds, e.g. [`Move` and `Forget`], large future compile-time performance improvements and will enable us to fix [the remaining type system unsoundnesses][Project Zero]. Even so, there are already [a lot of intended behavior changes](https://github.com/rust-lang/rust/issues?q=is%3Aissue%20state%3Aopen%20label%3Afixed-by-next-solver). When developing on nightly going forward, you may accidentally rely on behavior only supported by the new trait solver.

While we've spent a lot of time working on the new trait solver, it is still not perfect. Some crates have significant compile time performance regressions, some diagnostics are significantly worse, and there are a multiple minor issues and behavior differences we are aware of. We expect that there are likely also a lot of issues we are not aware of yet.

We have not spent much time micro-optimizing the new trait solver yet, so its current performance results are quite mixed. Even so, compiling some type system heavy crates is already significantly faster. The [`projection-caching`](https://github.com/rust-lang/rustc-perf/tree/main/collector/compile-benchmarks/projection-caching) benchmark is 5x faster than with the existing implementation and the performance benefits are even more pronounced in extreme cases like [Type System Chess](https://github.com/Dragon-Hatcher/type-system-chess). We expect to improve the compile time performance of the new trait solver as we move on. This new implementation enables a lot of optimizations which would otherwise not have been possible.

Rust's type system is incredibly complex and this change big enough that some amount of breakage is unavoidable and partially intended. We are tracking the intended breakage [on GitHub](https://github.com/rust-lang/trait-system-refactor-initiative/issues/211). As the new trait solver is not being widely tested by users makes it easy for minor regressions to slip in while working on it. People also keep writing more and more code over time, increasing the cost of intended breakage over time.

Because of this, we plan to stabilize the new trait solver even while it is not perfect yet, even while it results in undesirable regressions and may allow some code we'd like to later forbid again. There will be slightly more breakage when stabilizing the trait solver and in the few release after, as we work on fixing issues introduced by the initial stabilization.

### The next steps

We [have tested the new trait solver](https://github.com/rust-lang/rust/pull/133502) on all accessible projects on GitHub and crates.io [via crater](https://github.com/rust-lang/crater) and have also spent a lot of effort to make sure any - hopefully temporary - compile-time performance regressions are acceptable. We will continue this work in parallel to the ongoing stabilization and are currently going through a final test run and will triage all the breakage found.

We plan to enable the next-generation trait solver on nightly by default within the next month. This is not a stabilization, but should expose a lot more users to the new implementation, surfacing bugs and issues not covered by crater and our own testing. While we very much expect this to uncover new minor problems, our expectation is that it doesn't uncover any fundamental issue of the current implementation. We will release another small blogpost at this point.

We will continue to fix issues and prepare the necessary documentation for the full stabilization and intend to then stabilize the next-generation trait solver after its been enabled on nightly for while. I expect this to happen before the end of October this year. Once the trait solver is fully stable, we can then remove support for the old implementation on nightly.

### How can I help?

While we intend to stabilize the new trait solver with known bugs, we do want to hear about any breaking changes and performance or diagnostics regressions and how they are affecting you. Please [open an issue on GitHub](https://github.com/rust-lang/rust/issues/new?template=bug_report.md) if you are encounter any such issues. We generally work to make updating to new Rust versions as smooth as possible and to retain compatibility with older versions where possible.

You can explicitly test your projects with a latest nightly and make sure the next-generation trait solver is used by setting `RUSTFLAGS=-Znext-solver` when invoking the compiler. Once the next-generation trait solver has been enabled by default, you can disable it via `RUSTFLAGS=-Znext-solver=no`.

In case you want to help out with the crater triage, bug fixes, or performance work, there are currently a bunch of open issues [tracked on zulip](https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor/topic/call.20for.20participation/with/613938653) or reach out to `@lcnr`. 

Thank you!

[`Move` and `Forget`]: https://rust-lang.github.io/rust-project-goals/2026/move-trait.html
[Project Zero]: https://rust-lang.github.io/rust-project-goals/2026/roadmap-project-zero.html
[RTN]: https://rust-lang.github.io/rust-project-goals/2026/rtn.html