---
layout: post
title: "Incremental Compilation: Making a Comeback"
author: Felix Klock
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

On May 10th, we [released Rust 1.52.1][1.52.1 blog post], an emergency point
release working around incremental compiler bugs that had been promoted into
compiler errors. This post provides a status update on what we have done since
then, and ends by recommending Rust developers try out the 1.53-beta release:
Doing so benefits everyone, especially you.

The changes in 1.52.1 over 1.52.0 were:
 1. We made incremental compilation
something that developers had to opt into, via a `RUSTC_FORCE_INCREMENTAL`
environment variable, so that developers would not encounter the compiler error by accident, and
 2. We improved the
diagnostic messages for the newly added error, so that developers who *did* opt into using incremental compilation could effectively respond
if they ever encountered the error.

The [1.52.1 blog post][] explains in more detail what the bugs were and why we
took the step of making incremental-compilation an opt-in feature for that
release. The short version of the story is: A new internal "fingerprint" check
had revealed lots of (old) inconsistencies within the incremental-compilation
infrastructure, and at least one of those inconsistencies could lead to
miscompilations.

[1.52.1 blog post]: https://blog.rust-lang.org/2021/05/10/Rust-1.52.1.html

## Incremental Progress

Since then, we have made a lot of progress addressing all of these
inconsistencies. The internal "fingerprint" check, combined with all the great
bug reports supplied by Rust developers like you, gave us the information we
needed to fix the serious bugs related to this issue.

If you want to get more details about those bugs, and how each has been fixed, I
recommend seeing the summary [issue #84970][].

[issue #84970]: https://github.com/rust-lang/rust/issues/84970

I am proud to report that we have now backported the relevant fixes to the 1.53
beta channel. Therefore, starting with the 1.53.0 release, we will resume
providing incremental-compilation support *without* requiring use of the
`RUSTC_FORCE_INCREMENTAL` environment variable.

## 1.53-beta: Not Just For βreakfast Anymore

If you or your team *need* incremental compilation, and thus turned on the
`RUSTC_FORCE_INCREMENTAL=1` environment setting after the [1.52.1 blog post][],
you will likely benefit from all of these fixes that have landed for the 1.53
release. So, we recommend that you, our power users, try switching to the
1.53-beta release of Rust.

If you *didn't* turn on `RUSTC_FORCE_INCREMENTAL=1`, you will nonetheless likely
benefit from using the 1.53-beta release over 1.52.1, simply because the
1.53-beta restores the status quo of having `dev` or `test` [profiles][] default
to enabling incremental compilation. So, we recommend that you, our everyday
users, also try switching to the 1.53-beta release of Rust.

[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html

If you are someone who does not believe in the [law of the excluded middle][],
but who *does* care about Rust's success, here is my final pitch to you: we, the
Rust compiler team, want as many people as possible to use the 1.53 beta release
now, so that we find out about any previously unidentified "fingerprint"
problems well before the 1.53.0 stable release comes out on June 17th. So, we
recommend that you, the Rust enthusiasts, try switching to the 1.53-beta release
of Rust. (And, of course, file issues if you see any bugs!)

[law of the excluded middle]: https://en.wikipedia.org/wiki/Law_of_excluded_middle

## βetta Than Evvah

To update to the beta:

 * If you are using `rustup` (recommended): run  `rustup update beta` to get the
   latest `1.53-beta` with all of these great fixes, and then use [`rustup default beta`][]
   (or [`rustup override set beta`][] or a [toolchain file override][], depending on your
   project and development needs).

[`rustup default beta`]: https://rust-lang.github.io/rustup/overrides.html#default-toolchain
[`rustup override set beta`]: https://rust-lang.github.io/rustup/overrides.html#directory-overrides
[toolchain file override]: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file

 * [Other installation methods][], such as via your platform's package
   manager, are also available. In particular, you can download and run a
   [standalone installer][] of the beta release for your target.

[Other installation methods]: https://forge.rust-lang.org/infra/other-installation-methods.html
[standalone installer]: https://forge.rust-lang.org/infra/other-installation-methods.html#standalone

As we said in the [1.52.1 blog post][], we continue our work to improve the
stability of our releases. We value stability; but we also know features like
incremental compilation are important to our developers, and crucial to the
long-term success of Rust.

We hope you are all as pleased as we are to know that incremental compilation
will be coming back soon, and stronger than ever.
