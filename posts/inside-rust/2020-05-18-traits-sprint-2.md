---
layout: post
title: "Traits working group 2020 sprint 2 summary"
author: Jack Huey
team: The Traits WG <https://rust-lang.github.io/wg-traits/>
---

It's that time of year again: another traits working group sprint summary. And ohh boy, it was a busy sprint.

If you're unfamiliar with the traits working group, we posted a short summary in the [sprint 1 post]. In short, the overarching goal of the [traits working group] is to create a performant, extensible, and clean implementation of Rust's trait system.

## 2020 sprint 2

While the first sprint of the year somewhat lacked direction and we very much "figured it out while we went", this sprint was much smoother. This was in part because of the tools and procedures that we settled into in sprint 1, such as the [skill tree] or a running [sprint doc] to track progress.

### Credit where credit is due

We had tons of participation from many people, some new and some old. For that, a big thank you ❤:

* [Wilco Kusee]
* [Élie Roudninski]
* [Florian Diebold]
* [Jack Huey]
* [Charles Lew]
* [Niko Matsakis]
* [Nathan Whitaker]
* [Adam Bratschi-Kaye]
* [super-tuple]
* [David Ross]
* [Christofer Nolander]
* [Michael Bryan]
* [Mark McCaskey]
* [Wonwoo Choi]
* [Zahari Dichev]
* [Mikhail Babenko]
* [Mark Drobnak]

That's a lot!

[Wilco Kusee]: https://github.com/detrumi
[Élie Roudninski]: https://github.com/marmeladema
[Charles Lew]: https://github.com/crlf0710
[Niko Matsakis]: https://github.com/nikomatsakis
[Jack Huey]: https://github.com/jackh726
[Florian Diebold]: https://github.com/flodiebold
[Nathan Whitaker]: https://github.com/nathanwhit
[Adam Bratschi-Kaye]: https://github.com/adamrk
[super-tuple]: https://github.com/super-tuple
[David Ross]: https://github.com/daboross
[Christofer Nolander]: https://github.com/nolanderc
[Michael Bryan]: https://github.com/Michael-F-Bryan
[Mark McCaskey]: https://github.com/MarkMcCaskey
[Wonwoo Choi]: https://github.com/tirr-c
[Zahari Dichev]: https://github.com/zaharidichev
[Mikhail Babenko]: https://github.com/Areredify
[Mark Drobnak]: https://github.com/Mcat12


### Rustc integration MVP

As was mentioned in the [sprint 1 post], the previous experimental integration of [Chalk] into rustc was removed earlier this year. That integration was old and not based on the current Chalk codebase. At the end of the last sprint, we began reintegration. The plan was/is to start with a "minimum viable product" (MVP) using the new Chalk framework (for those curious, the new integration uses the `chalk-solve` crate rather than `chalk-engine`). This MVP had a few goals and limitations:
* Rustc types and goals are deeply and eagerly converted to Chalk types
* Lifetimes are sometimes ignored
* No constants and some missing types and traits
* Some hacks to make things work

With that said, the [rustc integration PR] has landed and the new experimental Chalk solver is available under the `-Z chalk` flag. Just as a forewarning: don't use this (yet). It's still very early in its implementation and things *won't* work more often than they *will*. **But** it is a start and it's only going to get better and more complete from here.

### Const in Chalk planning

Before this sprint started we decided that we wanted to plan a design meeting for consts in Chalk. We had that meeting on April 7th. However, we didn't expect to start working on the implementation *this* sprint. But alas, there is already a [PR open to implement consts in Chalk]. This has been helpful to uncover some design decisions within Chalk. We expect that this might be landing fairly soon.

### Moving towards a shared type library for rustc and Chalk

Currently, rustc and Chalk represent types in a slightly different manner. Also, Chalk is missing a few. In the current MVP implementation the conversion between rustc and Chalk types are "deep and eager", which means we do a lot of work to use Chalk as a trait solver. The eventual goal is to make a shared type library. There was a compiler team meeting to mostly "green-light" this from the rustc side of things. Most of the work so far, though, has been on Chalk to adding missing builtin types and traits.

### Basic support for `impl Trait` in Chalk

We landed initial support for `impl Trait` during this sprint. It doesn't yet support some features, such as generics. But there is an open PR to extend the functionality.

### Progress towards removing the leak check in rustc

In the rustc trait solver, there is currently a special check done in regards to lifetimes call the "leak check". Without going into the techinical details, there are some design flaws with this approach and it being there blocks features such lazy normalization (which is required for features such const generics and GATs). However, removing the leak check completely has some backward-compatiblity concerns. But [some progress] was made.

### Adding a recursive solver to Chalk

When Chalk was first written, it used a stateful recursive solver. It was then changed to use a prolog-solving approach called SLG. SLG uses a more stateless approach where answers to subgoals can be reused.

While SLG is more complete, there are some design tradeoffs. One example in particular is related to how we handle associated types. It's completely possible that we can and will resolve these design problems in the future. In the meantime, however, we ressurected the old recursive solver. [Rust-analyzer] has switched to using it and results have been positive.

For now, we'll continue to work on resolving design problems with the SLG solver. Eventually, we expect that we'll evaluate the two and pick one to stick with.

### Creating reproducable Chalk test files

Oftentimes we'll get a bug report where Chalk doesn't report the result one would expect. And as anyone who has maintained a piece of software knows, getting a minimal reproduction is difficult. What makes it even more difficult is that the goals and programs that Chalk understands are a "lowered" form of actual Rust code, which means not only do we have to make a minimal *Rust* example, but also a minimal *Chalk* example.

In order to help make this process easier, we have started to make a logging shim for Chalk to generate programs that Chalk can run and reproduce the bug. Moreso, it should be able to be used seemlessly, regardless of the user of Chalk, whether it be rustc, rust-analyzer, or anything else.

### Documentation in the Chalk book

We are committed to making the work that we do accessible to anyone interested, whether it be for those working on Chalk, on rustc, or just using Rust. As part of this effort, we previous started publishing a Chalk [book]. During this sprint, we have added a little bit more documentation. Additionally, Chalk-related documentation that used to be in the [rustc-dev-guide] has now been moved into the Chalk book.

## 2020 sprint 3

We haven't yet decided our goals for the next sprint. We are going to be doing our sprint planning in our weekly meeting on Tuesday, the 19th at 4:00 PM EST on [zulip]. We then plan to officially start the sprint the week after. If you're interested in helping out or joining the discussion, feel free to stop by!

[sprint 1 post]: https://blog.rust-lang.org/inside-rust/2020/03/28/traits-sprint-1.html
[traits working group]: https://rust-lang.github.io/wg-traits/
[skill tree]: https://rust-lang.github.io/wg-traits/roadmap/skill-tree.html
[sprint doc]: https://github.com/rust-lang/wg-traits/blob/master/sprints/2020-2.md
[Chalk]: https://github.com/rust-lang/chalk
[rustc integration PR]: https://github.com/rust-lang/rust/pull/69406
[PR open to implement consts in Chalk]: https://github.com/rust-lang/chalk/pull/393
[some progress]: https://github.com/rust-lang/rust/pull/70950
[Rust-analyzer]: https://github.com/rust-analyzer/rust-analyzer
[book]: http://rust-lang.github.io/chalk/book/
[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/144729-wg-traits
