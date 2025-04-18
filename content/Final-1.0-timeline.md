+++
path = "2015/02/13/Final-1.0-timeline"
title = "Rust 1.0: status report and final timeline"
authors = ["The Rust Core Team"]
aliases = ["2015/02/13/Final-1.0-timeline.html"]
+++

It's been five weeks since we released Rust 1.0-alpha! Before this
release cycle finishes next week, we want to give a status report and
update on the road to 1.0 final.

**TL;DR: Rust 1.0 final will be released by May 15, 2015**

## What is the overall timeline?

Based on the progress in this release cycle, we are now comfortable
committing to a precise release schedule for 1.0:

* Rust 1.0-alpha2 -- Feb 20
* All 1.0 modules stable on nightly -- around Mar 9
* Rust 1.0-beta -- Apr 3
* Rust 1.0 -- May 15

This schedule differs from the
[previous one](https://blog.rust-lang.org/2014/12/12/1.0-Timeline.html)
by nailing down an exact set of release cycles. It also opts for a
second alpha release and only a single beta release.

The main reason for calling the next release alpha2 rather than beta1
is that new path and IO APIs have only recently landed, and we would
like more time to collect feedback before marking them stable. More
details are below.

**Update:** An earlier version of this post listed Mar 31 as the 1.0-beta
release date, due to a miscalculation.  The correct date is Apr 3, exactly six
weeks after alpha2 and six weeks before 1.0.

## What's shipping in alpha2?

We've managed to land almost all of the features
[previously expected](https://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html)
for this cycle.

**The big headline here is that all major API revisions are
finished**: path and IO reform have landed. At this point, all modules
shipping for 1.0 are in what we expect to be their final form, modulo
minor tweaks during the alpha2 cycle.

Other highlights are as follows:

* **Closures**: Rust now supports
  [full capture-clause inference](https://github.com/rust-lang/rfcs/blob/master/text/0231-upvar-capture-inference.md)
  and has deprecated the temporary `|:|` notation, making closures
  much more ergonomic to use.

* **Destructors**: New
  [destructor rules](https://github.com/rust-lang/rfcs/pull/769)
  landed, obviating the need for `#[unsafe destructor]`.

* **Path reform**: The `path` module has been completely
  [redesigned](https://github.com/rust-lang/rfcs/pull/474) to resolve
  a number of semantic and ergonomic problems with the old module, and
  to take advantage of DST.

* **IO reform**: The `io` system has been
  [thoroughly revised](https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md)
  to improve robustness and cross-platform behavior, and to eschew
  ambitious high-level abstractions over the system. While almost all
  of the APIs are affected by this change, the changes move toward a
  much more conservative and consistent design.

* **Deref coercions**: A
  [new coercion](https://github.com/rust-lang/rfcs/pull/241) will
  follow smart pointers, so that you can pass `&Vec<T>` where `&[T]`
  is wanted, or `&Arc<T>` where `&T` is wanted. This removes most need
  for explicit slicing or the dreaded "cross-borrowing" `&*`, and
  means that `&` can be thought of as a general "borrow" operator.

* **Feature staging**: Rust now has a notion of
  [named API *features*](https://github.com/rust-lang/rfcs/pull/475)
  akin to language features, which is how we will manage API
  stabilization going forward. These named features make it easier to
  manage progress in `std`, and make it plausible to detect the
  minimum version of Rust needed for a crate.

* **For loops**: The new `IntoIterator` trait is now available and
  used for `for` loops, making it possible to write `for x in &vec`
  rather than `for x in vec.iter()`.

* **Range notation**: We have
  [finalized range notation](https://github.com/rust-lang/rfcs/pull/702),
  introducing `..` for "full ranges", which will make APIs like
  `collection.remove(..)` possible in the future.

* **Trait system**: New coherence rules were
  [finalized](https://internals.rust-lang.org/t/orphan-rules/1322),
  providing both flexibility and soundness for trait implementations.

* **Overflow semantics**: After a long debate, the final integer
  overflow semantics has
  [been decided](https://github.com/rust-lang/rfcs/pull/560) and is
  expected to land for alpha2. This change is expected to make it much
  easier to find over/underflow bugs when used in conjunction with
  fuzzing, etc.

* **Associated types**: many compiler bugs around associated types
  have been fixed, making them usable at large scale.

Some other changes have not landed at the time of writing but are
expected for alpha2:
[variance for type parameters](https://github.com/rust-lang/rfcs/pull/738),
[Send changes](https://github.com/rust-lang/rfcs/pull/458), and
[the great integer audit](https://github.com/rust-lang/rust/issues/22240).

Complete details will be available in the release notes next week.

## Why another alpha?

The main reason is that we want to leave recently-landed APIs, like IO
and path, unstable for a few more weeks while we collect feedback --
but the beta release is intended to disallow use of unstable features.

In more detail, Rust is drawing a
[difference between alpha and beta](https://blog.rust-lang.org/2014/12/12/1.0-Timeline.html)
connected with our
[stability system](https://blog.rust-lang.org/2014/10/30/Stability.html).
In alpha releases, it's possible to opt-in to unstable features, but
after beta, this will be possible only when using nightly builds. The
beta release will mark the point when a substantial portion of the
community can move off of nightlies.

As mentioned above, we have landed all of the major API revisions
needed for the 1.0 release, including path and IO reform. However,
some of these revisions landed relatively late in the cycle, and as a
community we don't have enough experience with the revised APIs to
declare them stable yet. Note that the API changes are, with a
couple exceptions, very conservative: they generally move us in the
direction of existing, successful libraries.

By producing 1.0-alpha2, we leave open a longer window for tweaks to
these APIs before declaring them stable. That window will close around
March 9.

### Is there risk of slippage by not moving to beta now?

It seems unlikely. Essentially all of the language and library
features needed for 1.0 have already landed, meaning that we will have
*12 weeks* of time to polish between alpha2 and 1.0 final.

## What will happen before 1.0?

All features that are required for shipping 1.0 have now landed. What
remains is polish, performance improvements, bugfixing, documentation
-- and gaining enough confidence in recently revised APIs to mark them
`#[stable]`.

The alpha2 release will officially deprecate (but leave available) the
old path and IO APIs. The new APIs are scheduled to be stabilized
by March 9. **Please try out these new APIs and help uncover
problems!**

After the March 9 deadline, it should be possible for substantial
crates to work with "stable Rust", i.e. without any use of
`#[feature]`. Between then and the beta release, we hope to work
directly with authors of crates.io packages to help move code to
stable Rust, and to uncover any gaps in stabilization.

By beta, we hope that a substantial part of the ecosystem will be off
of nightlies and on to stable releases. Getting there will require a
community-wide push toward stabilization, which we're coordinating via
[discuss](https://users.rust-lang.org/t/using-unstable-apis-tell-us-about-it/157/26)
-- if you haven't, please drop by and tell us the key unstable APIs
you're using.
