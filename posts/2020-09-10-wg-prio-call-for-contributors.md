---
layout: post
title: A call for contributors from the WG-prioritization team
author: The Rust WG-Prioritization Team
release: false
---

Are you looking for opportunities to contribute to the Rust community? Have some spare time to donate? And maybe learn something interesting along the way?

The [WG-prioritization][wg-prio] can be the right place for you: we are looking for new contributors!

## What is the WG-prioritization?

The Prioritization WG is a compiler working group dedicated to handling the most important bugs found in `rustc`, to ensure that they are resolved. We do triaging on the Github rust lang issue tracker, mainly deciding if bugs are critical (potential release blockers) and prepare the weekly agenda for the Compiler Team with the most pressing issues to be taken care of.

Here is a bit more [comprehensive description][wg-prio]. How we work is detailed [on the Rust Forge](https://forge.rust-lang.org/compiler/prioritization.html).

Our tooling is mainly the [triagebot](https://github.com/rust-lang/triagebot), a trustful messenger that helps us by sending notification to our [Zulip stream][zulip-wg-prio] when an issue on Github is labelled.

We also have a repository with some [issues and meta-issues](https://github.com/rust-lang/compiler-team-prioritization/issues), where we basically note down how we would like our workflow to evolve. Contributions to these issues are welcome, but a bit more context about the workflow of this Working Group is probably necessary.

Documentation is also a fundamental part of the onboarding package that we provide to newcomers. As we basically "organize and sort stuff", a lot happens without writing a single line of code but rather applying procedures to optimize triaging and issues prioritization.

This requires our workflow to be as efficient and well documented as possible. As such, we are always open to contributions to clarify the documentation (and fresh eyeballs are especially precious for that!).

## The typical week of a WG-prioritization member

Our week starts on Thursday/Friday after the Rust Compiler Team meeting (one of the cool teams that keep that beast at bay). We summarize the meeting and draft an agenda for the following one.

In the following days the WG-prioritization and other teams will asynchronously monitor the issue tracker - everyone at their own pace, when time allows - trying to assign a priority to new issues. This greatly helps the various teams (compiler team, standard library team, documentation team and so on) to sort and prioritize their work.

If the issue priority is not immediately clear, it will be tagged with a temporary label and briefly discussed on Zulip by the WG-prioritization: is this issue critical? Is it clear? Does it need a [minimal reproducible example](https://stackoverflow.com/help/minimal-reproducible-example) (often abbreviated in `MCVE`) or even better a [git bisect](https://github.com/rust-lang/cargo-bisect-rustc) to find a regression (we love contributors bisecting code)?

The day before the meeting the agenda is finalized and handed to the Compiler Team.

Someone from the WG-Prioritization will then attend the meeting and provide details (if needed) and take down notes.

Rinse and repeat for the next meeting.

Everything is described in excruciating detail on [Rust Forge](https://forge.rust-lang.org/compiler/prioritization/procedure.html). Please have a look there to learn more.

## How can I contribute?

- **Help with triaging compiler issues**: helping keeping the issue tracker tidy is very important for any big project. Labeling and pinging people to work on MCVEs or bisection is very helpful to resolve any issue, and is required for our next task:
- **Help with issues prioritization**: keep an eye on the messages on our Zulip stream (~10 issues a week) and cast a vote on what the priority should be. Analyze the issue, figure out how the release could be impacted. More votes balance the prioritization. A couple of made-up examples:
  - **ICE (Internal Compiler Error) on Rust stable channel for Tier3 supported platform**: unless it impacts Tier 1 platforms will likely be labelled as `P-low` (low priority). Will probably ping the relevant team and that's it until someone will pick it.
  - **A "hello world" kind of code snippet fails to compile in the latest nightly**: that's a critical issue (`P-critical`) and we must stop this regression before it gets into the `beta` or (gosh!) `stable` channel. The relevant team will be swiftly alerted.
  - **Obscure ICE when using some library**: if the issue doesn't smell bad, try assigning a `P-medium` and suggest the reporter to attach a `MCVE`.

    With some experience, you will develop an _instinct_ to prioritize issues :-)

- **Help properly summarize issues in the agenda**: what is this issue about? What has been already done to frame a context? Is is a regression? We add any detail that could be relevant to the Compiler team during their meeting. These folks are busy and could use all the help to get the context of an issue at a glance.

## Ok, but can I actually contribute? I don't feel skilled enough

Yes, you are! There will always be one or more members available to explain, mentor and clarify things. Don't be shy and do not refrain from asking questions. You will very quickly be able to give a helpful opinion in our discussions.

Everyone can contribute on their capacity and availability.

## Where do we hang out

One of the great things of the Rust governance is its openness. Join our stream [#t-compiler/wg-prioritization][zulip-wg-prio] and peek at how we work and if you want, also feel free to keep an eye to the weekly Team Compiler official meetings on [#t-compiler/meetings](https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings). You can even simply just hang out on our Zulip stream, see how things work and then get involved where you feel able.

The main contact points for this working group are Santiago Pastorino (`@Santiago Pastorino` on Zulip) and Wesley Wiser (`@Wesley Wiser` on Zulip).

See you there!

[wg-prio]: https://rust-lang.github.io/compiler-team/working-groups/prioritization
[zulip-wg-prio]: https://rust-lang.zulipchat.com/#narrow/stream/227806-t-compiler.2Fwg-prioritization
