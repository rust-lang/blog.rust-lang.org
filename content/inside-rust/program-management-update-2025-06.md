+++
path = "inside-rust/2025/06/30/program-management-update-2025-06"
title = "Program management update — June 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/"
+++

When I saw TC's [post][pm] on Inside Rust, back in March, about looking to bring on someone to help the Project build on its successful program management work, it was one of those situations where I had to apply &mdash; despite many uncertainties &mdash; because if I hadn't, I'd have regretted it for the rest of my life.

[pm]: https://blog.rust-lang.org/inside-rust/2025/03/18/hiring-for-program-management/

I feel grateful for having been brought on &mdash; I started at the beginning of June &mdash; and I'd like to give everyone an update on what's happened during this first month.

## Why a Rust program manager?

The June 2025 [Council update] summarizes well why the Project hired this for role:

> We've seen a lot of returns from good Project-directed program management. In support of this valuable work — so that we can do more with it while making it more sustainable — we allocated funds to the Edition and Goals teams to hire a program manager. [...] Going forward, we're expecting this work to help us and our teams across the Project better manage the many ongoing initiatives that are of importance to us.

[Council update]: https://blog.rust-lang.org/inside-rust/2025/06/11/leadership-council-update/#program-manager

I see this role being about two main things: direct support for the Edition and Goals teams, and engaging deeply with the teams of the Project overall.

The latter is important for maintaining the needed context on the work everyone is doing and the struggles they're facing. It's this on-the-ground knowledge that makes it possible to help contributors work effectively with teams, help teams identify and achieve their priorities, and help make the Edition and Goals programs continued successes.

Dedicated people in the Project have been doing this work in the time they have available, but it's one of those things that takes away from what else you're really trying to achieve.

Because of the Council making this work a priority and funding it &mdash; which was made possible by the generous unrestricted $1M [donation] from Microsoft in 2024 that backed the [Project Priorities budget] &mdash; I can focus fully on improving the program, thereby relieving burden from others and making the work more sustainable while growing its reach.

[Project Priorities budget]: https://github.com/rust-lang/leadership-council/issues/183
[donation]: https://rustfoundation.org/media/1m-microsoft-donation-to-fund-key-rust-foundation-project-priorities/

I'm still learning my way around the Project &mdash; how it does things, the tooling, getting to know everyone, etc. &mdash; but there were plenty of opportunities to start helping right away.

## Getting embedded in teams

First up, I've joined a lot of the regular synchronous meetings: Lang, Libs API, Cargo, Spec, Vision, Style, Leadership Council, and some others.

After getting introduced, I started taking minutes in every meeting. This has resulted in a more consistent record of what was discussed and how decisions were made, and this transparency helps teams collaborate. People were taking minutes before, but the coverage varied, and my doing this helps those on the teams focus on their work without also juggling this task.

Being where the discussions happen means I'm learning what each team is doing, what they're struggling with, and what help they need &mdash; and I'm able to act immediately on that. It also helps me learn their processes (and possibly work on improving those) and get a broader picture across the entire project.

Wonderfully, people immediately had thoughts on what they'd like to see. A lot of the concrete suggestions focused on areas where communication could be improved.

## Making connections

Having been an engineer, a team lead, and a people manager in past lives, there was always that moment in a meeting where somebody said, "we should talk to someone from that other team". Everyone would nod and agree, but then someone would have to actually set it up. It's such a small thing, but also just *such a bother*.

Being there and saying, "yep, I'll set it up", and then everyone moves on, confident that this will in fact happen, is great for everyone involved.

Indeed, this is what Josh said a couple of weeks ago:

> "It was great being in a Rust meeting, talking about setting up a one-off meeting with specific people, and then it magically happening without any of us having to do anything to schedule it. Often, scheduling a meeting is harder than actually having the meeting. Tomas makes it feel like we can concentrate on getting work done, and things don't get lost or fall through the cracks with him around."
>
> &mdash; Josh Triplett

Besides warming my heart, that's exactly the feeling we want to elicit with every interaction.

### Secure random generation in the standard library

The Libs API team is considering a proposal to add some traits related to random number generation into `core` and some functions that would produce secure random numbers into `std`:

