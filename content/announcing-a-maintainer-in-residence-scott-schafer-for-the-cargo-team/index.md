+++
path = "2026/09/28/announcing-a-maintainer-in-residence-scott-schafer-for-the-cargo-team"
title = "Announcing a Maintainer in Residence: Scott Schafer for the Cargo team"
authors = ["Jakub Beránek"]

[extra]
team = "the Funding team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-funding"
+++

At the end of August, we [announced][mir-announcement] our first Maintainers in Residence, Rust Project contributors who are funded for their upstream contributions and [maintenance work][what-is-maintenance-post] from the [Rust Foundation Maintainers Fund][rfmf] (RFMF). Since then, the Rust Leadership Council has dedicated more funds from its Project Priorities budget to RFMF, and together with AWS also providing additional funds, this allowed us to open a new full-time Maintainer in Residence (MiR) position to support the Cargo team. We would like to thank the Rust Leadership Council, AWS, and also the Rust Foundation for providing us with this opportunity! If you would like to help us hire more maintainers to improve Rust, consider [donating][rfmf] to RFMF.

This post explains why we chose to support the Cargo team specifically, and introduces [Scott Schafer](#introducing-scott-schafer), the new Cargo Maintainer in Residence.

## Why Cargo?

The new MiR full-time position is dedicated to helping with the maintenance of [Cargo][cargo], our build system and package manager. The Cargo project is deeply involved in many new Rust features, improvements, and [Project Goals][cargo-project-goals]. Combined with its cross-cutting nature, where it has to support many different use-cases and integrate with several other tools, it takes a lot of work just to keep up with its maintenance needs, let alone support so many feature requests and proposed changes.

Because of that, the Cargo team has sometimes struggled with meeting its maintenance demands. You might remember that for several years, it actually held a [feature freeze][cargo-feature-freeze], to reduce Cargo's internal tech debt, perform necessary refactorings, go through the issue and pull request backlog, and come up with scalable internal development and design processes, so that they could eventually go back to even thinking about adding new features.

Recently, some changes occurred within the team, which made it more difficult for them to meet their maintenance baseline. Some members of the team left, while others lost their dedicated funding for working on Cargo maintenance and had to scale down their involvement. The Funding team thus considered it very important to support this team, given that we had an opportunity to do so. And thus we decided to hire a full-time maintainer to work on Cargo for (at least) the next 12 months.

Even though we know that a single full-time maintainer will not completely solve the maintenance struggles of the Cargo team, we hope that it will improve the situation, and provide a bit of a relief for the team.

## Introducing Scott Schafer

<aside style="float: right; clear: both; margin-left: 10px;">
  <img alt="A photo of Scott Schafer" src="scott.jpg" width="460" style="width: 12em; border: 1px solid white;" />
</aside>

We are very happy to welcome Scott Schafer ([@muscraft][muscraft]) into the Maintainer in Residence role! Scott has joined the Cargo team [three years ago][scott-cargo-team-invitation], and apart from working on Cargo, he is also the lead of the Rust [Docker team][docker-team], which prepares official Docker images for every Rust version.

Apart from working on general maintenance of Cargo, Scott has implemented Cargo's [Workspace inheritance][workspace-inheritance] feature, and has also spearheaded a complex multi-year effort to switch the rendering of diagnostics in the Rust compiler to use the [annotate-snippets] crate. This effort has been completed in the Rust 1.93.0 release. Thanks to it, the same diagnostics interface can now be shared between the compiler and Cargo (and also other tools), which amongst other things unblocked further development of the Cargo linting system, which has now been stabilized and will ship in the Rust 1.100.0 release.

Everyone we talked about was very excited about Scott becoming a Cargo Maintainer in Residence, and we share that feeling. We wish Scott all the best in his new role, and we are very happy that we can support his maintenance work.

Here is what Scott thinks about it:

> I am incredibly excited to work on Cargo full-time! There have been so many things that I wish I could've worked on over the years, that I will now be able to get to. I hope that my efforts will bring Cargo into a more maintainable state.

## Conclusion

We are incredibly happy that we keep getting more funds for the Rust Foundation Maintainers Fund, which allows us to support Rust Project contributors. The funding team will be working with the supported maintainers, and also the funders, to ensure that they are all happy with the arrangement, so that we can secure stable funding for Rust maintenance for years to come.

If you would like to help us support more Rust maintainers, consider [donating][rfmf] to RFMF!

[mir-announcement]: https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/
[what-is-maintenance-post]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/
[muscraft]: https://github.com/Muscraft
[rfmf]: https://github.com/sponsors/rustfoundation
[cargo]: https://github.com/rust-lang/cargo
[cargo-project-goals]: https://goals.rust-lang.org/2026/goals.html#cargo-team
[cargo-feature-freeze]: https://blog.rust-lang.org/inside-rust/2022/03/31/cargo-team-changes/
[scott-cargo-team-invitation]: https://blog.rust-lang.org/inside-rust/2023/04/06/cargo-new-members/
[docker-team]: https://rust-lang.org/governance/teams/#team-docker
[workspace-inheritance]: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-package-table
[annotate-snippets]: https://github.com/rust-lang/annotate-snippets-rs
