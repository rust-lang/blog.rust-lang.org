+++
path = "inside-rust/2026/02/03/first-look-at-2026-project-goals"
title = "First look at 2026 Project goals"
authors = ["Niko Matsakis"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

Hi all,

This is an update on the Project goals plan.

## We want your feedback!

We have just published the [**first draft** of the Project goals RFC](https://rust-lang.github.io/rust-project-goals/2026/index.html). This first draft contains the [goals proposed thus far](https://rust-lang.github.io/rust-project-goals/2026/goals.html) as well as candidate [flagship themes](https://rust-lang.github.io/rust-project-goals/2026/flagships.html).

As a *first draft*, the purpose of this document is to **solicit feedback and spark discussion** -- so please, take a look! If you have questions or comments, open up a topic in the [#project-goals/2026-workshop][] channel.

[#project-goals/2026-workshop]: https://rust-lang.zulipchat.com/#narrow/channel/546987-project-goals.2F2026-workshop

## Questions

Here are some of the questions I would like people's input on (these questions link to questions in the FAQ that give more details):

* Do you have concerns about any goals that ask support from one of your teams?
    * E.g., you think this goal is a bad idea, or you think that your team does not have capacity to support the goal.
* Are the "Team Asks" in these goals appropriate?
    * In particular, [should they be larger](#i-see-a-small-medium-large-ask-of-my-team-what-does-that-mean)?
* What do you think of the [flagship themes](#what-is-a-flagship-theme), do they make sense to you?
    * Do you have alternative ideas for flagship themes? [You can propose your own.](#can-i-propose-a-new-flagship-theme)
    * Would you be interested in serving as a *point of contact* for one of those themes, helping the goals program to draft periodic updates and track overall progress?
* [Is there work that you would like to do that you think would make a good goal or fit well with the themes?](#what-if-i-have-a-goal-i-would-like-to-propose-is-it-too-late)
    * (If you're blocked on funding, [we may be able to help](#i-d-like-to-propose-a-goal-but-i-would-need-funding-to-work-on-it-is-there-funding-available-for-project-goals)!)

## Frequently asked questions

### Why are you saying 2026 goals and not 2026H1 goals?

We're shifting the process from something we do every 6 months to something we do once a year. The goal program is focused on larger, long-running goals that will take significant work during the year.

Team members can add new goals later in the year but you must already have a champion and general agreement from the team that this is a goal they like.

### What is the process timeline?

In each case the task begins at the start of the month and then extends throughout the month:

* [x] January: Solicit goal proposals and write up first draft.
* [ ] February: Solicit feedback and prepare RFC.
* [ ] March: Open RFC and get it merged.
    * The RFC must be approved by the leads of every team that has a "team ask" listed in any goal.
* [ ] April: Announce 2026 Project goals.

### What if I have a goal I would like to propose?  Is it too late?

No, it's not too late. We accept goal proposals throughout the year, but we require goals coming later in the year (including February) to already have a champion and buy-in from the team. (Of course, as a team member, you can champion your own goal within your team).

### I'd like to propose a goal, but I would need funding to work on it. Is there funding available for Project goals?

Yes! We are discussing with possible sponsors both of Project goals and of champions of Project goals. Please reach out to [nikomatsakis](https://rust-lang.zulipchat.com/#user/116009) to discuss.

### What is a "Flagship Theme"?

The idea of a flagship theme is that, for people paying only limited attention to the Rust Project, it should help them get a high-level view of the "big things" that they can expect from the Project. These stretch somewhat past a year, many of them (e.g., "Beyond the `&`") represent multi-year technical programs.

The current set of flagship themes was decided by reading over the goals and looking for the trends that we saw. However, I would like to get to the point where every *flagship theme* has a *point of contact* that is holding the vision, fielding questions, and helping to draft updates for periodic blog posts on overall progress.

### Can I propose a new flagship theme?

Yes! But please do so by Feb 14 (Valentine's Day). If you'd like to propose a new flagship theme, you can open a PR using the [`FLAGSHIP_TEMPLATE.md`](https://github.com/rust-lang/rust-project-goals/blob/main/src/FLAGSHIP_TEMPLATE.md). 

If you'd like to propose changes to a flagship theme, or would consider acting as a point-of-contact for a theme, reach out on [`#project-goals/2026-workshop`][#project-goals/2026-workshop].

### I see a "small|medium|large" ask of my team, what does that mean?

In brief:

* **Vibes**: Quick check if your idea is worth exploring and bringing a more mature version to the team in the future.
* **Small**: Routine small reviews and actions by the team.
    * --> This is typically appropriate for a task that will require a single PR, like adding a lint.
* **Medium**: Exploring a design or feature with dedicated support from a team member and involving the rest of the team in a vibe-check and small way.
    * --> This is typically appropriate for an experiment or something early stage or a task that will require multiple PRs to complete.
* **Large**: Reaching consensus within the team about a design or feature.
    * --> This is typically appropriate for RFCs, stabilizations, or other things that require broad team consensus.

### Can you give me some examples of what small|medium|large means?

Here are some rules of thumb and historic examples, focusing on lang + compiler:

| Task | Lang | Compiler |
| --- | --- | --- |
| Adding a lint | Small | Small |
| Landing a complex compiler change | - | Medium |
| Driving an early stage lang experiment | Medium | Medium |
| Accepting a lang RFC | Large | Medium |
| Stabilizing a lang feature | Large | Medium |
| Vet and stabilize subtle reference changes  | Large | - |
| Overhaul compiler's incremental system | - | Large |

We'll have to figure out what these levels mean for other teams!

The idea of the levels to help us avoid overcommitting:

* Team should expect to only take a small number of *large* items, quite possibly zero, depending on how much bandwidth they have.
* No individual should champion more than a small number of goals.

### What is a team champion?

A team *champion* is someone who agrees to support the goal owner on an individual basis. Champions should expect to meet with the goal owner on a weekly or biweekly basis.

Champions often help provide design guidance and advise on who within the team the owner ought to talk to for more information. They should also be prepared to field questions from the rest of the team about the goal.

### Can I champion my own goal?

If you are a member of the team in question, you can champion your own goal, sure.

### Why do you call these *Project goals* if some of them originate from outside the Project?

A Project goal is an agreement between...

* an **owner**, who will be doing the work;
* and a **team**, who will be accepting the work.

Owners may or may not be members of a Rust team.

We only accept goals where there are both owner(s) to do the work and team(s) that want it.

The fact that the team(s) agree to the goal is what makes it a Rust Project goal.