<https://github.com/rust-lang/rust/issues/130703>

There were a lot of open questions and the team wanted to talk them through with a [rand crate] maintainer. I reached out to [Diggory Hardy], set the meeting up, and then took the minutes. Progress was made.

[rand crate]: https://crates.io/crates/rand
[Diggory Hardy]: https://github.com/dhardy

### build-std

Right now, stable Rust doesn't allow building the standard library for your project. This has been desired in many different contexts: using tier-3 targets (for which Rust doesn't ship a standard library), optimizing it for a particular architecture (useful in embedded), patching things in and out, changing its configuration, and so on.

It is of particular interest to our partners at the [Rust for Linux project].

[Rust for Linux project]: https://rust-for-linux.com/

The list of use cases is almost as long as the number of RFCs that have tried to solve this over the years.

This year, the Project adopted [build-std] as a project goal. It needs close coordination between many teams.

[build-std]: https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html

I caught up on what has happened so far and started attending the build-std meetings. A lot of good work is already underway, and I'll be helping keep the tracking issues updated and helping the teams to clarify their expectations as we propose a new goal for the second half of 2025.

### TPDE

The Project's work often intersects with things happening outside the Project, e.g. in our ecosystem, in industry, in academia, etc. We want to make sure that we communicate with the relevant people, wherever they are.

An early example of this was prompted by a new paper that came out describing [TPDE], a compiler backend focused on producing unoptimized builds at impressive speeds.

[TPDE]: https://arxiv.org/abs/2505.22610

As compilation speed during development is frequently mentioned by our users, this is an area where the Project has a keen interest.

After people in the Project started discussing the paper, I invited its authors (Alexis Engelke, Tobias Schwarz and Tobias Kamm) to [the thread][tpde-thread] in our Zulip chat where they answered questions and clarified early misunderstandings.

[tpde-thread]: https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/TPDE.3A.20A.20Fast.20Adaptable.20Compiler.20Back-End.20Framework/

### Retpoline flags

The `-Zretpoline` and `-Zretpoline-external-thunk` flags are needed by the Rust for Linux project. Reviewers were unsure whether the author was ready for the [PR][retpoline-pr] adding these to be reviewed, so I reached out. The author made a few final updates and then marked it as ready for review. It's now been merged!

[retpoline-pr]: https://github.com/rust-lang/rust/pull/135927#issuecomment-2898902340

### `try_exact_div` method on `NonZero<{integer}>` ACP

The Libs API team does a weekly meeting where they go over open ACPs (API change proposals), decide whether each is something they want to see added to the library, and possibly request changes.

Sometimes the team decides generally what it wants to ask of the author, but someone has to do the work of writing that up clearly. As I get more comfortable with the work of the team, I'm volunteering to do that in more cases, as I did recently on [ACP #587][acp-feedback].

[acp-feedback]: https://github.com/rust-lang/libs-team/issues/587#issuecomment-2939452411

## Publications

### Compiler operations blog post

In January, the Project hired [Antonio (apiraino)][apiraino] to work on [compiler-ops] on a six month contract. The contract was coming up for renewal, and the Council needed to make a decision about renewing the funding.

[compiler-ops]: https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops
[apiraino]: https://github.com/apiraino

To support the Council in making this decision, and to help people inside and outside the Project understand better the work of compiler-ops, we decided to gather and publish details about the work that Antonio has been doing.

I reached out to Antonio who wrote a first draft which we then polished up together and turned into a  pull request. The Council reviewed the post and decided to [extend][council-funding] funding for the role for another six months.

[council-funding]: https://github.com/rust-lang/leadership-council/issues/181#issuecomment-2981573015

Here's that blog post:

<https://blog.rust-lang.org/inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations/>

### Project goals update

Every month, [the Goals team][goals-team] publishes an update on all the goals that are active for that period. This is a semi-automated effort where [tooling][rpg] gathers up comments from the tracking issues associated with each goal and prepares a draft post.

[goals-team]: https://www.rust-lang.org/governance/teams/launching-pad#team-goals

