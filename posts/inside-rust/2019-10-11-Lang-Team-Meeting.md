---
layout: post
title: "2019-10-10 Lang Team Triage Meeting"
author: Niko Matsakis
description: "2019-10-10 Lang Team Triage Meeting"
team: the lang team <https://www.rust-lang.org/governance/teams/lang>
---

We had our [weekly triage meeting] on 2019-10-10. You can find the
[minutes] on the [lang-team] repository; there is also a [video
recording] on YouTube. This week we had a number of folks who have
been working on const evaluation in attendance, which gave us an
opportunity to dive into some of the complexities involved there. We
also discussed how to make the const evaluation effort into a
"shepherded project", a concept that we are still actively developing
(see my blog post on [Shepherding 3.0] for the general idea).

## Other updates on shepherded items

This is our current list of "shepherded items", which are things we
are tracking week to week. We're still evolving the shepherding
system. Hopefully we'll soon have links for all of these to a good,
representative place that explains the overall status and roadmap. All
in good time.

* [unwind-ffi]
    * created a [zulip] stream (`#wg-ffi-unwind`)
    * preparing an RFC that creates the group official and lays out the roadmap
* "object safety" group (e.g., [#57893])
    * no major updates, still iterating on the "in progress" branch 
* re-rebalance coherence
    * we have a spreadsheet mapping out all the possible tests
    * we'll fill out the matrix, but probably ready to stabilize 🎉
* stabilize proc-macros generating macro-rules items
    * waiting on a report from pnkfelix
* grammar working group
    * no updates

[weekly triage meeting]: https://github.com/rust-lang/lang-team/#meeting-calendara
[minutes]: https://github.com/rust-lang/lang-team/blob/master/minutes/2019-10-10.md
[lang-team]: https://github.com/rust-lang/lang-team/
[video recording]: https://youtu.be/QvE9-zce5_4
[Shepherding 3.0]: http://smallcultfollowing.com/babysteps/blog/2019/09/11/aic-shepherds-3-0/
[zulip]: http://smallcultfollowing.com/babysteps/blog/2019/09/11/aic-shepherds-3-0/
[#57893]: http://smallcultfollowing.com/babysteps/blog/2019/09/11/aic-shepherds-3-0/
[unwind-ffi]: https://github.com/nikomatsakis/project-ffi-unwind
