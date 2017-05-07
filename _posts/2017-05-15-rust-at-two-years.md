---
layout: post
title: "Two years of Rust"
author: Carol (Nichols || Goulding)
description: "Rust, two years after 1.0"
---

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
- 4,303 new crates published;
- 284 library stabilizations;
- 2 years of [stability delivered].

On an **average week** this year, the Rust community merged X RFCs and
published Y brand new [crates]. Not a single day went by without at least one
new Rust library hitting the central package manager. And Rust topped the
"[most loved] language" for the second year in a row in the StackOverflow survey.

- stack overflow survey
- our survey
- https://thanks.rust-lang.org/
- http://rustjobs.rs/
- teams
- stabilized features
- babies (2)

[Rust's first birthday]: https://blog.rust-lang.org/2016/05/16/rust-at-one-year.html


[survey]: https://blog.rust-lang.org/2017/05/03/survey.html

[most loved]: https://insights.stackoverflow.com/survey/2017#technology-most-loved-dreaded-and-wanted-languages

[commits]: https://github.com/rust-lang/rust/commits/master
[RFCs]: https://github.com/rust-lang/rfcs
[stability delivered]: http://blog.rust-lang.org/2014/10/30/Stability.html
[crates]: https://crates.io/

### Rust in production

npm
- number of companies added

This year saw more companies [betting on Rust].

[betting on Rust]: https://www.rust-lang.org/friends.html

- oxidation

### Rust, improved

- [MIR became the default](https://github.com/rust-lang/rust/pull/34096)
- [Errors were updated to a new format](https://github.com/rust-lang/rust/issues/35233)
- [The question mark operator was added](https://github.com/rust-lang/rust/pull/31954)
- [Macros 1.1 enabled custom derive](https://github.com/rust-lang/rust/pull/35957)
- [Rustbuild now uses Rust to build Rust](https://github.com/rust-lang/rust/pull/37817)
- [`cargo check` does a type check of a project without building it completely for faster feedback](https://github.com/rust-lang/cargo/pull/3296)


### Rust ecosystem, improved

- [Rustup reached 1.0](https://github.com/rust-lang/rust/pull/31954)
- [Rust Language Server started](https://github.com/rust-lang-nursery/rls)
- futures/tokio
- ring
- ripgrep

- the book


### Rust in community

It turns out that people like to get together and talk Rust. We had a sold out
[RustCamp] last August, and several upcoming events in 2016:

- September 9-10, 2016: the first [RustConf] in Portland, OR, USA;
- September 17, 2016: [RustFest], the European community conference, in Berlin, Germany;
- October 27-18, 2016: [Rust Belt Rust], a Rust conference in Pittsburgh, PA, USA;
- 71 Rust-related [meetup] groups worldwide.

[RustCamp]: http://rustcamp.com/
[RustConf]: http://rustconf.com/
[RustFest]: http://www.rustfest.eu/blog/happy-birthday-announcing-rustfest
[Rust Belt Rust]: http://rust-belt-rust.com/
[meetup]: http://rust.meetup.com/

Happy birthday, Rust!
