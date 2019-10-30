---
layout: post
title: "2019-10-29 Infrastructure Team Meeting"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

Meeting run by Mark-Simulacrum. Minutes written by pietroalbini.  
Attending: alexcrichton, kennytm, Mark-Simulacrum, pietroalbini, shepmaster  
[Start of the conversation][discord]

[discord]: https://discordapp.com/channels/442252698964721669/443148319431065610/638784152014946321

## News from the team

- A big PR moving most of the CI build environment preparation to standalone
  scripts finally landed! This will allow multiple other CI improvements down
  the line!

- A crates.io staging environment behind CloudFront has been configured on
  [staging.cratesio.com](https://staging.cratesio.com), testing the
  configuration before deploying it on the main domain. Please note the staging
  server is on a temporary domain and will be moved in the near future.

## Rolling back the msys2 hack (P-high issue)

Last week there was an issue in our CI caused by a broken `ca-certificates`
msys2 package, that caused all the windows builders to fail. To patch the
problem an older version of that package was vendored and CI was configured to
download it instead of the broken one. Now that upstream fixed the issue,
pietroalbini will prepare PRs targeting both master and beta rolling back the
temporary patch.

## What to do with mirroring msys2/mingw?

The CI issue mentioned in the previous topic started thoughts about vendoring
msys2 and MinGW as a whole, to prevent such issues from happening in the
future. msys2 is a weird program to package as it’s more of a Linux-like
distribution on Windows, with its own package manager based on tooling borrowed
from Arch Linux.

Creating the initial mirrors is not trivial but doable, but the concern is how
to keep them updated, because for example an outdated OpenSSL is really bad.
mati865 suggested that other projects often tar the whole msys2 installation
directory and periodically update that, and we’ll explore that. More discussion
on the approach will likely happen on the PR implementing the changes.

## Restricted AWS access for docs.rs operators

One of the issues exposed in the [docs.rs outage postmortem][outage] was that
the people in the docs.rs on-call rotation did not have access to the AWS
resources part of docs.rs. pietroalbini proposed to give them limited access,
namely to stop/start the EC2 instance and to inspect/change the underlying S3
storage bucket. Mark-Simulacrum thought the access was not needed in practice,
and we agreed to revisit the topic if issues arise again.

[outage]: https://blog.rust-lang.org/inside-rust/2019/10/24/docsrs-outage-postmortem.html

## New server for perf

To ensure consistent results the collector for [perf.rust-lang.org] needs to be
on bare metal hardware, and until now it was a server lying under
alexcrichton’s desk. The server will have to be moved by December though, and
alexcrichton asked the team on feedback on what to do. There was consensus that
the best solution is to look for a rented bare metal server, and pietroalbini
will do an initial investigation.

[perf.rust-lang.org]: https://perf.rust-lang.org
