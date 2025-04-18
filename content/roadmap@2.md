+++
path = "2019/04/23/roadmap"
title = "Rust's 2019 roadmap"
authors = ["The Rust Core Team"]
aliases = ["2019/04/23/roadmap.html"]
+++

Each year the Rust community [comes together][roadmap-process] to set out a
roadmap. This year, in addition to the [survey], we put out a [call for blog
posts][blog-2019] in December, which resulted in [73 blog posts][read-rust]
written over the span of a few weeks. The end result is the recently-merged
[2019 roadmap RFC][rfc]. To get all of the details, please give it a read,
but this post lays out some of the highlights.

[roadmap-process]: https://github.com/rust-lang/rfcs/pull/1728
[survey]: https://blog.rust-lang.org/2018/11/27/Rust-survey-2018.html
[blog-2019]: https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html
[read-rust]: https://readrust.net/rust-2019/
[rfc]: https://github.com/rust-lang/rfcs/blob/master/text/2657-roadmap-2019.md

# The theme: Maturity

In short, 2019 will be a year of *rejuvenation* and *maturation* for the Rust
project. We shipped a lot of stuff last year, and grew a lot. Now it's time to
take a step back, take stock, and prepare for the future.

The work we have planned for this year falls into three major categories:

* Governance: improving how the project is run
* Finish long-standing requests: closing out work we've started but never finished
* Polish: improving the overall quality of the language and tooling

## Governance

Over the last three years, the Rust project has grown a lot. Rust used to have a core team of 8 members. When we added sub-teams in 2015, we grew to 23 members. We've now grown to over 100 — that's bigger than many companies! And of course, looking beyond the teams, the size of the Rust community as a whole has grown tremendously as well. As a result of this growth, we've found that the processes which served us well when we were a smaller project are starting to feel some strain.

Many of the teams have announced plans to review and revamp their processes so as to scale better. Often this can be as simple as taking the time to write down things that previously were understood only informally — sometimes it means establishing new structures. 

Because of this widespread interest in governance, we've also created a new [**Governance Working Group**][gov]. This group is going to be devoted to working with each team to hone its governance structure and to help pass lessons and strategies between teams.

[gov]: https://internals.rust-lang.org/t/governance-working-group-announcement/9637

Additionally, the RFC process has been a great boon for Rust, but as we've grown, there have been times where it didn't work so well too.
We may look at revisions to the process this year.

## Long-standing requests

There are a number of exciting initiatives that have been sitting in a limbo
state — the majority of the design is done, but there are some lingering
complications that we haven't had time to work out. This year we hope to take
a fresh look at some of these problems and try hard to resolve those
lingering problems.

Examples include:

- The Cargo team and custom registries
- The Language team is taking a look at async/await, specialization, const generics, and generic associated types
- The Libs team wants to finish custom allocators

## Polish

Finally, the last few years have also seen a lot of foundational work. The
compiler, for example, was massively refactored to support incremental
compilation and to be better prepared for IDEs. Now that we've got these
pieces in place, we want to do the “polish” work that really makes for a
great experience.

Examples:

- Compile times and IDE support
- Polishing the specification of the language by improving [the reference] and laying out [the unsafe code guidelines]
- The WebAssembly WG's work this year includes polishing our wasm support, for example, debugging

[the reference]: https://doc.rust-lang.org/stable/reference/
[the unsafe code guidelines]: https://github.com/rust-lang/unsafe-code-guidelines

## Conclusion

This post only covered a few examples of the plans we've been making. [If you'd like to see the full details, take a look at the RFC itself.][rfc]

Here's to a great 2019 for Rust!
