+++
layout = "post"
title = "Lang team October update"
author = "Niko Matsakis"
description = "Lang team October update"
team = "the lang team <https://lang-team.rust-lang.org/>"
+++

This week the lang team held its October planning meeting ([minutes]). We hold these meetings on the first Wednesday of every month. 

The planning meeting is used for:

* Checking in on the status of our [active initiatives]
* Planning the design meetings for the remainder of the month

After each meeting, we post an update (like this one!) with notes and meeting announcements. 

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-10-06-Planning-meeting.md

[active initiatives]: https://lang-team.rust-lang.org/initiatives.html

## Update from active initiatives

What follows are the updates that were submitted this month. Note that many of the more recent initiatives have dedicated repositories which contain other byproducts of the design process (e.g., evaluation documents, open design questions, etc). Take a look!

* [Async fundamentals update](https://rust-lang.github.io/async-fundamentals-initiative/updates/2021-oct.html):
    * **What is it?** Async fn in traits, async drop, async closures
    * Have designated an [MVP](https://rust-lang.github.io/async-fundamentals-initiative/roadmap/mvp.html) version of async functions in traits that we intend to stabilize first, and done a lot of exploration on next steps (read up on that in the ever evolving [evaluation doc](https://rust-lang.github.io/async-fundamentals-initiative/evaluation.html)).
* [Impl trait initiative update](https://rust-lang.github.io/impl-trait-initiative/updates/2021-oct.html)
    * **What is it?** `type Foo = impl Trait` at the module and impl level.
    * oli-obk has completed a rewrite of the inference engine to better match the design proposed the RFC, and it is slowly being merged in
    * We are working on explainer plus stabilization doc to "Type alias impl trait".
    * We would like to add syntax to give names for function types, which unblocks "impl trait in traits"
* [Dyn upcasting initiative update](https://rust-lang.github.io/dyn-upcasting-coercion-initiative/updates/2021-oct.html)
    * **What is it?** Ability to cast `dyn Foo` to `dyn Bar` if `trait Foo: Bar`
    * Made good progress, need to resolve a soundness question and then ready to merge.
* [Generic associated type initiative update](https://rust-lang.github.io/generic-associated-types-initiative/updates/2021-oct.html)
    * **What is it?** Ability to have associated types with generic parameters, such as `type Foo<'me>`, in traits.
    * Resolved the question of where clause defaults by deciding on a conservative, if not maximally ergonomic, path.
    * Making continued progress towards something we can stabilize. The intent is to start with some known ergonomic shortcomings and build from there.
* [Let else update](https://github.com/rust-lang/rust/issues/87335#issuecomment-933672440)
    * **What is it?** Generalization of `let` to permit you to match against something and panic, return, etc when match fails:
        * `let Ok(x) = something else panic!()`
    * Implementation available on nightly, may be feature complete
* [Deref patterns update](https://github.com/rust-lang/lang-team/issues/88#issuecomment-935056996)
    * **What is it?** Applying Deref impls in the context of a match, such as `match rc { Some(x) => ..., None => ... }`
    * Decided to build prototype that does not have any explicit syntax for deref and works against "known safe" std types.
* [Never type stabilization update](https://github.com/rust-lang/lang-team/issues/60#issuecomment-935233842)
    * **What is it?** The never type `!`[^never]
    * We have landed an improved, if complex, analysis that allows us to choose between the current behavior (fallback to `()`) and the new behavior (fallback to `!`) without breaking important existing code. The hope is to proceed with stabilization and slowly improve and simplify the rules using warnings, editions, or other tools.

In addition to the reports above, you'll find more detailed discussing in the [minutes].

[^never]: The never type is called never because it will *never* be stabilized. Ha! I kill me. --nikomatsakis

## Upcoming design meetings

We have planned the following design meetings:

* October 13: Syntax for where clauses in GATs and type aliases ([lang-team#120](https://github.com/rust-lang/lang-team/issues/120))
* October 20: Safety considerations for dyn upcasting ([lang-team#119](https://github.com/rust-lang/lang-team/issues/119))
* October 27: Forbidding unwinding from drop impls ([lang-team#97](https://github.com/rust-lang/lang-team/issues/97))

## Design meeting expectations

* The document for the meeting must be prepared by the triage meeting on Tuesday and posted to the tracking issue.
    * If it is not sent out by then, the meeting will be canceled. This gives folks 24 hour notice.
* There is no expectation that people will read the document before the meeting. The meeting will begin with a recap of the document.
    * However, there is no rule **against** reading the document beforehand and providing feedback or advice on how to improve it.
