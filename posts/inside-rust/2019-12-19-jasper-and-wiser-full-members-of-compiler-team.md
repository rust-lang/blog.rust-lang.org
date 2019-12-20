---
layout: post
title: "Congrats to compiler team members matthewjasper and wesleywiser"
author: Felix S. Klock II
description: "Congrats to compiler team members matthewjasper and wesleywiser"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

I am pleased to announce that [@matthewjasper][] and [@wesleywiser][]
have been made full members of the [compiler team][].

[@matthewjasper][] has been a huge contributor to the Non-Lexical
Lifetimes (NLL) work, filing issues and fixing bugs starting way back in
2017. Matthew has also been fixing compiler soundness bugs and making
miscellaneous improvements to Rust's Middle Intermediate
Representation (MIR). Most recently, Matthew has removed all uses of
`gensym` from the compiler, as well as the `gensym` functionality
itself (which was implemented in a way that injected 
[subtle bugs][#43900] with incremental compilation).

[@wesleywiser][] first started contributing to Rust way back in 2015,
before even the 1.0 release, with various documentation [fixes][#22633].
Since then, Wesley has improved the incremental compilation system,
added MIR optimization passes like constant-propagation and inlining,
and has been part of the compiler's self-profiler effort, starting with its
[first version][#51657] and
continuing today as a co-lead of [WG-self-profile][].

Congratulations to both [@matthewjasper][] and [@wesleywiser][], and thanks
for all of your contributions to the project!

[@matthewjasper]: https://github.com/matthewjasper/
[@wesleywiser]: https://github.com/wesleywiser/
[compiler team]: https://www.rust-lang.org/governance/teams/compiler
[#43900]: https://github.com/rust-lang/rust/issues/49300
[#22633]: https://github.com/rust-lang/rust/pull/22633
[#51657]: https://github.com/rust-lang/rust/pull/51657
[WG-self-profile]: https://rust-lang.github.io/compiler-team/working-groups/self-profile/
