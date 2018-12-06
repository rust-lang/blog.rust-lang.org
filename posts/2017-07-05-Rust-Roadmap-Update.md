---
layout: post
title: "Rust's 2017 roadmap, six months in"
author: Nicholas Matsakis
---

In January of this year, we adopted the [2017 Rust Roadmap][rr], which
laid out our plans for 2017. As part of the roadmap process, we plan
to regularly release updates on the progress of each roadmap item.
This post marks the halfway point through the year.

[rr]: https://github.com/rust-lang/rfcs/blob/master/text/1774-roadmap-2017.md

### Tracking progress via roadmap issues

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
edition of the official "Rust" book ([roadmap issue][rr7]), and we now have a
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
newer ideas, like the recently approved RFCs on [trait aliases] and
[match ergonomics]. On the [roadmap issue][rr17], you will find a
large list of initiatives, organized by the part of the language that
they target (e.g., ownership/borrowing, the trait system, etc). We
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
code-generation and simply looks for errors. Since code
generation typically takes 50% or more of compilation time, this can
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

In addition, the effort on the ["cookbook"][cook] described below will
also be a boon for discovering crates in a task-centric way.

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

The [Libz Blitz][blitzblog] proceeds apace! The Libz Blitz is a
systematic effort to identify the most broadly used crates in the Rust
ecosystem and to ensure that they all meet a consistent level of
completeness and quality. This effort entails collaborating on the
internals forum to review crates according to the [API guidelines],
filing and fixing the issues that arise, and writing examples for a
new ["cookbook" of Rust examples][cook].

The effort is structured to be highly amenable to contribution,
particularly from new Rust developers, and so far has resolved 99
crate issues across 10 crates, and created more than 30 examples for
the cookbook, thanks to the efforts of 53 contributors.

If you're interested in participating, take a look at the
[introductory post on the internals thread][blitz].

<!-- Thanks to Aaronepower, aergonaut, alisha17, AndyGauge, bjnyfv, brson, budziq,
cetra3, cGuille, chrisvittal, cldershem, Cldfire, crazymerlyn,
crypto-universe, DarkEld3r, derekdreery, dtolnay, Enet4, gsquire,
imor, jaemk, jehiggs, jmcomets, kbknapp, koivunej, kper, little-dude,
luisbg, MarkMcCaskey, Matt8898, Michael-F-Bryan, mikeastock, morrme,
neosilky, nivkner, omh1280, opilar, pepyakin, Piripant, pwoolcoc,
rap2hpoutre, robo9k, SamWhited, sb89, seanmonstar, SimonSapin,
theduke, tmccombs, tomecki, tomprince, topecongiro, vedantrathore,
xpayn, and everyone else who helps make Rust great. -->

[blitzblog]: https://blog.rust-lang.org/2017/05/05/libz-blitz.html
[blitz]: https://internals.rust-lang.org/t/rust-libz-blitz/5184
[API guidelines]: https://github.com/brson/rust-api-guidelines
[cook]: https://rust-lang-nursery.github.io/rust-cookbook/

### Rust should integrate easily into large build systems 

Most work on build system integration has focused on more clearly 
identifying what the challenges are and developing concrete proposals 
that target them. Some of the current avenues for exploration are:

- [Support for using Cargo to create a build plan but not execute it][3815]
- [Support declaring external dependencies in a first-class way][3816]
  (rather than via arbitrary build scripts as we do today)

[3815]: https://github.com/rust-lang/cargo/issues/3815
[3816]: https://github.com/rust-lang/cargo/issues/3816

We are hoping to pick up the pace on this work in the second half of the 
year, but could use help doing so. If either of the above Cargo improvements
is of interest to you, or if you have insights on build system integration
in general, please reach out on the [tracking issue][integration]!

[integration]: https://github.com/rust-lang/rust-roadmap/issues/12

### Rust's community should provide mentoring at all levels

When it comes to mentoring, we've been pursuing a few different
efforts. The first, [RustBridge], is specifically aimed at bringing
underrepresented folks into tech; it also serves as a great curriculum
for people completely new to Rust. The materials have already been
through several iterations and continue to evolve and improve, and we
are going to be
[running a RustBridge workshop the day before RustConf][rbw].  We
would like to see more RustBridge events.

[rbw]: http://rustconf.com/training.html
[RustBridge]: https://github.com/rust-community/rustbridge/

We also just launched [Increasing Rust's Reach], an initiative for hearing more
from groups currently underrepresented in Rust (or technology more generally),
and working together to make Rust accessible to a wider audience. While this
isn't a mentorship program per se (in many cases, it's the *participants* who 
are doing the teaching!), it is still geared toward lowering the on-ramp to
Rust's community.

