+++
path = "2025/12/03/crates.io-malicious-crates-evm-units-and-uniswap-utils"
title = "crates.io: Malicious crates evm-units and uniswap-utils"
authors = ["Walter Pearce"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-crates-io"
+++

## Summary

On December 2nd, the crates.io team was notified by Olivia Brown from the [Socket Threat Research Team][socket] of two malicious crates which were downloading a payload that was likely attempting to steal cryptocurrency.

These crates were:

- `evm-units` - 13 versions published in April 2025, downloaded 7257 times
- `uniswap-utils` - 14 versions published in April 2025, downloaded 7441 times, used `evm-units` as a dependency

## Actions taken

The user in question, `ablerust`, was immediately disabled, and the crates in question were deleted from crates.io shortly after. We have retained the malicious crate files for further analysis.

The deletions were performed at 22:01 UTC on December 2nd.

## Analysis

[Socket has published their analysis in a blog post](https://socket.dev/blog/malicious-rust-crate-evm-units-serves-cross-platform-payloads).

These crates had no dependent downstream crates on crates.io.

## Thanks

Our thanks to Olivia Brown from the [Socket Threat Research Team][socket] for reporting the crates. We also want to thank Carol Nichols from the crates.io team and Walter Pearce and Adam Harvey from the [Rust Foundation][foundation] for aiding in the response.

[foundation]: https://foundation.rust-lang.org/
[socket]: https://www.socket.dev/
