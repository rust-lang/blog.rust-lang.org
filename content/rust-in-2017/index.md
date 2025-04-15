+++
path = "2017/12/21/rust-in-2017"
title = "Rust in 2017: what we achieved"
authors = ["Aaron Turon"]
aliases = ["2017/12/21/rust-in-2017.html"]
+++

Rust’s development in 2017 fit into a single overarching theme: **increasing productivity, especially for newcomers to Rust**. From tooling to libraries to documentation to the core language, we wanted to make it easier to get things done with Rust. That desire led to [a roadmap](https://blog.rust-lang.org/2017/02/06/roadmap.html) for the year, setting out 8 high-level objectives that would guide the work of the team.

How’d we do? ***Really, really well***.

There’s not room in a single post to cover everything that happened, but we’ll cover some of the highlights below.

# The goals for 2017

## Rust should have a lower learning curve

- **Books**
  - [The Rust Programming Language](https://doc.rust-lang.org/stable/book/second-edition/) is largely in the final stages of editing. Steve and Carol are grateful for everyone who has read the drafts and provided feedback! [Preorder the print edition from No Starch Press](https://www.nostarch.com/rust), scheduled for release in May 2018.
  - [Programming Rust](http://shop.oreilly.com/product/0636920040385.do) by Jim Blandy and Jason Orendorff is available in print as of December 21, 2017!
  - [Rust in Action](https://www.manning.com/books/rust-in-action) by Tim McNamara is in Manning’s Early Access Program, with an estimated publication date of early 2019.
- **RustBridge curriculum**
  - [RustBridge workshops](https://rustbridge.github.io/) are focused on getting underrepresented people into Rust. Ashley Williams has vastly improved [the workshop curriculum](https://rustbridge.github.io/a-very-brief-intro-to-rust/#1) this year, and we’re planning on having a teacher training in early 2018. The curriculum is available for anyone to use, as long as events are only called RustBridge if they focus on underrepresented folks. See more about RustBridge in 2017 under the Mentorship goal!
- **Language improvements**
  - The [Ergonomics Initiative](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html) saw a large number of RFCs tackling rough edges across the language; the [Impl Period](https://blog.rust-lang.org/2017/09/18/impl-future-for-rust.html) then saw almost all of these RFCs implemented. These improvements cover ownership ([more flexible lifetimes](https://github.com/rust-lang/rfcs/pull/2094), [smoother pattern matching](https://github.com/rust-lang/rfcs/pull/2005), [more concise elision](https://github.com/rust-lang/rfcs/pull/2115)), the module system ([revamping paths for greater clarity](https://github.com/rust-lang/rfcs/pull/2126), [allowing nested imports](https://github.com/rust-lang/rfcs/pull/2128)), the trait system ([`impl Trait`](https://github.com/rust-lang/rfcs/pull/1951), [trait aliases](https://github.com/rust-lang/rfcs/pull/1733)) and more. **You can get an overview of all of these changes, and their current status, on [the tracking issue](https://github.com/rust-lang/rust/issues/46889)**. Altogether, these changes should eliminate or mitigate many of the most common learnability and ergonomics hazards that have been surfaced since Rust 1.0.

## Rust should have a pleasant edit-compile-debug cycle

- **The `cargo check` workflow**
  - Cargo [now offers](https://blog.rust-lang.org/2017/03/16/Rust-1.16.html) a `check` subcommand which can be used to speed up the edit-compile cycle when you’re  working on getting your code to pass the compiler’s checks. This mode, in particular, skips producing executable artifacts for crates in the dependency tree, instead doing just enough work to be able to type check the current crate.
- **Incremental recompilation**
  - The cornerstone of our approach to improving compilation times is incremental recompilation, allowing rebuilds to reuse significant pieces of work from prior compilations. Over the course of the year we have put a lot of work into making this happen and now we are happy to announce that incremental compilation will [start riding the trains](https://blog.rust-lang.org/2014/10/30/Stability.html) with the next beta version of the compiler in January and become available on the stable channel with Rust 1.24 in February!
  - You can see how incremental recompilation performs in practice on some of our key benchmarks below. Note that `-opt` refers to optimized builds, “best case” refers to a recompilation with no changes, and `println` refers to a recompilation with a small change, like adding a `println` call to a function body. We expect the 50+% speedups we’re seeing now to continue to grow next year as we push incremental recompilation more deeply through the compiler.
  - Together with the changes in the compiler we will also update Cargo to use incremental recompilation by default for select use cases, so you can take advantage of improved compile times without the need for additional configuration. Of course you will also be able to opt into and out of the feature on a case by case basis as you see fit.

![Incremental recompilation benchmarks](incr-bench.png)

## Rust should provide a solid, but basic IDE experience

- Rust now has solid IDE support in IntelliJ and via the Rust Language Server (RLS). Whether you prefer a fully-featured IDE or a more lightweight editor with IDE features, you can boost your productivity by taking advantage of great Rust integration.
- **IntelliJ**. Rust has official support in [JetBrains' IDEs](https://blog.jetbrains.com/blog/2017/08/04/official-support-for-open-source-rust-plugin-for-intellij-idea-clion-and-other-jetbrains-ides/) (IntelliJ IDEA, CLion, WebStorm, etc.), which includes:
  - Finding types, functions and traits across the whole project, its dependencies and the standard library.
  - Hierarchical overview of the symbols defined in the current file.
  - Search for all implementations of a given trait.
  - Go to definition of symbol at cursor.
  - Navigation to the parent module.
  - Refactoring and code generation
- **RLS**. The [RLS](https://github.com/rust-lang-nursery/rls) is an editor-independent source of intelligence about Rust programs. It is used to power Rust support in many editors including [Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust), [Visual Studio](https://marketplace.visualstudio.com/items?itemName=DanielGriffen.Rust), and [Atom](https://atom.io/packages/ide-rust), with more in the pipeline. It is on schedule for a 1.0 release in early 2018, but is currently [available in preview form](https://blog.rust-lang.org/2017/10/12/Rust-1.21.html) for all channels (nightly, beta, and stable). It supports:
  - Code completion (using Racer)
  - Go to definition (and peek definition if the editor supports it)
  - Find all references
  - Find impls for a type or trait
  - Symbol search (current file and project)
  - Reformatting using rustfmt, renaming
  - Apply error suggestions (e.g., to add missing imports)
  - Docs and types on hover
  - Code generation using snippets
  - Cargo tasks
  - Installation and update of the RLS (via rustup)

## Rust should integrate easily into large build systems

- **Alternative registries.** Cargo now has unstable support for [installing crates from registries other than crates.io](http://rust-lang.github.io/rfcs/2141-alternative-registries.html). This will enable companies to manage and use internal crates as easily as open source crates. Work is underway developing crate servers that are more tailored for private use than the crates.io server is.
- **Cargo as a component**. A lot of work this year went into gathering constraints from stakeholders who want to integrate Rust crates into a large existing build system (like [Bazel](https://bazel.build/)). The Cargo team has [formulated a vision](https://github.com/rust-lang/rfcs/pull/2136) of Cargo as a suite of components that can be customized or swapped out, making it easy for an external build system to manage the work it is built to do, while still integrating with crates.io and with Cargo workflows. While we did not get as far as we hoped in terms of implementing this vision, there is ongoing work spiking out “build plan generation” to a sufficient degree that it can support the Firefox build system and [Tup](http://gittup.org/tup/). This initial spike should provide a good strawman for further iteration in early 2018.

## Rust should provide easy access to high quality crates

- Crates.io added [categories](https://crates.io/categories) this year, which aim to provide a crate organization structure targeted towards providing crates that are good for a particular purpose.
- We had a [lively RFC discussion](https://github.com/rust-lang/rfcs/pull/1824) about the best way to order crates within categories and keywords, which included [a survey of how people evaluate crates](http://rust-lang.github.io/rfcs/1824-crates.io-default-ranking.html#appendix-user-research).
- That discussion culminated in a decision to [order crates by the number of downloads in the last 90 days](https://github.com/rust-lang/crates.io/issues/702), and surfacing of more information for people to use when doing their evaluation.
- Some of the additional information now available for crate authors to display on crates.io includes badges for CI status, [maintenance status](https://github.com/rust-lang/crates.io/issues/704), [code coverage](https://github.com/rust-lang/crates.io/issues/706), [GitHub statistics](https://github.com/rust-lang/crates.io/issues/705).
- Most importantly, [crates.io now displays a crate’s README on the crate page](https://github.com/rust-lang/crates.io/issues/81)! Crate authors are encouraged to use this capability to provide getting started documentation with a small example of what it looks like to use this crate, because good documentation and examples were among the most-mentioned positive signals that people take into account when evaluating crates.

## Rust should be well-equipped for writing robust servers

- **Futures and Tokio**
  - Much of the story for Rust on the server has revolved around its async I/O story. The futures crate [was introduced](http://aturon.github.io/blog/2016/08/11/futures/) in late 2016, and the Tokio project (which provides a networking-focused event loop for use with futures) [published its 0.1](https://tokio.rs/blog/tokio-0-1/) early in 2017. Since then, there’s been significant work building out the “Tokio ecosystem”, and a lot of feedback about the core primitives. Late in the year, the Tokio team proposed a [significant API revamp](https://github.com/tokio-rs/tokio-rfcs/pull/3) to streamline and clarify the crate’s API, and work is underway on a book dedicated to asynchronous programming in Rust. This latest round of work is expected to land very early in 2018.
- **Async ecosystem**
  - There’s been huge growth in the ecosystem around Tokio, including support for [curl](https://github.com/tokio-rs/tokio-curl), [openssl](https://github.com/alexcrichton/tokio-openssl), [inotify](https://github.com/dermesser/tokio-inotify), [unix signals](https://github.com/alexcrichton/tokio-signal), [cap’n proto](https://github.com/dwrensha/capnproto-rust), [sendfile](https://crates.io/crates/tk-sendfile), [postgres](https://crates.io/crates/tokio-postgres), [couchbase](https://crates.io/crates/couchbase), and more. In addition, Rust has async server libraries for both [HTTP1](http://hyper.rs/) and [HTTP2.](https://github.com/carllerche/h2)
- **Generators**
  - Thanks to a heroic community effort, Rust also saw [experimental generator support](https://github.com/rust-lang/rfcs/pull/2033) land in 2017! That support provides the ingredients necessary for `async`/`await` notation, which is [usable today](https://internals.rust-lang.org/t/help-test-async-await-generators-coroutines/5835) on nightly. Further work in this area is expected to be a high priority in early 2018.
- **Web frameworks**
  - Finally, sophisticated web frameworks like [Rocket](https://rocket.rs/) (sync) and [Gotham](http://gotham.rs/) (async) have continued to evolve this year, and take advantage of Rust’s expressivity to provide a robust but productive style of programming.

## Rust should have 1.0-level crates for essential tasks

- **Libz Blitz**. The library team launched the [Libz Blitz](https://blog.rust-lang.org/2017/05/05/libz-blitz.html) this year, a major effort to vet and improve a large number of foundational crates and push them toward 1.0 releases. It was a massive community effort: we performed a crowd-sourced “crate evaluation” every two weeks, fully vetting a crate against a clear set of guidelines, assessing the issue tracker, and sussing out any remaining design questions. While not all of the assessed crates have published a 1.0 yet, they are all very close to doing so. The full list includes: [log](https://github.com/rust-lang-nursery/log/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22), [env_logger](https://github.com/sebasmagri/env_logger/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22), [rayon](https://github.com/nikomatsakis/rayon/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22), [mio](https://github.com/carllerche/mio/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20label%3A%22help%20wanted%22%20), [url](https://github.com/servo/rust-url/issues/319), [num_cpus](https://github.com/seanmonstar/num_cpus/issues/55), [semver](https://github.com/steveklabnik/semver/issues/139), [mime](https://github.com/hyperium/mime/labels/help%20wanted), [reqwest](https://github.com/seanmonstar/reqwest/issues?q=is%3Aissue+is%3Aopen+label%3Aeasy), [tempdir](https://github.com/rust-lang-nursery/tempdir/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22), [threadpool](https://github.com/rust-threadpool/rust-threadpool/issues/86), [byteorder](https://github.com/BurntSushi/byteorder/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20label%3A%22help%20wanted%22%20), [bitflags](https://github.com/rust-lang-nursery/bitflags/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20label%3A%22help%20wanted%22%20), [cc-rs](https://github.com/alexcrichton/cc-rs), [walkdir](https://github.com/BurntSushi/walkdir/issues/47), [same-file](https://crates.io/crates/same-file), [memmap](https://crates.io/crates/memmap), [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs/issues?utf8=%E2%9C%93&q=is%3Aopen%20is%3Aissue%20label%3A%22help%20wanted%22%20), [flate2](https://github.com/alexcrichton/flate2-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).
- **API Guidelines**. A great by-product of the Libz Blitz is the [API Guidelines](https://rust-lang-nursery.github.io/api-guidelines/) book, which consolidates the official library team API guidance as informed by the standard library and the Libz Blitz process.

## Rust’s community should provide mentoring at all levels

- We ran 5 [RustBridge Workshops](https://rustbridge.github.io/) in 2017, in Kyiv, Ukraine; Mexico City, Mexico; Portland, OR, USA; Zurich, Switzerland; and Columbus, OH, USA! RustBridge workshops aim to get underrepresented folks started in Rust. Attendees get an introduction to syntax and concepts, work on some [exercism](http://exercism.io/) exercises, and build a web application that delivers emergency compliments and crab pictures. We hope to scale this program and help more folks run more workshops in 2018!
- The [Increasing Rust’s Reach program](https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html) brought people with skills from other areas (such as teaching) and with different experiences into Rust so that we can improve in areas where the community is missing these skills and experiences. The participants have helped immensely, and many are planning to continue helping in the Rust community going forward. We’re glad they’re here! Here are some blog posts about the experience:
  - Anna Liao: [Increasing Rust’s Reach: Porting a Python application to Rust](https://medium.com/@aliao22/increasing-rusts-reach-porting-a-python-application-to-rust-60eaf92d891b)
  - Ryan Blecher: [Increasing Rust's Reach: Afterthoughts](http://notryanb.github.io/increasing-rusts-reach.html)
  We learned a lot in the first incarnation of this program, and we have plans for another round in 2018!
- Last but not least, we also launched the first Rust [`impl Period`](https://blog.rust-lang.org/2017/09/18/impl-future-for-rust.html). This was an ambitious effort to simultaneously help get a lot of new people contributing to the Rust ecosystem while also getting a lot of things done. To that end, we created 40+ working groups, each with their own focus area, leaders, and chat channel. These groups identified good “entry points” for people who wanted to contribute, and helped mentor them through the changes needed. This event was a wild success and resulted in changes and contributions to all areas of Rust, ranging from the compiler internals to documentation to the ecosystem at large. To those of you who participated, a great big thank you — and please keep contributing! To those of you who didn’t get a chance, don’t worry: we hope to make this a regular tradition.

# 2018

We’ll be spinning up the 2018 roadmap process in the very near future; watch this space!

# Thank you!

We got a staggering amount of work done this year — and the “we” here includes an equally staggering number of people. Because the work has been spread out over so many facets of the project, it’s hard to provide a single list of people who contributed. For the impl period specifically, you can see detailed contribution lists in the newsletters:


- [Newsletter 1](https://internals.rust-lang.org/t/the-impl-period-newsletter-week-1/5971)
- [Newsletter 2](https://internals.rust-lang.org/t/the-impl-period-newsletter-2/6034)
- [Newsletter 3](https://internals.rust-lang.org/t/impl-period-newsletter-3/6185)
- [Newsletter 4](https://internals.rust-lang.org/t/the-impl-period-newsletter-4/6313)
- [Newsletter 5](https://internals.rust-lang.org/t/the-final-impl-period-newsletter/6408)

but of course, there have been contributions of all kinds during the year.

In this post, I’d like to specifically call out the ***leaders*** and ***mentors*** who have helped orchestrate our 2017 work. Leadership of this kind — where you are working to enable others — is hard work and not recognized enough. So let’s hand it to these folks!


- **Cargo**
  - [carols10cents](https://github.com/carols10cents), for sustained leadership and mentoring work throughout the year on crates.io.
- **Community**
  - [carols10cents](https://github.com/carols10cents), for running the [Increasing Rust’s Reach](https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html) program.
  - [ashleygwilliams](https://github.com/ashleygwilliams), for overhauling the [RustBridge](https://rustbridge.github.io/) curriculum and otherwise driving the program forward.
  - [jonathandturner](https://github.com/jonathandturner/), for overseeing the [2017 Rust community survey](https://blog.rust-lang.org/2017/09/05/Rust-2017-Survey-Results.html).
- **Compiler**
  - [nikomatsakis](https://github.com/nikomatsakis/), for an incredible amount of leadership, organization, and mentoring work, *and* for a lot of high-value hacking on NLL in particular.
  - [arielb1](https://github.com/arielb1/), likewise for mentoring *and* hacking work, spanning both NLL and the rest of the compiler.
  - [michaelwoerister](https://github.com/michaelwoerister/), for pushing continuously on delivering incremental recompilation, and creating opportunities for others to join in throughout the year.
  - [eddyb](https://github.com/eddyb), for continuing to act as a general compiler guru, and for tackling some truly heavy lifts around const generics this year.
- **Dev tools**
  - [nrc](https://github.com/nrc/), for overseeing the dev tools group as a whole, and for steady work toward shipping the RLS and rustfmt, despite many thorny infrastructure problems to get there.
  - [matklad](https://github.com/matklad), for the incredible work on [IntelliJ Rust](https://github.com/intellij-rust/intellij-rust/blob/master/CONTRIBUTORS.txt)[.](https://github.com/intellij-rust/intellij-rust/blob/master/CONTRIBUTORS.txt)
  - [xanewok](https://github.com/xanewok), for [enormous efforts](https://github.com/rust-lang-nursery/rls/graphs/contributors) making the RLS a reality.
  - [fitzgen](http://github.com/fitzgen), for happily corralling a huge contributor base around [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).
- **Docs**
  - [steveklabnik](https://github.com/steveklabnik/), for launching and overseeing a hugely exciting revamp of rustdoc.
  - [quietmisdreavus](https://github.com/QuietMisdreavus), for overseeing tons of activity in the docs world, but most especially for helping the community significantly improve rustdoc this year.
- **Infrastructure**
  - [mark-simulacrum](https://github.com/Mark-Simulacrum), for getting the [perf](https://perf.rust-lang.org/) website to a highly useful state, and for overhauling rustbuild to better support contribution.
  - [aidanhs](https://github.com/aidanhs/), for coordinating maintenance of crater.
- **Language**
  - [withoutboats](https://github.com/withoutboats/), for keeping us focused on the programmer experience and for helping the community navigate discussion around very thorny language design issues.
  - [cramertj](https://github.com/cramertj/), for keeping us focused on *shipping*, and in particular building consensus around some of the topics where that’s been hardest to find: impl Trait, and module system changes.
  - [nikomatsakis](https://github.com/nikomatsakis/), for making the [NLL RFC](https://github.com/rust-lang/rfcs/pull/2094) so accessible, and pioneering the idea of using a separate repo for it to allow for greater participation.
- **Libraries**
  - [brson](https://github.com/brson/), for envisioning and largely overseeing the Libz Blitz initiative.
  - [kodraus](https://github.com/kodraus), for gracefully taking over the Libz Blitz and seeing it to a successful conclusion.
  - [dtolnay](https://github.com/dtolnay), for taking on the API guidelines work and getting it to a coherent and polished state.
  - [budziq](https://github.com/budziq), for a ton of work coordinating and editing contributions to the cookbook.
  - [dhardy](https://github.com/dhardy), for leading a heroic effort to revamp the rand crate.

Technical leaders are an essential ingredient for our success, and I hope in 2018 we can continue to grow our leadership pool, and get even more done — together.
