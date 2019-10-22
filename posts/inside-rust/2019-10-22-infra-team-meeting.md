---
layout: post
title: "2019-10-22 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by pietroalbini. Mintues written by pietroalbini.  
Attending: aidanhs, alexcrichton, kennytm, Mark-Simulacrum, pietroalbini,
shepmaster  
[Start of the conversation][discord]

[discord]: https://discordapp.com/channels/442252698964721669/443148319431065610/636247640794857472

## News from the team

- All the AWS resources related to the bastion are now managed with Terraform.
  The documentation on [how to add whitelisted IPs][bastion-whitelist] has
  also been updated.
- Crater agents are now deployed with Ansible, and the related AWS resources
  are now managed with Terraform. Agents are configured to check for updates
  every 5 minutes.
- There was an outage of docs.rs on Sunday night, the postmortem is going to be
  published soon on the "Inside Rust" blog and work on the action items from it
  is ongoing.

[bastion-whitelist]: https://github.com/rust-lang/infra-team/blob/master/docs/hosts/bastion.md#updating-the-whitelisted-ips

## Putting a CDN in front of the CI S3 buckets (pietroalbini)

After an audit of our bandwidth usage for the `rust-lang-ci-sccache2` and
`rust-lang-ci-mirrors` S3 buckets we discovered CI is pulling respectively 8.3
TB and 1.4 TB from them each month. Egress from S3 is pricey (0.09$/GB), while
for us egress from CloudFront is way cheaper (0.04$/month, as across all of our
distributions we transfer out more than 150 TB each month). Putting CloudFront
in front of them is going to save us an estimate of 485$/month.

Thankfully all the data in those buckets is immutable (mirrors are versioned
and all caches are addressed by hash), so there are not going to be any
problems due to cache invalidation. Sccache doesn’t support querying a CDN yet
but aidanhs is going to write some pointers and pietroalbini is going to do the
implementation work. The caches CDN is also going to query a new
`rust-lang-ci-caches` bucket, to avoid the old naming scheme.

pietroalbini already setup
[ci-mirrors.rust-lang.org](https://ci-mirrors.rust-lang.org), and is going to
do the implementation work to both create a CDN distribution for caches and
switch the CI to query the CDN.

## Moving infra documentation to the forge (pietroalbini)

There is an effort in the project to centralize all the internal documentation
[on the forge](https://forge.rust-lang.org). pietroalbini proposed to move all
the infra team documentation to it. Everyone agreed, as we find value in having
everything reachable in a single place. aidanhs pointed out that meeting
minutes should not be migrated on the forge.

## Next meeting time

Europe will switch DST next week, so the next meeting will be at -1 hours for
europeans and at the same time for everyone else on the team.
