+++
path = "2026/11/16/gsoc-2026-results"
title = "Google Summer of Code 2026 results"
authors = ["Jakub Beránek"]

[taxonomies]
tags = ["google-summer-of-code"]

[extra]
team = "the mentorship team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-mentorship"
+++

The Rust Project [participated][gsoc-blog-post] in [Google Summer of Code (GSoC)][gsoc] for the third time this year. This time, we had 13 projects, whose contributors have been working on very hard for several months. As usually, the projects had various durations, so while some of them have ended in August, the last one has only finished in November, so we have been waiting until we have all the results before publishing this blog post.

**We are very happy to announce that all 13 projects were successful and passed their final evaluation!** We consider it to be a testament to the dilligence of our contributors, and the helpfulness of our mentors, that we are able to keep a very high rate[^success-rate] of successful GSoC projects several years in a row.

[^success-rate]: 97.5% over a three-year period, if you are wondering (40 out of 41 projects were successful so far).

At the start of summer, we had a video call with our contributors, so that we could all get to know each other a bit, and learn about their projects. Over the following few months, we continuously communicated with the contributors, provided guidance and answered their questions and reviewed their pull requests. And most importantly, we learned many new things together! It was an overall great experience. We are grateful for all Rust contributions done by our GSoC participants and for the time we spent working together, and we hope that we will continue seeing at least some of them around in the future.

**We would like to thank all our Rust GSoC 2026 contributors. You did a great job!**

