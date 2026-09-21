+++
path = "inside-rust/2026/09/25/leadership-council-update"
title = "Leadership Council update — September 2026"
authors = ["Jakub Beránek"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council! We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2026/07/06/leadership-council-update/

## Accomplishments so far

### Leadership Council September 2026 Elections

Every six months, the Leadership Council performs a [Council representative election][lc-election-announcement-post], to elect new representatives of several top-level teams. This time, the representatives of the Infrastructure, Language, Library, and Moderation teams will be elected.

The election process started in August, and facilitators of the four teams worked with their team and subteam members to elect the new representatives. The elections were finalized a few days ago, and here are the election results:

- Jakub Beránek ([@kobzol][kobzol]) will represent the Infrastructure team
- TODO will represent the Language team
- TODO will represent the Library team
- Oli Scherer ([@oli-obk][oli-obk]) will represent the Moderation team

We welcome them to the Leadership Council!

[issue-321]: https://github.com/rust-lang/leadership-council/issues/321
[lc-election-announcement-post]: https://blog.rust-lang.org/inside-rust/2026/08/18/leadership-council-repr-selection/
[lc-election-results-post]: TODO
[kobzol]: https://github.com/kobzol
[oli-obk]: https://github.com/oli-obk

### Project Director Elections

Same as last year, the Leadership Council has launched [an election][pd-election-post] to select two new Project Directors who will sit on the Rust Foundation Board of Directors ([#324][issue-324]). The nominations for the Director positions are being gathered until September 21, and the actual election should happen in October. Once we know the two elected Project Directors, we will announce them on the Rust Blog.

[issue-324]: https://github.com/rust-lang/leadership-council/issues/324
[pd-election-post]: https://blog.rust-lang.org/inside-rust/2026/08/28/electing-new-project-directors-2026/

### Rust Foundation Maintainers Fund

The recently established [Funding team] has been hard at work. Thanks to the funds from the [Rust Foundation Maintainers Fund][rfmf], they were able to financially support six Rust contributors, four Maintainers in Residence and two Maintenance Grantees (read the [announcement blog post][mir announcement] to learn more).

Since the last update, we decided to allocate an additional $120k to the Funding team ([#328][issue-318]), so that they can hire one additional full-time Maintainer in Residence. This is in addition to the $50k, which we have already allocated previously.

[Funding team]: https://github.com/rust-lang/funding/
[rfmf]: https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/
[mir announcement]: https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/

### Project priorities budget

Since 2024, the Project has managed a *Project Priorities* budget. The council spends funds from this budget on activities that support the work of the Project. Since the last update, the Council approved the following additional allocations.

At the start of July, we led talks about the Project Priorities budget with Rust Project Directors and the Rust Foundation. Based on those discussions, we decided to spend the whole available budget every year, instead of keeping a buffer for the following year, which is what we have been doing so far. This makes tracking of the budget and accounting simpler, and it also has the benefit of giving us more money to spend in 2026! Below you can see on what items we decided to spend it. ([#314][pr-314]).

As mentioned above, $120k was allocated to the [Funding team] for further spending on maintainer support ([#318][issue-318]).

An additional $32k was allocated to the [Mentors team] for the next [Outreachy] round in December 2026. ([#320][issue-320]).

We allocated an additional $32,325 to the travel budget for Rust Project members, in addition to the $100k that we previously allocated for 2026. ([#316][issue-316])

[Outreachy]: https://www.outreachy.org/
[Mentors team]: https://rust-lang.org/governance/teams/launching-pad/#team-mentors
[pr-314]: https://github.com/rust-lang/leadership-council/pull/314
[issue-316]: https://github.com/rust-lang/leadership-council/issues/316
[issue-318]: https://github.com/rust-lang/leadership-council/issues/318
[issue-320]: https://github.com/rust-lang/leadership-council/issues/320

### LLM policy discussions

The Council, together with many other members of the Rust Project, spent a lot of time discussing various policies related to using Large Language Models (LLMs) when interacting with the Rust Project and its repositories. Thanks to the awesome work of [jyn514] and many other people, the main `rust-lang/rust` repository now has an official [LLM policy][llm-policy]. Several other `rust-lang/rust` repositories followed suit and adopted this policy, usually with slight modifications.

That is not the end of the policy discussions though, because we would also like to have a Project-wide policy, and also have someone who will be responsible for updating the policies in the future. After many discussions, we settled on creating a new [LLM policy team][issue-308], to which we delegated the responsibility of coming up with a Project-wide policy and updating our existing LLM policies.

[issue-308]: https://github.com/rust-lang/leadership-council/issues/308
[llm-policy]: https://github.com/rust-lang/rust-forge/pull/1040
[jyn514]: https://github.com/jyn514

### Additional items

- We updated the Council observer policy, which specifies who can join the Council meetings and what they are allowed to do during them. The updated policy reflects more accurately how our meetings work today, and is generally more permissive ([#110][pr-110]).
- We slightly updated the suggestions for the Leadership Council representative selection process. The guidelines now suggest consulting Council candidates with the moderator team prior to them being selected as Council members ([#327][pr-327]).
- Tomáš Šedovič, one of the members of the Program Management team who was previously funded from our Project Priorities budget, has now been hired as a [full-time employee][tomas-employee] by the Rust Foundation! We are very happy about that.
- We finished the annual Council survey to receive feedback from Project members on the Council ([surveys#409](https://github.com/rust-lang/surveys/pull/409)). We are analyzing the results now, and will share them with Project members once the analysis is complete.

[pr-110]: https://github.com/rust-lang/leadership-council/pull/110
[pr-327]: https://github.com/rust-lang/leadership-council/issues/327
[tomas-employee]: https://rustfoundation.org/media/welcoming-rust-program-manager-tomas-sedovic-to-the-rust-foundation-team/

## Following our work

As you can see from the many links above, the work of the Council happens mostly in public on GitHub issues posted in [our repository](https://github.com/rust-lang/leadership-council/issues). The items on our meeting agendas are drawn from these. After discussing an item, we summarize that discussion and any shared [rationales](https://aturon.github.io/tech/2018/05/25/listening-part-1/) on the issue. When we make decisions, we propose on the issue a "final comment period" (FCP), and as with all FCPs in the Project, we're interested in and review any feedback that people have before or during this final comment period.

To follow our work in real time, watch our repository. You can also see meeting summaries posted on Zulip in [`#council > Meeting minutes & summaries`][summaries].

[summaries]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.20minutes.20.26.20summaries/with/561198432

## A word of thanks

For the past couple of years, the Leadership Council update blog posts were always prepared and written by [Eric Huss][ehuss], who has recently left the Rust Leadership Council. We wanted to thank him for all the work he put into them, and all his contributions to Rust. Thank you very much, Eric!

[ehuss]: https://github.com/ehuss

## Meeting minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes]. Links to the minutes since our last update are:

- [July 3, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-07-03.md)
- [July 17, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-07-17.md)
- [July 31, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-07-31.md)
- [August 14, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-08-14.md)
- [August 28, 2026](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2026-08-28.md)

[minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes
