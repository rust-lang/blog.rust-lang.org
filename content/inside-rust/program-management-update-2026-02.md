+++
path = "inside-rust/9999/12/31/program-management-update-2026-02"
title = "Program management update — February 2026"
authors = ["Tomas Sedovic", "Nurzhan Saken"]

[extra]
team = "the Program team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-program"
+++

# Program management update — February 2026

Greetings! This update is brought to you by Tomas Sedovic *and* Nurzhan Saken \o/.

## Hello from Nurzhan

I discovered the Rust Project in 2022 and quickly became *obsessed*. The language, the documentation, tracking issues, changelogs, blogs, talks, and user/contributor discussions — all of these scratched an itch and comforted me during a difficult period in my life. As a huge introvert, I mostly observed and didn't participate, so the job posting for a program manager felt like a wake-up call. Tomas put it well in his first update: I had to apply despite many doubts about whether I would actually do a good job; the regret of not going for it would have been devastating.

TC and Joel Marcey interviewed me in March–April 2025 and considered hiring me. This was a surprise because I felt like a total mess! Alas, you may know from the previous update that things didn't go as planned, and I didn't end up getting the position. Still, there was no going back, so I started engaging with Rust by stabilizing multiple features I was interested in. Eventually, more funds did get allocated, and I was hired.

The first month felt surreal. Suddenly, the imaginary barrier vanished, and I found myself sharing spaces (and even interacting) with the people whose work I had admired so much. Tomas had been leaving behind some impressive footprints, seemingly impossible to match. It was easy to feel like there'd been some mistake, that I didn't belong. Yet, everyone has been so welcoming. Tomas has been going out of his way to talk to me, support me, and loop me into everything happening in the Project. Thanks to that, I already feel mostly integrated, well-equipped, and really lucky that things have turned out this way.

## Project goals

February was focused on getting through the proposed goals and having teams look at them. We are currently at 0 open pull requests (with huge help from Nurzhan), and we incorporated the sizing and champion feedback from the teams looking at the goals.

Each goal typically has one or more asks for Project teams (e.g., Lang, Compiler, Libs), so it is important that they review all their asks, see whether they're clearly articulated, and determine whether the proposed changes make sense in the Project.

In March, we'll do a final pass over the individual goals to make sure they have the right champions and sizes. After that, we'll open an RFC listing all the proposed goals and get feedback from the teams.

That is an important time because, until now, goals were considered mostly in isolation. But with the RFC, we'll be looking at the full picture as teams consider their overall capacity for the full year.

We aim to have the RFC reviewed in March and officially start the new goal period at the beginning of April.

***

We also discussed the way we do Project goal updates. The current auto-generated blog posts leave a lot to be desired, and we're interested in your feedback!

If you've read them previously, and especially if you haven't, we'd love to hear from you! What's helpful? What isn't? What would you like to see vs. what might be distracting?

We're talking to the Content team about ways to highlight specific goals in individual posts or interviews. We're also looking at making the updates more meaningful. Hearing your perspective here would be valuable! You can tell us on the [Blog post thread on Zulip][goals-feedback] or DM the `T-program` alias.

[goals-feedback]: https://rust-lang.zulipchat.com/#narrow/channel/478266-project-goals.2Fmeta/topic/Blog.20.20post.2C.20tracking.20progress.20and.20presenting.20goals/with/574716353

Here are the recent blog posts we've published:

