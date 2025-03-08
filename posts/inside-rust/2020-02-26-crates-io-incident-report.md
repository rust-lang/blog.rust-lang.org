+++
layout = "post"
title = "crates.io incident report for 2020-02-20"
author = "Pietro Albini"
team = "the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>"
+++

On 2020-02-20 at 21:28 UTC we received a report from a user of crates.io that
their crate was not available on the index even after 10 minutes since the
upload. This was a bug in the crates.io webapp exposed by a GitHub outage.

## Root cause of the outage

In some corner cases the code that uploads new commits to the GitHub repository
of the index was returning a successful status even though the push itself
failed. The bug caused the job scheduler to think the upload was actually
successful, causing the job to be removed from the queue and producing a data
loss.

The outage was caused by that bug, triggered by an unexpected response during
[the GitHub outage][gh-outage] happening at the same time.

[gh-outage]: https://www.githubstatus.com/incidents/bd29l6zgr43g

## Resolution

The team analyzed the code of the background job uploading commits to the
index, and found a possible cause of the misreported success. A team member
[wrote the fix][fix], another one reviewed it and we then deployed the patch
directly to production.

At the same time, once we saw the index started to be updated again, we removed
the broken entries in the database manually and asked the reporter to upload
their crates again.

[fix]: https://github.com/rust-lang/crates.io/pull/2207

## Affected crates

- [`kaze`](https://crates.io/crates/kaze) 0.1.6
- [`wasmer-runtime-core`](https://crates.io/crates/wasmer-runtime-core) 0.14.0
- [`wasmer-win-exception-handler`](https://crates.io/crates/wasmer-win-exception-handler) 0.14.0

## Postmortem

Deploying the change took way longer than expected: there were changes landed
in master but waiting to be deployed on production, increasing the length of
the build process and the risks of the deploy. In the future we should deploy
hotfixes by branching off the current deployed commit, and cherry-picking the
fix on top of that. We should also strive to reduce the amount of time PRs sit
in master without being live.

Nobody was paged due to this incident, as our monitoring and alerting system
wasn’t able to catch the problem: we have monitoring in place for jobs failing
to execute, but in this case the job was mistakenly marked as correct. We
should implement periodic checks that ensure the database and the index are
correctly synchronized.

We were lucky that two members of the team with access to both the support
email and the production environment were online during the outage: without
paging available we could’ve noticed it way later than we did.

During the incident investigation we also found that our logging was not good
enough to properly diagnose the problem: there is no message logged when a
commit is pushed to the index, nor when a background job is executed. Also, the
API call to publish new crates doesn’t include the crate name in its line. We
should enhance our logging capabilities to find the root cause of issues
quickly during future incidents.

## Timeline of events

It took 1 hour and 31 minutes from the start of the incident to the deploy of
the fix.

### 2020-02-20

- **21:17 UTC: the authors of `kaze`, `wasmer-runtime-core`
  and `wasmer-win-exception-handler` published them on crates.io**
- 21:28 UTC: the author of `wasmer-runtime-core` and
  `wasmer-win-exception-handler` reports the issue to help@crates.io
- **21:31 UTC: GitHub updates their status page to report an outage**
- 21:33 UTC: Pietro notices the support mail, pings Sean on Discord, Sean
  starts investigating
- 21:35 UTC: Pietro got back to the author saying that the team was
  investigating
- 21:37 UTC: Sean and Pietro find the symptoms of the incident
- 21:50 UTC: Sean finds a possible cause for the bug
- 22:01 UTC: Sean deletes the affected versions from the database
- 22:09 UTC: Sean opens up [PR 2207][fix] with the fix
- **22:16 UTC: GitHub updates their status page to say the problem is fixed**
- 22:17 UTC: Pietro asks for changes on the PR
- 22:20 UTC: Sean addresses Pietro’s concerns in the PR
- 22:23 UTC: PR merged, Sean deploys it directly to master
- **22:48 UTC: Fix deployed on production**

### 2020-02-21

- 09:27 UTC: the author of `kaze` reports their crate was affected to
  help@crates.io
- 12:55 UTC: Pietro deletes the affected version of `kaze` from the database
  and gets back to the author of the crate
- 14:10 UTC: Pietro analyzes the crates.io database and finds out no other
  crate was affected

## Action items

- [#2226]: Add simple logging when we start the index publishing process.
- [#2227]: Add a periodic job that checks the index and the database for
  consistency, paging the on-call person if there are any mismatches. The job
  will need to account for the crates not yet published on the index but in the
  queue.
- [#2228]: Include the crate name in the HTTP log entry for the publish API
  call.
- [#2229]: Add in-depth logging for swirl background jobs, with information
  such as the job name or the parameters.
- [#2230]: Investigate whether we want to implement a self-healing feature to
  automatically synchronize the index in cases of mismatch.

[#2226]: https://github.com/rust-lang/crates.io/issues/2226
[#2227]: https://github.com/rust-lang/crates.io/issues/2227
[#2228]: https://github.com/rust-lang/crates.io/issues/2228
[#2229]: https://github.com/rust-lang/crates.io/issues/2229
[#2230]: https://github.com/rust-lang/crates.io/issues/2230
