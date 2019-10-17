---
layout: post
title: "Please welcome ecstatic-morse to compiler-contributors"
author: Niko Matsakis
description: "Please welcome ecstatic-morse to compiler-contributors"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

Please welcome [@ecstatic-morse] to the [compiler contributors] group!
They're working to make compile-time evaluation more expressive by
enabling `if`, `match` and other control flow in constants. As one of
their first major contributions, they implemented a dataflow analysis
to validate the bodies of `const`s and `const fn`s
([rust-lang/rust#64470]).

Congratulations [@ecstatic-morse], and thanks!

[@ecstatic-morse]: https://github.com/ecstatic-morse
[compiler contributors]: https://rust-lang.github.io/rfcs/2689-compiler-team-contributors.html
[rust-lang/rust#64470]: https://github.com/rust-lang/rust/pull/64470
