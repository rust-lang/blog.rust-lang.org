+++
path = "2021/05/10/Rust-1.52.1"
title = "Announcing Rust 1.52.1"
authors = ["Felix Klock, Mark Rousskov"]
aliases = ["2021/05/10/Rust-1.52.1.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
release = true
+++

The Rust team has prepared a new release, 1.52.1, working around a bug in
incremental compilation which was made into a compiler error in 1.52.0. We
recommend all Rust users, including those currently using stable versions prior
to 1.52.0, upgrade to 1.52.1 or disable incremental compilation. Guidance on how
to do so is available below.

If you have a previous version of Rust installed via rustup, getting Rust
1.52.1 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install]
from the appropriate page on our website.

[install]: https://www.rust-lang.org/install.html

# Summary

This release works around broken builds on 1.52.0, which are caused by newly
added verification. The bugs this verification detects are present in all Rust
versions[^1], and can trigger miscompilations in incremental builds, so downgrading
to a prior stable version is not a fix.

Users are encouraged to upgrade to 1.52.1 or disable incremental in their local
environment if on a prior version: please see the [what you should do][part3]
section for details on how to do so.

Incremental compilation is off by default for release builds, so few
production builds should be affected (only for users who have opted in).

Miscompilations that can arise from the bugs in incremental compilation generate incorrect code in final
artifacts, essentially producing malformed binaries, which means that in theory
any behavior is possible. In practice we are currently only aware of one
particular known miscompilation, but bugs due to incremental are notoriously
hard to track down: users frequently simply rebuild after some light editing if
they see unexpected results from their binaries, and this often causes
sufficient recompilation to fix the bug(s).

This post is going to:

 1. Explain [what the errors look like][part0],
 1. Explain [what the check does][part1], at a high level,
 2. Explain [how the check is presenting itself][part2] in the Rust 1.52.0 release,
 3. Tell you [what you should do][part3] if you see an unstable fingerprint on your project,
 4. Describe our plans for [how the Rust project will address][part4] the problems discussed here.

[part0]: #what-does-the-error-look-like
[part1]: #what-are-fingerprints-why-are-we-checking-them
[part2]: #how-does-this-show-up
[part3]: #what-should-a-rust-programmer-do-in-response
[part4]: #what-is-the-rust-project-going-to-do-to-fix-this

## What does the error look like?

The error message looks something like this, with the key piece being the "found
unstable fingerprints" text.

```
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(4565771098143344972, 7869445775526300234))`,
  right: `Some(Fingerprint(14934403843752251060, 623484215826468126))`: found unstable fingerprints for <massive text describing rustc internals elided>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
