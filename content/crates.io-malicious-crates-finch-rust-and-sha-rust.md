+++
path = "2025/12/05/crates.io-malicious-crates-finch-rust-and-sha-rust"
title = "crates.io: Malicious crates finch-rust and sha-rust"
authors = ["Carol Nichols and Adam Harvey"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-crates-io"
+++

## Summary

On December 5th, the crates.io team was notified by Kush Pandya from the [Socket Threat Research Team][socket] of two malicious crates which were trying to cause confusion with the existing `finch` crate but adding a dependency on a malicious crate doing data exfiltration.

These crates were:
- `finch-rust` - 1 version published November 25, 2025, downloaded 28 times, used `sha-rust` as a dependency
- `sha-rust` - 8 versions published between November 20 and November 25, 2025, downloaded 153 times

## Actions taken

The user in question, `face-lessssss`, was immediately disabled, and the crates in question were deleted from crates.io shortly after. We have retained the malicious crate files for further analysis.

The deletions were performed at 15:52 UTC on December 5th.

We reported the associated repositories to GitHub and the account has been removed there as well.

## Analysis

[Socket has published their analysis in a blog post](https://socket.dev/blog/malicious-crate-mimicking-finch-exfiltrates-credentials).

These crates had no dependent downstream crates on crates.io, and there is no evidence of either of these crates being downloaded outside of automated mirroring and scanning services.

## Thanks

Our thanks to Kush Pandya from the [Socket Threat Research Team][socket] for reporting the crates. We also want to thank Carol Nichols from the crates.io team and Adam Harvey from the [Rust Foundation][foundation] for aiding in the response.

[foundation]: https://foundation.rust-lang.org/
[socket]: https://www.socket.dev/
