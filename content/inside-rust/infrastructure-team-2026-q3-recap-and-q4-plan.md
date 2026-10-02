+++
path = "inside-rust/2026/10/12/infrastructure-team-q3-recap-and-q4-plan"
title = "Infrastructure Team 2026 Q3 Recap and Q4 Plan"
authors = ["Marco Ieni"]

[extra]
team = "The Rust Infrastructure Team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-infra"
+++

Here's what the Infrastructure Team delivered in Q3 2026 and what we're focusing
on in Q4.

You can find the previous blog post of this series [here](@/inside-rust/infrastructure-team-2026-q2-recap-and-q3-plan/index.md).

## Q3 2026 Accomplishments

### Audit logs in Datadog Cloud SIEM

We enabled audit logs for the following services and sent them to Datadog, so that the
Rust Foundation security team can analyze them using
[Cloud SIEM](https://www.datadoghq.com/product/cloud-siem/):

- The Rust GitHub Enterprise and the Rust Foundation GitHub Enterprise,
  via [audit log streaming](https://docs.github.com/en/enterprise-cloud@latest/admin/monitoring-activity-in-your-enterprise/reviewing-audit-logs-for-your-enterprise/streaming-the-audit-log-for-your-enterprise#setting-up-streaming-to-datadog).
- AWS, via [CloudTrail](https://aws.amazon.com/cloudtrail/).
- Heroku, Wiz, Fastly, and Datadog itself.

See the [GitHub issue](https://github.com/rust-lang/infra-team/issues/290) for more details.

### Web Application Firewall and DDoS protection for crates.io and docs.rs

Together with the crates.io and docs.rs teams, we set up the Fastly [Next-Gen WAF](https://docs.fastly.com/products/fastly-next-gen-waf)
(Web Application Firewall) for [docs.rs](https://docs.rs) and [crates.io](https://crates.io)
(see the [GitHub issue](https://github.com/rust-lang/simpleinfra/issues/1007)).
The WAF blocks potential security threats and enforces rate limits.

We also enabled Fastly [DDoS protection](https://www.fastly.com/products/ddos-protection) for both services.

Together, these helped us mitigate DDoS attacks against crates.io this quarter,
and reduce the load on the docs.rs server.

### CDN improvements for crates.io and docs.rs

We made several improvements to the Fastly configuration of crates.io and docs.rs:

- **Faster crate downloads processing:** We [removed a regex](https://github.com/rust-lang/simpleinfra/pull/1097)
  from the Fastly code that handles crate downloads, and did some other optimizations
  ([#1095](https://github.com/rust-lang/simpleinfra/pull/1095), [#1096](https://github.com/rust-lang/simpleinfra/pull/1096)).
  Requests now take 10% less compute time in the Fastly CDN nodes. Most users won't notice the difference,
  since networking dominates download latency, but it makes our CDN setup more efficient and sustainable.
- **`Content-Length` in `HEAD` responses:** Fastly `HEAD` responses [now include](https://github.com/rust-lang/simpleinfra/pull/1130)
  the `Content-Length` header for non-compressible artifacts such as `.crate` files.
  This fixes some issues encountered when using [Buck2](https://buck2.build/).
- **crates.io static assets served from the edge:** `GET` and `HEAD` requests for READMEs and other static
  assets of crates.io [are now resolved](https://github.com/rust-lang/simpleinfra/pull/1235)
  by the CDN instead of the crates.io web app hosted on Heroku.
- **docs.rs CDN logs in Datadog:** The docs.rs Fastly service now sends its logs to Datadog
  ([#1196](https://github.com/rust-lang/simpleinfra/pull/1196),
  [#1197](https://github.com/rust-lang/simpleinfra/pull/1197),
  [#1226](https://github.com/rust-lang/simpleinfra/pull/1226)), giving us better observability of the service.
- **CloudFront traffic:** In the past months, we sent 100% of crates.io traffic over Fastly
  to spend our AWS credits on other resources. We [restored](https://github.com/rust-lang/simpleinfra/pull/1219)
  a minimal amount of traffic (0.4%) over CloudFront to make sure it keeps working as intended,
  in case we need to fall back to it.

### New message queue between crates.io and docs.rs

We [provisioned](https://github.com/rust-lang/simpleinfra/issues/1075) an
[AWS SQS](https://aws.amazon.com/sqs/) FIFO queue that crates.io can use to notify docs.rs about new releases.

The queue is meant to replace the crates.io git index for this use case.
It's currently unused for lack of engineering time from the crates.io team.

### Renovate for the `rust` repository

We [configured](https://github.com/rust-lang/infra-team/issues/293) [Renovate](https://docs.renovatebot.com/)
for the [`rust-lang/rust`](https://github.com/rust-lang/rust) repository
(see the [configuration file](https://github.com/rust-lang/rust/blob/main/.github/renovate.json5)).
Renovate automatically opens PRs to fix security findings and update Cargo lockfiles and GitHub Actions.

From the [dependency dashboard](https://github.com/rust-lang/rust/issues/134129), maintainers can
ask Renovate to update individual dependencies when they want to.
You can see all the PRs opened by Renovate [here](https://github.com/rust-lang/rust/pulls?q=is%3Apr+author%3Arenovate-bot+is%3Aclosed).

### GitHub security alerts

We enabled [Dependabot alerts](https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts)
in the `rust-lang` GitHub organization. This means that:

- Maintainers receive alerts for vulnerabilities affecting their repositories.
- Maintainers can see these vulnerabilities at `https://github.com/rust-lang/<repo>/security/dependabot`.
- If configured, Renovate can open PRs to fix these vulnerabilities.
  See the [Renovate docs](https://docs.renovatebot.com/configuration-options/#vulnerabilityalerts) to learn more.

Note that this setting doesn't enable automatic Dependabot PRs. That's a separate setting called
"Dependabot security updates", which we don't plan to enable because we prefer using Renovate.

### More settings configured in the `team` repo

We keep moving manually configured settings into Infrastructure as Code (IaC) in the
[`team`](https://github.com/rust-lang/team) repository:

- **GitHub Pages:** Rust Project members can now set up GitHub Pages for their repositories
  by opening a PR. See the [docs](https://github.com/rust-lang/team/blob/main/docs/toml-schema.md#github-pages)
  and an [example](https://github.com/rust-lang/team/blob/9a1b2b6b0aa87d0ad1a7e712de5dea1cd4b3ef10/repos/rust-lang/rust-forge.toml#L7).
  Thanks to [cuba0001](https://github.com/cuba0001) for [working on this](https://github.com/rust-lang/team/issues/2518)!
- **Custom Properties:** GitHub [Custom Properties](https://docs.github.com/en/organizations/managing-organization-settings/managing-custom-properties-for-repositories-in-your-organization)
  are [now configurable](https://github.com/rust-lang/team/pull/2512) in the `team` repo.
  This allows repositories to opt in to automations developed by the Infrastructure Team.

### Hardware security keys

As a follow-up to the hardware security keys work we did in Q2, we [added a convention](https://github.com/rust-lang/team/pull/2622)
in the `team` repository for Project members who want to share the PIV attestation certificates
extracted from their YubiKeys. We can validate these certificates against our inventory of hardware keys
through their serial numbers.

More details in the [docs](https://forge.rust-lang.org/infra/docs/hardware-security-keys.html).

### GitHub Actions audits

We deployed the [crabwatch](https://github.com/rust-lang/crabwatch) workflow that audits all
workflows in the `rust-lang` GitHub organization.
We can [override](https://github.com/rust-lang/crabwatch/blob/4630d36d74262fbcbed9c42861737343db167eed/zizmor-policy.yml#L100)
what audits run for every repository, overriding the [default](https://github.com/rust-lang/crabwatch/blob/4630d36d74262fbcbed9c42861737343db167eed/zizmor-policy.yml#L7) settings.
We are now enforcing 23 [Zizmor](https://zizmor.sh/) audits across the entire organization, and
we want to enable more in the future!

Thanks to [Sandra](https://github.com/Sandijigs), the Outreachy intern who helped us work on this!
You can read about her internship experience in her [blog](https://sandraidjighere.wordpress.com/2026/07/10/building-crabwatch-a-month-into-my-outreachy-internship/).

### More powerful dev desktops and Rust Playground

We upgraded the EC2 instances of the dev desktops and the [Rust Playground](https://play.rust-lang.org/)
to make these services faster for Rust contributors and the Rust community:

- Playground: from `c5a.large` to `c7a.xlarge` ([PR](https://github.com/rust-lang/simpleinfra/pull/1220)).
- Dev desktops ([PR](https://github.com/rust-lang/simpleinfra/pull/1218)):
  - `dev-desktop-eu-1`: from `c6g.8xlarge` to `c9g.8xlarge`.
  - `dev-desktop-us-1`: from `c7g.12xlarge` to `c9g.12xlarge`.

We also increased the disk quota to 200GB per user on all four dev desktops
([#1182](https://github.com/rust-lang/simpleinfra/pull/1182), [#1187](https://github.com/rust-lang/simpleinfra/pull/1187)).

Learn more about dev desktops in the [Forge docs](https://forge.rust-lang.org/infra/docs/dev-desktop.html).

### ARM machine for rustc-perf

We [provisioned](https://github.com/rust-lang/simpleinfra/issues/1198) an EC2 `m9g.metal-48xl` instance
to run [rustc-perf](https://github.com/rust-lang/rustc-perf) on ARM.
This allows the compiler team to measure optimizations done for the ARM architecture.

### New CI runners

In partnership with Canonical, we enabled new
[External GitHub Actions runners](https://forge.rust-lang.org/infra/docs/external-ci-runners.html)
for the Rust Project:

- `s390x-unknown-linux-gnu`: currently being evaluated in the [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins) repository.
- `armv7-unknown-linux-gnueabihf`: used in the [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins) repository.

We also [enabled](https://github.com/rust-lang/rust-forge/pull/1063) three large ARM GitHub Actions runners
(8 vCPU, 32 GB RAM) for the [rustls](https://github.com/rustls) organization, covering Ubuntu 24.04,
Ubuntu 26.04, and Windows 11. This additional CI capacity is sponsored by ARM.

### `riscv64-unknown-linux-musl` promoted to tier 2 with host tools

We [promoted](https://github.com/rust-lang/rust/pull/158766) the `riscv64-unknown-linux-musl` target
to [tier 2 with host tools](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-with-host-tools).
RISC-V users targeting musl no longer need to compile Rust tools like `rustc` and `cargo` themselves.

### On-call rotation moved to Datadog

Since the Rust Foundation [joined Datadog's Open Source Program](https://rustfoundation.org/media/rust-foundation-joins-datadogs-open-source-program/),
we migrated the Rust Foundation on-call rotation from PagerDuty to [Datadog On-Call](https://docs.datadoghq.com/incident_response/on-call/).
This allows us to save money and to remove one service from our dependencies.

### Team leader rotation

We established a new [policy](https://github.com/rust-lang/infra-team/blob/395cd345bcd37ab02829a1a66d05be2159a25df0/about/team-leads.md)
for the Infrastructure Team leader.

As a result, we [rotated](https://github.com/rust-lang/infra-team/pull/301) one team leader:
[Marco](https://github.com/marcoieni) took the place of [JD](https://github.com/jdno).

### Main branch

We renamed the branch of 19 repositories to `main`:

- [lang-team](https://github.com/rust-lang/team/pull/2517)
- [rustc-hash](https://github.com/rust-lang/team/pull/2554)
- [rustc-perf](https://github.com/rust-lang/team/pull/2565)
- [rust-artwork](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608681816)
- [.github](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [compiler-team-prioritization](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [project-ffi-unwind](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [project-safe-transmute](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [wg-allocators](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [wg-cargo-std-aware](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Renaming.20default.20branches.20from.20master.20to.20main/near/608706242)
- [futures-rs](https://github.com/rust-lang/team/pull/2593)
- [pin-utils](https://github.com/rust-lang/team/pull/2593)
- [thanks](https://github.com/rust-lang/team/pull/2451)
- [rust-forge](https://github.com/rust-lang/team/pull/2697)
- [triagebot](https://github.com/rust-lang/team/pull/2696)
- [mdBook](https://github.com/rust-lang/team/pull/2720)
- [portable-simd](https://github.com/rust-lang/team/pull/2759)
- [rustup-components-history](https://github.com/rust-lang/team/pull/2758)
- [std-dev-guide](https://github.com/rust-lang/team/pull/2768)

Now `main` is used by 120 repositories.
There are still 73 repositories using `master`.

### Trusted publishing

We converted 6 crates to use [trusted publishing](https://crates.io/docs/trusted-publishing):

- [rustc-hash](https://github.com/rust-lang/team/pull/2555)
- [jobserver](https://github.com/rust-lang/team/pull/2595)
- [libc](https://github.com/rust-lang/team/pull/2619)
- [ctest](https://github.com/rust-lang/team/pull/2619)
- [docs_rs_crates_io](https://github.com/rust-lang/team/pull/2686)
- [font-awesome-as-a-crate](https://github.com/rust-lang/team/pull/2686)

## Q4 2026 Plans

### Finish Q3 goals

In Q3, we didn't manage to finish all our goals, so we will continue working on them in Q4:

* Improve access controls for Rust infrastructure with SAML SSO.
* Move mailing lists from Mailgun to Google Groups to reduce spam
* Consolidate logs on Datadog
* Stricter networking access rules for crater agents

You can read more about these goals in our [Q3 2026 plan](@/inside-rust/infrastructure-team-2026-q2-recap-and-q3-plan/index.md#q3-2026-plans).

### Secure deployments

Today, we deploy infrastructure from our personal laptops. Instead, we want to deploy from a secure
environment with strong access controls, where access is limited to the people who maintain the
deployment processes. Deploying from a centralized environment also makes it easier to automate
deployments and to monitor for unauthorized access or suspicious activity.

As a first step, we want to simplify our Infrastructure as Code setup, which currently uses both
Terraform and Terragrunt. We want to investigate whether it is still worth using both tools
(see the [GitHub issue](https://github.com/rust-lang/simpleinfra/issues/962)).

After simplifying the setup, we will decide how to automate deployments and start automating them.

### Decommission the monitoring instance

As part of our move to Datadog, we want to decommission our self-hosted monitoring
instance, which runs Prometheus, Alertmanager and Grafana.

## Join us

If you're interested in contributing to Rust's infrastructure, have a look at the
[infra-team](https://github.com/rust-lang/infra-team) repository to learn more about us
and reach out on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra).

We are always looking for new contributors!