* [December 2025](https://blog.rust-lang.org/2026/01/05/project-goals-2025-december-update/)
* [November 2025](https://blog.rust-lang.org/2025/12/16/Project-Goals-2025-November-Update.md/)
* [October 2025](https://blog.rust-lang.org/2025/11/19/project-goals-update-october-2025/)


## Program management tracking

Given that there are now two people taking on the program management work full time, we created places dedicated to that.

First off, we have a [backlog/board](https://github.com/orgs/rust-lang/projects/69) where we will track our work. Any issue from any rust-lang repo can be added there.

Since some topics won't fit into any of the existing repositories (things like scheduling the *signing & mirroring* meetings, or writing the monthly PM update come to mind), we also had the [program-team repository](https://github.com/rust-lang/program-team) created.

This can double for any documentation and tooling related to the effort.

Finally, you can now use the `T-program` alias on Zulip to talk to us, tag us on a thread, or make a request.

We're hoping this will make it clearer how to reach out to Program management, provide greater transparency (to show what we're actually working on), and, of course, help us (the PMs) coordinate better.

As of this writing, the repo and board are both empty. There are a few issues in other repos that we can link there, but mainly, Tomas needs to move the contents of his Rust org-mode file over.


## C++/Rust interoperability

The Foundation hired [teor](https://github.com/teor2345) for a short-term contract to speed up mapping out of the interop problem space, and they hit the ground running.

teor read through all the existing documents that we had, created a [template for each issue / problem statement](https://github.com/rustfoundation/interop-initiative/blob/main/problem-space/0000-template.md) (e.g., exceptions & unwinding or incompatible allocators), immediately started [documenting the known ones](https://github.com/rustfoundation/interop-initiative/tree/main/problem-space) and filing issues for the various [use cases](https://github.com/rustfoundation/interop-initiative/issues) that projects integrating these two languages have.

They're posting weekly updates on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop/topic/Interop.20Problem.20Space.20Mapping.20-.20Weekly.20Updates/with/578041637) and monthly ones in the [Project goal tracking issue](https://github.com/rust-lang/rust-project-goals/issues/388).

These updates and the focus on mapping out the technical aspects are a result of feedback provided to the Foundation by the Project members and the wider community (directly, as well as indirectly, e.g., through Project Directors and Program managers).

If this is an area you're interested in, please take a look at the [interop-initiative](https://github.com/rustfoundation/interop-initiative) and the [t-lang/interop Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop).


## Signing and mirroring

Following up from [January](//inside-rust/2026/02/11/program-management-update-2026-01/#crates-io-mirroring-and-verification), Walter Pearce proposed a draft of the Project goal. It focuses on building an MVP that sets up a mirror for Rustup targets. This would happen fully on the backend (and should therefore be completely transparent to any users) to look at bandwidth and logging cost reduction, stand up the security infrastructure, and get hands-on information to build the ultimate solution.

We've added a few more people to the group (e.g., Rustup folks), and the draft is under active discussion. There are still questions to resolve both on the technical and communication sides, but the proposal seems to be in a solid place, and we hope to have a PR open soon.


## Style

For the last several months, a lot of the meetings ended up canceled. At the end of January, I scheduled a new time with a fortnightly cadence, and we've met two times since.

In the first one, we triaged all the nominated issues. Recognizing the capacity issues (the Style team has three members, and lately, one of them has not been able to dedicate a lot of time), TC proposed we focus on getting things unblocked while leaning on the Project members.

So, when an issue comes in, the Style team considers it, figures out where it fits within the existing guidelines, and then asks the submitter for a concrete proposal instead of writing it all down themselves.

This practice is similar to how the Lang team operates, born out of similar capacity issues.

The last two meetings felt much more productive that way, but the capacity issue remains. We need help!

If you're interested in *how* the language is formatted -- providing input on what rustfmt should do -- we would love to have you! You can join the [t-style channel](https://rust-lang.zulipchat.com/#narrow/channel/346005-t-style) or [attend one of our meetings](https://rust-lang.github.io/calendar/style.ics).


## Rust for Linux

We've discussed the [Rust for Linux roadmap](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html) and went over all the other unstable features the [project is tracking](https://github.com/Rust-for-Linux/linux/issues/2) which aren't part of a particular goal. These are to be tracked in the roadmap, and we'll be spinning up goals where that makes sense.

There is an important milestone of sorts for Rust for Linux: the upcoming Debian 14 stable release (codename: Forky). It is a time when the project can upgrade to using a new Rust version and take advantage of any features shipped in the meantime. Debian releases happen roughly every two years, thus Forky is expected to be released around Summer 2027.

Rust for Linux supports a range of stable versions of the compiler, but it also needs some *[unstable features](https://rust-for-linux.com/unstable-features)*, which the team uses when there's reasonable confidence that things won't change much (or disappear!). There's generally a desire to limit churn, so replacing existing code with new features is something that typically happens when a new Debian stable is released.

With that in mind, we looked at features with the most significant impact that Rust for Linux wants to start using as soon as possible (i.e., with the next Debian). Some of the most notable ones are:

* [Arbitrary Self Types]
* [Field Projections]
* [Immovable types and guaranteed destructors](https://rust-lang.github.io/rust-project-goals/2026/move-trait.html)
* [ADT const params](https://rust-lang.github.io/rust-project-goals/2026/const-generics.html#adt-const-params)
* [`rustdoc --output-format=doctest`](https://github.com/rust-lang/rust/issues/134529)
* [Relaxing Rust's orphan rules.](https://github.com/rust-lang/rust/issues/136979) Later this year, the team wants to split the monolithic kernel crate into sub-crates, at which point they will need to deal with the orphan rules. To this end, [there has been a suggestion](https://github.com/rust-lang/rust/pull/150652#issuecomment-3707365609) to introduce *coherence domains*, a way to relax the orphan rules within a set of crates. We hope to flesh this out more through continued discussions.

In general, something that provides new language capabilities — especially when tied in with new syntax — is something that Rust for Linux wants to use soon so they don't have to rewrite a lot of code later on. If the new syntax can be hidden behind a macro, that is acceptable. Anything behind a feature flag is also acceptable so long as there's a reasonable expectation it will be stabilized roughly in that form (e.g., no massive syntax changes) or won't be removed. For example, while Rust for Linux would benefit greatly from [in-place initialization](https://rust-lang.github.io/rust-project-goals/2026/in-place-init.html) as a native language feature, the team has a working solution for it, so it's not as blocking as [field projections], which doesn't exist as an unstable feature yet.

[field projections]: https://rust-lang.github.io/rust-project-goals/2026/field-projections.html
[arbitrary self types]: https://rust-lang.github.io/rust-project-goals/2026/arbitrary-self-types.html

## Rust for CPython

At the beginning of February, we invited Miguel Ojeda (the Rust for Linux lead) to share insights from his team's experience introducing Rust into the Linux kernel.

Miguel went over the rough timeline of Rust for Linux: the effort gained traction in late 2020, leading to an RFC in early 2021. Minimal experimental support was merged into mainline in late 2022, and the experiment concluded in late 2025. Collaboration with the Rust Project began around 2021 and transitioned to regular meetings in early 2024.

When the project started, there was skepticism around introducing Rust into a mature C codebase. C had long been the main language used in the kernel, and there had been resistance to adopting additional languages (e.g., C++ was briefly experimented with in 1993 before being abandoned). There were concerns about the downsides of moving to a two-language codebase and the need for developers to learn a new language and toolchain with benefits that were not yet clear.

The team spent a lot of time discussing these concerns with maintainers and contributors on mailing lists and at conferences, as well as following public discussions and the news to understand how they could communicate better. In 2025, Miguel presented a keynote showcasing what kernel maintainers, team members, stakeholders, and companies had thought about Rust for Linux at [that year's FOSDEM event](https://archive.fosdem.org/2025/schedule/event/fosdem-2025-6507-rust-for-linux/).

In the [RFC Miguel proposed](https://lore.kernel.org/lkml/20210414184604.23473-1-ojeda@kernel.org/), they were honest about the advantages and disadvantages of Rust. A lot of the kernel experts thought that memory safety was the main appeal, but the RFC highlighted benefits beyond that: a clear separation between safe and unsafe code (with the former having no undefined behavior, memory safety violations, or data races), useful language features (rich enums, pattern matching, modules, hygienic macros), and powerful integrated tooling (`rustfmt`, `rustdoc`, lints). The resulting feedback led to the first "real" hardware driver (GPIO) being written in Rust (with a [line-by-line comparison](https://lwn.net/Articles/863459/) with the C implementation).

The team prioritized raising the bar on the quality of Rust code and documentation, e.g., requiring `SAFETY` comments around each `unsafe` block, consistent code formatting, and fully documenting all public APIs. Over time, the Rust experience also benefited the C side. For example, reviewing C code to design Rust abstractions sometimes uncovered issues that had gone unnoticed. More generally, the team has observed Rust concepts starting to percolate into the C side in discussions and designs; a recent example is the discussion around [Revocable for C](https://lwn.net/Articles/1058041/).

All of this strongly resonated with the CPython developers, who saw very similar reactions to their proposal and are facing similar technical challenges. Emma Smith, who leads this effort along with Kirill Podoprigora, mentioned that the Rust for Linux proposal was a huge inspiration for them to start this effort for CPython.

The Rust Project is happy to see cross-pollination in this space, and we'd love to see (and help!) more projects adding Rust to existing code.

Other than that, we kept talking about build systems and linking. Compared to when we started, we got through a lot of the immediately pressing topics. So now we're only meeting when there's enough to talk about. That currently works out to about every two weeks. 

## Project perspectives on AI

The Project started seeing more uses of LLM-generated pull requests across the board — from people using it to write or translate commit messages all the way to fully generated PRs where even the reviewer feedback was fed back to the model. The latter, in particular, often causes heavy reviewer burden for low-effort and low-quality changes.

The Project members' attitudes also vary greatly. There are some who use these tools, derive a lot of benefit from them, and believe the Project does too, while others want to ban their use wholesale for ethical, IP rights, labor, environmental, review burden, or externalized cost reasons.

Many free and open source projects have a policy around LLM tool use, disclosures, etc., and it has become clear that for reviewers, maintainers, and moderators to be able to operate effectively, we need a policy as well. These discussions have happened across multiple Zulip channels with a subset of people.

In addition, there may be Foundation members who are heavily involved in the AI space or Project goals proposed to, e.g., integrate AI tooling or standards.

For example, as of this writing, we can't enable/disable GitHub Copilot on individual repositories. If the Compiler team wants to block its use, we can't have any other rust-lang repo have it enabled (and *vice versa*). That is something we'll reach out to GitHub about, but either way, we need a Project-wide policy.

To that end, Niko asked the Project members for their personal views on the topic. You can read through the [comprehensive summary](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/feb27-summary.html) and [all the individual comments](https://nikomatsakis.github.io/rust-project-perspectives-on-ai/all-comments.html). These will serve as a starting point for drafting the actual policy.

## Worth a look

### Rust Foundation posts

* [FOSDEM 2026 — Rust Devroom in Review](https://rustfoundation.org/media/guest-blog-fosdem-2026-rust-devroom-in-review/)

### Rust Project updates

* [Writing a Linux GPU Kernel Driver in Rust with Daniel Almeida](https://youtu.be/rgjTPBRae6I)
  * Interview from Kangrejos 2025
* [Announcing Rust 1.94.0](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0/)
* [2025 State of Rust Survey Results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/)
* [Rust participates in Google Summer of Code 2026](https://blog.rust-lang.org/2026/02/19/Rust-participates-in-GSoC-2026/)


## Stats

Total words of meeting minutes written: 396.7k (June 2025–February 2026)

Meetings attended: 45

Total words of meeting minutes written (February): 73.7k

Average (mean) word count per team meeting:

* Cargo: 1.6k
* Lang triage: 2.5k
* Libs-API: 4.8k
* Leadership Council: 2.4k