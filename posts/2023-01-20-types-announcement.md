---
layout: post
title: "Officially announcing the types team"
author: Jack Huey
description: "An overview of the new types team"
team: The Types Team <https://github.com/rust-lang/types-team>
---

Oh hey, it's [another](https://blog.rust-lang.org/inside-rust/2022/09/29/announcing-the-rust-style-team.html) new team announcement. But I will admit: if you follow the [RFCs repository](https://github.com/rust-lang/rfcs/pull/3254), the [Rust zulip](https://rust-lang.zulipchat.com/#narrow/stream/144729-t-types), or were particularly observant on the [GATs stabilization announcement post](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html), then this *might* not be a surprise for you. In fact, this "new" team was officially established at the end of May last year.

There are a few reasons why we're sharing this post now (as opposed to months before or...never). First, the team finished a three day in-person/hybrid meetup at the beginning of December and we'd like to share the purpose and outcomes of that meeting. Second, posting this announcement now is just around 7 months of activity and we'd love to share what we've accomplished within this time. Lastly, as we enter into the new year of 2023, it's a great time to share a bit of where we expect to head in this year and beyond.

## Background - How did we get here?

Rust has grown significantly in the last several years, in many metrics: users, contributors, features, tooling, documentation, and more. As it has grown, the list of *things* people want to do with it has grown just as quickly. On top of powerful and ergonomic features, the demand for powerful tools such as IDEs or learning tools for the language has become more and more apparent. New compilers (frontend and backend) are being written. And, to top it off, we want Rust to continue to maintain one of its core design principles: safety.

All of these points highlights some key needs: to be able to *know* how the Rust language should work, to be able to *extend* the language and compiler with new features in a relatively painless way, to be able to *hook into* the compiler and be able to query important information about programs, and finally to be able to *maintain* the language and compiler in an amenable and robust way. Over the years, considerable effort has been put into these needs, but we haven't *quite* achieved these key requirements.

