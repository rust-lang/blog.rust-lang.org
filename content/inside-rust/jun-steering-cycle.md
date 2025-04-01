+++
path = "inside-rust/2022/06/03/jun-steering-cycle"
title = "Rust Compiler June 2022 Steering Cycle"
authors = ["Felix Klock"]
description = "The compiler team's June 2022 steering cycle"
aliases = ["inside-rust/2022/06/03/jun-steering-cycle.html"]

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++
On [Friday, June 3rd][jun-03-zulip-archive], the Rust Compiler team had a planning meeting for the June 2022 steering cycle.

[jun-03-zulip-archive]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings/topic/.5Bplanning.20meeting.5D.202022-06-03/near/284883023

**Note:** This schedule has changed since this post was first published: the dates of the first two meetings were revised.

### T-compiler June Steering Schedule

|           Date |            Meeting Id | Meeting Topic
|----------------|-----------------------|----------------
| [2022-06-10][] | [compiler-team#517][] | 2022 Q2 P-high review
| [2022-06-17][] | [compiler-team#516][] | path sanitisation changes [rfc#3127][]
| [2022-06-24][] | [compiler-team#484][] | Compiler Feature Backlog Bonanza
| [2022-07-01][] |         none          | (planning for July cycle)

[2022-06-10]: https://calendar.google.com/event?action=TEMPLATE&tmeid=NHY4Y3VmdXZqcWJxOWgzOXVyZWM5a3JjaWUgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[2022-06-17]: https://calendar.google.com/event?action=TEMPLATE&tmeid=Nm8xbGtqbHBzMjdpcTRjcHAybmw4a3Y0ZjEgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[2022-06-24]: https://calendar.google.com/calendar/event?eid=MmE2azQyb2ViODQ4NWwxMWViMzJka2g0cjIgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&ctz=GMT-04:00
[2022-07-01]: https://calendar.google.com/calendar/event?eid=MDJyYnJ1cGFtdWR1c2lnNjFmcHJ2b3JlODFfMjAyMjA3MDFUMTQwMDAwWiA2dTVycnRjZTZscnR2MDdwZmkzZGFtZ2p1c0Bn&ctz=GMT-04:00

[compiler-team#484]: https://github.com/rust-lang/compiler-team/issues/484
[compiler-team#516]: https://github.com/rust-lang/compiler-team/issues/516
[compiler-team#517]: https://github.com/rust-lang/compiler-team/issues/517

[rfc#3127]: https://github.com/rust-lang/rfcs/issues/3127

### Details

Every fourth Friday, the Rust compiler team decides how
it is going to use its scheduled steering and design meeting time over the next
three Fridays.

On Friday, 10 June, we will be doing a quarterly [review of the open P-high issues][compiler-team#517].
pnkfelix will do some ahead of time work triaging
some of the issues, and potentially prepare a meeting document summarizing the
remainder, to maximize the quality of our synchronous in-meeting time.

On Friday, 17 June, we will review [RFC PR #3127][rfc#3127], which proposes
new rustc and Cargo options to allow path sanitisation by default.

On Friday, 24 June, we will be having a ["backlog bonanza"][compiler-team#484], in a
similar vein to that done by T-lang, to review the list of unimplemented or
partially-implemented features. pnkfelix and Jack Huey will prepare a document
to drive the meeting.

On Friday, 1 July, we will hold our planning meeting for the next steering cycle in July.

Each meeting will run from 2pm to 3pm GMT, and will take place on the
[T-compiler/meetings zulip stream][zulip].

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings
