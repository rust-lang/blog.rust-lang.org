---
layout: post
title: "2019-11-14 Compiler Team Triage Meeting"
author: "Wesley Wiser"
description: "2019-11-14 Compiler Team Triage Meeting"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

The compiler team had our weekly triage meeting on 2019-11-14.
You can find the [minutes](https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2019-11-14/) on the [compiler-team](https://github.com/rust-lang/compiler-team) repository.
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

## Announcements

- Request for assistance: "Rustc panics (NoSolution): could not prove Binder(projection soup)" [#65581](https://github.com/rust-lang/rust/issues/65581)

- Request for assistance: "Rust 1.38 regressions weren't fully triaged" [#65577](https://github.com/rust-lang/rust/issues/65577)

- Request for assistance: "Miscompilation with target-cpu=znver1 (AMD Ryzen 1000/2000 series) on Windows + LLVM 9." [#63959](https://github.com/rust-lang/rust/issues/63959)

- [@cjgillot] replaced a lot of TypeFoldable impls with a derive [#66384](https://github.com/rust-lang/rust/pull/66384)

- The Infra team has finished evaluating GitHub Actions and we're switching!
  - This will have a signficant, positive impact on CI build time.

- [@centril] is fixing useless `<std macros>` spans [#66364](https://github.com/rust-lang/rust/pull/66364)

## Working group sync

This week we heard from three working groups because we ran out of time in the previous meeting.

### [wg-polonius](https://rust-lang.github.io/compiler-team/working-groups/polonius/)

- Made a lot of progress on the completeness goals with move/initialization errors and subset errors both getting close to completion.

- Fixed the last failure in the rustc test suite.
  - There are still the same 2 OOMs as last time, we haven't had much time to look at those yet.

- Made diagnostics output match NLL in a lot more cases.

- Did some cleanup in our terminology by picking better names for our atoms hopefully making it clearer in the process, and more work is planned here.
  - "origin" instead of "region"
  - "path" instead of "MovePath"

- There is a [polonius book](https://rust-lang.github.io/polonius/) now! It's sparse at the moment but more documentation work is in-flight and planned.

- The exploration and prototype on the rules offering more flow-sensitive precision for the analysis has also progressed a lot.

- There's also been some refactoring, and quite a bit of work on performance. Since the latter can step on the other work and vice-versa, we decided to focus on completeness first, and then after that has been achieved, re-adapt and land the optimization work.

- [@nikomatsakis] did a presentation on Polonius at RustBelt Rust. [Slides](https://nikomatsakis.github.io/rust-belt-rust-2019/)

- [@albins] has finished their master's thesis and is currently rewriting most of the [draft](https://rust-lang.zulipchat.com/user_uploads/4715/ufu5BGNrkzVbV8FtkK3Tco6M/Albins-Thesis-draft-version.pdf).

- We hope to have a "polonius work week" at the end of November to push the in-progress work over the finish line together.

### [wg-self-profile](https://rust-lang.github.io/compiler-team/working-groups/self-profile/)

- We've nearly completed our long standing MVP goal!
  - [@simulacrum] has done some nice work to polish the integration with perf.rlo
  - We've added tracking for all the events we're aware of that should be traced with the exception of trait selection.
    - We could really use some input as to what would be helpful to track!

- [@mw] has been working on some changes to the binary format we record events in.
  - The new format is more compact so results in a smaller trace file and hopefully less runtime overhead.
  - The new format is also more amenable to recording query keys, which is a highly requested feature.

- [@wesleywiser] has added some crate level docs to make getting into the code easier.

- [@wesleywiser] also added code to record process id, start time, and arguments to the trace file which we've started using.

- [@andjo403] has been a roll with a lot of great PRs!
  - We now have a dedicated tool for generating flamegraphs directly so you don't have to use the Perl scripts anymore.
  - Some internal refactoring that makes adding new tools easier.
  - Lots of work on the Chromium dev tools exporter:
    - New option to collapse disjoint threads so it's a little more manageable
    - New option to filter out small events under a configurable threshold (necessary for very large compilations)
    - You can now have multiple crate compilations in the same export file. This is similar to what cargo build -Z timings can do but much more detailed.

### [wg-rls-2.0](https://rust-lang.github.io/compiler-team/working-groups/rls-2.0/)

- Work is procedding on splitting core of rust-analyzer into crates.

- Find usages is implemented.

- Macro expansion now can map source ranges to expanded ranges, so goto def correctly goes "inside" macro call.

- More chalk an type inference work, specifically, support for closures.

- There's ongoing discussion about the general planning about rustc, rls, and rust-analyzer.

[@cjgillot]: https://github.com/cjgillot
[@centril]: https://github.com/centril
[@nikomatsakis]: https://github.com/nikomatsakis
[@albins]: https://github.com/albins
[@simulacrum]: https://github.com/mark-simulacrum
[@mw]: https://github.com/michaelwoerister
[@wesleywiser]: https://github.com/wesleywiser
[@andjo403]: https://github.com/andjo403
