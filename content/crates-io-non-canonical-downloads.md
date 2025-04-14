+++
path = "2023/10/27/crates-io-non-canonical-downloads"
title = "crates.io: Dropping support for non-canonical downloads"
authors = ["Tobias Bieniek"]
aliases = ["2023/10/27/crates-io-non-canonical-downloads.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

## TL;DR

- We want to improve the reliability and performance of crate downloads.
- "Non-canonical downloads" (that use URLs containing hyphens or underscores where the crate published uses the opposite) are blocking these plans.
- On 2023-11-20 support for "non-canonical downloads" will be disabled.
- cargo users are unaffected.

## What are "non-canonical downloads"?

The "non-canonical downloads" feature allows everyone to download the `serde_derive` crate from <https://crates.io/api/v1/crates/serde_derive/1.0.189/download>, but also from <https://crates.io/api/v1/crates/SERDE-derive/1.0.189/download>, where the underscore was replaced with a hyphen (crates.io normalizes underscores and hyphens to be the same for uniqueness purposes, so it isn't possible to publish a crate named `serde-derive` because `serde_derive` exists) and parts of the crate name are using uppercase characters. The same also works vice versa, if the canonical crate name uses hyphens and the download URL uses underscores instead. It even works with any other combination for crates that have multiple such characters (please don't mix them…!).

## Why remove it?

Supporting such non-canonical download requests means that the crates.io server needs to perform a database lookup for every download request to figure out the canonical crate name. The canonical crate name is then used to construct a download URL and the client is HTTP-redirected to that URL.

While we have introduced a caching layer some time ago to address some of the performance concerns, having all download requests go through our backend servers has still started to become problematic and at the current rate of growth will not become any easier in the future.

Having to support "non-canonical downloads" however prevents us from using CDNs directly for the download requests, so if we can remove support for non-canonical download requests, it will unlock significant performance and reliability gains.

## Who is using "non-canonical downloads"?

`cargo` always uses the canonical crate name from the package index to construct the corresponding download URLs. If support was removed for this on the crates.io side then cargo would still work exactly the same as before.

Looking at the crates.io request logs, the following user-agents are currently relying on "non-canonical downloads" support:

- cargo-binstall/1.1.2
- Faraday v0.17.6
- Go-http-client/2.0
- GNU Guile
- python-requests/2.31.0

Three of these are just generic HTTP client libraries. [GNU Guile](https://www.gnu.org/software/guile/) is apparently a programming language, so most likely this is also a generic user-agent from a custom user program.

[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) is a tool enabling installation of binary artifacts of crates. The maintainer is already aware of the upcoming change and confirmed that more recent versions of `cargo-binstall` should not be affected by this change.

We recommend that any scripts relying on non-canonical downloads be adjusted to use the canonical names from the package index, the database dump, or the crates.io API instead. If you don't know which data source is best suited for you, we welcome you to take a look at [the crates.io data access page](https://crates.io/data-access).

## What is the plan?

1. **Today:** Announce the removal of support for non-canonical downloads on the main Rust blog.
2. **2023-11-20:** Disable support for non-canonical downloads and return a migration error message instead, to alert remaining users of this feature of the need to migrate. This still needs to put load on the application to detect a request is using a non-canonical download URL.
3. **2023-12-18:** Return a regular 404 error instead of the migration error message, allowing us to get rid of (parts of) the database query.

Note that we will still need the database query for download counting purposes for now. We have plans to remove this requirement as well, but those efforts are blocked by us still supporting non-canonical downloads.

If you want to follow the progress on implementing these changes or if you have comments you can subscribe to the corresponding [tracking issue](https://github.com/rust-lang/crates.io/issues/7341). Related discussions are also happening on the [crates.io Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/non-canonical.20downloads).
