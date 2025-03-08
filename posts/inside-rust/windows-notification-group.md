+++
layout = "post"
date = 2020-06-09
title = "Announcing the Windows and ARM notification groups"
author = "Niko Matsakis"
description = "Announcing the Windows and ARM notification groups"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++

We are forming two new groups in the compiler team:

* A Windows group, for helping us to diagnose and resolve Windows-related issues.
* An ARM group, for helping us to resolve issues specific to the ARM architectures

Each of these groups are "notification groups", which means that anyone can add their own name to the list -- if you do, you'll receive pings when Windows- or ARM-related bugs arise.

Each group also has an associated Zulip stream ([`#t-compiler/windows`], [`#t-compiler/arm`]) where people can go to pose questions and discuss topics specific to that target.

To get a better idea for what the groups will do, here are some examples of the kinds of questions where we would have reached out to the Windows group for advice in determining the best course of action:

* Should we remove the legacy InnoSetup GUI installer? [#72569]
* What names should we use for static libraries on Windows? [#29520] 

So, if you are interested in participating, please sign up for the Windows or aarch64 groups! To do so, you open a PR against the [rust-lang/team] repository. Just follow these examples (but change the username to your own):

* [Windows example]
* [ARM example]

[rust-lang/team]: https://github.com/rust-lang/team
[Windows example]: https://github.com/rust-lang/team/pull/348
[ARM example]: https://github.com/rust-lang/team/pull/358
[#72569]: https://github.com/rust-lang/rust/pull/72569
[#29520]: https://github.com/rust-lang/rust/pull/29520
[`t-compiler/windows`]: https://rust-lang.zulipchat.com/#narrow/stream/242869-t-compiler.2Fwindows
[`t-compiler/arm`]: https://rust-lang.zulipchat.com/#narrow/stream/242906-t-compiler.2Farm
