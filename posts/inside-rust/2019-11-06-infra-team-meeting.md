---
layout: post
title: "2019-11-05 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by shepmaster. Minutes written by pietroalbini.  
Attending: aidanhs, alexcrichton, kennytm, Mark-Simulacrum, pietroalbini, shepmaster  
[Start of the conversation](https://discordapp.com/channels/442252698964721669/443148319431065610/641335927721033732)

## Removing MSYS2 ca-certificates workaround (P-high issue)

A few weeks ago our CI broke due to a broken `ca-certificates` MSYS2 package,
which caused all Windows builders to fail. The temporary patch was to install a
vendored copy of that package, and since it’s not needed anymore pietroalbini
landed a PR this week removing the hack from our CI configuration.

The other part of the issue is figuring out whether to vendor MSYS2 and MinGW
as a whole, but there is the issue of keeping the mirrors sort of up to date,
and we don’t have a clear idea on how to fix that at the moment. We decided to
downgrade the issue to P-medium and to talk about mirroring and vendoring at
the All Hands 2020. In preparation of that meeting it will be useful to audit
what we mirror at the moment and how old that is, but it’s not urgent and
nobody has the time to work on it right now.

## Figuring out data retention on Azure Pipelines (P-medium issue)

This is not yet an issue, as the current retention is configured to 2 years.
We’re waiting on some talks with Microsoft to settle before starting to poke
people about this again.

## Re-enable LLVM/debug assertions on slow builders (P-medium issue)

We still don’t have the time budget to enable them back, but increasing the
core count should allow us to do that.

## New server for perf

alexcrichton ordered an
[AX41-NVMe](https://www.hetzner.com/dedicated-rootserver/ax41-nvme) bare metal
server from Hetzner as a replacement benchmarking machine for perf, paid by
Mozilla. We’re waiting on Hetzner to give us access to it before simulacrum can
try it out and configure it. If we don’t get access in a few days alexcrichton
is going to ping them.

## Lifecycle policy for static.rust-lang.org

static.rust-lang.org is backed by a S3 bucket, and since 2016 versioning is
enabled on the bucket to prevent issues with accidental file deletions. Some of
the files in that bucket are overridden each day though (for example nightly
compilers), keeping a bunch of past versions stored. Those past versions are
useless as there isn’t an easy way to get them from the CDN, and the files are
also stored separately in other places on that bucket. Everyone agreed we
should enable lifecycle policy to delete those useless files, and we reached a
consensus on deleting them after three months. This won’t be noticeable by end
users, installing old nightlies by date will still work.

## What to do with the rust-lang-ci S3 bucket

`rust-lang-ci` is a really old and currently unused S3 bucket which was used to
store CI artifacts before we migrated them to `rust-lang-ci2`. There are still
some files in there, so we enabled bucket logging for a month to see how
frequent files there were accessed.

Turns out, we had a total of 86 successful requests in a month, split between:

- 69 requests were accessing cargo builds of 1.14.0
- 17 requests were accessing old CI mirrors

Due to the low traffic we decided to remove those old CI mirrors, but for the
cargo builds the question is more complicated. Due to a bug in the manifest
generation back then, installing Rust 1.14.0 from rustup actually downloads
Cargo from the bucket instead of the CDN, and removing those files would
permanently break installing Rust 1.14.0. There was disagreement on whether to
do that inside the team, and we reached the decision to wait until pietroalbini
investigates whether redirects are feasible to configure in S3.

## Early feedback on pietroalbini’s WIP ci-generate branch

pietroalbini is working on a branch that implements a generator for our CI
configuration, from a simplified custom DSL
([branch](https://github.com/pietroalbini/rust/tree/ci-generate) -
[documentation](https://github.com/pietroalbini/rust/tree/ci-generate/src/tools/generate-ci-config)).
While the generator has a number of small benefits for us, the main driver
between the creation of the branch is to work around some limitations in GitHub
Actions’s configuration syntax, namely the lack of importable templates. There
wasn’t time to properly discuss this so we deferred to next week.

## Revisiting meeting run rotation

A month ago we started rotating who runs the meetings, intending to revisit the
decision today. The team felt either neutral or positive about the idea, so
we’ll keep doing that for a while. aidanhs is going to run the next meeting.
