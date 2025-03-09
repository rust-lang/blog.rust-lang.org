+++
layout = "post"
date = 2020-03-26
title = "rustc-dev-guide Overview"
author = "Chris Simpkins"
description = "2020-03-26 rustc-dev-guide Overview"
team = "the Rustc Dev Guide Working Group <https://www.rust-lang.org/governance/teams/compiler#wg-rustc-dev-guide>"
+++

The `rustc` compiler includes over 380,000 lines of source across more than 40 crates<sup>1</sup> to support the lexing through binary linking stages of the Rust compile process. It is daunting for newcomers, and we recognize that a high-level survey of the pipeline is warranted.

In our [December update](https://blog.rust-lang.org/inside-rust/2019/12/20/wg-learning-update.html), we announced plans for the publication of the "rustc-dev-guide Overview". Our goal is to describe the integrated components of the compiler in a high-level document for users and potential developers. The Overview will be published at the beginning of the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/) to orient readers to the more detailed documentation of the compiler in subsequent chapters.

## Rustc Overview Structure

We will break the compiler down to address two questions at a high-level:

1. What does the compiler do to your source code?
2. How does the compiler do it?

As we address these general areas, we will provide a synopsis that briefly covers frequent community questions like:

- What are the conflicting goals of the compiler, and how are issues like compiler speed, compiler memory usage, program speed, program size, and compiler stability/correctness balanced?
- What are the stages of the compile process, and how do they fit together?
- What are the intermediate representations of my source code?
- What happens to generics during the compile process?
- What kind of optimizations are performed during the compile process?
- How does incremental compilation work?
- Does `rustc` have support for parallel compilation?

## Get Involved!

Work is in progress on the Overview, and we need your help. A working draft of the document is available in [this pull request](https://github.com/rust-lang/rustc-dev-guide/pull/633) on the `rustc-dev-guide` GitHub repository.

If there is an area of `rustc` that you would like to understand better and it is appropriate for an overview document, please open an issue on our [issue tracker](https://github.com/rust-lang/rustc-dev-guide/issues) to let us know.

And if you know the compiler and want to pitch in on the rustc-dev-guide Overview, open a pull request with your revisions. We welcome your contributions and look forward to your participation!

## Interested in Learning (Working Group)?

Are you interested in learning more about the `rustc` compiler and teaching others? Drop by our [Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide) and say hello!

---

<sup>1</sup> These numbers account for lines in Rust files across all dependencies necessary to build `rustc`. Thanks to @LeSeulArtichaut for these calculations! See [the notes on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide/topic/The.20Rustc.20Overview.3A.20blog.20post/near/189441101) for additional details.
