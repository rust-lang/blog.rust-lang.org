---
layout: post
title: "2019-11-07 Compiler Team Triage Meeting"
author: "Wesley Wiser"
description: "2019-11-07 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-11-07.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-11-07/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

## Announcements

- [@pnkfelix] is moving to the US and will be working in the Eastern timezone.

- [@wesleywiser] has been working on the constant propagation pass and wants to [turn it on][const_prop_on] for debug & release builds to improve compilation time.

- Rust 1.39 has shipped with `async`/`await` support and other goodies.

## Working group sync

<br/>

### [wg-polonius]

We ran out of time this week to have a check-in from this working group.

### [wg-rfc-2229]

This working group is currently on a pause.

[@pnkfelix]: https://github.com/pnkfelix
[@wesleywiser]: https://github.com/wesleywiser
[const_prop_on]: https://github.com/rust-lang/rust/pull/66074
[wg-polonius]: https://rust-lang.github.io/compiler-team/working-groups/polonius/
[wg-rfc-2229]: https://rust-lang.github.io/compiler-team/working-groups/rfc-2229/
