---
layout: post
title: "One year of Rust"
author: Aaron Turon
description: "Rust's trajectory one year after 1.0"
---

Rust is a language that gives you:

- uncompromising performance and control;
- prevention of entire categories of bugs, including [classic concurrency pitfalls];
- ergonomics that often rival languages like [Python] and [Ruby].

It's a language for writing highly reliable, screamingly fast software&mdash;and
having fun doing it.

[classic concurrency pitfalls]: http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[Python]: http://lucumr.pocoo.org/2015/5/27/rust-for-pythonistas/
[Ruby]: http://diesel.rs/

And yesterday, Rust turned one year old.

<img src="http://rustcamp.com/images/after/cupcakes.jpg" height="300px" width="300px">

### Rust in numbers

A lot has happened in the last 365 days:

- 11,894 [commits] by 702 contributors added to the core repository;
- 88 [RFCs] merged;
- 18 compiler targets introduced;
- 9 releases shipped;
- 1 year of [stability delivered].

On an **average week** this year, the Rust community merged two RFCs and
published 53 brand new [crates]. Not a single day went by without at least one
new Rust library hitting the central package manager. And Rust topped the
"[most loved] language" in this year's StackOverflow survey.

Speaking of numbers: we recently launched a [survey] of our own, and want to
hear from you whether you are an old hat at Rust, or have never used it.

One place where our numbers are not where we want them to be: community
diversity. We've had ongoing local outreach efforts, but the Rust community team
will soon be launching a coordinated, global effort following the [Bridge] model
(e.g. RailsBridge). If you want to get involved, or have other ideas for
outreach, please let [the community team] know.

[survey]: http://blog.rust-lang.org/2016/05/09/survey.html

[most loved]: https://stackoverflow.com/research/developer-survey-2016

[commits]: https://github.com/rust-lang/rust/commits/master
[RFCs]: https://github.com/rust-lang/rfcs
[stability delivered]: http://blog.rust-lang.org/2014/10/30/Stability.html
[crates]: https://crates.io/

[Bridge]: http://bridgefoundry.org/
[the community team]: mailto:community-team@rust-lang.org

### Rust in production

This year saw more companies [betting on Rust]. Each one has a story, but two
particularly resonated.

[betting on Rust]: https://www.rust-lang.org/friends.html

First, there's Dropbox. For the last several years, the company has been
secretively working on a move
[away from AWS and onto its own infrastructure][dropbox]. The move, which is now
complete, included developing custom-build hardware and the software to drive
it. While much of Dropbox's back-end infrastructure is historically written in
Go, for some key components the memory footprint and lack of control stood in
the way of achieving the server utilization they were striving for. They rewrote
those components in Rust.  In the [words of Jamie Turner][dropbox quote], a lead
engineer for the project, "the advantages of Rust are many: really powerful
abstractions, no null, no segfaults, no leaks, yet C-like performance and
control over memory."

[dropbox]: http://www.wired.com/2016/03/epic-story-dropboxs-exodus-amazon-cloud-empire/
[dropbox quote]: https://news.ycombinator.com/item?id=11283688

Second, there's Mozilla. They've long been developing [Servo] as a research
browser engine in Rust, but their first *production* Rust code shipped through a
different vehicle: *Firefox*. In Firefox 45, without any fanfare, Rust code for
[mp4 metadata parsing] went out to OSX and 64-bit Linux users; it will hit
Windows in version 48. The code is currently running in test mode, with its
results compared against the legacy C++ library: 100% correctness on
[1 *billion* reported executions][ff]. But this code is just the tip of the iceberg:
after laying a lot of [groundwork for Rust integration], Firefox is poised to
bring in significant amounts of new Rust code, including components from
Servo&mdash;and not just in test mode.

[Servo]: https://github.com/servo/servo/
[mp4 metadata parsing]: https://github.com/mozilla/mp4parse-rust
[ff]: https://telemetry.mozilla.org/new-pipeline/dist.html#!cumulative=0&end_date=2016-04-07&keys=__none__!__none__!__none__&max_channel_version=release%252F45&measure=MEDIA_RUST_MP4PARSE_SUCCESS&min_channel_version=null&product=Firefox&sanitize=1&sort_keys=submissions&start_date=2016-03-03&table=0&trim=1&use_submission_date=0
[groundwork for Rust integration]: http://wiki.mozilla.org/Oxidation

We're hearing [similar stories] from a range of other shops that are putting
Rust into production: Rust helps a team punch above its weight. It gives many of
the same benefits as traditional systems languages while being more
approachable, safer and often more productive.

[similar stories]: http://confreaks.tv/videos/rustcamp2015-using-rust-from-c-or-any-language

These are just a few stories of Rust in production, but we'd love to [hear yours]!

