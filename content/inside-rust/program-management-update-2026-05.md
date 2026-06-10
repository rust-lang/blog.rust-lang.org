+++
path = "inside-rust/9999/12/31/program-management-update-may-2026"
title = "Program management update — May 2026"
authors = ["Tomáš Šedovič and Nurzhan Saken"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++
# Program management update — May 2026

## RustWeek and All Hands!

It finally happened. Nurzhan, Tomáš, and hundreds of Rustaceans all met up in Utrecht, the Netherlands for [RustWeek and All Hands](https://2026.rustweek.org/).

RustWeek is a week-long conference organized by [RustNL](https://rustnl.org/). It typically features two days of talks, three to four days of workshops, an unconference, and a hackathon. It's absolutely fantastic and open to everyone. The All Hands is a three-day event of team- and topic-specific sessions, bringing Project members together to meet, collaborate, resolve issues, and plan ahead. For the last two years, these events were co-located, so most Project members got to attend both.

The dates for [RustWeek 2027](https://2027.rustweek.org/) are already set: May 24–29, in Utrecht again. The dates or location for the All Hands have not yet been announced.

We'll post a retrospective later, but in the meantime, it was fantastic to meet everyone.

### Interviews

During the conference, the Content team recorded twelve interviews with some of the speakers and people in the Project. We got involved too: Nurzhan was really excited (and anxious!) to meet and interview Alice Cecile (Bevy Engine) on program/project management. The interviews have been fully produced by a videographer hired from the team's budget, and they should be posted in the next month or so.

Apart from that, the team has a backlog of interviews recorded at the last Kangrejos and RustConf that is over nine months old now. They hired an editor to take care of them, so we're excited to see more interviews posted soon.

## Program management

We published the [final update for the 2025H2 Project goals](https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/), and kicked off the new 2026 period.

We had some goals-related sessions at the All Hands and got to talk to people about their needs and experiences. There was also a discussion about funding for maintainers which we plan to integrate into goals.

We're now working on displaying funding information (goals looking for funding, goals that are funded, etc.) on the [Project Goals page](https://rust-lang.github.io/rust-project-goals/2026/index.html). For an example, see the [Funding section of the Fast Builds page](https://rust-lang.github.io/rust-project-goals/2026/roadmap-fast-builds.html#funding).

Some other things we're looking into:

* The feasibility of using goals ubiquitously instead of the current Compiler MCPs (Major Change Proposals), Library ACPs (API Change Proposals), and Language experiments.
  * The Types team set a precedent for this recently by [deciding](https://github.com/rust-lang/types-team/issues/134) to move away from MCPs in favor of goals.
* Making the creation of new goals more straightforward and lightweight.
* Streamlining goal updates and making them more useful.
* Refactoring the tooling for managing goals.
* Closer integration with the Funding team, grants, etc.

Basically, we expect goals to be opened throughout the year on a rolling basis. How do we handle that?

We have also expanded the coverage of meetings. The Rustdoc team used to have chat-based meetings, but recently decided to try video calls instead, and asked us to minute them. We also reached out to the Types team asking them to move their meeting to a non-conflicting time slot (30 minutes earlier) so that we could start attending those too. Hopefully, this will help us keep the teams more connected.

## Funding Rust maintainers

The [Rust Foundation Maintainer Fund (RFMF) RFC](https://rust-lang.github.io/rfcs/3931-rfmf-rust-foundation-maintainer-fund.html) has been merged.

The Foundation will begin raising money for the Maintainer Fund. This money will be dedicated to people doing maintenance work. This includes things such as reviewing, triaging, large-scale refactoring, and development that's largely invisible but critical to the Project going forward or needed to unlock new features.

The RFC establishes a Maintainer in Residence program (MiR — not to be confused with [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)) to provide long-term, mostly full-time support for maintainers — complementing the shorter-term [Grants program](https://github.com/rust-lang/rfcs/pull/3919).

This program was heavily inspired by Python's [Developers in Residence](https://www.python.org/psf/developersinresidence/).

The [Funding team](https://rust-lang.org/governance/teams/launching-pad/#team-funding) wrote about the program [here](https://blog.rust-lang.org/2026/06/02/launching-the-rust-foundation-maintainers-fund/), and the Foundation published a [companion piece on their site](https://rustfoundation.org/media/help-fund-the-people-who-build-rust/).

If you're an individual, you can [donate money to the Maintainer Fund via this GitHub sponsor page](https://github.com/sponsors/rustfoundation). If you're a company or larger organization, reach out to <maintainers-fund@rustfoundation.org>. All proceeds will go directly to the maintainers.

If you want to support specific people, head to the [Funding page](https://rust-lang.org/funding/).

And last but not least, RustNL has recently hired several Rust maintainers as well. [Visit their page to learn more](https://rustnl.org/maintainers/).

## Mirroring

At the All Hands, Joel Marcey distributed YubiKeys that will be used for mirror attestations. Emily, Walter, and Dirkjan put together a guide on how to use them (private for now, undergoing feedback) and verify attestation.

Arlo has practiced root signing and shared examples [here](https://github.com/arlosi/tuf-on-ci).

The team discussed whether [TUF](https://theupdateframework.io/) is the right solution for the crates.io index. TUF was originally designed for offline signing, but the crates.io index changes very frequently and will need to be signed every minute.

The main concern was that following the TUF spec to the letter would be infeasible due to resulting in too many HTTP requests. The standard seems to be moving in the right direction to solve this (they're listening to our input), but we don't want to be blocked on that.

There are options the team could consider, but they'd make crates.io incompatible with other TUF clients.

Walter said that we don't necessarily need to support other clients for the crates.io use case, and that the changes we'd need to do to [rust-tuf](https://github.com/theupdateframework/rust-tuf) aren't substantial enough for this to be a problem.

Rustup doesn't have the same constraints, so we'll be following the spec there.

## Rust for Linux

### All Hands session

The Rust for Linux team was invited to the All Hands and hosted an office hours session there. There were rustfmt and clippy representatives in [the room](https://github.com/rust-lang/all-hands-2026/issues/18).

Jieyou Xu wanted to clarify what exactly the Linux team needs to remove the ["trailing double slash hack"](https://docs.kernel.org/rust/coding-guidelines.html#imports). The team wants this option to be stabilized at some point, but in the meantime they'd be okay with using something like `RUSTC_BOOTSTRAP` (which allows using unstable features with a stable toolchain). Rustfmt, however, doesn't have a way to do this today.

The team runs rustfmt in CI, but they're currently only looking for crashes (ICE — internal compiler error). They want get to a place where they could catch unintentional differences in code formatting, and enforce consistent style in CI.

Alejandra González mentioned that getting changes into clippy is now bottlenecked by reviewers. Reviewing clippy code is complex. There's a lot of things to check, and edge cases to investigate. But Alejandra stressed that if someone is interested, they'd be happy to teach them, and they'd appreciate the help.

### Recoverable integer overflow

Jana Dönszelmann opened a (draft) [pull request providing a numerical overflow handler](https://github.com/rust-lang/rust/pull/157314). This is something the team has wanted for a long time. Currently, Rust will panic on integer overflows (e.g. adding `1` to a `u8` value of `255` — since there's no `256` in `u8`) in debug mode, and wrap (without panicking) in release.

The kernel needs the ability to detect overflows and react to them in some fashion. Jana's PR provides such a hook:

```rust
//@ no-prefer-dynamic
//@ compile-flags: -Coverflow-checks=recoverable
//@ run-pass
//@ check-run-results
#![feature(recoverable_integer_overflow)]
#![allow(arithmetic_overflow, unused)]

#[core::panic::integer_overflow_action]
fn overflow() {
    println!("overflow happened")
}

fn main() {
    let mut x = 255u8;
    x += 1;
}
```

The likely use case for this on the Linux side will be to print a kernel message the first time such a call happens (possibly tainting the kernel in the process) and keep going without spamming the log with more messages.

## Tomáš's July/August absence

As a heads-up, Tomáš will be on medical leave from 2026-07-12 until 2026-08-09. There is nothing to be worried about! Nurzhan will handle the work side of things.

We put our planned time off in the [PM calendar](https://calendar.google.com/calendar/u/0?cid=N2RmMTJiZjc2YTFlZjU5MWNlYTQ4MmJjNDMxODEzYzcxZDg1MDViMTFhODk1NjQ1MzUxNjQ5ZTkwZWQ2NzUwNUBncm91cC5jYWxlbmRhci5nb29nbGUuY29t) and the [PM time-off Zulip thread](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/PM.20time.20off/with/595113250).

## Worth a look

### Rust Project updates

* [Maintainer spotlight: Tiffany Pek Yuan (@tiif)](https://blog.rust-lang.org/inside-rust/2026/06/03/maintainer-spotlight-tiffany-pek-yuan-tiif/)
* [March 2026 Project Director Update](https://blog.rust-lang.org/inside-rust/2026/05/04/project-director-update/)
* [Launching the Rust Foundation Maintainers Fund](https://blog.rust-lang.org/2026/06/02/launching-the-rust-foundation-maintainers-fund/)
* [Announcing Rust 1.96.0](https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/)
* [Security Advisory for Cargo (CVE-2026-5223)](https://blog.rust-lang.org/2026/05/25/cve-2026-5223/)
* [Security Advisory for Cargo (CVE-2026-5222)](https://blog.rust-lang.org/2026/05/25/cve-2026-5222/)
* [Project goals update — April 2026 (end of 2025H2)](https://blog.rust-lang.org/2026/05/18/project-goals-2026-04/)

### Rust Foundation posts

* [Help Fund the People Who Build Rust](https://rustfoundation.org/media/help-fund-the-people-who-build-rust/)
* [Congratulations, Walter Pearce: OpenSSF Ambassador!](https://rustfoundation.org/media/congratulations-walter-pearce-openssf-ambassador/)
* [Rust Foundation and Package Registry Leaders Unite to Address Open Source Sustainability Crisis](https://rustfoundation.org/media/rust-foundation-and-package-registry-leaders-unite-to-address-open-source-sustainability-crisis/)

## Stats

Total words of meeting minutes written: 593.9k (June 2025–May 2026)

Meetings attended: 23

Total words of meeting minutes written (May): 42.6k

Average (mean) word count per team meeting:

* Cargo: 1.6k
* Lang triage: 3.5k
* Libs-API: 4.8k
* Leadership Council: 2.6k
