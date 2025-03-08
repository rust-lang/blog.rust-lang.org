+++
layout = "post"
title = "Welcoming David Wood to compiler team and Jack Huey to compiler-contributors"
author = "Wesley Wiser"
description = "Please welcome David Wood to the compiler team and Jack Huey to compiler-contributors"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++

Please welcome [David Wood] to the compiler team and [Jack Huey] to the [compiler-contributors] group!

David Wood ([@davidtwco]) has been a frequent contributor to Rust in many different parts of the compiler.
Much of David's recent work has been focused on [polymorphisation] which allows rustc to reduce the number of duplicated generic functions in certain situations and on adding [split DWARF] support to the LLVM backend.
Previously, David has worked on numerous diagnostic improvements, internal compiler error fixes and the [non-lexical lifetimes] initiative.

Jack Huey ([@jackh726]) has been a major contributor to the [Chalk] project which is a re-implementation of the Rust trait system using logical-programming constructs with the eventual goal of replacing the current system.
In addition to Chalk, Jack has also worked on various refactorings to make the rustc trait code more like Chalk.
Jack is also the co-lead of the [traits working group] which is coordinating this effort.

Congratulations [David Wood] and [Jack Huey] and thanks!

[David Wood]: https://github.com/davidtwco
[@davidtwco]: https://github.com/davidtwco
[polymorphisation]: https://davidtw.co/media/masters_dissertation.pdf
[split DWARF]: https://github.com/rust-lang/rust/pull/77117
[non-lexical lifetimes]: https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html
[Jack Huey]: https://github.com/jackh726
[@jackh726]: https://github.com/jackh726
[Chalk]: https://github.com/rust-lang/chalk
[traits working group]: https://rust-lang.github.io/compiler-team/working-groups/traits/
[compiler-contributors]: https://rust-lang.github.io/rfcs/2689-compiler-team-contributors.html
