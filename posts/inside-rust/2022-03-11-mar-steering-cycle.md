+++
layout = "post"
title = "Rust Compiler March 2022 Steering Cycle"
author = "Felix Klock"
description = "The compiler team's March 2022 steering cycle"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++
On [Friday, March 11th][mar-11-zulip-archive], the Rust Compiler team had a planning meeting for the March steering cycle.

[mar-11-zulip-archive]: https://zulip-archive.rust-lang.org/stream/238009-t-compiler/meetings/topic/.5Bplanning.20meeting.5D.202022-03-11.html

Every fourth Friday, the Rust compiler team decides how
it is going to use its scheduled steering and design meeting time over the next
three Fridays.

On Friday, 18 March, we will be having a ["backlog bonanza"][ct484], in a
similar vein to that done by T-lang, to review the list of unimplemented or
partially-implemented features. pnkfelix and Jack Huey will prepare a document
to drive the meeting.

[ct484]: https://github.com/rust-lang/compiler-team/issues/484

On Friday, 25 March, we will be reviewing [formal methods in Rust][ct488], and
also discuss how to provide a [stable interface to MIR][ct498], `rustc`'s middle
intermediate representation, for use by external tools (such as those developed
by the formal methods community). pnkfelix and Xavier Denis will prepare a
document to drive the meeting.

Note: The MIR interface topic is of interest to a number of external
stakeholders, so we expect to have the document to drive the meeting ready for
review well ahead of this meeting. Please reach out to pnkfelix if you are
working on a project that would benefit from stable MIR, so we can determine if
you should be included in the development of that document. (Of course, all are
welcome at the steering meeting itself.)

[ct488]: https://github.com/rust-lang/compiler-team/issues/488
[ct498]: https://github.com/rust-lang/compiler-team/issues/498

On Friday, 1 April, we will discuss [robust incremental compilation][ct490]. The
compiler team has opted to disable incremental compilation in the stable channel
twice in the past year. This meeting will discuss the pervasive issues suffered
by the current design, and possible strategies to validate the incremental
compilation subsystem going forward. mw and Aaron Hill will prepare a document
to drive the meeting.

[ct490]: https://github.com/rust-lang/compiler-team/issues/490

The two meetings in March will run from 2pm to 3pm GMT.

The meeting on April 1st has a different time than normal; it will run from 4pm
to 5pm GMT.

All of the meetings will take place on the [T-compiler/meetings zulip stream][zulip].

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings

You can find a public link to the [compiler team calendar][cal] here; it lists
these meetings as well as a number of other meetings for the compiler team and
various projects and working groups.

[cal]: https://rust-lang.github.io/compiler-team/#meeting-calendar