[hear yours]: https://github.com/rust-lang/rust-www/issues/new?title=New+Website+Logo%3A+[insert+name]%0A&body=To+list+your+organization%27s+logo+on+the+Rust+website%2C+fill+out+the+following+information+and+click+%22submit+new+issue%22.+Alternately%2C+you+may+edit+_data%2Fusers.yml+as+described+therein+and+submit+a+pull+request.%0D%0A%0D%0A-+Organization+name%3A+%28as+you+want+it+displayed%29%0D%0A-+Homepage+url%3A+%28homepage%2Fprimary+entry+point+for+users%29%0D%0A-+Logo+url%3A+%28svg+if+possible%2C+pngs+over+400x200px+with+transparent+backgrounds+are+also+acceptable%29%0D%0A-+How+you+are+using+Rust%3A+%28one+sentence+describing+your+use+of+Rust%29%0D%0A-+Url+describing+Rust+usage%3A+%28optional+link+to+e.g.+blog+post+explaining+how+you+use+Rust%29%0D%0A-+Organization+contact%3A+%28name+and+email.+we+may+contact+you+when+updating+this+page.+alternately+you+may+email+this+information+to+user-logos%40rust-lang.org+and+it+will+be+kept+secret%29.%0D%0A

### Rust, improved

Of course, Rust itself hasn't been standing still. The focus in its first year
has been growing and polishing its ecosystem and tooling:

- **Ecosystem**. The standard library has steadily expanded, with growth focused
  on [filesystem access], [networking], [time], and [collections] APIs&mdash;and
  dramatically better documentation coverage.  There's good support for working
  with C libraries via the [libc], [winapi], and [gcc] crates. And new libraries
  for [low-level async io][mio], [easy parallelism][rayon],
  [lock-free data structures][crossbeam],
  [Rails-like object-relational mapping][diesel], [regular expressions][regex],
  and several [parsing][nom] [libraries][lalrpop], including [html5ever], a
  unique HTML5 parser that leverages Rust's macro system to make the code
  resemble the spec as closely as possible. These are just scratching the
  surface, of course, and ecosystem growth, curation and
  coherence&mdash;particularly around async IO and the web stack&mdash;will
  continue to be a major focus in the coming year.

[filesystem access]: http://static.rust-lang.org/doc/master/std/fs/index.html
[networking]: http://static.rust-lang.org/doc/master/std/net/index.html
[time]: http://static.rust-lang.org/doc/master/std/time/index.html
[collections]: http://static.rust-lang.org/doc/master/std/collections/index.html
[libc]: https://github.com/rust-lang/libc
[winapi]: https://github.com/retep998/winapi-rs
[gcc]: https://github.com/alexcrichton/gcc-rs
[mio]: https://github.com/carllerche/mio/
[rayon]: http://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/
[crossbeam]: http://aturon.github.io/blog/2015/08/27/epoch/
[regex]: https://github.com/rust-lang-nursery/regex
[diesel]: http://diesel.rs/
[nom]: https://github.com/Geal/nom
[lalrpop]: http://smallcultfollowing.com/babysteps/blog/2015/09/14/lalrpop/
[html5ever]: https://kmcallister.github.io/talks/rust/2014-rust-macros/slides.html

