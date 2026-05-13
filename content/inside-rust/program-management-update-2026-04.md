+++
path = "inside-rust/9999/12/31/program-management-update--april-2026"
title = "Program management update — April 2026"
authors = ["Tomáš Šedovič and Nurzhan Saken"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++

Greetings! This was a busy month. The Project goals RFC has been accepted and we're officially in the 2026 goals period. At the time of writing, RustWeek is exactly one week away. Time flies!

## Program management schedule

Nurzhan and Tomáš have settled on our meeting schedule. Roughly speaking, we're taking turns attending each meeting and we've set it up to reduce the amount of back-to-back meetings and long evenings. We're also getting some call-free Fridays, which is always nice.

[The calendar is publicly accessible](https://calendar.google.com/calendar/u/0?cid=N2RmMTJiZjc2YTFlZjU5MWNlYTQ4MmJjNDMxODEzYzcxZDg1MDViMTFhODk1NjQ1MzUxNjQ5ZTkwZWQ2NzUwNUBncm91cC5jYWxlbmRhci5nb29nbGUuY29t) and we keep it up-to-date.

## The 2026 RustWeek / All Hands

In the week of 18th May, Utrecht will briefly become a city with the highest per-capita Rustacean concentration. Just like last year, it will host the co-located RustWeek and Rust All Hands conferences!

The pre-registration and workshops will start on Monday, with the actual RustWeek conferences on the Tuesday and Wednesday, followed by three days of the All Hands.

Have a look at the [speakers](https://2026.rustweek.org/speakers/), [schedule](https://2026.rustweek.org/overview/), check out the website for more information and if you plan to go, [don't forget to buy a ticket](https://event.onliveevent.nl/rustweek-2026).

The two of us plan to be there. Feel free to say hi!

## Project goals

The [2026 Project goals RFC has been accepted](https://rust-lang.github.io/rfcs/3935-Project-Goals-2026.html)! This happened about a month later than we planned. We'll keep that in mind for the next year and as most of the Project goals team members are going to be at the All Hands, we plan to do a retrospective there.

All the new tracking issues have been created and the continuing ones have been updated too. Any goals that weren't renewed for 2026 are now closed.

If you're a point of contact on a goal, you will start getting bot messages reminding you to post an update on the tracking issue. These reminders happen roughly twice a month and won't trigger for you if you've already posted an update recently.

Now that the RFC is merged, we'll publish one last 2025h2 Project goals update blog post. And then (possibly kicking things off at the All Hands again), we'll plan how we'll be doing updates going forward. There's been several issues brought up with the current way the posts are formatted, the information they show, and whether they're useful at all.

## Outreachy

Throughout April, the [Outreachy](https://www.outreachy.org/) internship applicants were getting set up and started sending their contributions. A few days ago, the organizers selected four interns.

These are the projects our interns signed up for:
* Calling overloaded C++ functions from Rust.
* Code coverage of the Rust compiler at scale.
* Fuzzing the a-mir-formality type system implementation.
* Improve the security of GitHub Actions of the Rust Project.

You can read more about the projects and Outreachy in general in [Jack Huey's blog post][outreachy-blog].

The interns will work on these projects over the next three months. Congratulations to the selected interns and huge thanks to Jack, the mentors, and everyone else who participated <3.

Exciting times ahead!

[outreachy-blog]: https://blog.rust-lang.org/2026/05/04/outreachy-2026-may/

## C++ interop update

In February 2026, the Foundation hired teor to speed up the mapping of the interop problem space and their work continues apace.

teor has been posting monthly updates on the [C++ interop Project goal tracker](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3970777014) and [weekly ones on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop/topic/Interop.20Problem.20Space.20Mapping.20-.20Weekly.20Updates/with/574868164).

We now have around [30 interop problem statements and use cases](https://github.com/rustfoundation/interop-initiative/issues) listed in [the interop-initiative repo](https://github.com/rustfoundation/interop-initiative). If you're at all interested in this, do check it out. The [problem statements](https://github.com/rustfoundation/interop-initiative/tree/main/problem-space) are really well described, include example code and links to prior art.

[Here is a string interop example](https://github.com/rustfoundation/interop-initiative/blob/main/problem-space/0002-string-interop.md).

As part of the contribution round, the Outreachy applicants provided executable code samples, e.g. [calling overloaded C++ functions from Rust](https://github.com/rustfoundation/interop-initiative/tree/main/examples/cpp-overloading).

teor also started a lang experiment to [implement the `splat` feature](https://github.com/rust-lang/rust/issues/153629). This will provide a way to define function overloading and variable arguments in Rust. The initial focus is to improve interoperability with C++ which does allow both.

In addition to this work showing direct impact, Joel Marcey published an [update on the Rust Foundation Interop Initiative](https://rustfoundation.org/media/rust-foundation-interop-initiative-update-from-research-to-implementation/). The post goes over the initiative's accomplishments. These include working with ISO WG21 (the committee responsible for the C++ standardization). There seems to be a strong consensus for providing memory safety mechanisms to C++, but this will take multiple years.

So the Foundation is now focusing more on the immediate needs of projects with large C++ codebases who want to (or already started to) adopt Rust. This is where all the work teor's been doing comes in.

## Content team

The last few months have been difficult to dedicate time to producing useful content for everyone on the team. We've published a couple of interviews ([Daniel Almeida on a Rust GPU driver in Linux](https://www.youtube.com/watch?v=rgjTPBRae6I) and [Tyler Mandry on C++/Rust interop](https://www.youtube.com/watch?v=wzDESESJH8A)), and [a Rust 1.95 changelog overview](https://youtu.be/NZlmaIgkUQ8?si=EN4q-DALc9ZYna-p), but we have a backlog of another 9 recordings in the pipeline.

A lot of the folks will also be at the All Hands and we're planning to do more interviews there.

What became clear is that the volunteering contribution model that works for code, documentation, etc., is less successful when it comes to things like recording and editing video. We're able to do this (as demonstrated by what we already put out), but the time commitment is really high compared to what a professional could do even at the lowest level of effort.

To that end, [TC proposed a 2026 funding request to the Council](https://github.com/rust-lang/leadership-council/pull/279). In it, we asked for $15,000 USD to hire an editor who would process our existing backlog as well as videographers for the events we'll conduct the bulk of our upcoming interviews at (All Hands and RustConf).

The Council has approved this request, and we're now in talks with people we might hire.

This would let us focus on utilizing our knowledge and connections to interview great people and share their thoughts with the rest of the community.

## Rust for Linux

A lot of the Rust for Linux folks will attend the RustWeek and/or All Hands. The Rust Project also extended invitation to the kernel developers and some of them will be there too.

The project will hold a few sessions at the All Hands: [In-place Initialization](https://github.com/rust-lang/all-hands-2026/issues/17), [Field Projections](https://github.com/rust-lang/all-hands-2026/issues/16), [CoccinelleForRust](https://github.com/rust-lang/all-hands-2026/issues/9), [Compiling Rust to BPF](https://github.com/rust-lang/all-hands-2026/issues/10) and [office hours](https://github.com/rust-lang/all-hands-2026/issues/18).

We continued having our regular fortnightly meetings; here's what we discussed in April:

### zerocopy features in std

Given the plan to include [zerocopy](https://github.com/google/zerocopy) in Linux, the team brought up the possibility of stabilizing a few unstable pieces of the standard library that zerocopy ends up re-implementing.

These are [`ptr_metadata`](https://doc.rust-lang.org/nightly/std/intrinsics/fn.ptr_metadata.html) and [`Freeze`](https://doc.rust-lang.org/nightly/std/marker/trait.Freeze.html). Having these in would [help with the zerocopy maintenance](https://github.com/rust-lang/rust/issues/81513#issuecomment-2414679019).

[`Freeze` now has an open RFC as `core::marker::NoCell`](https://github.com/rust-lang/rfcs/pull/3633) while `ptr_metadata` is blocked on [Sized Hierarchy](https://github.com/rust-lang/rust/issues/144404).

### null-ptr-deref

The team would like to have an (optional) guarantee that the compiler will never remove null checks on raw pointers. In C, dereferencing a null pointer is undefined behavior, so the compiler may optimize away subsequent null checks against that pointer. However, the null check can still serve as a safeguard against other bugs, and in C, the kernel now disables the optimization that would remove it.

### Edition migration tooling

The team wants to move to the new edition, as it would fix some of the existing bugs and provide nicer language and library APIs. However, they're concerned about situations where the same code has different semantics in different editions.

Making semantic changes is something that the language explicitly allows, and while we provide tooling to migrate or at least highlight these cases to prevent breakages, this tooling cannot guarantee catching 100% of the issues.

A challenge with the current approach is that when the kernel migrates to a new edition, the previous releases will stay on older editions (potentially more than one), and these situations can be really easy to miss when backporting a change. Anything that changes the drop order (e.g. the [`if let` temporary scope change](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html) in the 2024 edition) could result in undefined behavior (UB).

Miguel Ojeda brought this up to check on what guarantees the language and tooling make. They're looking for a way to flag cases where the same code has different semantics with zero false negative rate (i.e. it will not let something slip by).

TC explained that the current migration tooling (edition lints and `cargo fix`) is not set up to make this a guarantee, although we try to get as close as possible and put a lot of effort into migration testing.

Benno Lossin suggested writing tooling that would compile the code being evaluated under both editions and compare the MIR output. If they're identical, no semantic changes occurred. Otherwise, there may or may not be changes. This risks generating false positives, but at least the reviewers can be confident that nothing has been silently accepted. This seems feasible and might be something the Project would be interested to adopt and maintain, but there's no bandwidth to do this within the edition team.

Interestingly, the edition migration itself isn't an issue — the Rust for Linux team pays attention to the edition migration guide and checks things. The main issue is with backports, which are semi-automatic. If a patch is flagged for a backport, and it applies cleanly, there's little human supervision involved. When the backport patch (coming from a newer edition) uses new features, compilation fails on the older edition, flagging people to pay attention. However, when the code looks identical in both editions but behaves differently, we may run into issues.

In the end, several options were proposed, each with their own trade-offs:

* Stay on an edition until all the stable branches move to a MSRV (minimal supported Rust version) that allows migrating to a new edition wholesale.
* Develop an outside tool that would compile the code in both editions and check whether they produce the same MIR.
* Develop review processes for backporting patches on the kernel side to handle these cases.
* Develop processes and tooling within the kernel for writing all code such that it always falls within the intersection between all editions supported on all branches.

This is a topic we will almost certainly come back to.

## Project Director election process update

In autumn 2025, Tomáš facilitated the Project Director election. A part of the facilitator role is to publish a retrospective, clarify the documentation, and propose changes to the election process (if relevant).

This is the [2025 PD election retrospective](https://hackmd.io/4SgDbeQ9Sd2KlknfT85X0Q). The [PR that clarifies the election processes is here](https://github.com/rust-lang/leadership-council/pull/286).

## Worth a look

### Rust Foundation posts

* [Announcing the 2026 Rust-Edu Refresh and CFP](https://rustfoundation.org/media/guest-post-announcing-the-2026-rust-edu-refresh-and-cfp/)
* [Project Director Update — March 2026](https://rustfoundation.org/media/project-director-update-march-2026/)
* [Welcoming Symposium to the Rust Innovation Lab](https://rustfoundation.org/media/welcoming-symposium-to-the-rust-innovation-lab/)
* [RustConf 2026: Speakers Announced, Registration Open](https://rustfoundation.org/media/rustconf-2026-speakers-announced-registration-open/)
    * Tomáš, one of our PMs, is among the speakers, [sharing the progress on the Rust in CPython initiative](https://rustconf2026.sched.com/event/2KHsg).
* [Rust Foundation Interop Initiative Update: From Research to Implementation](https://rustfoundation.org/media/rust-foundation-interop-initiative-update-from-research-to-implementation/)
* [Rust Foundation and Package Registry Leaders Unite to Address Open Source Sustainability Crisis](https://rustfoundation.org/media/rust-foundation-and-package-registry-leaders-unite-to-address-open-source-sustainability-crisis/)

### Rust Project updates

* [Raising the baseline for the `nvptx64-nvidia-cuda` target](https://blog.rust-lang.org/2026/05/01/nvptx-baseline-update/)
* [Announcing Google Summer of Code 2026 selected projects](https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/)
* [Announcing Rust 1.95.0](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/)
    * [Check out the accompanying Release Changelog video produced by the Content Team](https://youtu.be/NZlmaIgkUQ8?si=EN4q-DALc9ZYna-p) ([zulip](https://rust-lang.zulipchat.com/#narrow/channel/122651-general/topic/Rust.20Release.20Changelog.20-.201.2E95.2E0.20-.20Video.20by.20Content.20Team/with/586298114)).
* [docs.rs: building fewer targets by default](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)
* [Changes to WebAssembly targets and handling undefined symbols](https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/)
    * Warning, this will introduce a small breaking change to WebAssembly targets
* [crates.io: Help test our new web frontend](https://blog.rust-lang.org/inside-rust/2026/04/17/crates-io-svelte-public-testing/)
    * crates.io has its frontend rebuild using Svelte 5. You can test it here: https://crates.io/svelte/
* [Infrastructure Team 2026 Q1 Recap and Q2 Plan](https://blog.rust-lang.org/inside-rust/2026/04/14/infrastructure-team-q1-recap-and-q2-plan/)


## Stats

Total words of meeting minutes written: 551.3k (June 2025–April 2026)

Meetings attended: 45

Total words of meeting minutes written (April): 80.8k

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 3.1k
* Libs-API: 4.7k
* Leadership Council: 2.4k
