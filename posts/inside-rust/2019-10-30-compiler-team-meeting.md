---
layout: post
title: "2019-10-24 Compiler Team Triage Meeting"
author: "Wesley Wiser"
description: "2019-10-24 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-10-24.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-10-24/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two fo the compiler team working groups.

## Announcments

- [@simulacrum](https://github.com/Mark-Simulacrum) landed the rust-std split PR which decreases the size of the rustc-dev rustup component [#65474](https://github.com/rust-lang/rust/pull/65474)

## Working group sync

<br/>

### [wg-nll](https://rust-lang.github.io/compiler-team/working-groups/nll/)

- Rust 1.40 (current nightly) will be the first stable release without the HIR borrow checker.
This means Non Lexical Lifetimes will be available on Rust 2015.

- At this point, wg-nll has completed its purpose and will be disbanded.
  - wg-polonius is still going strong though!

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-24.20.2354818/near/178960111)

### [wg-parallel-rustc](https://rust-lang.github.io/compiler-team/working-groups/parallel-rustc/)

- Work is proceeding slowly but steadily.
There are regular triage meetings every Monday on the compiler calendar.

- The current goal is to refactor/audit compiler locks and other parts of the parallel query system and to investigate improving performance.

- The current MVP is to do high-level parallelization in a few critical places such as typechecking & linting.

- Currently, we're not seeing the speedups we want from parallelization but work is ongoing to remove locking and improve the jobserver integration.

- [Video recording of last meeting](https://youtu.be/lVjW0Nw8N_g)

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-24.20.2354818/near/178960379)
