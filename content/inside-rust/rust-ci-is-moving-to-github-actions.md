+++
layout = "post"
date = 2020-07-23
title = "Rust's CI is moving to GitHub Actions"
author = "Pietro Albini"
team = "the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>"
+++

The Rust Infrastructure Team is happy to announce that, as part of the
[evaluation we started last year][eval], most of Rust’s CI is moving to GitHub
Actions! While we don’t expect the change to have any impact on our users, the
switch will considerably improve the experience for compiler contributors.

One of the major pain points for compiler contributors over the past few years
has been waiting for PRs to be merged. We value having an always-green master
branch, and to ensure that, we test and merge just one PR at a time, with the
other approved ones [waiting in the queue][homu]. Our CI is extensive too, with
57 machines building and testing the compiler across all the platforms we
support. On our previous system, each of those builders took between three to
four hours to finish. Combined with testing one PR at a time, this often causes
PRs to wait in the queue for days before being tested.

Making the CI setup faster is a permanent goal of the Infrastructure Team, and
GitHub Actions provided us with a great opportunity to improve landing time:
GitHub offered to sponsor a fully managed, private pool of 8-core VMs to run
our builds on, which is a big improvement compared to the 2-core VMs we were
previously using. GitHub Actions also provides most of the features we loved
about Azure Pipelines while being integrated with GitHub’s permissions and UI,
which made the switch even more fruitful.

As of July 22nd, all the CI builds for the [rust-lang/rust] repository except
for macOS builds are running exclusively on GitHub Actions! We’re still running
macOS builds on Azure Pipelines for the time being, as we’re waiting on GitHub
to fix [issue #71988][71988], but we hope to move them to GitHub Actions soon.

We’d like to thank GitHub for sponsoring our CI builders and Microsoft for the
Azure Pipelines capacity we used over the past year.

[eval]: https://blog.rust-lang.org/inside-rust/2019/11/14/evaluating-github-actions.html
[homu]: https://bors.rust-lang.org/queue/rust
[rust-lang/rust]: https://github.com/rust-lang/rust
[71988]: https://github.com/rust-lang/rust/issues/71988
