+++
path = "inside-rust/9999/12/31/rust-langrust-is-adopting-an-llm-policy"
title = "rust-lang/rust is adopting an LLM policy"
authors = ["Jynn Nelson"]
+++

Recently, [five teams in the Rust project][read-only FCP link] adopted a [policy][forge-page] that I originally authored,
governing how [Large Language Models][llms] can be used when contributing to the `rust-lang/rust` monorepo.
Notably, the new policy is *not* an official stance on LLMs, and does *not* apply everywhere in the Rust project.
I wrote it for a very specific purpose, described below.

[llms]: https://en.wikipedia.org/wiki/Large_language_model

This post talks about why we ratified that policy, what it says, and how this will affect contributors.

The policy affects the following groups of people:
- People who review or moderate PRs on `rust-lang/rust`.
- People who author PRs with LLM-generated code on `rust-lang/rust`.
- People who discover issues using LLMs and post them on `rust-lang/rust`.
- People who write issues or comments that directly quote an LLM on `rust-lang/rust`.

If you are not in one of those groups, you don't have to change anything about how you work.

## Why was this policy created?

While the Rust project is a collection of technical artifacts,
it is also a *community* of people who work together to build, maintain, and extend those artifacts.
When we talk about "contributing to the Rust project", we partly mean work on those artifacts,
but we also mean joining that community and collaborating with the people already there.

Even before this policy was created, people were using LLMs to contribute to rust-lang/rust.
Some of those uses respected our community:
translating messages to English so people could draft them in their native language;
finding poor diagnostics for code snippets that new contributors to Rust might write;
analyzing RFCs to see if they were missing a discussion of other parts of the language that could affect the design.
Some of those uses, sometimes unintentionally, did not.

I've seen LLMs causing three main issues for our community:
1. Polished technical products no longer indicate effort and understanding.
2. Making code easier to write exacerbates our existing issues with review bandwidth.
3. People mechanically copy-pasting to and from an LLM is a waste of our time.

As time went on, these issues grew and grew, until we had to create dedicated channels and moderation policy for how to deal with them.
However, those channels work against our goals of being transparent and welcoming, because new contributors have no idea what the rules are.

The new policy formalizes those rules publicly, so that new contributors know how to join our community without getting their PRs closed for reasons they don't understand,
and so that existing reviewers can easily point to the rules as an actionable reason when closing PRs that don't follow them.

### Technical products no longer indicate effort

It used to be that if an open source project got a polished, well-tested, detailed PR,
that indicated that there was someone on the other end who had put time, effort, and understanding into the PR.
That influenced Rust's culture in several ways:
- We are generally reluctant to close PRs, since they represent someone else's hard work.
- Our process emphasizes incremental discussions, where PRs are allowed to change existing design if we discover new facts during creation or review.
- We treat PRs as an indication that someone is interested in joining our community and being mentored to work on future PRs.

With LLMs, none of these signals are reliable.
Polished PRs no longer indicate effort;
authors of polished PRs no longer necessarily understand their code—and in the case of autonomous agents, there is no longer someone on the other end at all;
and because it's become so much easier to write code, a polished PR no longer indicates that someone is likely to stick around for the long term.

### Making code easier to write causes review issues

At the time of writing, there are **1,281 open PRs** to rust-lang/rust.
This represents a staggering amount of time invested by both authors and reviewers.
We have long had the problem that there are more people who want to write code than people willing to review it.
With the advent of LLMs, this problem only gets worse.

