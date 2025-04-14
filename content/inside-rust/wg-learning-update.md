+++
path = "inside-rust/2019/12/20/wg-learning-update"
title = "An Update from WG-Learning"
authors = ["mark-i-m"]
aliases = ["inside-rust/2019/12/20/wg-learning-update.html"]

[extra]
team = "the Rustc Dev Guide Working Group"
team_url = "https://www.rust-lang.org/governance/teams/compiler#wg-rustc-dev-guide"
+++

# An update from WG-Learning

In our last post in [October][oct] we gave an overview what the Learning WG is
and what we are doing. We have made a lot of progress since that post, and we
have also held a meeting to decide what to work on next. So let's dig in...

## Work completed

We mentioned before that we are in the process of producing [rustc-dev-guide][rg]
chapters from the "Compiler Lecture Series" videos. The goal is to try to
produce guide chapters that are approachable for beginners and give a good
foundation for exploring and hacking on the compiler.

Recently, we merge a [chapter][salsach] on [`salsa`][salsa] by `@Karrq`.
`salsa` is a crate that makes incremental computation easier. While it is not
used in `rustc` itself, it is heavily inspired by it, and it is used by
[`rust-analyzer`][ra].

We also collectively have been working on a chapter about [`ty::Ty`][ty] and
the way that rustc represents types internally. You can find that PR
[here][typr]. This has been a big effort for a few months now, and we are excited
to have this new chapter in the guide.

## What's next?

We just had a [planning meeting][meeting] to discuss what to work on next. The
guide has some long-standing holes and shortcomings that we would like to address.

Specifically, the Learning WG decided that we wanted to pursue the following goals next:
- Write an overview chapter
- Gather source material for chapters on monomorphization and LLVM

### Overview chapter

One of the challenges with big software systems is understanding how everything
fits together. We have seen this problem come up with the rustc-dev-guide; the chapters
tunnel down into a single part of the compiler, but it is hard to get a good
view of all the things that happen to a piece of code between lexing and linking.

We want to remedy this problem by creating an Overview chapter that walks
through some example from the beginning of the compiler to the end of the
compiler at a high level.  We plan to put this chapter at the beginning of
(part 2 of) the guide, so that it guide readers as to what part of the
compilation they are reading about in the subsequent chapters.

### Monomorphization, Codegen, LLVM

One of the biggest gaps in the guide currently is what happens to your code
after the MIR is produced. We have chapters on almost everything that happens
before that (though many of them are pretty slim), but we have almost nothing
after the MIR is produced, borrow checked, and optimized.

In particular, after the MIR is optimized, we need to [monomorphize][glos] it,
produce LLVM IR from it, call LLVM to produce executable code, and then link
everything to form a final binary object.

The Learning WG will work on collecting information to write chapters on these
topics.

## Getting involved

Did any of this sound interesting to you? We would love for you to join us! You
can the Learning WG on the [`t-compiler/wg-rustc-dev-guide`][zulip] stream on Zulip.
Feel free to stop by and ping us.


[oct]: https://blog.rust-lang.org/inside-rust/2019/10/28/rustc-learning-working-group-introduction.html
[rg]: https://rustc-dev-guide.rust-lang.org/
[salsa]: https://crates.io/crates/salsa
[salsach]: https://rustc-dev-guide.rust-lang.org/salsa.html
[ra]: https://github.com/rust-analyzer/rust-analyzer
[ty]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc/ty/type.Ty.html
[typr]: https://github.com/rust-lang/rustc-dev-guide/pull/530
[meeting]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide/topic/planning.20meeting
[glos]: https://rustc-dev-guide.rust-lang.org/appendix/glossary.html
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide
