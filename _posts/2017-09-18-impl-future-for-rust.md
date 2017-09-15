---
layout: post
title: "impl Future for Rust"
author: Aaron Turon
description: "The Rust community is going to finish out its 2017 roadmap with a bang—and we want your help!"
---

The Rust community has been hard at work on our [2017 roadmap], but as we come
up on the final quarter of the year, we're going to kick it into high gear—and
we want you to join us!

[2017 roadmap]: https://github.com/rust-lang/rfcs/pull/1774

Our goals for the year are ambitious:

* [Rust should have a lower learning curve](https://github.com/rust-lang/rust-roadmap/issues/3).
* [Rust should have a pleasant edit-compile-debug cycle](https://github.com/rust-lang/rust-roadmap/issues/1).
* [Rust should provide a solid, but basic IDE experience](https://github.com/rust-lang/rust-roadmap/issues/2).
* [Rust should provide easy access to high quality crates](https://github.com/rust-lang/rust-roadmap/issues/9).
* [Rust should be well-equipped for writing robust, high-scale servers](https://github.com/rust-lang/rust-roadmap/issues/10).
* [Rust should have 1.0-level crates for essential tasks](https://github.com/rust-lang/rust-roadmap/issues/11).
* [Rust should integrate easily into large build systems](https://github.com/rust-lang/rust-roadmap/issues/12).
* [Rust's community should provide mentoring at all levels](https://github.com/rust-lang/rust-roadmap/issues/13)

**To finish off these goals, we intend to spend the rest of the year focused
purely on "implementation" work—which doesn't just mean code!** In particular, we
are effectively spinning down the [RFC process] for 2017, after having merged
[almost 90] RFCs this year!

[RFC process]: https://github.com/rust-lang/rfcs#rust-rfcs
[almost 90]: https://github.com/rust-lang/rfcs/pulls?utf8=%E2%9C%93&q=is%3Apr%20merged%3A%3E2017-01-01

So here's the plan. Each Rust team has put together several *working groups*
focused on a specific sub-area. Each WG has a leader who is responsible for
carving out and coordinating work, and a dedicated chat channel for getting
involved. We are working hard to divvy up work items into many shapes and sizes,
and to couple them with mentoring instructions and hands-on mentors. **So if
you've always wanted to contribute to Rust but weren't sure how, this is the
perfect opportunity for you.** Don't be shy—we want and need your help, and, as
per our roadmap, our aim is mentoring at *all* levels of experience.

## A few points of order

There are a few online venues for keeping in the loop with working group activity:

- There is a [dedicated Gitter community](https://gitter.im/rust-impl-period/)
  with channels for each working group, as well as
  a [global channel](https://gitter.im/rust-impl-period/Lobby) for talking about
  the process as a whole, or getting help finding your way to a working group.

- The brand-new [findwork](https://www.rustaceans.org/findwork) site, which
  provides an entry point to a number of open issues across the Rust project,
  including those managed by working groups (see the "impl period" tab). Thanks,
  @nrc, for putting this together!

We also plan two in-person events, paired with upcoming Rust conferences. Each
of them is a two-day event populated in part by Rust core developers; come hang
out and work together!

- [October 2-3 at RustFest](http://blog.rustfest.eu/this-week-in-rustfest-9-impl-days).
- [October 24-25 at Rust Belt Rust](https://goo.gl/forms/e9hmmsFw4owhhDf62).

As usual, all of these venues abide by the [Rust code of conduct]. But more than
that: this "impl period" is a chance for us all to have fun *collaborating* and
*helping* each other, and those participating in the official venues are
expected to meet the highest standards of behavior.

[Rust code of conduct]: https://www.rust-lang.org/conduct.html

## The working groups

Without further ado, here's the initial lineup! (A few more working groups are
expected to arise over time.)

### Compiler team

<table>
<tr>
    <td><b>WG-compiler-errors</b></td>
    <td>Make Rust's error messages even friendlier.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-front</b></td>
    <td>Dip your toes in with parsing and syntax sugar.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-middle</b></td>
    <td>Implement features that involve typechecking.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-traits</b></td>
    <td>Want generic associated types? You know what to do.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-incr</b></td>
    <td>Finish incremental compilation; receive undying love.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-nll</b></td>
    <td>Delve into the bowels of borrowck to slay the beast: NLL!</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-const</b></td>
    <td>Const generics. Enough said.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Libs team

<table>
<tr>
    <td><b>WG-libs-blitz</b></td>
    <td>Help finish off the Blitz before all the issues are gone!</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-cookbook</b></td>
    <td>Work on bite-sized examples to get folks cooking with Rust.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-guidelines</b></td>
    <td>Take the wisdom from the Blitz and pass it on.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-simd</b></td>
    <td>We want stable SIMD, and we need your help.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-openssl</b></td>
    <td>Want better docs for openssl? So do we.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-rand</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Docs team

<table>
<tr>
    <td><b>WG-docs-rustdoc</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-docs-rustdoc2</b></td>
    <td>Get in on a bottom-up revamp of rustdoc!</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-docs-rbe</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Dev tools team

<table>
<tr>
    <td><b>WG-dev-tools-rls</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-vscode</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-rustfmt</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-IntelliJ</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-rustup</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-clippy</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-bindgen</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Cargo team

<table>
<tr>
    <td><b>WG-cargo-native</b></td>
    <td>Let's make native dependencies as painless as we can.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-registries</b></td>
    <td>Going beyond crates.io to support custom registries.</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-pub-deps</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-integration</b></td>
    <td>How easy can we make it to use Cargo with your build system?</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Infrastructure team

<table>
<tr>
    <td><b>WG-infra-crates.io</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-perf</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-crater2</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-secure</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-host</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-rustbuild</b></td>
    <td></td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>

### Core team

<table>
<tr>
    <td><b>WG-core-site</b></td>
    <td>The web site is getting overhauled; help shape the new content!</td>
    <td><a href="">Learn more</a></td>
    <td><a href="">Chat</a></td>
</tr>
</table>
