+++
path = "inside-rust/2023/07/21/crates-io-postmortem"
title = "crates.io Postmortem: Broken Crate Downloads"
authors = ["Tobias Bieniek"]
aliases = ["inside-rust/2023/07/21/crates-io-postmortem.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

(based on https://www.atlassian.com/incident-management/postmortem/templates)

## Summary

On 2023-07-20 between 12:17 and 12:30 UTC all crate downloads from crates.io were broken due to a deployment that contained a bug in the download URL generation. 

During this time we had an average of 4.71K requests per second to crates.io, resulting in about 3.7M failed requests, including the retry attempts from cargo.

The incident was noticed by the developer triggering the production deployment after seeing elevated request-per-second numbers in our monitoring dashboard after the deployment. At this point the root cause for the elevated numbers was not clear yet, but a community member notified the developer via [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376960060).

Immediately after the notification, the broken deployment was rolled back to the previous deployment, fixing the downloads again.

## Leadup

At 17:41 UTC on 2023-07-19 a [pull request](https://github.com/rust-lang/crates.io/pull/6834) to crates.io was merged, finishing the migration of the crates.io codebase to use the [object_store](https://crates.io/crates/object_store) crate for AWS S3 access, instead of our previous custom solution.

This pull request refactored the way the crate and readme download endpoints generated redirect URLs.

## Fault

The pull request introduced a few tests for the previously untested functionality, though unfortunately it was using values different from the environment variable content that is used by crates.io in production. This led to the production code path not being tested properly.

The production code path contained a bug where the URL generated from the "CDN prefix" and "path" components was missing a slash (`/`) separator. 

This led to <https://crates.io/api/v1/crates/smallvec/1.10.0/download> redirecting to <https://static.crates.iocrates/smallvec/smallvec-1.10.0.crate> instead of <https://static.crates.io/crates/smallvec/smallvec-1.10.0.crate>.

## Impact

For about 13 minutes, between 12:17 and 12:30 UTC on 2023-07-20, our users experienced this incident.

This incident affected all users trying to download crate files from crates.io during that time.

The issue manifest in our users seeing errors like this when running `cargo`:

```text
warning: spurious network error (3 tries remaining): [6] Couldn't resolve host name (Could not resolve host: static.crates.iocrates)
warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: static.crates.iocrates)
warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: static.crates.iocrates)
error: failed to download from `https://crates.io/api/v1/crates/serde_derive/1.0.173/download`
```

<https://github.com/rust-lang/crates.io/issues/6850> was submitted and upvoted 12 times.

## Detection

The developer triggering the production deployment was monitoring the crates.io Grafana dashboard during the deployment and noticed elevated levels of request-per-second numbers for the download endpoint. This was a symptom of cargo retrying the download multiple times before giving up.

About 11 minutes after the deployment, a community member notified the crates.io team via [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376960060) about the [GitHub issue](https://github.com/rust-lang/crates.io/issues/6850) that was opened, describing the failing downloads.

## Response

After the incident was detected, the deploying developer immediately initiated a rollback to the previous deployment through the Heroku user interface. This process took about one minute due to the login procedure and ensuring that the right buttons in the user interface are used.

## Recovery

After rolling back to the previous deployment the system immediately recovered itself and produced correct redirect URLs again.

A fix for the broken pull request was subsequently developed and merged, including more tests for the broken code path with more real-world values. The fix was then tested on the staging environment before it got deployed to production too.

## Timeline

### 2023-07-19

- 12:32 UTC – <https://github.com/rust-lang/crates.io/pull/6834> (Migrate remaining `Uploaders` code into `Storage` system) was opened
- 17:41 UTC – <https://github.com/rust-lang/crates.io/pull/6834> (Migrate remaining `Uploaders` code into `Storage` system) was merged, automatically deploying to the staging environment

### 2023-07-20

- 10:00 UTC – <https://github.com/rust-lang/crates.io/pull/6848> (Fix `readme` field parsing of `Cargo.toml` files) was opened
- 10:13 UTC – <https://github.com/rust-lang/crates.io/pull/6848> (Fix `readme` field parsing of `Cargo.toml` files) was merged, automatically deploying to the staging environment
- 12:08 UTC – <https://staging.crates.io/crates/crates-staging-test-tb/0.69.30> was published to the staging environment to smoke test the publish process and the `Cargo.toml` parsing fix
- 12:16 UTC – A [message](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376956537) was sent to the `deployments` topic of the `t-crates-io` Zulip stream, notifying users of the upcoming deployment.
- 12:17 UTC – The staging deployment was promoted to the production environment
- 12:18 UTC – Another [message](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376957223) was sent to the Zulip stream, notifying users that the deployment was completed.
- 12:24 UTC – <https://github.com/rust-lang/crates.io/issues/6850> (Crates.io crate download API is redirecting to invalid URL) was opened
- 12:25 UTC – The continuing request-per-second anomaly was deemed unusual enough to trigger another [message](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376959143) to the Zulip stream.
- 12:28 UTC – A community member notified the crates.io team on the [Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/deployments/near/376960060) about the GitHub issue and the deploying developer acknowledged the incident.
- 12:30 UTC – The deployment was rolled back, temporarily fixing the issue for our users.
- 13:38 UTC – <https://github.com/rust-lang/crates.io/pull/6851> (Fix download URL generation) was opened
- 13:49 UTC – <https://github.com/rust-lang/crates.io/pull/6851> (Fix download URL generation) was merged, automatically deploying to the staging environment
- 14:08 UTC – <https://staging.crates.io/crates/crates-staging-test-tb/0.69.31> was published to the staging environment to smoke test the publish process and check that the download URL generation fix was working correctly
- 14:10 UTC – The staging deployment was promoted to the production environment

## Root cause identification: The Five Whys

- The redirect URLs for crate and readme downloads were broken in production.

  **Why were the redirect URLs broken?**

  - There was a bug introduced in pull request [#6834](https://github.com/rust-lang/crates.io/pull/6834) which made it all the way into our production environment.

    **Why was there a bug introduced in this pull request?**

    - The pull request introduced tests, but did not test all code paths.

      **Why did the pull request not test all code paths?**

      - The code was structured in a way that made testing with different "CDN prefix" values complicated.

        **Why was the code structured in a way that made testing different values complicated?**

        - The code had not been unit tested before and the refactoring stopped at a point where the code could at least be tested with a hardcoded value.

          **Why did the refactoring stop at that point?**

          - It was deemed "good enough for now" by the developer.

    - The pull request was not reviewed by another developer.

      **Why was the pull request not reviewed by another developer?**

      - The developer creating the pull request misjudged the potential impact of a bug in the pull request. They did not explicitly request a review from the crates.io team and merged it themselves after a few hours.

        **Why was no code review requested from the crates.io team?**

        - The number of active team members in the [crates.io team](https://www.rust-lang.org/governance/teams/crates-io) is quite small. Reviewing dozens of pull requests per months from the one developer who is employed to work fulltime on crates.io would be a recipe for burnout for the other members of the crates.io team. For that employed fulltime developer it would also not work well if they were blocked on waiting for reviews for the majority of their time. The current way of working is that code reviews are only requested for high-impact pull requests. 

        **Why was the potential impact misjudged?**

        - The developer forgot to think about the fact that this change affected the crate download endpoint of crates.io, which is the endpoint that handles 99% of the traffic to the server.

          **Why did the developer forget to check if a high-priority endpoint is affected?**

          - There is no checklist or guide describing in which case a pull request should be seen as having a high potential impact and thus needing explicit code review from the crates.io team.

    **Why did the bug make it into production?**

    - The crate download endpoint was not tested on the staging environment before promoting it to production.

      **Why was the crate download not tested?**

      - The test plan for the staging environment only includes publishing a new version and seeing that reflected on the website and in the package index repository.

        **Why does the test plan not include crate downloads?**

        - Since 99% of all requests to crates.io are for crate downloads, the test plan definitely should include this process. There is intentionally no download button on the webpage though, so the URL for the download has to be constructed manually.
  
          **Why does the download URL need to be constructed manually?**
  
          - Because the smoke test procedure on our staging environment is currently a completely manual process without any automation.

## Root causes

- The failing code was structured in a way that made it hard to test different variants and code paths.
- There is no checklist describing which pull requests should be seen as high-impact.
- The smoke test procedure on the staging environment does not include crate downloads and is a manual process.

## Backlog check

There are no specific items in the backlog that could have prevented this incident.

## Recurrence

A previous incident caused crate publishing to not work anymore. The learning from this incident was to ensure that the smoke testing procedure includes the publishing process. Unfortunately, this did not include the crate file download though. 

## Lessons learned

- The detection time from deployment to incident notification could have been faster if the symptom was identified earlier to be caused by the cargo retry behavior. The heightened awareness of the deploying developer due to the change in Grafana numbers however contributed to this issue being fixed faster.
- The response time from incident notification to rollback and fixing the issue was fast.
- All code should be structured in a way that makes testing the different code paths easy.
- We need clearer rules on which pull requests require code reviews.
- The smoke test procedure should include crate downloads.
- The smoke test procedure should be automated as much as possible.

## Corrective actions

- **HIGH** Include crate downloads in the smoke test plan for the staging environment
- **MEDIUM** Automate the staging environment smoke tests
- **MEDIUM** Develop rules on which pull requests require explicit code review
