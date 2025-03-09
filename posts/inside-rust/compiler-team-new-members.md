+++
layout = "post"
date = 2024-11-12
title = "Announcing four new members of the compiler team"
author = "davidtwco and wesleywiser"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++
Its been no time at all since [we restructured the team and recognised our existing and new
members][blog_reorg], but we've already got new compiler team members to announce and recognise:

- [ChrisDenton](https://github.com/ChrisDenton), team member
  - ChrisDenton is a go-to Windows expert for much of the compiler team and is regularly helpful in
    resolving all sorts of tricky issues related to linkage, debuginfo, syscalls and Windows.
- [Mara Bos](https://github.com/m-ou-se), team member
  - Mara needs no introduction - in addition to co-leading the library team, and being a member
    of the leadership council, Mara makes various contributions to the compiler in her various
    projects to improve the language and its standard library, including recent improvements to the
    compiler's format string handling, but also diagnostic improvements and edition changes.
- [rcvalle](https://github.com/rcvalle), team member
  - rcvalle has been working on improving Rust's support for exploit mitigations and sanitizers
    for a few years and has led the exploit mitigations project group, including writing the
    compiler's exploit mitigation documentation and implementing Control Flow Integrity support in
    the compiler.
- [workingjubilee](https://github.com/workingjubilee), team member
  - workingjubilee is another prolific contributor, weighing in on nearly every discussion about
    low-level language semantics. They have made many varied contributions to the compiler
    improving our layout computation, understanding of ABIs, codegen, interop with C/C++ and wasm.

Thanks to all of our new members for their contributions!

[blog_reorg]: https://blog.rust-lang.org/inside-rust/2024/11/01/compiler-team-reorg.html
