---
layout: post
title: "2019-11-19 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by pietroalbini. Minutes written by pietroalbini.  
Attending: alexcrichton, kennytm, Mark-Simulacrum, pietroalbini, shepmaster  
[Start of the conversation][discord]

[discord]: https://discordapp.com/channels/442252698964721669/443148319431065610/646409370095190016

## Migrating crates.io behind CloudFront

While static.crates.io (hosting the source code of all the published crates) is
behind CloudFront, that’s not true today for the crates.io web application.
This causes problems because crates.io is hosted on Heroku, which requires
pointing a CNAME to Heroku’s DNS. Unfortunately crates.io doesn’t use a
subdomain, which prevents us from using CNAMEs. All our DNS zones are on AWS
Route53, but AWS only supports CNAMEs on the apex pointing to other AWS
resources. Because of that, the crates.io DNS was managed on a different
service until today, causing maintenance issues on our end.

The solution we’re working torwards is to put CloudFront in front of crates.io,
and that will finally allow us to migrate the crates.io domain to AWS.
pietroalbini is finishing the last infra touches, and we expect to deploy the
changes in the coming days.

## DNS management with Terraform

pietroalbini announced to the rest of the team that he started working on
migrating the DNS records of our domains to Terraform. He already migrated the
zones of [cratesio.com] and [crates.io], and he plans to migrate the other ones
over the coming weeks. pietroalbini also wrote [documentation] on this setup.

[cratesio.com]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/services/dns/cratesio.com.tf
[crates.io]: https://github.com/rust-lang/simpleinfra/blob/master/terraform/services/dns/crates.io.tf
[documentation]: https://forge.rust-lang.org/infra/docs/dns.html

## Another GitHub Actions CI configuration prototype

pietroalbini continued his investigation into another prototype for our new
GitHub Actions configuration. To reiterate, GitHub Actions doesn’t support
templates or includes, so the infrastructure team is looking into a way to
reduce duplication between our pipelines.

Since the two prototypes presented at last week’s meeting weren’t liked by all
the team, pietroalbini continued experimenting, and the prototype presented
this week was based around YAML anchors, a standard YAML feature to reuse parts
of data inside a single file. Unfortunately GitHub Actions explicitly disables
YAML anchors, so pietroalbini wrote a small tool that pre-generates the
expanded configuration file and commits it into the repo.

The team liked this approach much more, as it doesn’t introduce any new
configuration syntax while keeping the configuration files in a manageable
state. pietroalbini is going to polish the prototype and open a PR for it in
the coming days.
