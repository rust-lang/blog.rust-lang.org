---
layout: post
title: "Completing the transition to the new borrow checker"
author: Niko Matsakis
---

For most of 2018, we've been issuing warnings about various bugs in the
borrow checker that we plan to fix -- about two months ago, in the
current Rust nightly, those warnings became **hard errors**. In about
two weeks, when the nightly branches to become beta, those hard errors
will be in the beta build, and they will eventually hit stable on
December 19th, as part of Rust 1.40.0. **If you're testing with
Nightly, you should be all set -- but otherwise, you may want to go
and check to make sure your code still builds. If not, we have advice
for fixing common problems below.**

### Background: the non-lexical lifetime transition

When we [released Rust 2018 in Rust 1.31][2018], it included a new
version of the borrow checker, one that implemented ["non-lexical
lifetimes"][nll]. This new borrow checker did a much more precise
analysis than the original, allowing us to eliminate a lot of
unnecessary errors and make Rust easier to use. I think most everyone
who was using Rust 2015 can attest that this shift was a big
improvement.

### The new borrow checker also fixed a lot of bugs

What is perhaps less well understood is that the new borrow checker
implementation *also* fixed a lot of bugs. In other words, the new
borrow checker did not just accept more programs -- **it also rejected
some programs that were only accepted in the first place due to memory
unsafety bugs in the old borrow checker!**

[2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[nll]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes
[MIR]: https://blog.rust-lang.org/2016/04/19/MIR.html

### Until recently, those fixed bugs produced warnings, not errors

Obviously, we don't want to accept programs that could undermine
Rust's safety guarantees. At the same time, as part of our commitment
to stability, we try to avoid making sudden bug fixes that will affect
a lot of code. Whenever possible, we prefer to "phase in" those
changes gradually. We usually begin with "Future Compatibility
Warnings", for example, before moving those warnings to hard errors
(sometimes a small bit at a time). Since the bug fixes to the borrow
checker affected a lot of crates, we knew we needed a warning period
before we could make them into hard errors.

To implement this warning period, we kept two copies of the borrow
checker around (this is a trick we use quite frequently, actually).
The new checker ran first. If it found errors, we didn't report them
directly: instead, we ran the old checker in order to see if the crate
*used* to compile before. If so, we reported the errors as Future
Compatibility Warnings, since we were changing something that used to
compile into errors.

### All good things must come to an end; and bad ones, too

Over time we have been slowly transitioning those future compatibility
warnings into errors, a bit at a time. About two months ago, we
decided that the time had come to finish the job. So, over the course
of two PRs, we [converted all remaining warnings to errors][a] and
then [removed the old borrow checker implementation][b].

[a]: https://github.com/rust-lang/rust/pull/63565
[b]: https://github.com/rust-lang/rust/pull/64790

### What this means for you

**If you are testing your package with nightly, then you should be
fine.** In fact, even if you build on stable, we always recommend that
you test your builds in CI with the nightly build, so that you can
identify upcoming issues early and report them to us.

**Otherwise, you may want to check your dependencies.** When we
decided to remove the old borrow checker, we also analyzed which
crates would stop compiling. For anything that seemed to be widely
used, we made sure that there were newer versions of that crate
available that *do* compile (for the most part, this had all already
happened during the warning period). But if you have those older
versions in your `Cargo.lock` file, and you are only using stable
builds, then you may find that your code no longer builds once 1.40.0
is released -- you will have to upgrade the dependency.

The most common crates that were affected are the following:

* `url` version 1.7.0 -- you can upgrade to 1.7.2, though you'd be better off upgrading to 2.1.0
* `nalgebra` version 0.16.13 -- you can upgrade to 0.16.14, though you'd be better off upgrading to 0.19.0
* `rusttype` version 0.2.0 to 0.2.3 -- you can upgrade to 0.2.4, though you'd be better upgrading to 0.8.1

You can find out which crates you rely upon using the [cargo-tree] command. If you find
that you *do* rely (say) on `url` 1.7.0, you can upgrade to 1.7.2 by executing:

```bash
cargo update --package url --precise 1.7.2
```

[cargo-tree]: https://crates.io/crates/cargo-tree

### Want to learn more?

If you'd like to learn more about the kinds of bugs that were fixed --
or if you are seeing errors in your code that you need to fix -- take
a look at this [excellent blog post by Felix Klock][nllpost], which
goes into great detail.

[nllpost]: http://blog.pnkfx.org/blog/2019/06/26/breaking-news-non-lexical-lifetimes-arrives-for-everyone/
