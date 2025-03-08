+++
layout = "post"
title = "Re-organising the compiler team and recognising our team members"
author = "davidtwco and wesleywiser"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++
Back in June, the compiler team merged [RFC 3599][rfc] which re-structured the team to ensure the
team's policies and processes can support the maintenance of the Rust compiler going forward.

Since the last change to the structure of the compiler team, the project has grown a lot -  the
compiler receives roughly twice as many contributions, more team members regularly take on
additional responsibilities such as performance triage or backport reviews, and many contributors
are now paid to work on the project.

It is imperative that the compiler team and its procedures are able to scale to meet the demands
on the project, both to ensure that the compiler team's outputs remain high-quality, but also to
avoid over-working and burning out our team members.

RFC 3599 aims to recognise all of the ways that team members are currently contributing, to ensure
that the team's processes remain efficient as the team grows, and to strike a balance between team
membership enabling work efficiency and team membership being a form of status and recognition. Team
members who have been contributing for a year or more and want to participate in the maintenance
activities that keep the team going can choose to be maintainers as well as team members. See [the
full RFC][rfc] for more detailed motivations.

With RFC 3599 merged, we're now implementing the compiler team's new structure and with this post,
announcing and recognising the contributions of the compiler team's membership:

[rfc]: https://github.com/rust-lang/rfcs/pull/3599

