---
layout: post
title: "2019-11-18 IDE team meeting"
author: Aleksey Kladov, Igor Matuszewski
team: the IDE team <https://www.rust-lang.org/governance/teams/dev-tools#ides>
---

Meeting run by nikomatsakis. Minutes written by nikomatsakis.
Attending: nikomatsakis, pnkfelix, Xanewok, matklad
[Notes](https://hackmd.io/fAnj6pNqRRGIyDQ4el5tcQ)

# The Rust IDE
In the last compiler/IDE team meeting we've discussed the overall direction for IDE support in Rust.

At the moment, the two IDEs developed as part of the Rust project are Rust Language Server (RLS) and rust-analyzer.
The former is currently being shipped with the Rust distribution while the latter serves as a foundation for the "RLS 2.0" working group.

Unfortunately, these are actively developed in separation without much code-sharing between the two.
We'd like to change that and to find out how we can unify these efforts.
Therefore, we've been having a series of talks with the aim of elaborating the design space and creating a proposal for how to improve the situation going forward.

This blog post gives a short summary from our most recent meeting.

# Why 2 IDEs?
The main benefits of rust-analyzer is greater performance (because of fully-lazy compilation model) and somewhat richer feature-set (due to more flexible analysis API).
The main benefits of RLS is precision (it uses `rustc` under the hood).
Additionally, RLS is the main consumer of save-analysis infrastructure, which is a good fit for tools which need a static view of the codebase, such as [cargo-src](https://github.com/rust-dev-tools/cargo-src) or [lsif](https://code.visualstudio.com/blogs/2019/02/19/lsif).

# Save-analysis

What is "save-analysis"?
It is an unstable format which rustc uses to record information about the compiled code.
It contains a pretty high-level information.
For example, for each identifier in the source-crate, save-analyzer will map this identifier to a definition and list of usages.
`env RUSTFLAGS="-Zunstable-options -Zsave-analysis" cargo check` can be used to instruct `rustc` to produce save-analysis files (in JSON format).
Because save-analysis is produced directly from rustc iternal data structures, it is guaranteed to be correct (modulo bugs in rustc itself).

# Query model

The fundamental problem with save-analysis is that it is computed for the whole crate at once.
This is pretty slow for non-trivial crates, and is also wasteful.
At any given moment in time, only a small fraction of analysis information is really required.
rust-analyzer solves this by using [`salsa`](https://github.com/salsa-rs/salsa) queries for code analysis.
The result is a compilation model which is fully lazy across the whole crate graph.
This model is similar to what rustc is using internally, but is more lazy both "vertically" and "horizontally".
Vertically, `rustc` starts to be incremental only after parsing and macro expansion; rust-analyzer is incremental on per-file basis.
Horizontally, `rustc` compiles one crate at a time; rust-analyzer uses queries for the whole crate graph.

# Way forward
Our current hypothesis is that it is possible to integrate both approaches without doubling the engineering effort.
Specifically, we will add an option to rust-analyzer to use save-analysis for find-usages and rename functionality.
That way, we'll get precise results for most important queries, without slowing down completion.
Unlike RLS, however, rust-analyzer will not link to rustc and instead will rely on cargo for running the compiler and producing save-analysis data.
If this approach works, we will consider freezing RLS and focusing fully on rust-analyzer.
Long term, the plan is to unify the save-analysis fallback path and the lazy analysis.

In parallel to this RLS/rust-analyzer unification effort, we continue to pursue rustc library-ification, with a specific focus on traits solving (via chalk) and type inference.
"Library-ification" is a term we've been using for the process of extracting code out of rustc into re-usable libaries which can be shared by both rustc and rust-analyzer.
The goal is to use library-ification to gradually reduce the amount of duplicated code between rustc and rust-analyzer, with the goal of eventually either having a single code-base, or having the vast majority of the logic be shared.
