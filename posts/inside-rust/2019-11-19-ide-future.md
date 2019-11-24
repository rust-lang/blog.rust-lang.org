---
layout: post
title: "2019-11-18 IDE team (?) meeting"
author: Aleksey Kladov, Igor Matuszewski
team: the IDE team <https://www.rust-lang.org/governance/teams/dev-tools#ides>
---

Meeting run by nikomatsakis. Minutes written by nikomatsakis.
Attending: nikomatsakis, pnkfelix, Xanewok, matklad
[Notes](https://hackmd.io/fAnj6pNqRRGIyDQ4el5tcQ)

# The Rust IDE
In the last compiler/IDE team meeting we've discussed the overall direction for IDE support in Rust.

At the moment, the two IDEs developed as part of the Rust project are Rust Language Server (RLS) and Rust Analyzer.
The former is currently being shipped with the Rust distribution while the latter serves as a foundation for the "RLS 2.0" working group.

Unfortunately, these are actively developed in separation without much code-sharing between the two.
We'd like to change that and to find out how we can unify these efforts.

# Why 2 IDEs?
The main benefits of rust-analyzer is greater performance (because of fully-lazy compilation model) and somewhat richer feature-set (due to more flexible analysis API).
The main benefits of RLS is precision (it uses `rustc` under the hood).
Additionally, RLS is the main consumer of save-analysis infrastructure, which is a good fit for tools which need a static view of the codebase, such as [cargo-src](https://github.com/rust-dev-tools/cargo-src) or [lsif](https://code.visualstudio.com/blogs/2019/02/19/lsif).

# Save-analysis
TODO(xanewok): Intro a bit what is it and how it allows us to be 'precise'

# Query model
TODO(xanewok): Intro a bit about laziness and how it can drastically reduce latency

# Way forward
Our current hypothesis is that it is possible to integrate both approaches without doubling the engineering effort.
Specifically, we will add an option to rust-analyzer to use save-analysis for find-usages and rename functionality.
Unlike RLS, however, rust-analyzer will not link to rustc and instead will rely on cargo for running the compiler and producing save-analysis data.
If this approach works, we will consider freezing RLS and focusing fully on rust-analyzer.
Long term, the plan is to unify the save-analysis fallback path and the lazy analysis.

In parallel to this RLS/rust-analyzer unification effort, we continue to pursue rustc library-ification, with a specific focus on traits solving (via chalk) and type inference.
