---
layout: post
title: "Planning the 2021 Roadmap"
author: The Rust Core Team
release: false
---

The core team is beginning to think about the 2021 Roadmap, and we want to hear from the community. We’re going to be running two parallel efforts over the next several weeks: the 2020 Rust Survey, to be announced next week, and a call for blog posts.

Blog posts can contain anything related to Rust: language features, tooling improvements, organizational changes, ecosystem needs — everything is in scope. We encourage you to try to identify themes or broad areas into which your suggestions fit in, because these help guide the project as a whole.

One way of helping us understand the lens you're looking at Rust through is to give one (or more) statements of the form "As a X I want Rust to Y because Z". These then may provide motivation behind items you call out in your post. Some examples might be:

- "As a day-to-day Rust developer, I want Rust to make consuming libraries a better experience so that I can more easily take advantage of the ecosystem"
- "As an embedded developer who wants to grow the niche, I want Rust to make end-to-end embedded development easier so that newcomers can get started more easily"

This year, to make sure we don’t miss anything, when you write a post please submit it into [this google form](https://forms.gle/Hv41uA5qJEY89XRm7)! We will try to look at posts not submitted via this form, too, but posts submitted here aren’t going to be missed. Any platform — from blogs to GitHub gists — is fine! We plan to close the form on October 5th.

To give you some context for the upcoming year, we established these high-level goals for 2020, and we wanted to take a look back at the first part of the year. We’ve made some excellent progress!

- Prepare for a possible Rust 2021 Edition
- Follow-through on in-progress designs and efforts
- Improve project functioning and governance

## Prepare for a possible Rust 2021 Edition

There is now an [open RFC](https://github.com/rust-lang/rfcs/pull/2966) proposing a plan for the 2021 edition! There has been quite a bit of discussion, but we hope to have it merged within the next 6 weeks. The plan is for the new edition to be much smaller in scope than Rust 2018. It it is expected to include a few minor tweaks to improve language usability, along with the promotion of various edition idiom lints (like requiring `dyn Trait` over `Trait`) so that they will be “deny by default”. We believe that we are on track for being able to produce an edition in 2021.

## Follow-through on in-progress designs and efforts

One of our goals for 2020 was to push “in progress” design efforts through to completion. We’ve seen a lot of efforts in this direction:

- The inline assembly RFC has [merged](https://rust-lang.github.io/rfcs/2873-inline-asm.html) and new implementation ready for experimentation
- Procedural macros have been stabilized in most positions [as of Rust 1.45](https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#stabilizing-function-like-procedural-macros-in-expressions-patterns-and-statements)
- There is a proposal for a MVP of const generics, which we’re hoping to [ship in 2020](https://without.boats/blog/shipping-const-generics/)
- The async foundations group is expecting to post an RFC on the `Stream` trait soon
- The FFI unwind project group is closing out a long-standing soundness hole, and the [first RFC](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html) there has been merged
- The safe transmute project group has proposed a [draft RFC](https://github.com/rust-lang/rfcs/pull/2981)
- The traits working group is polishing Chalk, preparing rustc integration, and seeing experimental usage in rust-analyzer. You can learn more in [their](https://blog.rust-lang.org/inside-rust/2020/03/28/traits-sprint-1.html) [blog](https://blog.rust-lang.org/inside-rust/2020/05/18/traits-sprint-2.html) [posts](https://blog.rust-lang.org/inside-rust/2020/07/17/traits-sprint-3.html).
- We are transitioning to rust-analyzer as the official Rust IDE solution, with a [merged RFC](https://rust-lang.github.io/rfcs/2912-rust-analyzer.html) laying out the plan
- Rust’s tier system is being formalized with guarantees and expectations set in an [in-progress RFC](https://github.com/rust-lang/rfcs/pull/2803)
- Compiler performance work continues, with wins of [10-30%](https://perf.rust-lang.org/compare.html?start=2020-01-01&end=&stat=instructions%3Au) on many of our benchmarks
- Reading into uninitialized buffers has an open [RFC](https://github.com/sfackler/rfcs/blob/read-buf/text/0000-read-buf.md), solving another long-standing problem for I/O in Rust
- A project group proposal for portable SIMD in std has an open [RFC](https://github.com/KodrAus/rfcs/blob/simd-pg/text/0000-stdsimd.md)
- A project group proposal for error handling ergonomics, focusing on the std::error API, has an open [RFC](https://github.com/yaahc/rfcs/blob/ehpg/text/0000-project-error-handling.md)
- `std::sync` module updates are in brainstorming phase
- Rustdoc's support for intra-doc links is [close to stabilization](https://github.com/rust-lang/rust/pull/74430)!

There’s been a lot of other work as well both within the Rust teams, but these items highlight some of the issues and designs that are being worked on actively by the Rust teams.

## Improve project functioning and governance

Another goal was to document and improve our processes for running the project. We had three main subgoals.

### Improved visibility into state of initiatives and design efforts

The Rust teams are moving to the use of [project groups](https://rust-lang.github.io/rfcs/2856-project-groups.html) for exploratory work, aiming to create dedicated groups of people who can explore an area, propose a design, and see it through to completion. The language team has kicked us off with [safe transmute](https://github.com/rust-lang/project-safe-transmute/), [FFI unwind](https://github.com/rust-lang/project-ffi-unwind/), and [inline assembly](https://github.com/rust-lang/project-inline-asm) project groups. All of these have been enormous successes! Other teams are looking to use this model as well.

The compiler team has begun publishing [weekly performance triage reports](https://github.com/rust-lang/rustc-perf/tree/master/triage), in the continuing drive to reduce compile times. The LLVM working group has also been helping to highlight performance regressions in [LLVM itself](https://nikic.github.io/2020/05/10/Make-LLVM-fast-again.html), to reduce compile time performance regressions when updating LLVM.

The [compiler team](https://github.com/rust-lang/compiler-team/) has introduced [Major Change Proposals](https://forge.rust-lang.org/compiler/mcp.html) as a way to introduce larger changes to the implementation, surfacing design questions before implementation work begins. The [language team](https://github.com/rust-lang/lang-team/) is also experimenting with a [similar process](https://lang-team.rust-lang.org/proposing_a_project.html) for gaining quick language team feedback on proposals and, potentially, forming project groups. These both give a high-level view of changes being proposed, letting interested parties follow along without needing to subscribe to our very busy repositories.

### Increase mentoring, leadership, and organizational bandwidth

- The language team has identified a path for contributors to membership on the team, involving participation and leading in project group efforts. For more details, see [their post](https://blog.rust-lang.org/inside-rust/2020/07/09/lang-team-path-to-membership.html).
- The Governance working group has been formalizing existing processes into RFCs, such as the [Project Group RFC](https://rust-lang.github.io/rfcs/2856-project-groups.html), [Access Policy RFC](https://github.com/rust-lang/rfcs/pull/2872), and more.
- The library team is pioneering the effort of drafting formal [charters](https://github.com/KodrAus/rfcs/blob/libs-governance/text/0000-libs-governance.md) for teams, with the help of the governance working group.

### Making design discussions more productive and less exhausting

The primary effort here has been the project groups, which have so far been largely a success. We expect to do more here in the future.
