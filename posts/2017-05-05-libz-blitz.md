---
layout: post
title: "The Rust Libz Blitz"
author: Brian Anderson, David Tolnay, and Aaron Turon
description: "Improving the quality and maturity of Rust's core ecosystem"
---

To help bring our [2017 vision for Rust] to fruition, the Rust subteams are
launching initiatives targeted at specific roadmap goals. **This post covers the
library team's major initiative: raising a solid core of the Rust crate
ecosystem to a consistent level of completeness and quality.** This work is
guided in part by what we learned in producing the Rust standard library, and
will result in a "cookbook" chock full of ready-made examples for common tasks,
drawing from a careful selection of Rust crates.

(Also, ICYMI: please participate in the [State of Rust survey]; even if you don't use Rust, we'd love to hear from you!)

[State of Rust survey]: https://blog.rust-lang.org/2017/05/03/survey.html

[2017 vision for Rust]: https://blog.rust-lang.org/2017/02/06/roadmap.html

## Batteries included?

**You can't be productive in a language without libraries that are easy to find
and use.** Of course, the easiest library to find is the standard library, and
many languages have taken a "batteries included" approach that includes APIs for
a broad set of tasks directly in the standard library. There's a lot of upside
to doing so: it means there's just one place to look for common tasks, with APIs
that (hopefully!) cohere across tasks, compatibility across the ecosystem, and a
promise of library maintenance. What's not to like?

There's a countervailing mindset which, in its harshest terms, says "the
standard library is where code goes to die" (a complaint sometimes heard about
batteries-included standard libraries). Because the standard library is coupled
to the language itself, it is released at the same frequency as the language is,
and making breaking changes means breaking the whole language. For these reasons
and others, standard libraries tend to evolve conservatively.

As usual in the Rust world, our goal is to have our cake and eat it too. **Can
we provide something like a batteries-included experience, without ossifying
that code in the standard library?** This is a particularly vital question for
Rust because, as a community, we are still rapidly exploring and iterating on
best practices for ownership-based API design.

The key ingredient to our approach is [Cargo], the package manager that shipped
with Rust 1.0 and has been improving ever since. Cargo provides a unified
dependency and workflow story across the entire Rust ecosystem, and makes adding
dependencies a painless and relatively low-risk proposition. The ability to
share code with ease is a powerful change to the character of, and audience for,
traditional systems programming tasks.

[Cargo]: https://blog.rust-lang.org/2016/05/05/cargo-pillars.html

Because of Cargo, we can "include batteries" without literally putting them into
the standard library; pulling in other crates is nearly as easy as using the
standard library (and will likely get even easier this year). But to capitalize
on this ability to the fullest, we need to have excellent libraries. **Our
mission for 2017 is to ensure that crates are available for a wide swath of
common tasks and that they are *discoverable*, *cohesive*, *featureful*, and
*well-documented*.**

## The Rust Libz Blitz

Fortunately, while the standard library has stayed small and focused, the
crates.io ecosystem has been growing at breakneck pace. So the challenge here
isn't greenfield library work. It's more a question of: **how can the Rust
community work in a focused way to polish, consolidate, and surface a core set
of libraries for essential tasks?** Our answer is the Rust Libz Blitz.

The basic idea is to:

- Collectively review a few crates at a time in a shared forum. The review draws
  in part from a set of [API guidelines].
- Every two weeks, hold a library team meeting focused on open questions around
  one of these crates, with the author in attendance.
- Push this feedback onto a tracking issue for the crate, and point the Rust
  community there to help shoulder the burden of making improvements.
- Write up entries for each crate in a new Rust Cookbook, which will make it
  easy to discover the crate and jump into using it.

The Rust library team has been quietly engaging in this process already for the
past couple of months, to sort out the kinks. But now, we want to open it up to
the whole Rust community. In the rest of the post, we'll dive a bit more deeply
into the mechanics and goals of this process.

### Rust standards of quality

