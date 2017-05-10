---
layout: post
title: "Two years of Rust"
author: Carol (Nichols || Goulding)
description: "Rust, two years after 1.0"
---

It's been two years since Rust 1.0 was released. Happy second birthday, Rust!

![Group picture from RustFest Berlin][group-pic]{:class="center"}

*Rustaceans at RustFest Berlin, September 2016. Picture by Fiona Castiñeira*

[group-pic]: /images/2017-05-Second-Birthday/rustfest-berlin.jpeg

Rust, a systems programming language focused on safety, speed, and concurrency,
has now achieved stability without stagnation by maintaining backwards
compatibility with version 1.0 for two years despite making many improvements.
Conveniently, Rust's birthday is a bit under halfway through 2017, which makes
this a great time to reflect not only on the progress in the last year but also
on the progress of our [2017 Roadmap] goals.

[2017 Roadmap]: https://blog.rust-lang.org/2017/02/06/roadmap.html

After reading this post, if you'd like to give us your feedback on how we're
doing and where Rust should focus next, please fill out our [2017 State of Rust
survey]!

[2017 State of Rust survey]: https://blog.rust-lang.org/2017/05/03/survey.html

But first, let's do the numbers!

## Rust in numbers

A lot has happened since [Rust's first birthday]:

<!--
Will rerun these just before publishing:

commits:
`git log --since=2016-05-15 --until=2017-05-15 --oneline | wc`

contributors:
`git log --since=2016-05-15 --until=2017-05-15 --oneline --format="%aN" | sort | uniq | wc`

rfcs:
`git ls-tree a4a22d7c5dd71724bb2cd0fe2db5026338d0b270 text/ | wc` => there were 255 RFCs a year ago
`git ls-tree HEAD text/ | wc` => there are 310 RFCs now
310 - 255 =  55

crates:
`select count(*) from crates where date(created_at) >= '2016-05-15';`

friends:
https://github.com/rust-lang/rust-www/compare/bb7f26b...master#diff-ee76f5da00d603d5e34d053a3dc63395
-->

- 10,583 [commits] by 655 contributors added to the core repository;
- 55 [RFCs] merged;
- 9 major releases and 2 patch releases shipped;
- 4,303 new [crates] published;
- 284 library stabilizations;
- 10 languages [rust-lang.org] has been translated into;
- 48 new companies [running Rust in production][friends];
- 2 new teams (Docs and Style);
- 24 occasions of adding people to teams, 5 retirings of people from teams;
- 3 babies born to people on [the Rust teams];
- 2 years of [stability delivered].

On an average week this year, the Rust community merged 1 RFC and published 83
brand new [crates]. Rust topped the "[most loved] language" for the second year
in a row in the StackOverflow survey. Also new this year is
[thanks.rust-lang.org], a site where you can browse contributors by release!

[Rust's first birthday]: https://blog.rust-lang.org/2016/05/16/rust-at-one-year.html
[survey]: https://blog.rust-lang.org/2017/05/03/survey.html
[most loved]: https://insights.stackoverflow.com/survey/2017#technology-most-loved-dreaded-and-wanted-languages
[commits]: https://github.com/rust-lang/rust/commits/master
[RFCs]: https://github.com/rust-lang/rfcs
[rust-lang.org]: https://www.rust-lang.org/
[friends]: https://www.rust-lang.org/en-US/friends.html
[stability delivered]: http://blog.rust-lang.org/2014/10/30/Stability.html
[thanks.rust-lang.org]: https://thanks.rust-lang.org/
[the Rust teams]: https://www.rust-lang.org/en-US/team.html
[crates]: https://crates.io/

## Rust in production

In addition to the 48 new Rust friends, we now have a [Rust jobs website]! More
and more companies are choosing Rust to solve problems involving performance,
scaling, and safety. Let's check in on a few of them.

[Rust jobs website]: http://rustjobs.rs/

Mozilla, Rust's main sponsor, has accelerated their use of Rust in production.
Not only did [Servo start shipping nightly builds], [Firefox 48] marked the
first Firefox release that included Rust code as part of the [Oxidation]
project. [Project Quantum], announced in October 2016, is an effort to
incrementally adopt proven parts of Servo into Firefox's rendering engine,
Gecko.

[Servo start shipping nightly builds]: https://blog.servo.org/2016/06/30/servo-nightlies/
[Firefox 48]: https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/
[Oxidation]: https://wiki.mozilla.org/Oxidation
[Project Quantum]: https://medium.com/mozilla-tech/a-quantum-leap-for-the-web-a3b7174b3c12

[GNOME], a free and open source desktop environment for Linux, went from
experimenting with Rust in [librsvg] in October 2016 to a [hackfest] in March
to work on the interoperability between GNOME and Rust to enable more GNOME
components to be written in Rust. The hackfest participants made good progress,
be sure to check out the reports at the bottom of the hackfest page for all the
details. We're all excited about the possibilities of Rust and GNOME working
together!

[GNOME]: https://www.gnome.org/
[librsvg]: https://people.gnome.org/~federico/news-2016-10.html#25
[hackfest]: https://wiki.gnome.org/Hackfests/Rust2017

This year, [npm] started using Rust in production to serve JavaScript packages,
using Rust to eliminate performance bottlenecks in their platform that serves
around 350 million packages a day. [Ashley Williams recently gave a
talk][ag_dubs] at RustFest in Ukraine about npm's experience with Rust in
production; video is coming soon!

[npm]: https://www.npmjs.com/
[ag_dubs]: http://2017.rustfest.eu/talks/#how-i-convinced-the-world-s-largest-package-manager-to-use-rust-and-so-can-you

## Rust in community

Speaking of conferences, We've had four Rust conferences in the last year:

- September 9-10, 2016: [RustConf 2016] in Portland, OR, USA;
- September 17, 2016: [RustFest 2016] in Berlin, Germany;
- October 27-28, 2016: [Rust Belt Rust 2016] in Pittsburgh, PA, USA;
- April 29-30, 2017: [RustFest 2017] in Kyiv, Ukraine.

[RustConf 2016]: TODO ATURON PROMISED
[RustFest 2016]: http://2016.rustfest.eu/
[Rust Belt Rust 2016]: http://conf2016.rust-belt-rust.com/
[Rustfest 2017]: http://2017.rustfest.eu/

And we have at least three conferences coming up!

- August 18-19, 2017: [RustConf 2017] in Portland, OR, USA;
- September, 2017: [Another RustFest] in Zurich, Switzerland;
- October 26-27, 2017: [Rust Belt Rust 2017] in Columbus, OH, USA.

[Rust Belt Rust 2017]: http://conf2017.rust-belt-rust.com/
[RustConf 2017]: http://rustconf.com/
[Another RustFest]: https://rustfest.ch/

That's not even including the [103 meetups worldwide][meetup] about Rust! Will
you be the one to run the fourth conference or start the 104th meetup? [Contact
the community team] for help and support!

[meetup]: http://rust.meetup.com/
[Contact the community team]: https://community.rs/

## Rust in 2017

The [2017 Roadmap] goals have been great for focusing community efforts towards
the most pressing issues facing Rust today. Of course we'd love for every
aspect of Rust to improve all the time, but we don't have an infinite number of
contributors with an infinite amount of time available yet!

Let's check in on some of the initiatives in each of the goals in the roadmap.
Take a look at each goal's tracking issue for even more initiatives than we're
mentioning here!

### [Rust should have a lower learning curve](https://github.com/rust-lang/rust-roadmap/issues/3)

The second edition of [The Rust Programming Language Book] is one chapter shy
of having its initial content complete! There's lots more editing to be done to
get the book ready for publication in October, though. The print version is
currently available for [preorder from No Starch], and the online version of
the second edition has boarded [the beta train] and will be an option in the
documentation shipped with Rust 1.18.0. Steve and I have gotten feedback that
[the ownership chapter] especially is much improved and has helped people
understand ownership related concepts better!

[The Rust Programming Language Book]: https://github.com/rust-lang/book
[preorder from No Starch]: https://www.nostarch.com/rust
[the beta train]: https://doc.rust-lang.org/beta/book/
[the ownership chapter]: https://doc.rust-lang.org/nightly/book/second-edition/ch04-00-understanding-ownership.html

The [Language Ergonomics Initiative] is another part of the lower learning
curve goal that has [a number of improvements][lei-tracker] in its pipeline.
The language team is eager to mentor people (another goal!) who are interested
in getting involved with moving these ergonomic improvement ideas forward by
writing RFCs and working with the community to flesh out the details of how
these improvements would work. Comment on the [tracking issue][lei-tracker] if
you'd like to jump in!

[Language Ergonomics Initiative]: https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html
[lei-tracker]: https://github.com/rust-lang/rust-roadmap/issues/17

Also check out:

- [The Rust Cookbook](https://github.com/brson/rust-cookbook)
- [The new error format rolled out in Rust 1.12.0](https://github.com/rust-lang/rust/issues/35233)
- [The question mark operator stabilized in Rust 1.13.0](https://github.com/rust-lang/rust/pull/31954)

### [Rust should have a pleasant edit-compile-debug cycle](https://github.com/rust-lang/rust-roadmap/issues/1)

Compiler improvements are ongoing. This year, [MIR became a default part of the
compilation process](https://github.com/rust-lang/rust/pull/34096), which was a
prerequisite to unlocking further improvements.

Below is a table of [benchmarks] comparing the time it takes to compile a few
crates and tests with Rust 1.8.0 (the stable release a year ago) to Rust 1.17.0
(the stable release today). On average, compile times have improved by 5-10%,
but some worst-case behavior has been fixed that results in >95% improvements
in certain programs. A couple of crates did show a slight regression of 1-15%
in compile times, but in most cases it's a win. The helloworld crate saw a 110%
increase in compile time, but it only equates to +0.1 second, which is probably
a small fixed overhead that gets overwhelmed in any larger project.

| Benchmark | 1.8.0 (time) | 1.8.0 ([rss]) | 1.17.0 (time) | 1.17.0 ([rss]) | % change (time) | % change ([rss]) |
|-------|--------------|---------------|---------------|----------------|-----------------|------------------|
| hyper 0.5.0 | 5.97s | 248MB | 5.44s | 234MB | -8.9% | -5.6 % |
| html5ever 2016-08-25 | 5.16s | 234MB | 3.98s | 228MB | -22.9% | -2.6% |
| issue-32062 | 10.40s | 127MB | 0.26s | 89MB | -97.5% | -29.9% |
| inflate 0.1.0 | 4.60s | 134MB | 4.06s | 141MB | -11.7% | 5.2% |
| regex 0.1.30 | 2.49s | 169MB | 2.86s | 178MB | 14.9% | 5.3% |
| helloworld | 0.09s | 78MB | 0.19s | 83MB | 111.1% | 6.4% |
| jld-day15-parser | 42.38s | 204 MB | 1.38s | 111MB | -96.7% | -45.6% |
| tuple-stress | 4.62s | 260MB | 4.67s | 363MB | 1.1% | 39.6% |
| rust-encoding 0.3.0 | 2.21s | 239MB | 1.91s | 163MB | -13.6% | -31.8% |
| issue-32278 | 2.22s | 213MB | 1.98s | 138MB | -10.8% | -35.2% |

*Thanks to Mark Simulacrum for gathering this benchmark data and Simon Heath
for analyzing it!*

[benchmarks]: https://github.com/rust-lang-nursery/rustc-benchmarks
[rss]: https://en.wikipedia.org/wiki/Resident_set_size

[`cargo check`] does a type check of a project without building it completely
for faster feedback.

Incremental compilation: https://internals.rust-lang.org/t/incremental-compilation-beta/4721
[`cargo check`]: https://github.com/rust-lang/cargo/pull/3296

### [Rust should provide a basic, but solid IDE experience](https://github.com/rust-lang/rust-roadmap/issues/2)

As part of our IDE initiative, we created the [Rust Language Server] project.
Its goal is to create a single tool that makes it easy for any editor or IDE to
have the full power of the Rust compiler for error checking, code navigation,
and refactoring by using the standard [language server
protocol](http://langserver.org/) created by Microsoft and Eclipse.

While still early in its life, today the RLS is [available from rustup] for
nightly users. It provides type information on hover, error messages as you
type, and different kinds of code navigation. It even provides refactoring and
formatting as unstable features! It works with projects as large as Cargo.
We're excited to watch the RLS continue to grow and hope to see it make its way
to stable Rust later this year.

[Rust Language Server]: https://github.com/rust-lang-nursery/rls
[available from rustup]: https://github.com/rust-lang-nursery/rls#setup

*Thanks to Jonathan Turner for this RLS summary!*

### [Rust should have 1.0-level crates for essential tasks](https://github.com/rust-lang/rust-roadmap/issues/11), and [Rust should provide easy access to high quality crates](https://github.com/rust-lang/rust-roadmap/issues/9)

The [recent post on the Libz Blitz] details the Library Team's initiative to
increase the quality of crates for common tasks; that post is excellent so I
won't repeat it here! I will note that many of the issues that the Libs Team
is going to create will be great starter issues. For the blitz to be the best
it can be, the Libs Team is going to need help from the community-- that means
YOU! :) They're willing to mentor people interested in contributing!

In order to make awesome crates easier to find for particular purposes,
crates.io now has [categories] for crate authors to better indicate the use
case of their crate. Crates can also now have CI badges, and
[more improvements to crates.io's interface] are coming that will help you
choose the crates that fit your needs.

[recent post on the Libz Blitz]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[categories]: https://crates.io/categories
[more improvements to crates.io's interface]: https://github.com/rust-lang/rust/issues/41616

### [Rust should be well-equipped for writing robust, high-scale servers](https://github.com/rust-lang/rust-roadmap/issues/10)

- [Futures] and [tokio] enable zero-cost asynchronous programming

[Futures]: https://crates.io/crates/futures
[tokio]: https://tokio.rs/

### [Rust should integrate easily into large build systems](https://github.com/rust-lang/rust-roadmap/issues/12)

This initiative is mostly in the ideas stage; there's lots of great discussion
on the tracking issue that has resulted in a few Cargo issues:

* [Support creating a build plan](https://github.com/rust-lang/cargo/issues/3815)
* [Support declaring external dependencies](https://github.com/rust-lang/cargo/issues/3816)

Keep an eye out for more improvement in this area soon!

### [Rust's community should provide mentoring at all levels](https://github.com/rust-lang/rust-roadmap/issues/13)

There were [RustBridge] events before RustFest Berlin and Rust Belt Rust, and
there's another planned for the day before RustConf coming up!

[RustBridge]: https://github.com/rust-community/rustbridge

There are many people who have started getting involved with various
initiatives. I've worked with many people at many places in their Rust journey:
helping out with the conferences, giving their first conference talks,
providing feedback on the book, working on crates, contributing to Rust itself,
and joining teams! While we haven't made as much formal, measurable progress on
this front, everywhere I turn I see Rustaceans helping other Rustaceans.

## Rust in the future

While Rust is still just a toddler, it's still growing and still going strong.
The libraries and the infrastructure are maturing, we're paving the on-ramp,
and we're supporting each other. I'm optimistic about the direction Rust is
taking!

Happy birthday, Rust! Here's to many more! 🎉
