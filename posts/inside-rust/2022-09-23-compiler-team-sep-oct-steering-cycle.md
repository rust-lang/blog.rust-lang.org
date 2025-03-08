+++
layout = "post"
title = "Rust Compiler Early October 2022 Steering Cycle"
author = "Felix Klock"
description = "The compiler team's early October 2022 steering cycle"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++

On [Friday, September 23rd][sep-23-zulip-archive], the Rust Compiler team had a planning meeting for the September/October 2022 steering cycle.

[sep-23-zulip-archive]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings/topic/.5Bplanning.20meeting.5D.202022-09-23/near/300372578

### T-compiler June Steering Schedule

| Date           | Meeting Id            | Meeting Topic                         |
|----------------|-----------------------|---------------------------------------|
| [2022-09-30][] | [compiler-team#559][] | 2022 Q3 P-high review                 |
| [2022-10-07][] | [compiler-team#544][] | Survey retrospective                  |
| [2022-10-14][] | [compiler-team#540][] | Dealing with disabled tests           |
| [2022-10-21][] | none                  | (planning for October/November cycle) |

[2022-09-30]: https://calendar.google.com/event?action=TEMPLATE&tmeid=Mm9tM2VzOWszaWw0Z3RudWlhNzF0ZHMwbzMgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2022-10-07]: https://calendar.google.com/event?action=TEMPLATE&tmeid=NnA2bWtoaGQ5NnVudGkwdnM5aWZ1YmpqNG0gNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2022-10-14]: https://calendar.google.com/event?action=TEMPLATE&tmeid=NW52ZGNhYzVrbWJxdG0yOTBpN2Q2ZmExaXIgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2022-10-21]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MDJyYnJ1cGFtdWR1c2lnNjFmcHJ2b3JlODFfMjAyMjEwMjFUMTQwMDAwWiA2dTVycnRjZTZscnR2MDdwZmkzZGFtZ2p1c0Bn&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com&scp=ALL

[compiler-team#559]: https://github.com/rust-lang/compiler-team/issues/559
[compiler-team#544]: https://github.com/rust-lang/compiler-team/issues/544
[compiler-team#540]: https://github.com/rust-lang/compiler-team/issues/540

### Details

Every fourth Friday, the Rust compiler team decides how
it is going to use its scheduled steering and design meeting time over the next
three Fridays.

On Friday, 30 September, we will do a quarterly [review of the open P-high issues][compiler-team#559].
pnkfelix will do some ahead of time work [in zulip](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/reviewing.20P-high.20issues.202022.20.28Q3.29/near/300390068)
triaging
some of the issues, and potentially prepare a meeting document summarizing the
remainder, to maximize the quality of our synchronous in-meeting time.

On Friday, 7 October, we will do a [retrospective][compiler-team#544] on the 2022 T-compiler surveys
that were used to drive the February ambitions post and the midyear update post.
We will talk about the questions that were asked, what we wish had been asked,
what value we got from the survey as it went, and what to do the same and/or
differently in the future. (In principle, this meeting should inform the way
that we drive the retrospective on how the year as a whole went for the compiler
team.)

On Friday, 14 October, we will discuss how to [deal with "disabled tests"][compiler-team#540]:
tests that are turned off at some point (e.g. due to failures in a component we
do not control). We do not currently have any protocols in place to review such
tests or decide when to try to re-enabling them.

On Friday, 21 October, we will hold our planning meeting for the next steering
cycle in October and November.

Each meeting will run from 2pm to 3pm GMT, and will take place on the
[T-compiler/meetings zulip stream][zulip].

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings
