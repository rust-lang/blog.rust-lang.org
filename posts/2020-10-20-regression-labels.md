---
layout: post
title: "Marking issues as regressions"
description: "Now anyone can mark issues as regressions!"
author: Camelid
team: the release team <https://www.rust-lang.org/governance/teams/operations#release>
---

If you've contributed to Rust before, you have likely used a command like this
to [set labels][rustbot-labeling-docs]:

> **@rustbot** modify labels: A-typesystem C-enhancement

Setting labels on issues and pull requests helps us keep track of and filter
issues. There are many kinds of labels you can set to mark an issue
as pertinent to a particular team, related to a part of the compiler, or
what kind of bug it is (a hang, a crash, etc.).

We also have labels that help us keep track of regressions so that we can
prioritize them and fix them quickly if they are severe. We have three
regression labels, each representing when the regression occurred:

* `regression-from-stable-to-nightly`
* `regression-from-stable-to-beta`
* `regression-from-stable-to-stable`

We recently had a case where a [regression was not caught][internals-thread]
before a release because the issue was not marked with a regression label.
So we have now [added the ability][regression-label-pr] for *anyone* to set
regression labels on issues! This is all you have to do to mark an issue as a
regression and it will automatically be prioritized:

> **@rustbot** modify labels: regression-from-stable-to-{release-channel}

Just replace `{release-channel}` with one of `nightly`, `beta`, or `stable`,
depending on when the regression occurred, and you're good to go!

Alternatively, if you are reporting a new regression, you can use the regression
[issue template]. It will guide you through the process of reporting a
regression and providing information that will help us fix the issue.

Finally, if you have an issue that is *not* a regression, but is still something
that is important to be fixed, you can request prioritization with:

> **@rustbot** prioritize

We really appreciate it if you mark all regressions with an appropriate label
so we can track them and fix them as soon as possible!

[rustbot-labeling-docs]: https://rustc-dev-guide.rust-lang.org/rustbot.html#issue-relabeling
[internals-thread]: https://internals.rust-lang.org/t/1-46-is-unusable-for-me-solved/13161/10
[regression-label-pr]: https://github.com/rust-lang/rust/pull/77555
[issue template]: https://github.com/rust-lang/rust/issues/new/choose
