+++
layout = "post"
date = 2024-06-26
title = "Types Team Update and Roadmap"
author = "lcnr"
team = "The Types Team <https://github.com/rust-lang/types-team>"
+++

It has been more than a year since [the initial blog post][TypesAnnouncement] announcing the Types team, and our initial set of goals. For details on what the team is, why it was formed, or our previously-stated overarching goals, go check out that blog post. In short the Types team's purview extends to the parts of the Rust language and compiler that involve the type system, e.g. type checking, trait solving, and borrow checking. Our short and long term goals effectively work to make the type system sound, consistent, extensible, and fast.

Before getting into details, it's worth sharing a quick point: the team over the last year has been very successful. Oftentimes, it's hard to measure impact, particularly when long-term roadmap goals are hard to quantify progress on and various short-term goals either are hit or aren't. But, there is one clear statistic that is somewhat indicative of the team's progress: over the last year or so, [more than 50 user-facing changes][FCPs] have landed, each separately approved by Types Team consensus through FCP.

The changes lie at the boundary between language design and implementation, and the Types Team (which is a subteam of both the Language and Compiler Teams) existing means that not only does the Rust Project have the bandwidth to make these decisions but we also have enough people with the knowledge and experience of the type system to make informed decisions that overall make the language better.

## The priorities of the types team

To evaluate our progress over the last year and our roadmap going forward,
lets start with our main priorities in order of importance. We will refer
to them during the remainder of this post. To reach our goals, we need a
a healthy group of maintainers which have the expertise and capacity to
react to issues and to implement complex changes.

### The type system should be Sound

One of the main promises of Rust is that there cannot be undefined behavior when using
only safe code. It might surprise you that there are currently [known type system
bugs][unsoundCAT] which break these guarantees. Most of these issues were found by people familiar with
the inner workings of the compiler by explicitly looking for them and we generally do not expect
users to encounter these bugs by accident. Regardless, we deeply care about fixing them
and are working towards a fully sound and ideally verified type system.

### The type system should be Consistent

The type system should be easy to reason about. We should avoid rough edges and
special-cases if possible. We want to keep both the implementation and user-facing behavior
as simple as possible. Where possible we want to consider the overall design instead of
providing local targeted fixes. This is necessary to build trust in the soundness of the
type system and allows for a simpler mental model of Rust.

### The type system should be Extensible

Rust is still evolving and we will be required to extend the type system to enable new
language features going forward. This requires the type system to be extensible and
approachable. The design of the language should not be adapted to work around
short-comings of its current type system implementation. We should collaborate with
other teams and users to make sure we're aware of their problems and consider possible
future extensions in our implementation and design.

### The type system should be Fast

We care about the compile times of Rust and want to consider the impact on compile times
of our designs. We should look for effective approaches to speed up the existing implementation,
by improving caching or adding fast paths where applicable. We should also be aware of the
compile time impact of future additions to the type system and suggest more performant
solutions where possible.

## Updates

We have been very active over the last year and made some significant progress. There
are also a few non-technical updates we would like to share.

## Organizational updates

First, a huge welcome to the two new members to team since the announcement post: [@BoxyUwU] and [@aliemjay]. [@BoxyUwU] has been doing a lot of work on const generics and made significant contributions to the design of the next generation trait solver. [@aliemjay] has been working on some very subtle improvements to opaque types - `impl Trait` - and to borrow checking. They are both invaluable additions to the team.

We also organized another in-person Types Team meetup last October, immediately prior to EuroRust. We discussed the state of the team, looked at current implementation challenges and in-progress work, and reviewed and updated [the roadmap from the previous meetup][PrevRoadmap]. Most of this will be covered in this blog post.

