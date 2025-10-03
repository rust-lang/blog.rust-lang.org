+++
path = "inside-rust/2023/02/10/compiler-team-feb-steering-cycle"
title = "Rust Compiler February 2023 Steering Cycle"
authors = ["Felix Klock"]
description = "The compiler team's February 2023 steering cycle"
aliases = ["inside-rust/2023/02/10/compiler-team-feb-steering-cycle.html"]

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

On [Friday, February 10th][feb-10-zulip-archive], the Rust Compiler team had a planning meeting for the February 2023 steering cycle.

[feb-10-zulip-archive]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings/topic/.5Bplanning.20meeting.5D.202023-02-10/near/327073587

### T-compiler June Steering Schedule

| Date           | Meeting Id            | Meeting Topic                                   |
|----------------|-----------------------|-------------------------------------------------|
| [2023-02-17][] | [compiler-team#589][] | Dealing with PR review latency                  |
| [2023-02-24][] | [compiler-team#583][] | Scope and goals of rust-lang/rust optimizations |
| [2023-03-03][] | [compiler-team#590][] | P-high review for 2023 Q1                       |
| [2023-03-10][] | none                  | (planning for March cycle)                      |

[2023-02-17]: https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=Nzk5YW5ybjZhZHI5c243cjllZmdhc2RkMG8gNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2023-02-24]: https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=MDFpY2NtNmFxbWZ1Y2tpN3Fka2Vqa251YWkgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2023-03-03]: https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=MDk5ZDhtMjAzcmt2ZDlmMmR0ZWE0cXB2ZjUgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[2023-03-10]: https://calendar.google.com/calendar/event?action=TEMPLATE&tmeid=MDJyYnJ1cGFtdWR1c2lnNjFmcHJ2b3JlODFfMjAyMzAzMTBUMTUwMDAwWiA2dTVycnRjZTZscnR2MDdwZmkzZGFtZ2p1c0Bn&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com

[compiler-team#589]: https://github.com/rust-lang/compiler-team/issues/589
[compiler-team#583]: https://github.com/rust-lang/compiler-team/issues/583
[compiler-team#590]: https://github.com/rust-lang/compiler-team/issues/590

### Details

Every fourth Friday, the Rust compiler team decides how
it is going to use its scheduled steering and design meeting time over the next
three Fridays.

On Friday, 17 February, we will discuss ways to improve our Pull Request review
latency. Jack Huey, apiraino, and oli-obk will work on a document to drive the
meeting, collecting ideas on how to attack the problem.

On Friday, 24 February, we will discuss our philosophy about code optimizations
that are implemented in the rust-lang/rust repository (as opposed to
optimizations that are implemented in LLVM itself, which is where the bulk of
our optimization logic currently resides). Jak{e,ob} Degen will author the
document driving this meeting.

On Friday, 3 March, we will do a quarterly [review of the open P-high issues][compiler-team#590].
pnkfelix will do some ahead of time work [in zulip](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/reviewing.20P-high.20issues.202022.20.28Q3.29/near/300390068)
triaging
some of the issues, and potentially prepare a meeting document summarizing the
remainder, to maximize the quality of our synchronous in-meeting time.

On Friday, 10 March, we will hold our planning meeting for the next steering
cycle in March and April.

Each meeting will run from 2pm to 3pm GMT, and will take place on the
[T-compiler/meetings zulip stream][zulip].

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings
