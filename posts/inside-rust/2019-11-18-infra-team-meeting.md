---
layout: post
title: "2019-11-12 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by pietroalbini. Minutes written by pietroalbini.
Attending: alexcrichton, kennytm, Mark-Simulacrum, pietroalbini, sgrif, shepmaster
[Start of the conversation](https://discordapp.com/channels/442252698964721669/443148319431065610/643872655312683018)

## Rust 1.14.0 Cargo artifacts on the rust-lang-ci S3 bucket

Following on last meeting’s topics, pietroalbini investigated whether it would
be possible to redirect requests for Cargo 1.14.0 from the `rust-lang-ci` S3
bucket to our CDN. The answer is no, as the S3 support for redirects requires a
different endpoint than the one included in the manifest. The two main courses
of action were to do nothing (keeping the files in the `rust-lang-ci` S3 bucket),
or to rewrite and resign the manifest of Rust 1.14.0 to point to the CDN.

Mark-Simulacrum was in favor of doing nothing, but pietroalbini pointed out
keeping releases in two different places will make mirroring and caching
efforts more complex, as mirror authors will have to special-case a separate
domain just for a release. We then decided to rewrite and resign the manifest.

shepmaster proposed to add a test somewhere to ensure this doesn’t happen
again, but nobody on the team had the time to do this. If someone is interested
please ask in the infra channel.

## Deduplicating CI configuration on GitHub Actions

One issue about GitHub Actions is it doesn’t allow to include shared pieces of
configuration into workflows, forcing us to duplicate (for example) the steps
needed to complete a build. pietroalbini investigated a few ways to work around
the limitation, and the options he presented during the meeting are either
writing our own configuration format and having a tool generate the GitHub
Actions configuration from it, or duplicating the configuration manually and
having a tool to ensure the manually duplicated things don’t diverge.

The rest of the team expressed concerns with generating the configuration, as
using the generator adds even more complexity to our already complex CI. On the
other hand pietroalbini didn’t like the tool to ensure the configuration
doesn’t diverge, as that would make changes to the CI configuration harder for
the people doing it. The agreement in the meeting is that pietroalbini will try
to create other proof of concepts, hoping to find a better solution.

## New server for perf

alexcrichton didn’t hear back from Hetzner about the new perf server yet.