- [alexcrichton](https://github.com/alexcrichton), team member
    - alexcrichton has been a prolific contributor since 2013 and has over 2500 merged pull
      requests. Recently they have been working to improve Rust's WASM support for emerging
      standards.
- [apiraino](https://github.com/apiraino), team member
    - apiraino is an invaluable member of the compiler team, handily tackling the team's
      operational work (agenda preparation, meeting notes, automation, etc) so that the team's work
      progresses smoothly each week.
- [b-naber](https://github.com/b-naber), team member
    - b-naber has been contributing to the compiler since 2020 and has worked on the compiler's
      constant evaluation, on constant generics and on generic associated types in that time.
- [bjorn3](https://github.com/bjorn3), team member
    - bjorn3 has been an active contributor to the compiler since 2017, becoming a compiler team
      contributor in 2020. bjorn has been instrumental in the compiler's support for multiple
      codegen backends and is the primary author of the Cranelift codegen backend.
- [BoxyUwU](https://github.com/BoxyUwU), maintainer
    - BoxyUwU has been contributing relentlessly since 2020, immediately jumping into some of the
      more challenging and technical areas in the compiler. Boxy is a member of the types team and
      has contributed heavily to the implementation and design of constant generics.
- [camelid](https://github.com/camelid), team member
    - camelid has contributed for many years, improving constant generics, project documentation, and compiler diagnostics
      and making many refactorings and cleanups on the compiler codebase. 
- [chenyukang](https://github.com/chenyukang), maintainer
    - Since 2021, yukang has been tirelessly improving the compiler's diagnostics. It would be hard
      to find a part of the compiler which hasn't had a diagnostic improved by yukang.
- [cjgillot](https://github.com/cjgillot), maintainer
    - cjgillot is another reliable and consistent contributor who has made countless improvements
      to the compiler since they started contributing, notably to the MIR and its optimisations,
      the query system and the HIR.
- [compiler-errors](https://github.com/compiler-errors), maintainer
    - compiler-errors is a prolific contributor and prominent member of the types team. They have
      authored numerous improvements to compiler diagnostics, resolved countless ICEs and made
      large refactorings and improvements to the compiler frontend. compiler-errors has worked to
      stabilize many recent features for T-types and WG-async, like async functions in traits. It
      would be difficult to find a contributor who hasn't been helped out or had a patch reviewed by
      compiler-errors.
- [cuviper](https://github.com/cuviper), team member
    - cuviper regularly contributes to the compiler's build system, driver and LLVM backend, and
      regularly performs backports.
- [davidtwco](https://github.com/davidtwco), maintainer + team co-lead
    - davidtwco co-leads the compiler team and has been contributing since 2017, making patches to
      various parts of the compiler, contributing to various working groups such as non-lexical
      lifetimes, and implementing features such as the non-exhaustive attribute and split debuginfo.
- [DianQK](https://github.com/DianQK), team member
    - DianQK has been contributing for over a year and has focused on improving the compiler's MIR
      optimisations and fixing and improving our use of LLVM.
- [durin42](https://github.com/durin42), team member
    - durin42 has been a strong contributor to the compiler's LLVM backend, debuginfo and general
      code generation infrastructure since they started contributing in 2021.
- [eholk](https://github.com/eholk), team member + council representative
    - eholk is active in the compiler team and async working group, contributing to the design and
      implementation of the proposed *dyn\** types, *generator functions* and *for await* loops.
      Eric also represents the compiler team in the project's leadership council.
- [est31](https://github.com/est31), team member
    - est31 has been a frequent contributor for almost eight years, making lots of helpful fixes
      and refactorings throughout the compiler. est31 can often be found providing helpful reviews
      and suggestions to Rust's open PRs.
- [estebank](https://github.com/estebank), maintainer
    - estebank is almost synonymous with better compiler diagnostics - over eight years and a
      thousand pull requests later, it would be hard to find a Rust user who hasn't seen a
      diagnostic improved by estebank.
- [fee1-dead](https://github.com/fee1-dead), maintainer
    - fee1-dead has made many impactful contributions since starting to contribute in 2021,
      including C string literals, keyword generic/effect experiments, const trait design &
      implementation and many bug fixes and diagnostic improvements.
- [flodiebold](https://github.com/flodiebold), team member
    - flodiebold is a long-time prolific contributor to rust-analyzer, making over 300 pull
      requests since starting to contribute to the language server in 2018.
- [fmease](https://github.com/fmease), team member
    - fmease has been contributing since 2022, making various improvements to the compiler to
      support rustdoc, as well as refactorings, bug fixes and diagnostic improvements.
- [jackh726](https://github.com/jackh726), maintainer
    - jackh726 co-leads the types team and has made numerous improvements to the implementation of
      the type system, most notably enabling the stabilization of generic associated types.
- [jieyouxu](https://github.com/jieyouxu), team member
    - jieyouxu does invaluable work in helping maintain bootstrap, compiletest and the new
      `run_make_support` library for `run-make` tests, as well as fixing ICEs and improving
      diagnostics.
- [jswrenn](https://github.com/jswrenn), team member
    - jswrenn has been a stalwart member of the safe transmute project group for years and has
      made various contributions implementing the fruits of the group's efforts.
- [lcnr](https://github.com/lcnr), maintainer
    - lcnr co-leads the types team and is one of the team's foremost experts on the language's
      type system and the compiler's implementation of it. lcnr's recent work has been focused on
      implementing and stabilizing the compiler's next-generation trait solver.
- [lqd](https://github.com/lqd), maintainer
    - lqd started out in the non-lexical lifetimes working group back in 2018 and has been part
      of the fabric of the project since. Compiler performance has seen significant improvements
      thanks to lqd's work on enabling LTO for the compiler and supporting lld. lqd is currently
      leading work on Polonius, the next generation of Rust's borrow checker.
- [lukas-code](https://github.com/lukas-code), team member
    - lukas-code has been contributing regularly since 2022, making improvements and fixing bugs
      throughout the compiler's codebase.
- [Mark-Simulacrum](https://github.com/Mark-Simulacrum), maintainer
    - Mark-Simulacrum has been working on the Rust project for almost a decade and frequently
      contributes to the team through backports, reverts and patches throughout the codebase. For
      many years, they have helped maintain critical compiler infrastructure such as bootstrap and
      the compiler test harness.
- [matthewjasper](https://github.com/matthewjasper), maintainer
    - matthewjasper has been contributing since 2017 and was a key contributor to the non-lexical
      lifetimes working group. They have since made significant improvements to the MIR, progress
      on specialization and stabilizing the THIR unsafeck.
- [Nadrieril](https://github.com/Nadrieril), maintainer
    - Nadrieril is the compiler team's expert on exhaustiveness checking, pattern analysis and
      match lowering, their significant refactoring and improvement work have enabled progress on
      previously blocked features such as slice patterns, or-patterns, exhaustive patterns and
      deref patterns.
- [nagisa](https://github.com/nagisa), team member
    - nagisa has been a compiler team member for many years, with their initial work on Rust
      dating back to 2014. nagisa improves the compiler's LLVM backend and everything to do with
      our MIR, codegen, debuginfo and the compiler backends in general.
- [nikic](https://github.com/nikic), team member
    - nikic is the team's LLVM expert and helps ensure the compiler is keeping pace with changes
      in LLVM upstream. nikic is also the lead maintainer of LLVM and has made many improvements
      in LLVM to better support Rust.
- [nikomatsakis](https://github.com/nikomatsakis), team member
    - nikomatsakis needs no introduction, as one of the original members of the Rust project and
      former lead of the compiler team. nikomatsakis has worked on crucial parts of the compiler
      since their initial implementation.
- [Noratrieb](https://github.com/Noratrieb), maintainer
    - Noratrieb has been a staple of the contributor community since they started in 2021,
      working on a swathe of refactorings, bug fixes, features and improvements throughout the
      compiler. Prolific contributors like Nora have an outsized impact across the codebase. Nora
      can often be found answering questions and helping other contributors on Zulip!
- [nnethercote](https://github.com/nnethercote), maintainer
    - nnethercote has been working on compiler performance since 2016, including the benchmarking
      and profiling infrastructure. He has also cleaned up a lot of old crufty code across many
      parts of the compiler.
- [oli-obk](https://github.com/oli-obk), maintainer
    - oli-obk is a long-time compiler team member whose prolific contribution history is a
      challenge to summarize, but include constant evaluation, constant generics, pattern types,
      MIR optimisations, diagnostics, clippy improvements, and existential types.
- [petrochenkov](https://github.com/petrochenkov), maintainer
    - petrochenkov is another long-time compiler team member who primarily maintains the compiler's
      name resolution and macro expansion, notoriously tricky and nuanced subsystems of the
      compiler.
- [pnkfelix](https://github.com/pnkfelix), maintainer
    - Former compiler team co-lead, pnkfelix is yet another long-time team member has made
      contributions throughout the compiler and made significant contributions to the borrow
      checker, early MIR, and early compiler architecture.
- [RalfJung](https://github.com/ralfjung), team member
    - Known for his work on Miri and Stacked Borrows, an operational semantics for memory accesses
      in the language, RalfJ is the team's foremost expert on the operational semantics of the
      language and also deeply involved in const evaluation. He has been working on achieving sound
      semantics down to lowest levels of the compiler.
- [saethlin](https://github.com/saethlin), maintainer
    - saethlin has made significant improvements to MIR, Miri, and codegen in their contributions
      since starting in 2021 and has become an invaluable source of knowledge on the language's
      operational semantics/unsafe code guidelines.
- [scottmcm](https://github.com/scottmcm), team member
    - scottmcm is a member of the language team who also regularly implements improvements in the
      compiler, particularly in the MIR and compiler's codegen, always trying to get to the perfect
      machine code.
- [SparrowLii](https://github.com/sparrowlii), maintainer
    - SparrowLii is a relatively new member of the team who has resurrected and led the effort to
      parallelize the compiler and has been responsible for the great strides that effort has made
      alongside members of the parallel rustc and compiler performance working groups.
- [spastorino](https://github.com/spastorino), maintainer
    - spastorino is another alum of the non-lexical lifetimes working group, starting to contribute
      in late 2017. Since NLL, spastorino has implemented negative impls in coherence, refactored
      return position impl trait in trait code to lower as a GAT, and has done a lot of refactors/bugfixes
      to the compiler.
- [TaKO8Ki](https://github.com/TaKO8Ki), team member
    - TaKO8Ki has made lots of diagnostic improvements, helped the team keep on top of regressions
      by adding plenty of regression tests, done lots of refactorings and cleanup, fixed a bunch of
      ICEs in their time contributing.
- [tgross35](https://github.com/tgross35), team member
    - tgross35 has been contributing for two years and has been leading the implementation of the
      new `f16` and `f128` types.
- [the8472](https://github.com/the8472), team member
    - the8472 has been a contributor since 2020 and has helped to improve the quality of code
      generated by the compiler via changes to the Rust standard library.
- [tmandry](https://github.com/tmandry), team member
    - tmandry has been leading the async working group since its inception and has made remarkable
      contributions to pushing forward the async support in the compiler.
- [tmiasko](https://github.com/tmiasko), team member
    - tmiasko has been contributing for almost four years and has reliably completed invaluable
      work on the compiler's MIR representation, optimisations and LLVM code generation, becoming
      one of the team's experts in these areas.
- [Urgau](https://github.com/Urgau), team member
    - Urgau has worked on a wide variety of improvements in their time contributing, from
      `check-cfg` to range patterns, `black_box` to lints, and much more.
- [WaffleLapkin](https://github.com/WaffleLapkin), team member
    - WaffleLapkin has been another staple of the contributor community since 2020, doing a
      variety of essential refactors, bug fixes and performance improvements. Like Nora above,
      Waffle is a prolific contributor whose improvements across the compiler have a major impact.
- [wesleywiser](https://github.com/wesleywiser), maintainer + team co-lead
    - wesleywiser is co-lead of the compiler team and has been contributing since 2015 while
      working on various parts of the compiler including self-profiling, incremental compilation,
      MIR optimizations and Windows & Linux debugging support.
- [Zalathar](https://github.com/Zalathar), team member
    - Zalathar has been contributing for a little over a year and has had a massive impact,
      working on significant and widespread refactorings to the compiler's support for code
      coverage instrumentation.

There are also some team members choosing to become alumni who have made valuable contributions
during their time as team members:

- [Aaron1011](https://github.com/Aaron1011)
    - Aaron1011 contributed widely since 2017, touching basically every part of the compiler,
      fixing bugs, making improvements and doing important refactorings wherever they went.
- [eddyb](https://github.com/eddyb)
    - eddyb is a prolific and incredibly knowledgable member of the compiler team who has made
      widespread improvements throughout the compiler for many years. Much of their work has
      focused on the LLVM backend, the initial implementation and improvements to MIR, the v0 Rust
      name mangling scheme, and a huge number of bug fixes and architectural improvements to the
      compiler.
- [michaelwoerister](https://github.com/michaelwoerister)
    - Another long-time compiler team member, michaelwoerister has been responsible for huge
      improvements and progress in the project's debuginfo, codegen, incremental compilation, LTO
      and PGO, going back to 2013.

Thank you to our past, current and future compiler team members and maintainers!
