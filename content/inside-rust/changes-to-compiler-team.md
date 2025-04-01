+++
path = "inside-rust/2020/12/14/changes-to-compiler-team"
title = "Changes to Rust compiler team"
authors = ["Felix S. Klock II"]
description = "recent leadership and membership changes"
aliases = ["inside-rust/2020/12/14/changes-to-compiler-team.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

There have been important changes recently to the Rust compiler team.

## Leadership Rotation

Here is something very new for our team: We are rotating the leadership. Niko Matsakis will be stepping down from their role as co-lead, and Wesley Wiser will be joining Felix Klock as the co-leads for the team.

Niko remains a compiler team member and will continue his contributions, especially on the RFC 2229, Polonius and Chalk projects. [Niko's blog post](https://smallcultfollowing.com/babysteps/blog/2020/12/11/rotating-the-compiler-team-leads/) discusses their motivations for stepping down in more detail.

Wesley has been contributing to Rust since 2015, and became a compiler team member in 2019. Wesley has contributed heavily to the compiler's [`self-profile` infrastructure](https://blog.rust-lang.org/inside-rust/2020/02/25/intro-rustc-self-profile.html), to its collection of [MIR optimizations](https://blog.rust-lang.org/inside-rust/2019/12/02/const-prop-on-by-default.html), and to its const-eval and incremental compilation subsystems. Wesley has also already proven himself as an effective leader: they are currently co-lead of several working groups: wg-incr-comp, wg-prioritization, and wg-self-profile.

Niko and I are delighted that Wesley is taking on this new leadership role, and I look forward to steering the compiler alongside them.

## Membership Changes

In addition to the changes in leadership, we also have a new compiler team member to announce: lcnr.

lcnr has been contributing to the compiler since 2018. They have improved the type system via changes such as higher-ranked lifetimes in predicates ([#73503](https://github.com/rust-lang/rust/pull/73503)), and also have focused for the past two years on improving Rust's support for complex generic expressions in constants (tracked at [#76560](https://github.com/rust-lang/rust/pull/76560))
