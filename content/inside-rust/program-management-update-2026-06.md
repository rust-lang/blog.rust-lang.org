+++
path = "inside-rust/2026/07/14/program-management-update--june-2026"
title = "Program management update — June 2026"
authors = ["Tomáš Šedovič"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++
# Program management update — June 2026

## Meetings

We started attending the Types meetings. The topic is something neither of us are deeply familiar with, but the team was extremely welcoming, and kept stressing that we ask questions any time we can't follow something. To help us better understand recent topics, they pointed us to the [Well-formedness article on the Rust Compiler Development Guide page](https://rustc-dev-guide.rust-lang.org/analysis/well-formed.html).

The Rustdoc team considered having virtual face-to-face meetings in place of the async ones they've been having over Zulip. We joined, and minuted the meeting, but the team decided to stick with async for now. They did raise an interest in doing something akin to the Language Design meetings — and they would appreciate a PM presence there.

Finally, the Types team started a [library trait evolution experiment](https://rust-lang.github.io/rust-project-goals/2026/library-trait-evolution-experiment.html) to explore designs for evolving the standard library types (e.g. [supertrait auto-impl](https://rust-lang.github.io/rust-project-goals/2026/supertrait-auto-impl.html), [`Sized` hierarchy](https://rust-lang.github.io/rust-project-goals/2026/scalable-vectors.html), and [immobile types](https://rust-lang.github.io/rust-project-goals/2026/move-trait.html)). There is a [Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/612373-t-lang.2Flibrary-trait-evolution) for discussion, and a [recurring meeting](https://github.com/rust-lang/calendar/pull/123) we set up.

## Project goals

### Funding

Each goal can now add a section requesting funding (e.g., [improve `rustc_codegen_cranelift` performance](https://rust-lang.github.io/rust-project-goals/2026/improve-cg_clif-performance.html#funding)). Roadmaps now list the funding information across all their goals (e.g., [Fast Builds](https://rust-lang.github.io/rust-project-goals/2026/roadmap-fast-builds.html#funding)). There is also a [funding overview across all goals](https://rust-lang.github.io/rust-project-goals/2026/funding.html).

If you have a Project goal that needs financial support, add a funding section according to the [template](https://github.com/rust-lang/rust-project-goals/blob/a2d4a0ca701794550cb6072ce0ed355c53bb3f83/src/TEMPLATE.md?plain=1#L101). If you're interested in supporting a goal financially, you may reach out to the contact listed on the [funding page](https://rust-lang.github.io/rust-project-goals/2026/funding.html).

### Help wanted

We also have a page for goals that are [looking for contributors](https://rust-lang.github.io/rust-project-goals/2026/contributors.html), analogous to the funding page. If you're interested in joining an effort, please check it out, and contact the goal owner.

If your goal needs help, add a `Help wanted` section using the [template](https://github.com/rust-lang/rust-project-goals/blob/a2d4a0ca701794550cb6072ce0ed355c53bb3f83/src/TEMPLATE.md?plain=1#L89).

### New goals

We've accepted several new goal proposals:

* [Reintroduce a FCW to borrowck](https://rust-lang.github.io/rust-project-goals/2026/borrowck-fcw.html)
* [Allow turbofishing late bound vars](https://rust-lang.github.io/rust-project-goals/2026/turbofishing-late-bound.html)
* [C interop: f80, f128 and c_longdouble](https://rust-lang.github.io/rust-project-goals/2026/interop-f80-f128.html)
* [Library Trait Evolution Experiment](https://rust-lang.github.io/rust-project-goals/2026/library-trait-evolution-experiment.html)
* [View types experiment](https://rust-lang.github.io/rust-project-goals/2026/view-types-experiment.html)

We're considering new Project goal proposals year-round! Here's [how to propose a new goal](https://rust-lang.github.io/rust-project-goals/how_to/propose_a_goal.html). Once you open the pull request, we'll work with you to make sure everything is set.

Note that we still require that the teams with asks on them sign off, and that teams with medium or larger asks have a champion. This is a way to make sure the teams aren't over-committing their capacity.

We're also looking for ways to streamline the current process, let teams handle their goals more directly, and make larger long-term improvements.

## Leadership Council

We'd like to highlight a few developments that recently came out of the Leadership Council meetings.

As a reminder: the meetings happen every two weeks on Friday at 11:30 New York time. We publish [meeting notes in the Council repo](https://github.com/rust-lang/leadership-council/tree/main/minutes/sync-meeting) and [summaries on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.20minutes.20.26.20summaries/with/609194999). The draft agenda is usually [posted](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.202026-07-03/with/609195027) on the Monday before the meeting.

Keep in mind that the Council meetings [may be observed](https://github.com/rust-lang/leadership-council/blob/main/procedures/meeting-observers.md) by all Project members — if you'd like to join, talk to your [representative](https://rust-lang.org/governance/teams/leadership-council/).

If you don't know who your representative is, look your name up in the [All Rust team members page](https://rust-lang.org/governance/people/), then click on the teams you're active in, which should get you to the top-level teams (e.g. Compiler, Language, Library, Launching pad, Infrastructure, etc.), and then look up the current representative on the [Leadership Council team page](https://rust-lang.org/governance/teams/leadership-council/).

### T-comprehensibility

[David Wood proposed chartering a Comprehensibility team](https://github.com/rust-lang/leadership-council/issues/298#issuecomment-4650707850):

> Comprehensibility is a long-standing deficiency of the Rust Project. New contributors and organisations interested in participating in the project have a difficult time understanding how to contribute, and cross-team collaboration is impeded by mismatched expectations and working practices. Teams retain the ultimate decision making power about the content of their charters, process and policies, but t-comprehensibility will be involved in the drafting and updating to ensure that the project's documentation is consistent and coherent overall.

Documentation (especially process documentation) is something that must be actively maintained, and without a sustained effort it's among the first things to become stale. Having a team of individuals with an interest to keep it updated sounds like a great idea.

The [proposal](https://github.com/rust-lang/leadership-council/issues/298) has finished its final comment period (FCP) and is expected to merge. The team creation will follow.

### The next All Hands

[Mara Bos proposed](https://github.com/rust-lang/leadership-council/issues/300), and the [Council accepted](https://github.com/rust-lang/leadership-council/issues/300#issuecomment-4642685220), to hold the next All Hands in 2027, and collocate it with RustWeek, organized by RustNL, once again. They have also agreed to reserve $50,000 for the All Hands out of the next year's Council budget, if there is money left after PMs are paid. This will be in addition to funding the travel grants (again, contingent on the availability of funds).

Based on the feedback received from the All Hands attendees, this is fantastic news for a huge number of people.

### LLM committee

oli proposed [creating an LLM committee](https://github.com/rust-lang/leadership-council/issues/308) that would have the power (delegated by the Council) to write and edit the LLM policy for the Project. It would be composed of 4-5 people representing the diverse views on the topic across the Project, be trusted by the Council and the Project, and have the ability to work together and make progress despite disagreement.

The team is explicitly designed such that everyone should feel comfortable reaching out to at least one committee member. It should also be able to adapt to the frequent changes in the AI industry without causing too much churn.

This is very much under open, active discussion. It will need a careful read and consideration. We're happy with this development and hope it works out.

On a personal note, I (Tomáš) would love to see Rust moving towards having a clearer policy on the matter, and as a project, we have not done so thus far. I'm curious to see what happens here. I hope we all manage to stay on topic on the issue (this is about *how* we move forward, not *what* the policy should be and *why*).

### Program management

There were two proposals concerning Program management opened in the last couple of weeks:

* [Discuss long-term place for Program Management program](https://github.com/rust-lang/leadership-council/issues/307) by TC
* [Charter Program Management Team](https://github.com/rust-lang/leadership-council/issues/309) by Rémy Rakic (lqd)

The first one focuses on who's responsible for the funding and management of the Program managers (presently Nurzhan and Tomáš, and possibly Antonio Piraino serving as compiler-ops). Josh Triplett suggests this be the same team that would manage the Maintainers in Residence.

The second one is concerned with chartering the PM team, listing their responsibilities and expectations. People management is work that plenty of people (e.g. TC, Niko Matsakis, Eric Huss) have done before we joined, and it will keep happening. The proposal acknowledges this and formalizes the role.

## Rust for Linux

Our regular meetings resumed after the All Hands.

Miguel Ojeda mentioned that [zerocopy](https://github.com/google/zerocopy) is now the first (third-party) crate they [vendor](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/rust/zerocopy) that can directly generate code in the kernel image \o/.

The team would also like to have a [runtime toggle in Rustdoc that would let them show private and hidden items](https://github.com/rust-lang/rust/issues/149106) in the documentation. Rustdoc can show this information already, but only at build-time using the `--document-private-items` and `--document-hidden-items`. We've opened a [discussion on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/Runtime.20toggle.20for.20private.2Fhidden.20items/with/606458092), but it hasn't been resolved yet.

Finally, Benno Lossin shared an [example](https://github.com/BennoLossin/field-projections-designs/blob/5bb55ed33ad8bbb947be25a0661bbe003c7353c0/examples/simple/src/main.rs) of what field access would desugar to using the Field projection mechanism of places and handles. This is important because it lets us validate it on existing field projections (e.g. for shared and mutable references), and see exactly what any proposed syntax for extending this to other kinds of pointers would do in normal Rust code that we ship today.

## Rust for CPython

Following the discussion at the All Hands, the CPython folks [met with us on 2026-07-02](https://docs.google.com/document/d/1mh6Ven8yRtPhTnI45Zl-ow2xhN1X8CEH-dvmpaH8Z_U) to discuss Project goals.

We came up with some goals worth considering:

* Rust in CPython in general (it might make sense for this to be a Roadmap)
* Project build design space (building things without using Cargo)
* Thread state memory
* Stabilizing macro metavar
* Stabilizing the Rust dylib format

In some cases, people are either currently working on or have looked into similar areas in the past, so the team will have those conversations first and then start opening goals.

## Worth a look

June (and the early bits of July) was extremely prolific in terms of blog posts and announcements!

### Project updates

* [How Josh helps Rust manage code across multiple repositories](https://blog.rust-lang.org/inside-rust/2026/06/04/how-josh-helps-rust-manage-code-across-multiple-repositories/)
* [April & May 2026 Project Director Update](https://blog.rust-lang.org/inside-rust/2026/07/01/project-director-update/)
* [Rustup update: our plans for the 1.30 release cycle](https://blog.rust-lang.org/inside-rust/2026/07/03/rustup-update-1.30/)
* [Leadership Council update — June 2026](https://blog.rust-lang.org/inside-rust/2026/07/06/leadership-council-update/)
* [Together for a healthier Clippy](https://blog.rust-lang.org/inside-rust/2026/07/06/unite-for-clippy/)
* [Maintainer spotlight: Gen Li (@rami3l)](https://blog.rust-lang.org/inside-rust/2026/07/07/maintainer-spotlight-gen-li-rami3l/)
* [The many journeys of learning Rust](https://blog.rust-lang.org/2026/06/25/vision-doc-journeys-to-learning-rust/)
* [Announcing Rust 1.96.1](https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/)
* [Announcing Rust 1.97.0](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/)

### Foundation posts

* [An AI Security Engineer in Residence for the Rust Ecosystem](https://rustfoundation.org/media/an-ai-security-engineer-in-residence-for-the-rust-ecosystem/)
    * Congratulations, Jacob!
* [Building the Rust Commercial Network: A Home for Rust in Production](https://rustfoundation.org/media/building-the-rust-commercial-network-a-home-for-rust-in-production/)
* [Rust Foundation Welcomes OpenAI As Platinum Member, Announces Donation to Rust Project](https://rustfoundation.org/media/rust-foundation-welcomes-openai-as-platinum-member-announces-donation-to-rust-project/)
* [On OpenAI’s Support for Rust](https://rustfoundation.org/media/on-openais-support-for-rust/)
* [Rust Foundation Member Announcement: Integer 32, Convex, Renesas, Peeriot, & Processing Foundation](https://rustfoundation.org/media/rust-foundation-member-announcement-integer-32-convex-renesas-peeriot-processing-foundation/)
* [Rust Commercial Network Launches to Bring Commercial Users of Rust Language Together](https://rustfoundation.org/media/rust-commercial-network-launches-to-bring-commercial-users-of-rust-language-together/)
* [Rust Foundation Trusted Training Program Launches, Giving Learners a Mark of Quality to Trust](https://rustfoundation.org/media/rust-foundation-trusted-training-program-launches-giving-learners-a-mark-of-quality-to-trust/)
* [Why (& How) We Built the Rust Foundation Trusted Training Program](https://rustfoundation.org/media/why-how-we-built-the-rust-foundation-trusted-training-program/)
* [Welcoming Wild to the Rust Innovation Lab](https://rustfoundation.org/media/welcoming-wild-to-the-rust-innovation-lab/)
* [Project Director Update — April & May 2026](https://rustfoundation.org/media/project-director-update-april-may-2026/)
* [Recap: Learning Rust from Scratch with Mainmatter at Barcelona’s First-Ever Upskilling Week](https://rustfoundation.org/media/recap-learning-rust-from-scratch-with-mainmatter-at-barcelonas-first-ever-upskilling-week/)
* [Strengthening Rust Infrastructure Security with Ubuntu Pro](https://rustfoundation.org/media/strengthening-rust-infrastructure-security-with-ubuntu-pro/)
* [Rust Foundation Trusted Training Spotlight: Mainmatter](https://rustfoundation.org/media/rust-foundation-trusted-training-spotlight-mainmatter/)

## Stats

Total words of meeting minutes written: 662k (June 2025–June 2026)

Meetings attended: 40

Total words of meeting minutes written (May): 68.1k

Average (mean) word count per team meeting:

* Cargo: 2.1k
* Lang triage: 3.0k
* Libs-API: 5k
* Leadership Council: 2.1k
