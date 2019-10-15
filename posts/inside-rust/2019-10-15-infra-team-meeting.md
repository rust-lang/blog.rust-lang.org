---
layout: post
title: "2019-10-10 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by kennytm. Minutes written by pietroalbini.  
Attending: alexcrichton, kennytm, Mark-Simulacrum, pietroalbini, sgrif,
shepmaster  
[Start of the conversation][discord]

[discord]: https://discordapp.com/channels/442252698964721669/443148319431065610/633710764762464285

## Publishing the meeting minutes (pietroalbini)

The infrastructure team recently started keeping minutes of the meetings (these
ones!), recording everything we decided and what the rationale of the decision
was. The original minutes are stored in a private Paper document, as they also
contain minutes of the private parts of the meetings.

pietroalbini proposed to store a public version of the minutes (with the
private parts removed) on the
[infra-team](https://github.com/rust-lang/infra-team) repository, and to
cross-post them on the [“Inside Rust”](https://blog.rust-lang.org/inside-rust)
blog. The rest of the team agreed on this. pietroalbini also volunteered to
make the work of writing the minutes and publishing them.

## Binary signing and the SignPath offer (pietroalbini)

A few weeks ago the team received an email from SignPath offering code signing
certificates and infrastructure to the Rust project. The main concern team
members had was the lack of time to implement the changes on our end, as
everyone is busy with other tasks. Everyone agreed to revisit the topic and
their offer once someone inside the team has time to drive the implementation
effort.