Finally, as discussed in the [RFC](https://rust-lang.github.io/rfcs/3254-types-team.html), we would like to have leads rotate out regularly, largely to help share the burden and experience of leads' work. So with that being said, [@nikomatsakis](https://github.com/nikomatsakis) is rotating out and [@lcnr](https://github.com/lcnr) is joining to co-lead alongside [@jackh726](https://github.com/jackh726/).

## Roadmap progress and major milestones

### The next-generation trait solver

There has been [a lot of work][NewSolver] on the [next-generation trait solver][InitiativeRepo].
The initiative posted [a separate update][InitiativeUpdate] at the end of last year. While
we would have liked to [stabilize its use in coherence][StabilizeNS] a few months ago,
this surfaced additional small behavior regressions and hangs, causing delays. We are working on fixing these issues and intend to merge the stabilization PR soon. We are getting close to compiling the standard library
and the compiler with the new solver enabled everywhere, after which will be able to run
crater to figure out the remaining issues. We expect there to be a long tail of minor issues
and behavioral differences from the existing implementation, so there's still a lot to do
here. There are also open design questions which we will have to resolve before stabilizing
the new implementation.

### Async and `impl Trait`

We stabilized `async`-fn in traits (AFIT) and return-position `impl Trait` in
traits (RPITIT) in version 1.75 thanks to a significant effort by [@compiler-errors] and
[@spastorino]. [@cjgillot] greatly improved the way generators, and therefore async functions,
are represented in the type system[^107421]. This allowed us to support recursive
`async`-functions without too much additional work[^117703].

Designing the next-generation trait solver surfaced issues and future-compatibility challenges
of our type-alias `impl Trait` (TAIT) implementation using the old trait solver. We are
currently reworking the design and implementation. [@oli-obk] is spear-heading this effort.
This also impacts RPIT edge-cases, forcing us to be careful to avoid accidental breakage.
There are some open language design questions for TAIT, so we plan to
stabilize associated type position `impl Trait` (ATPIT) as it avoids these language design
questions while still closing the expressiveness gap.

### `a-mir-formality`

We made limited progress on [`a-mir-formality`] over the last year, mostly
because we were able to allocate less time than expected towards this work.
We have used it as the foundation towards an intuitive approach to
coinductive traits which are necessary for many of the remaining unsound
issues.

### Fixing soundness issues

We fixed multiple long-standing unsound issues, see [the full list of closed issues](https://github.com/rust-lang/rust/issues?q=is%3Aissue+label%3AI-unsound+label%3AT-types+-label%3Arequires-nightly+is%3Aclosed+closed%3A%3C2024-06-20+). The most most notable of which was [#80176](https://github.com/rust-lang/rust/issues/80176). This subtle issue caused us to accept methods in trait implementations whose function signature had outlives requirements not present in the trait definition. These requirements were then never proven when calling the trait method. As there were some crates which relied on this pattern by accident, even if it their usages didn't exploit this unsoundness, we first merged a [future-compatibility lint](https://github.com/rust-lang/rust/issues/105572) which we then moved to a hard error after a few versions.

We've also spent time on [categorizing the remaining open issues][unsoundCat] and integrating
them into our longterm planning. Most of the remaining ones are blocked on the
next-generation trait solver as fixing them relies on coinductive trait semantics
and improvements to implied bounds. There are some remaining issues which can be at
least partially fixed right now, and we intend to work through them as we go.
Finally, there are some issues for which we still haven't figured out the best
approach and which require some further considerations.

## Going forward

We made significant progress during the last year but we are not done! This section covers our goals for the rest of 2024. For each item we also link to the project goals that we have proposed as part of the Rust Project's [experimental new roadmap process](https://blog.rust-lang.org/inside-rust/2024/05/07/announcing-project-goals.html).

### `-Znext-solver`

* [Next-generation trait solver project goal](https://rust-lang.github.io/rust-project-goals/2024h2/next-solver.html)

Our biggest goal is to use the [next-generation trait solver][InitiativeRepo]
everywhere by default and to fully replace the existing implementation. We are currently
finalizing the stabilization of [its use in coherence checking][StabilizeNS]. This should
already fix multiple unsound issues and fix a lot of smaller issues and inconsistencies of
the current implementation. See the stabilization report for more details.

We are also working on extracting its implementation into a separate library
outside of the compiler itself. We would like to share the trait solver with
rust-analyzer by the end of this year. They currently use [chalk] which is no longer
actively maintained. Using the next-generation trait solver in rust-analyzer
should result in a lot of additional testing for the solver while also improving
the IDE experience by positively impacting performance and correctness.

We intend to slowly roll out the solver in other areas of the compiler until we're able
to fully remove the existing implementation by the end of 2025. This switch will fix
multiple unsound issues by itself and will unblock a significant amount of future work.
It will generally cleanup many rough edges of the type system, such as associated types
in higher-ranked types. There are many unsound issues which can only be fixed once we exclusively
use the new implementation. 

### `a-mir-formality`

* [`a-mir-formality` project goal](https://rust-lang.github.io/rust-project-goals/2024h2/a-mir-formality.html)

We intend to more actively develop `a-mir-formality` this year to use it in our design process.
Using it to model parts of the type system has already been incredibly impactful and we want
to build on that. We are working on more effective testing of `a-mir-formality` by enabling its
use for actual Rust code snippets and by adding fuzzing support. This will allow us to gain
additional confidence in our model of the type system and will guide its future development.

We plan to fully formalize some components of the type system this year. Coherence is fairly
self-contained, very subtle, and soundness-critical. This has prevented us from making significant
improvements to it in the past. We also intend to formalize coinductive trait semantics, which are
difficult to reason about and necessary to fix many longstanding soundness issues.

### Language changes and polonius

* [Associated Type Position Impl Trait (ATPIT) project goal](https://rust-lang.github.io/rust-project-goals/2024h2/ATPIT.html)
* [Polonius on Nightly project goal](https://rust-lang.github.io/rust-project-goals/2024h2/Polonius.html)

We intend to get the internal implementation of opaque types ready for the stabilization
of TAIT and ATPIT this year. We are also hoping to land significant improvements to our
handling of associated types in coherence checking this year.

Our other goal is to get [Polonius], the next generation borrow checker, available on nightly, which would put us in a position to stabilize in 2025 once we have time to do more optimization and testing.

[polonius]: https://blog.rust-lang.org/inside-rust/2023/10/06/polonius-update.html

We also intend to support the development of other language features, such as `async`-closures, which are part of the [async project goal](https://rust-lang.github.io/rust-project-goals/2024h2/async.html),
and `dyn`-trait upcasting, which will hopefully get stabilized in the near future.

## Roadmap

### EOY 2024

- next-generation trait solver
    - stable in coherence
    - used by rust-analyzer
- ATPIT stabilized
- a-mir-formality
    - support for fuzzing and testing Rust snippets
    - complete model of coherence and coinductive trait semantics
- full polonius implementation available on nightly 

### EOY 2025

- next-generation trait solver used everywhere by default
- TAIT stabilized
- polonius stabilized

## EOY 2027

- next-generation trait solver
    - support for coinduction and (implicit) where-bounds on `for<'a>`
    - enable perfect derive
- a-mir-formality fully model soundness critical parts of Rust
- all known type system unsoundnesses fixed


[TypesAnnouncement]: https://blog.rust-lang.org/2023/01/20/types-announcement.html
[PrevRoadmap]: https://blog.rust-lang.org/2023/01/20/types-announcement.html#roadmap
[InitiativeUpdate]: https://blog.rust-lang.org/inside-rust/2023/12/22/trait-system-refactor-initiative.html
[InitiativeRepo]: https://github.com/rust-lang/trait-system-refactor-initiative/
[devGuide]: https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html
[`a-mir-formality`]: https://github.com/rust-lang/a-mir-formality
[FCPs]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AT-types+label%3Adisposition-merge+is%3Amerged+closed%3A%3E2023-01-20+sort%3Acreated-asc+
[NewSolver]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+label%3AWG-trait-system-refactor+-label%3Arollup+is%3Amerged+closed%3A%3E2023-01-20+sort%3Acreated-asc+
[StabilizeNS]: https://github.com/rust-lang/rust/pull/121848
[unsoundCat]: https://github.com/orgs/rust-lang/projects/44/views/1
[chalk]: https://github.com/rust-lang/chalk/

[@aliemjay]: https://github.com/aliemjay
[@BoxyUwU]: https://github.com/boxyuwu
[@compiler-errors]: https://github.com/compiler-errors
[@oli-obk]: https://github.com/oli-obk
[@spastorino]: https://github.com/spastorino
[@cjgillot]: https://github.com/cjgillot

[^107421]: stabilized in <https://github.com/rust-lang/rust/issues/107421>
[^117703]: stabilized in <https://github.com/rust-lang/rust/issues/117703>
