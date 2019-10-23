---
layout: post
title: "docs.rs outage postmortem"
author: Pietro Albini
team: the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>
---

At 2019-10-21 01:38 UTC the docs.rs website went down because no available disk
space was left on the server hosting the application. Crate builds were failing
since 2019-10-20 00:55 UTC due to the same reason.

## Root cause of the outage

docs.rs needs to store the built documentation on the filesystem before
uploading it to the database, and it does so in the
`/opt/docs-rs-prefix/documentations` directory. docs.rs never cleared that
directory though, so over time it started to increase its size until it caused
this outage. Code to periodically purge temporary directories was present, but
it was never configured to purge the one which caused the outage.

## Resolution

As the directory doesn’t contain any persistent data we cleared it and the web
server was restarted. Once we were confident the situation was resolved all the
crates that failed due to the outage were queued for a rebuild.

## Postmortem

The increased disk usage was gradual over weeks, slowly reaching 100% and
causing the outage. While monitoring systems were in place and recorded graphs
of the increase, no alert was configured so nobody noticed the problem. We need
to add alerts when disk usage reaches 90%, so the problem can be investigated
and dealt with on time.

Crates started to fail to build a day earlier, and close to no builds were
successfully completed since then. We need to setup alerts when most of the
builds are failing: as we don’t have the necessary metrics at the moment to
reliably alert we'll have to add extra instrumentation as well.

Our response was slower due to issues with our on-call rotation for the
service. The primary contacts don’t have the level of access required to
increase the disk space of the instance (the temporary fix that was
investigated at first but discarded after the discovery nobody awake could do
it), and the backup contacts don’t have any production access or expertise on
docs.rs.

## Timeline of events

Unless otherwise noted all events happened on 2019-10-21, and all times are in
UTC.

- **2019-10-20 00:55: crate builds started failing due to the low disk space**
- **01:38: alerts fired for the docs.rs website being down, [ashleygwilliams]
  (backup contact) got paged**
- 01:39: [QuietMisdreavus] joins into the operations channel
- 01:39: [QuietMisdreavus] found the reason for the outage (full root partition)
- 01:52: [ashleygwilliams] proposed to increase disk space, nobody with
  permissions required to so was awake or available though
- 01:56: [ashleygwilliams] contacts [Mark-Simulacrum], who has the access
  required to increase disk space
- 01:57: [QuietMisdreavus] found the directory taking up all the disk space
- 02:00: [QuietMisdreavus] removed the directory taking up all the disk space
- 02:03: [QuietMisdreavus] restarted the web server
- **02:06: CDN propagated the changes, docs.rs back online**
- 02:06: [Mark-Simulacrum] joins into the operations channel
- 08:19: [pietroalbini] added builds failed during the outage back into the
  queue
- **19:27: builds of the crates failed during the outage finished**

[ashleygwilliams]: https://github.com/ashleygwilliams
[QuietMisdreavus]: https://github.com/QuietMisdreavus
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[pietroalbini]: https://github.com/pietroalbini

## Action items

* Update the docs.rs source code to cleanup the offending directory
  automatically.
* Add alerts when the available disk space on a server is below 10%.
* Add alerts when most of the builds are failing.
* Revisit the on-call rotation to make sure everyone on it has the
  permissions to either react to the incidents or escalate.
