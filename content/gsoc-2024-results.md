+++
path = "2024/11/07/gsoc-2024-results"
title = "Google Summer of Code 2024 results"
authors = ["Jakub Beránek, Jack Huey and Paul Lenz"]
aliases = ["2024/11/07/gsoc-2024-results.html"]
+++

As we have previously [announced][gsoc-blog-post], the Rust Project participated
in [Google Summer of Code (GSoC)][gsoc] for the first time this year. Nine contributors have been tirelessly working on their exciting projects
for several months. The projects had various durations; some of them have ended in August, while the last one has been
concluded in the middle of October. Now that the final reports of all the projects have been submitted, we can happily
announce that all nine contributors have passed the final review! That means that we have deemed all of their projects to be successful,
even though they might not have fulfilled all of their original goals (but that was expected).

We had a lot of great interactions with our GSoC contributors, and based on their feedback, it seems that they were also
quite happy with the GSoC program and that they had learned a lot. We are of course also incredibly grateful for all their
contributions - some of them have even continued contributing after their project has ended, which is really awesome.
In general, we think that Google Summer of Code 2024 was a success for the Rust Project, and we are looking forward to
participating in GSoC (or similar programs) again in the near future. If you are interested in becoming a (GSoC) contributor,
check out our [project idea list](https://github.com/rust-lang/google-summer-of-code).

Below you can find a brief summary of each of our GSoC 2024 projects, including feedback
from the contributors and mentors themselves. You can find more information about the projects
[here][gsoc-project-list].

### Adding lint-level configuration to cargo-semver-checks

- Contributor: [Max Carr](https://github.com/suaviloquence/)
- Mentor: [Predrag Gruevski](https://github.com/obi1kenobi)
- [Final report](https://blog.mcarr.one/gsoc-final/)

[cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) is a tool designed for automatically detecting semantic versioning conflicts, which is planned to one day become a part of Cargo itself. The goal of this project was to enable cargo-semver-checks to ship additional *opt-in* lints by allowing users to configure which lints run in which cases, and whether their findings are reported as errors or warnings. Max achieved this goal by implementing a [comprehensive system](https://github.com/obi1kenobi/cargo-semver-checks?tab=readme-ov-file#lint-level-configuration) for configuring `cargo-semver-checks` lints directly in the `Cargo.toml` manifest file. He also extensively discussed the design with the Cargo team to ensure that it is compatible with how other Cargo lints are configured, and won't present a future compatibility problem for merging cargo-semver-checks into Cargo.

Predrag, who is the author of `cargo-semver-checks` and who mentored Max on this project, was very happy with his contributions that even went beyond his original project scope:

> He designed and built one of our most-requested features, and produced design prototypes of several more features our users would love. He also observed that writing quality CLI and functional tests was hard, so he overhauled our test system to make better tests easier to make. Future work on cargo-semver-checks will be much easier thanks to the work Max put in this summer.

Great work, Max!

### Implementation of a faster register allocator for Cranelift

- Contributor: [Demilade Sonuga](https://github.com/d-sonuga)
- Mentors: [Chris Fallin](https://github.com/cfallin) and [Amanieu d'Antras](https://github.com/Amanieu)
- [Final report](https://d-sonuga.netlify.app/gsoc/regalloc-iii/)

The Rust compiler can use various *backends* for generating executable code. The main one is of course the LLVM backend, but there are other backends, such as [GCC][gcc backend], [.NET](#rust-to-net-compiler---add-support-for-compiling--running-cargo-tests) or [Cranelift][clif backend]. Cranelift is a code generator for various hardware targets, essentially something similar to LLVM. The Cranelift backend uses Cranelift to compile Rust code into executable code, with the goal of improving compilation performance, especially for debug (unoptimized) builds. Even though this backend can already be faster than the LLVM backend, we have identified that it was slowed down by the register allocator used by Cranelift.

Register allocation is a well-known compiler task where the compiler decides which registers should hold variables and temporary expressions of a program. Usually, the goal of register allocation is to perform the register assignment in a way that maximizes the runtime performance of the compiled program. However, for unoptimized builds, we often care more about the compilation speed instead. 

Demilade has thus proposed to implement a new Cranelift register allocator called `fastalloc`, with the goal of making it as fast as possible, at the cost of the quality of the generated code. He was very well-prepared, in fact he had a prototype implementation ready even before his GSoC project has started! However, register allocation is a complex problem, and thus it then took several months to finish the implementation and also optimize it as much as possible. Demilade also made extensive use of fuzzing to make sure that his allocator is robust even in the presence of various edge cases.

Once the allocator was ready, Demilade benchmarked the Cranelift backend both with the original and his new register allocator using our compiler [benchmark suite][rustc-perf]. And the performance results look awesome! With his faster register allocator, the Rust compiler executes up to 18% less instructions across several benchmarks, including complex ones like performing a debug build of Cargo itself. Note that this is an *end-to-end* performance improvement of the time needed to compile a whole crate, which is really impressive. If you would like to examine the results in more detail or even run the benchmark yourself, check out Demilade's [final report](https://d-sonuga.netlify.app/gsoc/regalloc-iii/), which includes detailed instructions on how to reproduce the benchmark.

Apart from having the potential to speed up compilation of Rust code, the new register allocator can be also useful for other use-cases, as it can be used in Cranelift on its own (outside the Cranelift codegen backend). What can we can say other than we are very happy with Demilade's work! Note that the new register allocator is not yet available in the Cranelift codegen backend out-of-the-box, but we expect that it will eventually become the default choice for debug builds and that it will thus make compilation of Rust crates using the Cranelift backend faster in the future.

[gcc backend]: https://github.com/rust-lang/rustc_codegen_gcc
[clif backend]: https://github.com/rust-lang/rustc_codegen_cranelift

### Improve Rust benchmark suite

- Contributor: [Eitaro Kubotera](https://github.com/s7tya)
- Mentor: [Jakub Beránek](https://github.com/kobzol)
- [Final report](https://gist.github.com/s7tya/96dc1cae4ca9e59841a95f0f48d023d6)

This project was relatively loosely defined, with the overarching goal of improving the user interface of the [Rust compiler benchmark suite][rustc-perf]. Eitaro tackled this challenge from various angles at once. He improved the visualization of runtime benchmarks, which were previously a second-class citizen in the benchmark suite, by adding them to our [dashboard](https://perf.rust-lang.org/dashboard.html) and by implementing [historical charts](https://github.com/rust-lang/rustc-perf/pull/1922) of runtime benchmark results, which help us figure out how is a given benchmark behaving over a longer time span.

Another improvement that he has worked on was embedding a profiler trace visualizer directly within the `rustc-perf` website. This was a challenging task, which required him to evaluate several visualizers and figure out a way how to include them within the source code of the benchmark suite in a non-disruptive way. In the end, he managed to integrate [Perfetto](https://ui.perfetto.dev/) within the suite website, and also performed various [optimizations](https://github.com/rust-lang/rustc-perf/pull/1968) to improve the performance of loading compilation profiles.

Last, but not least, Eitaro also created a completely new user interface for the benchmark suite, which runs entirely in the [terminal](https://github.com/rust-lang/rustc-perf/pull/1955). Using this interface, Rust compiler contributors can examine the performance of the compiler without having to start the rustc-perf website, which can be challenging to deploy locally.

Apart from the mentioned contributions, Eitaro also made a lot of other smaller improvements to various parts of the benchmark suite. Thank you for all your work!

### Move cargo shell completions to Rust

- Contributor: [shanmu](https://github.com/shannmu)
- Mentor: [Ed Page](https://github.com/epage)
- [Final report](https://hackmd.io/@PthRWaPvSmS_2Yu_GLbGpg/Hk-ficKpC)

Cargo's completion scripts have been hand maintained and frequently broken when changed. The goal for this effort was to have the completions automatically generated from the definition of Cargo's command-line, with extension points for dynamically generated results.

shanmu took the prototype for dynamic completions in [clap][clap] (the command-line parser used by Cargo), got it working and tested for common shells, as well as extended the parser to cover more cases. They then added extension points for CLI's to provide custom completion results that can be generated on the fly.

In the next phase, shanmu added this to nightly Cargo and added different custom completers to match what the handwritten completions do. As an example, with this feature enabled, when you type `cargo test --test=` and hit the Tab key, your shell will autocomplete all the test targets in your current Rust crate! If you are interested, see the [instructions][cargo-shell-completion] for trying this out. The link also lists where you can provide feedback.

You can also check out the following issues to find out what is left before this can be stabilized:
- [clap#3166](https://github.com/clap-rs/clap/issues/3166)
- [cargo#14520](https://github.com/rust-lang/cargo/issues/14520)

[clap]: https://github.com/clap-rs/clap
[cargo-shell-completion]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#native-completions

### Rewriting esoteric, error-prone makefile tests using robust Rust features

- Contributor: [Julien Robert](https://github.com/Oneirical)
- Mentor: [Jieyou Xu](https://github.com/jieyouxu)
- [Final report](https://oneirical.github.io/gsocfinal/)

The Rust compiler has several test suites that make sure that it is working correctly under various conditions. One of these suites is the [`run-make` test suite][run-make], whose tests were previously written using `Makefile`s. However, this setup [posed several problems][initial-run-make-issue]. It was not possible to run the suite on the Tier 1 Windows MSVC target (`x86_64-pc-windows-msvc`) and getting it running on Windows at all was quite challenging. Furthermore, the syntax of `Makefile`s is quite esoteric, which frequently caused mistakes to go unnoticed even when reviewed by multiple people.

Julien helped to convert the `Makefile`-based `run-make` tests into plain Rust-based tests, supported by a test support library called [`run_make_support`][run-make-support]. However, it was not a trivial "rewrite this in Rust" kind of deal. In this project, Julien:

- Significantly improved the test documentation;
- Fixed multiple bugs that were present in the `Makefile` versions that had gone unnoticed for *years* -- some tests were never testing anything or silently ignored failures, so even if the subject being tested regressed, these tests would not have caught that.
- Added to and improved the test support library API and implementation; and
- Improved code organization within the tests to make them easier to understand and maintain.

Just to give you an idea of the scope of his work, he has ported almost [250](https://github.com/rust-lang/rust/pulls?q=is%3Apr+author%3Aoneirical+is%3Amerged) Makefile tests over the span of his GSoC project! If you like puns, check out the branch names of Julien's PRs, as they are simply *fantestic*.

As a result, Julien has significantly improved the robustness of the `run-make` test suite, and improved the ergonomics of modifying existing `run-make` tests and authoring new `run-make` tests. Multiple contributors have expressed that they were more willing to work with the Rust-based `run-make` tests over the previous `Makefile` versions.

The vast majority of `run-make` tests [now use the Rust-based test infrastructure][run-make-tracking-issue], with a few holdouts remaining due to various quirks. After these are resolved, we can finally rip out the legacy `Makefile` test infrastructure.

[run-make]: https://github.com/rust-lang/rust/tree/master/tests/run-make
[run-make-support]: https://github.com/rust-lang/rust/tree/master/src/tools/run-make-support
[initial-run-make-issue]: https://github.com/rust-lang/rust/issues/40713
[run-make-tracking-issue]: https://github.com/rust-lang/rust/issues/121876

### Rewriting the Rewrite trait

- Contributor: [SeoYoung Lee](https://github.com/ding-young)
- Mentor: [Yacin Tmimi](https://github.com/ytmimi)
- [Final report](https://ding-young.github.io/posts/gsoc-final/)

[rustfmt] is a Rust code formatter that is widely used across the Rust ecosystem thanks to its direct integration within Cargo. Usually, you just run `cargo fmt` and you can immediately enjoy a properly formatted Rust project. However, there are edge cases in which `rustfmt` can fail to format your code. That is not such an issue on its own, but it becomes more problematic when it fails *silently*, without giving the user any context about what went wrong. This is what was happening in `rustfmt`, as many functions simply returned an `Option` instead of a `Result`, which made it difficult to add proper error reporting.

The goal of SeoYoung's project was to perform a large internal refactoring of `rustfmt` that would allow tracking context about what went wrong during reformatting. In turn, this would enable turning silent failures into proper error messages that could help users examine and debug what went wrong, and could even allow `rustfmt` to retry formatting in more situations.

At first, this might sound like an easy task, but performing such large-scale refactoring within a complex project such as `rustfmt` is not so simple. SeoYoung needed to come up with an approach to incrementally apply these refactors, so that they would be easy to review and wouldn't impact the entire code base at once. She introduced a new trait that enhanced the original `Rewrite` trait, and modified existing implementations to align with it. She also had to deal with various edge cases that we hadn't anticipated before the project started. SeoYoung was meticulous and systematic with her approach, and made sure that no formatting functions or methods were missed.

Ultimately, the refactor was a success! Internally, rustfmt now keeps track of more information related to formatting failures, including errors that it could not possibly report before, such as issues with macro formatting. It also has the ability to provide information about source code spans, which helps identify parts of code that require spacing adjustments when exceeding the maximum line width. We don't yet propagate that additional failure context as user facing error messages, as that was a stretch goal that we didn't have time to complete, but SeoYoung has expressed interest in continuing to work on that as a future improvement!

Apart from working on error context propagation, SeoYoung also made various other improvements that enhanced the overall quality of the codebase, and she was also helping other contributors understand `rustfmt`. Thank you for making the foundations of formatting better for everyone!

[rustfmt]: https://github.com/rust-lang/rustfmt

### Rust to .NET compiler - add support for compiling & running cargo tests

- Contributor: [Michał Kostrubiec](https://github.com/FractalFir)
- Mentor: [Jack Huey](https://github.com/jackh726)
- [Final report](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_2_0.html)

As was already mentioned above, the Rust compiler can be used with various codegen backends. One of these is the [.NET backend][codegen-clr], which compiles Rust code to the Common Intermediate Language (CIL), which can then be executed by the .NET Common Language Runtime (CLR). This backend allows interoperability of Rust and .NET (e.g. C#) code, in an effort to bring these two ecosystems closer together.

At the start of this year, the .NET backend was already able to compile complex Rust programs, but it was still lacking certain crucial features. The goal of this GSoC project, implemented by Michał, who is in fact the sole author of the backend, was to extend the functionality of this backend in various areas. As a target goal, he set out to extend the backend so that it could be used to run tests using the `cargo test` command. Even though it might sound trivial, properly compiling and running the Rust test harness is non-trivial, as it makes use of complex features such as dynamic trait objects, atomics, panics, unwinding or multithreading. These features were especially tricky to implement in this codegen backend, because the LLVM intermediate representation (IR) and CIL have fundamental differences, and not all LLVM intrinsics have .NET equivalents.

However, this did not stop Michał. He has been working on this project tirelessly, implementing new features, fixing various issues and learning more about the compiler's internals every new day. He has also been documenting his journey with (almost) daily [updates on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/421156-gsoc/topic/Project.3A.20Rust.20to.20.2ENET.20compiler), which were fascinating to read. Once he has reached his original goal, he moved the goalpost up to another level and attempted to run the compiler's own test suite using the .NET backend. This helped him uncover additional edge cases and also led to a refactoring of the whole backend that resulted in significant performance improvements.

By the end of the GSoC project, the .NET backend was able to properly compile and run almost 90% of the standard library `core` and `std` test suite. That is an incredibly impressive number, since the suite contains thousands of tests, some of which are quite *arcane*. Michał's pace has not slowed down even after the project has ended and he is still continuously improving the backend. Oh, and did we already mention that his backend also has experimental support for emitting *C* code, effectively acting as a *C* codegen backend?! Michał has been very busy over the summer.

We thank Michał for all his work on the .NET backend, as it was truly inspirational, and led to fruitful discussions that were relevant also to other codegen backends. Michał's next goal is to get his backend upstreamed and create an official .NET compilation target, which could open up the doors to Rust becoming a first-class citizen in the .NET ecosystem. 

[codegen-clr]: https://github.com/FractalFir/rustc_codegen_clr

### Sandboxed and deterministic proc macro using WebAssembly

- Contributor: [Apurva Mishra](https://github.com/mav3ri3k)
- Mentor: [David Lattimore](https://github.com/davidlattimore)
- [Final report](https://github.com/mav3ri3k/rust/blob/gsoc24/gsoc24.md)

Rust procedural (proc) macros are currently run as native code that gets compiled to a shared object which is loaded directly into the process of the Rust compiler. Because of this design, these macros can do whatever they want, for example arbitrarily access the filesystem or communicate through a network. This has not only obvious security implications, but it also affects performance, as this design makes it difficult to cache proc macro invocations. Over the years, there have been various discussions about making proc macros more *hermetic*, for example by compiling them to WebAssembly modules, which can be easily executed in a sandbox. This would also open the possibility of distributing precompiled versions of proc macros via crates.io, to speed up fresh builds of crates that depend on proc macros.

The goal of this project was to examine what would it take to implement WebAssembly module support for proc macros and create a prototype of this idea. We knew this would be a very ambitious project, especially since Apurva did not have prior experience with contributing to the Rust compiler, and because proc macro internals are very complex. Nevertheless, some progress was made. With the help of his mentor, David, Apurva was able to create a prototype that can load WebAssembly code into the compiler via a shared object. Some work was also done to make use of the existing `TokenStream` serialization and deserialization code in the compiler's `proc_macro` crate.

Even though this project did not fulfill its original goals and more work will be needed in the future to get a functional prototype of WebAssembly proc macros, we are thankful for Apurva's contributions. The WebAssembly loading prototype is a good start, and Apurva's exploration of proc macro internals should serve as a useful reference for anyone working on this feature in the future. Going forward, we will try to describe more incremental steps for our GSoC projects, as this project was perhaps too ambitious from the start.

### Tokio async support in Miri

- Contributor: [Tiffany Pek Yuan](https://github.com/tiif)
- Mentor: [Oli Scherer](https://github.com/oli-obk)
- [Final report](https://gist.github.com/tiif/3e08ba6e8cfb1d078e6155410108ae48)

[miri][miri] is an interpreter that can find possible instances of undefined behavior in Rust code. It is being used across the Rust ecosystem, but previously it was not possible to run it on any non-trivial programs (those that ever `await` on anything) that use [tokio][tokio], due a to a fundamental missing feature: support for the `epoll` syscall on Linux (and similar APIs on other major platforms).

Tiffany implemented the basic `epoll` operations needed to cover the majority of the tokio test suite, by crafting pure `libc` code examples that exercised those `epoll` operations, and then implementing their emulation in miri itself. At times, this required refactoring core miri components like file descriptor handling, as they were originally not created with syscalls like `epoll` in mind.

Surprising to everyone (though probably not tokio-internals experts), once these core `epoll` operations were finished, operations like async file reading and writing started working in miri out of the box! Due to limitations of non-blocking file operations offered by operating systems, tokio is wrapping these file operations in dedicated threads, which was already supported by miri.

Once Tiffany has finished the project, including stretch goals like implementing async file operations, she proceeded to contact tokio maintainers and worked with them to run miri on most tokio tests in CI. And we have good news: so far no soundness problems have been discovered! Tiffany has become a regular contributor to miri, focusing on continuing to expand the set of supported file descriptor operations. We thank her for all her contributions!

[miri]: https://github.com/rust-lang/miri
[tokio]: https://tokio.rs/

## Conclusion

We are grateful that we could have been a part of the Google Summer of Code 2024 program, and we would also like to extend our gratitude to all our contributors! We are looking forward to joining the GSoC program again next year.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc-blog-post]: https://blog.rust-lang.org/2024/05/01/gsoc-2024-selected-projects.html
[gsoc-project-list]: https://github.com/rust-lang/google-summer-of-code/blob/main/gsoc/past/2024.md
[gsoc-repo]: https://github.com/rust-lang/google-summer-of-code
[rustc-perf]: https://github.com/rust-lang/rustc-perf