Most of the work of reviewing is not simply catching bugs.
A great deal of it is deciding *whether this direction is a good approach*, whether the PR is a good idea at all.
In other words, [reviewing is made of decisions](https://web.archive.org/web/20260213080731/https://siderea.dreamwidth.org/1219758.html).

"Shotgunning" PRs at reviewers incurs a high mental cost for them.
I think most authors of LLM PRs believe that they are sincerely helping, but from our perspective,
the code itself is the smallest and in some ways least important part of the change.
We care much more about authors *understanding* what the code does, *planning* how it will change in the future, and *deciding* what it should look like.
The code itself cannot help with any of those.

### Mechanically copy-pasting LLM output is a waste of time

We will often get people who respond to review comments by copy-pasting them into their LLM, then copy-pasting its response back onto GitHub.
Bluntly: **this is a waste of everyone's time**.
If we wanted an LLM's opinion, we could have asked it ourselves.
We want to hear *your* thoughts, not a machine's.

Moreover, this is a breach of trust between the reviewer and the author.
Our assumption when we review is that we're talking to a real person who wants to do their best work.
Pasting LLM text creates suspicion: does the author actually care? Is there a person here at all?

### So why a policy?

Before this policy, we had a "wild west" approach to moderation.
We had dozens of LLM PRs; no disclosure rules; people trying to add risky [MIR optimizations] as their first PR;
and people posting "Verification: [`git diff --check`][diff-check]" in their PR description as if that did something.
While we had *something* for moderators to point to in the form of ["Empower reviewers to reject burdensome PRs"](https://triagebot.infra.rust-lang.org/gh-comments/rust-lang/compiler-team/issues/893),
our enforcement was inconsistent and our rules were not published anywhere.
In practice, the rule was "anything goes, as long as it's not *obviously* horrible".
Compared to the previous situation, the new policy is both much more strict and much more clear.

[MIR optimizations]: https://rustc-dev-guide.rust-lang.org/mir/optimizations.html
[diff-check]: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---check

Regardless of your opinions on whether LLMs are good, bad, or a secret third thing, they can no longer be *ignored*.
Our choices are not "no policy" or "policy".
Our choice is whether to have the policy be an unofficial list of moderation notes or something we stand by publicly.

Why not ban LLMs altogether, or allow any use of LLMs that we think are pro-social?
Because Rust governance doesn't work that way.
We do not have a benevolent dictator who can say
["No LLM-generated content, whether it be code or prose."](https://ziglang.org/code-of-conduct/#strict-no-llm-no-ai-policy)
or ["AI is a tool, just like other tools we use"](https://lore.kernel.org/linux-media/CAHk-=wi4zC+Ze8e+p3tMv8TtG_80KzsZ1syL9anBtmEh5Z40vg@mail.gmail.com/).

Rust operates by consensus. As the policy says:

> There is not a consensus within the Rust project—and likely never will be—about when/how/where it is acceptable to use AI-based tools.
> Many members of the Rust project and community find value in AI;
> many others feel that its negative impact on society and the climate are severe enough that no use is acceptable.
> Still others are working out their opinion.
>
> Despite these differences, there are many values we all share:
>
> - Building a community of deep experts in our collective projects.
> - Building an inclusive community where all feel welcome and respected.

We want it to be possible to change the policy in the future.
The policy has several provisions that make it easier to change than it was to originally adopt.
The leadership council is also considering creating a sub-team that would handle LLM policy so we have fewer "nightmare" 30-person approval requirements.

I do not think every rule in this policy is wholly good.
I *do* think that writing our rules down is better than not writing them down,
and that having a policy that everyone kinda dislikes pushes us to improve our governance structures.

## What does the policy say?

The shortest summary of the policy is this:

> It's fine to use LLMs to answer questions, analyze, distill, refine, check, suggest, review. But not to **create**.

Uses in the first category are allowed, sometimes requiring disclosure.
Uses in the second category are heavily restricted.

### General rules

No one except the author is required to read LLM output unless they choose to:
LLM output isn't allowed in public docs, PR descriptions, and comments unless it's clearly marked;
reviewers aren't required to look at LLM PRs if they don't want to.

Avoid making LLMs a requirement to contribute to `rust-lang/rust`:
policies must be written first for humans, and only summarized for machines;
LLM reviews cannot substitute for human review or self-review.

You are allowed to generate LLM content that only you see, without disclosure,
as long as you do not post it anywhere that you expect us to read or review.

Disclosure is required for machine translation, "trivial" changes, discovering bugs, and reviewing other people's work using an LLM.
We welcome messages posted in your native language; English translation is not required to contribute.

There are very strict guidelines on LLM-generated code changes:
> Pre-arranged, non-critical, high-quality, well-tested, and well-reviewed code changes that are originally created by an LLM are allowed, **with disclosure**.

The policy holds LLM-generated changes to a *higher* bar than human-authored changes, not a lower one:
LLM PRs are required to have tests, full stop, regardless of how hard that is, as well as various other restrictions;
LLMs must not generate soundness-critical changes unless the author is already a domain expert, and even then it's strongly discouraged.

In general, the policy focuses on *understanding*, helping to ensure that we have a mental model of our code, not just artifacts that mechanically do the right thing.
Our motivation is influenced by [Profession by Isaac Asimov][asimov].
No programmer tapes.

[asimov]: https://web.archive.org/web/20201109034130/https://www.abelard.org/asimov.php

### Moderation rules

**You must disclose LLM-generated content.**
You can choose to not post LLM content, or you can choose to post it and disclose its origin.
You may not hide LLM involvement.

**Harassment is not allowed.**
You may not harass people for using an LLM, regardless of whether or not their use is banned by the policy.
You must follow the [Code of Conduct] at all times when interacting with the Rust project.

[Code of Conduct]: https://rust-lang.org/policies/code-of-conduct/

See [the policy itself][forge-page] for more information.

Some parts of the policy are unenforceable.
This is not a bug.
The goal is *not* to catch every violation, but to create a clear bright-line rule:
Disclosure is required for all public LLM text, unless specifically exempted by the policy.
This allows moderators to identify violations based on *actions*, not on intent, and only consider intent when deciding how to respond.

## How does this affect contributors?

### Issue reporters

You must disclose any LLM involvement in discovering or reporting issues.
You must tell us if you found an issue using an LLM.
You must clearly quote and indicate which parts of your report were LLM-generated;
the "no LLM-generated comments" rule applies to you too.

### Authors who post LLM-generated code

I have written a list of guidelines you should follow if you make a PR to `rust-lang/rust` with LLM-generated code.
You can avoid thinking about them, or indeed reading the list at all, if you follow this simple guideline from the policy:

> It's fine to use LLMs to answer questions, analyze, distill, refine, check, suggest, review. But not to **create**.

See the ["Allowed" section of the policy](https://forge.rust-lang.org/policies/llm-usage.html) for a full list of what's meant by that.
See [rustc-dev-guide][llm-writing] for the full list of guidelines.

[llm-writing]: https://rustc-dev-guide.rust-lang.org/llm-guidance/writing.html

### Reviewers

#### General review

You are allowed to close PRs that don't follow the policy, no questions asked.
Please point the author to [#llm-mentoring] at the same time.
See [the dev guide][llm-reviewing] for exact circumstances and suggested wording.

You are not responsible for determining whether a PR is LLM-generated;
that responsibility lies with the author.
We will add a PR template that asks authors whether their code was LLM-generated so that this rarely comes up.

If an author claims their code is not LLM-generated, but you're still not sure, please report the PR *privately* to moderation.
Style is not evidence; please do not accuse people of using an LLM.
Reporting is not intended to be a penalty; the mod team is interested in seeing non-violations as well as violations.

#### Review of LLM code

If you have volunteered to review LLM PRs, the following section applies to you.
No one is required to review LLM PRs unless they volunteer.

Everyone is expected to follow the new policy, not just authors.
That means it is **your responsibility** to check whether an LLM-created PR touches an area that's disallowed by the policy,
such as docs, diagnostics, or soundness-critical changes.
You may request that the author redo it without LLM-generated code, in which case this section doesn't apply.

There's a more detailed summary of the rules you're expected to enforce in [the dev guide][llm-reviewing].
The [official policy][forge-page] remains canonical.

## What next?

Quite a lot of work has been put into this by moderators, team leads, reviewers, council representatives, and various other people inside and outside the project.
Some of that work started months ago before the policy itself was written.
I'd like to thank everyone who contributed, whether directly or indirectly.

This is not the end of the story.
One of the goals of the policy is to help us gather data:
Are people doing interesting and useful things with LLMs? Are they learning? Are they making repeat contributions?
The answers to those questions will help us determine how the policy changes in the future.

This is not the first LLM policy published by teams in the Rust project,
and hopefully it will not be the last.
While the current policy only applies to the `rust-lang/rust` monorepo,
I still believe that Rust would benefit from a project-wide policy that specifies what we expect in chats, forums, public communications, repositories without an explicit policy, and other cross-project areas.

[llm-reviewing]: https://rustc-dev-guide.rust-lang.org/llm-guidance/reviewing.html
[forge-page]: https://forge.rust-lang.org/policies/llm-usage.html
[#llm-mentoring]: https://rust-lang.zulipchat.com/join/rlfvpemsaacs3pfi6kwqnqjb/
[read-only FCP link]: https://triagebot.infra.rust-lang.org/gh-comments/rust-lang/rust-forge/pull/1040#issuecomment-4438128685
