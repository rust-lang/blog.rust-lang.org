+++
path = "2026/10/26/how-to-fund-rust-contributors"
title = "How to fund Rust contributors"
authors = ["Jakub Beránek"]

[extra]
team = "the Funding team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-funding"
+++

> TLDR:
>
> - To fund general maintenance, donate to the [Rust Foundation Maintainers Fund][rfmf] or sponsor the [RustNL Maintainers Team][rustnl-maintainers]
> - To fund specific features and improvements, sponsor [fundable Rust Project Goals][goals-looking-for-funding]
> - To fund individual contributors, donate to their [GitHub Sponsors][sponsor-individual-contributors] accounts

There is always so much to do in the Rust Project! New features to implement, bugs to fix, performance optimizations to make, issues to triage… It takes a lot of work to keep our high bar of quality while also shipping new features and improvements. And the workload keeps increasing with the continuous growth of Rust adoption.

As is unfortunately common in open source projects, a large fraction of that work is currently done by volunteers. That causes various issues, such as people burning out or complex features taking many years to ship.

Unsurprisingly, one of the best ways to help improve that is to get more people who work on Rust funded! This year, several funding initiatives started ramping up, and we already experience many positive effects. They helped us improve the maintenance status of critical Rust projects and make significant progress on several long-awaited features, such as allocators, the never type, the next trait solver or the Polonius borrow checker. However, only a fraction of people working on Rust are currently funded for their work.

We know that there are Rust users, both individuals and companies, who would like to help improve Rust by funding it, but it might not always be obvious *how exactly* to do that. This post thus provides suggestions on how you can support the development of Rust in three ways.

## Fund general maintenance

[Maintenance][what-is-maintenance-post] is a continuous process required to keep Rust (or any software project for that matter) working. It consists of two main activities. The first is general housekeeping, such as ensuring that Continuous Integration works, dependencies are kept up-to-date and issues, bugs and security incidents are promptly triaged and resolved. The second is performing refactorings and code reviews, which ensures that the codebase will be able to evolve and continuously ship new features, without accruing extensive technical debt.

Having quality maintenance is crucial for the long-term health of Rust, yet it is typically very difficult to find funding for it; funders often want to sponsor flashy new features, rather than just "keeping the lights on". However, without sufficient maintenance, it becomes much more difficult, and much slower, to ship new features. Improving the maintenance of Rust thus also helps us ship features and improvements faster, though the effects are sometimes only observable over longer periods of time.

Funding maintenance is a great choice if you want to let us decide how best to improve Rust. The Rust [Funding team][funding-team] continuously looks for Rust teams and projects that require help with their maintenance efforts, and allocates available funds to Rust maintainers who could help improve the situation.

**To fund Rust maintainers, consider donating to the [Rust Foundation Maintainers Fund][rfmf] or sponsoring the [RustNL Maintainers Team][rustnl-maintainers].**

## Fund specific features and improvements

There are always many Rust improvements and new features that are in progress. We track some of those initiatives through [Rust Project Goals][project-goals]. Each Goal describes a specific improvement to Rust: what it is, who is going to develop and drive it forward and which Rust teams are willing to help reviewing and maintaining it.

Some of those Goals are looking for funding. Either to help their owner dedicate time and focus to push the given feature forward, or to fund maintainers who will perform refactorings necessary to unblock that feature and review its implementation. Often funding is required for both. When a Goal is properly funded, the probability that it will achieve its milestones is much higher, which then translates in the given improvement being shipped sooner to Rust users.

Funding Project Goals is a great choice if you want to help push forward specific Rust features or Rust toolchain improvements that will benefit you or your company.

**To fund contributors working on Project Goals, consider taking a look at [Goals that need funding][goals-looking-for-funding] and reaching out to their funding contact.** If in doubt how to proceed, reach out to the Rust [Funding team](mailto:funding@rust-lang.org).

## Fund individual contributors

There are [hundreds][rust-project-members] of people in the Rust Project who continuously work on improving Rust. Some of them have personal fundraising profiles (e.g. on GitHub Sponsors), which allows you to support them directly.

There might be various reasons for supporting a specific person. Maybe they are maintaining a crate that you love to use, maybe you like reading their blog posts, or maybe you are simply a fan of the work that they do. Whatever the reason, we are sure that they would appreciate your support!

Funding individual contributors is a great choice if you want to directly support Rust contributors whose work you appreciate.

**To fund individual Rust contributors, check out their GitHub profile or personal website to see what options there are for supporting them.** You can also examine our [list][sponsor-individual-contributors] of Rust Project members who can be supported through GitHub Sponsors[^non-exhaustive].

[^non-exhaustive]: Note that this list is not exhaustive; there are some Project members who accept donations through GitHub Sponsors (or another fundraising platform), but did not opt into being listed on this page.

## Conclusion

We hope that this post is useful to anyone who would potentially like to invest funds into making Rust better. You can also find a shortened version of the "how to fund Rust guide" at the [Funding page][funding-page] of the Rust website, or check out a RustConf 2026 talk by [Niko Matsakis][niko-funding-talk] about how Project Goals and their funding works.

If you are a company looking to fund the development of Rust, consider joining the [Rust Commercial Network][rust-commercial-network]. It can help you find opportunities for funding improvements in specific areas related to Rust usage (e.g. user interfaces, language interop, supply chain security, etc.) and collaborate on funding Rust development with other companies.

There are of course other ways of helping Rust other than donating funds. For example, you can check out Project Goals that are [looking for contributors][goals-looking-for-contributors] or more generally check our [contributor onboarding guide][contributor-onboarding]. Companies can also help by giving employees paid time to contribute upstream, or by hiring Rust contributors to do that work.

[what-is-maintenance-post]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/
[funding-page]: http://rust-lang.org/funding
[funding-team]: https://www.rust-lang.org/governance/teams/launching-pad#team-funding
[sponsor-individual-contributors]: https://rust-lang.org/funding/#people
[rust-project-members]: https://rust-lang.org/governance/people/
[rfmf]: https://github.com/sponsors/rustfoundation
[rustnl-maintainers]: https://rustnl.org/maintainers/
[niko-funding-talk]: https://www.youtube.com/watch?v=-VLo5g-AfJ8
[project-goals]: https://goals.rust-lang.org/2026/goals.html
[goals-looking-for-contributors]: https://goals.rust-lang.org/2026/contributors.html
[goals-looking-for-funding]: https://goals.rust-lang.org/2026/funding.html
[contributor-onboarding]: https://forge.rust-lang.org/how-to-start-contributing.html
[rust-commercial-network]: https://rustfoundation.org/rust-commercial-network/
