+++
path = "inside-rust/9999/12/31/program-management-update--end-of-2025"
title = "Program management update — End of 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

This is a combined update for November and December. I had less time to work in November, and I'll be off for the winter holidays, so combining them makes sense.

## Project goals

A lot has happened here! Niko and I kept talking about how we want to see the Goals moving forward, and we held four *office hours* sessions to talk it through with people interested in the topic.

### Annual cadence

In the end, we settled on annual goals for Flagship and other large initiatives, but with a twist: if you're a Project member and want to propose a goal, and have everything (team vibes, review capacity, etc.) sorted, you can start a new goal anytime you'd like.

We want to make proposing a goal that you plan to implement within your team as simple as possible.

There are several reasons we're moving flagship initiatives, external goals, and large endeavors to an annual cadence:

First, the current six-month cadence has the wrap-up and call for new goals in months when a lot of people take holidays (December-January and July-August), including the people running the program, writing goal RFCs, and team leads who need to discuss the proposed goals and provide checkboxes.

Second, a lot of goals end up continuing into the next period, so this could reduce the administrative burden.

Third, the current schedule doesn't align well with fiscal cycles. This can complicate goals that are funded either by companies employing developers or by those providing external funding for goals they want to see.

And finally, we want to align goals closer with the Editions (which we also want to make easier to deliver, possibly sooner than every three years).


### rust-project-goals

