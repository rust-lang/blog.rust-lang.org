+++
path = "2024/05/01/gsoc-2024-selected-projects"
title = "Announcing Google Summer of Code 2024 selected projects"
authors = ["Jakub Beránek, Jack Huey and Paul Lenz"]
aliases = ["2024/05/01/gsoc-2024-selected-projects.html"]
+++

The Rust Project is [participating][gsoc blog post] in [Google Summer of Code (GSoC) 2024][gsoc], a global program organized by Google which is designed to bring new contributors to the world of open-source.

In February, we published a list of [GSoC project ideas][project idea list], and started discussing these projects with potential GSoC applicants on our [Zulip][zulip gsoc]. We were pleasantly surprised by the amount of people that wanted to participate in these projects and that led to many fruitful discussions with members of various Rust teams. Some of them even immediately began contributing to various repositories of the Rust Project, even before GSoC officially started!

After the initial discussions, GSoC applicants prepared and submitted their project proposals. We received 65 (!) proposals in total. We are happy to see that there was so much interest, given that this is the first time the Rust Project is participating in GSoC.

A team of mentors primarily composed of Rust Project contributors then thoroughly examined the submitted proposals. GSoC required us to produce a ranked list of the best proposals, which was a challenging task in itself since Rust is a big project with many priorities! We went through many rounds of discussions and had to consider many factors, such as prior conversations with the given applicant, the quality and scope of their proposal, the importance of the proposed project for the Rust Project and its wider community, but also the availability of mentors, who are often volunteers and thus have limited time available for mentoring.

In many cases, we had multiple proposals that aimed to accomplish the same goal. Therefore, we had to pick only one per project topic despite receiving several high-quality proposals from people we'd love to work with. We also often had to choose between great proposals targeting different work within the same Rust component to avoid overloading a single mentor with multiple projects.

In the end, we narrowed the list down to twelve best proposals, which we felt was the maximum amount that we could realistically support with our available mentor pool. We submitted this list and eagerly awaited how many of these twelve proposals would be accepted into GSoC.

## Selected projects
On the 1st of May, Google has announced the accepted projects. We are happy to announce that `9` proposals out of the twelve that we have submitted were accepted by Google, and will thus participate in Google Summer of Code 2024! Below you can find the list of accepted proposals (in alphabetical order), along with the names of their authors and the assigned mentor(s):

- **[Adding lint-level configuration to cargo-semver-checks](https://summerofcode.withgoogle.com/programs/2024/projects/hADSyIDV)** by Max Carr, mentored by Predrag Gruevski
- **[Implementation of a Faster Register Allocator For Cranelift](https://summerofcode.withgoogle.com/programs/2024/projects/zxxeGZMt)** by d-sonuga, mentored by Chris Fallin and Amanieu d'Antras
- **[Improve Rust benchmark suite](https://summerofcode.withgoogle.com/programs/2024/projects/MeyNanKI)** by s7tya, mentored by Jakub Beránek
- **[Move cargo shell completions to Rust](https://summerofcode.withgoogle.com/programs/2024/projects/jjnidpgn)** by shanmu, mentored by Ed Page
- **[Rewriting Esoteric, Error-Prone Makefile Tests Using Robust Rust Features](https://summerofcode.withgoogle.com/programs/2024/projects/P5BC91Hr)** by Julien Robert, mentored by Jieyou Xu
- **[Rewriting the Rewrite trait](https://summerofcode.withgoogle.com/programs/2024/projects/gHEu3vxc)** by SeoYoung Lee, mentored by Yacin Tmimi
- **[Rust to .NET compiler - add support for compiling & running cargo tests](https://summerofcode.withgoogle.com/programs/2024/projects/IIHP5ozV)** by Fractal Fir, mentored by Jack Huey
- **[Sandboxed and Deterministic Proc Macro using Wasm](https://summerofcode.withgoogle.com/programs/2024/projects/kXG0mZoj)** by Apurva Mishra, mentored by David Lattimore
- **[Tokio async support in Miri](https://summerofcode.withgoogle.com/programs/2024/projects/rk1Ey4hN)** by Tiffany Pek Yuan, mentored by Oli Scherer

**Congratulations to all applicants whose project was selected!** The mentors are looking forward to working with you on these exciting projects to improve the Rust ecosystem. You can expect to hear from us soon, so that we can start coordinating the work on your GSoC projects. 

We would also like to thank all the applicants whose proposal was sadly not accepted, for their interactions with the Rust community and contributions to various Rust projects. There were some great proposals that did not make the cut, in large part because of limited review capacity. However, even if your proposal was not accepted, we would be happy if you would consider contributing to the projects that got you interested, even outside GSoC! Our [project idea list][project idea list] is still actual, and could serve as a general entry point for contributors that would like to work on projects that would help the Rust Project maintainers and the Rust ecosystem.

Assuming our involvement in GSoC 2024 is successful, there's a good chance we'll participate next year as well (though we can't promise anything yet) and we hope to receive your proposals again in the future! We also are planning to participate in similar programs in the very near future. Those announcements will come in separate blog posts, so make sure to subscribe to this blog so that you don't miss anything.

The accepted GSoC projects will run for several months. After GSoC 2024 finishes (in autumn of 2024), we plan to publish a blog post in which we will summarize the outcome of the accepted projects.

[gsoc]: https://summerofcode.withgoogle.com
[gsoc blog post]: https://blog.rust-lang.org/2024/02/21/Rust-participates-in-GSoC-2024.html
[zulip gsoc]: https://rust-lang.zulipchat.com/#narrow/stream/421156-gsoc
[project idea list]: https://github.com/rust-lang/google-summer-of-code
