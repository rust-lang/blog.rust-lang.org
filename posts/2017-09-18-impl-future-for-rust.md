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
per our roadmap, our aim is mentoring at *all* levels of experience. To get started,
say hello in the chat rooms for any of the work groups you're interested in!

## A few points of order

There are a few online venues for keeping in the loop with working group activity:

- There is a [dedicated Gitter community](https://gitter.im/rust-impl-period/)
  with channels for each working group, as well as
  a [global channel](https://gitter.im/rust-impl-period/Lobby) for talking about
  the process as a whole, or getting help finding your way to a working group.
  **For those who prefer IRC, a good [bridge](https://irc.gitter.im/) is available**!

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

**If you find a group that interests you, please say hello in the corresponding
chat room!**

### Compiler team

<table>
<tr>
    <td><b>WG-compiler-errors</b></td>
    <td>Make Rust's error messages even friendlier.</td>
    <td><a href="https://paper.dropbox.com/doc/Compiler-errors-FSZdfXAGo3uMQ1wuDcZcy">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-errors">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-front</b></td>
    <td>Dip your toes in with parsing and syntax sugar.</td>
    <td><a href="https://paper.dropbox.com/doc/Parser-and-Name-Resolution-Front-end-b0SZiNroIE1HK3lHKm8k7">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-front">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-middle</b></td>
    <td>Implement features that involve typechecking.</td>
    <td><a href="https://paper.dropbox.com/doc/Middle-Type-checker-XEPTHIWvzlvqkSC3cluTr">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-middle">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-traits</b></td>
    <td>Want generic associated types? You know what to do.</td>
    <td><a href="https://paper.dropbox.com/doc/Trait-system-LCgNlSbM5cPOyEyWdoqzW">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-traits">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-incr</b></td>
    <td>Finish incremental compilation; receive undying love.</td>
    <td><a href="https://paper.dropbox.com/doc/Incremental-Compilation-GtIsqsyiXfiyzOh99xp9R">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-incr">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-nll</b></td>
    <td>Delve into the bowels of borrowck to slay the beast: NLL!</td>
    <td><a href="https://paper.dropbox.com/doc/Non-Lexical-Lifetimes-u5uc6VxJic67K2ynmTiFV">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-nll">Chat</a></td>
</tr>
<tr>
    <td><b>WG-compiler-const</b></td>
    <td>Const generics. Enough said.</td>
    <td><a href="https://paper.dropbox.com/doc/Const-system-hNGg3H7sqnHb6nf39zpwl">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-compiler-const">Chat</a></td>
</tr>
</table>

### Libs team

<table>
<tr>
    <td><b>WG-libs-blitz</b></td>
    <td>Help finish off the Blitz before all the issues are gone!</td>
    <td><a href="https://paper.dropbox.com/doc/libz-blitz-ymXpoWVNDwVDigdrJ5o49">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libz-blitz">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-cookbook</b></td>
    <td>Work on bite-sized examples to get folks cooking with Rust.</td>
    <td><a href="https://paper.dropbox.com/doc/Rust-cookbook-DFaopl45jyZGWKI6iFDwD">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libs-cookbook">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-guidelines</b></td>
    <td>Take the wisdom from the Blitz and pass it on.</td>
    <td><a href="https://paper.dropbox.com/doc/API-Guidelines-bDAAOER4WHdxJ1XtEAFYs">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libs-guidelines">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-simd</b></td>
    <td>Provide access to hardware parallelism in Rust! </td>
    <td><a href="https://paper.dropbox.com/doc/simd-9H0xb83w1TD8Tc1yEG75M">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libs-simd">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-openssl</b></td>
    <td>Want better docs for openssl? So do we.</td>
    <td><a href="https://paper.dropbox.com/doc/OpenSSL-crate-FRMKrV0PjCVqFSBHfmNS5">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libs-openssl">Chat</a></td>
</tr>
<tr>
    <td><b>WG-libs-rand</b></td>
    <td>Craft a stable, core crate for randomness.</td>
    <td><a href="https://github.com/rust-lang/rfcs/pull/2152">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-libs-rand">Chat</a></td>
</tr>
</table>

### Docs team

<table>
<tr>
    <td><b>WG-docs-rustdoc</b></td>
    <td>Help make docs beautiful for everyone!</td>
    <td><a href="https://paper.dropbox.com/doc/Rustdoc-issue-roundup-ZSIIXNDGEPozTM9axn0BO">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-docs-rustdoc">Chat</a></td>
</tr>
<tr>
    <td><b>WG-docs-rustdoc2</b></td>
    <td>Get in on a bottom-up revamp of rustdoc!</td>
    <td><a href="https://paper.dropbox.com/doc/WG-rustdoc2-3lxugWOmvpXC2eMaAQK04">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-docs-rustdoc2">Chat</a></td>
</tr>
<tr>
    <td><b>WG-docs-rbe</b></td>
    <td>Teach others Rust in the browser.</td>
    <td><a href="https://paper.dropbox.com/doc/WG-rbe-Tgd0wu70N6zSmACkNL3TI">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-docs-rbe">Chat</a></td>
</tr>
<tr>
    <td><b>WG-docs-checklist</b></td>
    <td>Help finish the standard library documentation!</td>
    <td><a href="https://paper.dropbox.com/doc/Finish-the-docs-checklist-WG-docs-checklist-7CiH332jWblQUTqUOVXaZ">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-docs-checklist">Chat</a></td>
</tr>
</table>

### Dev tools team

<table>
<tr>
    <td><b>WG-dev-tools-rls</b></td>
    <td>Help make Rust's IDE experience first class.</td>
    <td><a href="https://paper.dropbox.com/doc/Rust-Language-Server-RLS-XQbsngZNog9pkt0AfcMo7">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-rls">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-vscode</b></td>
    <td>Improve Rust's IDE experience for VSCode.</td>
    <td><a href="https://paper.dropbox.com/doc/Rust-support-in-Visual-Studio-Code-RZ34qWGwy04Xwc82NFi78">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-vscode">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-clients</b></td>
    <td>Implement new RLS clients: Atom, Sublime, Visual Studio...</td>
    <td><a href="https://paper.dropbox.com/doc/New-RLS-clients-VrtQKnZR4r3uLD1VBypRI">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-clients">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-IntelliJ</b></td>
    <td>Polish up an already-rich Rust IDE experience.</td>
    <td><a href="https://paper.dropbox.com/doc/Intellij-Rust-IYJGtI7uAjdqr2igv4Y7r">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-IntelliJ">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-rustfmt</b></td>
    <td>Make Rust's code the prettiest!</td>
    <td><a href="https://paper.dropbox.com/doc/rustfmt-7yTxFPEHtV6jktqZ2pRj1">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-rustfmt">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-rustup</b></td>
    <td>Make Rust's first impression even better!</td>
    <td><a href="https://paper.dropbox.com/doc/rustup-mngGQUtX1UkBay3wgOGJi">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-rustup">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-clippy</b></td>
    <td>It looks like you're trying to write a linter. Want help?</td>
    <td><a href="https://paper.dropbox.com/doc/Clippy-integration-and-improvements-gebwGlwNOoy6UGLspGO4T">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-clippy">Chat</a></td>
</tr>
<tr>
    <td><b>WG-dev-tools-bindgen</b></td>
    <td>Make FFI'ing to C and C++ easy, automatic, and robust!</td>
    <td><a href="https://paper.dropbox.com/doc/bindgen-xTXplHlfqJpnDvPhMqmfW">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-dev-tools-bindgen">Chat</a></td>
</tr>
</table>

### Cargo team

<table>
<tr>
    <td><b>WG-cargo-native</b></td>
    <td>Let's make native dependencies as painless as we can.</td>
    <td><a href="https://paper.dropbox.com/doc/Declarative-native-dependencies-iLRUq6Zt2tPtLWE9IyLqS">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-cargo-native">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-registries</b></td>
    <td>Going beyond crates.io to support custom registries.</td>
    <td><a href="https://paper.dropbox.com/doc/Cargo-Multiple-Registries-IrW9bRuZ1rdc4o9UPdQM9">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-cargo-registries">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-pub-deps</b></td>
    <td>Teach Cargo which of your dependencies affects your users.</td>
    <td><a href="https://paper.dropbox.com/doc/Cargo-pubpriv-dependencies-JDXpDtGRnz8CY3KYlcUBD">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-cargo-pub-deps">Chat</a></td>
</tr>
<tr>
    <td><b>WG-cargo-integration</b></td>
    <td>How easy can it be to use Cargo with your build system?</td>
    <td><a href="https://paper.dropbox.com/doc/Cargo-build-system-integration-1sqRG8uyCqxv9EfoS8cco">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-cargo-integration">Chat</a></td>
</tr>
</table>

### Infrastructure team

<table>
<tr>
    <td><b>WG-infra-crates.io</b></td>
    <td>Try your hand at a production Rust web app!</td>
    <td><a href="https://paper.dropbox.com/doc/Crates.io-g8NWnnNIeTq8DaqjGoZLr">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-crates.io">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-perf</b></td>
    <td>Let's make sure Rust gets faster.</td>
    <td><a href="https://paper.dropbox.com/doc/Perf.rlo-dp5rp6tSg7bOMmQwLygp4">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-perf">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-crater</b></td>
    <td>Regularly testing the compiler against the Rust ecosystem.</td>
    <td><a href="https://paper.dropbox.com/doc/Crater-D7DpG48tMhhHhrUm8kyhY">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-crater">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-secure</b></td>
    <td>Help us implement best practices for Rust's infrastructure!</td>
    <td><a href="https://paper.dropbox.com/doc/Securing-Infrastructure-xq5FfLQs1hkxwgxDsHi2Z">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-secure">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-host</b></td>
    <td>Managing the services that keep the Rust machine running.</td>
    <td><a href="https://paper.dropbox.com/doc/Host-WiqosInW7SpUblFVGKeOo">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-host">Chat</a></td>
</tr>
<tr>
    <td><b>WG-infra-rustbuild</b></td>
    <td>Streamline the compiler build process.</td>
    <td><a href="https://paper.dropbox.com/doc/Rustbuild-Cz96pk6FBtP54JClTCDNd">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-infra-rustbuild">Chat</a></td>
</tr>
</table>

### Core team

<table>
<tr>
    <td><b>WG-core-site</b></td>
    <td>The web site is getting overhauled; help shape the new content!</td>
    <td><a href="https://paper.dropbox.com/doc/rust-lang.org-content-improvement-uGns2d39DFgT0X9FQg0yD">Learn more</a></td>
    <td><a href="https://gitter.im/rust-impl-period/WG-core-site">Chat</a></td>
</tr>
</table>
