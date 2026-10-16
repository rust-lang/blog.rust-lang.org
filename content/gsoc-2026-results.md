+++
path = "2026/11/16/gsoc-2026-results"
title = "Google Summer of Code 2026 results"
authors = ["Jakub Beránek"]

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

TODO

### Adding WebAssembly Linking Support to Wild
- Contributor: [Kei Akiyama](https://github.com/lapla-cogito)
- Mentor: [David Lattimore](https://github.com/davidlattimore)

TODO

### Bringing autodiff and offload into Rust CI
- Contributor: [Shota Sugano](https://github.com/sgasho)
- Mentor: [Manuel Drehwald](https://github.com/ZuseZ4)

TODO

### Debugger for Miri
- Contributor: [Mohamed Ali Mohamed](https://github.com/moabo3li)
- Mentor: [Oli Scherer](https://github.com/oli-obk)

TODO

### Implementing impl and mut restrictions
- Contributor: [Ryosuke Yamano](https://github.com/CoCo-Japan-pan)
- Mentors: [Jacob Pratt](https://github.com/jhpratt), [Urgau](https://github.com/Urgau)

TODO

### Improving Ergonomics and Safety of serialport-rs
- Contributor: [Tanmay](https://github.com/NONnonHere)
- Mentor: [Christian Meusel](https://github.com/sirhcel)

TODO

### libc: transition differing bit-width time and offset variants and deprecate bug-prone constants
- Contributor: [Adam Martinez](https://github.com/dybucc)
- Mentor: [Trevor Gross](https://github.com/tgross35)

TODO

### Link Linux kernel and its Modules with Wild
- Contributor: [Vishruth Thimmaiah](https://github.com/vishruth-thimmaiah)
- Mentor: [David Lattimore](https://github.com/davidlattimore)

TODO

### Migrating rust-analyzer assists to SyntaxEditor
- Contributor: [Shourya Sharma](https://github.com/Shourya742)
- Mentors: [Chayim Refael Friedman](https://github.com/ChayimFriedman2), [Lukas Wirth](https://github.com/Veykril)

TODO

### Port std::arch test suite to rust-lang/rust
- Contributor: [Sumit Kumas](https://github.com/xonx4l)
- Mentors: [Jakub Beránek](https://github.com/Kobzol), [Folkert de Vries](https://github.com/folkertdev)

The `std::arch` module of the Rust standard library contains various intrinsic functions that provide architecture and vendor-specific functionality. Their implementation lives in the [stdarch](https://github.com/rust-lang/stdarch) repository, which is included as a [Josh][josh-post] subtree in the main compiler repo (`rust-lang/rust`).

When someone changes the intrinsics in the main repo, breakages can silently creep in, because the main repo does not execute all the `stdarch` tests in its CI. That can cause friction when we synchronize the changes between the two repositories. Ideally, we would run all of `stdarch`'s tests in the main repo to detect any issues as soon as possible.

And that is where Sumit came in! Over the course of his GSoC project, he examined the (very!) complex ways the intrinsics are tested in `stdarch`, and started porting them over to the main repo. But this was not a matter of copying over a few lines of YAML. The compiler repo uses its own build system (called `bootstrap`) and a CI workflow structure that is quite different from the `stdarch` repository. Sumit thus had to first understand what is each test suite doing, reimplement it as a part of bootstrap and finally integrate it into the compiler's CI workflows.

We run ~90 CI jobs across several hardware architectures, operating systems and *OS versions*. Getting all that to work while testing *vendor-specific* intrinsics required a lot of dilligent work… and a lot of patience. It took *five months* for [one][stdarch-gen-tests] of the test suites to finally succeed on the main repo's CI, and it required Sumit to completely refactor how the suite worked, for it to be more compatible with bootstrap. As another example, a [different][intrinsic-tests] test suite that Sumir also got to work required running the tests in a `qemu` emulator on CI.

Sumit ported most of the `stdarch` test suites to `bootstrap`, including intrinsics verification against reference *C* implementations, checks for automatically generated source code being up-to-date, and various unit tests for individual intrinsics and CPU feature detection logic. There are is one [remaining][core-arch-tests] test suite to port, but the ported tests are already helping us find issues, which is great.

You can check Sumit's `rust-lang` PRs [here][xonx4l-prs]; notice that he was also contributing to other parts of Rust in the meantime! Sumit's work will help us make LLVM upgrades and land intrinsics changes more easily. Thank you, Sumit!

[josh-post]: https://blog.rust-lang.org/inside-rust/2026/06/04/how-josh-helps-rust-manage-code-across-multiple-repositories/
[core-arch-tests]: https://github.com/rust-lang/rust/pull/160850
[stdarch-gen-tests]: https://github.com/rust-lang/rust/pull/156674
[intrinsic-tests]: https://github.com/rust-lang/rust/pull/156356
[xonx4l-prs]: https://github.com/search?q=author%3Axonx4l+org%3Arust-lang&type=pullrequests

### Reorganizing tests/ui/issues
- Contributor: [zedddie](https://github.com/zedddie)
- Mentors: [Teapot](https://github.com/Teapot4195), [Kivooeo](https://github.com/Kivooeo)

TODO

### Utilize debugger APIs to improve debug info test accuracy and error reporting
- Contributor: [Anthony Bolden](https://github.com/Walnut356)
- Mentors: [Jakub Beránek](https://github.com/Kobzol), [Jieyou Xu](https://github.com/jieyouxu)

TODO

### XDG path support for rustup
- Contributor: [Guicheng Liu](https://github.com/Cloud0310)
- Mentor: [rami3l](https://github.com/rami3l)

TODO

    

## Conclusion

We would like to thank all contributors who have participated in Google Summer of Code 2026 with us! Your contributions helped make Rust better, and introduced you as new members of our community, which is a reason to celebrate. We are looking forward to participating in GSoC again next year!

We would also like to thank Google for organizing the Google Summer of Code program and for allowing us to a large number of projects every year. And last, but not least, we would like to thank all the Rust mentors who were tirelessly helping our contributors to complete their projects. Without you, Rust GSoC would not be possible.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc-blog-post]: https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/
[gsoc-project-list]: https://github.com/rust-lang/google-summer-of-code/blob/main/gsoc/runs/2026.md
[gsoc-repo]: https://github.com/rust-lang/google-summer-of-code
[contributor-guide]: https://forge.rust-lang.org/how-to-start-contributing.html
[outreachy-post]: https://blog.rust-lang.org/2026/05/04/outreachy-2026-may/
