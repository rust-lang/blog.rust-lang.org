+++
path = "inside-rust/2021/03/03/lang-team-mar-update"
title = "Lang team March update"
authors = ["Niko Matsakis"]
description = "Lang team March update"
aliases = ["inside-rust/2021/03/03/lang-team-mar-update.html"]

[extra]
team = "the lang team"
team_url = "https://lang-team.rust-lang.org/"
+++

Today the lang team held its March planning meeting ([minutes]). We hold these meetings on the first Wednesday of every month. 

The planning meeting is used for:

* Checking in on the status of our active projects
* Planning the design meetings for the remainder of the month

After each meeting, we post an update (like this one!) with notes and meeting announcements. 

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-03-03-Planning-Meeting.md

## Update from active projects

Did you know that you can see the lang team's active projects on our [project board](https://github.com/rust-lang/lang-team/projects/2)? We're still experimenting and evolving the setup, but the goal is that it should give you a quick overview of what kinds of things the lang team is focused on, and what stage they are in their development. Our minutes contain a [writeup for each active project](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-02-03-Planning-Meeting.md#project-updates-and-discussion), but let me call out a few highlights here:

* type ascription expressions:
    * we've recently made progress on type ascriptions and are likely to overcome the last issue blocking stabilization
    * we would like to post a revised RFC
    * there is some debate about the `x: T` syntax that was previously proposed, and some desire to find an alternative (but also plenty of folks who liked it)
* never type:
    * current status is still [#79366](https://github.com/rust-lang/rust/pull/79366)
    * simulacrum is going to recap the scenario for next triage meeting
    * we are considering whether it makes sense to leverage 2021 Edition in any way
* const generics:
    * min const generics will be stable in 1.51 
    * we are looking at various small extensions
    * there is now a weekly meeting to look into improvements
* declarative macro repitition counts:
    * there is an [open RFC](https://github.com/rust-lang/rfcs/pull/3086) with proposed FCP
* instruction set attribute:
    * exploration continues, the interaction of instruction set attribute with inline is not great, but it's not clear how to improve
* revised try trait:
    * generally positive vibes about the [RFC 3058](https://github.com/rust-lang/rfcs/pull/3058/)
    * one question is whether to consider leveraging edition to tweak the desugaring and bypass some of the "accidental stabilization"
        * currently evaluating how many projects rely on this behavior; most of them don't want to
* ffi-unwind:
    * C-unwind implementation has almost landed
    * exploring impact of longjmp on optimization

## Upcoming design meetings

We planned three design meetings for March. Our meetings are open for anyone to join and observe. They are also typically recorded and posted to YouTube. Ping nikomatsakis or joshtriplett for info about attending.

* March 10 -- no meeting, pnkfelix is absent
* March 17 -- RFC backlog bonanza recap ([lang-team#84](https://github.com/rust-lang/lang-team/issues/84))
* March 24 -- lang team reorg: shepherds 4.0, triage update ([lang-team#85](https://github.com/rust-lang/lang-team/issues/85))
* March 24 -- How to dismantle an `&Atomic` bomb ([lang-team#82](https://github.com/rust-lang/lang-team/issues/82))

## Design meeting expectations

We also settled some our expectations around preparation for design meetings:

* The document for the meeting must be prepared by the triage meeting on Tuesday.
    * If it is not sent out by then, the meeting will be canceled. This gives folks 24 hour notice.
* There is no expectation that people will read the document before the meeting. The meeting will begin with a recap of the document.
    * However, there is no rule **against** reading the document beforehand and provided feedback or advice on how to improve it.


