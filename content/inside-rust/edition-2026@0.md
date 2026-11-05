+++
path = "inside-rust/2026/10/27/edition-2026"
title = "Edition 2026"
authors = ["TC", "Eric Holk", "Pete LeVasseur", "Mark Rousskov"]

[extra]
team = "the Rust Edition Team"
team_url = "https://rust-lang.org/governance/teams/launching-pad/#team-edition"
+++

One of our core values, in Rust, is *stability without stagnation*. Editions are part of how we deliver on that. By letting people opt in to changes that would otherwise be breaking, we're able to ship things our users want that, without editions, would be too difficult (or impossible) or would require making unpleasant design compromises.

Since Rust 1.0, which we call Rust 2015, we've shipped three editions: Rust 2018, Rust 2021, and Rust 2024.

The next edition will be Rust 2026. We expect it to stabilize on nightly later this year and ship on stable in early 2027.

Unlike prior edition announcements, we're *not* soliciting items for this edition. We're shipping what's already ready. Why?

It's what we foreshadowed. People have long wanted editions to work more like regular releases, where items go in when they're ready and, periodically, a release is cut. Correspondingly, we've encouraged edition items to be put forward at any time, and, just after Rust 2024 shipped, we added the nightly `future` edition for this purpose.

We have [one item] that just missed Rust 2024 and has been waiting patiently on this train. We'll be shipping that change, a new error, in this edition. (Often, edition items are just new errors that clean up the language and allow later features to be shipped.)

We expect that some people will suggest this edition is "too small" or "not worth it". On the Edition team, we've considered and discussed that carefully, and we disagree — a view we'll formalize in our FCP for stabilizing Rust 2026. Here's our thinking:

Editions are a tool teams use to improve Rust and to better serve our users. Our job, on the Edition team, is to make that tool work really well. And for any tool to work well, you have to be sure that it will be there when you need it.

We want to make it so that nobody ever says, "we could do this, but it would need an edition (so let's not do it)."

When people say that, the edition tool has failed them. We're working to correct that failure, and in shipping Rust 2026, we're taking the first step toward this.

You may be thinking, "wait, I had items I wanted to propose; don't let the train leave without me!" Don't worry. Rust 2026 is the *beginning*. We're serious about getting edition items out when they're ready. We've been hard at work planning how best to do that. And we've been discussing these plans with others in the Project, listening, and collecting feedback. We're continuing that work and expect to have more to say soon.

Don't wait, though, to push forward any edition items you have. Part of the reason we're shipping Rust 2026 is to demonstrate that we *will* ship an edition with what is ready to go. Make your items ready to go, and we'll put them out with the next edition. If you have questions about how to get your items ready, [reach out]. We're here to help with that.

[one item]: https://github.com/rust-lang/rust/issues/134986
[reach out]: https://rust-lang.zulipchat.com/#narrow/channel/268952-edition