To support this, Niko made changes to the [`rust-project-goals` repo](https://github.com/rust-lang/rust-project-goals) which I reviewed and followed up with a small fix.

After that, I addressed some of the issues with how we were generating reports and blog posts:

* Added dark mode support.
* Fixed rendering of the Field Projection syntax (things like `foo.@bar.@baz` were interpreting `@bar` etc. as GitHub usernames, resulting in `foo.[Ber Clausen][].[Baz Shkara][]`, which completely lost the original meaning).
* The code blocks were also being mangled and were hard to read.
* The "Goals looking for help" and "Other goal updates" headers were shown on the same line if there were no goals looking for help.
* Updated the [Propose a new goal](https://rust-lang.github.io/rust-project-goals/how_to/propose_a_goal.html) page to reflect the 2026 period.

I also published the [November goal update](https://blog.rust-lang.org/2025/12/16/Project-Goals-2025-November-Update.md/) and fixed the issues above for the September and October posts.

We've received plenty of suggestions on the current way the goals are formatted and posts published:

* The "goal summary" boxes take up a lot of space, duplicate some information, and are less relevant to the post.
* Having an index of all the goals would be nice.
* Skip/hide/deprioritize goals that don't have any updates listed in a given month.
* Fix the header formatting in the updates -- the `<h2>` elements in particular are visually massive, especially when viewed on phones.
* Provide a view that lists all the goals a champion has.
* Convert the GitHub IDs to Zulip IDs for the post updates we're sending to Zulip.
* Replace the "progress bar" with something that more clearly outlines the current status (e.g., "on track," "delayed," etc.).

I'll be looking at those as well.

We've also been grappling with whether to keep doing monthly updates at all, especially now that goals will be annual. We definitely don't want to put a lot of noise on the Rust blog, and maybe having less granular updates (e.g., every three months or so) would work better.

Personally, I've gotten a lot of benefit from these, even before joining the Project. They provided a nice snapshot of what was going on, where the Project was going, and updates on the few I particularly cared about (e.g., cargo-script).

So if we can get these into a shape where they're good for that purpose (with the benefit that they just land in your RSS feed and you don't have to subscribe to a ton of tracking issues on GitHub), I'd love to do that.

If we can't get there, we'll look at changing the format and/or posting them less frequently (maybe once a quarter).



### Timeline / next steps

We're accepting suggestions for new goals until the end of January 2026. After that, we'll publish the first draft of what the 2026 goals look like, and we'll keep refining them throughout February.

We plan to publish the RFC in March and start the 2026 goal season officially in April.

We will do quarterly check-ins and have the goals run until the end of the year when we put out proposals for the next ones.

You can [learn more about the timeline in Niko's post](https://blog.rust-lang.org/inside-rust/2025/12/04/want-to-propose-a-2026-rust-project-goal-we-are-here-to-help/).

### Propose a 2026 goal right now!

The time to propose a goal is now, especially if it's something that would require nontrivial capacity for reviews, design feedback, etc., from the teams.

If you have something in mind, look at the [Propose a new goal](https://rust-lang.github.io/rust-project-goals/how_to/propose_a_goal.html) page, and we'll work with you on getting it ready.

If you'd like to propose a goal but aren't sure about something, are struggling to fill out the template, or aren't even sure whether having a goal makes sense, please reach out to us!

Start a topic on the [project-goals/2026-workshop Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/546987-project-goals.2F2026-workshop), and we'll talk it through.

I've published all this in the [Please submit 2026 Project goal proposals](https://blog.rust-lang.org/inside-rust/2025/12/16/please-submit-2026-project-goal-proposals/) post, and Niko and I have been talking to people about potential goals they may want to open.

We've been clarifying whether something would be a good Project goal, what the process is, reaching out to team leads about their ideas for flagship goals/themes, and helping out with the proposals people posted on Zulip.


## Rust for Linux

Recently, Linux held its annual [Linux Kernel Maintainer Summit](https://events.linuxfoundation.org/linux-kernel-maintainer-summit/). One of the topics discussed there was the state of Rust in the Linux kernel. After five years and seeing the progress and impact made, [Rust in Linux is no longer treated as an experiment](https://lwn.net/SubscriberLink/1050174/63aa7da43214c3ce/), and the `experimental` tag will be removed \o/.

The two main concerns that remain are having all the necessary features in *stable* Rust (something we're steadily working toward) and platform support.

While Rust now supports all the main platforms for the kernel, there is a long tail that's left out. As such, Rust is not part of the Linux core yet, and one way we hope to address this is with the help of GCC.

There are two projects in this space:

1. [rustc_codegen_gcc](https://github.com/rust-lang/rustc_codegen_gcc) is a *codegen* backend: it takes the final abstract syntax tree Rust produces and then passes it to this backend, which will use GCC to produce the final binary/library file.
1. [gccrs](https://github.com/Rust-GCC/gccrs) is a full alternative Rust compiler for GCC, using its toolchain (analogous to how Rust's compiler uses LLVM).

In addition, the Rust for Linux people keep getting things stabilized and pushing for missing features:

* There's slow but steady progress on stabilizing the compiler flags Rust for Linux needs.
* In-place Initialization design keeps being worked on. There are two main paths we could take: the impl Init + init expression one that Alice Ryhl proposed, and the out-pointers one that Taylor Cramer brought in.
    * The team's trying to push both and see how far they can go because there are a lot of edge cases that this feature needs to cover.
    * On top of that, Yosh Wuyts was looking at a more higher-level notation to use for placing functions—something that would be easier to write and could desugar to either proposal.
    * Ding planned to request feedback on the end-user syntax at the conference; we're waiting to see how that went.
* There is still big interest in guaranteeing value placement in the compiler, and we're excited to see how Olivier refines it.
* Similarly, the Field projections discussion is very much ongoing with new proposals coming up. Benno and Nadri are pushing it forward, but there's a huge space to map out.

All of these endeavors have a lot to sort out design-wise, with the likelihood of multiple Lang experiments and RFCs coming out. We're planning for all these goals to continue into 2026.

Finally, Alice Ryhl has been invited to the [Language Advisors team](https://rust-lang.org/governance/teams/lang/#team-lang-advisors) -- this is a group of people that the Lang team values and reaches out to for opinions. Congratulations!


## CPython

A month ago, Emma Smith and Kirill Podoprigora, Python core developers, opened a pre-PEP (think pre-RFC) for introducing Rust into CPython:

<https://discuss.python.org/t/pre-pep-rust-for-cpython/104906/141>

CPython is the canonical interpreter for Python, and as the name suggests, it is written in C. Emma and Kirill propose adding Rust as an additional language the project could use.

The main motivations are memory safety, fearless refactoring, and ergonomics. Given Rust's popularity -- and especially its ability to let people dive into lower-level topics without worrying about C's footguns -- there's hope it might bring in more contributors as well.

This would allow Python to start using Rust for its standard library functionality, making it safer and more contributor-friendly. The idea of adding it to the core language was also floated but would come up later.

A few Rust Project members, including myself, talked to them, mainly to learn what they need, what the concerns might be, where we could help, and to see if we could set up a collaboration similar to what we have for Rust for Linux.

This has been productive, and I expect we'll be talking and sharing more.

A lot of their needs, as well as motivations, are similar to Rust for Linux: `build-std`, platform support, sanitizer support, and arbitrary self types.

Their first priority is taking to heart all the feedback from the proposal, identifying all the features they need from Rust (and with our help, hopefully addressing at least some of those with our current functionality), and listing the platforms they're looking to support.



## write_fmt infrastructure meeting with **Types**

As a follow-up to the [`fmt::Write` infrastructure](https://blog.rust-lang.org/inside-rust/2025/11/19/program-management-update--october-2025/#fmt-write-infrastructure) effort, I set up a meeting with Josh Triplett, Amanieu d'Antras, Tyler Mandry, and the Types team.

They were interested in the status of the "impl subtrait via supertrait" feature. This would be really helpful in providing a smooth transition from the old `Write` trait to the new one.

This was more of a vibe check where the Types team asked about their requirements, provided some suggestions, and suggested the Libs and Lang folks put together a detailed document of the design they're looking for and set up a deep dive on that.

Here are the meeting notes I took: <https://hackmd.io/pwSkzuYkStaQK59QfUwaIA>

## Project priorities Council meeting

There was an off-cycle Leadership Council meeting to discuss Project Priorities funding with the Foundation.

Some background: in December 2023, [Microsoft donated one million US dollars of unrestricted (i.e., it can be used for any purpose) funds to the Rust Foundation][ms1m]. A third of it was used to hire an Infrastructure engineer, and the rest went to the Project to use for things they see as important -- the Project priorities fund.

[ms1m]: https://rustfoundation.org/media/1m-microsoft-donation-to-fund-key-rust-foundation-project-priorities/

This was used for the travel budget, the compiler-ops position, and Program Management (hello!).

The Project saw this as a success, and they asked the Foundation to provide these funds on an ongoing basis. During the meeting, the Council and Project directors spoke with Abi Broom (Director of Operations) and Dr. Rebecca Rumbul (the Foundation's Executive Director and CEO), who shared how much money they'll be able to provide. Combined with the remainder of the Microsoft budget, the Foundation plans to give the Project a total of $650k USD for 2026.

This will be used to support:

* Travel budget
* compiler-ops
* Program Management

And leave a sizable chunk for other things the Project wants to do, for example, experiments and maintainer support.

You can [read the minutes in the leadership-council repo](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-11-14.md).


## Worth a look

### Call to Action: Who'd like to edit interviews with Rust folks?

The Content team now has a backlog of video interviews that need to be edited and published. This is a great opportunity to contribute if you're so inclined!

If you have a background in video editing or you're willing to learn, please drop by the [t-content Zulip channel][content-video]!

[content-video]: https://rust-lang.zulipchat.com/#narrow/channel/523012-t-content/topic/.5B.F0.9F.97.A3.EF.B8.8F.20Call.20to.20Action.5D.20Who'd.20like.20to.20edit.20interview.20content.3F/with/562358088

We track all our videos on the [Interview Inventory board](https://github.com/orgs/rust-lang/projects/65/views/1). Just look for `Recorded (not edited)`.


### Directly sponsoring Rust contributors

Many Rust contributors have individual sponsorships set up. Jakub Beránek (Kobzol) set up the [Funding](https://rust-lang.org/funding/) page that lists people you can sponsor directly.

You can read more in the blog post [Making it easier to sponsor Rust contributors](https://blog.rust-lang.org/2025/12/08/making-it-easier-to-sponsor-rust-contributors/).


### Rust Foundation Maintainer Fund

My [previous update](https://blog.rust-lang.org/inside-rust/2025/11/19/program-management-update--october-2025/#funding-rust-maintainers) mentioned the Foundation setting up a maintainer fund. As this is supposed to provide direct value to the Project, the Leadership Council has been tasked with figuring out how to use this money.

The [Council has formed a Rust Foundation Maintainer Fund Design committee][rfmf-committee] to answer and document that question.

[rfmf-committee]: https://github.com/rust-lang/leadership-council/blob/main/committees/maintainer-fund-design-group.md

They will talk to a wide variety of people (maintainers, leads, teams, the LC, the Foundation) to gather information and write an RFC that will describe how the fund will work.

You can follow what they're doing in the [#funding channel on Zulip][funding].

[funding]: https://rust-lang.zulipchat.com/#narrow/channel/548261-funding

### Rust Foundation posts

* [Fall 2025 Project Director update](https://rustfoundation.org/media/fall-2025-project-director-update/): Carol Nichols summarized what's happened on the PD side.
* [RustConf 2026: Early Details and What’s Coming Next](https://rustfoundation.org/media/rustconf-2026-early-details/): First details about the next RustConf. It will take place in Quebec, Canada, which might be more accessible for international travelers.
* [The RustConf 2026 Call for Proposals Is Open!](https://rustfoundation.org/media/the-rustconf-2026-call-for-proposals-is-open/): Submit your RustConf proposals by 2026-02-16!
* [2025 In Review](https://rustfoundation.org/2025/): The Foundation employees highlight their accomplishments. Among other things, this includes Infra, Security, and C++ interoperability.

### Rust blog posts

* [Announcing Rust 1.92.0](https://blog.rust-lang.org/2025/12/11/Rust-1.92.0/)
* [Lessons learned from the Rust Vision Doc process](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/)
* [Switching to Rust's own mangling scheme on nightly](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)
* [Google Summer of Code 2025 results](https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/)
* [November project goals update](https://blog.rust-lang.org/2025/12/16/Project-Goals-2025-November-Update.md/)


## Stats

Meetings attended: 61

Meeting minutes word count since the last update: 95.3k

Total meeting minutes word count: 265.2k (June-December)

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 6.5k
* Libs-API: 5.2k
* Leadership council: 2.9k
