---
layout: post
title: "loop miscompilation fix"
author: Mark Rousskov
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

We have landed a fix in nightly which fixes a [longstanding soundness hole] in
Rust, where `loop {}` is undefined behavior in LLVM, but not in Rust.

The compiler now correctly compiles these loops, but this can have a performance
impact on code, as it reduces some optimizations that LLVM previously performed.
In practice, running [Firefox benchmarks], we did not measure any performance
delta. In [artificial benchmarks], this change has at most a 50% increase on
wall-clock performance.

We are landing this change as it fixes undefined behavior in practice, while
having essentially zero impact on applications written in Rust.

However, since it does have the potential to reduce performance of code, we are
publishing this post to inform users. We may revert the change if we get
widespread feedback of performance regressions that aren't easily fixable.

[longstanding soundness hole]: https://github.com/rust-lang/rust/issues/28728
[Firefox benchmarks]: https://treeherder.mozilla.org/perf.html#/compare?originalProject=try&originalRevision=2b9e4386bfc5de74883781a869b95521e5a36290&newProject=try&newRevision=9e5d8cb0607a10a3645d5d4ac79f2df9864d41d7&framework=1
[artificial benchmarks]: https://docs.google.com/spreadsheets/d/1I5p0_ChO7xakIYUvbhVlgmaKnvDdvOnkpv7zDZAy-CQ/edit