That draft needs to be checked and massaged into the final form for publication. As part of this, the goals team (yours truly included) reached out to several people to make sure all flagship goals had received an update (check!).

The May update is here:

<https://blog.rust-lang.org/2025/06/20/may-project-goals-update/>

This is one of those places where I'll be able and happy to take most or all of the work off of others.

### This post

It's maybe a little recursive to mention here, but we plan for this update to be a monthly feature. As part of investing in this work, the Council asked for regular reporting, and it's important generally that people know what's happening with the program management program itself. (It's turtles all the way down!)

If you have any feedback on the format or of what you'd like to see, please [let me know][tomas]!

[tomas]: https://rust-lang.zulipchat.com/#user/893815

## Leave it better than you found it

Leaving things better than you found them is an old hiking rule that resonates with me deeply.

Whenever I encounter something that's incorrect, missing, a papercut, or just something that could be better, I go in and fix it. (If you find anything like this, let me know, and I'll dive in.)

As one example, following the [RustConf announcement][rust-conf], some people in the community mistakenly [thought there were only a handful of speakers coming to the 2025 RustConf][lineup-confusion]. I brought this to the attention of the Rust Foundation Communications Director (Gracie Gregory) and she made the post clearer and the full speaker line-up more prominent.

As another, I've been helping to clean up team websites and calendars, e.g. [calendar PR#91](https://github.com/rust-lang/calendar/pull/91), [calendar PR#92](https://github.com/rust-lang/calendar/pull/92), and [lang-team PR#333](https://github.com/rust-lang/lang-team/pull/333).

[design-meeting]: https://lang-team.rust-lang.org/meetings/design.html
[calendar]: https://github.com/rust-lang/calendar/
[rust-conf]: https://rustfoundation.org/media/announcing-the-rustconf-2025-speaker-lineup/
[lineup-confusion]: https://rust-lang.zulipchat.com/#narrow/channel/335408-foundation/topic/Clearer.20messaging.20on.20RustConf.20speaker.20lineup/
[lang-cal]: https://lang-team.rust-lang.org/calendar.html
[rpg]: https://github.com/rust-lang/rust-project-goals/

## What's next

* Handle more work in support of the collaboration with Rust for Linux (and thereby free up Niko's and TC's time). E.g.:
    * Prepare agendas for the meetings, run them, and keep minutes.
    * Update the tracking issues.
    * Make the monthly update for each project goal.
* Project goal administration (i.e. free up Niko's time). E.g.:
    * Smooth things out during the call for proposals period (read proposals, work with authors to improve proposals, get feedback, get proposals merged).
    * Work with authors to collect regular updates.
    * Prepare the monthly blog post on time.
    * Prepare for the next call for proposals.
* Edition administration (i.e. free up TC's and Eric Huss' time).
* Revise meeting minutes after meetings to make them more clear and readable, and put all meeting minutes in searchable places.
* Follow-up on conversations from Rust Week and the All Hands with external sponsors and contributors (e.g., on C++ interop, safety-critical, security, specification work, and AI).
* Improve the [tooling][rpg] for project goals (e.g., currently it crashes with an inscrutable error if the `gh` tool is not authenticated &mdash; I'll fix it).
* Reach out to the code signing folks to improve collaboration in a way similar to what's being done with build-std.

## In closing

Phew, this was a busy month! Busy, but really fulfilling. I love working in open source and with people from all over space and time(zones). The fact that all the information is out there _feels right_.

All the feedback I've received so far has been really positive (but I absolutely want to hear if there's anything you'd like to see or see change!).

While everyone _can_ set up meetings and do other supportive work, I love being able to do these often less glamorous parts so as to let everyone else focus on what they're best at and care most about.

Everyone has been really welcoming, patient with my questions, and full of suggestions for how I can help right away. Thank you all!

_Special thanks to Niko Matsakis, Josh Triplett, Antonio (apiraino), and Rémy Rakic (lqd) &mdash; and in particular to TC who [proposed][pm-proposal] and [defined][pm] this role, worked to bring me on, and set me up for success by helping me get up to speed and withstanding my barrage of questions._

[pm-proposal]: https://github.com/rust-lang/leadership-council/issues/151
