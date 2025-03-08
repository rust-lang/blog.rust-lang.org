+++
layout = "post"
title = "Lang team July update"
author = "Niko Matsakis"
description = "Lang team July update"
team = "the lang team <https://lang-team.rust-lang.org/>"
+++

On 2021-07-07, the lang team held its July planning meeting ([minutes]). These meetings are tyically held the first Wednesday of every month.

The planning meeting is used for:

- Checking in on the status of our active initiatives
- Planning the design meetings for the remainder of the month

After each meeting, we post an update (like this one!) with notes and meeting announcements.

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-07-07-Planning-meeting.md

## Update on meeting recordings

As proposed in [lang-team#100](https://github.com/rust-lang/lang-team/issues/100), we have decided to discontinue recording our meetings. A number of people reported that they felt uncomfortable participating if they knew that their participation would be made available on YouTube, and we decided to prioritize participation in the meeting itself over making recordings available. We are aware that many people enjoyed viewing the recordings and also that they were useful for enabling more "asynchronous" participation or giving more color to lang-team reasoning (the minutes can't capture all the details). We are investigating alternatives here and may resume some form of recording at some future time. We encourage folks to engage on Zulip if you have thoughts on the solution space, or propose an MCP with concrete ideas!

## Update from active initiatives

Did you know that you can see the lang team's active initiatives on our [project board](https://github.com/rust-lang/lang-team/projects/2)? We're still experimenting and evolving the setup, but the goal is that it should give you a quick overview of what kinds of things the lang team is focused on, and what stage they are in their development. Here are some notable updates from some of the active initiatives:

- [Never type stabilization:](https://github.com/rust-lang/lang-team/issues/60#issuecomment-870126162)
  - After a lot of experimentation, we may be approaching stabilization here!
  - The plan is to begin with a complex fallback rule that preserves most existing patterns, and then to deprecate and evolve it over time.
- [FFI Unwind:](https://github.com/rust-lang/lang-team/issues/19#issuecomment-875772875)
  - There is a pending PR that, when landed, closes all remaining issues with "C-unwind", clearing the way for possible stabilization.
- [Inline assembly:](https://github.com/rust-lang/lang-team/issues/20)
  - There are still a few active blockers, but there is also some discusison on the thread of a "minimum inline assembly" stabilization that could proceed in the near future!
- [`#[instruction_set]` attribute:](https://github.com/rust-lang/rust/issues/74727)
  - The implementation is complete but doesn't produce optimal code. We are considering whether to stabilize in its current form, since it may be of use. **We are actively seeking feedback and experimentation from folks who might be interested in using this feature, which allows you to specify the instruction set for a particular function.**

## Upcoming design meetings

We planned two design meetings for this month. Each design meeting begins with a review of a document, which is posted publicly on the meeting issue at least 24 hours before the meeting. Our meetings are typically open for anyone to observe; you'll find timing and other details on [our calendar](https://lang-team.rust-lang.org/calendar.html).

- July 21: Lang team process follow-up ([lang-team#104](https://github.com/rust-lang/lang-team/issues/104))
- July 28: Structural equality ([lang-team#94](https://github.com/rust-lang/lang-team/issues/94))
