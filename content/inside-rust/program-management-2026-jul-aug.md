+++
path = "inside-rust/9999/12/31/program-management-2026-jul-aug"
title = "Program management in July–August 2026"
authors = ["Nurzhan Saken"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++

It's been _fun_ since the last update. Tomáš was away for a month, and I had to get my wisdom teeth pulled right after he left.

Despite everything, a lot has happened!

## Project Goals

Since the previous update, we've opened seven more goals:

- [Contracts: primitive ownership assertions][goals#734]
- [Process-Safe Rustup Toolchain Operations][goals#736]
- [End-to-End Executable Rust Specification][goals#749]
- [`f16b` Primitive Type][goals#757]
- [Polymorphic code generation experiment][goals#758]
- [C interop: `Complex<T>`][goals#761]
- [Allocators 1.0][goals#762]

[goals#734]: https://github.com/rust-lang/goals/issues/734
[goals#736]: https://github.com/rust-lang/goals/issues/736
[goals#749]: https://github.com/rust-lang/goals/issues/749
[goals#757]: https://github.com/rust-lang/goals/issues/757
[goals#758]: https://github.com/rust-lang/goals/issues/758
[goals#761]: https://github.com/rust-lang/goals/issues/761
[goals#762]: https://github.com/rust-lang/goals/issues/762

There's now a roadmap for [Binary size reduction][roadmap-binary-size], which includes [async future][goals#622] and [statemachine optimization][goals#623] goals, [build-std][goals#274], and future possibilities that aren't goals yet.

[roadmap-binary-size]: https://goals.rust-lang.org/2026/roadmap-binary-size-reduction.html
[goals#622]: https://github.com/rust-lang/goals/issues/622
[goals#623]: https://github.com/rust-lang/goals/issues/623
[goals#274]: https://github.com/rust-lang/goals/issues/274

One goal was completed ([Stabilize Cargo's linting system][goals#650]), and two were discontinued ([Experimental language specification][goals#627] and [Continue Experimentation with Pin Ergonomics][goals#389]).

[goals#650]: https://github.com/rust-lang/goals/issues/650#issuecomment-5451728278
[goals#627]: https://github.com/rust-lang/goals/issues/627#issuecomment-5455597457
[goals#389]: https://github.com/rust-lang/goals/issues/389#issuecomment-4703217795

Several goals have received partial funding ([Incremental Systems Rethought][goals#641], [Native async fn dynamic dispatch in traits][goals#625], and [Cargo cross workspace cache][goals#626]), and some are now fully funded ([C interop: `f80`, `f128`, and `c_longdouble`][goals#701], [C interop: `Complex<T>`][goals#761], and [Async statemachine optimization][goals#623]). Don't forget to add your goal to the [funding table](https://goals.rust-lang.org/2026/funding.html) if it needs funding ([see template](https://goals.rust-lang.org/TEMPLATE.html#funding))!

[goals#641]: https://github.com/rust-lang/goals/issues/641
[goals#625]: https://github.com/rust-lang/goals/issues/625
[goals#626]: https://github.com/rust-lang/goals/issues/626
[goals#701]: https://github.com/rust-lang/goals/issues/701

Some goal owners and contributors have been writing about their work beyond the regular goal updates. Here are some of the places to check out:

- [Monthly updates][hexcat-updates] by [Hexcat][hexcat]
- [Rust Maintainer Monthly Report: August 2026][rami3l-august] by [Gen Li][rami3l]
- [Status Update - July 2026][bsan-update] by the [BorrowSanitizer team][bsan]
- [Field Projection Designs][field-proj] by [Benno Lossin][benno]
- [Four levels of in-place initialization][in-place] by [Yoshua Wuyts][yoshua]
- [Optimizing the new trait solver][trait-solver] by [Jana Dönszelmann][jana]
- [Rust Function Overloading - Call for Experimentation][overloading] by [teor][teor]
- [A Vision for a Rust Formal Specification][spec-vision] by [Nadrieril][nadri]

[hexcat-updates]: https://hexcat.nl/updates/
[rami3l-august]: https://rami3l.github.io/posts/rust-worklog-2026-08/
[bsan-update]: https://borrowsanitizer.com/status/july_2026.html
[field-proj]: https://bennolossin.github.io/field-projections-designs/
[in-place]: https://blog.yoshuawuyts.com/four-levels-of-in-place-initialization
[trait-solver]: https://donsz.nl/blog/new-solver-performance
[overloading]: https://blog.rust-lang.org/inside-rust/2026/08/19/overloading-experiment/
[spec-vision]: https://nadrieril.github.io/blog/2026/06/16/formal-spec-vision.html
[hexcat]: https://github.com/hexcatnl
[rami3l]: https://blog.rust-lang.org/inside-rust/2026/07/07/maintainer-spotlight-gen-li-rami3l/
[bsan]: https://github.com/borrowsanitizer
[benno]: https://github.com/BennoLossin
[yoshua]: https://github.com/yoshuawuyts
[jana]: https://github.com/jdonszelmann
[teor]: https://github.com/teor2345
[nadri]: https://github.com/Nadrieril

### Infrastructure

The Goals repository has been renamed to [rust-lang/goals][goals] from the verbose [rust-lang/rust-project-goals][goals], and we've renamed the Zulip channels to [#goals], [#goals/proposed], and [#goals/meta]. The website now lives at [goals.rust-lang.org][goals-web]; the old links should redirect there.

Triagebot used to ping every [#goals] topic twice a month, which was [incredibly noisy][ping-noise] to anyone following that channel. Many of the goal owners weren't even subscribed to it, so they never received the reminders. [This was finally reworked][ping-rework], and now goal owners receive one private message with an aggregate of all their goals needing updates. Some wanted triagebot to [ping them more often][ping-often], and it's now possible to configure the reporting frequency per goal (weekly, biweekly, or the default of every four weeks). [Let us know][ping-reports] how you like it!

We've set up automatic creation of Zulip topics for [labeled goal proposals][goal-proposals] in [#goals/proposed] so that there's less friction in soliciting team champions and collecting feedback.

[ping-noise]: https://rust-lang.zulipchat.com/#narrow/channel/478266-goals.2Fmeta/topic/changing.20the.20way.20we.20update/with/610622235
[ping-rework]: https://github.com/rust-lang/triagebot/pull/2469
[ping-often]: https://rust-lang.zulipchat.com/#narrow/channel/478266-goals.2Fmeta/topic/different.20update.20cadences/with/607428208
[ping-reports]: https://rust-lang.zulipchat.com/#narrow/channel/478266/topic/triagebot.20reports

### Team building

The Goals team now has a [regular meeting][goals-meeting] every Thursday at 14:00 UTC. We use that time to figure out what the program should mean, how it should be implemented, and how we can communicate it better to the various audiences involved.

If you have any wishes, complaints, or thoughts about the Goals program, please [join the meeting][goals-calendar] or [reach out to us][goals-members]!

[goals]: https://github.com/rust-lang/goals
[goals-web]: https://goals.rust-lang.org
[goal-proposals]: https://github.com/rust-lang/goals/pulls?q=is%3Apr+label%3AC-goal-proposal
[#goals]: https://rust-lang.zulipchat.com/#narrow/channel/435869
[#goals/proposed]: https://rust-lang.zulipchat.com/#narrow/channel/546987
[#goals/meta]: https://rust-lang.zulipchat.com/#narrow/channel/478266
[goals-meeting]: https://rust-lang.zulipchat.com/#narrow/channel/478266-goals.2Fmeta/topic/Recurring.20meeting
[goals-calendar]: https://rust-lang.github.io/calendar/goals.ics
[goals-members]: https://rust-lang.zulipchat.com/#groups/745280/T-goals/members

## Content

Seems like we've accidentally created a rite of passage in which a program manager has to [anxiously get involved in at least one t-content production][pm-2025-09] :-). I've mentioned before that I got to interview [Alice Cecile][alice-cecile] ([Bevy Engine][bevy]) at RustWeek. This and twelve other interviews are now [available on YouTube][youtube-rustweek], so please check them out!

The content team [has started publishing interviews from RustConf and Kangrejos 2025][youtube-kangrejos] as well.

Apart from that, the team has been producing a series about Rust release changelogs. [Here's the 1.97 episode!][youtube-changelog] They're hoping to involve more people from across the Project in future episodes (and other videos), so that maintainers and contributors can talk about their work directly. Be on the lookout for a call for participation if you're interested or know someone who might!

Fun fact: I actually met [Chris Biscardi][chris-biscardi] at RustWeek and invited him to a content team meeting about the changelog series. One thing led to another, and Chris ended up joining the team and has been bringing in a lot of valuable content-making experience since! This makes me happy.

[alice-cecile]: https://github.com/alice-i-cecile
[bevy]: https://bevy.org/
[pm-2025-09]: https://blog.rust-lang.org/inside-rust/2025/10/14/program-management-update-2025-09/#t-content
[youtube-rustweek]: https://www.youtube.com/watch?v=9knWm4hEoYM&list=PLTpi2LlwcDO4&index=6
[youtube-kangrejos]: https://www.youtube.com/watch?v=HPRSKYnhP7Q&list=PLTeBDijbsY6w&index=1
[youtube-changelog]: https://www.youtube.com/watch?v=lUoQ3uGSQA0
[chris-biscardi]: https://github.com/christopherbiscardi

## Rust for Linux

This cycle, Rust for Linux brought up some more interesting feature requests.

### The core of `core`?

One of the topics Rust for Linux [has been tracking][rfl-core-wanted] is _modularization of `core`_. This is the ability to remove parts of the [core standard library][core] that the kernel doesn't need because of semantic differences or duplication. This includes standard atomics, IO, 128-bit numbers, floating-point numbers, and Unicode tables.

There are different ways to support such modularization, each with its own caveats. Perhaps, the most fundamental one is to keep every feature under its own flag, but that requires building the standard library ([see the build-std goal][build-std]) and introduces a combinatorial explosion that's difficult to test. The standard library could also expose a small number of "feature layers" instead, but then it becomes hard to find a partitioning that makes sense for everyone.

Another approach is to define the smallest possible subset of `core` that's needed for the language to work. This subset, tentatively named `lang`, would only include things like [lang items][lang-items] and could lead to a stable alternative to the internal [`#![no_core]` feature][no-core]. For now, that seems most realistic, and the RfL team is willing to experiment with it and possibly develop a proposal. Can we get a Project Goal out of this?

[core]: https://doc.rust-lang.org/core/index.html
[rfl-core-wanted]: https://github.com/Rust-for-Linux/linux/issues/514
[build-std]: https://goals.rust-lang.org/2026/build-std.html
[no-core]: https://github.com/rust-lang/rust/issues/29639
[lang-items]: https://doc.rust-lang.org/unstable-book/language-features/lang-items.html

### Integer casts

There's an ongoing appetite for more intentional casting mechanisms between integers. The kernel already lints against lossless casts, and recently introduced a `num::casts` module for lossless conversions that are missing from the core library (e.g., `u32 -> usize` and `usize -> u64`). This leaves the non-trivial `as` casts that may truncate or change signedness.

One option would be to lint against `cast_possible_truncation`, `cast_sign_loss`, and `cast_possible_wrap`, and justify each warning with an explicit `#[expect(..., reason = "...")]`. This gets awkward in kernel code, though, because lint expectations may depend on configuration ([see coding guidelines][rfl-doc-lints]). [Miguel has also mentioned][miguel-expect] that the `reason` syntax is somewhat verbose, hence the existing request for tooling support for tagged comments like `// CAST: ...`.

We may also get a resolution to this from the API side: the libs team is actively discussing numeric casting APIs, which may address this. [See this meta-ACP.][cast-meta-acp]

[cast-meta-acp]: https://github.com/rust-lang/libs-team/issues/833
[rfl-doc-lints]: https://www.kernel.org/doc/html/latest/rust/coding-guidelines.html#lints
[miguel-expect]: https://lore.kernel.org/all/CANiq72k21-kke0XFNf0iQ5gpECjAtEPg00tm-JwpNjff34n5+g@mail.gmail.com/

### Newtype literal constructors

We also discussed improving the ergonomics of `NonZero` and other newtypes. Specifically, it would be nice to be able to construct such types from bare literals:

```rust
const NONZERO_OK: NonZeroU8 = 42; // works
const NONZERO_BAD: NonZeroU8 = 0; // error
```

There have been proposals to [generalize this to user types][from-literal], e.g., through a `FromLiteral` trait family, but nothing that's usable today. So far, the kernel developers have been working around this (and the lack of const traits) using [macro hacks][rfl-cv].

[from-literal]: https://internals.rust-lang.org/t/pre-rfc-trait-for-coercion-of-untyped-numeric-literals
[rfl-cv]: https://lkml.iu.edu/2608.3/09777.html

## Project updates

Here are some of the recent Project updates you might have missed:

- [All Hands 2026 retrospective](https://blog.rust-lang.org/inside-rust/2026/07/31/all-hands-2026-retrospective/)
- [Funding team progress update — July 2026](https://blog.rust-lang.org/inside-rust/2026/08/04/funding-team-progress-update-july-2026/)
- [rust-lang/rust is adopting an LLM policy](https://blog.rust-lang.org/inside-rust/2026/08/05/rust-langrust-is-adopting-an-llm-policy/)
- [Leadership Council September 2026 Representative Selections](https://blog.rust-lang.org/inside-rust/2026/08/18/leadership-council-repr-selection/)
- [Announcing our first Maintainers in Residence](https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/)
- [Electing new Project Directors 2026](https://blog.rust-lang.org/inside-rust/2026/08/28/electing-new-project-directors-2026/)

Until next time! <3
