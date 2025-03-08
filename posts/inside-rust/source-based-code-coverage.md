+++
layout = "post"
date = 2020-11-12
title = "Source-based code coverage in nightly"
author = "Tyler Mandry"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++


Support has landed in the nightly compiler for source-based code coverage,
and we want your help testing it!


# What is _source-based_ code coverage, exactly?

You may already be familiar with code coverage, which shows you which lines
of code execute. Code coverage is usually applied to tests to find out which
code is actually being tested and which code isn’t.

Nightly Rust already supports another kind of source code coverage, commonly
called gcov, which relies on debug info to map from LLVM IR to lines of
source code. Instrumentation is then added in the LLVM backend during code
generation to count how many times each line is run.

However, since LLVM doesn’t know exactly how Rust code is structured, there’s
a lot lost in the translation between Rust source and LLVM IR. Line-level
granularity is sometimes too coarse, and debug info can be unreliable,
especially when building in release mode. The result is coverage reports that
only show an approximation of what code actually executed.

Source-based code coverage instrumentation is applied by the Rust compiler,
not LLVM. This instrumentation is more precise because it's being done in
MIR, which holds a mapping between the original Rust source code and the
control-flow graph of the program.

That means things like short-circuited conditionals, closures, and match
guards are all precisely counted. And since instrumentation counters are
injected as regular MIR statements, the compiler can further optimize the
program without affecting coverage results.

[![Comparison of gcov and source-based coverage results][comparison-img]][comparison-img]

_Above: A comparison of the gcov (left) and source-based coverage (right)
results. gcov highlights skipped lines, marked with #####, while source-based
coverage highlights exact regions of code that were skipped. Note that on
line 30, one boolean subexpression is short-circuited. This is surfaced by
source-based coverage but not gcov._

What this means is that source-based code coverage is both efficient and
accurate. LLVM’s existing coverage tools ([llvm-profdata] and [llvm-cov])
generate both coverage summaries and very fine-grained code regions, helping
you find gaps in your testing coverage. What you do about that is up to you!

[comparison-img]: ../../../../images/inside-rust/2020-11-12-source-based-code-coverage/comparison.png
[llvm-profdata]: https://llvm.org/docs/CommandGuide/llvm-profdata.html
[llvm-cov]: https://llvm.org/docs/CommandGuide/llvm-cov.html

# Trying it out

Work on the implementation [began back in April][MCP], and [many PRs
later][PRs], it’s ready for you to try. All you need is a recent nightly and
a tool to read the coverage reports.

[Take a look at this guide to get started][guide]. If you spot any issues,
please [report them]. It’s a huge help!

Finally, if you try it out and it works well, we’d also like to hear from
you! Come by the [Zulip stream] for this change or comment on the [feature
request].

[MCP]: https://github.com/rust-lang/compiler-team/issues/278
[PRs]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+author%3Arichkadel+is%3Aclosed+closed%3A%3C2020-11-06
[guide]: https://doc.rust-lang.org/nightly/rustc/instrument-coverage.html
[report them]: https://github.com/rust-lang/rust/issues/new/choose
[Zulip stream]: https://rust-lang.zulipchat.com/#narrow/stream/233931-t-compiler.2Fmajor-changes/topic/Implement.20LLVM-compatible.20source-based.20cod.20compiler-team.23278
[feature request]: https://github.com/rust-lang/rust/issues/34701

# Acknowledgements

The implementation work was all done by Rich Kadel; thanks to him for all the
amazing work he’s done. Thanks also to Wesley Wiser for helping with reviews,
to Bob Wilson for lending his experience with LLVM's InstrProf coverage APIs,
and to eddyb for their guidance in choosing a MIR-based approach.
