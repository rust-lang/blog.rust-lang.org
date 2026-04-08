+++
path = "inside-rust/9999/12/31/program-management-update-2026-03"
title = "Program management update — March 2026"
authors = ["Tomas Sedovic and Nurzhan Saken"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++

Greetings and happy spring to you if you live in a place that's getting spring now!

## Team meetings

After his onboarding, Nurzhan is able (and comfortable) to attend team meetings alone. That lets us split the load evenly while keeping the same PM coverage. We can cover for absences, but overall, our personal meeting strain has dropped by half.

This is great! As wonderful as it is to be in the room where things are being decided, help people out, and record the discussions for posterity, it is draining work — especially on days with 6–7 hours of back-to-back meetings.

We're still experimenting with the schedule, but we've made a conscious choice not to specialize. We both participate in each team's discussion and get to know people, and have them get to know us. That makes it easier to share our knowledge and context, as well as being able to step in when the other one can't make it.

We're both, of course, talking regularly, reading notes from the meetings we didn't attend, and if there's anything that needs to be tracked or handed over from one meeting to the next, we do that.

This frees us up to do more of all the other work!

On the topic of meetings, two changes happened. First, [Josh Triplett proposed to temporarily pause the Style team meetings](https://rust-lang.zulipchat.com/#narrow/channel/346005-t-style/topic/Meeting.202026-03-31/near/582686622). For the last few months (and sporadically for much longer than that), only two Style team members were participating.

The team needs more members (as does rustfmt) and ideally a rustfmt liaison in the meeting. We are looking for options, but for the time being, urgent issues can be handled asynchronously.

The other bit of news is that the two Libs-API meetings have merged into one. We used to have one session for items (issues, PRs, etc.) nominated to the team, and a separate one to go through the open [API Change Proposals (ACPs)][acp]. These both happened on Tuesdays with an hour in between.

But the boundary between them wasn't particularly strong (if we cleared the nominated backlog, we started looking at ACPs in the first meeting), and the people generally wanted to just continue.

[And now they can!](https://github.com/rust-lang/calendar/pull/114) The two meetings merged, which lets everyone keep going and finish an hour earlier as well. Or more if the discussion is particularly draining.

[acp]: https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html

## Program management board

We have now populated the [Rust program management board](https://github.com/orgs/rust-lang/projects/69/views/2) so you can see what we're up to.

This lets us track and coordinate work better, as well as making it more transparent.

## Project goals

The [2026 Project goals RFC](https://github.com/rust-lang/rfcs/pull/3935) is now open! This is the culmination of the work the Goals team has done in the last few months.

You can take a look, and if you have any questions, drop by the [project-goals/2026-workshop](https://rust-lang.zulipchat.com/#narrow/channel/546987-project-goals.2F2026-workshop) channel.

And if you are a team lead, please review the RFC and either raise a concern or tick your box!

We're also getting to the end of the 2025 H2 goal period. We have one final update in the works (that we'd like to publish once the 2026 RFC is merged), and then it's full speed ahead for 2026.

A good number of the 2025 goals will continue into the 2026 period, and for these, not much will change. We'll reuse the tracking issues, Zulip channels, and everything else.

The new goals will have a [tracking issue created](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue). Any goal updates should go there, and they'll be automatically posted to the [corresponding Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals), where we can have a discussion.

Twice a month, we're also sending reminders to post an update (if there is a recent update already, the reminder will not be sent).

## FLS Release Notes

The [FLS](https://rust-lang.github.io/fls/) is a specification of Rust for use in safety-critical environments such as cars, originally developed by [Ferrous Systems](https://ferrous-systems.com/). It was donated to the Rust Project and is now maintained by the [FLS team](https://github.com/rust-lang/fls-team).

As part of their [Stabilize FLS releases](https://rust-lang.github.io/rust-project-goals/2026/stabilize-fls-releases.html) goal, they plan to release a new version of the FLS six weeks after any given release of Rust is shipped.

Until now, they've been playing catch-up by waiting for a release and then going through the release notes, looking for any changes that need to be reflected in the document (specifically, language changes). They would like to begin earlier, but they need a list of issues getting into a given release before the notes are all typed up.

So Pete LeVasseur [reached out to the Release team](https://rust-lang.zulipchat.com/#narrow/channel/241545-t-release/topic/meeting.202026-03-27/with/582221788) and [agreed to try participating in the initial triage process](https://rust-lang.zulipchat.com/#narrow/channel/241545-t-release/topic/t-fls.20would.20like.20to.20help.20you.20.28and.20help.20ourselves.29.20with.20notes/with/583905135) for release issues. That way, they'll understand the process better, help out, and get the issue list they need early!

This is a good reminder that the [process is completely public](https://forge.rust-lang.org/release/release-notes.html). More contributors are always welcome, and this can also serve as an accessible entry point into more general contribution.

## Mirroring crates.io and Rust releases

Early in March, we opened the [Implement Verifiable Mirroring Prototype][mirroring-goal] project goal. This is a continuation of the previous signing explorations but with a narrower focus. The plan is to provide a mirror for Rust crates and releases to help with high-traffic environments such as GitHub Actions.

The prototype will let us verify the proposed approaches (such as using [The Update Framework (TUF)][tuf] to validate mirror updates) and key rotation while providing functionality that should help reduce bandwidth usage and cost (with GitHub Actions accessing mirrors hosted on Azure).

This functionality will be implemented behind unstable feature flags, but if it proves out, we can use it to build more general mirroring support.

Until the tracking issue for this goal is created, updates will be posted to the [tbd-signing][tbd-signing] channel in Zulip.

[tbd-signing]: https://rust-lang.zulipchat.com/#narrow/channel/417663-tbd-signing
[tuf]: https://theupdateframework.io/
[mirroring-goal]: https://rust-lang.github.io/rust-project-goals/2026/mirroring.html

## Rust Foundation Maintainers fund (RFMF) RFC

In December 2025, we [mentioned the maintainer fund][dec-rfmf] and the formation of the [RFMF Design Committee][rfmf-committee]. They interviewed several open source organizations that have similar programs in place (Python Software Foundation, Scala Center, Django, and Zig Software Foundation) and [proposed an RFC][rfmf-rfc].

The RFC proposes creating a Maintainer in Residence role:

> The Maintainer in Residence program is dedicated to hiring long-term maintainers and funding their maintenance work in full. Maintainers' in Residence time is split between priorities guided by the teams they are supporting and priorities of their own choosing within the Project.

These maintainers are expected to be Project members with good standing in the community, able to do long-term work keeping the Project healthy and evolving.

It also proposes forming a Funding team, which will select the candidates along with the Leadership Council and ensure the program's overall success.

This is complementary to the proposed [Grants program][grants], which provides financial support for twelve months for people already doing valuable work for the Project. They're not being hired or contracted, but we want to recognize their great work and make it easier for them to continue.

[dec-rfmf]: https://blog.rust-lang.org/inside-rust/2025/12/19/program-management-update--end-of-2025/#rust-foundation-maintainer-fund
[rfmf-committee]: https://github.com/rust-lang/team/blob/main/teams/rfmf-design-committee.toml
[rfmf-rfc]: https://github.com/rust-lang/rfcs/pull/3931
[grants]: https://github.com/rust-lang/rfcs/pull/3919

## Outreachy internships

The Rust Project is participating in [Outreachy][outreachy]! It is a paid open source internship program for people underrepresented in tech.

The Council has [allocated money for three internship slots][outreachy-funding] for the May–August 2026 internship round.

[outreachy]: https://www.outreachy.org/
[outreachy-funding]: https://github.com/rust-lang/leadership-council/issues/264
[outreachy-projects]: https://www.outreachy.org/communities/cfp/the-rust-project/

We have [five projects proposed][outreachy-projects] (adding Rust to an existing C/C++ build system, calling overloaded C++ functions from Rust, code coverage of the Rust compiler at scale, fuzzing the a-mir-formality type system implementation, and improving the security of our GitHub Actions). Right now, we are in the contribution period. This is where the applicants make contributions to the Project, and these contributions will help select the interns.

Personally, I (Tomas) am extremely happy about this. I love what Outreachy does and stands for. I've known and worked with several people who have participated in the program, and they are all fantastic people and colleagues.

Huge thanks to Jack Huey for organizing it! And thanks to Niko, Rémy, tiif, teor, Joel Marcey, and Ethan Smith for signing up as mentors and co-mentors!

## Rust for Linux

Boxy is leading [const generics](https://rust-lang.github.io/rust-project-goals/2026/const-generics.html) and wanted to know which areas would be most relevant to the Rust for Linux team. There's a lot of functionality under the const generics umbrella, and so it's important to know what to prioritize.

This is what the team brought up:

1. **Ability to do arithmetic on const generic types**: e.g., the kernel has a type [`Bounded`](https://rust.docs.kernel.org/next/kernel/num/bounded/struct.Bounded.html) that has a value and a maximum size (in bits). Both the bit width and value are const values. They want to be able to do arithmetic on these types (starting with bit shifts), which will guarantee that the result fits within the specified size at compile time.

2. **Argument-position const generics**: right now, the const generic types must be specified in the type bound section (within the angle brackets). So, for example, you have to write: `Bounded::<u8, 4>::new::<7>()` instead of the more natural `Bounded::<u8, 4>::new(7)`. This gets more complicated when there's const-time calculation happening rather than just a numerical constant — in which case this also needs to be wrapped in curly brackets: `{ ... }`.

3. **Being generic over types other than numbers**: pointers would be useful for [asm_const_ptr](https://rust-lang.github.io/rfcs/3848-asm-const-ptr.html). String literals would be useful, too — even if they're just passed through without being processed or operated on. And if going from a passthrough string makes it possible to pass through any type, that would help the team replace some typestate patterns they're using with an `enum`.

In other updates, Gary added an [infrastructure to provide pointer projection via a macro][macro-projection].

[macro-projection]: https://lore.kernel.org/rust-for-linux/20260302164239.284084-1-gary@kernel.org/

The `dma_read!`/`dma_write!` macros switched over to it. Note that this is done entirely via macros and doesn't use any Field projections language features. The [Field projection][field-projection] syntax and traits should make this more ergonomic and integrate the borrow checker so we can accept more code.

[field-projection]: https://rust-lang.github.io/rust-project-goals/2026/field-projections.html

On that note, the lang team held a design meeting to discuss Field Projections via Virtual Places, the latest proposal from Benno Lossin, Nadrieril, and Tyler Mandry. You can read the [proposal and meeting notes here][field-projection-design].

Finally, as a follow-up on the rustfmt `use` discussion, Tomas opened [an issue](https://github.com/rust-lang/rustfmt/issues/6829) for keeping imports on individual lines the way Rust for Linux needs without having to use the trailing `//` workaround.

[field-projection-design]: https://rust-lang.github.io/rust-project-goals/2026/field-projections.html

## Rust for CPython

We had one meeting with the CPython folks in March.

### Drop with context

Python objects are reference-counted pointers. At any given time, they're either attached to the Python interpreter or not. When attached, their reference count can increase or decrease, but they must be detached to make any native calls. These pointers can't be `Clone` or `Drop`, because the trait implementations don't know whether they're attached to the interpreter state.

There's an open question on how to model this in Rust so that it's both safe and ergonomic.

David Hewitt asked about the "Drop with context" feature, which would require that an associated context (e.g., a pointer to the Python interpreter state) is implicitly passed along to the `drop` function.

This is something the Project has been thinking about (see [Tyler Mandry's 2021 post](https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/), the recent [What If Traits Carried Values post by Nadrieril](https://nadrieril.github.io/blog/2026/03/22/what-if-traits-carried-values.html), and the related [2026 Dictionary-passing style goal proposal](https://rust-lang.github.io/rust-project-goals/2026/dictionary-passing-style-experiment.html)). David, Josh Triplett, and Jack Huey plan to keep discussing this on Zulip as well as at the 2026 All Hands.

### Sanitizers

CPython runs sanitizers (AddressSanitizer or ASan, MemorySanitizer or MemSan, UndefinedBehaviorSanitizer or UBSan, and ThreadSanitizer or TSan) in CI to provide additional checks of the C code. These are important for Python's memory safety.

Emma Smith wanted to confirm whether LLVM and GCC-based sanitizers can be mixed — and they can't. When using Rust, the C portions of the Python interpreter will have to be built using Clang, and those will need to use its own sanitizers. And we need to make sure that these cover the same ground.

The team also asked about compatibility between LLVM releases given that Rust uses its own LLVM fork. The answer is that while the major versions of LLVM are incompatible, Rust's fork and the corresponding upstream version should be compatible. Rust carries backported upstream bugfixes and supports building with stock LLVM.

There is also ongoing work to [stabilize MemorySanitizer and ThreadSanitizer](https://rust-lang.github.io/rust-project-goals/2026/stabilization-of-sanitizer-support.html) driven by Rust for Linux.

### Future meetings

We haven't had any new topics after the beginning of the month, and so we put the meetings on hold for now. When new topics come up, we will talk again, but at this point, there's just a lot of work to be done. The Rust and CPython folks do plan to hold a joint session at the 2026 All Hands.

In the meantime, Emma Smith [posted a progress update from their perspective](https://blog.python.org/2026/04/rust-for-cpython-2026-04/). The team has finished the build system work, and it's passing the CPython CI. In the coming months, they'll plan the internal Rust API design, and after that, write the PEP (Python Enhancement Proposal — similar to our RFCs) targeting Python 3.16.

## Worth a look

### Rust Foundation posts

* [Canonical Joins the Rust Foundation as a Gold Member](https://rustfoundation.org/media/canonical-joins-the-rust-foundation-as-a-gold-member/)
* [What’s Next for the Rust Innovation Lab?](https://rustfoundation.org/media/whats-next-for-the-rust-innovation-lab/)

### Rust Project updates

* [Announcing Rust 1.94.0](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0/) and [1.94.1](https://blog.rust-lang.org/2026/03/26/1.94.1-release/)
* [Security advisory for Cargo](https://blog.rust-lang.org/2026/03/21/cve-2026-33056/)
* [Changes to WebAssembly targets and handling undefined symbols](https://blog.rust-lang.org/2026/04/04/changes-to-webassembly-targets-and-handling-undefined-symbols/)
  * Heads-up for a **breaking** change for WebAssembly builds
* [docs.rs: building fewer targets by default](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)
  * Heads-up for a **breaking** change docs.rs will make on 2026-05-01
* [What we heard about Rust's challenges](https://blog.rust-lang.org/2026/03/20/rust-challenges/)
* [Announcing rustup 1.29.0](https://blog.rust-lang.org/2026/03/12/Rustup-1.29.0/)
* [2025 State of Rust Survey Results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/)
* [Call for Testing: Build Dir Layout v2](https://blog.rust-lang.org/2026/03/13/call-for-testing-build-dir-layout-v2/)
    * Since Cargo 1.91 it's possible to separate the location of intermediate build artifacts (build-dir) and final artifacts (target-dir) into separate directories.
    * Now, Cargo is changing the layout of `build-dir`. The target directory's layout is unchanged.
    * They're asking to test this to help uncover things such as build scripts using it to infer target_dir or executable path.
    * Please consider testing this and reporting any issues you encounter.
* [January & February 2026 Project Director Update](https://blog.rust-lang.org/inside-rust/2026/03/25/project-director-update/)
* [Leadership Council update — March 2026](https://blog.rust-lang.org/inside-rust/2026/04/06/leadership-council-update/)
    * Rémy Rakic joined as the compiler team representative
    * Josh Triplett joined as the libs team representative

## Stats

Total words of meeting minutes written: 470.5k (June 2025–March 2026)

Meetings attended: 35

Total words of meeting minutes written (March): 73.8k

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 3.3k
* Libs-API: 4.3k
* Leadership Council: 2.4k
