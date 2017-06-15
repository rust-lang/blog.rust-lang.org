---
layout: post
title: "Rust Roadmap Update"
author: Nicholas Matsakis
---

In January of this year, we adopted the [2017 Rust Roadmap][rr], which
laid out our plans for 2017. Now that 2017 is roughly half over, we
thought we would check in on our progress in each of our roadmap areas
thus far.  In addition to talking about what's been going on, we'll
also include some notes on how you can get involved.

[rr]: https://github.com/rust-lang/rfcs/blob/master/text/1774-roadmap-2017.md

### Tracking progress on the roadmap

First, a meta note. If you'd like to follow along with the progress on
a particular roadmap initiative, or to find out more about how you can
get involved, the best place to go is the
[list of issues on the rust-roadmap repo](https://github.com/rust-lang/rust-roadmap/issues/).
There you will find an issue dedicated to each of the major
initiatives that we are pushing on. These issues contain links to
ongoing work. You'll find a number of links to issues like this in the
descriptions that follow.

### Rust should have a lower learning curve

The most direct way to make Rust easier to learn is to improve the way
that we teach it. To that end, we've been hard at work on a brand new
edition of the "Rust" book ([roadmap issue][rr7]), and we now have a
[complete draft available online][book]. This new edition puts
ownership front and center and it also has expanded coverage of a
number of other areas in Rust, such as error handling, testing,
matching, modules, and more. Even better, you can
[pre-order a printed version][preorder] through No Starch Press. If
you'd like to help out, there is still
[lots of editing work to be done!](https://github.com/rust-lang/book/projects/1)

[rr7]: https://github.com/rust-lang/rust-roadmap/issues/7
[preorder]: https://www.nostarch.com/rust
[book]: https://doc.rust-lang.org/book/

We've also been working on a number of language changes aimed at
improving [language ergonomics][ergo]. These range from long-standing
proposals, like [non-lexical lifetimes][rr16] or [`impl Trait`], to
newer ideas, like the recently approved RFCs on [trait aliases][] and
[match ergonomics][]. On the [roadmap issue][rr17], you will find a
large list of initiatives, organized by the part of the language that
they target (e.g., ownwership/borrowing, the trait system, etc). We
are actively looking for people to help with writing RFCs but also
implementing the accepted RFCs -- if you're interested in helping out
with something, look for the "mentoring" contacts listed in the
roadmap issue or on the tracking issue for a specific RFC. And, of
course, if you think you've got a good idea that's not listed, open up
a thread on internals and let's talk about it!

[ergo]: https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html
[rr17]: https://github.com/rust-lang/rust-roadmap/issues/17
[rr16]: https://github.com/rust-lang/rust-roadmap/issues/16
[`impl Trait`]: https://github.com/rust-lang/rfcs/pull/1951
[trait aliases]: https://github.com/rust-lang/rfcs/pull/1733
[match ergonomics]: https://github.com/rust-lang/rfcs/pull/2005

### Rust should have a pleasant edit-compile-debug cycle

We've been targeting the problem of improving compiler performance in
a number of different ways. One of the simplest is the
[`cargo check` command that we released in Rust 1.16][1.16]
-- this command does a limited form of compilation which skips
code-generation and simply looks for type-errors. Since code
generation typically takes 50% of more of compilation time, this can
be really useful in the early stages when you are writing new code and
trying to get it to compile, particularly if you have a multi-crate
project. This command is also used by the RLS.

Of course, eventually you want to run your code, and for that you need
a full compilation. In order to make *that* faster, we've been hard at
work retooling the compiler to work in an incremental fashion.
Earlier this year, we advertised
[the "beta" release of incremental on nightly builds][icbeta].  While
the beta version sometimes achieved quite large speedups, we also
found that the dependency tracking was not as robust or effective as
we would like. Therefore, we are now adopting a new and improved
approach to incremental compilation that we expect be ready in the
next month or so. If you're interested in helping with the transition
to the new system, check out
[the incremental compilation roadmap issue][rr4] or
[the tracking issue for the new algorithm itself][42293]; you can also
follow [this internals thread][soyouwantic], where we regularly post
links to bugs that include mentoring instructions.

Looking beyond incremental compilation, we've also been taking steps
to optimize compilation time in other ways. Probably the most
important step in that respect has been getting a new version of
[the `perf.rust-lang.org` website][perf] up and running. The "perf"
website tracks the effect of each and every PR on compilation
performance, so that we can detect and correct regressions. In fact,
the new website immediately led to some
[major performance improvements][41469]. There is still lots of work
that could be done to improve it, however, ranging from evaluating and
improving our benchmark suite to improving the web interface; see the
[tracking issue on this topic][42611] for more.

[1.16]: https://blog.rust-lang.org/2017/03/16/Rust-1.16.html
[rr4]: https://github.com/rust-lang/rust-roadmap/issues/4
[icbeta]: https://internals.rust-lang.org/t/incremental-compilation-beta/4721
[42293]: https://github.com/rust-lang/rust/issues/42293
[soyouwantic]: https://internals.rust-lang.org/t/so-you-want-to-help-with-incremental-compilation/5313
[evaluating and improving out benchmark suite]: https://internals.rust-lang.org/t/measuring-compiler-performance/4966
[tracking the runtime of generated code]: https://github.com/rust-lang/rust/issues/31265
[perf]: http://perf.rust-lang.org/
[improving the web interface]: https://github.com/rust-lang-nursery/rustc-perf/issues
[41469]: https://github.com/rust-lang/rust/pull/41469
[42611]: https://github.com/rust-lang/rust/issues/42611

### Rust should provide a solid, but basic IDE experience

Since it first debuted at RustConf last year, the Rust Language
Service (RLS) has been growing rapidly ([roadmap issue][rr6]). It now
offers support for most basic IDE operations, such as "jump to
definition" or "find all uses", as well as offering code completion
(via [the racer project](https://github.com/racer-rust/racer)). At
this point, the focus is primarily on polish: making the RLS easier to
install and fixing bugs. For example, we recently made it possible to
[install the RLS directly through rustup][rlsinstall].

If you'd like to give the RLS a spin, the easiest way is to use
[the VSCode plugin]; however, the RLS is a generic server (it speaks
Microsoft's [Language Server Protocol][lsp]), and there are also
clients available for a number of other editors. A word of warning: at
this point, the RLS is still in an "alpha" period. While it is
eminently usable, you may encounter bugs or other limitations.

If you'd like to get involved with the RLS, check out the
[roadmap issue][rr6] or the
[RLS issue listing](https://github.com/rust-lang-nursery/rls/issues);
in particular, those things tagged with ["good first issue"][gfirls].

[gfirls]: https://github.com/rust-lang-nursery/rls/issues?q=is%3Aopen+is%3Aissue+label%3Agood-first-issue
[rr6]: https://github.com/rust-lang/rust-roadmap/issues/6
[the VSCode plugin]: https://github.com/jonathandturner/rls_vscode
[rlsinstall]: https://github.com/rust-lang-nursery/rls#step-2-switch-to-nightly
[lsp]: https://github.com/Microsoft/language-server-protocol

### Rust should provide easy access to high quality crates

As the size of the crates.io ecosystem has grown, the simple search
and sorting criteria used by the crates.io website are no longer that
helpful for figuring out which crates you should use. To address this,
we've added [categories] and [a number of badges] that crate authors
can add to their crates. These help people find crates for a
particular purpose and judge a crate’s quality at a glance. In
addition, [RFC 1824][1824] laid out a plan for improving the default
sort in crates.io and exposing additional information to help in
selecting a crate.  There is [a tracking issue][41616] that lists the
pieces of this RFC, and we’d love contributions towards those pieces!
Once the RFC is completely implemented and people have had a chance to
use the features for a while, we plan to ask the community for
feedback and make adjustments.

[categories]: https://crates.io/categories
[a number of badges]: http://doc.crates.io/manifest.html#package-metadata

[1824]: https://github.com/rust-lang/rfcs/pull/1824
[41616]: https://github.com/rust-lang/rust/issues/41616

### Rust should be well-equipped for writing robust servers

We've made some excellent strides the first half of this year towards
making Rust well equipped for writing robust servers.  The [`futures`]
crate and [Tokio] project continue to explore the asynchronous I/O
ecosystem and have seen some heavy usage through crates like [Hyper]
and production users like [linkerd-tcp]. Additionally we've seen
projects like [Rocket] continue to tirelessly improve the ergonomics
of Rust-on-the-server. A [recent discussion] of what the biggest
blockers are today has highlighted that [async/await] notation, better
Tokio/futures documentation, and a solid HTTP foundation for the
ecosystem are some of the next highest priorities. We plan to enable
async/await notation on the nightly Rust channel by the end of the
year and position it for stabilization in early 2018. This in turn
should help fuel a rewrite of Tokio's/`future`'s documentation and
continue to grow community support for crates such as HTTP.

[`futures`]: https://crates.io/crates/futures
[Tokio]: https://tokio.rs
[Hyper]: https://hyper.rs
[linkerd-tcp]: https://blog.buoyant.io/2017/03/29/introducing-linkerd-tcp/
[Rocket]: https://rocket.rs/
[recent discussion]: https://users.rust-lang.org/t/what-does-rust-need-today-for-server-workloads/11114
[async/await]: https://github.com/alexcrichton/futures-await

### Rust should have 1.0-level crates for essential tasks

We've started a systematic effort to identify the most broadly used
crates in the Rust ecosystem and to ensure that they all meet a
consistent level of completeness and quality. This effort is called
the [Libz Blitz][blitzblog]. Every two weeks, the libs team holds a
meeting focused on a particular, widely used crate, with the crate
author(s) in attendance. In that meeting, the state of the API,
documentation, and other details are reviewed, and a checklist of
actions that are needed before a 1.0 release is drawn up. That
checklist can then be used to drive community involvement and help
push the crate over the finish line.  If you're interested in
participating, take a look at the
[introductory post on the internals thread][blitz], which highlights
how you can get involved in the initial evaluations, or go and find a
work item from one of the crates that has already been evaluated:

- [log]
- [reqwest]
- [walkdir]
- [error-chain]

[log]: https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-16-log/5185
[reqwest]: https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-30-reqwest/5248
[walkdir]: https://internals.rust-lang.org/t/crate-evaluation-for-2017-06-13-walkdir/5306
[error-chain]: https://internals.rust-lang.org/t/crate-evaluation-for-2017-06-27-error-chain/5362
[blitzblog]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[blitz]: https://internals.rust-lang.org/t/rust-libz-blitz/5184

### Rust should integrate easily into large build systems 

There has been a lot of progress on more clearly identifying what the
challenges are and developing concrete proposals that target them.  We
now have a pretty decent idea of what improvements are needed to ease
build system integration, with buy-in from stakeholders. That
includes:

- [Support for using Cargo to create a build plan but not execute it][3815]
- [Support declaring external dependencies in a first-class way][3816]
  (rather than via arbitrary build scripts as we do today)
  
[3815]: https://github.com/rust-lang/cargo/issues/3815
[3816]: https://github.com/rust-lang/cargo/issues/3816

### Rust's community should provide mentoring at all levels

When it comes to mentoring, we've been pursuing a few different
efforts. The first, [RustBridge][], is focused on building teaching
and workshop materials that target people completely new to Rust. The
materials have already been through several iterations and continue to
evolve and improve.

In addition, the various Rust teams have been pursuing a number of
different initiatives trying to encourage people to get involved in
the Rust project itself:

- We've [added three new teams][newteams] -- infrastructure, cargo,
  and dev-tools -- and hence created new areas where people can get
  involved.
- The lang team has adopted the new [shepherd role][sr]. Shepherds are
  experienced members of the community who have demonstrated both
  excellent technical acumen and the ability to build consensus and
  understand alternative perspectives. Shepherds attend language
  meetings and help to steer discussion on RFCs and guide them towards
  a useful conclusion.
- The lang team has also been working on "mentoring" RFCs. The
  [roadmap issue for the ergonomics initiative][rr17], for example,
  lists a number of RFCs where we are actively recruiting.
- A big part of the ["Libz Blitz"][blitz] is helping to direct community effort
  to pushing libraries over the finish line.
- The compiler team has been actively pursuing "mentoring bugs"
  ([bugs tagged as E-mentor][Em]), which include written instructions,
  as well as [drawing up plans][compiler] to improve the documentation
  of the code and workflows.

[sr]: https://internals.rust-lang.org/t/language-team-shepherds/4595
[RustBridge]: http://bridge.community.rs/
[Em]: https://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aopen%20is%3Aissue%20label%3AE-mentor%20label%3AT-compiler%20
[compiler]: https://internals.rust-lang.org/t/compiler-team-making-it-easer-to-contribute-to-rustc/5345
[newteams]: https://internals.rust-lang.org/t/announcing-the-infrastructure-cargo-and-dev-tools-teams-disbanding-the-tools-team/5256

### Other projects

Although they are not official Roadmap items, we also want to
highlight a few major initiatives that have seen great progress
lately. The [embedded Rust initiative][eri] is doing awesome stuff XXX
FILL THIS IN.

[eri]: https://github.com/rust-embedded/

### Conclusion

In conclusion, you can see that it's been a very busy six months in
Rust land. I'd like to thank all the many people who have been
involved in pushing these projects over the finish line!

