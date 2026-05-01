+++
path = "2026/05/04/outreachy-2026-may"
title = "Rust is participating in Outreachy"
authors = ["Jack Huey"]

[extra]
team = "the mentorship team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-mentorship"
+++

The Rust Project has been building up a good history of participating in various open-source mentorship programs, including [Google Summer of Code](https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/) for three years (including this year) and [previously OSPP](https://blog.rust-lang.org/2024/05/07/OSPP-2024/). We're happy to announce that this year we are also participating in [Outreachy](https://www.outreachy.org/) starting in the May 2026 cohort.

Each of these mentorship programs has different criteria for eligibility depending on who they target and the motivations of the program. Outreachy provides internships in open source, to people from any background who face underrepresentation, systemic bias, or discrimination in the technical industry where they are living. You can learn more about the Outreachy program [on their website](https://www.outreachy.org/).

## What is Outreachy and how is it different than Google Summer of Code

Outreachy is similar to Google Summer of Code (GSoC) in some aspects, but different in others. First off, unlike GSoC, Outreachy interns first apply to the overall program and only *then* can apply to specific communities. Second, while oftentimes GSoC applicants submit various contributions prior to their application, Outreachy has a dedicated period where contributions are not just optional, but required. Finally, Outreachy applicants submit an application similar to GSoC applications and communities pick interns based on those applications and the interns' contributions. Outreachy has two internship periods per year, one running from May to August (in which we are currently participating) and one from December to March.

The other major difference between Google Summer of Code and Outreachy is the source of intern stipends. For GSoC, Google graciously covers contributor stipends and overhead. For Outreachy, communities instead cover the interns' stipends and overhead.

## We are mentoring 4 interns for the May 2026 cohort

Because of limited funding availability and mentoring capacity, the Rust Project decided to select four interns for mentorship. We'll briefly share these projects below.

### Calling overloaded C++ functions from Rust

[Ajay Singh](https://www.github.com/Ajay-singh1) has been selected, mentored by [teor](https://github.com/teor2345), [Taylor Cramer](https://github.com/cramertj), and [Ethan Smith](https://github.com/thunderseethe).

This project aims to implement an experimental feature for calling overloaded C++ functions from Rust, and to begin testing that feature in a few representative use cases.

### Code coverage of the Rust compiler at scale

[Akintewe Oluwasola](https://github.com/akintewe) has been selected, mentored by [Jack Huey](https://github.com/jackh726/).

This project aims to develop the workflows to run and analyze code coverage of the compiler at the scale of the entire compiler test suite and on ecosystem crates detected by crater. The hope is to be able to detect when the compiler is inadequately tested, both within the compiler and in the ecosystem, and to build tools to do continuous analysis on this.

### Fuzzing the a-mir-formality type system implementation 

[Tunde-Ajayi Olamiposi](https://github.com/System625) has been selected, mentored by [Niko Matsakis](https://github.com/nikomatsakis/), [Rémy Rakic](https://github.com/lqd/), and [tiif](https://github.com/tiif).

This project aims to implement fuzzing for [a-mir-formality](https://github.com/rust-lang/a-mir-formality/), an in-progress model for Rust's type and trait system.  The goal is to generate programs in order to identify rules with underspecified semantics in a-mir-formality.

### Improve the security of GitHub Actions of the Rust Project 

[oghenerukevwe Sandra Idjighere](https://github.com/rukysandy) has been selected, mentored by [Marco Ieni](https://github.com/marcoieni) and [Ubiratan Soares](https://github.com/ubiratansoares).

This project aims to improve the security of GitHub Actions workflows of the repositories owned by the Rust Project. It will develop tools and workflows, integrating with existing software, to analyze Github repositories and detect if they follow the best security practices, fix existing issues, and ensure that good security practices are followed in the future.

## What's next

Over the next 3 months, the interns will work closely with their mentors to make progress on their projects. When the internship period is over, we'll write another blog post to share the results! See you then!

We also want to thank all the people that submitted applications and made contributions. It was quite tough to decide which applicants to select. Hopefully we will participate in Outreachy again in the future and there are other opportunities to participate. We also very much welcome you to stick around and continue being involved - there is a ton of places in the Rust Project with opportunities to be involved.
