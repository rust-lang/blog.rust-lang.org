---
layout: post
title: "Announcing the Metrics Initiative"
author: Jane Losare-Lusby and Esteban Kuber
team: The Rust Compiler Team <https://www.rust-lang.org/governance/teams/compiler>
---

We ([Jane](https://github.com/yaahc/) and [Esteban](https://github.com/estebank)) are excited to announce the start of the Metrics initiative, an **opt-in** and privacy-preserving system for recording various metrics and metadata from Rust compiler invocations. These metrics can then be optionally uploaded by Rust users to provide us with useful information about how Rust is being used in the wild.

We're envisioning three use cases for metrics within rustc.

* Supporting feature development
    * e.g., answering specific questions such as when the old and new trait solvers diverge or helping identify and resolve bugs before impacting users
* Guiding improvements to User Experience,
    * e.g., knowing which compiler errors are causing the most confusion or are hit the most frequently, focusing on improving those first, and verifying that the improvements help
* Improving perf feedback loops and insight,
    * e.g., helping identify pathological edge cases, similar to [work](https://nnethercote.github.io/2022/02/25/how-to-speed-up-the-rust-compiler-in-2022.html) @nnethercote has done manually in the past

We're at the point of the initiative where we would like to inform the project members about it and start implementing the metrics infrastructure in collaboration with their real-world needs.

For more information about the initiative, please check out the tracking issue and related links: https://github.com/rust-lang/rust/issues/128914.

**Please reach out with any use cases you have in mind!**