[Increasing Rust's Reach]: https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html

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

### Areas of Exploration

In addition to the main roadmap items, the roadmap RFC called out two
"areas of exploration". These are areas with strong potential for Rust
that are still in a more exploratory phase.

#### Embedded Rust initiative

The embedded Rust ecosystem continues to grow. A bare metal
concurrency [framework] for Cortex-M microcontrollers geared towards
[robotics] and [control] systems has been recently developed. And
[Tock], an embedded OS that also targets Cortex-M microcontrollers,
has been making progress towards pure
[Rust userland applications][libtock] and continues growing
[its driver support][tock-blog].

[framework]: https://docs.rs/cortex-m-rtfm/0.1.1/cortex_m_rtfm/
[robotics]: https://github.com/japaric/2wd
[control]: https://mobile.twitter.com/japaricious/status/845697935572656128
[Tock]: https://www.tockos.org/
[libtock]: https://github.com/helena-project/libtock-rs
[tock-blog]: https://www.tockos.org/blog/

On the compiler side support for the MSP430 architecture has been improving
thanks to [the community's effort][msp430], and
[support for the AVR architecture][avr-rust] is also being worked on, out of
tree, by the community.

[msp430]: https://github.com/rust-embedded/rfcs/issues/20
[avr-rust]: https://github.com/avr-rust/rust/issues

On the community side efforts in standardizing the development workflow are on
going with the creation and development of [guides], [project templates],
[core crates] and [tooling]. Recently a [survey] to identify the current needs
of embedded developers was performed and the corresponding [roadmap issue] was
updated to reflect the results. In response to the survey results a community
effort to create a Hardware Abstraction Layer, that will serve as a base for
building the embedded crate ecosystem, has been [started] and the design
[discussion] is currently on going. An "Are we embedded yet?" web site that will
reflect the up to date state of the embedded ecosystem and track its progress is
also [being worked on][site].

[guides]: http://blog.japaric.io/quickstart/
[project templates]: https://github.com/japaric/photon-quickstart
[core crates]: https://github.com/japaric/cortex-m
[crates]: https://github.com/japaric/cortex-m
[tooling]: https://github.com/japaric/svd2rust
[survey]: https://users.rust-lang.org/t/rust-for-embedded-development-where-we-are-and-whats-missing/10861
[roadmap issue]: https://github.com/rust-lang/rust-roadmap/issues/15
[started]: https://github.com/japaric/embedded-hal
[discussion]: https://github.com/japaric/embedded-hal/issues
[site]: https://github.com/rust-embedded/rfcs/issues/15

(Thanks to [Jorge Aparicio][japaric] for this writeup.)

[japaric]: https://github.com/japaric

#### Integrating with Other Languages: `bindgen` Update

##### C and C++

[`bindgen`][bindgen-github] is the frontier for automating integration
of C and C++ libraries into Rust code bases. `bindgen` takes header
files as input, and outputs the appropriate foreign function and type
declarations so that the C/C++ library can be used with minimal effort
from Rust.

`bindgen` has become a crucial infrastructure for the [Stylo project][stylo],
which brings Servo's style system to Firefox. As the Stylo project nears
shipping, we've been hammering `bindgen` into shape for production. This has
manifested itself as tons of reliability and correctness work. Our test suite is
ever expanding, we've been focusing on correctness of struct layout, size, and
alignment for ABI corner cases such as bitfields, as well as test coverage and
support across different versions of libclang.

At the same time, we've been working to improve the [contribution
experience][bindgen-contributing]. We've been documenting workflows, adding
[visualizations of our internal representation][bindgen-graphviz] for debugging, and mentoring
new contributors. Since the [creation of the Rust DevTools
team][rust-dev-tools], we've also been talking with other toolsmiths about
fostering contribution to common tools. Expect to hear more on this soon.

Finally, we also introduced a [`bindgen` Users Guide][bindgen-users-guide] to
help folks get up and running with `bindgen` in their project.

(Thanks to [Nick Fitzgerald][fitzgen] for this writeup.)

[bindgen-github]: https://github.com/servo/rust-bindgen
[stylo]: https://wiki.mozilla.org/Stylo
[bindgen-contributing]: https://github.com/servo/rust-bindgen/blob/master/CONTRIBUTING.md
[bindgen-graphviz]:https://github.com/servo/rust-bindgen/blob/master/example-graphviz-ir.png
[rust-dev-tools]: https://internals.rust-lang.org/t/announcing-the-infrastructure-cargo-and-dev-tools-teams-disbanding-the-tools-team/5256
[bindgen-users-guide]: https://servo.github.io/rust-bindgen/
[fitzgen]: https://github.com/fitzgen

##### Other languages/environments

Higher level languages come with their own integration challenges, often involving cooperation with an external runtime system (which possibly includes a garbage collector). Here's a quick rundown of some of the major projects on this front:

- **Ruby**: the [Helix] project is geared toward writing Ruby extensions in Rust, and publicly launched a couple of months ago.
- **Node.js**: the [Neon] project is similarly geared toward writing Node.js modules in Rust, and continues to see active development.
- The **GNOME Object system**: after a sprint pairing up Rust and GNOME core developers, we have the basics of an [integration story](http://smallcultfollowing.com/babysteps/blog/2017/05/02/gnome-class-integrating-rust-and-the-gnome-object-system/) for Rust and the GObject system.
- The [Rust FFI Omnibus] gives guidance for the basics of calling into Rust from a variety of languages.

There are many less high-profile projects in this space as well; if you'd like yours to be tracked as part of the roadmap, please leave a comment on the [tracking issue][ffi-tracker]!

[Helix]: http://usehelix.com/
[Neon]: https://github.com/dherman/neon
[Rust FFI Omnibus]: http://jakegoulding.com/rust-ffi-omnibus/
[ffi-tracker]: https://github.com/rust-lang/rust-roadmap/issues/14

### Conclusion

In conclusion, you can see that it's been a very busy six months in
Rust land. I'd like to thank all the many people who have been
involved in pushing these projects over the finish line!

