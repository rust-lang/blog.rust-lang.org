+++
path = "2026/06/02/launching-the-rust-foundation-maintainers-fund"
title = "Launching the Rust Foundation Maintainers Fund"
authors = ["Funding team"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

> If you want to financially support the development of Rust, please consider [donating][rfmf-sponsors] to the Rust Foundation Maintainers Fund.

A few months ago, the Rust Foundation announced the [Rust Foundation Maintainers Fund][rfmf-announcement] (RFMF). Since then, the Rust Project has been closely cooperating with the Rust Foundation to determine how exactly this fund will be used to support Rust maintainers. This resulted in the acceptance of [RFC #3931][rfmf-rfc], which established the [Funding team][funding-team] and the [Maintainer in Residence][mir] program.

The primary goal of the Funding team is to ensure that maintainers who work on Rust and its toolchain will be properly supported. We will talk to Rust Project members to figure out their funding situation, meet Rust team leads to learn about their maintenance needs, approach companies to find opportunities for them to invest into Rust by supporting Rust maintainers, coordinate various funding efforts and ensure that the beneficial effects of funded maintenance are visibly promoted, with the help of the [Content team][content-team].

[Maintainer in Residence][mir] is a new program dedicated to financially supporting existing Rust Project maintainers[^dir]. Each Maintainer in Residence will be funded to [maintain][maintenance-post] one or more critical parts of Rust, such as the compiler, the standard library, Cargo, Clippy or one of many other projects that the Rust Project develops and maintains. The funded work will include activities such as performing large-scale refactorings, code reviews, unblocking new features, issue triaging, mentoring other contributors and more, and will be split between priorities guided by the teams they are supporting and priorities of their own choosing within the Project. Where applicable, Maintainers in Residence are also encouraged to propose, champion, and drive forward [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/).

The goal of this program is to provide stable and long-term funding so that maintainers can focus on important work that ensures the long-term health of Rust. The funding team will select Maintainers in Residence based on funding availability and maintenance needs within the Rust Project, and help ensure that they are successful. We expect that this will usually be a (near) full-time position, but that will depend on the nature of the work and the area of maintenance.

[^dir]: This program was inspired by the [Developer in Residence][developer-in-residence] concept used by the Python Software Foundation (PSF), with which we led several helpful discussions. Thank you, PSF!

This program extends our existing support for Rust maintainers, such as the [program management program](https://blog.rust-lang.org/inside-rust/2026/04/09/program-management-update-2026-03/) and the [compiler-ops program](https://blog.rust-lang.org/inside-rust/2025/06/05/a-glance-at-the-team-compiler-operations). An important development is that we now have a centralized [mechanism][rfmf-sponsors] for gathering donations from both individuals and companies, and a dedicated team that will help direct those funds to specific maintainers. You can find more details about the funding team and the Maintainer in Residence program in the [RFC][rfmf-rfc].

We expect to hire the first Maintainer in Residence in the upcoming months and announce it on this blog, so stay tuned!

## How to contribute funds

If you are an individual who wants to help Rust succeed and thrive, you can donate to the RFMF through [GitHub Sponsors][rfmf-sponsors][^sponsors]. Companies who would like to invest in better maintenance of Rust can also donate through GitHub Sponsors or they can contact the Rust Foundation [directly](mailto:contact@rustfoundation.org).

[^sponsors]: Note that the fact that GitHub Sponsors is currently enabled on the `rustfoundation` GitHub organization, and not the `rust-lang` organization, is an implementation detail that might change in the future. All donations raised on this Sponsors page will be routed to the Rust Foundation Maintainers Fund and will be spent on directly supporting Rust Project maintainers.

The important thing is that **all proceeds from this fund will be directly used to support Rust Project maintainers**. We currently expect that to happen primarily through the Maintainer in Residence program, but it can also be done in the form of smaller-scale grants or other mechanisms, as determined by the Funding team. We will figure this out on the go, as this is also quite new for us.

We really appreciate each donation, however small, because with more money we can hire more maintainers to ensure that we can continue to develop Rust and that important improvements are not blocked on maintenance tasks. This is especially important at this time, where Rust is starting to get used more and more in the industry in various application areas, which increases the need for sustained maintenance. The importance of multiple funding sources is underscored by an unfortunate trend we currently observe, where key Rust maintainers are losing their funding for Rust work due to budget shifts. The Rust Foundation Maintenance Fund is designed to provide stable funding for Rust maintainers that is less dependent on sudden shifts in the job market and the IT industry.

As with most things, there is no one-size-fits-all solution, so there are multiple ways to support Rust financially. The [RustNL Maintainers Team][rustnl-maintainers] recently hired several Rust Project maintainers. Previously, we [wrote][individual-contributors] about how you can support specific individuals working on Rust. And there are also Rust Project Goals [in search of funding][project-goals-funding]. We welcome all efforts that can help support Rust Project maintainers, who often do work that is near invisible and thankless, while at the same time incredibly important and necessary, on a volunteer basis.

Thank you for considering sponsoring the development and maintenance of Rust! You can find more information about funding Rust on our [Funding page][funding-page].

[rfmf-announcement]: https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/
[rfmf-rfc]: https://rust-lang.github.io/rfcs/3931-rfmf-rust-foundation-maintainer-fund.html
[rfmf-sponsors]: https://github.com/sponsors/rustfoundation
[content-team]: https://rust-lang.org/governance/teams/launching-pad/#team-content
[funding-team]: https://rust-lang.org/governance/teams/launching-pad/#team-funding
[mir]: https://rust-lang.github.io/rfcs/3931-rfmf-rust-foundation-maintainer-fund.html#expectations-placed-on-maintainers-in-residence
[maintenance-post]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/
[developer-in-residence]: https://www.python.org/psf/developersinresidence/
[rustnl-maintainers]: https://rustnl.org/maintainers
[individual-contributors]: https://blog.rust-lang.org/2025/12/08/making-it-easier-to-sponsor-rust-contributors/
[funding-page]: https://rust-lang.org/funding/
[project-goals-funding]: https://rust-lang.github.io/rust-project-goals/2026/funding.html
