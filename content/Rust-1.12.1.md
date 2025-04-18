+++
path = "2016/10/20/Rust-1.12.1"
title = "Announcing Rust 1.12.1"
authors = ["The Rust Core Team"]
aliases = [
    "2016/10/20/Rust-1.12.1.html",
    "releases/1.12.1",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.12.1. Rust is a
systems programming language with a focus on reliability, performance, and
concurrency.

As always, you can [install Rust 1.12.1][install] from the appropriate page on our
website, or install via [rustup] with `rustup update stable`.

[install]: https://www.rust-lang.org/install.html

### What's in 1.12.1 stable

Wait... one-point-twelve-point... one?

In [the release announcement for 1.12][one-twelve] a few weeks ago, we said:

[one-twelve]: https://blog.rust-lang.org/2016/09/29/Rust-1.12.html

> The release of 1.12 might be one of the most significant Rust releases since
> 1.0.

It was true. One of the biggest changes was turning on a large compiler
refactoring, [MIR], which re-architects the internals of the compiler. The
overall process went like this:

[MIR]: https://blog.rust-lang.org/2016/04/19/MIR.html

* Initial MIR support landed in nightlies back in Rust 1.6.
* While work was being done, a flag, `--enable-orbit`, was added so that
  people working on the compiler could try it out.
* Back [in October], we would always attempt to build MIR, even though it
  was not being used.
* A flag was added, `-Z orbit`, to allow users on nightly to try and use MIR
  rather than the traditional compilation step ('trans').
* After substantial testing over months and months, for Rust 1.12, [we enabled
  MIR by default].
* In Rust 1.13, [MIR will be the only option].

[in October]: https://github.com/rust-lang/rust/pull/28748
[we enabled MIR by default]: https://github.com/rust-lang/rust/pull/34096
[MIR will be the only option]: https://github.com/rust-lang/rust/pull/35764

A change of this magnitude is huge, and important. So it's also important to do
it right, and do it carefully. This is why this process took so long; we
regularly tested the compiler against every crate on crates.io, we asked people
to try out `-Z orbit` on their private code, and after six weeks of beta, no
significant problems appeared. So we made the decision to keep it on by default
in 1.12.

But large changes still have an element of risk, even though we tried to reduce
that risk as much as possible. And so, after release, 1.12 saw a fair number of
regressions that we hadn't detected in our testing. Not all of them are
directly MIR related, but when you change the compiler internals so much, it's
bound to ripple outward through everything.

### Why make a point release?

Now, given that we have a six-week release cycle, and we're halfway towards
Rust 1.13, you may wonder why we're choosing to cut a patch version of Rust
1.12 rather than telling users to just wait for the next release. We have
previously said something like "point releases should only happen in extreme
situations, such as a security vulnerability in the standard library."

The Rust team cares deeply about the stability of Rust, and about our users'
experience with it. We could have told you all to wait, but we want you to know
how seriously we take this stuff. We think it's worth it to demonstrate our
commitment to you by putting in the work of making a point release in this
situation.

Furthermore, given that this is not security related, it's a good time to
practice actually cutting a point release. We've never done it before, and the
release process is [semi-automated] but still not completely so. Having a point
release in the world will also [shake out any bugs][bugs] in dealing with point
releases in other tooling as well, like [rustup]. Making sure that this all goes
smoothly and getting some practice going through the motions will be useful if
we ever need to cut some sort of *emergency* point release due to a security
advisory or anything else.

[semi-automated]: https://forge.rust-lang.org/release-process.html
[rustup]: https://www.rustup.rs/
[bugs]: https://github.com/rust-lang/rust/pull/37173#issuecomment-253938822

This is the first Rust point release since [Rust 0.3.1], all the way back in
2012, and marks 72 weeks since Rust 1.0, when we established our six week
release cadence along with a commitment to aggressive stability
guarantees. While we're disappointed that 1.12 had these regressions, we're
really proud of Rust's stability and will to continue expanding our efforts to
ensure that it's a platform you can rely on. We want Rust to be the most
reliable programming platform in the world.

[Rust 0.3.1]: https://mail.mozilla.org/pipermail/rust-dev/2012-July/002152.html

### A note about testing on beta

One thing that you, as a user of Rust, can do to help us fix these issues
sooner: test your code against the beta channel! Every beta release is a
release candidate for the next stable release, so for the cost of an extra
build in CI, you can help us know if there's going to be some sort of problem
before it hits a stable release! It's really easy. For example, on
[Travis](https://travis-ci.org/), you can use this as your `.travis.yml`:

```yaml
language: rust
rust:
  - stable
  - beta
```

And you'll test against both. Furthermore, if you'd like to make it so that any
beta failure doesn't fail your own build, do this:

```yaml
matrix:
  allow_failures:
    - rust: beta
```

The beta build may go red, but your build will stay green.

Most other CI systems, such as [AppVeyor](https://www.appveyor.com/), should
support [something
similar](https://www.appveyor.com/docs/build-configuration/#allow-failing-jobs).
Check the documentation for your specific continuous integration product for
full details.

### Full details

There were nine issues fixed in 1.12.1, and all of those fixes have been
backported to 1.13 beta as well. 

* [ICE: 'rustc' panicked at 'assertion failed: concrete_substs.is_normalized_for_trans()' #36381][36381]
* [Confusion with double negation and booleans][36856]
* [rustc 1.12.0 fails with SIGSEGV in release mode (syn crate 0.8.0)][36875]
* [Rustc 1.12.0 Windows build of `ethcore` crate fails with LLVM error][36924]
* [1.12.0: High memory usage when linking in release mode with debug info][36926]
* [Corrupted memory after updated to 1.12][36936]
* ["Let NullaryConstructor = something;" causes internal compiler error: "tried to overwrite interned AdtDef"][37026]
* [Fix ICE: inject bitcast if types mismatch for invokes/calls/stores][37112]
* [debuginfo: Handle spread_arg case in MIR-trans in a more stable way.][37153]

[36381]: https://github.com/rust-lang/rust/issues/36381
[36856]: https://github.com/rust-lang/rust/issues/36856
[36875]: https://github.com/rust-lang/rust/issues/36875
[36924]: https://github.com/rust-lang/rust/issues/36924
[36926]: https://github.com/rust-lang/rust/issues/36926
[36936]: https://github.com/rust-lang/rust/issues/36936
[37026]: https://github.com/rust-lang/rust/issues/37026
[37112]: https://github.com/rust-lang/rust/issues/37112
[37153]: https://github.com/rust-lang/rust/issues/37153

In addition, there were four more regressions that we decided not to include
in 1.12.1 for various reasons, but we'll be working on fixing those as soon
as possible as well.

* [ICE, possibly related to associated types of associated types?][36325]
* [Compilation of a crate using a large static map fails on latest i686-pc-windows-gnu Beta][36799]
* [Regression: "no method found" error when calling same method twice, with HRTB impl][37154]
* [ICE: fictitious type sizing_type_of][37109]

[36325]: https://github.com/rust-lang/rust/issues/36325
[36799]: https://github.com/rust-lang/rust/issues/36799
[37154]: https://github.com/rust-lang/rust/issues/37154
[37109]: https://github.com/rust-lang/rust/issues/37109

You can see the full diff from 1.12.0 to 1.12.1
[here](https://github.com/rust-lang/rust/pull/37173).
