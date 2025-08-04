+++
path = "inside-rust/9999/12/31/program-management-update-2025-07"
title = "Program management update — July 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/"
+++

Welcome to the second Rust PM update!

Things were a bit busy in personal life. We had to temporarily move to a smaller apartment which was quite time-consuming and energy-draining. And after that, I took a week off and we spent it in the mountains.

So things were slower in July. What dominated was the Project Goals work.

## Wrapping up 2025 H1 goals

As we entered the second half of the year, it was time to wrap up the current Project Goals and get the new ones going. In the spirit of helping people focus on other things, I took this over.

First, I've sent a couple requests for goal updates. This lead me to noticing and filing an issue with the triagebot. More details in the [`ping-goals` retry section](#ping-goals-retry-issue).

After people provided their final messages, I've published the final 2025H1 Project Goals update:

https://github.com/rust-lang/blog.rust-lang.org/pull/1667

(**TODO** change the link to the published blog if it's merged in time)

With that done, I've closed most of the tracking issues. Some goals are being renewed for the H2 period so we're keeping those open.

(**TODO**: again, make sure we've actually closed the issues before publishing this post.)

## Starting 2025 H2 goals

With all of that out of the way, Niko published the [call for 2025 H2 goal proposals][2025H2CFP].

[2025H2CFP]: /inside-rust/2025/06/23/project-goals-2025h2-call-for-submissions/

Based on the feedback we've received, we've made some changes in this period:

* We're getting the proposals in front of the Teams and their leads earlier (well before the final RFC is written) so they are aware and have input into what's being proposed.
* We're asking them to suggest what they'd like to see as a flagship goal before the final selection is made.
* For each proposed goal, we've requsted each team with asks mentioned in the goal to select a "champion" who commits to making sure the goal is not blocked by that team.

Once all the champions are selected, we'll meet with the team leads, go over the goals and figure out which ones we want to accept.

Similarly, we'll take their input regarding flagship goals.


As the goals were being proposed, Niko, Rémy and I looked at the PRs, fixed any CI issues and merged them.

For each team, I've opened a call for champions Zulip thread listing goals that requested their support. [Here's an example for the Lang team](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/2025H2.20Goal.20Review).

As with any new process, people had questions. I was there to help clarify what we're asking for, the overall plan and next steps. When there were specific questions for a goal, I made sure to connect the right people.

## Verification and Secure Mirroring for crates.io

We now have a detailed update on the crates.io code signing program and its status.

Quick summary:

* We've settled on using TAP-16 Merkle Tree implementation of [TUF (The Update Framework)][tuf] for crates.io.
* We've settled on maintaining a top-level quorum but removing immediate levels for simplicity.
* We've reached a consensus with the Infrastructure team for deployment planning.
* A public MVP implementation is coming Soon™ (by the end of August).
* We will test various optimizations to settle on something that works for our tooling (Rustup, releases, crates.io, update, signing etc.).
* Role keys will live in KMS (Key Management Service).

Open Questions and Next Steps:

* Which specific optimizations will fit our needs.
* How to implement the Merkle tree to reduce round-trips.
* Stand up infrastructure to host this.


For more details, read this [goal update for more details](https://github.com/rust-lang/rust-project-goals/issues/271#issuecomment-3133590786) and/or reach out to @walterhpearce.

One of the things on my radar is to start publishing more frequent and detailed updates. I've joined the signing meetings to be more aware of what's happening.

[tuf]: https://theupdateframework.io/

## Rust for Linux

[Rust for Linux][rfl] is an ongoing effort (started in 2020) to add support for Rust to the Linux kernel. The project allows to write kernel code, such as drivers and other modules, in a memory safe language, with hopefully fewer bugs and nicer tooling. In addition, [one of its goals](https://lore.kernel.org/lkml/20210414184604.23473-1-ojeda@kernel.org/) it to allow more people get involved overall in developing the kernel thanks to the usage of a modern language.

[rfl]: https://rust-for-linux.com/

The project currently has to rely on unstable Rust. This makes it less appealing for companies and individuals as unstable features can by definition change or even be removed. We want there to be minimal (ideally zero) churn on Rust code that's been accepted to the kernel.

There's been an ongoing collaboration with the Rust Project to get the language, compiler and tooling to a point where it can be completely compiled on stable Rust.

Rust for Linux was a Flagship goal the [second half of 2024][rfl2024h2] as well as the [first half of 2025][rfl2025h1].

[rfl2024h2]: https://github.com/rust-lang/rust-project-goals/blob/main/src/2024h2/rfl_stable.md


[rfl2025h1]: https://rust-lang.github.io/rust-project-goals/2025h1/rfl.html

This effort requires close collaboration with the Lang and Compiler teams, among others, and contact points on both sides to bridge the gap between the two projects. Until now, that was done by Niko Matsakis et al. on the Rust side and Miguel Ojeda et al. on the Rust for Linux side.

One of the hopes of the PM role was to be able to step in and bring the communication between efforts like these closer and free up the Project members' time.

I am now running the meetings, preparing the agenda, and making sure the tracking issues are up-to-date and bringing any requests or concerns to the relevant Project teams or people.

Miguel Ojeda will continue to be the point of contact on the Linux side.

I helped Miguel propose two 2025 H2 goals ([compiler][rfl-compiler] and [language][rfl-lang] features for getting Rust for Linux into stable Rust) and should they be accepted I will be the Point of contact for them.

[rfl-compiler]: https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-compiler.html
[rfl-lang]: https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-language.html

This lets Niko disengage from the program completely and focus on the many other things he's doing across the Project.


## Misc

### `ping-goals` retry issue

The request for updates is done by the `ping-goals` command of the @triagebot. This returned an error ("Failed to await at this time: connection closed before message completed"), but did actually ping everyone.

[Multiple times][goals-spam], in fact.

[goals-spam]: https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Prepare.20const.20traits.20for.20stabilization.20.28goals.23106.29

As you can see in the thread, this wasn't a new behavior either. Precisely the sort of paper cut I care about improving for everyone:

<https://github.com/rust-lang/triagebot/issues/2108>

[Urgau][urgau] looked into it and found out this is because rustbot was sending the pings one-by-one and taking some time to finish.

[urgau]: https://github.com/Urgau

Zulip interpreted this as a timeout and retried the requests a few times.

[Here's the fix he opened][ping-goals-fix]. 

[ping-goals-fix]: https://github.com/rust-lang/triagebot/pull/2109

Thank you Urgau!

### Leadership Council Minutes

The [Leadership Council][council] is a team charged with the success of the Project as a whole. They're composed of members of the top-level Rust teams.

The Council meets every two weeks and after each meeting they publish minutes to [the leadership-council repository][leadership-council-minutes].

This makes them accountable to the teams they represent and transparent to the Project and the community as a whole. It also helps show if they're missing anything important to the teams who can then bring it to the Council's attention.

As with every other meeting, when an active participant has to take minutes, this results in diminishing their attention to the topic as well as the quality of their notes (most people can't speak or fully consider what other people said and write at the same time).

And similarly, going through the minutes, making them legible and publishing them takes additional effort and attention. When these things pile up, they can even lead to burning out.

Having taken over the minutes for the Leadership Council meetings in June, I've now closed the loop by taking over the clean-up and publishing as well.

[council]: https://www.rust-lang.org/governance/teams/leadership-council

[leadership-council-minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes/sync-meeting

## Fun Stats

I've collected a few numbers to show some of the regular work that's happening.

Total words of meeting minutes written: 92.6k (June + July). That's more than [the novel I'm currently reading][bjones]!

[bjones]: https://en.wikipedia.org/wiki/Bridget_Jones%27s_Diary_(novel)

### The current month (July 2025)

Meetings attended: 24

Total words written: 51.5k

Average (mean) word count per team meeting:
* Cargo: 1.6k
* Lang triage: 2.2k
* Libs: 4.7k
* Leadership council: 2.9k

In contrast, this is the week when I was away:
* Cargo: 600
* Lang triage: 604
* Libs: 705

### The previous month (June 2025)

Meetings attended: 39

Total words written: 41.1k

Average word count:
* cargo: 1.3k
* Lang triage: 1.8k
* Libs: 3.9k
* Leadership council: 1.9k

### May 2025

I wasn't here in May so these can serve as a comparison.

Meetings attended: 0

Average word count:
* cargo: 857
* Lang triage: 1.4k
* Libs: 1.3k
* Leadership council: 1.5k

### Caveat emptor

More words doesn't necessarily mean better and there are other aspects that affect this: number of people in the meeting, how much discussion a topic needs, etc. And this is not a target I or anyone else is holding me to.

But it is kind of interesting to see.

If you have an idea for any more useful things to track, please let me know!
