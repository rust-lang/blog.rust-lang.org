+++
layout = "post"
title = "Lang team design meeting update"
author = "Niko Matsakis"
description = "Summary of some of the recent lang team design meetings"
team = "the lang team <https://www.rust-lang.org/governance/teams/lang>"
+++

Hello! Did you know that the [lang team] now has regular design
meetings? We use these meetings to dig deeper into the output of
active project groups. After the meeting, we typically post a
recording to [YouTube] as well as some [minutes into the lang-team
repository][min]. I wanted to write a quick update listing out some of
the meetings we've had recently as well as some of our upcoming
meetings.

[YouTube]: https://www.youtube.com/playlist?list=PL85XCvVPmGQg-gYy7R6a_Y91oQLdsbSpa
[lang team]: https://www.rust-lang.org/governance/teams/lang
[min]: https://lang-team.rust-lang.org/minutes.html

### Recent lang-team design meetings

We recently held two lang-team design meetings:

* The **const evaluation project group** discussed the overall state
  of const evaluation and a ["skill tree"] that they've been
  developing to show **what some of the next steps are** and how they
  relate to one another.  We also discussed **what "unsafe" might mean
  in a const evaluation context** and in particular [Ralf's proposal to
  consider "things that may not be const-evaluable" as "unsafe" in a
  const fn][ralfj].
  * [Minutes][m1], [recording][r1]
* **Safe transmute project group**: We discussed the approaches explored
  by the safe transmute group, and in particular did a bit of a **deep
  dive into the [exciting `typic` crate][typic]** being developed by
  [jswrenn]. We looked at what it might make sense to pursue as an
  **immediate RFC** and what ought to stay in the realm of library experimentation
  for the time being.
  * [Minutes][m2], [recording][r2]
  
[m1]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-06-24-const-eval-unsafe-and-skill-tree.md
[r1]: https://youtu.be/b3p2vX8wZ_c
[m2]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-07-01-safe-transmute-typic.md
[r2]: https://youtu.be/3aw-5Fcyo7s
[ralfj]: https://www.ralfj.de/blog/2018/07/19/const.html
["skill tree"]: https://github.com/nikomatsakis/skill-tree#what-is-a-skill-tree
[typic]: https://github.com/jswrenn/typic
[jswrenn]: https://github.com/jswrenn

### How lang-team design meeting proposals work

Every proposed meeting begins with an issue on the lang-team
repository. If you're curious, you can take a look at the [open issues
with the `meeting proposal` label][mp] to get an idea of what
meetings are being considered; if a meeting has been scheduled, it
will also be tagged with [`meeting scheduled`][ms] and have some comments as
to the current date.

[mp]: https://github.com/rust-lang/lang-team/issues?q=label%3Ameeting-proposal
[ms]: https://github.com/rust-lang/lang-team/issues?q=label%3Ameeting-scheduled

We currently schedule meetings in a rather ad-hoc fashion, basically
hammering it out over Zulip. I'd probably like to move us to a more
"regularly scheduled" scheduling meeting, like the compiler team, but
we haven't adopted such a scheme yet.

Anyone can propose a design meeting, though they are meant to be
proposed mostly in conjunction with active project groups or other
ongoing discussions, and not just out of the blue. Moreover, anyone is
welcome to listen in on a design meeting, though you should be aware
that the meeting is typically being recorded. Zoom links are available
upon request.

### Upcoming lang-team meeting proposals

We currently have two pending lang-team meeting proposals:

* A proposal to discuss the path to lang-team membership ([rust-lang/lang-team#32]).
* A proposal to discuss enforcing bounds on type aliases and how we
  might phase that in ([rust-lang/lang-team#25]).
  
We expect to be scheduling those soon, and we'll update the issues
with dates when they are available.

[rust-lang/lang-team#32]: https://github.com/rust-lang/lang-team/issues/32
[rust-lang/lang-team#25]: https://github.com/rust-lang/lang-team/issues/25
