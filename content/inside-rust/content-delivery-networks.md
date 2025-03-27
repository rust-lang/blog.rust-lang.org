+++
layout = "post"
date = 2023-01-24
title = "Diversifying our Content Delivery Networks"
author = "Jan David Nose"
team = "The Rust Infrastructure Team <https://www.rust-lang.org/governance/teams/infra>"
+++

Over the past few weeks, the [Infrastructure Team] has been working on setting
up a second [Content Delivery Network] (CDN) for releases and crates.

## What is changing

Rust releases (`static.rust-lang.org`) and crates (`static.crates.io`) will be
served through both [AWS CloudFront](https://aws.amazon.com/cloudfront/) and
[Fastly](https://www.fastly.com).

This is a transparent change to the infrastructure that doesn't require users or
developers to take any action. We expect no downtime for Rust users as part of
the migration.

This also covers only the CDNs for releases (`static.rust-lang.org`) and
crates.io (`static.crates.io`), no changes are planned for the rest of Rust's
domains.

## Timeline

We are starting the rollout of the new CDN tomorrow, on January 25th, with
`static.crates.io`.

The rollout will happen very gradually over a period of time. We will start to
send a small percentage of traffic through [Fastly](https://www.fastly.com/),
while observing the performance and reliability of the new network. As we gain
confidence, we will slowly increase the amount of traffic.

Once `static.crates.io` has been deployed, we will start working on
`static.rust-lang.org`.

## Report issues

If something _does_ break, please let the [Infrastructure Team] know on
[#t-infra on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/t-infra).

[content delivery network]: https://en.wikipedia.org/wiki/Content_delivery_network
[infrastructure team]: https://www.rust-lang.org/governance/teams/infra
