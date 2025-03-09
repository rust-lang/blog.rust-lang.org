+++
layout = "post"
date = 2023-02-08
title = "DNS Outage on 2023-01-25"
author = "Jan David Nose"
team = "The Rust Infrastructure Team <https://www.rust-lang.org/governance/teams/infra>"
+++

On Wednesday, 2023-01-25 at 09:15 UTC, we deployed changes to the production
infrastructure for crates.io. During the deployment, the DNS record for
`static.crates.io` failed to resolve for an estimated time of 10-15 minutes.
Users experienced build failures during this time, because crates could not be
downloaded. Around 9:30 UTC, the DNS record started to get propagated again and
by 9:40 UTC traffic had returned to normal levels.

## Root Cause of the Outage

The Rust infrastructure is managed with Terraform, a tool to configure and
provision infrastructure-as-code. The [Infrastructure team] recently made
changes to this configuration to separate the `staging` and `production`
environments for crates.io so that both can be deployed independently of each
other.

This feature was used to develop and test the infrastructure for a second
Content Delivery Network (CDN) for `static.crates.io` in the `staging`
environment. When the configuration was ready, we
[scheduled and announced](https://blog.rust-lang.org/inside-rust/2023/01/24/content-delivery-networks.html)
the rollout for January 25th.

The deployment to `production` contained two changes that were developed,
deployed, and tested individually on `staging`: a new TLS certificate for the
current Content Delivery Network and updated DNS records.

When we deployed this configuration to `production`, Terraform first removed the
current certificate and DNS records. It then started to issue a new certificate,
which took around 10 minutes. During this time, there was no DNS record for
`static.crates.io` and users experienced build failures. After the new
certificate was provisioned, Terraform recreated the DNS records.

## Resolution

The outage resolved itself after Terraform finished the deployment and created a
new DNS record for `static.crates.io`. For some users, the outage lasted a few
minutes longer due to caches in their DNS server.

## Postmortem

The outage could have been avoided by deploying the changes to the TLS
certificate and DNS records individually. We have identified two reasons why
this did not happen as well as lessons that we are taking from this.

This was one of the first times that we used the new tooling around environments
to deploy changes to `production`. One of its features is that the `production`
environment is locked to a specific Git commit. When deploying in the past, we
set this to the latest commit on `master`. This was done here as well, with the
consequence that the deployment applied multiple changes simultaneously.

Another way to look at this is that `production` and `staging` diverged too much
over time, because we did not deploy the changes when we merged them into the
main branch. If we had deployed the changes when they were merged into the main
branch, we would have isolated the DNS change. But given the importance of
crates.io to the Rust ecosystem, we were hesitant to deploy multiple times
without announcing the changes to the community first.

The lessons that we are taking away from this incident are as follows:

  - We need to document the process of deploying changes to production, in
    particular how to pick the Git commit and how to review the changeset.
    Defining a process will enable us to iterate and improve it over time, and
    avoid the same issue in the future.
  - Changes that have been developed and tested in isolation on `staging` should
    be deployed individually and in sequence to `production`. We need to add
    this to the documentation.
  - When we merge changes into the main branch, we need to ensure that they get
    deployed to `production` as well. This avoids a drift between the
    configuration in Git and what is deployed.

[infrastructure team]: https://www.rust-lang.org/governance/teams/infra