In all the work on the standard library, we have gained a sense of what makes
Rust APIs great: principles like "make performance characteristics clear" and
"use ownership to encapsulate invariants"; consistent naming conventions like
`mut`, `ref`, `as`, `into`, that let users intuit what they don't know
from what they do; small details of presentation that add up, like consistently
documenting possible error cases in a dedicated "Errors" section; and so many
more factors to consider when publishing Rust crates.

In 2017 we plan to apply the principles behind the design of `std` to an
ever-widening foundation of critical Rust libraries. Along the way we will
document what we learn about Rust library design.

The product of this process will be a mature core of libraries together with a
set of [API guidelines] that Rust authors can follow to gain insight into the
design of Rust and level up crates of their own interest. Today those guidelines
are in a very early state, but give you a sense of where we are headed with this
effort. Take a look and file issues where things are unclear or missing!

[API guidelines]: https://github.com/brson/rust-api-guidelines

### Rust standards of documentation

We recently ran a small survey to see what Rust programmers care most about when
evaluating a crate. The [most common criterion], by far, was documentation.  So
a big part of the Libz Blitz process will be evaluating and improving the
documentation of the targeted crates, which will supplement ongoing efforts to
[rewrite the Rust book] and provide comprehensive [examples] for the standard
library. The API guidelines contain a section dedicated to great documentation,
and we aim for a consistent, easy to navigate docs experience across all of the
core crates.

[most common criterion]: https://github.com/rust-lang/rfcs/blob/master/text/1824-crates.io-default-ranking.md#factors-considered-when-ranking-crates
[rewrite the Rust book]: https://github.com/rust-lang/book/
[examples]: https://github.com/rust-lang/rust/issues/29329

Beyond documentation within the crates themselves (which tends to be more of the
"reference" flavor), we are also starting a [Rust Cookbook], which will be
organized around problems you want to solve, and provide quick, concrete example
code you can drop directly into your project, drawing from a variety of crates
(including the standard library). Examples like these are one of the most useful
tools for quickly getting productive with a library, and collecting them all in
one place, organized by problem, will help get us to a "batteries included"
experience when working with Rust.

Part of the work done in the Libz Blitz for each crate will be identifying the
key problem statements the crate is trying to address, and adding strong
examples for each to the cookbook.

[Rust Cookbook]: https://github.com/rust-lang-nursery/rust-cookbook

### Rust standards of discoverability

Perhaps the biggest downside to having a small standard library is the problem
of discoverability: when a Rust programmer needs to take on a task not covered
by the standard library, how do they figure out where to look?

The cookbook mentioned above will undoubtedly become a major part of our
answer. But at the same time, we want to ensure that there's space for new
crates to emerge, and that people can easily find crates beyond the most common
problem areas. To that end, we're attacking discoverability head on, by rolling
out badges for various quality indicators on crates.io, and greatly improving
the default ranking of results (a design debated vigorously through [an
RFC]). There's plenty of room for more work here, so if you have ideas about
further improving discoverability on crates.io, [please start a thread]!

[an RFC]: https://github.com/rust-lang/rfcs/pull/1824
[please start a thread]: https://internals.rust-lang.org/

### Rust standards of community

We have some idea of what goes into publishing exceptional Rust crates, but the
Rust library team is not the sole authority of Rust API design—the Rust crate
ecosystem is created by all of us together and there are many lessons yet to
learn. In recognition of that, **the library team is architecting our efforts to
be as welcoming and inclusive as we can**.

We will kick off forum discussions for evaluating existing crates that we
believe are vital to set on a path toward stability, and which mostly need
polish-level work to get there. These evaluations will be collaborative and
intended for public commentary. They are designed to identify any issues
separating the crate from "1.0" status (in terms of quality and API stability,
as well as literal version number). Issues may include poor adherence to Rust
API design principles, substantial missing functionality, planned API refactors,
incomplete documentation, or any number of unique circumstances.

We'll have a small handful of such evaluations going on in parallel. Every two
weeks, the library team will take up one of the evaluations for discussion in
our regular video conference, which will be recorded and made available on [Air
Mozilla] and the [Rust YouTube channel]. One goal of these meetings will be to
plan out a roadmap for stabilizing the crate in question. The other goal will be
to identify any lessons to be learned from the crate and distill those into
broadly applicable API guidelines.

