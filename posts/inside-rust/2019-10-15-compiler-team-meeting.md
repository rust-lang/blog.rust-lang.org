---
layout: post
title: "2019-10-10 Compiler Team Triage Meeting"
author: Wesley Wiser
description: "2019-10-10 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-10-10.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-10-10/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

## Announcements

- [@centril](https://github.com/centril) is splitting `libsyntax` into data and logic parts which will decrease the amount of code `librustc` depends on.

- [@nagisa](https://github.com/nagisa) is working on a cross-platform version of stacker which will allow us to avoid stack overflows in rustc.

- There is a compiler team [design meeting](https://rust-lang.github.io/compiler-team/minutes/design-meeting/2019-10-11-DepGraph-persistence-PR62038/) scheduled for tomorrow (2019-10-11) to discuss some of [@Zoxc](https://github.com/zoxc)'s PRs.

- [@pnkfelix](https://github.com/pnkfelix) is revising the code that handles structural match checking.

- [@nikomatsakis](https://github.com/nikomatsakis) has a PR ([#65232](https://github.com/rust-lang/rust/issues/65232)) up which helps us get closer to lazy normalization.

- `wg-traits` is going to start holding a weekly "office hours" video call to help answer questions and teach people about trait system internals.

## Working group sync

<br/>

### [wg-learning](https://rust-lang.github.io/compiler-team/working-groups/learning/)

`wg-learning` aims to make the compiler easier to learn by ensuring that rustc-guide and api docs are “complete”.

- `wg-learning` has been working on transcribing videos from the [compiler lecture series](https://www.youtube.com/watch?v=elBxMRSNYr4&list=PL85XCvVPmGQhOL-J2Ng7qlPvDVOwYpGTN) into [rustc-guide](https://rust-lang.github.io/rustc-guide/) chapters.

- Originally, individuals were assigned one or lectures to complete but that hasn't worked very well.

- Recently, they're trying to work on one video at a time as a team with much better results.

- There's [a PR](https://paper.dropbox.com/doc/Ty-lecture-summary--Almbjo_id6n8CKrOHlsTMG2dAg-4jFj9bVOLlW7uhIOWHITX) open for a new chapter based on the `ty` lecture.

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-10.20.2354818/near/177816515)

### [wg-llvm](https://rust-lang.github.io/compiler-team/working-groups/llvm/)

`wg-llvm` encompasses work in LLVM upstream fixing the issues and implementing features that matter to Rust.

- rustc has upgraded to the LLVM 9 release;
  - Which allows us to replace some the emscripten stuff with LLVM’s toolchain.
  - As part of these upgrades we will likely end up dropping support for the super old LLVM 6, which in turn allows us to stop building the unnecessary Go & OCaml bindings to LLVM, which in turn helps rustc build times slightly.
- People are also working on enabling use of the new pass manager, which might give us some tangible code quality improvements over the status quo.

[Link to full discussion](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/weekly.20meeting.202019-10-10.20.2354818/near/177817116)
