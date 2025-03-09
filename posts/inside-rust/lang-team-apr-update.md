+++
layout = "post"
date = 2021-04-17
title = "Lang team April update"
author = "Niko Matsakis"
description = "Lang team April update"
team = "the lang team <https://lang-team.rust-lang.org/>"
+++

This week the lang team held its April planning meeting ([minutes]). We normally hold these meetings on the first Wednesday of every month, but this month we were delayed by one week due to scheduling conflicts.

The planning meeting is used for:

* Checking in on the status of our active projects
* Planning the design meetings for the remainder of the month

After each meeting, we post an update (like this one!) with notes and meeting announcements. 

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-04-14-Planning-Meeting.md

## Update from active projects

Did you know that you can see the lang team's active projects on our [project board](https://github.com/rust-lang/lang-team/projects/2)? We're still experimenting and evolving the setup, but the goal is that it should give you a quick overview of what kinds of things the lang team is focused on, and what stage they are in their development. 

The minutes contain [links the tracking issues for each project](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-04-14-Planning-Meeting.md#updates-from-active-groups-and-projects) and those issues contain status reports. Here are some interesting updates:

* [never type update](https://github.com/rust-lang/lang-team/issues/60#issuecomment-814509681):
    * we have a general plan for hybrid fallback that we think will allow us to stabilize `!` at long last without breaking existing crates
* [improved closure capture (RFC 2229)](https://github.com/rust-lang/lang-team/issues/50#issuecomment-814526085):
    * implementation is working quite well and migration is nearly implemented also
* [ffi-unwind](https://github.com/rust-lang/lang-team/issues/19#issuecomment-814581675):
    * C-unwind implementation has landed
    * there is some ongoing work to fix some bugs in the initial implementation
    * looking at potentially moving to considering setjmp/longjmp
* [nested pattern stabilization](https://github.com/rust-lang/rust/pull/83386#issuecomment-819719603) has almost completed

## Upcoming design meetings

We planned two design meetings for April. Our meetings are open for anyone to join and observe. They are also typically recorded and posted to YouTube. Ping nikomatsakis or joshtriplett for info about attending.

* April 21 -- proposed "wasm" ABI ([lang-team#90](https://github.com/rust-lang/lang-team/issues/90)), featuring special guest Alex Crichton
* April 28 -- generators ([lang-team#92](https://github.com/rust-lang/lang-team/issues/92)), featuring special guest Esteban Küber

## Design meeting expectations

* The document for the meeting must be prepared by the triage meeting on Tuesday and posted to the tracking issue.
    * If it is not sent out by then, the meeting will be canceled. This gives folks 24 hour notice.
* There is no expectation that people will read the document before the meeting. The meeting will begin with a recap of the document.
    * However, there is no rule **against** reading the document beforehand and providing feedback or advice on how to improve it.