```

This is the error caused by the internal consistency check, and as stated in the diagnostic, it yields an "Internal Compiler Error" (or ICE). In other words, it represents a bug in the internals of the Rust compiler itself. In *this* case, the ICE is revealing a bug in incremental compilation that predates the 1.52.0 release and could result in miscompilation if it had not been caught.

## What are fingerprints? Why are we checking them?

The Rust compiler has support for "incremental compilation", which has been described in a [2016 blog post][]. When incremental compilation is turned on, the compiler breaks the input source into pieces, and tracks how those input pieces influence the final build product. Then, when the inputs change, it detects this and reuses artifacts from previous builds, striving to expend effort solely on building the parts that need to respond to the changes to the input source code.

[2016 blog post]: https://blog.rust-lang.org/2016/09/08/incremental.html

Fingerprints are part of our architecture for detecting when inputs change. More specifically, a fingerprint (along with some other state to establish context) is a 128-bit value intended to uniquely identify internal values used within the compiler. Some compiler-internal results are stored on disk ("cached") between runs. Fingerprints are used to validate that a newly computed result is unchanged from the cached result. (More details about this are available in the [relevant chapter of the rustc dev guide][rustc-dev-guide-fingerprints].)

[rustc-dev-guide-fingerprints]: https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html#checking-query-results-for-changes-hashstable-and-fingerprints

The fingerprint stability check is a safeguard asserting internal consistency of
the fingerprints. Sometimes the compiler is forced to rerun a query, and expects
that the output is the same as from a prior incremental compilation session. The
newly enabled verification checks that the value is indeed as expected, rather
than assuming so. In some cases, due to bugs in the compiler's implementation,
this was not actually the case.

## History

We [initially added][pr-45867] these fingerprint checks as a tool to use when
developing rustc itself, back in 2017. It was solely provided via an unstable
`-Z` flag, only available to nightly and development builds.

More recently, in March, we encountered a [miscompilation][issue-82920] that led us to [turn on `verify-ich` by default][pr-83007]. The Rust compiler team decided it was better to catch fingerprint problems and abort compilation, rather than allow for potential miscompilations (and subsequent misbehavior) to sneak into Rust programmer's binaries.

[pr-45867]: https://github.com/rust-lang/rust/pull/45867
[issue-82920]: https://github.com/rust-lang/rust/issues/82920
[pr-83007]: https://github.com/rust-lang/rust/pull/83007

When we first turned on the fingerprint checks by default, there was a steady
stream of issues filed by users of the nightly (and beta) toolchains, and steady
progress has been made on identifying fixes, a number of which have already
landed.

In the past week, we had started [making plans][issue-84970] to improve the
user-experience, so that the diagnostic issued by the check would do a better
job of telling the programmer what to do in response. Unfortunately, this was
done under the assumption that the new verification would ship in 1.53, not
1.52.

[issue-84970]: https://github.com/rust-lang/rust/issues/84970

It turns out `verify-ich` was turned on in version 1.52.0, which was [released recently][].

[released recently]: /2021/05/06/Rust-1.52.0/

Today's new release, 1.52.1, works around the breakage caused by the newly added
verification by temporarily changing the defaults in the Rust compiler to disable
incremental unless the user knowingly opts in.

## How does this show up

Essentially, for some crates, certain sequences of edit-compile cycles will cause `rustc` to hit the "unstable fingerprints" ICE. I showed one example at the start of this blog post.

Another recent example looks [like this](https://github.com/rust-lang/rust/issues/85039):

```
thread 'rustc' panicked at 'found unstable fingerprints for predicates_of(<massive text describing rustc internals elided>)', /rustc/.../compiler/rustc_query_system/src/query/plumbing.rs:593:5
```

They all arise from inconsistencies when comparing the incremental-compilation cache stored on disk against the values computed during a current `rustc` invocation, which means they all arise from using incremental compilation.

There are several ways that you may have incremental compilation turned on:

1. You may be building with the `dev` or `test` [profiles][] which default to having incremental compilation enabled.
2. You may have set the [environment variable][env-vars] `CARGO_INCREMENTAL=1`
3. You may have enabled the `build.incremental` [setting in your Cargo config][cargo-config]
4. You may have enabled the `incremental` [setting in your Cargo.toml][cargo-toml] for a given profile

[env-vars]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-reads
[cargo-config]: https://doc.rust-lang.org/cargo/reference/config.html#buildincremental
[cargo-toml]: https://doc.rust-lang.org/cargo/reference/profiles.html#incremental
[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html

If your project has not adjusted the defaults, then when running `cargo build
--release` or otherwise in the `release` profile configuration incremental is
disabled on all Rust versions[^1], and these issues should not affect your release
builds.

## What should a Rust programmer do in response

The Internal Compiler Error asks you to report a bug, and if you can do so, we still want that information. We *want* to know about the cases that are failing.

But regardless of whether or not you file a bug, the problem can be worked around on your end by either:

 1. upgrading to 1.52.1, if you have not yet done so (which will disable
    incremental for you), or
 2. deleting your incremental compilation cache (e.g. by running `cargo clean`), or
 3. forcing incremental compilation to be disabled, by setting `CARGO_INCREMENTAL=0` in your environment or `build.incremental` to `false` in the `config.toml`.

We recommend that users of 1.52.0 upgrade to 1.52.1, which disables incremental
compilation.

We do *not* recommend that users of 1.52.0 downgrade to an earlier version of Rust in response to this problem. As noted above, there is at least one instance of a silent [miscompilation][issue-82920] caused by incremental compilation that was not caught until we added the fingerprint checking.

If a user is willing to deal with the incremental verification ICE's, and wishes
to opt back into the 1.52.0 behavior, they may set `RUSTC_FORCE_INCREMENTAL` to
`1` in their environment. The Rust compiler will then respect the
`-Cincremental` option passed by Cargo, and things will work as before, though
with the added verification. Note that this flag does not enable incremental if
it has not already been separately enabled (whether by Cargo or otherwise).

If you are currently using a toolchain prior to 1.52.0, and wish to continue
doing so, we recommend that you disable incremental compilation to avoid hitting
silent miscompilations.

On all Rust builds since incremental has landed, it has been a major
improvement to compile times for many users, and has only improved over time. We
acknowledge that the workarounds presented here and recommendations are painful,
and will be working hard to ensure the situation is as temporary as possible.

## What is the Rust project going to do to fix this

### Short-term plan

We have issued 1.52.1 today which:

* Disables incremental compilation in the Rust compiler (unless asked for by a
  new environment variable, `RUSTC_FORCE_INCREMENTAL=1`).
* Improves diagnostic output for the new verification if incremental compilation is enabled,
  indicating how to work around the bugs by purging incremental state or
  disabling incremental.

This is intended to be a mitigation that helps the majority of Rust users have
an upgrade path to a safe Rust compiler which does not have the risk of
miscompiling their code, but also provide the option for users willing to deal
with the errors to do so.

We expect to continue to actively invest in fixing the bugs, and depending on
our confidence in the fixes, may issue a 1.52.2 point release which backports
those fixes to the stable channel. Users wishing to help us test can use the
nightly channel, and report bugs to rust-lang/rust with any ICEs they
are seeing.

We are also currently not planning to disable incremental on the beta channel,
but this decision has not been firmly committed to. A number of fixes are
available on 1.53 beta today, so users who wish to continue using incremental
may want to switch to that. Nightly will always have the latest in fixes, of
course.

### Long-term plan

The long-term plan is to fix the bugs! Incremental compilation is the only realistic way for the Rust compiler to be able to provide a fast edit-compile-run cycle for all of its programmers, and so we need to address [all of the issues][issue-list] that have been identified thus far via `verify-ich`. (There are 32 such issues as of this writing, though many are duplicates.)

We are actively investing in this, and a number of bugs have already been
identified and fixed. Depending on the state of the fixes, future stable
releases (1.53 and onwards) will likely re-enable incremental compilation.

[issue-list]: https://github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+unstable+fingerprints

The Rust teams will also be developing plans to ensure we have better tracking
systems in place in the future for bugs, both to prevent situations like this
from arising again, but also to further increase the stability of our releases
by tracking bugs more accurately as they propagate across channels.

[^1]: Since incremental was first enabled, which was in Rust 1.24.
