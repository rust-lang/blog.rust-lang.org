---
layout: post
title: "Goodbye, docs team"
author: Steve Klabnik
description: "The docs team is winding down"
team: the core team <https://www.rust-lang.org/governance/teams/core>
---

I'll cut right to the chase: the docs team no longer exists.

Back in [August of 2016](https://github.com/rust-lang/rfcs/pull/1683), we
formed a team to work on documenting Rust. Here's the original description
and motivation:

> The Rust documentation team will be responsible for all of the things
> listed above. Specifically, they will pertain to these areas of the Rust
> project:
> 
> * The standard library documentation
> * The book and other long-form docs
> * Cargo's documentation
> * The Error Index
> 
> Furthermore, the documentation team will be available to help with
> ecosystem documentation, in a few ways. Firstly, in an advisory capacity:
> helping people who want better documentation for their crates to understand
> how to accomplish that goal. Furthermore, monitoring the overall ecosystem
> documentation, and identifying places where we could contribute and make a
> large impact for all Rustaceans. If the Rust project itself has wonderful
> docs, but the ecosystem has terrible docs, then people will still be
> frustrated with Rust's documentation situation, especially given our
> anti-batteries-included attitude. To be clear, this does not mean owning the
> ecosystem docs, but rather working to contribute in more ways than just the
> Rust project itself.
> 
> We will coordinate in the #rust-docs IRC room, and have regular meetings,
> as the team sees fit. Regular meetings will be important to coordinate
> broader goals; and participation will be important for team members. We hold
> meetings weekly.

At the time, all of this was sorely needed. There weren't as many people working
on Rust, and there wasn't that much documentation.

But documentation is a funny thing. It's really a cross-cutting concern. One
team of folks writing docs for tons of other teams of folks doesn't really
*work*, long-term. In the short term, it was an absolutely neccesary and good
strategy. Today, it doesn't make as much sense. Let's look again at those original
resources:

* The standard library's documentation is pretty much filled out, and when new APIs
  are added, the libs team writes some initial docs.
* The book is maintained by Steve and Carol.
* Cargo's documentation is the responsibility of the Cargo team (and the docs
  team never really helped here. I always wanted to, but years later, it just
  hasn't worked out.)
* The error index describes compiler errors, and so that's the compiler team's
  job.

We've also added way more stuff:

* Rust by Example
* The `rustc` book and `rustc` guide
* The reference
* The nomicon

The list goes on and on. And all this time, the membership of the team didn't
really grow; I tried several times to get folks involved, but most people
just plain don't like writing docs. At this point, the only person really
writing docs is me, and I haven't had a ton of time lately either. So we
haven't had a docs team meeting since August of 2018. There also aren't
really docs RFCs these days. As such, this blog post isn't really announcing
the end of the docs team as much as it is describing what is already true
today.

I will still be doing my work on core, and the book. And I plan on submitting
some more docs PRs in the future.

I would like to thank everyone who's been on the team in the past, and
everyone who's submitted documentation PRs over the years. A lot of people
really love Rust's documentation, and that wouldn't have been possible
without all of you.
