+++
layout = "post"
date = 2020-03-19
title = "Resolving Rust's forward progress guarantees"
author = "Mark Rousskov"
description = "Should side-effect be the fix?"
team = "the compiler team <https://www.rust-lang.org/governance/teams/compiler>"
+++

There has been a longstanding miscompilation in Rust: programs that do not make
[forward progress]. Note that the previous link is to the C++ definition; Rust
is not C++, but currently LLVM optimizes all LLVM IR with the assumption that a
lack of forward progress is undefined behavior.

Note also that Rust does not define a lack of forward progress as [undefined
behavior], while C++ does. It is particularly common to encounter the
miscompilation "intentionally" when writing panic handlers and other such code
with a body of `loop {}`. Some users also report that they've unintentionally
hit this bug in recursive code which accidentally lacks a base case.

Somewhat recently, LLVM added an intrinsic which tells the optimizer that
forward progress has been made. On nightly Rust, you can enable this with
`-Zinsert-sideeffect`, which will use some heuristics to insert it where it's
possibly needed (currently, massively overshooting the minimal set).

However, recent attempts to enable this intrinsic by default hit a snag: it's
very expensive on compile times to do so ([3-30% regressions][compile-time
regressions]). There is some runtime effect as well; check builds (which do not
generate LLVM IR or run LLVM passes) regressed by up to 3-7%.

The current implementation in rustc emits calls to the side effect intrinsic
very aggressively; certainly in way more cases than is strictly necessary.
However, there's not really any good ideas on how to improve the analysis rustc
does without missing edge cases: we'd have to be "as good" as LLVM to emit only
when necessary.

Upstream, in LLVM, discussion has been ongoing for some time around whether, and
how to, adjust LLVM's model to permit frontends for languages like Rust to
opt-out of the forward progress guarantees. It seems unlikely that a solution
will materialize in upstream LLVM that allows us to opt-out in the short term.

However, having said that, side effect itself is likely improvable to at least
avoid the excessive consecutive calls, as demonstrated by this [IR][IR-test]
that occurs after LLVM optimizations. It seems plausible that those
improvements may also reduce the compile time hit that we see when enabling
side effect on the rustc side. Having said that, how simple these improvements
are is unclear.

We would love to hear feedback and suggestions on how to resolve this problem!
Please leave feedback on [this internals
thread](https://internals.rust-lang.org/t/resolving-rusts-forward-progress-guarantees/12003).

[IR-test]: https://gist.github.com/nikic/7e521def71d106c345a255e464b18d3f
[compile-time regressions]: https://perf.rust-lang.org/compare.html?start=66b0c97070f422cb82baaaafc79ee94cab4396c5&end=548b5e75afd6bad696920dfdb69c9812ce0488f1
[forward progress]: https://en.cppreference.com/w/cpp/language/memory_model#Forward_progress
[undefined behavior]: https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#undefined-behavior
