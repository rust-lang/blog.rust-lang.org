+++
path = "inside-rust/2025/10/16/infrastructure-team-q3-recap-and-q4-plan"
title = "Infrastructure Team 2025 Q3 Recap and Q4 Plan"
authors = ["Marco Ieni"]

[extra]
team = "The Rust Infrastructure Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-infra"
+++

As we wrap up the third quarter of 2025, the Infrastructure Team is excited to share what we've accomplished and what's coming next.

## Q3 2025 Accomplishments

### crates-io-auth-action

We created the [`crates-io-auth-action`](https://github.com/rust-lang/crates-io-auth-action), which can be used to obtain publish tokens for `crates.io`
using [trusted publishing](https://crates.io/docs/trusted-publishing).

### Critical data asset backup

One of our most important security improvements this quarter was establishing an out-of-band backup system for Rust's critical data assets.
Previously, all of Rust's releases and crates were stored exclusively on AWS.
We've now created a backup in Google Cloud Platform that includes all Rust
releases and crates, with daily incremental updates.

The GCP account is owned by two Rust Foundation staff members who are not AWS administrators
(i.e., they aren't admins on the Infrastructure Team).
This makes it significantly more difficult for an attacker to compromise or delete both the primary and backup data, as they would need to breach at least two separate accounts.

Learn more about this initiative in our
[docs](https://github.com/rust-lang/infra-team/blob/3292c4614889ac7427dc4729bd0ad3ee97ab5be7/service-catalog/rust-assets-backup/README.md).

### CDN alerts

The CDNs for Rust releases and crates are critically important for users of Rust. If either suffers a service disruption, it directly impacts users' ability to install Rust and build their projects.

In the past, we had set up integrations for AWS CloudFront and Fastly, our Content Delivery Networks, with Datadog, our monitoring platform. While we had metrics coming into Datadog in real time, we had not set up alerts on top of them.

In Q3, we created multiple alerts that monitor our CDNs and report when traffic falls below predefined thresholds. These alerts ping the engineering team at the Rust Foundation, which has a runbook for incidents, including notifying the community.

GitHub issue: [rust-lang/infra-team#179](https://github.com/rust-lang/infra-team/issues/179).

### `rust-lang.org` is now a static website

We converted [`rust-lang.org`](http://rust-lang.org) from a Rust web server deployed on Heroku to a static website hosted on GitHub Pages.
This change eliminates several security and operational concerns:

- **Enhanced security**: Static sites are inherently more secure and less vulnerable to DDoS attacks.
- **Simplified infrastructure**: Removes the complexity of managing a web server, which is unnecessary
  to serve static content.
- **Cost reduction**: GitHub Pages hosting is free, while the Foundation was paying for the previous
  Heroku deployment.
- **Improved robustness**: Static hosting is generally more reliable and performant.

Thanks to [Manishearth](https://github.com/Manishearth) and [senekor](https://github.com/senekor) from the website team for their reviews and support.

GitHub PR: [rust-lang/www.rust-lang.org#2174](https://github.com/rust-lang/www.rust-lang.org/issues/2174).

### GitHub organization members cleanup

The Infra Team created an automated process to clean up all GitHub organizations managed by the Rust Project by removing members who are not part of any team within those organizations.
You can learn more in the Inside Rust Blog [post](https://blog.rust-lang.org/inside-rust/2025/08/26/removing-inactive-members-from-github-organizations/).

Thanks to [me-diru](https://github.com/me-diru) for starting the implementation.

### The new Bors runs Rust CI try builds

The Infra and [Bors](https://rust-lang.org/governance/teams/infra/#team-infra-bors)
teams continued working on migrating the Rust CI from the legacy Bors ([Homu](https://github.com/rust-lang/homu))
to the new [Bors](https://github.com/rust-lang/bors), written in Rust.

Starting in July, all try builds (`@bors try`) have run exclusively through the new bors.
This is a significant step forward in improving the reliability of our continuous integration infrastructure.

GitHub PR: [rust-lang/bors#352](https://github.com/rust-lang/bors/pull/352)

The GSoC project for implementing merge functionality in Bors has concluded (we will post a blog post in the upcoming weeks that will describe the results of all our GSoC projects). We plan to deploy bors to production and fully replace homu by it in the upcoming months.

### Support optional CI jobs

Until now, CI only had jobs that ran on PRs or when a merge to the default branch was attempted (i.e., the auto build).
We added support for CI jobs that run only on demand, allowing contributors to run these tests on PRs whenever they are concerned about breaking certain parts of the codebase. This is useful for testing tier 2 and tier 3 targets in CI, whose tests don’t always run on CI by default.

This feature was requested in [\#t-infra \> Testing for T2/T3 targets](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Testing.20for.20T2.2FT3.20targets/with/526715751) and documented in [rust-lang/rust#143283](https://github.com/rust-lang/rust/pull/143283).

### Automating subtree synchronization

We have implemented a new tool called [`rustc-josh-sync`](https://github.com/rust-lang/josh-sync), which makes it easier to synchronize changes between [Josh](https://josh-project.github.io/josh/intro.html) subtrees and the main `rust-lang/rust` repository. We have migrated the existing Josh subtrees (such as `miri` or `rust-analyzer`) to this tool, which enables us to perform subtree synchronizations in a more unified way.

### Implementing support for multiple machines for rustc-perf

We made a lot of progress with the Rust project goal for [improving](https://rust-lang.github.io/rust-project-goals/2025h1/perf-improvements.html) our compiler benchmark suite. The benchmark suite is now able to run benchmarks on multiple machines (called collectors) in parallel. This will enable us to reduce the latency of waiting for benchmark results, and also allow us to benchmark the compiler on different architectures than just x64 (e.g. ARM) and in theory even on different OSes than just Linux (e.g. Windows). The project goal has been carried over to the next goal period. We plan to deploy the new system into production in the upcoming months.

### Repository default branch renames

We're updating our repositories to use more inclusive naming conventions. This quarter, we renamed the default branches from `master` to `main` in the [`rustc-dev-guide`](https://github.com/rust-lang/rustc-dev-guide) ([PR](https://github.com/rust-lang/rustc-dev-guide/pull/2570)), [`www.rust-lang.org`](https://github.com/rust-lang/www.rust-lang.org) ([PR](https://github.com/rust-lang/www.rust-lang.org/pull/2205)) and [`blog.rust-lang.org`](https://github.com/rust-lang/blog.rust-lang.org) ([PR](https://github.com/rust-lang/blog.rust-lang.org/pull/1689/)) repositories.

Thanks to [senekor](https://github.com/senekor), [tshepang](https://github.com/tshepang), [carols10cents](https://github.com/carols10cents), and everyone else involved.

### Talks and interviews

At RustConf 2025:

- [JD](https://github.com/jdno) was interviewed about the Rust Infrastructure team. You can find the video [here](https://www.youtube.com/watch?v=r7i-2wHtNjw).
- [Marco](https://github.com/marcoieni) presented the talk *"How We Made the Rust CI 75% Cheaper"*. You can find the video [here](https://www.youtube.com/watch?v=Gzk4uG-YzJI).

## Q4 2025 Plans

Looking ahead to Q4, we have planned the following initiatives:

### Hire a new Rust Foundation infrastructure engineer

[JD](https://github.com/jdno), one of the two Infrastructure Engineers employed full-time by the Rust Foundation, resigned to start his own company.
JD is staying on the team as a volunteer, but he will be able to dedicate less time than before.
This quarter, we want to hire a new Infrastructure Engineer to restore the previous capacity of the team.
If you are interested, check the Rust Foundation [careers page](https://rustfoundation.org/careers/).

We would like to take this opportunity to thank JD for his three years of
invaluable contributions and support to the Rust Infrastructure Team and the Rust community.
To learn more about his transition, you can read his [blog post](https://www.jdno.dev/leaving-the-rust-foundation/).

### docs.rs infrastructure modernization

[docs.rs](https://docs.rs) is still deployed to a single, manually provisioned and managed EC2 instance.
We want to collaborate with the [docs.rs](http://docs.rs) team to understand what kind of infrastructure would best fit [docs.rs](http://docs.rs) and to provision it, making [docs.rs](http://docs.rs) more robust and scalable.

### External hardware CI policy

Today, we run CI only on AWS and GitHub-hosted runners, which are operated by us.
Unfortunately, these cloud providers don't support all Rust [targets](https://doc.rust-lang.org/rustc/platform-support.html), and emulation has limitations.
To raise the tier of some of these targets,
some organizations have offered to run the Rust CI on their own hardware.

We want to write a policy to define the requirements that the external hardware and the entity operating it must satisfy.
For example, if the hardware has an uptime of 50%, we can't run CI jobs on it, because it would block the development of Rust.

GitHub issue: [rust-lang/infra-team\#201](https://github.com/rust-lang/infra-team/issues/201)

### GCP Dev Desktops

Google donated some GCP credits to the Rust Foundation for next year.
We plan to use part of the credits to spin up one or two dev desktops in GCP to provide more VMs to contributors working on Rust.

Note that these machines can be discontinued in the future based on funding.

Learn more about Dev Desktops in the [Rust Forge](https://forge.rust-lang.org/infra/docs/dev-desktop.html).

### Conferences

Some members of the Infrastructure team will attend [EuroRust](https://www.eurorust.eu/) and [RustLab](https://rustlab.org/).
Feel free to reach out to us if you want to meet in person!

In particular, on November 4th, [Marco](https://github.com/marcoieni) is giving a talk at RustLab titled *“1.5 years in the infra team: what we cooked and what’s next”*.

## Join us!

If you're interested in contributing to Rust's infrastructure, have a look at the
[infra-team](https://github.com/rust-lang/infra-team) repository to learn more about us
and reach out on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra).

We are always looking for new contributors!
