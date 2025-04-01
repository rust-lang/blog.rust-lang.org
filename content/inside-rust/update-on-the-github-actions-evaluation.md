+++
path = "inside-rust/2020/04/07/update-on-the-github-actions-evaluation"
title = "Update on the GitHub Actions evaluation"
authors = ["Pietro Albini"]
aliases = ["inside-rust/2020/04/07/update-on-the-github-actions-evaluation.html"]

[extra]
team = "the infrastructure team"
team_url = "https://www.rust-lang.org/governance/teams/operations#infra"
+++

The infrastructure team is happy to report that [the evaluation we started last
year][prev] of [GitHub Actions][gha] as the new CI platform for the
[rust-lang/rust] repository is making progress!

On March 20th, 2020 we merged [PR #70190][70190], adding the GitHub Actions
configuration to the compiler repository. We’re still gating merges on the
existing Azure Pipelines setup, but running the two providers side by side
allows us to find bugs in the GitHub Actions configuration without impacting
the work of our contributors.

Once [all outstanding issues][gha-issues] are fixed, the Infrastructure Team
will make the decision to either switch to GitHub Actions or stay on Azure
Pipelines. We expect the decision to happen in a couple of months.

## What’s changing with GitHub Actions?

This change should have no visible effect to any user of Rust, but will greatly
improve the experience of our contributors.

The main difference our contributors are going to notice is a big reduction of
our CI times. In the current Azure Pipelines setup builds regularly take more
than 3 hours to finish (with 60 parallel 2-core VMs), while we expect the new
GitHub Actions setup to take less than half the time to finish a build, thanks
to a dedicated pool of 8-core VMs GitHub generously prepared for us.

Another technical change is that we’re now running most CI builds on the
[rust-lang-ci/rust] fork. This should only impact team members that want to get
a list of all the past builds, and should be completly transparent to everyone
else thanks to our integration bot [@bors].

## What configuration is the project using?

Our CI configuration is available at [`src/ci/github-actions/ci.yml`][config].
Note that our configuration is not using the standard GitHub Actions syntax,
but we’re relying on a preprocessor to expand YAML anchors to ease the
maintenance work on our end.

## Why are you moving away from Azure Pipelines?

We're happy with Azure Pipelines as a product, but both Microsoft and GitHub
asked us to try GitHub Actions as it's more closely integrated into the GitHub
workflow we already use. After we used it for a while in other repositories we
were satisfied enough to start evaluating a migration for [rust-lang/rust].

[prev]: https://blog.rust-lang.org/inside-rust/2019/11/14/evaluating-github-actions.html
[gha]: https://github.com/features/actions
[rust-lang-ci/rust]: https://github.com/rust-lang-ci/rust
[rust-lang/rust]: https://github.com/rust-lang/rust
[70190]: https://github.com/rust-lang/rust/pull/70190
[gha-issues]: https://github.com/rust-lang/rust/labels/A-github-actions
[@bors]: https://github.com/rust-lang/homu
[config]: https://github.com/rust-lang/rust/blob/master/src/ci/github-actions/ci.yml
