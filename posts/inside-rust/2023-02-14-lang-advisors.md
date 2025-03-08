+++
layout = "post"
title = "Language team advisors"
author = "Josh Triplett, Niko Matsakis"
team = "The Rust Lang Team <https://www.rust-lang.org/governance/teams/lang>"
+++

[RFC #3327](https://github.com/rust-lang/rfcs/pull/3327) created a new lang-team subteam, the lang team advisors. The advisors team recognizes people who regularly aid the Rust community and the lang team in particular in language design decisions. We already value their input highly and treat their concerns as blocking on features or proposals. The advisors team gives us a way to acknowledge them officially.

The initial advisors team consists of the following people:

**Ralf Jung** is a leader in designing Rust's rules for unsafe code as well as, through his work on miri, the semantics of compile-time evaluation. His work on stacked borrows and minirust has moved the state of that conversation forward in major ways, and he has also driven a number of language changes related to that area.

**Jakob Degen** is one of the authorities around the semantics of unsafe code. He has consistently shown the ability to aggregate opinion, identify the key constraints to respect and those to disregard, and find consensus solutions.

**Mark Rousskov** has been a huge part of the Rust community for many years now, and participates regularly in lang-team meetings. He has a wide knowledge of Rust and its nooks and crannies, and often brings key insights to our discussions.

**Jack Huey** co-leads the types team, and provides expertise in the workings of Rust's trait and type system, as well as the chalk system.

**Amanieu d'Antras** leads the design of inline assembly and has been involved as an expert in a number of other areas, such as the "FFI unwind" working group.

**Wesley Wiser** is the co-lead of the compiler team. He's been involved in the project for many years and is an expert on the overall compiler architecture as well as several areas within.

**Alex Crichton** is a well-known figure to many Rustaceans. Among other things, he is a former lead of the libs team, a key cargo contributor, and drove extensive work for Rust in WebAssembly. Indeed, it's hard to find a part of Rust that Alex hasn't had an impact on.

Finally, as part of this change, **Taylor Cramer** will be stepping back as a full-time lang team member and becoming an advisor. In his time on the lang team, Taylor was a core driver for `async`/`await`, `impl Trait`, and a number of other highly impactful language features. We look forward to continuing to have his guidance as an advisor going forward.
