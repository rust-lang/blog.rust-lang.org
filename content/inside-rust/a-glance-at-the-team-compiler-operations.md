+++
path = "inside-rust/9999/12/31/a-glance-at-the-team-compiler-operations"
title = "A glance at the Team Compiler operations"
authors = ["apiraino"]

[extra]
team = "the Compiler Ops team"
team_url = "https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops"
+++

Hello everyone! This blog post will probably be a bit different, I will outline the work of a perhaps less visible structure in the Compiler Team: [The "Operations" team][compiler-ops].

I am Antonio, I've been helping the Rust project for a couple of years and recently I gave a short talk at the RustWeek '25 about my experience. Since then I received encouraging feedback and suggestions to expand a little more about it.

It all started with the "Prioritization" Working Group (which some on you [may remember][wg-prio-call-for-contributors]), this core work was - and is still - to stay on top of the regressions filed in the main [Rust project git repository][rust-git] and do some first triaging to assess priority, bisecting where they originated, pinging relevant people: high priority regressions should be fixed by the next release (following our six-weekly cadence). We want to do our best to release stable compilers and libraries, without unexpected breaking changes. Prioritizing regressions often require technical context and the contribution of the Compiler Team members is essential for this work: thanks to everyone!

Gradually this work grew up organically, there was room to freely explore a relatively young space: improving the organization and workflows of the Compiler team. In hindsight, [Niko][niko] and [Santiago][santi] had a great intuition and planted the seeds. I am the gardener that is looking after these young but robust trees.

[compiler-ops]: https://www.rust-lang.org/governance/teams/compiler#team-compiler-ops
[rust-git]: https://github.com/rust-lang/rust/issues

## Expanding the work and formalizing the Compiler Team Operations ("Ops")

Today this work has expanded quite a bit and articulated in multiple branches. All this is made possible by creating and funding the "Operations" role, which allowed us to take time and focus on these improvements.

- **Follow decisional processes**: the Compiler Team (not dissimilar from other teams) formalized some decisional processes ([MCP], [FCP] and [RFC]) to provide tools for contributors to make proposals. Process itself can only do so much when you everyone is doing 10 things at the same time, so someone needs to ensure that these processes move forward and ask questions if they are blocked.
- **Monitoring pull request reviews**: Compiler Team members have a finite amount of time and are fencing off an *infinite* amount of incoming contributions to review, follow-up and help bring to completions. The Rust project is a community work, so it is fundamental that contributions are not abandoned or forgotten. Staying on top of these contributions, sending reminders and ensuring that they don't get stale is part of the basic machinery that keeps the project rolling.
- **Organization of design/triage meetings**: the Compiler team meets every week to review the most important things, sometimes it is asked for some dedicated time to help design a specific feature. Doing the minutiae around these meetings takes a non-negligible amount of time!
- **Improve tooling and documentation**: the Compiler Team wouldn't go far without some ancillary tooling to help with any of the above points. These tools need maintenance and evolution, like every piece of software. Documentation, goes without saying, is never enough and it's part of our "Welcome flyer" when a contributor shows at the door.

All this sounds like after all there is a lot more than *just* looking at regressions! So it was time to evolve what was a seminal Working Group and build the foundation for a more formal structure in charge of the "Operations", a field that for me encompasses a lot of that is not hacking into the compiler or designing features. The idea of the "Operations" comes from the Language Team (see [team#1091](https://github.com/rust-lang/team/pull/1091)) and I happily "stole" it: having homogeneous structures across the project is a good thing.

The scope of "Operations" expands as the Compiler Team itself expands (almost 60 members and counting!) and all this puts a premium on coordination work. The Rust project (that celebrated the [10th anniversary of the 1.0 release][10-years-of-crabs]) is entering a new phase and the organizational part must keep up. If we squint enough, there is a lot of work ahead.

During the last months we re-organized the Compiler Team into a single entity, closed all the old Working Groups that were not active anymore, unified all the documentation in the same place, the [Forge documentation][forge.r-l.org] website. We are constantly working on small quality of life improvements and automation bits that reduce to contributors.

Every team in the Rust project has their own workflows, so I can only speak for the Compiler Team, but we are implementing more automation around processes. At the RustWeek '25 there were discussions around possible "joint" points for teams, more inter-team communication, shared processes: we are moving in a direction where we can make some "assumptions" about how a team works and where things are. For example every team cranking out code use the same process to handle regressions and backports.

In the future we would like to tackle further common frictions points, such as the first onboarding for new contributors. A recurrent question is "who should I talk to, to learn more about X?" and the answer to this requires work in multiple areas (documentation, website, our Zulip chat, etc.).

[MCP]: https://forge.rust-lang.org/compiler/proposals-and-stabilization.html#how-do-i-submit-an-mcp
[FCP]: https://forge.rust-lang.org/compiler/proposals-and-stabilization.html#targets
[RFC]: https://forge.rust-lang.org/libs/maintaining-std.html#when-is-an-rfc-needed
[10-years-of-crabs]: https://rustweek.org/celebration/
[forge.r-l.org]: https://forge.rust-lang.org/libs/maintaining-std.html#when-is-an-rfc-needed

## Collaboration

The word "team" appears 28 times in this blog post, so I leave the most important thing to the last: nothing would be possible without the great help and initiative of other enthusiastic project members that in the true spirit of FOSS, figure out how to experiment and improve things. Among others, I want to especially acknowledge the work of [Urgau], [Jieyou Xu], [Jack Huey] and [Jakub Beránek], people on whose shoulders I stand, and [Wesley Wiser] and [David Wood] for being the best team leads one could wish for.

## Conclusions

In this blog post I've tried to shed light on some work that is perhaps less visible outside of the Rust project. The Compiler Team Ops is needed, it makes things work better for everyone in many small ways and - looking ahead - will help the whole Rust Project grow sustainably.

[wg-prio-call-for-contributors]: https://blog.rust-lang.org/2020/09/14/wg-prio-call-for-contributors
[niko]: https://smallcultfollowing.com/babysteps/
[santi]: https://santiagopastorino.com
[Jieyou Xu]: https://github.com/jieyouxu
[Urgau]: https://github.com/urgau
[Jack Huey]: https://github.com/jackh726
[Jakub Beránek]: https://github.com/kobzol
[Wesley Wiser]: https://github.com/wesleywiser
[David Wood]: https://github.com/davidtwco