- **Platforms and targets**. Rust's memory footprint is not much bigger than C's, which
  makes it ideal for using in all kinds of places. Over the last year, Rust
  gained the ability to work directly with the native [MSVC toolchain] on
  Windows, to target [musl][musl] (thereby creating a binary that can be used with
  zero dependencies on *any* variety of Linux), to target Android and ARM
  devices, and [many more platforms][platforms]. The new [rustup tool] makes it
  a breeze to manage and compile to these various targets. As of
  [Rust 1.6][no_std], you can use Rust without its full standard library,
  limiting to a core library that does not require any OS services (and hence is
  suitable for [writing OSes](http://os.phil-opp.com/)
  [in Rust][intermezzos]). Finally, there are an increasing number of libraries
  for embedding Rust code into other contexts, like [node.js][neon],
  [Ruby][helix] and [Go][rure-go].

[MSVC toolchain]: https://github.com/rust-lang/rust/pull/25350
[MUSL]: https://www.musl-libc.org/
[platforms]: https://forge.rust-lang.org/platform-support.html
[rustup tool]: http://blog.rust-lang.org/2016/05/13/rustup.html
[no_std]: http://blog.rust-lang.org/2016/01/21/Rust-1.6.html
[intermezzos]: https://intermezzos.github.io/
[neon]: http://calculist.org/blog/2015/12/23/neon-node-rust/
[helix]: http://blog.skylight.io/introducing-helix/
[rure-go]: https://github.com/BurntSushi/rure-go

- **Tools**. Because Rust looks just like C on the outside, it's instantly
  usable with a wide range of existing tools; it works out of the box with
  [lldb], [gdb], [perf], [valgrind], [callgrind], and many, many more.  Our
  focus has been to [enrich the experience] for these tools by adding
  [Rust-specific hooks][gdb hooks] and [workflows][cargo profile]. Another major
  priority is providing full IDE support, in part by providing daemonized
  services from the compiler; we made [good progress][IDEs] on that front this
  year, and thanks to the [Racer] project, [numerous IDE plugins] are already
  providing some semantic support for Rust. At the same time, the [rustfmt] code
  formatting tool has matured to the point that the Rust community is ready to
  produce an [official style].  And the beating heart of Rust's workflow,
  [Cargo], gained numerous abilities this year, most notably the
  [install subcommand].

[lldb]: http://lldb.llvm.org/
[gdb]: https://www.gnu.org/software/gdb/
[perf]: https://perf.wiki.kernel.org/index.php/Main_Page
[valgrind]: http://valgrind.org/
[callgrind]: https://kcachegrind.github.io/html/Home.html
[enrich the experience]: https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html
[gdb hooks]: https://sourceware.org/ml/gdb-patches/2016-04/msg00570.html
[cargo profile]: http://www.suchin.co/2016/05/11/Introducing-Cargo-Profiler/
[IDEs]: https://www.rust-lang.org/ides.html
[Racer]: https://github.com/phildawes/racer
[numerous IDE plugins]: https://areweideyet.com/
[rustfmt]: https://github.com/rust-lang-nursery/rustfmt
[official style]: https://github.com/rust-lang/rfcs/pull/1607
[Cargo]: http://blog.rust-lang.org/2016/05/05/cargo-pillars.html
[install subcommand]: http://blog.rust-lang.org/2015/12/10/Rust-1.5.html

- **Compiler**. We've seen some [across-the-board improvements] to compile
  times, and now offer [parallelized code generation][parallel codegen] for
  further speedups. But the biggest wins will come from the ongoing work on
  [incremental compilation], which will minimize the amount of work the needed
  when recompiling code after editing it. A vital step here was the move to a
  [custom intermediate representation][MIR], which has many other benefits as
  well. Another focus has been errors, including
  [detailed explanations of most errors][error index], and ongoing work to
  [improve the "at a glance" readability of errors][error format]. Expect to
  hear more on both fronts soon.

[across-the-board improvements]: http://blog.rust-lang.org/2015/06/25/Rust-1.1.html
[parallel codegen]: http://blog.rust-lang.org/2015/08/06/Rust-1.2.html
[incremental compilation]: https://github.com/rust-lang/rfcs/pull/1298
[MIR]: http://blog.rust-lang.org/2016/04/19/MIR.html
[error index]: https://doc.rust-lang.org/error-index.html
[error format]: https://internals.rust-lang.org/t/new-error-format/3438

- **Core language**. We've kept one list purposefully short this year: growth in
  the core language. While we have some important features in the pipeline (like
  [improved error handling], [more flexible borrowing rules] and
  [specialization]), [Rust users] by and large are happy with the core language
  and prefer the community to focus on the ecosystem and tooling.

[Rust users]: https://internals.rust-lang.org/t/production-user-research-summary/2530
[improved error handling]: https://github.com/rust-lang/rfcs/pull/243
[more flexible borrowing rules]: http://smallcultfollowing.com/babysteps/blog/2016/04/27/non-lexical-lifetimes-introduction/
[specialization]: https://github.com/rust-lang/rfcs/pull/1210

There's a lot more to say about what's happened and what's coming up in the Rust
world&mdash;over the coming months, we'll be using this blog to say it.

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

And that's no surprise. From a personal perspective, the best part about working
with Rust is its [community]. It's hard to explain quite what it's like to be
part of this group, but two things stand out. First, its sheer *energy*: so much
happens in any given week that [This Week in Rust] is a vital resource for
anyone hoping to keep up. Second, its *welcoming spirit*. Rust's core message is
one of empowerment&mdash;you can [fearlessly] write safe, low-level systems
code&mdash;and that's reflected in the community. We're all here to learn how to
be better programmers, and support each other in doing so.

[fearlessly]: http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
[This Week in Rust]: https://this-week-in-rust.org/
[community]: https://www.rust-lang.org/community.html

There's never been a better time to get started with Rust, whether through
attending a local [meetup], saying hello in the [users forum], watching
[a talk], or reading [the book]. No matter how you find your way in, we'll be
glad to have you.

[users forum]: https://users.rust-lang.org/
[a talk]: http://www.infoq.com/presentations/rust-thread-safety
[the book]: https://doc.rust-lang.org/book/

Happy birthday, Rust!
