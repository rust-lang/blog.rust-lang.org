+++
path = "2025/05/08/gsoc-2025-selected-projects"
title = "Announcing Google Summer of Code 2025 selected projects"
authors = ["Jakub Beránek"]

[extra]
team = "the mentorship team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-mentorship"
+++

The Rust Project is [participating][gsoc blog post] in [Google Summer of Code (GSoC)][gsoc] again this year. GSoC is a global program organized by Google that is designed to bring new contributors to the world of open-source.

In March, we published a list of [GSoC project ideas][project idea list], and started discussing these projects with potential GSoC applicants on our [Zulip][zulip gsoc]. We had many interesting discussions with the potential contributors, and even saw some of them making non-trivial contributions to various Rust Project repositories, even before GSoC officially started!

After the initial discussions, GSoC applicants prepared and submitted their project proposals. We received 64 proposals this year, almost exactly the same number as last year. We are happy to see that there was again so much interest in our projects.

A team of mentors primarily composed of Rust Project contributors then thoroughly examined the submitted proposals. GSoC required us to produce a ranked list of the best proposals, which was a challenging task in itself since Rust is a big project with many priorities! Same as last year, we went through several rounds of discussions and considered many factors, such as prior conversations with the given applicant, the quality of their proposal, the importance of the proposed project for the Rust Project and its wider community, but also the availability of mentors, who are often volunteers and thus have limited time available for mentoring.

As is usual in GSoC, even though some project topics received multiple proposals[^most-popular], we had to pick only one proposal per project topic. We also had to choose between great proposals targeting different work to avoid overloading a single mentor with multiple projects.

[^most-popular]: The most popular project topic received seven different proposals!

In the end, we narrowed the list down to a smaller number of the best proposals that we could still realistically support with our available mentor pool. We submitted this list and eagerly awaited how many of them would be accepted into GSoC.

## Selected projects
On the 8th of May, Google has announced the accepted projects. We are happy to share that **19** Rust Project proposals were accepted by Google for Google Summer of Code 2025. That's a lot of projects, which makes us super excited about GSoC 2025!

Below you can find the list of accepted proposals (in alphabetical order), along with the names of their authors and the assigned mentor(s):

