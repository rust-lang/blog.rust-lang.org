---
layout: post
title: "Marking issues as regressions"
description: "Now anyone can mark issues as regressions!"
author: Camelid
team: the release team <https://www.rust-lang.org/governance/teams/operations#release>
---

The Rust project gets many issues filed every day, and we need to keep track
of them all to make sure we don't miss anything. To do that we use GitHub's
issue labels feature, and we need your help to make sure we fix regressions
as soon as possible!

We have many issue labels that help us organize our issues, and we have a few
in particular that mark an issue as a regression. These labels will ping a Rust
working group called the [*prioritization working group*][wg-prioritization],
whose members will work to determine the severity of an issue and then
prioritize it. But, this won't happen unless someone marks the issue with one
of those labels!

We recently had a case where a [regression was not caught][internals-thread]
before a release because the issue was not marked with a regression label.
So we have now [added the ability][regression-label-pr] for *anyone* to set
regression labels on issues! This is all you have to do to mark an issue as a
regression and it will automatically ping people to prioritize it:

> **@rustbot** modify labels: regression-untriaged

Alternatively, if you are reporting a new regression, you can use the regression
[issue template]. It will guide you through the process of reporting a
regression and providing information that will help us fix the issue.

Finally, if you have an issue that is *not* a regression, but is still something
that is important to be fixed, you can request prioritization with:

> **@rustbot** prioritize

We really appreciate it if you mark all regressions with an appropriate label
so we can track them and fix them as soon as possible!

[wg-prioritization]: https://rust-lang.github.io/compiler-team/working-groups/prioritization
[internals-thread]: https://internals.rust-lang.org/t/1-46-is-unusable-for-me-solved/13161/10
[regression-label-pr]: https://github.com/rust-lang/rust/pull/77555
[issue template]: https://github.com/rust-lang/rust/issues/new/choose
