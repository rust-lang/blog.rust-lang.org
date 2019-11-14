---
layout: post
title: Evaluating GitHub Actions
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

The Rust Infrastructure team is happy to announce that we’re starting an
evaluation of [GitHub Actions](https://github.com/features/actions) as a
replacement for Azure Pipelines as the CI provider of the
[rust-lang/rust](https://github.com/rust-lang/rust) repository.

We’ve been part of the beta of GitHub Actions since the beginning, following
its development closely and testing it on a lot of smaller repositories in our
organization, and we’re really satisfied so far with the product. GitHub
Actions provides most of the features we love about Azure Pipelines, while
being integrated with GitHub’s UI, permissions and workflows.

GitHub has also offered to sponsor a dedicated pool of builders with increased
resources. Our extensive but time-consuming CI is one of the major pain points
for compiler contributors, and the additional resources have the potential to
drastically improve our developers’ experience. We have achieved 60% faster
builds in preliminary testing thanks to the increased core count in the
dedicated builder pool, and there is still large room to parallelize and finish
builds even faster.

Our plan is to start running GitHub Actions in parallel with Azure Pipelines in
the next few weeks, and we’ll keep the community updated as we learn more.

---

**[Update]** Some members of the community asked why we're considering to
switch away from Azure Pipelines so soon after migrating to it. We want to
reaffirm that we're happy with Pipelines as a product, but both Microsoft and
GitHub asked us to try GitHub Actions because it's more closely integrated into
the GitHub workflow that we already use. After we used it for a while in other
repositories we were satisfied enough to start evaluating a migration for
rustc's CI.
