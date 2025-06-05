+++
path = "inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations"
title = "A glance at compiler team operations"
authors = ["apiraino"]

[extra]
team = "the Compiler Ops team"
team_url = "https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops"
+++

Hello everyone! This blog post will probably be a bit different. I will outline the work of a perhaps less visible structure in the Compiler Team: [the "operations" team][compiler-ops].

I am Antonio. I've been helping the Rust Project for a couple of years, and recently I gave a short talk at RustWeek 2025 about my experience. Since then I received encouragement to expand a little more about it.

It all started with the "prioritization" working group (which some of you [may remember][wg-prio-call-for-contributors]). This core work was &mdash; and is still &mdash; to stay on top of the regressions filed in the main [Rust Project git repository][rust-git] and do initial triage to assess priority, bisect, and ping relevant people. High priority regressions should be fixed by the next release (following our six-week cadence). We want to do our best to release stable compilers and libraries without unexpected breaking changes. Prioritizing regressions often requires technical context, and the continued support of Compiler Team members is essential to this work. Thanks to everyone!

Gradually this work organically grew. There was room to freely explore a relatively young space: improving the organization and workflows of the Compiler Team. In hindsight, [Niko][niko] and [Santiago][santi] had great intuition and planted these seeds. I am the gardener that is looking after these young but robust trees.

[compiler-ops]: https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops
[rust-git]: https://github.com/rust-lang/rust/issues

## Expanding the work and formalizing Compiler Team Operations ("Ops")

Today this work has expanded quite a bit and forked multiple branches. All this is made possible by creating and funding the "operations" role, which allowed us to focus on these improvements.

- **Follow decisional processes**: the Compiler Team (not dissimilar to other teams) formalized some decisional processes ([MCPs], [FCPs], and [RFCs]) to provide tools for contributors to make proposals. Process itself can only do so much when everyone is doing 10 things at the same time, so someone needs to ensure that these processes move forward and ask questions if things are blocked.
- **Monitoring pull request reviews**: Compiler Team members have a finite amount of time and are fending off an *infinite* amount of incoming contributions to review, follow-up on, and help bring to completion. The Rust Project is a community work, so it is fundamental to ensure that contributions are not abandoned or forgotten. Staying on top of these contributions, sending reminders, and ensuring that they don't get stale is part of the basic machinery that keeps the project rolling.
- **Organization of design/triage meetings**: the Compiler Team meets every week to review the most important things. Sometimes it is asked for dedicated time to help design a specific feature. Doing the minutiae around organizing these meetings takes a non-negligible amount of time!
- **Improve tooling and documentation**: the Compiler Team wouldn't go far without some ancillary tooling to help with these tasks. These tools need maintenance and evolution, like every piece of software. Documentation &mdash; it goes without saying &mdash; can always be improved, and it's part of our "welcome flyer" when a contributor shows at the door.

All this sounds like, after all, there is a lot more than *just* looking at regressions! So it was time to evolve what was a seminal working group and build the foundation for a more formal structure in charge of the "operations" of the compiler team &mdash; a field that for me encompasses a lot that is not hacking on the compiler or designing features. The idea of "operations" comes from the Language Team (see [team#1091](https://github.com/rust-lang/team/pull/1091)) and I happily "stole" it. Having homogeneous structures across the Project is a good thing.

The scope of "operations" expands as the Compiler Team itself expands (it now has almost 60 members and counting!) and all this puts a premium on coordination work. The Rust Project (that celebrated its [10th anniversary of the 1.0 release][10-years-of-crabs]) is entering a new phase, and the organizational part must keep up. If we squint enough, there is a lot of work ahead.

During the last many months, we reorganized the Compiler Team into a single entity, closed all the old working groups that were no longer active, and unified all the documentation into the same place (the [Forge][forge.r-l.org]). We are constantly working on small quality of life improvements and automation bits that reduce the burden on contributors.

Every team in the Rust Project has its own workflows, so I can only speak for the Compiler Team, but we are implementing more automation around processes. At RustWeek 2025 there were discussions around possible "joint" points for teams &mdash; more inter-team communication, shared processes, etc. &mdash; we are moving in a direction where we can make some "assumptions" about how a team works and where things are. For example every team cranking out code uses the same process to handle regressions and backports.

In the future we would like to tackle further common frictions points, such as the onboarding of new contributors. A recurring question is "who should I talk to in order to learn more about X?", and the answer to this requires work in multiple areas (documentation, the website, our Zulip instance, etc.).

[MCPs]: https://forge.rust-lang.org/compiler/proposals-and-stabilization.html#how-do-i-submit-an-mcp
[FCPs]: https://forge.rust-lang.org/compiler/proposals-and-stabilization.html#targets
[RFCs]: https://forge.rust-lang.org/libs/maintaining-std.html#when-is-an-rfc-needed
[10-years-of-crabs]: https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/
[forge.r-l.org]: https://forge.rust-lang.org/compiler/

## Collaboration

The word "team" appears many times in this blog post, so I leave the most important thing to the last. Nothing would be possible without the great help and initiative of other enthusiastic Project members that, in the true spirit of F/OSS, figure out how to experiment and improve things. Among others, I want to especially acknowledge the work of [Urgau], [Jieyou Xu], [Jack Huey], and [Jakub Beránek] &mdash; people on whose shoulders I stand &mdash; and [Wesley Wiser] and [David Wood] for being the best team leads one could wish for.

## Conclusion

In this blog post I've tried to shed light on work that is perhaps less visible outside of the Rust Project. The work of Compiler Team Ops is needed. It makes things work better for everyone, and &mdash; looking ahead &mdash; will help the whole Rust Project grow sustainably.

[wg-prio-call-for-contributors]: https://blog.rust-lang.org/2020/09/14/wg-prio-call-for-contributors
[niko]: https://smallcultfollowing.com/babysteps/
[santi]: https://santiagopastorino.com
[Jieyou Xu]: https://github.com/jieyouxu
[Urgau]: https://github.com/urgau
[Jack Huey]: https://github.com/jackh726
[Jakub Beránek]: https://github.com/kobzol
[Wesley Wiser]: https://github.com/wesleywiser
[David Wood]: https://github.com/davidtwco