- **[ABI/Layout handling for the automatic differentiation feature](https://summerofcode.withgoogle.com/programs/2025/projects/USQvru7i)**	by [Marcelo Domínguez](https://github.com/sa4dus), mentored by [Manuel Drehwald](https://github.com/ZuseZ4) and [Oli Scherer](https://github.com/oli-obk)
- **[Add safety contracts](https://summerofcode.withgoogle.com/programs/2025/projects/UYWEKUkd)** by [Dawid Lachowicz](https://github.com/dawidl022), mentored by [Michael Tautschnig](https://github.com/tautschnig)
- **[Bootstrap of rustc with rustc_codegen_gcc](https://summerofcode.withgoogle.com/programs/2025/projects/KmfCY0i6)** by [Michał Kostrubiec](https://github.com/FractalFir), mentored by [antoyo](https://github.com/antoyo)
- **[Cargo: Build script delegation](https://summerofcode.withgoogle.com/programs/2025/projects/nUt4PdAA)** by [Naman Garg](https://github.com/namanlp), mentored by [Ed Page](https://github.com/epage)
- **[Distributed and resource-efficient verification](https://summerofcode.withgoogle.com/programs/2025/projects/5677hd6S)** by [Zhou Jiping](https://github.com/zjp-CN), mentored by [Michael Tautschnig](https://github.com/tautschnig)
- **[Enable Witness Generation in cargo-semver-checks](https://summerofcode.withgoogle.com/programs/2025/projects/MMRSG9WU)** by [Talyn Veugelers](https://github.com/GlitchlessCode), mentored by [Predrag Gruevski](https://github.com/obi1kenobi)
- **[Extend behavioural testing of std::arch intrinsics](https://summerofcode.withgoogle.com/programs/2025/projects/DeMQAjwi)** by [Madhav Madhusoodanan](https://github.com/madhav-madhusoodanan), mentored by [Amanieu d'Antras](https://github.com/amanieu)
- **[Implement merge functionality in bors](https://summerofcode.withgoogle.com/programs/2025/projects/HlR12jqX)** by [Sakibul Islam](https://github.com/Sakib25800), mentored by [Jakub Beránek](https://github.com/kobzol)
- **[Improve bootstrap](https://summerofcode.withgoogle.com/programs/2025/projects/2KNHAlKz)** by [Shourya Sharma](https://github.com/Shourya742), mentored by [Jakub Beránek](https://github.com/kobzol), [Jieyou Xu](https://github.com/jieyouxu) and [Onur Özkan](https://github.com/onur-ozkan)
- **[Improve Wild linker test suites](https://summerofcode.withgoogle.com/programs/2025/projects/ps99Kaqk)** by [Kei Akiyama](https://github.com/lapla-cogito), mentored by [David Lattimore](https://github.com/davidlattimore)
- **[Improving the Rustc Parallel Frontend: Parallel Macro Expansion](https://summerofcode.withgoogle.com/programs/2025/projects/SBW3GMno)** by [Lorrens](https://github.com/LorrensP-2158466), mentored by [Sparrow Li](https://github.com/sparrowlii)
- **[Make cargo-semver-checks faster](https://summerofcode.withgoogle.com/programs/2025/projects/qs2rDLG4)** by [JosephC](https://github.com/CLIDragon), mentored by [Predrag Gruevski](https://github.com/obi1kenobi)
- **[Make Rustup Concurrent](https://summerofcode.withgoogle.com/programs/2025/projects/CpXV4kzH)** by [Francisco Gouveia](https://github.com/FranciscoTGouveia), mentored by [rami3l](https://github.com/rami3l)
- **[Mapping the Maze of Rust's UI Test Suite with Established Continuous Integration Practices](https://summerofcode.withgoogle.com/programs/2025/projects/KP02lKL4)** by [Julien Robert](https://github.com/oneirical), mentored by [Jieyou Xu](https://github.com/jieyouxu)
- **[Modernising the libc Crate](https://summerofcode.withgoogle.com/programs/2025/projects/r3LkZkOy)** by [Abdul Muiz](https://github.com/mbyx), mentored by [Trevor Gross](https://github.com/tgross35)
- **[New proc-macro Server API for Rust-Analyzer](https://summerofcode.withgoogle.com/programs/2025/projects/76ekEjd1)** by [Neil Wang](https://github.com/DriedYellowPeach), mentored by [Lukas Wirth](https://github.com/veykril)
- **[Prepare stable_mir crate for publishing](https://summerofcode.withgoogle.com/programs/2025/projects/3y9x5X8O)** by [Makai](https://github.com/makai410), mentored by [Celina Val](https://github.com/celinval)
- **[Prototype an alternative architecture for cargo fix using cargo check](https://summerofcode.withgoogle.com/programs/2025/projects/fBOCR2Sp)** by [Glen Thalakottur](https://github.com/Pyr0de), mentored by [Ed Page](https://github.com/epage)
- **[Prototype Cargo Plumbing Commands](https://summerofcode.withgoogle.com/programs/2025/projects/fTDzc0sk)** by [Vito Secona](https://github.com/secona), mentored by [Cassaundra](https://github.com/cassaundra)

**Congratulations to all applicants whose project was selected!** The mentors are looking forward to working with you on these exciting projects to improve the Rust ecosystem. You can expect to hear from us soon, so that we can start coordinating the work on your GSoC projects.

We would also like to thank all the applicants whose proposal was sadly not accepted, for their interactions with the Rust community and contributions to various Rust projects. There were some great proposals that did not make the cut, in large part because of limited mentorship capacity. However, even if your proposal was not accepted, we would be happy if you would consider contributing to the projects that got you interested, even outside GSoC! Our [project idea list][project idea list] is still actual and could serve as a general entry point for contributors that would like to work on projects that would help the Rust Project maintainers and the Rust ecosystem. Some of the [Rust Project Goals][rust project goals] are also looking for help.

There is also a good chance we'll participate in GSoC next year as well (though we can't promise anything at this moment), so we hope to receive your proposals again in the future!

The accepted GSoC projects will run for several months. After GSoC 2025 finishes (in autumn of 2025), we will publish a blog post in which we will summarize the outcome of the accepted projects.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc blog post]: https://blog.rust-lang.org/2025/03/03/Rust-participates-in-GSoC-2025.html
[zulip gsoc]: https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc
[project idea list]: https://github.com/rust-lang/google-summer-of-code
[rust project goals]: https://rust-lang.github.io/rust-project-goals/2025h1/goals.html