To extend a little, and put some numbers to paper, there are currently around 220 open tracking issues for [language](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue+label%3AT-lang), [compiler](https://github.com/rust-lang/rust/issues?page=1&q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue+label%3AT-compiler), or [types](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue+label%3AT-types) features that have been accepted but are not completely implemented, of which about half are at least 3 years old and many are several years older than that. Many of these tracking issues have been open for so long not solely because of bandwidth, but because working on these features is hard, in large part because putting the relevant semantics in context of the larger language properly is hard; it's not easy for anyone to take a look at them and know what needs to be done to finish them. It's clear that we still need better foundations for making changes to the language and compiler.

Another number that might shock you: there are currently 62 open [unsoundness issues](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-unsound). This sounds *much* scarier than it really is: nearly all of these are edges of the compiler and language that have been found by people who specifically poke and prod to find them; in practice these will not pop up in the programs you write. Nevertheless, these are edges we want to iron out.

## The Types Team

Moving forward, let's talk about a smaller subset of Rust rather than the entire language and compiler. Specifically, the parts relevant here include the type checker - loosely, defining the semantics and implementation of how variables are assigned their type, trait solving - deciding what traits are defined for which types, and borrow checking - proving that Rust's ownership model always holds. All of these can be thought of cohesively as the "type system".

As of [RFC 3254](https://rust-lang.github.io/rfcs/3254-types-team.html), the above subset of the Rust language and compiler are under the purview of the types team. So, what exactly does this entail?

First, since around 2018, there existed the "traits working group", which had the primary goal of creating a performant and extensible definition and implementation of Rust's trait system (including the [Chalk](https://github.com/rust-lang/chalk) trait-solving library). As time progressed, and particularly in the latter half of 2021 into 2022, the working group's influence and responsibility naturally expanded to the type checker and borrow checker too - they are actually strongly linked and its often hard to disentangle the trait solver from the other two. So, in some ways, the types team essentially subsumes the former traits working group.

Another relevant working group is the [polonius working group](https://rust-lang.github.io/compiler-team/working-groups/polonius/), which primarily works on the design and implementation of the [Polonius](https://github.com/rust-lang/polonius) borrow-checking library. While the working group itself will remain, it is now also under the purview of the types team.

Now, although the traits working group was essentially folded into the types team, the creation of a *team* has some benefits. First, like the [style team](https://blog.rust-lang.org/inside-rust/2022/09/29/announcing-the-rust-style-team.html) (and many other teams), the types team is not a *top level* team. It actually, currently uniquely, has *two* parent teams: the lang and compiler teams. Both teams have decided to delegate decision-making authority covering the type system.

The language team has delegated the part of the *design* of type system. However, importantly, this design covers less of the "feel" of the features of type system and more of how it "works", with the expectation that the types team will advise and bring concerns about new language extensions where required. (This division is not strongly defined, but the expectation is generally to err on the side of more caution). The compiler team, on the other hand, has delegated the responsibility of defining and maintaining the implementation of the trait system.

One particular responsibility that has traditionally been shared between the language and compiler teams is the assessment and fixing of soundness bugs in the language related to the type system. These often arise from implementation-defined language semantics and have in the past required synchronization and input from both lang and compiler teams. In the majority of cases, the types team now has the authority to assess and implement fixes without the direct input from either parent team. This applies, importantly, for fixes that are *technically* backwards-incompatible. While fixing safety holes is [not covered under Rust's backwards compatibility guarantees](https://blog.rust-lang.org/2014/10/30/Stability.html#what-are-the-stability-caveats), these decisions are not taken lightly and generally require team signoff and are assessed for potential ecosystem breakage with [crater](https://github.com/rust-lang/crater). However, this can now be done under one team rather than requiring the coordination of two separate teams, which makes closing these soundness holes easier (I will discuss this more later.)

## Formalizing the Rust type system

As mentioned above, a nearly essential element of the growing Rust language is to know how it *should* work (and to have this well documented). There are relatively recent efforts pushing for a Rust specification (like [Ferrocene](https://github.com/ferrocene/specification) or [this open RFC](https://github.com/rust-lang/rfcs/pull/3355)), but it would be hugely beneficial to have a formalized definition of the type system, regardless of its potential integration into a more general specification. In fact the existence of a formalization would allow a better assessment of potential new features or soundness holes, without the subtle intricacies of the rest of the compiler.

As far back as 2015, not long after the release of Rust 1.0, an experimental Rust trait solver called Chalk began to be written. The core idea of Chalk is to translate the surface syntax and ideas of the Rust trait system (e.g. traits, impls, where clauses) into a set of logic rules that can be solved using a Prolog-like solver. Then, once this set of logic and solving reaches parity with the trait solver within the compiler itself, the plan was to simply replace the existing solver. In the meantime (and continuing forward), this new solver could be used by other tools, such as rust-analyzer, where it is used today.

Now, given Chalk's age and the promises it had hoped to be able to deliver on, you might be tempted to ask the question "Chalk, when?" - and plenty have. However, we've learned over the years that Chalk is likely not the correct long-term solution for Rust, for a few reasons. First, as mentioned a few times in this post, the trait solver is only but a part of a larger type system; and modeling how the entire type system fits together gives a more complete picture of its details than trying to model the parts separately. Second, the needs of the *compiler* are quite different than the needs of a *formalization*: the compiler needs performant code with the ability to track information required for powerful diagnostics; a good formalization is one that is not only complete, but also easy to maintain, read, and understand. Over the years, Chalk has tried to have both and it has so far ended up with neither.

So, what are the plans going forward? Well, first the types team has begun working on a formalization of the Rust typesystem, currently coined [a-mir-formality](https://github.com/nikomatsakis/a-mir-formality/). An initial experimental phase was written using [PLT redex](https://redex.racket-lang.org/), but a Rust port is in-progress. There's a lot to do still (including modeling more of the trait system, writing an RFC, and moving it into the rust-lang org), but it's already showing great promise.

Second, we've begun an [initiative](https://github.com/rust-lang/types-team/issues/58) for writing a new trait solver in-tree. This new trait solver is more limited in scope than a-mir-formality (i.e. not intending to encompass the entire type system). In many ways, it's expected to be quite similar to Chalk, but leverage bits and pieces of the existing compiler and trait solver in order to make the transition as painless as possible. We do expect it to be pulled out-of-tree at some point, so it's being written to be as modular as possible. During our types team meetup earlier this month, we were able to hash out what we expect the structure of the solver to look like, and we've already gotten that [merged into the source tree](https://github.com/rust-lang/rust/pull/105661).

Finally, Chalk is no longer going to be a focus of the team. In the short term, it still may remain a useful tool for experimentation. As said before, rust-analyzer uses Chalk as its trait solver. It's also able to be used in rustc under an unstable feature flag. Thus, new ideas currently could be implemented in Chalk and battle-tested in practice. However, this benefit will likely not last long as a-mir-formality and the new in-tree trait solver get more usable and their interfaces become more accessible. All this is not to say that Chalk has been a failure. In fact, Chalk has taught us a lot about how to think about the Rust trait solver in a logical way and the current Rust trait solver has evolved over time to more closely model Chalk, even if incompletely. We expect to still support Chalk in some capacity for the time being, for rust-analyzer and potentially for those interested in experimenting with it.

## Closing soundness holes

As brought up previously, a big benefit of creating a new types team with delegated authority from both the lang and compiler teams is the authority to assess and fix unsoundness issues mostly independently. However, a secondary benefit has actually just been better procedures and knowledge-sharing that allows the members of the team to get on the same page for what soundness issues there are, why they exist, and what it takes to fix them. For example, during our meetup earlier this month, we were able to go through the full list of soundness issues (focusing on those relevant to the type system), identify their causes, and discuss expected fixes (though most require prerequisite work discussed in the previous section).

Additionally, the team has already made a number of soundness fixes and has a few more in-progress. I won't go into details, but instead am just opting to putting them in list form:

* [Consider unnormalized types for implied bounds](https://github.com/rust-lang/rust/pull/99217): landed in 1.65, no regressions found
* [Neither require nor imply lifetime bounds on opaque type for well formedness](https://github.com/rust-lang/rust/pull/95474): landed in 1.66, no regressions found
* [Add `IMPLIED_BOUNDS_ENTAILMENT` lint](https://github.com/rust-lang/rust/pull/105575): landing in 1.68, future-compat lint because many regressions found (of unsoundness)
* [Check ADT fields for copy implementations considering regions](https://github.com/rust-lang/rust/pull/105102): currently open, ready to land
* [Register wf obligation before normalizing in wfcheck](https://github.com/rust-lang/rust/pull/100046): currently open, regressions found, needs additional work
* [Handle projections as uncovered types during coherence check](https://github.com/rust-lang/rust/pull/100555): currently open, some regressions found, future-compat lint suggested
* [Don't normalize in AstConv](https://github.com/rust-lang/rust/pull/101947): landing in 1.68, 1 small regression found

As you can see, we're making progress on closing soundness holes. These sometimes break code, as assessed by crater. However, we do what we can to mitigate this, even when the code being broken is technically unsound.

## New features

While it's not technically under the types team purview to *propose and design* new features (these fall more under lang team proper), there are a few instances where the team is heavily involved (if not driving) feature design.

These can be small additions, which are close to bug fixes. For example, [this PR](https://github.com/rust-lang/rust/pull/104765) allows more permutations of lifetime outlives bounds than what compiled previously. Or, these PRs can be larger, more impactful changes, that don't fit under a "feature", but instead are tied heavily to the type system. For example, [this PR](https://github.com/rust-lang/rust/pull/100386) makes the `Sized` trait coinductive, which effectively makes more cyclic bounds compile (see [this test](https://github.com/rust-lang/rust/pull/100386/files#diff-7efe7060b98871be57269858d3abd0c9a6f877a6c65fd0fba54ef122cd2d5281) for an example).

There are also a few larger features and feature sets that have been driven by the types team, largely due to the heavy intersection with the type system. Here are a few examples:

* Generic associated types (GATs) - The feature long predates the types team and is the only one in this list that has actually been stabilized so far. But due to heavy type system interaction, the team was able to navigate the issues that came on its final path to stabilization. See [this blog post](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html) for much more details.
* Type alias impl trait (TAITs) - Implementing this feature *properly* requires a thorough understanding of the type checker. This is close to stabilization. For more information, see [the tracking issue](https://github.com/rust-lang/rust/issues/63063).
* Trait upcasting - This one is relatively small, but has some type system interaction. Again, see [the tracking issue](https://github.com/rust-lang/rust/issues/65991) for an explanation of the feature.
* Negative impls - This too predates the types team, but has recently been worked on by the team. There are still open bugs and soundness issues, so this is a bit away from stabilization, but you can follow [here](https://github.com/rust-lang/rust/issues/68318).
* Return position impl traits in traits (RPITITs) and async functions in traits (AFITs) - These have only recently been possible with advances made with GATs and TAITs. They are currently tracked under a single [tracking issue](https://github.com/rust-lang/rust/issues/91611).

## Roadmap

To conclude, let's put all of this onto a roadmap. As always, goals are best when they are specific, measurable, and time-bound. For this, we've decided to split our goals into roughly 4 stages: summer of 2023, end-of-year 2023, end-of-year 2024, and end-of-year 2027 (6 months, 1 year, 2 years, and 5 years). Overall, our goals are to build a platform to maintain a sound, testable, and documented type system that can scale to new features need by the Rust language. Furthermore, we want to cultivate a sustainable and open-source team (the types team) to maintain that platform and type system.

A quick note: some of the things here have not quite been explained in this post, but they've been included in the spirit of completeness. So, without further ado:

**6 months**
* The work-in-progress new trait solver should be testable
* a-mir-formality should be testable against the Rust test suite
* Both TAITs and RPITITs/AFITs should be stabilized or on the path to stabilization.

**EOY 2023**
* New trait solver replaces part of existing trait solver, but not used everywhere
* We have an onboarding plan (for the team) and documentation for the new trait solver
* a-mir-formality is integrated into the language design process

**EOY 2024**
* New trait solver shared by rustc and rust-analyzer
    * Milestone: Type IR shared
* We have a clean API for extensible trait errors that is available at least internally
* "Shiny features"
    * Polonius in a usable state
    * Implied bounds in higher-ranked trait bounds (see [this issue](https://github.com/rust-lang/rust/issues/90696) for an example of an issue this would fix)
    * Being able to use `impl Trait` basically anywhere
* Potential edition boundary changes

**EOY 2027**
* (Types) unsound issues resolved
* Most language extensions are easy to do; large extensions are feasible
* a-mir-formality passes 99.9% of the Rust test suite

## Conclusion

It's an exciting time for Rust. As its userbase and popularity grows, the language does as well. And as the language grows, the need for a sustainable type system to support the language becomes ever more apparent. The project has formed this new types team to address this need and hopefully, in this post, you can see that the team has so far accomplished a lot. And we expect that trend to only continue over the next many years.

As always, if you'd like to get involved or have questions, please drop by the [Rust zulip](https://rust-lang.zulipchat.com/#narrow/stream/144729-t-types).