Even though this year we had fewer projects than in 2025, mostly due to limited mentoring capacity, Google Summer of Code 2026 was again a success for the Rust Project. We think that GSoC is a great way of introducing new contributors to our community, and we are looking forward to participating in GSoC (and other mentorship programs, such as [Outreachy][outreachy-post]) again in the near future. If you are interested in becoming a (GSoC) contributor, check out our [GSoC project idea list](https://github.com/rust-lang/google-summer-of-code) and our [guide for new contributors][contributor-guide].

Below you can find a brief summary of our GSoC 2026 projects. You can find more information about the original goals of the projects [here][gsoc-project-list]. For easier navigation, here is an index of the project descriptions in alphabetical order:


- [A Frontend for Safe GPU Offloading in Rust](#a-frontend-for-safe-gpu-offloading-in-rust) by [Marcelo Domínguez](https://github.com/sa4dus)
- [Adding WebAssembly Linking Support to Wild](#adding-webassembly-linking-support-to-wild) by [Kei Akiyama](https://github.com/lapla-cogito)
- [Bringing autodiff and offload into Rust CI](#bringing-autodiff-and-offload-into-rust-ci) by [Shota Sugano](https://github.com/sgasho)
- [Debugger for Miri](#debugger-for-miri) by [Mohamed Ali Mohamed](https://github.com/moabo3li)
- [Implementing impl and mut restrictions](#implementing-impl-and-mut-restrictions) by [Ryosuke Yamano](https://github.com/CoCo-Japan-pan)
- [Improving Ergonomics and Safety of serialport-rs](#improving-ergonomics-and-safety-of-serialport-rs) by [Tanmay](https://github.com/NONnonHere)
- [libc: transition differing bit-width time and offset variants and deprecate bug-prone constants](#libc-transition-differing-bit-width-time-and-offset-variants-and-deprecate-bug-prone-constants) by [Adam Martinez](https://github.com/dybucc)
- [Link Linux kernel and its Modules with Wild](#link-linux-kernel-and-its-modules-with-wild) by [Vishruth Thimmaiah](https://github.com/vishruth-thimmaiah)
- [Migrating rust-analyzer assists to SyntaxEditor](#migrating-rust-analyzer-assists-to-syntaxeditor) by [Shourya Sharma](https://github.com/Shourya742)
- [Port std::arch test suite to rust-lang/rust](#port-std-arch-test-suite-to-rust-lang-rust) by [Sumit Kumas](https://github.com/xonx4l)
- [Reorganizing tests/ui/issues](#reorganizing-tests-ui-issues) by [zedddie](https://github.com/zedddie)
- [Utilize debugger APIs to improve debug info test accuracy and error reporting](#utilize-debugger-apis-to-improve-debug-info-test-accuracy-and-error-reporting) by [Anthony Bolden](https://github.com/Walnut356)
- [XDG path support for rustup](#xdg-path-support-for-rustup) by [Guicheng Liu](https://github.com/Cloud0310)

And now strap in, as there is a ton of great content to read about here!
    
### A Frontend for Safe GPU Offloading in Rust
- Contributor: [Marcelo Domínguez](https://github.com/sa4dus)
- Mentor: [Manuel Drehwald](https://github.com/ZuseZ4)
- [Final report](https://sa4dus.github.io/posts/gsoc2026-final-report/)

TODO

### Adding WebAssembly Linking Support to Wild
- Contributor: [Kei Akiyama](https://github.com/lapla-cogito)
- Mentor: [David Lattimore](https://github.com/davidlattimore)
- [Final report](https://lapla.dev/posts/gsoc_final2)

The goal of this project was simple on paper: teach the [Wild][wild] linker to link WebAssembly (Wasm) programs. However, adding support for a new linker target is of course far from simple. Luckily, Kei already had a lot of experience with Wild, as this was his second [GSoC project][kei-gsoc-2025] working on the Wild linker.

And as expected, Kei did great a job! Prior to GSoC 2026, Wild had no Wasm output support. Now, the Wasm support is sufficiently good that Wild can link a range of complex programs to Wasm. For example, it is able to link itself, SQLite, ripgrep, CPython and even the SpiderMonkey JavaScript engine.

And the linking performance is excellent! When compared to `wasm-ld`, the de-facto standard Wasm linker, which is based on the LLD linker, Wild is 2-15x faster, and uses 2-5x less memory. You can find more benchmarks in his [final report][wild-linker-wasm-final-report], where he also explains that achieving this performance took quite some work, because Wild's ELF linking performance did not immediately translate to being fast when linking Wasm.

Even after this GSoC project ended, Kei has continued improving Wild's Wasm support, as well as contributing to other parts of Wild. Thank you for bringing fast linking to a new set of use-cases, Kei! You are a valued member of the Wild team.

[wild]: https://github.com/davidlattimore/wild
[kei-gsoc-2025]: https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#improve-wild-linker-test-suites
[wild-linker-wasm-final-report]: https://lapla.dev/posts/gsoc_final2#link-performance

### Bringing autodiff and offload into Rust CI
- Contributor: [Shota Sugano](https://github.com/sgasho)
- Mentor: [Manuel Drehwald](https://github.com/ZuseZ4)
- [Final report](https://sgasho.github.io/posts/gsoc-2026-final-report/)

TODO

### Debugger for Miri
- Contributor: [Mohamed Ali Mohamed](https://github.com/moabo3li)
- Mentor: [Oli Scherer](https://github.com/oli-obk)
- [Final report](https://gist.github.com/moabo3li/10b6ca701cb998ab36781e81ac0f100d)

TODO

### Implementing impl and mut restrictions
- Contributor: [Ryosuke Yamano](https://github.com/CoCo-Japan-pan)
- Mentors: [Jacob Pratt](https://github.com/jhpratt), [Urgau](https://github.com/Urgau)
- [Final report](https://gist.github.com/CoCo-Japan-pan/ced54db688080b94ec777bd750e87b34)

TODO

### Improving Ergonomics and Safety of serialport-rs
- Contributor: [Tanmay](https://github.com/NONnonHere)
- Mentor: [Christian Meusel](https://github.com/sirhcel)
- [Final report](https://nonnonhere.github.io/posts/gsoc2026-report/)

TODO

### libc: transition differing bit-width time and offset variants and deprecate bug-prone constants
- Contributor: [Adam Martinez](https://github.com/dybucc)
- Mentor: [Trevor Gross](https://github.com/tgross35)
- [Final report](https://dybucc.github.io/gsoc-2026-report/)

[`libc`][libc] is a foundational crate that is used by thousands of other crates across the Rust ecosystem. For the past several years, its maintainers have been attempting to publish a stable 1.0 release of the crate. Because this crate has to support interoperating with C on many compilation targets with weird quirks, this requires an enormous amount of work. We already had a GSoC project for moving `libc` towards 1.0 [last year][libc-gsoc-2025], and the goal of Adam's project was to make more progress in that direction.

And we can happily say that progress was indeed made! The original goal of Adam's project was to deprecate a number of constants that were difficult to maintain and cleanup inconsistencies in handling 32-bit/64-bit variants of file and time handling functions[^2038]. Adam was able to resolve that in a few weeks, and then he moved on to resolve a number of various issues blocking the [1.0 milestone][milestone].

Saying that Adam was productive would be… an understatement. We could follow his work in real-time, because he was posting [daily GSoC reports][libc-gsoc-updates] for *116 days (!) in a row*. The dilligence and consistency with which he tackled so many complicated issues was an inspiration to us. In fact, sometimes the `libc` reviewers were not able to keep up!

You can find all Adam's `libc` pull requests [here][libc-prs]. You might notice that many of them have lots of comments, and even more discussions happened in GitHub issues and on Zulip. In addition to implementing the necessary code, making changes that affect interesting C platforms and architectures also required him to do a ton of research, and a lot of considerations of edge cases and possible backwards compatibility breakages. His work also took him on a whirlwind tour of diverse platforms; Adam's contributions spanned FreeBSD, NetBSD, Android, Hurd, NuttX, AIX, VxWorks, Fuchsia, L4Re and more.

Even though more work will still be required to get `libc` to 1.0, Adam's contributions made a significant dent in this effort. Thank you very much for all your work!

[^2038]: If you were wondering, then yes, this is actually related to the famous [2038 problem][2038] :)

[libc]: https://github.com/rust-lang/libc
[libc-gsoc-2025]: https://blog.rust-lang.org/2025/11/18/gsoc-2025-results/#modernising-the-libc-crate
[milestone]: https://github.com/rust-lang/libc/issues/3248
[libc-gsoc-updates]: https://dybucc.github.io/gsoc-2026-report/daily-reports/
[libc-prs]: https://github.com/rust-lang/libc/pulls?q=is%3Apr+author%3Adybucc
[2038]: https://en.wikipedia.org/wiki/Year_2038_problem

### Link Linux kernel and its Modules with Wild
- Contributor: [Vishruth Thimmaiah](https://github.com/vishruth-thimmaiah)
- Mentor: [David Lattimore](https://github.com/davidlattimore)
- [Final report](https://vishthimmaiah.dev/blog/gsoc-26/)

Linker scripts are arcane commands that can be used to instruct a linker to construct the final binary in a custom way. The [Wild][wild] linker used to implement only basic linker script functionality, which Vishruth greatly expanded during his GSoC project.

Thanks to that, Wild can now successfully link Linux kernel modules, and it's even close to being able to link the Linux kernel itself! Vishruth's work involved greatly expanding expression evaluation support, location counters, program headers, etc. In addition to adding support for many missing bits of linker script support, Vishruth implemented support for partial linking (-r) as this is needed in order to link Linux kernel modules. You can find all the linker script features that he implemented [here][linker-script-support].

Adding all those peculiar features to Wild required performing many cross-cutting changes to its codebase, which Vishruth managed to do really well. And it was not at the cost of performance! Wild now manages to link Linux kernel modules such as `btrfs` or `amdgpu` 2-4x faster than the BFD (GNU ld) or LLD linkers[^mold]. You can find more benchmarks in his [final report][wild-linker-modules-final-report].

[^mold]: In case you were wondering, the `mold` linker does not support linker scripts required to link Linux kernel modules at the moment, so Vishruth did not compare the performance of Wild against `mold` here.

Vishruth has continued to contribute to Wild even after his GSoC project ended and has become a valued contributor to the project. Thank you very much for making Wild better, Vishruth!

[wild]: https://github.com/davidlattimore/wild
[linker-script-support]: https://github.com/wild-linker/wild/blob/main/LINKER_SCRIPT_SUPPORT.md
[wild-linker-modules-final-report]: https://vishthimmaiah.dev/blog/gsoc-26#benchmarks

### Migrating rust-analyzer assists to SyntaxEditor
- Contributor: [Shourya Sharma](https://github.com/Shourya742)
- Mentors: [Chayim Refael Friedman](https://github.com/ChayimFriedman2), [Lukas Wirth](https://github.com/Veykril)
- [Final report](TODO)

TODO

### Port std::arch test suite to rust-lang/rust
- Contributor: [Sumit Kumas](https://github.com/xonx4l)
- Mentors: [Jakub Beránek](https://github.com/Kobzol), [Folkert de Vries](https://github.com/folkertdev)
- [Final report](https://hackmd.io/@7qtUS4O9Qqadcel1SvxXiQ/BySYQX7vMe)

The `std::arch` module of the Rust standard library contains various intrinsic functions that provide architecture and vendor-specific functionality. Their implementation lives in the [stdarch](https://github.com/rust-lang/stdarch) repository, which is included as a [Josh][josh-post] subtree in the main compiler repo (`rust-lang/rust`).

When someone changes the intrinsics in the main repo, breakages can silently creep in, because the main repo does not execute all the `stdarch` tests in its CI. That can cause friction when we synchronize the changes between the two repositories. Ideally, we would run all of `stdarch`'s tests in the main repo to detect any issues as soon as possible.

And that is where Sumit came in! Over the course of his GSoC project, he examined the (very!) complex ways the intrinsics are tested in `stdarch`, and started porting them over to the main repo. But this was not a matter of copying over a few lines of YAML. The compiler repo uses its own build system (called `bootstrap`) and a CI workflow structure that is quite different from the `stdarch` repository. Sumit thus had to first understand what is each test suite doing, reimplement it as a part of bootstrap and finally integrate it into the compiler's CI workflows.

We run ~90 CI jobs across several hardware architectures, operating systems and *OS versions*. Getting all that to work while testing *vendor-specific* intrinsics required a lot of dilligent work… and a lot of patience. It took *five months* for [one][stdarch-gen-tests] of the test suites to finally succeed on the main repo's CI, and it required Sumit to completely refactor how the suite worked, for it to be more compatible with bootstrap. As another example, a [different][intrinsic-tests] test suite that Sumir also got to work required running the tests in a `qemu` emulator on CI.

Sumit ported most of the `stdarch` test suites to `bootstrap`, including intrinsics verification against reference *C* implementations, checks for automatically generated source code being up-to-date, and various unit tests for individual intrinsics and CPU feature detection logic. There is one [remaining][core-arch-tests] test suite to port, but the ported tests are already helping us find issues, which is great.

You can check Sumit's `rust-lang` PRs [here][xonx4l-prs]; notice that he was also contributing to other parts of Rust in the meantime! Sumit's work will help us make LLVM upgrades and land intrinsics changes more easily. Thank you, Sumit!

[josh-post]: https://blog.rust-lang.org/inside-rust/2026/06/04/how-josh-helps-rust-manage-code-across-multiple-repositories/
[core-arch-tests]: https://github.com/rust-lang/rust/pull/160850
[stdarch-gen-tests]: https://github.com/rust-lang/rust/pull/156674
[intrinsic-tests]: https://github.com/rust-lang/rust/pull/156356
[xonx4l-prs]: https://github.com/search?q=author%3Axonx4l+org%3Arust-lang&type=pullrequests

### Reorganizing tests/ui/issues
- Contributor: [zedddie](https://github.com/zedddie)
- Mentors: [Teapot](https://github.com/Teapot4195), [Kivooeo](https://github.com/Kivooeo)
- [Final report](https://gist.github.com/zedddie/7170d2280bc9e19da84cd7eb4e3b32e0)

TODO

### Utilize debugger APIs to improve debug info test accuracy and error reporting
- Contributor: [Anthony Bolden](https://github.com/Walnut356)
- Mentors: [Jakub Beránek](https://github.com/Kobzol), [Jieyou Xu](https://github.com/jieyouxu)
- [Final report](https://walnut356.github.io/posts/gsoc-2026-final-report/)

The Rust debugging experience leaves a [lot to be desired][rust-debuginfo-survey]. One reason why we struggled to improve it is that many of our debuginfo test suites did not even run in CI for the past few years, as they were difficult to maintain. When something changed in the type visualizers, the implementation details of a standard library type or in debugger internals, someone had to go through tens of tests and manually update them. This was not sustainable, so the test suite rotted.

Luckily for us, Anthony took that affront personally. Even before his GSoC project, he spent considerable effort on improving the quality of debug visualizers of various Rust standard library types, and gained an impressive amount of knowledge about debuggers.

During his GSoC project, he completely overhauled how our debuginfo test infrastructure works, both for [LLDB][lldb-repr] and [GDB][gdb-repr]. Thanks to his improvements, we can now easily "rebless" the reference output when something changes, which makes it much easier to keep our test suite up-to-date. The new tests also record much more information about the internal state of the debugger recorded during the test, which helps with diagnosing issues that cause type visualizers to stop working.

In addition to redesigning the test suite, Anthony also actually managed to re-enable the LLDB tests on both Windows and Linux in our CI again (after several years of being skipped there!). This will allow us to avoid debuginfo regressions and make it easier to improve the Rust debugging experience. Oh, and it goes without saying that while doing all that, he also made numerous improvements to Rust type visualizers. You can find his PRs [here][walnut-prs].

Anthony's amazing contributions and his deep expertise in debuggers led to him being invited to the Rust [Compiler team][compiler-team], which was much deserved. What else can we add, other than: thank you for your awesome work!

[rust-debuginfo-survey]: https://blog.rust-lang.org/2026/09/07/rust-debugging-survey-2026-results/
[lldb-repr]: https://github.com/rust-lang/rust/pull/158298
[gdb-repr]: https://github.com/rust-lang/rust/pull/160377
[walnut-prs]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+state%3Amerged+author%3Awalnut356
[compiler-team]: https://rust-lang.org/governance/teams/compiler/#team-compiler

### XDG path support for rustup
- Contributor: [Guicheng Liu](https://github.com/Cloud0310)
- Mentor: [rami3l](https://github.com/rami3l)
- [Final report](https://cloud0310.github.io/posts/2026-GSoC-project-report.html)

The support for platform-specific directories (platformdirs) has been a long-standing feature request for rustup, to have more fine-grained filesystem directory choices. This project investigated how that issue could be resolved.

The real complexity of this feature request lies within a decade worth of different assumptions that previous iterations of rustup and cargo have made: it turns out that quite a lot of them are no longer true if we were to implement platformdirs support truly faithfully.

Guicheng first worked on cleaning up the interactions between `rustup self uninstall` and `$CARGO_HOME/bin` as a warmup. This issue itself is a very good demonstration of all that compatibility baggage: somehow `$CARGO_HOME/bin` has become the default binary directory for all things Rust, so rustup ended up installing itself and all its proxies into it. As a result, it has been super annoying to many that everything in `$CARGO_HOME` is gone with `rustup self uninstall`.

After that, he started the actual implementation of platformdirs support in rustup. This was no easy task! Not because the code itself was hard to write, but because it takes a lot of effort to figure out what should be the desired behavior in the first place. Also, in order to determine the appropriate platformdirs semantics, some prior research is often required to see how things became what they are today, so the decoupling of different concerns could proceed, and migration paths could be designed accordingly. It was in this sense that this is different from a typical GSoC project, and luckily thanks to the effort he has put into this project, we have not only come up with the final patch, but also discovered and fixed some tiny preexisting issues with this long-untouched aspect of the rustup codebase along the way.

Thank you for digging into this complex topic, Guicheng!

    

## Conclusion

We would like to thank all contributors who have participated in Google Summer of Code 2026 with us! Your contributions helped make Rust better, and introduced you as new members of our community, which is a reason to celebrate. We are looking forward to participating in GSoC again next year!

We would also like to thank Google for organizing the Google Summer of Code program and for allowing us to a large number of projects every year. And last, but not least, we would like to thank all the Rust mentors who were tirelessly helping our contributors to complete their projects. Without you, Rust GSoC would not be possible.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc-blog-post]: https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/
[gsoc-project-list]: https://github.com/rust-lang/google-summer-of-code/blob/main/gsoc/runs/2026.md
[gsoc-repo]: https://github.com/rust-lang/google-summer-of-code
[contributor-guide]: https://forge.rust-lang.org/how-to-start-contributing.html
[outreachy-post]: https://blog.rust-lang.org/2026/05/04/outreachy-2026-may/
