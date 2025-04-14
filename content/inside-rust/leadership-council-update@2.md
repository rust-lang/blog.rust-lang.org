+++
path = "inside-rust/2024/02/13/leadership-council-update"
title = "February 2024 Leadership Council Update"
authors = ["Leadership Council"]
aliases = ["inside-rust/2024/02/13/leadership-council-update.html"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council!
We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2023/11/13/leadership-council-update.html

## Accomplishments so far

### Project Structure

The Council has agreed to make the [Release team] a subteam of the [Infra team] in [RFC #3533].
[Ryan Levick] has stepped off the Council as part of this transition, and we thank him for the very significant help he has provided.
Ryan has been a part of the Core team since 2021, and was part of the group that designed and created the Leadership Council, and has earned a well deserved break.

[Release team]: https://www.rust-lang.org/governance/teams/infra#Release%20team
[Infra team]: https://www.rust-lang.org/governance/teams/infra
[RFC #3533]: https://rust-lang.github.io/rfcs/3533-combine-infra-and-release-teams.html
[Ryan Levick]: https://github.com/rylev

We created the [Project Structure Committee] to help drive the discussion about the "shape" of the Rust project, and to define the goals and responsibilities of that committee.
There is not much progress to report at this time, as the group is still trying to get a grasp on how to approach it.
In the meantime, [Eric Huss] created a [org visualizer] to visually see the entire scope of the project.

There is an [untracked people] document where we are exploring people in the project who are in an ambiguous state, or not properly tracked in the team database.
Additionally, [Jakub Beránek] has done fantastic work in updating the team database to make sure all repositories are tracked and tied to teams correctly, submitting well over [100 PRs][kobzol-prs], and improving the infrastructure that manages the teams.

[untracked people]: https://hackmd.io/@rust-leadership-council/Bk6ge9Xu6
[Eric Huss]: https://github.com/ehuss
[Project Structure Committee]: https://github.com/rust-lang/leadership-council/blob/main/committees/project-structure.md
[org visualizer]: https://ehuss.github.io/org-chart/org-chart.html
[kobzol-prs]: https://github.com/rust-lang/team/pulls?q=is%3Apr+is%3Aclosed

There is a [new policy][team-policy] for management of the [Rust Team Database], which records the list of all teams, members, repositories, and their permissions.
Additionally, the [team-repo-admins] ([Mark-Simulacrum], [Ryan Levick], and [Jack Huey]) are a group of people responsible for approving changes to the team database under that policy.

[team-repo-admins]: https://github.com/rust-lang/team/blob/master/teams/team-repo-admins.toml
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[Ryan Levick]: https://github.com/rylev
[Jack Huey]: https://github.com/jackh726
[Rust Team Database]: https://github.com/rust-lang/team
[team-policy]: https://github.com/rust-lang/rust-forge/pull/707

### Wrapping up Project Director Elections

[Eric Holk][eholk] created a [retrospective of the Project Director elections][retro].
This gathers the lessons we learned during the process in October and ideas to improve the process in the future.
We have agreed to hold the next Foundation Project Director election in September 2024 for the two seats of Ryan Levick and Mark Rousskov, which we hope will run more smoothly based on this review.

[eholk]: https://github.com/eholk
[retro]: https://github.com/rust-lang/leadership-council/blob/main/reports/project-director-elections/2023-project-director-elections-retrospective.md

### Google Summer of Code

We agreed that the project should explore the possibility of participating in [Google Summer of Code][gsoc], thanks to the efforts of [Jakub Beránek] and [Jack Huey], along with the Foundation agreeing to help with submitting the application.
Proposals and coordination are happening in the [google-summer-of-code repo][gsoc-repo].
More information about this will be shared in the future if our application is approved.

[gsoc]: https://summerofcode.withgoogle.com/
[gsoc-repo]: https://github.com/rust-lang/google-summer-of-code
[Jakub Beránek]: https://github.com/kobzol
[Jack Huey]: https://github.com/jackh726

### Leads Summit

We agreed to have a Leads Summit in London, UK just before RustNation in March.
This is an opportunity for project leads to meet in person, to discuss project-wide goals, and to develop leadership skills.

### Council Representative Appointments

We have agreed that the next Council Representative appointments will happen in March.
We are in the process of finalizing which half of teams will be going through the appointment process, and will follow up soon with more details.

### Additional Items

And a few minor items:

* Approved TWiR to be added to the <https://social.rust-lang.org/> Mastodon service. The infra team is working on getting this finalized.
* The [RustConf Steering Committee RFC][rfc-3549] had various discussions among the Council, but it was ultimately closed by the author to reconsider how this will be approached in the future.
* [issue #22][issue-22]: The Council formally adopted the T-core RFCs ([RFC#2872] [RFC#3259] [RFC#3339]), and has begun work to move those forward.
* [issue #60][issue-60]: The Council approved a policy for handling [private meeting minutes].
* Permissions for the [Rust Forge] are now formally written down ([PR #725][forge-725]), indicating that the infra team is responsible for it, and any Rust team, working group, or project group may post their content on it.
* Thanks to [David Tolnay], the team representation of the Council is now displayed on the [website][lc-gov].

[Rust Forge]: https://forge.rust-lang.org/
[forge-725]: https://github.com/rust-lang/rust-forge/pull/725
[rfc-3549]: https://github.com/rust-lang/rfcs/pull/3549
[issue-22]: https://github.com/rust-lang/leadership-council/issues/22
[issue-60]: https://github.com/rust-lang/leadership-council/issues/60
[RFC#3339]: https://github.com/rust-lang/rfcs/pull/3339
[RFC#3259]: https://github.com/rust-lang/rfcs/pull/3259
[RFC#2872]: https://github.com/rust-lang/rfcs/pull/2872
[private meeting minutes]: https://github.com/rust-lang/leadership-council/blob/main/procedures/synchronous-meetings.md#private-minutes
[lc-gov]: https://www.rust-lang.org/governance/teams/leadership-council
[David Tolnay]: https://github.com/dtolnay

## Ongoing work

There are various efforts underway on projects that have had significant discussions since the last update, but have not concluded with any decisions, yet.
They are:

* Planning around the Foundation's upcoming budget.
* Investigation and discussions around the Rust trademarks, and what if any policy changes we should enact.
* Work on defining and clarifying affiliation limits and the substitute policy, for both the Foundation Project Directors ([PR #61]) and the Council itself ([issue #27]).
  There is a [work in progress][affiliation-limits] exploring this.
* Investigating process for managing external requests for the council via ZenDesk.

[affiliation-limits]: https://hackmd.io/@rust-leadership-council/BJMOxfP5p
[PR #61]: https://github.com/rust-lang/leadership-council/pull/61
[issue #27]: https://github.com/rust-lang/leadership-council/issues/27

## Meeting Minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes].
Links to the minutes since our last update are:

* [November 10, 2023](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2023-11-10.md)
* [December 8, 2023](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2023-12-08.md)
* [January 5, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-01-05.md)
* [January 19, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-01-19.md)

[minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes
