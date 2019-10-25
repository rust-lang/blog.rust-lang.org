---
layout: post
title: "Planning meeting update"
author: Niko Matsakis
description: "Planning meeting update"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

In our planning meeting today, the compiler team has scheduled our
next batch of upcoming design meetings:

* On Nov 1, we will discuss "incremental dep-graph storage" (see
  [rust-lang/compiler-team#199]), which is a plan to improve the
  performance of storing incremental compilation data, as well as
  reducing peak memory usage.
* On Nov 8, we will discuss "A unified framework for dataflow
  analysis" (see [rust-lang/compiler-team#202]), which is a proposal
  for how an improved dataflow analysis that can help with const
  evaluation.
* On Nov 15, we will do a "working group review" (see
  [rust-lang/compiler-team#187]), examining how well the working group
  system has been working and what we might do to improve it.

[rust-lang/compiler-team#202]: https://github.com/rust-lang/compiler-team/issues/202
[rust-lang/compiler-team#199]: https://github.com/rust-lang/compiler-team/issues/199
[rust-lang/compiler-team#187]: https://github.com/rust-lang/compiler-team/issues/187

### Did you know?

Most weeks, the compiler team has some sort of design meeting. These
meetings take place on Zulip and are open to all. Every 4 weeks, we do
a planning meeting to pick the next few meetings from the list of open
proposals. You can find [more details about how the compiler-team
steering meeting process here][details].

[details]: https://rust-lang.github.io/compiler-team/about/steering-meeting/
