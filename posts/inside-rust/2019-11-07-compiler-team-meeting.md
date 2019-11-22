---
layout: post
title: "2019-10-31 Compiler Team Triage Meeting"
author: "Wesley Wiser"
description: "2019-10-31 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-10-31.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-10-31/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

# 2019-10-31

## Announcments

Rust 1.39 ships on Thursday!

## Working group sync

<br/>

### [wg-pgo](https://rust-lang.github.io/compiler-team/working-groups/pgo/)

- PGO is available in the stable compiler. Docs are in the rustc-guide and the rustc-book

- Unfortunately we don't observe significant performance gains from applying it (except for tiny synthetic test cases).

- [@michaelwoerister] asked for people to try PGO on irlo but haven't gotten a lot of feedback.

- There is one interesting theory that Rust doesn't profit as much as C++ because it defaults to having fewer compilation units and thus can make better inlining decisions in the non-PGO case.

- The working group will probably wind down as [@michaelwoerister] don't have time to pursue further and there are no actual known issues in the implementation.

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-31.20.2354818/near/179539153)

### [wg-pipelining](https://rust-lang.github.io/compiler-team/working-groups/pipelining/)

- Pipelining support has shipped in Rust 1.38.

- The working group is winding down.

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-31.20.2354818/near/179539371)

[@michaelwoerister]: https://github.com/michaelwoerister