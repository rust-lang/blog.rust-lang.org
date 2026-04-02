+++
path = "inside-rust/2026/04/03/leadership-council-update"
title = "Leadership Council update — March 2026"
authors = ["Eric Huss"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council! We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2025/12/10/leadership-council-update/

## Accomplishments so far

### Welcome Rémy Rakic and Josh Triplett to the Council

The March representative selections have completed. There were several changes made to the Council:

- [Mara Bos] stepped down as the [libs team] representative to switch to be the [launching pad] representative.
- [Josh Triplett] has joined to take Mara's place as the [libs team] representative.
- [Rémy Rakic] has joined as the [compiler team] representative.
- [Eric Huss] remains as the [dev tools team] representative.

We'd like to thank our outgoing representatives [James Munns] and [Josh Stone] for their time on the Council. We've greatly appreciated their support!

Thanks to everyone who participated in the process! The next representative selections will be in September 2026 for the other half of the Council.

[Mara Bos]: https://github.com/m-ou-se/
[launching pad]: https://forge.rust-lang.org/governance/council.html#the-launching-pad-top-level-team
[libs team]: https://www.rust-lang.org/governance/teams/library
[Josh Triplett]: https://github.com/joshtriplett/
[Rémy Rakic]: https://github.com/lqd/
[compiler team]: https://www.rust-lang.org/governance/teams/compiler
[Eric Huss]: https://github.com/ehuss/
[dev tools team]: https://www.rust-lang.org/governance/teams/dev-tools
[James Munns]: https://github.com/jamesmunns
[Josh Stone]: https://github.com/cuviper/

### Welcome Nurzhan Saken as our second Program Manager

In February we welcomed [Nurzhan Saken] as our second Program Manager joining [Tomas Sedovic]. This will help make our PM program more sustainable and strengthen the support it provides. You can read more about Nurzhan in [the recent PM update](https://blog.rust-lang.org/inside-rust/2026/03/27/program-management-update-2026-02/#hello-from-nurzhan).

[Nurzhan Saken]: https://github.com/nxsaken/
[Tomas Sedovic]: https://github.com/tomassedovic/

### Project priorities budget

Since 2024, the Project has managed a *Project Priorities* budget. The council spends funds from this budget on activities that support the work of the Project. At the start of 2026, the Foundation allocated to this budget $306k (USD) in new funds and transferred $106k in remaining funds from the Project Grants program. For the past several months the Council has been working on allocating towards various priorities. The following items have been approved:

- Raised our travel grant limit to $100k ([#254](https://github.com/rust-lang/leadership-council/pull/254)). Note that in addition to this, the Foundation is providing an additional $50k for RustConf travel.
- Allocated an additional $160k for the Program Manager program, which combined with the remaining budget from 2025 gave us about $344k starting balance for the year. ([#255](https://github.com/rust-lang/leadership-council/issues/255))
- Allocated $25k for [Outreachy](https://www.outreachy.org/) mentorship ([#264](https://github.com/rust-lang/leadership-council/issues/264)).
- Allocated $55k for the compiler-ops contract ([#244](https://github.com/rust-lang/leadership-council/issues/244)).

In addition to these already allocated funds, we are discussing whether to reserve the remaining amount with the following:

- Funding the [Content Team](https://github.com/rust-lang/content-team) with $15k to help produce video content ([#279](https://github.com/rust-lang/leadership-council/pull/279)).
- Restarting the grants program with $100k to support existing maintainers ([RFC#3919](https://github.com/rust-lang/rfcs/pull/3919)).
- Considering an unspecified amount for web design support on [crates.io](https://crates.io/) ([#278](https://github.com/rust-lang/leadership-council/issues/278)).

### Additional items

- Finalized an alumni policy that involves an annual notification to members to let them know which teams they are a member of, and to check if they are still active. [#218](https://github.com/rust-lang/leadership-council/pull/218)
- Updated the responsibilities of Project Directors to better define and clarify expectations. [#250](https://github.com/rust-lang/leadership-council/pull/250)
- Clarified that the mentor team doesn't need to ask the Council to participate in [GSoC 2026](https://summerofcode.withgoogle.com/programs/2026). [#253](https://github.com/rust-lang/leadership-council/issues/253)
- The Foundation has updated the process for travel grant applications. [#259](https://github.com/rust-lang/leadership-council/pull/259)
- Approved a proposal for delegating guest invites for the All-Hands. [#252](https://github.com/rust-lang/leadership-council/issues/252)

## Ongoing work

### Rust Foundation Maintainers Fund

In December we formed a committee ([#248](https://github.com/rust-lang/leadership-council/pull/248)) to put together a proposal to define how to use the funds from the [Rust Foundation Maintainer Fund](https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/). The RFMF is a dedicated fund used to support Rust Project maintainers.

The team has been working very hard on a proposal which is now open for comments at [RFC#3931](https://github.com/rust-lang/rfcs/pull/3931).

### Contribution policy

A lively discussion has been taking place around adopting a policy for low-effort contributions, particularly those involving AI assistance ([#273](https://github.com/rust-lang/leadership-council/issues/273)). A second proposal was submitted and has since moved to [RFC#3936](https://github.com/rust-lang/rfcs/pull/3936) to gather comments from the project.

### Additional items

- [James Munns] has continued to work on the Rust Society proposal by collecting a group of interested parties to help administer the program. [#260](https://github.com/rust-lang/leadership-council/issues/260)

## Following our work

As you can see from the many links above, the work of the Council happens mostly in public on GitHub issues posted in [our repository](https://github.com/rust-lang/leadership-council/issues). The items on our meeting agendas are drawn from these. After discussing an item, we summarize that discussion and any shared [rationales](https://aturon.github.io/tech/2018/05/25/listening-part-1/) on the issue. When we make decisions, we propose on the issue a "final comment period" (FCP), and as with all FCPs in the Project, we're interested in and review any feedback that people have before or during this final comment period.

To follow our work in real time, watch our repository. You can also see meeting summaries posted on Zulip in [`#council > Meeting minutes & summaries`][summaries].

[summaries]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.20minutes.20.26.20summaries/with/561198432

## Meeting minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes]. Links to the minutes since our last update are:

- [December 5, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-12-05.md)
- [December 19, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-12-19.md)
- [January 16, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-01-16.md)
- [January 30, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-01-30.md)
- [February 13, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-02-13.md)
- [February 27, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-02-27.md)
- [March 13, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-03-13.md)
- [March 27, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-03-27.md)
