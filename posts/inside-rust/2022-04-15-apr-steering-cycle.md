+++
layout = "post"
title = "Rust Compiler April 2022 Steering Cycle"
author = "Felix Klock"
description = "The compiler team's April 2022 steering cycle"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++
On [Friday, April 8th][apr-08-zulip-archive], the Rust Compiler team had a planning meeting for the April 2022 steering cycle.

[apr-08-zulip-archive]: https://zulip-archive.rust-lang.org/stream/238009-t-compiler/meetings/topic/.5Bplanning.20meeting.5D.202022-04-08.html

Every fourth Friday, the Rust compiler team decides how
it is going to use its scheduled steering and design meeting time over the next
three Fridays.

I want to apologize for how late I am posting this message: Our planning meeting
was a week ago, and my intent each cycle is to put up this post that day, so
that everyone, not just the compiler team members, has a chance participate in
the meetings. But, since this is going out a week late, it means one of the
meetings already happened (today).

On Friday, 15 April, we discussed [salsa 2.0's entity design][ct507]. [Salsa][]
is a generic framework for on-demand incrementalized computation, which has many
ties to `rustc`'s query system. Niko Matsakis authored the document that drove
the meeting's discussion. The discussion was on [zulip][ct507-archive], and
should eventually be available on Rust's public zulip archive.

[ct507]: https://github.com/rust-lang/compiler-team/issues/507

[Salsa]: https://github.com/salsa-rs/salsa

[ct507-archive]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings/topic/.5Bsteering.20meeting.5D.202022-04-15.20compiler-team.23507/near/279082491

On Friday, 22 April, we will be having a meeting to discuss compiler team
[leadership and succession planning][ct506]. Leadership of the Compiler Team,
and selection of new leads, has thus far been performed in an ad-hoc manner. We
will be reviewing a document authored by Felix Klock that tries to fix this by
writing down 1. what the leads do today, 2. the minimum we expect from future
leads, and 3. what process we should use for selecting new leads.

We are expecting this meeting may not occupy a full hour slot, and plan to
occupy any remaining time doing ["backlog bonanza"][ct484], reviewing the list
of unimplemented or partially-implemented features.

[ct506]: https://github.com/rust-lang/compiler-team/issues/506
[ct484]: https://github.com/rust-lang/compiler-team/issues/484

On Friday, 29 April, we will be having a meeting to discuss the future of Rust's
[incrementatal compilation][ct491]. The incremental compilation system has been
hard to maintain and hasn't been delivering as much value as we had hoped, at
least given the amount of effort that goes into maintaining it. We already had a
[meeting on Friday, 1 April][ct490] that looked at our high-level options going
forward; this meeting is a follow-up to that, but will now be informed by the
aforementioned discussion of Salsa 2.0's entity design, which would resolve some
(but not all) of the issues that plague us today.

[ct491]: https://github.com/rust-lang/compiler-team/issues/491
[ct490]: https://github.com/rust-lang/compiler-team/issues/490

The three meetings in April each run from 2pm to 3pm GMT.

All of the steering meetings take place on the [T-compiler/meetings zulip stream][zulip-meetings].

[zulip-meetings]: https://rust-lang.zulipchat.com/#narrow/stream/238009-t-compiler.2Fmeetings

