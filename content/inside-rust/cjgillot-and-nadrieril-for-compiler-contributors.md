+++
path = "inside-rust/2020/12/28/cjgillot-and-nadrieril-for-compiler-contributors"
title = "Please welcome cjgillot and Nadrieril to compiler-contributors"
authors = ["Wesley Wiser"]
description = "Please welcome cjgillot and Nadrieril to compiler-contributors"
aliases = ["inside-rust/2020/12/28/cjgillot-and-nadrieril-for-compiler-contributors.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

Please welcome [@cjgillot] and [@Nadrieril] to the [compiler-contributors] group!

[@cjgillot] has been working to improve the query system used internally in `rustc` which powers incremental compilation.
Some of their improvements have been to reduce unnecessary work performed during incremental compilation, leading to faster builds.
Other improvements have made the query system leaner allowing `rustc` to bootstrap faster.
[@cjgillot] has also made many tweaks and optimizations to the query system.

[@Nadrieril] has been working on the pattern matching system resulting in improvements to compilation performance, code readability and related diagnostics generated by rustc.
They have also been working on implementing features related to pattern matching like or-patterns and slice-patterns and helping push those features closer to stabilization.

Congratulations [@cjgillot] and [@Nadrieril] and thanks!

[@cjgillot]: https://github.com/cjgillot
[@Nadrieril]: https://github.com/Nadrieril
[compiler-contributors]: https://rust-lang.github.io/rfcs/2689-compiler-team-contributors.html
