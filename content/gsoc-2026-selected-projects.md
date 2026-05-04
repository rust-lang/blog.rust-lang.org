+++
path = "2026/04/30/gsoc-2026-selected-projects"
title = "Announcing Google Summer of Code 2026 selected projects"
authors = ["Jakub Beránek"]

[extra]
team = "the mentorship team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-mentorship"
+++

As [previously announced][gsoc blog post], the Rust Project is participating in [Google Summer of Code (GSoC)][gsoc] 2026. GSoC is a global program organized by Google that is designed to bring new contributors to the world of open source.

A few months ago, we published a list of [GSoC project ideas][project idea list], and started discussing these projects with potential GSoC applicants on our [Zulip][zulip gsoc]. We had many interesting discussions with the potential contributors, and even saw some of them making non-trivial contributions to various Rust Project repositories before GSoC officially started!

The applicants prepared and submitted their project proposals by the end of March. This year, we received 96 proposals, which is a 50% increase from last year. We are glad that there was again a lot of interest in our projects! Like many other GSoC organizations this year, we somewhat struggled with some AI-generated proposals and low-quality contributions generated using AI agents, but it stayed manageable.

GSoC requires us to produce an ordered list of the best proposals, which is always challenging, as Rust is a big project with many priorities. Our mentors examined the submitted proposals and evaluated them based on their prior interactions with the given applicant, their contributions so far, the quality of the proposal itself, but also the importance of the proposed project for the Rust Project and its wider community. We also had to take mentor bandwidth and availability into account. Unfortunately, we had to cancel some projects due to several mentors losing their funding for Rust work in the past few weeks.

As is usual in GSoC, even though some project topics received multiple proposals[^most-popular], we had to pick only one proposal per project topic. We also had to choose between proposals targeting different work to avoid overloading a single mentor with multiple projects. In the end, we narrowed the list down to the best proposals that we could still realistically support with our available mentor pool. We submitted this list and eagerly awaited how many of them would be accepted into GSoC.

[^most-popular]: The most popular project topic received fourteen different proposals!

## Selected projects
On the 30th of April, Google has announced the accepted projects. We are happy to share that **13** Rust Project proposals were accepted by Google for Google Summer of Code 2026. That is a lot of projects! We are really happy and excited about GSoC 2026!

Below you can find the list of accepted proposals (in alphabetical order), along with the names of their authors and the assigned mentor(s):

- **[A Frontend for Safe GPU Offloading in Rust](https://summerofcode.withgoogle.com/programs/2026/projects/eF3fkjrN)** by [Marcelo Domínguez](https://github.com/sa4dus), mentored by [Manuel Drehwald](https://github.com/ZuseZ4)
- **[Adding WebAssembly Linking Support to Wild](https://summerofcode.withgoogle.com/programs/2026/projects/Fx0vHvcq)** by [Kei Akiyama](https://github.com/lapla-cogito), mentored by [David Lattimore](https://github.com/davidlattimore)
- **[Bringing autodiff and offload into Rust CI](https://summerofcode.withgoogle.com/programs/2026/projects/Wg1mCCHL)** by [Shota Sugano](https://github.com/sgasho), mentored by [Manuel Drehwald](https://github.com/ZuseZ4)
- **[Debugger for Miri](https://summerofcode.withgoogle.com/programs/2026/projects/TBpqK07H)** by [Mohamed Ali Mohamed](https://github.com/moabo3li), mentored by [Oli Scherer](https://github.com/oli-obk)
- **[Implementing impl and mut restrictions](https://summerofcode.withgoogle.com/programs/2026/projects/xFrskRCv)** by [Ryosuke Yamano](https://github.com/CoCo-Japan-pan), mentored by [Jacob Pratt](https://github.com/jhpratt) and [Urgau](https://github.com/Urgau)
- **[Improving Ergonomics and Safety of serialport-rs](https://summerofcode.withgoogle.com/programs/2026/projects/g4xMTT5l)** by [Tanmay](https://github.com/NONnonHere), mentored by [Christian Meusel](https://github.com/sirhcel)
- **[libc: transition differing bit-width time and offset variants and deprecate bug-prone constants](https://summerofcode.withgoogle.com/programs/2026/projects/AObylxqh)** by [Adam Martinez](https://github.com/dybucc), mentored by [Trevor Gross](https://github.com/tgross35)
- **[Link Linux kernel and its Modules with Wild](https://summerofcode.withgoogle.com/programs/2026/projects/svblODn5)** by [Vishruth Thimmaiah](https://github.com/vishruth-thimmaiah), mentored by [David Lattimore](https://github.com/davidlattimore)
- **[Migrating rust-analyzer assists to SyntaxEditor](https://summerofcode.withgoogle.com/programs/2026/projects/lif4YQOE)** by [Shourya Sharma](https://github.com/Shourya742), mentored by [Chayim Refael Friedman](https://github.com/ChayimFriedman2) and [Lukas Wirth](https://github.com/Veykril)
- **[Port std::arch test suite to rust-lang/rust](https://summerofcode.withgoogle.com/programs/2026/projects/Aak8J6RB)** by [Sumit Kumar](https://github.com/xonx4l), mentored by [Jakub Beránek](https://github.com/Kobzol) and [Folkert de Vries](https://github.com/folkertdev)
- **[Reorganizing tests/ui/issues](https://summerofcode.withgoogle.com/programs/2026/projects/l4jlMDP9)** by [zedddie](https://github.com/zedddie), mentored by [Teapot](https://github.com/Teapot4195) and [Kivooeo](https://github.com/Kivooeo)
- **[Utilize debugger APIs to improve debug info test accuracy and error reporting](https://summerofcode.withgoogle.com/programs/2026/projects/gzkF5BG0)** by [Anthony Bolden](https://github.com/Walnut356), mentored by [Jakub Beránek](https://github.com/Kobzol) and [Jieyou Xu](https://github.com/jieyouxu)
- **[XDG path support for rustup](https://summerofcode.withgoogle.com/programs/2026/projects/jP7dTlN6)** by [Guicheng Liu](https://github.com/Cloud0310), mentored by [rami3l](https://github.com/rami3l)

**Congratulations to all applicants whose project was selected!** Our mentors are looking forward to working with you on these exciting projects to improve the Rust ecosystem. You can expect to hear from us soon, so that we can start coordinating the work on your GSoC projects.

We are excited to mentor three contributors who already experienced GSoC with us in the previous year. Welcome back, Kei, Marcelo and Shourya!

We would like to thank all the applicants whose proposal was sadly not accepted, for their interactions with the Rust community and contributions to various Rust projects. There were some great proposals that did not make the cut, in large part because of limited mentorship capacity. However, even if your proposal was not accepted, we would be happy if you would consider contributing to the projects that got you interested, even outside GSoC! Our [project idea list][project idea list] is still current and could serve as a general entry point for contributors that would like to work on projects that would help the Rust Project and the Rust ecosystem. Some of the [Rust Project Goals][rust project goals] are also looking for help.

There is a good chance we'll participate in GSoC next year as well (though we can't promise anything at this moment), so we hope to receive your proposals again in the future!

The accepted GSoC projects will run for several months. After GSoC 2026 finishes (in autumn of 2026), we will publish a blog post in which we will summarize the outcome of the accepted projects.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc blog post]: https://blog.rust-lang.org/2026/02/19/Rust-participates-in-GSoC-2026/
[zulip gsoc]: https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc
[project idea list]: https://github.com/rust-lang/google-summer-of-code
[rust project goals]: https://rust-lang.github.io/rust-project-goals/2026/goals.html
