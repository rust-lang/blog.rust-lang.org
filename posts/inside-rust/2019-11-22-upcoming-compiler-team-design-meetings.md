---
layout: post
title: "Upcoming compiler-team design meetings"
author: Niko Matsakis
description: "Upcoming compiler-team design meetings"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

In our [planning meeting today], the [compiler team] has scheduled our
next batch of upcoming design meetings. You can find the exact times
on the compiler team's [meeting calendar]:

* On Nov 29 ([calendar event][ce1]), we will discuss
  [rust-lang/compiler-team#213], a proposal to extend rustc's data
  structures to enable outside crates to experiment with building a
  Rust REPL.
* On Dec 6 ([calendar event][ce2]), we will discuss
  [rust-lang/compiler-team#175], which is a plan to modify the
  `rustc_interface` trait to enable end-to-end query support in the
  compiler.
* On Dec 13 ([calendar event][ce3]), we will discuss
  [rust-lang/compiler-team#222], which is a roadmap and strategy for
  eventually merging rustc, [rust-analyzer], and the RLS into one
  coherent set of projects.
* On Dec 20 ([calendar event][ce4]), we will discuss
  [rust-lang/compiler-team#209], which is a proposal to create a
  "major changes process" for the compiler, to augment these design
  meetings.

[rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer/
[ce1]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MjM3aGsxdXY0dHBybXBxZ3ZxOGp1ZjdicjEgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[ce2]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MTByaTZsZG1pZGI1Y2RqdGZ1cHV2djNncTEgNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[ce3]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MGxrb2p2cG9lNGFnYTIybWVtcGRoZzdjdmogNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[ce4]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MW12ZWI4NW9zZ2s2dHRkbW5wbjdjMG43Zm8gNnU1cnJ0Y2U2bHJ0djA3cGZpM2RhbWdqdXNAZw&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com
[rust-lang/compiler-team#213]: https://github.com/rust-lang/compiler-team/issues/213
[rust-lang/compiler-team#175]: https://github.com/rust-lang/compiler-team/issues/175
[rust-lang/compiler-team#222]: https://github.com/rust-lang/compiler-team/issues/222
[rust-lang/compiler-team#209]: https://github.com/rust-lang/compiler-team/issues/209

### Did you know?

Most weeks, the compiler team has some sort of design meeting. These
meetings take place on Zulip and are open to all. Every 4 weeks, we do
a planning meeting to pick the next few meetings from the list of open
proposals. You can find [more details about how the compiler-team
steering meeting process here][details].

[details]: https://rust-lang.github.io/compiler-team/about/steering-meeting/
[meeting calendar]: https://rust-lang.github.io/compiler-team/#meeting-calendar
[planning meeting today]: https://zulip-archive.rust-lang.org/131828tcompiler/03407planningmeeting20191122.html
[compiler team]: https://www.rust-lang.org/governance/teams/compiler
