+++
path = "inside-rust/2025/03/17/leadership-council-update"
title = "March 2025 Leadership Council Update"
authors = ["Eric Huss"]
aliases = ["inside-rust/2025/03/17/leadership-council-update.html"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council!
We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2024/12/09/leadership-council-update.html

## Accomplishments so far

### Team structure updates

We have moved several teams and added some new ones. Some of this is part of the ongoing work described in the [Shape of Rust](#shape-of-rust) section below.

The book team (which is responsible for [The Rust Programming Language book](https://doc.rust-lang.org/book/)) and the [Rust By Example](https://doc.rust-lang.org/rust-by-example/) team have been moved from the Launching Pad to subteams of the [lang-docs](https://www.rust-lang.org/governance/teams/lang#team-lang-docs) team. This is in effort to clean up the organization of the Launching Pad. [leadership-council#123](https://github.com/rust-lang/leadership-council/issues/123), [leadership-council#139](https://github.com/rust-lang/leadership-council/issues/139).

The Edition 2024 Project Group has been converted to the Edition Team as part of the Launching Pad. This new team has a charter of clearer responsibilities for running the edition process on an ongoing basis. [leadership-council#149](https://github.com/rust-lang/leadership-council/issues/149).

We approved the creation of the Mentorship team as a subteam of the Launching Pad. This new team is responsible for the Rust organization's participation in programs like [Google Summer of Code](https://summerofcode.withgoogle.com/). Details about Rust's participation in GSoC 2025 was [recently announced](https://blog.rust-lang.org/2025/03/03/Rust-participates-in-GSoC-2025.html). [leadership-council#153](https://github.com/rust-lang/leadership-council/issues/153), [leadership-council#146](https://github.com/rust-lang/leadership-council/issues/146).

We approved the creation of the [Goals team](https://rust-lang.github.io/rust-project-goals/admin/team.html) as a subteam of the Launching Pad. This team is responsible for running the project goals program. [leadership-council#150](https://github.com/rust-lang/leadership-council/issues/150)

### Program management

We approved reserving $200k (USD) of the Council's budget to hire for the [Program Management position](https://hackmd.io/VGauVVEyTN2M7pS6d9YTEA) ([leadership-council#151](https://github.com/rust-lang/leadership-council/issues/151)). This is initially intended to support the Goals and Edition programs. The Foundation is assisting with this process, and initial steps for advertising the position have started.

### All hands

Work continues for preparation of the all-hands event in May 2025 at [RustWeek 2025] which corresponds with Rust's 10-year anniversary. We discussed and approved several requests:

- We approved how to handle the invitations. [leadership-council#135](https://github.com/rust-lang/leadership-council/issues/135)
- We agreed to raise the cap for travel grants that the Foundation may automatically accept due to the expected higher costs of this event. [leadership-council#148](https://github.com/rust-lang/leadership-council/issues/148)
- We agreed that COVID safety should be a priority, and set up several expectations for the event. [leadership-council#147](https://github.com/rust-lang/leadership-council/issues/147)
- We approved guidelines for guest invites to the all-hands [leadership-council#158](https://github.com/rust-lang/leadership-council/issues/158)

[RustWeek 2025]: https://rustweek.org/

### Additional items

And a few other items:

- Approved the removal of the "experimental" status of the travel grant program [leadership-council#122](https://github.com/rust-lang/leadership-council/pull/122)
- Approved the changes for how license/copyright information is included in the Rust distributions [leadership-council#120](https://github.com/rust-lang/leadership-council/issues/120)
- Started the [March 2025 representative selection process](https://blog.rust-lang.org/inside-rust/2025/02/14/leadership-council-repr-selection.html)
- Finalized removal of core team infrastructure pieces [leadership-council#29](https://github.com/rust-lang/leadership-council/issues/29)
- Added clarifications for the Project Director selection process [leadership-council#121](https://github.com/rust-lang/leadership-council/pull/121), [leadership-council#134](https://github.com/rust-lang/leadership-council/pull/134)
- Clarified and approved the ask of the council in the [cryptographic verification and mirroring goal](https://rust-lang.github.io/rust-project-goals/2025h1/verification-and-mirroring.html) and the [all-hands goal](https://rust-lang.github.io/rust-project-goals/2025h1/all-hands.html).

## Ongoing work

There are various efforts underway on projects that have had significant discussions since the last update, but have not concluded with any decisions, yet.
They are:

### Shape of Rust

Work into the shape of Rust discussion has recently been centered around the Launching Pad. In particular, we have been discussing how to organize teams such as the [Security Response Working Group](https://github.com/rust-lang/leadership-council/issues/141) which have cross-cutting concerns across all teams in the organization. James Munns (Launching Pad representative) has been working on [a proposal](https://gist.github.com/jamesmunns/cb93f9577a4c99d7f5f319bb22b4a28f) which would reframe the Launching Pad into more of a permanent structure that would house teams with cross-cutting concerns across the organization. This proposal also includes the concept of a "Rust Society" ([leadership-council#159](https://github.com/rust-lang/leadership-council/issues/159)) [previously proposed](https://thejpster.org.uk/blog/blog-2024-02-09/) by Jonathan Pallant. The Rust Society would take on the role of housing community-oriented groups. James is continuing to work on and refine these ideas with the Council.

We have also recently received a request for a [GPU Working Group](https://github.com/rust-lang/leadership-council/issues/155) which we broke down into three separate concerns. First, project-focused work includes things like changes to the compiler and the language. Second, project collaboration with the community of developers interested in GPU support, and is something more akin to what we classify as "domain working groups" like the embedded working group. And third, industry collaboration which involves collaboration with industry partners where the Foundation may be more suited to support.

### Foundation budget

In addition to the [program management](#program-management) role discussed above, we also discussed ways we could potentially focus some of our funding on project inward-facing infrastructure support ([leadership-council#136](https://github.com/rust-lang/leadership-council/issues/136)). We noted that we do not have a good understanding of what the needs are of the project members when it comes to internal tooling and infrastructure. We also noted that funding this kind of role could be difficult since it would somehow need to be integrated into the project without adding burden to the project itself.

### Additional items

- We have been discussing more about the issue of communication and connection with the project directors and the Foundation. We previously had Mark Rousskov sharing a seat on both the Council and the Project Directors which provided a bridge to easily communicate between the groups. However, now the Mark is no longer a Director, we have lost that bridge. [leadership-council#41](https://github.com/rust-lang/leadership-council/issues/41#issuecomment-2587685025)
- We realized the website team is in a bit of limbo with understanding the responsibility about the content of the website, which had some expectations that this would get addressed at the Council level. [website production usages](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/website.20production.20usages)
- Project members have been discussing an [AI policy](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/AI.20policy) for the Rust org as a whole, but no specific proposal has arisen, yet.

## Meeting minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes].
Links to the minutes since our last update are:

* [December 20, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-12-20.md)
* [January 17, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-01-17.md)
* [January 31, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-01-31.md)
* [February 14, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-02-14.md)
* [February 28, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-02-28.md)

[minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes
