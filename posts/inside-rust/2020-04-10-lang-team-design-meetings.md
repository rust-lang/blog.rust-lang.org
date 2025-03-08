+++
layout = "post"
title = "April Lang Team Design Meetings"
author = "Josh Triplett"
description = "Lang Team Design Meetings scheduled for April"
team = "the language team <https://www.rust-lang.org/governance/teams/lang>"
+++

We've scheduled our **language team design meetings** for April. We have plans
for three meetings:

# try blocks, Try traits, functions that try, oh my!

(Update: tentatively moved to May 4.)

Clear the path to stabilizing `try` blocks and the `Try` trait, and identify
some next steps for function-level try.

Agenda:

* [Resolving `Ok`-wrapping for `try`
  blocks](https://github.com/rust-lang/rust/issues/70941)
* Discuss revisions to the `Try` trait required for stabilization.
* Discuss syntax for early exit from a `try` with an error (`Err(e)?`): `fail`,
  `throw`, `raise`, `yeet`, etc.
* If we have time, talk some about possibilities for function-level `try`.

# April 20 -- Edition planning

Plan the 2021 edition, with a checklist of language features we will need to
land in 2020 if we want to ship them in Rust 2021. We hope to emerge from this
meeting with a checklist to guide our efforts, which we can adapt through the
process.

# April 27 -- Type aliases and traits enforcement

Long-standing issue: `type Foo<T: Trait> = ...` doesn't enforce `T: Trait`.

Goal:

* Discuss the situation with type aliases and decide what actions we may want
  to take.
* Or, if we don't reach a decision, decide what measurements we might need to
  reach one.
* Ideally, decide about [estebank's
  PR](https://github.com/rust-lang/rust/pull/69741).

## About the language team design meetings

The idea of the design meeting is that it's a time for us to have in-depth
discussions on some particular topic. This might be a burning problem that
we've discovered, an update on some existing design work, or a forward looking
proposal.

The meetings are open for anyone to listen in and attend. They are typically
also recorded and posted online, along with minutes, after the fact. They
generally take place on Mondays at noon Eastern time, 9am Pacific time -- but
for the precise scheduling you should check the [lang team calendar]. Scheduled
meetings are subject to change and cancelation. In that case, the calendar
events will be updated.

[lang team calendar]: https://github.com/rust-lang/lang-team/#meeting-calendar
