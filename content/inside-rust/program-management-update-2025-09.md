+++
path = "inside-rust/2025/10/14/program-management-update-2025-09"
title = "Program management update — September 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

# Program management update — September 2025

As you may have noticed, the August PM update had a broader scope than earlier ones. It included a deeper dive into the reflection and variadic generics features and took the opportunity to describe the nature of the Leadership Council and the Foundation Project Director roles.

I think it's useful to share details on what's happening throughout the broader Project in addition to the things I have personally done.

Please [let me know][tomas-zulip] whether this is indeed valuable to you, whether these updates should only focus on the direct PM work, or whether there's anything else you'd like.

[tomas-zulip]: https://rust-lang.zulipchat.com/#user/893815

## Leadership Council

The Council representatives have been selected. We have one new representative: [Jakub Beránek](https://github.com/Kobzol/). Jakub will represent the Infrastructure team.

<https://blog.rust-lang.org/inside-rust/2025/09/23/leadership-council-repr-selection/>

## Project Directors

The Foundation Project Director selection process has completed.

I gathered all the nominations, verified that the nominees had accepted their nominations, and asked everyone to provide a statement for their candidacy.

Those statements are listed in the [Project Director Candidate Nominations 2025 hackmd][pd-hackmd].

[pd-hackmd]: https://hackmd.io/Q6PxtJhmQVOgN3RjTT3xqA

The elections were held on Friday, October 3rd, 2025, during which the Leadership Council selected the three new Project Directors in accordance with the [election process][election-process].

[election-process]: https://github.com/rust-lang/leadership-council/blob/main/policies/project-directorship/election-process.md

## C++ Interop at RustConf

I did not travel to RustConf and haven't participated live much, due to the sizable (9-hour) time difference.

I did, however, remotely join the C++ interoperability session that Jon Bauman organized on 2025-09-02 and took notes while I was awake.

There was an overview of the current tooling (bindgen, CXX, Zngur, Crubit); attendees talked about how they're handling interop in their organizations and discussed the next steps to move the work forward.

The [notes are here][cpp-interop] and I'm writing a blog post to summarize what happened.

[cpp-interop]: https://hackmd.io/Ngoc6POlT4CywmocKh4MzQ

## Kangrejos 2025

I attended [Kangrejos, the annual Rust for Linux workshop in Spain, Oviedo][kangrejos]. Rust for Linux is a project that aims at making Rust the second official language in the Linux kernel (C is the first one).

This was an absolutely packed two-day event. The roughly thirty attendees were all sitting in the same room, had lunch in the same space, and went to dinner together.

Miguel Ojeda told me this is by design &mdash; that way there's a lot of opportunity for everyone to get to know everyone else even without lengthy and potentially awkward introduction sessions. Everyone learns about the various efforts going on, people can talk, share information, and collaborate.

It was a really intense experience, but it worked as advertised. I ended up speaking with a good number of the attendees, learned a lot about what's going on, and got to know the Rust for Linux team I'm working with.

The attendees were a pretty diverse bunch. The majority were people working on various aspects of Rust for Linux, but we also had representatives from the Linux kernel (Greg KH who's directly supporting the effort), LWN.net, Debian, and the Rust Project (Tyler Mandry and yours truly).

The event consisted of a series of talks, presentations, and workshops &mdash; some to provide a status update, others to showcase a new tool or process. The event was intentionally not recorded or minuted so that everyone could speak freely and openly.

To get a sense, you can see the [topics and some of the slides on the Kangrejos 2025 page][kangrejos-talks].

[kangrejos]: https://kangrejos.com/
[kangrejos-talks]: https://kangrejos.com/2025

Here are some things I personally found interesting:

**Rust is already used in the mainline kernel:**

* [QR code panic generation](https://rust-for-linux.com/drm-panic-qr-code-generator)
* [Nova (driver for NVIDIA GPUs)](https://rust-for-linux.com/nova-gpu-driver)
* [Tyr (driver for Arm Mali GPUs)](https://rust-for-linux.com/tyr-gpu-driver)

... and more. You can see the full list at the [Rust for Linux](https://rust-for-linux.com/) page under the "Users — in mainline" section.

**Rust for Linux needs are beneficial to the evolution of the Rust language**

Rust aims to be a low-level language that you can (among other things) write kernel code in. This is exactly what Rust for Linux is doing &mdash; in a large, well-established kernel at that.

Any areas for improvement that RfL encounters will likely also benefit other low-level projects (kernels, embedded, filesystems, etc.).

**[Coccinelle for Rust](https://rust-for-linux.com/coccinelle-for-rust)**

This is a tool that lets you describe transformations you want to make and then apply them across the code base.

It supports relatively straightforward things such as renaming a function or reordering parameters. But it operates on AST level and lets you do things like turning:

```rust
info!("Window resized to: {}x{}", width, height);
```

to:

```rust
info!("Window resized to: {width}x{height}");
```

across the codebase. Across all such `format!`-like invocations, parameters etc. And taking care to not interpolate field accesses or function calls (which are currently not allowed in the "format strings").

Tathagata Roy ran the workshop for us. We tested Coccinelle out on some pre-existing examples, including the one above. The evening after, I ran it on all my repos. I'm really excited about this.

**Clippy**

There was a really good status update on Clippy work specifically for Rust for Linux by Alejandra González. I had no idea that was happening, but it makes perfect sense. Clippy can highlight code that's not desirable even if it's allowed by the Rust compiler.

Rust for Linux relies on these lints and their CI runs Clippy as well.

Last year, Alejandra's work made Clippy 40-60% faster.

For next steps, she talked about getting first class support for Rust for Linux up to using Clippy's CI to check the Rust for Linux codebase to highlight any potential breakage Clippy could cause.

Alejandra also talked about making lint configuration stable. Some lints can [have their behavior changed via `clippy.toml`][clippy-toml]:

[clippy-toml]: https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#configure-the-behavior-of-some-lints

This is currently unstable, but the Clippy team is working on a stabilization RFC.

Miguel Ojeda stressed that the most important thing for Rust for Linux usage is to not show any false positives. When they enable a lint, they need to make sure that it only finds legitimate issues.

**[GCCRS: GCC Front-End For Rust](https://github.com/rust-gcc/gccrs)**

Pierre-Emmanuel Patry talked about an alternative implementation of the Rust compiler within the GNU compiler toolchain.

This effort is valuable because it provides a completely separate compiler (which could for example compile rustc at some point and therefore bootstrap Rust). It can help specification efforts by highlighting areas where the compilers interpret things differently, which can then force a decision and have both compilers align on it, and provide Rust for people who either require the GNU toolchain or who would benefit from the platforms that are supported by GNU but not LLVM.

The "front-end" here means that GCCRS reads Rust code and transforms it into an [intermediate representation (IR)][ir] that GCC tooling turns into binary code, similar to how the Rust compiler outputs LLVM IR.

[ir]: https://en.wikipedia.org/wiki/Intermediate_representation

Pierre-Emmanuel highlighted the difficulties of compiling even a "simple" `for` loop: to do that you need to be able to resolve traits and handle iterators and macros.

Their main target is to compile Rust's [core library][core].

[core]: https://doc.rust-lang.org/core/

After that, they've set their sights on compiling the Linux kernel (including the Rust part). Linux can currently be built with either GCC or LLVM, but it is strongly recommended that everything use one or the other. This will be beneficial to some projects that are required to use the GCC toolchain.

Interestingly, the main effort is not going into the borrow-checker at all. They're using [Polonius][polonius] right now and the compiler is only handling *correct* Rust programs. In the future they're looking to integrate the next-gen borrow checker, but that's some ways off (and would require it being moved to its own crate).

[polonius]: https://github.com/rust-lang/polonius


## T-Content

Several members of the [Content team][t-content] attended RustConf and recorded several interviews there. The first of these, an interview with [Jan David Nose][jdno] from the crates.io team, is up:

[t-content]: https://rust-lang.org/governance/teams/launching-pad/#team-content
[jdno]: https://github.com/jdno

<https://www.youtube.com/watch?v=r7i-2wHtNjw>

Two weeks later, Tyler Mandry and I worked with Miguel Ojeda (the Rust for Linux lead and Kangrejos organizer) to find time and space to interview people at Kangrejos.

We've managed to record a few and after we edit them, we'll start publishing them on the [Rust YouTube channel][rustvideos].

[rustvideos]: https://www.youtube.com/@RustVideos

This has been a fascinating experience. I've been (remotely) interviewed once, but I've never seen the behind the scenes. Mostly, I've helped Tyler get things set up and kept an eye on all the microphones and cameras to make sure everything was recording (one or our cameras had a silent 30 minute recording limit).

But I did take the opportunity to step in front of the camera and interview one of the attendees. I was worried I was going to mess up and ruin the interview, but watching the raw footage, it seems to have turned out fine.

I thought my role in the Content team would be mainly support and possibly in writing/publishing. But now I'm open to being part of the recording process as well. Especially if we'll have more time to prepare ahead of time and won't have to squeeze it between lunch and conference sessions :-).

## 2025H2 Goals

Niko Matsakis opened the [RFC for Project goals for the second half of this year](https://github.com/rust-lang/rfcs/pull/3849).

Rémy (lqd) and I reviewed the RFC and followed up on the checklists &mdash; making sure that every required person checked their box (after helping to resolve their concerns).

To make things easier, Niko plans to provide a page that always shows the current status of all goals. The idea is that each team would be able to review it periodically and see if e.g. there's something they can do to get a goal unstuck.

I will continue to write the regular updates and this should help with that work too. And I plan to set up a retrospective and see if there's anything people would like to change.

Niko introduced me to Nandini who's a post-doc at Carnegie Mellon, researching how open source communities organize themselves. She started to conduct in-depth interviews with people from the Project about Goals specifically. That work is ongoing, but once it is complete, it will provide really valuable data.


## Variadic generics micro survey

Finally, a follow-up on variadic generics.

Olivier Faure [announced a survey][vargen-blog] to gather information from the Rust ecosystem on how people work around the lack of variadic generics in Rust, which features are important to them and what they would use it for.

[vargen-blog]: https://blog.rust-lang.org/inside-rust/2025/09/22/variadic-generics-micro-survey/

Since the space of possibilities is vast, this will help us prioritize what the community needs most.

If this is of interest to you &mdash; and especially if this is something you already had to work around &mdash; please fill out the survey:

<https://www.surveyhero.com/c/rust-variadic-generics-survey>

The survey will be open until Monday, October 20th, 2025.


## Stats

*Lighter than usual, because most regular meetings were canceled during RustConf and I've missed all the meetings during the Kangrejos week as well.*

Total words of meeting minutes written: 169.9k (June - September).

Meetings attended: 23

Total words of meeting minutes written (September): 31.3k

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 2.8k
* Libs-API: 4k
* Leadership council: 2.8k