[Air Mozilla]: https://air.mozilla.org/
[Rust YouTube channel]: https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA

We will have a rotating band of guests (including the crate authors) at the
video conferences by invitation, in order to strengthen the bonds between the
Rust team and the authors of the Rust ecosystem, and to foster shared values
among the same.

Based on the evaluations and library team review, we will file issues that we
believe are important to resolve before producing a stable release of the
crate. **We are counting on many of these issues being amenable to casual
contributions**.

Here are the issues that arose from the very simple [`byteorder`]
crate (all resolved now):

- [Add a supertrait to hide trait details](https://github.com/BurntSushi/byteorder/issues/69)
- [`ByteOrder::default` should `panic!` not `unreachable!`](https://github.com/BurntSushi/byteorder/issues/68)
- [Put panic and error docs in "Panics" and "Errors" sections](https://github.com/BurntSushi/byteorder/issues/72)
- [Make sure there are enough examples](https://github.com/BurntSushi/byteorder/issues/75)
- [Add CI badges to Cargo.toml](https://github.com/BurntSushi/byteorder/issues/74)
- [Add "categories" to Cargo.toml](https://github.com/BurntSushi/byteorder/issues/73)
- [Add `#[doc(html_root_url)]`](https://github.com/BurntSushi/byteorder/issues/77)

Beyond `byteorder` we've already worked this process for several other simple
crates to get a feel for it: [`bitflags`], [`tempdir`], [`flate2`], and
[`lazy_static`]. Even these, the most basic of crates, have some work left to do
but you can expect them to have "1.0" releases soon. As the crates under
evaluation grow in scope, presumably the tasks that arise will grow as well.

[`byteorder`]: https://github.com/BurntSushi/byteorder/issues/76
[`bitflags`]: https://github.com/rust-lang-nursery/bitflags/issues/80
[`tempdir`]: https://github.com/rust-lang-nursery/tempdir/issues/28
[`flate2`]: https://github.com/alexcrichton/flate2-rs/issues/89
[`lazy_static`]: https://github.com/rust-lang-nursery/lazy-static.rs/issues/70


### What crates are we going to focus on?

Rust has some amazing libraries in the works: things like [`tokio`]
which is a really fast asynchronous I/O framework; like [`diesel`], a
really fast library for interacting with database tables; and like
[`rocket`], a really fast web framework.

[`tokio`]: https://tokio.rs/
[`diesel`]: https://diesel.rs/
[`rocket`]: https://rocket.rs/

We are not going to be touching them.

These high-profile libraries are built on **dozens of smaller crates that
perform crucial functions across the ecosystem**: random number generation,
memory mapping, compression, and so much more. We need to polish off the lower
layers of the Rust stack so that those high-level libraries have a stable
foundation to build on.

Focusing on these lower layers is not only a practical consideration but a
technical one—a crate should not be stable until its public dependencies are
stable. Consider what it would mean for a hypothetical [`diesel`] 1.0 to export
a function returning a type from a still-rapidly-developed [`uuid`] 0.5. Before
long it would be incompatible with other libraries using newer versions of
`uuid`.

[`uuid`]: https://doc.rust-lang.org/uuid/uuid/index.html

Furthermore, many of these high-level libraries are very much undergoing their
own rapid development, with their own strong leadership. We don't want to impose
on those crates' authors' abilities to experiment with their designs by
pressuring them to declare crates stable before they are ready. Those libraries
will mature in time.

But there are many foundational libraries that have reached relative stability,
and are in some cases functionally complete and being widely used in
production. Some need little work to put them across the finish line. Some, like
the [`rand`] crate, are widely used but known to have unsatisfactory designs. We
want to finally put those crates to bed.

[`rand`]: https://doc.rust-lang.org/rand/rand/index.html

We are currently drawing this list of foundational crates by combining crates.io
dependency data, various existing curated crate listings, and good old gut
feeling. The exact list is definitely up for debate, and we hope that the
broader community will also apply this process completely independently to
crates the libs team won't have time to discuss.

### How can I help?

We're glad you asked! Creating a solid core of libraries for a young
language is not the work of a single person or team—it is the work of
the entire community. The central point of coordination is a [thread
on the Rust internals forum]. Read that thread for all the dirty
details about what we're doing, and for ongoing opportunities to get
involved.

[thread on the Rust internals forum]: https://internals.rust-lang.org/t/rust-libs-blitz/5184

Roles that need your help:

- **Crate lead**. Each crate needs a volunteer to lead the evaluation
  effort. That entails starting up a thread, breaking up the evaluation work
  into small work items that can be taken on by others in the community, keeping
  the discussion moving in productive directions, making sure the evaluation is
  completed on time, presenting the results at the libs team meeting, and,
  finally, filing discrete, actionable issues on everything raised, and funneling
  them to TWiR.

  *Anyone can be a crate lead, but it's a substantial commitment and is largely
  about organization, communication, and consensus, and requires presenting to
  the libs team in a video meeting.*

- **Crate evaluator**. This is the preliminary work of comparing a crate to the
  API guidelines, identifying deficiencies, and raising observations about API
  design that may impact the guidelines. It will often require working with the
  crate author to understand a crate's known shortcomings.

  Each evaluation will be published to the coordination thread, providing an
  opportunity for general feedback. Open feedback allows for a wide range of
  voices to be heard and is an important check against design myopia.

  *This effort is collaborative and everyone is welcome to participate in this
  way at any time. The crate lead will help create opportunities to jump in.*

- **Documentation slinger**. A lot of the work involved in getting a crate up to
  full quality is improving documentation, including writing up cookbook entries.
  There will be lots of opportunities for this kind of high value work.

  *Everyone is welcome to participate in this way at any time. The
   crate lead will help create opportunities to jump in.*

- **Library hacker**. Somebody must do the programming work of resolving the
  issues on the roadmap to stability for each crate. We expect to produce many
  discrete work items and that many of them will be accessible to inexperienced
  Rust programmers.

  *Everyone is welcome to participate in this way at any time. The crate lead
  will help create opportunities to jump in.*

- **Library designer**. There remain some important libraries that are only
  lightly maintained but are known to require significant design work (again
  thinking of [`rand`] especially). These will not improve without a dedicated
  and skilled author to drive them forward. We are going to be looking out for
  authors with the design skills necessary to fix the problems, and the social
  skills necessary to navigate the process.

  *We will occasionally make pleas for help on specific design matters as they
  arise.*

- **Libs team guest**. The library team will spend one of their video meetings
  reviewing each crate evaluation. We hope that the crate authors will be
  interested in joining us, sharing their unique expertise, and getting to know
  the libs team. This kind of interaction creates strong bonds in a way that
  communicating through text does not. We hope this will foster shared values
  across the ecosystem, and pave the way for expanding the libs team more
  formally.

  *This will be by invitation and focused on authors with an existing reputation
  for high quality work and productive collaboration.*

- **Crate author**. The libs team has some specific functionality and crates it
  wants to focus on this year. Outside of that, we are hopeful that the process
  and guidelines we develop will be widely useful and that crate authors will
  independently seek to evaluate and improve their own crates in similar ways.

## The TL;DR

Here's the plan:

- We will directly improve the most important crates you use every
  day.
- We will provide crate authors the guidance they need to do the same,
  in the form of **[API guidelines]**.
- We will create an endless stream of accessible contribution
  opportunities that directly contribute to key Rust strategic goals.
- We will produce cohesive documentation on how to use Rust's most
  foundational crates, in the form of a **[crate cookbook]**.
- We will launch a self-sustaining process of library improvement that
  can by applied consistently across the entire ecosystem.

[crate cookbook]: https://rust-lang-nursery.github.io/rust-cookbook/

Thank you to everyone who has contributed thus far, including Alisha
Aneja, Andrew Gallant, Brad Anderson, Charles Chamberlain, Dan
Burkert, David Harris, Jan-Erik Rediger, Peter Atashian, Roman Frołow,
Sean McArthur, Simon Sapin, Stephan Buys, Steven Fackler, Trent Spice.

