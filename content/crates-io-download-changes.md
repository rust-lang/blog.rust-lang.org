+++
path = "2024/03/11/crates-io-download-changes"
title = "crates.io: Download changes"
authors = ["Tobias Bieniek"]
aliases = ["2024/03/11/crates-io-download-changes.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

Like the rest of the Rust community, [crates.io](https://crates.io) has been growing rapidly, with download and package counts increasing 2-3x year-on-year. This growth doesn't come without problems, and we have made some changes to download handling on crates.io to ensure we can keep providing crates for a long time to come.

## The Problem

This growth has brought with it some challenges. The most significant of these is that all download requests currently go through the crates.io API, occasionally causing scaling issues. If the API is down or slow, it affects all download requests too. In fact, the number one cause of waking up our crates.io on-call team is "slow downloads" due to the API having performance issues.

Additionally, this setup is also problematic for users outside of North America, where download requests are slow due to the distance to the crates.io API servers.

## The Solution

To address these issues, over the last year we have decided to make some changes:

**Starting from 2024-03-12, `cargo` will begin to download crates directly from our static.crates.io [CDN](https://en.wikipedia.org/wiki/Content_delivery_network) servers.** 

This change will be facilitated by modifying the [`config.json`](https://github.com/rust-lang/crates.io-index/blob/master/config.json) file on the package index. In other words: no changes to `cargo` or your own system are needed for the changes to take effect. The `config.json` file is used by `cargo` to determine the download URLs for crates, and we will update it to point directly to the CDN servers, instead of the crates.io API.

Over the past few months, we have made several changes to the crates.io backend to enable this:

- We [announced the deprecation of "non-canonical" downloads](https://blog.rust-lang.org/2023/10/27/crates-io-non-canonical-downloads.html), which would be harder to support when downloading directly from the CDN.

- We changed how downloads are counted. Previously, downloads were counted directly on the crates.io API servers. Now, we analyze the log files from the CDN servers to count the download requests.

![crates.io download graph of an arbitrary crate showing that on 2024-02-16, download numbers increased](../../../images/2024-03-11-crates-io-download-changes/download-graph.png)

The latter change has caused the download numbers of most crates to increase, as some download requests were not counted before. Specifically, crates.io mirrors were often downloading directly from the CDN servers already, and those downloads had previously not been counted. For crates with a lot of downloads these changes will be barely noticeable, but for smaller crates, the download numbers have increased quite a bit over the past few weeks since we enabled this change.


## Expected Outcomes

We expect these changes to significantly improve the reliability and speed of downloads, as the performance of the crates.io API servers will no longer affect the download requests. Over the next few weeks, we will monitor the performance of the system to ensure that the changes have the expected effects.

We have noticed that some non-cargo build systems are not using the `config.json` file of the index to build the download URLs. We will reach out to the maintainers of those build systems to ensure that they are aware of the change and to help them update their systems to use the new download URLs. The old download URLs will continue to work, but these systems will be missing out on the potential performance improvement.

We are excited about these changes and believe they will greatly improve the reliability of crates.io. We look forward to hearing your feedback!
