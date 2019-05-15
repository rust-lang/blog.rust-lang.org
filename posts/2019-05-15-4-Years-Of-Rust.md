---
layout: post
title: "4 years of Rust"
author: The Rust Core Team
---

On May 15th, 2015, [Rust][rust-release] was released to the world! After 5 years of open development (and a couple of years of sketching before that), we finally hit the button on making the attempt to create a new systems programming language a serious effort!

It’s easy to look back on the pre-1.0 times and cherish them for being the wild times of language development and fun research. Features were added and cut, syntax and keywords were tried, and before 1.0, there was a big clean-up that removed a lot of the standard library. For fun, you can check Niko’s blog post on [how Rust's object system works][rust-object-system], Marijn Haverbeke’s talk on [features that never made it close to 1.0][marijn-rustfest] or even the [introductory slides about Servo][servo-introduction], which present a language looking very different from today.

Releasing Rust with stability guarantees also meant putting a stop to large visible changes. The face of Rust is still very similar to Rust 1.0. Even with the changes from last year’s 2018 Edition, Rust is still very recognizable as what it was in 2015. That steadiness hides that the time of Rust’s fastest development and growth is *now*. With the stability of the language and easy upgrades as a base, a ton of new features have been built. We’ve seen a bunch of achievements in the last year:

- We have been StackOverflow’s [“Most loved programming language”][stackoverflow] 4 consecutive years in a row
- We opened up a whole new area of development for stable Rust: [embedded development][rust-embedded]
- [Rust+WASM][rust-wasm] went from an experiment to a usable product, making rustc the first compiler with focus on supporting WASM
- We shipped a new language edition: [Rust 2018][rust-2018]
- [Crates.io][crates-io] passed a billion downloads and has over 25,000 crates available
- There’s now over 100 meetups around the world, in 42 countries
- 6(!) new conferences were spun up ([RustRush][rustrush], [RustCon Asia][rustcon-asia], [Oxidize][oxidize], [Rust LATAM][rust-latam], [Colorado Gold Rust][coloradogoldrust], [RustLab Italy][rustlab])

This list could go on and on. While the time before and after release was a time where language changes had huge impact how Rust is perceived, it's becoming more and more important what people start building in and around it. This includes projects like whole game engines, but also many small, helpful libraries, meetup formats, tutorials other educational material. Birthdays are a great time to take a look back over the last year and see the happy parts!

Rust would be nothing, and especially not winning prizes, without its community. Community happens everywhere! We would like to thank everyone for being along on this ride, from team members to small scale contributors to people just checking the language out and finding interest in it. Your interest and curiosity is what makes the Rust community an enjoyable place to be. Some meetups [are running birthday parties][calendar] today to which everyone is invited. If you are not attending one, you can take the chance to celebrate in any other fashion: maybe show us a picture of what you are currently working on or talk about what excites you. If you want to take it to social media, consider tagging our [Twitter account][twitter] or using the hashtag #rustbirthday.

[rust-release]: https://blog.rust-lang.org/2015/05/15/Rust-1.0.html
[rust-object-system]: https://smallcultfollowing.com/babysteps/blog/2012/04/09/rusts-object-system/
[marijn-rustfest]: https://www.youtube.com/watch?v=olbTX95hdbg
[servo-introduction]: http://venge.net/graydon/talks/intro-talk-2.pdf
[stackoverflow]: https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted
[rust-embedded]: https://www.rust-lang.org/what/embedded
[rust-2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[rust-wasm]: https://www.rust-lang.org/what/wasm
[crates-io]: http://crates.io
[rustrush]: https://rustrush.ru/
[rustcon-asia]: https://rustcon.asia/
[oxidize]: https://oxidizeconf.com/
[coloradogoldrust]: https://cogoldrust.com/
[rustlab]: https://www.rustlab.it/
[rust-latam]: https://rustlatam.org/
[calendar]: https://calendar.google.com/calendar/embed?showTitle=0&showPrint=0&showTabs=0&showCalendars=0&mode=AGENDA&height=400&wkst=1&bgcolor=%23FFFFFF&src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com&color=%23691426&ctz=Europe%2FMadrid
[twitter]: https://twitter.com/rustlang
