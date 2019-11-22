---
layout: post
title: "2019-10-17 Compiler Team Triage Meeting"
author: "Wesley Wiser"
description: "2019-10-17 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-10-17.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-10-17/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

## Announcements

- Design meeting on [debuginfo strategy](https://github.com/rust-lang/compiler-team/issues/186) tomorrow (2019-10-18) on Zulip.
  - [Link to full meeting](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/design.20meeting.202019-10-18/near/178476377)

- The traits working group had a [design meeting](https://github.com/rust-lang/wg-traits/tree/master/minutes) yesterday on lazy normalization.

- [@nikomatsakis] has been looking into lazy normalization specifically for constants and hopes to have some notes to share soon.

- The LLVM ICE-breakers working group is nearly ready to go live.

- [@spastorino] has nearly finished interning `Place` projections which is an incremental step to MIR 2.0.

- [@centril] is continuing to work on splitting `libsyntax` apart which will decrease the amount of code `librustc` depends on.

## Working group sync

<br/>

### [wg-mir-opt](https://rust-lang.github.io/compiler-team/working-groups/mir-opt/)

- [@wesleywiser] Moved promoted MIR out of `mir::Body` [#63580](https://github.com/rust-lang/rust/pull/63580)

- [@wesleywiser] Reimplemented the `ConstProp` optimization pass on top of the existing const eval code [#64419](https://github.com/rust-lang/rust/pull/64419)

- [@spastorino] Converted `mir::Place` to no longer be a recursive data structure [#63420](https://github.com/rust-lang/rust/pull/63420)

- There is ongoing work in both const prop and place refactorings.

- We've figured out a plan for "weird" place projections like dereferences (virtual locals).

[Link to full conversation](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-17.20.2354818/near/178389131)

### [wg-meta](https://rust-lang.github.io/compiler-team/working-groups/meta/)

- The [Inside Rust](https://blog.rust-lang.org/inside-rust/index.html) blog has launched.

- The [ICE-Breaker group](https://rust-lang.github.io/rustc-guide/ice-breaker/about.html) has been formed!

- The [LLVM ICE-Breaker group](https://rust-lang.github.io/rustc-guide/ice-breaker/llvm.html) is also being formed.

[Link to full conversation](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-17.20.2354818/near/178389266)


[@nikomatsakis]: https://github.com/nikomatsakis
[@spastorino]: https://github.com/spastorino
[@centril]: https://github.com/centril
[@wesleywiser]: https://github.com/wesleywiser
