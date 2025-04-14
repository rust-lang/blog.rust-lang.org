+++
path = "inside-rust/2021/02/03/lang-team-feb-update"
title = "Lang team February update"
authors = ["Niko Matsakis"]
description = "Lang team February update"
aliases = ["inside-rust/2021/02/03/lang-team-feb-update.html"]

[extra]
team = "the lang team"
team_url = "https://lang-team.rust-lang.org/"
+++

Today the lang team held its first planning meeting ([minutes]). From now on, we're going to hold these meetings on the first Wednesday of every month. 

The planning meeting is used for:

* Checking in on the status of our active projects
* Planning the design meetings for the remainder of the month

After each meeting, we plan to post an update (like this one!) with notes and meeting announcements. 

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-02-03-Planning-Meeting.md

## Update from active projects

Did you know that you can see the lang team's active projects on our [project board](https://github.com/rust-lang/lang-team/projects/2)? We're still experimenting and evolving the setup, but the goal is that it should give you a quick overview of what kinds of things the lang team is focused on, and what stage they are in their development. Our minutes contain a [writeup for each active project](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2021-02-03-Planning-Meeting.md#project-updates-and-discussion), but let me call out a few highlights here:

* async foundations:
    * continued progress on polish, new traits
    * making plans to stabilize async functions in traits
    * working on a [vision document](https://hackmd.io/p6cmRZ9ZRQ-F1tlhGaN9rg) that lays out a multi-year vision for how async I/O should look/feel in Rust
* const generics:
    * min const generics is stable in nightly, will be coming to a stable release soon
* rfc 2229 ("minimal closure capture"):
    * continued progress on the implementation, things are going well
    * we will likely add a `capture!` macro to use for migration; it would force the capture of a particular local variable (and not some subpath of it)
* inline assembly
    * we are investigating stabilising inline assembly for certain architectures (but not all)

## Upcoming design meetings

We planned three design meetings for February. Our meetings are typically open for anyone to observe, although we have one closed meeting this week. They are also typically recorded and posted to YouTube. Ping nikomatsakis or joshtriplett for info about attending.

* Feb 10 \[CLOSED\]: [Growing the team](https://github.com/rust-lang/lang-team/issues/81). The plan is to talk about our plans to grow the team and recruit new members. **Closed because we expect some frank talk about who might be a good candidate.**
* Feb 17: [Improving trust in the Rust compiler](https://github.com/rust-lang/lang-team/issues/79), discussing the Ferrocene proposal and the semantics of MIR.
* Feb 24: [2021 idiom lint overview](https://github.com/rust-lang/lang-team/issues/83), discussing the various 2018 idiom lints and whether they ought to be included in 2021.



