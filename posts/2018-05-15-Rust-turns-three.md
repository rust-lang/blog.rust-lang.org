---
layout: post
title: "Rust turns three"
author: Aaron Turon
description: "Three years ago today, the Rust community released Rust 1.0 to the world, with our initial vision of fearless systems programming."
---

Three years ago today, the Rust community released [Rust 1.0] to the world, with
our initial vision of fearless systems programming. As per tradition, we’ll
celebrate Rust’s birthday by taking stock of the people and the product, and
especially of what’s happened in the last year.

[Rust 1.0]: https://blog.rust-lang.org/2015/05/15/Rust-1.0.html

# The People

Rust is a people-centric, consensus-driven project. Some of the most exciting
developments over the last year have to do with how the project itself has
grown, and how its processes have scaled.

The [official teams](https://www.rust-lang.org/en-US/team.html) that oversee the
project **doubled** in size in the last year; there are now over a hundred
individuals associated with one or more of the teams. To accommodate this scale,
the team structure itself has evolved. We have top-level teams covering the
language, library ecosystem, developer tooling, documentation, community, and
project operations. Nested within these are dozens of subteams and working
groups focused on specific topics.

Rust is now used in a huge variety of companies, including both newcomers and
big names like Google, Facebook, Twitter, Dropbox, Microsoft, Red Hat, npm and,
of course, [Mozilla](https://blog.rust-lang.org/2017/11/14/Fearless-Concurrency-In-Firefox-Quantum.html);
it’s also in the top 15 languages this year on GitHub. As a byproduct, **more and more
developers are being paid to contribute back to Rust**, many of them full
time. As of today, Mozilla employees make up only 11% of the official Rust
teams, and just under half of the total number of people paid to work on
Rust. (You can read detailed whitepapers about putting Rust into
production [here](https://www.rust-lang.org/en-US/whitepapers.html).)

![Graphs of Rust team growth][team]

[team]: /images/2018-05-Third-Birthday/team.png

Finally, the Rust community continues to work on inclusivity, through outreach
programs like [Rust Reach](https://blog.rust-lang.org/2018/04/02/Increasing-Rusts-Reach-2018.html) and
[RustBridge](https://rustbridge.github.io/), as well as
[structured mentoring](https://blog.rust-lang.org/2017/09/18/impl-future-for-rust.html) and
investments in [documentation](https://rust-lang-nursery.github.io/rustc-guide/)
to ease contribution. For 2018, a major goal is to
[connect and empower Rust’s *global* community](https://blog.rust-lang.org/2018/03/12/roadmap.html),
which we’re doing both through conference launches in multiple new continents,
as well as work toward internationalization throughout the project.

# The Product

If you spend much time reading this blog, you’ll know that the major theme of
our work over the past year has been **productivity**. As we said
in [last year’s roadmap](https://blog.rust-lang.org/2017/02/06/roadmap.html):

> From tooling to libraries to documentation to the core language, we want to
> make it easier to get things done with Rust.

This work will culminate in a major release later this year: **Rust 2018
Edition**. The release will bring together improvements in every area of the
project, polished into a new “edition” that bundles the changes together with
updated documentation and onboarding.
The [roadmap](https://blog.rust-lang.org/2018/03/12/roadmap.html) has some details about
what to expect.

The components that make up Rust 2018 will be shipped as they become ready on
the stable compiler. Recent releases include:

- [On-by-default incremental compilation](https://blog.rust-lang.org/2018/02/15/Rust-1.24.html)
- [A rewritten official book](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html)
- [Several of the planned language improvements, including `impl Trait` and `match` improvements](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html)

The next couple of releases will
include [stable SIMD](https://github.com/rust-lang/rust/issues/48556) support,
procedural macros, custom allocators, and more. The final big features
— [lifetime system improvements](https://github.com/rust-lang/rfcs/pull/2094)
and [async/await](https://github.com/rust-lang/rfcs/pull/2394) — should both
reach feature complete status on nightly within weeks. Vital tools like the RLS and
`rustfmt` are also being polished for the new edition, including RFCs for finalizing
the [style](https://github.com/rust-lang/rfcs/pull/2436)
and [stability](https://github.com/rust-lang/rfcs/pull/2437) stories.

To help tie all this work to real-world use-cases, we’ve also targeted four
domains for which Rust provides a compelling end-to-end story that we want to
show the world as part of Rust 2018. Each domain has a dedicated working group
and is very much open for new contributors:

- [Embedded devices](https://internals.rust-lang.org/t/announcing-the-embedded-devices-working-group/6839)
- [Command-line apps](https://internals.rust-lang.org/t/announcing-the-cli-working-group/6872)
- [The browser, node.js, and portable embedding via WebAssembly](https://internals.rust-lang.org/t/come-join-the-rust-and-webassembly-working-group/6845/2)
- [Networking services](https://internals.rust-lang.org/t/announcing-the-network-services-working-group-wg-net/7354)

As [Rust 2018](https://blog.rust-lang.org/2018/03/12/roadmap.html) comes into
focus, we plan to provide a “preview” of the new edition for cutting-edge
community members to try out. Over the past couple of weeks we kicked off a
sprint to get the basics nailed down, but we need more help to get it ready for
testing. If you’re interested, you can dive into:


- [The rustfix issue tracker](https://github.com/rust-lang-nursery/rustfix/issues/)
- [Bugs in the lints that drive rustfix](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-rust-2018-preview)
- [The new “Edition Guide”, which will need lots of eyeballs on pull requests for its content](https://github.com/rust-lang-nursery/edition-guide)

# The Postscript

Rust’s growth continues to accelerate at a staggering rate. It has been voted
the [Most Loved Language on StackOverflow](https://insights.stackoverflow.com/survey/2018/#most-loved-dreaded-and-wanted)
for all three years since it shipped. Its community has never been healthier or more
welcoming. If you’re curious about using or contributing to Rust, there’s never
been a better time to get involved.

Happy 3rd birthday, Rust.
