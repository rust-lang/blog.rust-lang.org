+++
path = "inside-rust/9999/12/31/june-2025-rust-program-management-update"
title = "June 2025 Rust Program Management Update"
authors = ["Tomas Sedovic"]
+++

When I saw the [Rust Program manager post on Inside Rust][pm], it was one of those situations where I had to apply&#8201;&#8212;&#8201;despite many uncertainties&#8201;&#8212;&#8201;because if I hadn't, I'd regret it for the rest of my life.

[pm]: https://blog.rust-lang.org/inside-rust/2025/03/18/hiring-for-program-management/

I feel grateful having been brought in and I'd like to give everyone an update on what's happened during this first month.

## Rust PM Why?

This [June 2025 Leadership Council Update][council-update] summarises it well:

> We've seen a lot of returns from good Project-directed program management. In support of this valuable work — so that we can do more with it while making it more sustainable — we allocated funds to the Edition and Goals teams to hire a program manager.
> [...]
> Going forward, we're expecting this work to help us and our teams across the Project better manage the many ongoing initiatives that are of importance to us.

[council-update]: https://blog.rust-lang.org/inside-rust/2025/06/11/leadership-council-update/#program-manager

I see this role being about two main things: direct support for the Goals and Edition teams, and engaging deeply with the Rust teams overall.

The latter is important to maintain the context on the work everyone's doing, the struggles they're facing and being able to help them. This knowledge will then also translate to the Goals and Editions programs.

People were doing this in their spare time, but it's one of those things that takes away from what you're really trying to achieve.

I can focus fully to improving the program, relieving burden from folks doing this on the side, building connections and making sure that relevant information reaches the right people.

I'm still learning how the Project does things, the tooling, getting to know everyone, but there were plenty of opportunities to start helping right away.

## Getting Embedded in Teams

First up, I've joined a lot of the regular synchronous meetings: Lang, Libs API, Cargo, Spec, Vision, Style, Leadership Council and a few others.

We all got introduced and I started taking minutes in every meeting. There's room for improvement, but this resulted a consistent record on what was discussed across the teams. People were taking notes before, but this let them talk without worrying about the written record.

Being where the discussions happen means I'm learning what each team is doing, what they're struggling with, any connections they need and I'm able to act on those. It also helps me learn their processes (and possibly work on improving those) as well as get the broader picture across the entire project.

Wonderfully, people immediately had thoughts on what they'd like to see. A lot of concrete suggestions of areas where the communication could be improved&#8201;&#8212;&#8201;especially around goals where the team was asked for support and then hasn't seen many updates.


## Leave It Better Than You Found It

This is an old hiking rule that resonates with me deeply.

