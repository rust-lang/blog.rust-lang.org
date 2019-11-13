---
layout: post
title: "Governance WG Call For Participation"
author: Erin Power
team: The Governance WG <https://github.com/rust-lang/wg-governance>
---

Hello everyone, the governance working group has been working a few efforts, but
we haven't made as much progress as we would have liked over the past few
months, so we are going to try out a new process and meeting agenda aimed at
trying to get more work done in the time we know we have.

## New Meeting Structure

Previously we have been doing most of our work async while using our bi-weekly
meeting call to triage tasks. The problem we ran into with this is that a lot of
the members also heavy contributors to the Rust project as a whole, and it can
be can hard for some people to schedule time write an initial draft.

To address this instead of triaging during the meeting and working on tasks in
async, we're going to hold focused topic based meetings, and use the time between
meetings, to publish posts like this and to research and prepare for the next
topic. To help do this we're going to extend our current meeting duration from
30 minutes to an hour. (The meeting will still be every two weeks.)

The current goals are to documenting the de-facto governance structure, provide
the result as a RFC and then if merged provide a version on
[forge.rust-lang.org](https://forge.rust-lang.org/) so that it has greater visbility. We also want to
schedule people involved in Rust and other governance structures to come and
talk about their experiences.

For deciding what topics, we're going to rotate who takes the lead for each
meeting. It's that person's responsibility to decide the topic and to
prepare a call for participation similar to this post informing people of
the topic and how they might best prepare if they wish to join.

We also hope that having a focused topic will reduce any barrier of expected
knowledge in order to participate and contribute. With that said let's talk
about the topic for next meeting.

## Reviewing & Examining Previous Governance RFCs

Our first topic for new meeting is going to be to read [RFC 1068], the
original Rust Governance RFC, review how accurate it is to today's structure,
and see any if there are questions that we have that it doesn't answer. Here
are some other relevant RFCs for additional context:

- ["North Star" RFC] lays out the Rust roadmap process.
- [Compiler contributors RFC] details the process of contributing to the
  compiler and progression towards joining the compiler team.
- [Compiler bug fix procedure] defines the best practices for making a bug fix
  to the compiler.

Our next meeting is going to be at **22:00 UTC on Tuesday, November 19th**
and we'd like to encourage anyone who's interested, regardless of their
previous experience to read those RFCs and come to the `#wg-governance`
channel on discord to attend the meeting. (Our meetings are done over a video
call with Zoom, but we use the discord channel to organise ourselves).

If some reason you know you won't be able to attend these meetings but would
still like to participate. Please feel free to post any questions about Rust's
governance as [issues on our GitHub repository][gh-issues], even if you are 
not available to attend the working group's meetings.

[rfc 1068]: https://rust-lang.github.io/rfcs/1068-rust-governance.html
["north star" rfc]: https://github.com/rust-lang/rfcs/blob/26197104b7bb9a5a35db243d639aee6e46d35d75/text/1728-north-star.md
[compiler contributors rfc]: https://rust-lang.github.io/rfcs/2689-compiler-team-contributors.html
[compiler bug fix procedure]: https://rust-lang.github.io/rfcs/1589-rustc-bug-fix-procedure.html
[gh-issues]: https://github.com/rust-lang/wg-governance/issues?q=is%3Aissue+is%3Aopen+label%3AQuestion
