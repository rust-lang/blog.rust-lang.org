+++
path = "inside-rust/2024/05/07/announcing-project-goals"
title = "Rust Project Goals Submission Period"
authors = ["Niko Matsakis and Josh Triplett"]
aliases = ["inside-rust/2024/05/07/announcing-project-goals.html"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

We're happy to announce the start of an experimental roadmapping effort dubbed **Rust project goals**. As described in [RFC 3614][], the plan is to establish a slate of **project goals** for the second half of 2024 (2024H2). **We need your help!** We are currently seeking ideas for project goals, particularly those that have motivated owners who have time and resources to drive the goal to completion. [If you'd like to propose a goal, read more about the process here!][propose]

## Goals aren’t everything, but they are our priorities

It's important to emphasize that **goals are not an exhaustive list of everything we will do**. Rather, they are the "big rocks", the top priority items that we most want to focus on and complete.

## Goals represent a dual commitment

To be accepted, a goal needs an [owner][], the individual (or small set of individuals) who has agreed to do the work needed to move the goal to completion. The goal itself must then be **accepted by one or more Rust teams**, who are responsible for supporting that owner with reviews, feedback, and prompt decision-making.

Eventually, we expect project teams to offer a "menu" of available resources, such as reviewer bandwidth, or design/review meeting slots.

A project goal allows the owner to get advance approval that the relevant Rust teams care about the problem enough to commit resources (e.g. meeting time and review bandwidth), which in turn can allow the owner to commit resources (e.g. design and development time) towards solving the problem.

## Goals describe problems, not solutions

Goals focus on the problem that we are planning to solve, not on the specific solution that we will use to solve it. When teams accept a goal, they are saying that the problem is important and they are willing to put in work helping the owner to solve it. Sometimes the owner will have a pretty clear idea how they want to proceed, but often they won't, and that's ok.

A solution sketch for a project goal might include experimentation to determine the right path for a solution, and any solution people have in mind might completely change as the goal progresses.

## Owners shape the proposal, teams accept it

Being an [owner][] is a key role, and one that we have not previously recognized in a formal way within Rust.

Being an owner is a commitment, but it’s also a privilege. The owner is expected to serve as the overall expert in the space. They will engage with stakeholders, and accept and coordinate input on the design.

In exchange for committing energy towards the goal, owners are entrusted with designing and proposing solutions. Ultimately, Rust Project teams will review those proposed solutions and approve or reject them, but the expectation is that teams will provide feedback or additional requirements, rather than taking over the design themselves.

## Goal sizes

> It’s difficult to make predictions, especially about the future.

Project goals will typically describe what the shiny future Rust will look like with the goal accomplished. However, humans are notoriously bad at planning and estimating, and in addition, resource commitments may be time-bounded and may require re-evaluation on a regular cadence.

Thus, project goals should focus primarily on a medium-term goal that can be accomplished within the year, and realistic milestones to be accomplished in that timeframe. Some project goals may be entirely completed within such a timeframe. Other project goals may set out incremental milestones (e.g. shipping an RFC), or even just experiments to determine feasibility.

## Goals without owners

In order for a team to accept a goal and commit resources towards it, it needs an owner. However, in some cases project teams are aware of long-standing problems they'd like solutions to, but they don't have a specific owner identified who has committed to work on those problems.

As an experiment, we're also going to set out a few [provisionally accepted][] ["goals without owners"][gwo], which teams have tentatively reviewed and said they'd *like* to commit to if an owner steps up. We're doing this both to give some examples of project goals, and to invite solutions to these long-standing problems. If this works out well, we'll likely formalize a clearer process for preapproving this kind of goal-in-need-of-owner.

If you want to draft a goal like this without an owner, please check with us and with the prospective project team.

Conversely, if you'd like to become the owner of one of these [goals in need of owners][gwo], please get in touch with someone from the teams listed on the goal.

## The goal selection process

In general, teams can accept a new project goal at any time, as long as they have the resources to commit to it. However, we'd also like to use project goals to form roadmaps. In addition, we want to make it as easy as possible for teams to evaluate what resources they're committing and whether all those commitments are compatible.

Thus, for this first experiment, we primarily aim to select a set of goals for the second half of 2024 (2024H2). In the future, we'll try to find the right balance between accepting new goals at any time and having a regular cycle of roadmaps and work.

## Getting started

If you'd like to propose a goal, take a look at the [documentation for proposing a goal][propose].

If you'd like to become the owner of a [goal in need of an owner][gwo], please get in touch with us.

If you'd like somewhere to discuss project goals, [join us on Zulip][Zulip].

[RFC 3614]: https://github.com/rust-lang/rfcs/pull/3614

[propose]: https://rust-lang.github.io/rust-project-goals/how_to/propose_a_goal.html

[gwo]: https://rust-lang.github.io/rust-project-goals/2024h2/slate.html#provisional-goals-in-need-of-owners

[Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals-2024h2

[owner]: https://rust-lang.github.io/rust-project-goals/about/owners.html

[provisionally accepted]: https://rust-lang.github.io/rust-project-goals/about/provisional_goals.html