Whenever I (or anyone else&#8201;&#8212;&#8201;just let me know!) encounter something that's incorrect, or missing or just a papercut or something that could be better, I go in and fix it. A few highlights:

* Added `Design meeting: ` prefix to the lang-team issue template on GitHub: [lang-team PR#333](https://github.com/rust-lang/lang-team/pull/333).
    * A tiny papercut: when [proposing a design meeting][design-meeting], the Lang team has a GitHub issue template with the expected structure. It is expected that the issue's title starts with the words "Design meeting: ", but this was missing from the template.
* Style and Vision meetings added to Rust calendar: [calendar PR#91](https://github.com/rust-lang/calendar/pull/91) & [calendar PR#92](https://github.com/rust-lang/calendar/pull/92).
    * [The Rust team meetings][calendar] are published in an `ics` calendar people can subscribe to. The Style and Vision meetings were missing from the list.
* Helped resolve the speaker line-up confusion following the [RustConf announcement][rust-conf]:
    * Some people in the community mistakenly [thought there were only a handful of speakers coming to the 2025 Rust Conf][lineup-confusion].
    * I brought this to the attention the Rust Foundation Communications Director (Gracie Gregory) and she made the post clearer and the full speaker line-up more prominent.
* The Lang Team's [Calendar page][lang-cal] listed a wrong time for the for the design meeting and didn't have the meeting link despite the meeting being open to the publice. The [lang-team PR#334](https://github.com/rust-lang/lang-team/pull/334):
    * Fixes the meeting time.
    * Adds the meeting link.
    * Adds links that help figure out when the meeting happens in the reader's local time.
* I've noticed a couple small things around the `gh` tool integration with the [rust-project-goals repo][rpg] that I plan on addressing soon.

[design-meeting]: https://lang-team.rust-lang.org/meetings/design.html
[calendar]: https://github.com/rust-lang/calendar/
[rust-conf]: https://rustfoundation.org/media/announcing-the-rustconf-2025-speaker-lineup/
[lineup-confusion]: https://rust-lang.zulipchat.com/#narrow/channel/335408-foundation/topic/Clearer.20messaging.20on.20RustConf.20speaker.20lineup/
[lang-cal]: https://lang-team.rust-lang.org/calendar.html
[rpg]: https://github.com/rust-lang/rust-project-goals/

## Making Connections

Having been an engineer, a team lead and a people manager in the past lives, there was always that moment at a meeting where someone said "we should talk to someone from that team".

Which everyone agrees on and then someone has actually set it up and it's such a small thing, but also just *such a bother*.

Being there and saying "yep, I'll set it up" and then everyone moves on confident that this will in fact happen is great for everyone involved.

Indeed, this is what Josh said a couple of weeks ago:

> "It was great being in a Rust meeting, talking about setting up a one-off meeting with specific people, and then it magically happening without any of us having to do anything to schedule it. Often, scheduling a meeting is harder than actually having the meeting. Tomas makes it feel like we can concentrate on getting work done, and things don't get lost or fall through the cracks with him around."  
--- Josh Triplett, Rust Project lang, library, Cargo, spec, and style teams.

Besides warming my heart, that's exactly the feeling we want to elicit with every interaction.


### Secure Random Generation in the Standard Library

The Lang Team were considering the proposal to add some traits related to random number generation into `core` and some functions that would produce secure random numbers into `std`:

<https://github.com/rust-lang/rust/issues/130703>

There were a lot of open questions and the Libs-API team wanted to talk them through with a [rand crate][rand] maintainer. I've reached out to [Diggory Hardy][dhardy], set the meeting up and then took minutes.

[rand]: https://crates.io/crates/rand
[dhardy]: https://github.com/dhardy


### TPDE Compiler Back-End Framework

The Rust project's work often intersects with what's going on inside the broader ecosystem or even outside&#8201;&#8212;&#8201;for example in academia. We want to make sure that when that happens, we communicate with the people doing this "outside work".

An early example of this is a new paper that came out describing [TPDE: a compiler back-end framework][tpde] focused on producing unoptimised build with impressive speedup compared to LLVM.

[tpde]: https://arxiv.org/abs/2505.22610

As compilation speed is a frequent topic, this is an area the Project has a very keen interest in.

I've invited the authors of the paper (Alexis Engelke, Tobias Schwarz and Tobias Kamm) to our [Zulip chat where they answered some questions and clarified some early misunderstandings][tpde-thread].

[tpde-thread]: https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/TPDE.3A.20A.20Fast.20Adaptable.20Compiler.20Back-End.20Framework/

### build-std

Right now, stable Rust doesn't allow building the standard library for your project. This has been desired in many different contexts: using tier-3 targets (for which Rust doesn't ship a standard library), optimising it for a particular architecture (useful in embedded), patching things in and out, changing its configuration and so on.

It is of particular interest to the [Rust for Linux effort][rfl], too.

[rfl]: https://rust-for-linux.com/

The list of use cases is almost as long as the number of RFCs that tried to solve this over the years.

This year, the Project has committed to building the standard library as as [one of the goals][buildstd-goal]. It needs close coordination between many teams.

[buildstd-goal]: https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html

I've caught up on what what happened so far and started attending the build-std meetings. A lot of good work was already underway, but I've explained the need to update the tracking issues, clarified the teams' expectations if we propose a new goal for the second half of 2025 and hopefully started a relationship that will make it easier for us all to communicate going forward.

### Retpoline flags PR

The [pull request][retpoline-pr] adding the `-Zretpoline` and `-Zretpoline-external-thunk` flags is needed by the Rust for Linux project. The PR looked good enough to be reviewed by the team, but needed the author's go ahead.

I've reached out, they made a few final updates and then marked it as ready for review and eventually got it merged!

[retpoline-pr]: https://github.com/rust-lang/rust/pull/135927#issuecomment-2898902340


### `try_exact_div` method on `NonZero<{integer}>` ACP

The Libs API team does a weekly meeting where they go over open ACPs (API Change Proposals), decide whether each is something they want to see added to the library and possibly request changes before accepting it.

I [relayed the team's feedback][acp-feedback] back to the ACP's author.

[acp-feedback]: https://github.com/rust-lang/libs-team/issues/587#issuecomment-2939452411



## Publications

### Compiler Operations Blog Post

About six months ago, the Rust Project hired [Antonio (apiraino)][apiraino] for the [compiler-ops role][compiler-ops].

[compiler-ops]: https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops

[apiraino]: https://github.com/apiraino

As the contract was coming to a close, the Rust Leadership Council needed to make a decision about renewing the funding.

We've decided to gather and publish what Antonio has been doing. This would provide useful data to the Council as well as help the broader community and stakeholders understand the decisions and work of the Project.

I've reached out to Antonio and brought up the blog idea. He wrote a draft, we've polished it up together and I've opened the pull request.

This post was then discussed by the Leadership Council who decided to [extend the role for another six months][council-funding].

[council-funding]: https://github.com/rust-lang/leadership-council/issues/181#issuecomment-2981573015

Here's the blog post:

<https://blog.rust-lang.org/inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations/>


### May Project Goals Update

Every month, [the Goals team][goals-team] publishes an update on all the goals that are active for that period. This is a semi-automated effort where [rust-project-goal tooling][rpg] gathers up comments from the tracking issues associated with each goal and prepares a draft post.

[goals-team]: https://www.rust-lang.org/governance/teams/launching-pad#team-goals

That needs to checked and massaged into the final form and then published. As part of this, the goals team (yours truly included) reached out to several people to make sure all the Flagship goals have an update (check!).

The May update is here:

<https://blog.rust-lang.org/2025/06/20/may-project-goals-update/>

This is one of those things I'll be happy to mostly (or completely) take over.

### This Post

A little tongue-in-cheek, but I plan for this to be a monthly feature.

Any feedback on the format, what you'd like to see (or not) please [let me know][tomas]!

[tomas]: https://rust-lang.zulipchat.com/#user/893815


## What's Next

* Take over the updates for Rust for Linux (and free up Niko's time)
    * Run meeting, keep minutes
    * Update tracking issues, links to the minuthes
    * Monthly update for project goals
* Project goal administration (ditto)
    * Smooth things out during the CFP period (read proposals, get feedback, get proposals merged)
    * Ping authors for regular updates
    * Prepare the monthly blog post _in a timely fashion_
    * Prepare for next CFP towards the end of the goal program
* Put all meeting minutes in searchable places
* Follow-up on some conversations from Rust Week / All Hands with external sponsors and contributions (C++ interop and the like)
* [rust-project-goals][rpg] tooling improvements (crashes with an inscrutable error if the `gh` tool is not authenticated)
* Reach out to the Code Signing folks similarly to the build-std team

## In Closing

Phew, this was a busy month!

Busy, but really fulfilling. I love working in open source, with people from all over the Earth and across time(zones).

The fact that all the information is out there is something I've been missing recently and _it feels right_.

All the feedback I've heard so far has been really positive (but I absolutely want to hear if there's anything you'd like to see or change!).

And like, everyone can set up a meeting. But I love being able to do these often less glamorous parts of the work letting everyone focus on what they actually care about.

Everyone has been really welcoming, patient with my questions and full of suggestions of how I can help right away. Thank you all!

_Special thanks to Antonio (apiraino), Josh Triplett, Niko Matsakis, Rémy Rakic (lqd) and TC who helped me get started and withstood a barrage of questions._
