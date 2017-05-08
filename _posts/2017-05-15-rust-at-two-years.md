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

TODO:
- intro
- what rust is
- 2 years old


* [Rust should have a lower learning curve](https://github.com/rust-lang/rust-roadmap/issues/3)
* [Rust should have a pleasant edit-compile-debug cycle](https://github.com/rust-lang/rust-roadmap/issues/1)
* [Rust should provide a solid, but basic IDE experience](https://github.com/rust-lang/rust-roadmap/issues/2)
* [Rust should provide easy access to high quality crates](https://github.com/rust-lang/rust-roadmap/issues/9)
* [Rust should be well-equipped for writing robust, high-scale servers](https://github.com/rust-lang/rust-roadmap/issues/10)
* [Rust should have 1.0-level crates for essential tasks](https://github.com/rust-lang/rust-roadmap/issues/11)
* [Rust should integrate easily into large build systems](https://github.com/rust-lang/rust-roadmap/issues/12)
* [Rust's community should provide mentoring at all levels](https://github.com/rust-lang/rust-roadmap/issues/13)


### Rust in numbers

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
-->

- 10,583 [commits] by 655 contributors added to the core repository;
- 55 [RFCs] merged;
- 9 major releases and 2 patch releases shipped;
- 4,303 new [crates] published;
- 284 library stabilizations;
- 3 babies born to people on [the Rust teams];
- 2 years of [stability delivered].

On an **average week** this year, the Rust community merged X RFCs and
published Y brand new [crates]. Not a single day went by without at least one
new Rust library hitting the central package manager. And Rust topped the
"[most loved] language" for the second year in a row in the StackOverflow survey.

- our survey
- https://thanks.rust-lang.org/
- http://rustjobs.rs/
- teams
- stabilized features

[Rust's first birthday]: https://blog.rust-lang.org/2016/05/16/rust-at-one-year.html
[survey]: https://blog.rust-lang.org/2017/05/03/survey.html
[most loved]: https://insights.stackoverflow.com/survey/2017#technology-most-loved-dreaded-and-wanted-languages
[commits]: https://github.com/rust-lang/rust/commits/master
[RFCs]: https://github.com/rust-lang/rfcs
[stability delivered]: http://blog.rust-lang.org/2014/10/30/Stability.html
[the Rust teams]: https://www.rust-lang.org/en-US/team.html
[crates]: https://crates.io/

### Rust in production

npm
gnome

- number of companies added

This year saw more companies [betting on Rust].

[betting on Rust]: https://www.rust-lang.org/friends.html

- oxidation

### Rust, improved

-
- [Errors were updated to a new format](https://github.com/rust-lang/rust/issues/35233)
- [The question mark operator was added](https://github.com/rust-lang/rust/pull/31954)
- [Macros 1.1 enabled custom derive](https://github.com/rust-lang/rust/pull/35957)
- [Rustbuild now uses Rust to build Rust](https://github.com/rust-lang/rust/pull/37817)
- [`cargo check` does a type check of a project without building it completely for faster feedback](https://github.com/rust-lang/cargo/pull/3296)

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

| Crate | 1.8.0 (time) | 1.8.0 ([rss]) | 1.17.0 (time) | 1.17.0 ([rss]) | % change (time) | % change ([rss]) |
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

### Rust ecosystem, improved

- [Rustup reached 1.0](https://github.com/rust-lang/rust/pull/31954)
- [Rust Language Server started](https://github.com/rust-lang-nursery/rls)
- futures/tokio
- ring
- ripgrep

- the book


### Rust in community

We've had four Rust conferences in the last year:

- September 9-10, 2016: [RustConf 2016] in Portland, OR, USA;
- September 17, 2016: [RustFest 2016] in Berlin, Germany;
- October 27-28, 2016: [Rust Belt Rust 2016] in Pittsburgh, PA, USA;
- April 29-30, 2017: [RustFest 2017] in Kyiv, Ukraine.

[RustConf 2016]: TODO
[RustFest 2016]: http://2016.rustfest.eu/
[Rust Belt Rust 2016]: http://conf2016.rust-belt-rust.com/
[Rustfest 2017]: http://2017.rustfest.eu/

And we have at least three conferences coming up!

- August 18-19, 2017: [RustConf 2017] in Portland, OR, USA;
- September, 2017: [Another RustFest] in Zurich, Switzerland;
- October 26-27, 2017: [Rust Belt Rust 2017] in Columbus, OH, USA;

[Rust Belt Rust 2017]: http://conf2017.rust-belt-rust.com/
[RustConf 2017]: http://rustconf.com/
[Another RustFest]: https://rustfest.ch/

And that's not even including the [103 meetups worldwide][meetup] about Rust!
Will you be the one to run the fourth conference or start the 104th meetup?
[Contact the community team] for help and support!

[meetup]: http://rust.meetup.com/
[Contact the community team]: https://community.rs/

Happy birthday, Rust!
