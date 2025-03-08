+++
layout = "post"
date = 2024-08-12
title = "Rust Project goals for 2024"
author = "Niko Matsakis"
team = "Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>"
+++

With the merging of [RFC #3672][], the Rust project has selected a **slate of 26 Project Goals** for the second half of 2024 (2024H2). This is our first time running an [experimental new roadmapping process][RFC #3614]; assuming all goes well, we expect to be running the process roughly every six months. Of these goals, we have designated three of them as our **flagship goals**, representing our most ambitious and most impactful efforts: (1) finalize preparations for the Rust 2024 edition; (2) bring the Async Rust experience closer to parity with sync Rust; and (3) resolve the biggest blockers to the Linux kernel building on stable Rust. As the year progresses we'll be posting regular updates on these 3 flagship goals along with the 23 others.

[RFC #3672]: https://github.com/rust-lang/rfcs/pull/3672#issuecomment-2254599176
[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614

## Rust’s mission

All the goals selected ultimately further Rust's mission of **empowering everyone to build reliable and efficient software**. Rust targets programs that prioritize

* reliability and robustness;
* performance, memory usage, and resource consumption; and
* long-term maintenance and extensibility.

We consider "any two out of the three" to be the right heuristic for projects where Rust is a strong contender or possibly the best option, and we chose our goals in part so as to help ensure this is true.

## Why these particular flagship goals?

[**2024 Edition.**](https://rust-lang.github.io/rust-project-goals/2024h2/Rust-2024-Edition.html) 2024 will mark the 4th Rust edition, following on the 2015, 2018, and 2021 editions. Similar to the [2021 edition](https://github.com/nikomatsakis/rfcs/blob/rfl-project-goal/text/3085-edition-2021.html), the 2024 edition is not a "major marketing push" but rather an opportunity to correct small ergonomic issues with Rust that will make it overall much easier to use. The [changes planned](https://github.com/rust-lang/rust/issues?q=label%3AC-tracking-issue+label%3AA-edition-2024) for the 2024 edition include (1) supporting `-> impl Trait` and `async fn` in traits by aligning capture behavior; (2) permitting (async) generators to be added in the future by reserving the `gen` keyword; and (3) altering fallback for the `!` type. The [plan][2024plan] is to finalize development of 2024 features this year; the Edition itself is planned for Rust v1.85 (to be released to beta 2025-01-03 and to stable on 2025-02-20).

[2024plan]: https://rust-lang.github.io/rust-project-goals/2024h2/Rust-2024-Edition.html#milestones

[**Async.**](https://rust-lang.github.io/rust-project-goals/2024h2/async.html) In 2024 we plan to deliver several critical async Rust building block features, most notably support for *async closures* and *`Send` bounds*. This is part of a multi-year program aiming to raise the experience of authoring "async Rust" to the same level of quality as "sync Rust". Async Rust is widely used, with 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications. 

[**Rust for Linux.**](https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html) The [experimental support for Rust development in the Linux kernel][RFL] is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet today that support rests on a [number of unstable features][RFL#2], blocking the effort from ever going beyond experimental status. For 2024H2 we will work to close the [largest gaps that block support][rflgaps].

[RFL]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2
[rflgaps]: https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html#the-next-six-months

## Highlights from the other goals

In addition to the flagship goals, the roadmap defines [23 other goals][]. Here is a subset to give you a flavor:

* [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2024h2/cargo-script.html), allowing single-file Rust scripts that embed dependencies; owned by [Ed Page](https://github.com/epage).
* [Scalable Polonius support on nightly](https://rust-lang.github.io/rust-project-goals/2024h2/Polonius.html), improving Rust's borrow checker to [support conditional returns and other patterns](https://blog.rust-lang.org/inside-rust/2023/10/06/polonius-update.html); owned by [Rémy Rakic](https://github.com/lqd).
* [Move parallel front end closer to stability](https://rust-lang.github.io/rust-project-goals/2024h2/parallel-front-end.html), improving Rust compilation times by as much as 20%; owned by [Sparrow Li](https://github.com/SparrowLii).
* [Ergonomic ref counting](https://rust-lang.github.io/rust-project-goals/2024h2/ergonomic-rc.html), reducing the syntactic overhead of working with reference-counted data; owned by [Jonathan Kelley](https://github.com/jkelleyrtp).
* [Implementing "merged doctests"](https://rust-lang.github.io/rust-project-goals/2024h2/merged-doctests.html), which combine doctest files into one test to save testing time; owned by [Guillaume Gomez](https://github.com/GuillaumeGomez).

Check out [the whole list][23 other goals]! (Go ahead, we'll wait, but come back here afterwards!)

[23 other goals]:https://rust-lang.github.io/rust-project-goals/2024h2/index.html#project-goals

## How to track progress

As the year progresses, we will be posting regular blog posts summarizing the progress on the various goals. If you'd like to see more detail, the [2024h2 milestone][m] on the [rust-lang/rust-project-goals][] repository has tracking issues for each of the goals. Each issue is assigned to the [owner(s)][o] of that particular goal. You can subscribe to the issue to receive regular updates, or monitor the [#project-goals channel](https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals) on the [rust-lang Zulip](https://rust-lang.zulipchat.com). Over time we will likely create other ways to follow along, such as a page on rust-lang.org to visualize progress (if you'd like to help with that, reach out to @nikomatsakis, thanks!).

[rust-lang/rust-project-goals]: https://rust-lang.github.io/rust-project-goals/
[m]: https://github.com/rust-lang/rust-project-goals/milestone/2
[o]: https://rust-lang.github.io/rust-project-goals/about/owners.html

It's worth stating up front: **we don't expect all of these goals to be completed**. Many of them were proposed and owned by volunteers, and it's normal and expected that things don't always work out as planned. In the event that a goal seems to stall out, we can either look for a new owner or just consider the goal again in the next round of goal planning.

## How we selected project goals

Each project goal began as a PR against the [rust-lang/rust-project-goals][] repository. As each PR came in, the goals were socialized with the teams. This process sometimes resulted in edits to the goals or in breaking up larger goals into smaller chunks (e.g., a far-reaching goal for ["higher level Rust"](https://github.com/rust-lang/rust-project-goals/pull/10) was broken into two specific deliverables, a [user-wide build cache](https://rust-lang.github.io/rust-project-goals/2024h2/user-wide-cache.html) and [ergonomic ref counting](https://rust-lang.github.io/rust-project-goals/2024h2/ergonomic-rc.html)). Finally, the goals were collated into [RFC #3672][], which listed each goals as well as all the asks from the team. This RFC was approved by all the teams that are being asked for support or other requests.

## Conclusion: Project Goals as a "front door" for Rust

To me, the most exciting thing about the Project Goals program has been seeing the [goals][1] [coming][2] [from][3] [outside][4] the existing Rust maintainers. My hope is that the Project Goal process can supplement RFCs as an effective "front door" for the project, offering people who have the resources and skill to drive changes a way to float that idea and get feedback from the Rust teams *before* they begin to work on it.

[1]: https://rust-lang.github.io/rust-project-goals/2024h2/cargo-semver-checks.html
[2]: https://rust-lang.github.io/rust-project-goals/2024h2/Rust-for-SciComp.html
[3]: https://rust-lang.github.io/rust-project-goals/2024h2/user-wide-cache.html
[4]: https://rust-lang.github.io/rust-project-goals/2024h2/std-verification.html

Project Goals also help ensure the sustainability of the Rust open source community. In the past, it was difficult to tell when starting work on a project whether it would be well-received by the Rust maintainers. This was an obstacle for those who would like to fund efforts to improve Rust, as people don't like to fund work without reasonable confidence it will succeed. Project goals are a way for project maintainers to "bless" a particular project and indicate their belief that it will be helpful to Rust. The Rust Foundation is using project goals as one of their criteria when considering [fellowship applications](https://foundation.rust-lang.org/grants/fellowships/), for example, and I expect over time other grant programs will do the same. But project goals are useful for others, too: having an approved project goal can help someone convince their employer to give them time to work on Rust open source efforts, for example, or give contractors the confidence they need to ensure their customer they'll be able to get the work done.

The next round of goal planning will be targeting 2025H1 and is expected to start in October. We look forward to seeing what great ideas are proposed!
