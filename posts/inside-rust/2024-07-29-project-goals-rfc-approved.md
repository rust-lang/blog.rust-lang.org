---
layout: post
title: "Project goals RFC approved"
author: Niko Matsakis
team: Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>
---

I'm happy to announce that [RFC #3672 has been merged](https://github.com/rust-lang/rfcs/pull/3672#issuecomment-2254599176) and so we have our roadmap for 2024h2! This blog post is a brief internal announcement, to be followed by a wider public announcement on the main Rust blog.

The final slate had 26 goals, with 3 designated as flagship goals:

* [Release the Rust 2024 edition](https://rust-lang.github.io/rust-project-goals/2024h2/Rust-2024-Edition.html)
* [Bring the Async Rust experience closer to parity with sync Rust](https://rust-lang.github.io/rust-project-goals/2024h2/async.html)
* [Resolve the biggest blockers to Linux building on stable Rust](https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html)

Each of the goals has an open issue, assigned to the goal owner(s), tracked on the [2024h2 milestone][2024h2]. Take a look, there's a lot of exciting stuff on there!

[2024h2]: https://github.com/rust-lang/rust-project-goals/milestone/2

## Goal status reporting

Triagebot is going to be pinging goal owners on Zulip for updates on a regular basis -- we'll have to tune the threshold. Updates should be placed as comments on the [github tracking issues on the rust-project-goals repository][2024h2]. The issues are only for status updates: discussion should take place on the associated topic on [the **project-goals** Zulip channel](https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals). All comments placed on the tracking issue are forwarded to the channel as well by triagebot.

I also plan to post a public blog post that collects together all the updates ~every 6 weeks, roughly aligned with Rust releases. These blog posts will be assembled by scraping updates from the Github issues.

## Next round of goals

The idea is to repeat the goal program every 6 months. I expect planning for 2025h1 to start in October.
