+++
path = "2025/09/24/crates.io-malicious-crates-fasterlog-and-asyncprintln"
title = "crates.io: Malicious crates faster_log and async_println"
authors = ["Walter Pearce"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-crates-io"
+++

**Updated September 24th, 2025 17:34:38 UTC** - Socket has also published their own [accompanying blog post][socket-blog] about the attack.

## Summary

On September 24th, the crates.io team was notified by Kirill Boychenko from the [Socket Threat Research Team][socket] of two malicious crates which were actively searching file contents for Etherum private keys, Solana private keys, and arbitrary byte arrays for exfiltration.

These crates were:
- `faster_log` - Published on May 25th, 2025, downloaded 7181 times
- `async_println` - Published on May 25th, 2025, downloaded 1243 times

The malicious code was executed at runtime, when running or testing a project depending on them. Notably, they did not execute any malicious code at build time. Except for their malicious payload, these crates copied the source code, features, and documentation of legitimate crates, using a similar name to them (a case of typosquatting[^typosquatting]).

## Actions taken

The users in question were immediately disabled, and the crates in question were deleted[^deletion] from crates.io shortly after. We have retained copies of all logs associated with the users and the malicious crate files for further analysis.

The deletion was performed at 15:34 UTC on September 24, 2025.

## Analysis

Both crates were copies of a crate which provided logging functionality, and the logging implementation remained functional in the malicious crates. The original crate had a feature which performed log file packing, which iterated over an associated directories files.

The attacker inserted code to perform the malicious action during a log packing operation, which searched the log files being processed from that directory for:

- Quoted Ethereum private keys (0x + 64 hex)
- Solana-style Base58 secrets
- Bracketed byte arrays

The crates then proceeded to exfiltrate the results of this search to `https://mainnet[.]solana-rpc-pool[.]workers[.]dev/`.

These crates had no dependent downstream crates on crates.io.

The malicious users associated with these crates had no other crates or publishes, and the team is actively investigating associative actions in our retained[^retention] logs.

## Thanks

Our thanks to Kirill Boychenko from the [Socket Threat Research Team][socket] for reporting the crates. We also want to thank Carol Nichols from the crates.io team, Pietro Albini from the Rust Security Response WG and Walter Pearce from the [Rust Foundation][foundation] for aiding in the response.

[^deletion]: The crates were preserved for future analysis should there be other attacks, and to inform scanning efforts in the future.
[^retention]: One year of logs are retained on crates.io, but only 30 days are immediately available on our log platform. We chose not to go further back in our analysis, since IP address based analysis is limited by the use of dynamic IP addresses in the wild, and the relevant IP address being part of an allocation to a residential ISP.
[^typosquatting]: typosquatting is a technique used by bad actors to initiate dependency confusion attacks where a legitimate user might be tricked into using a malicious dependency instead of their intended dependency — for example, a bad actor might try to publish a crate at `proc-macro3` to catch users of the legitimate `proc-macro2` crate.

[foundation]: https://foundation.rust-lang.org/
[init]: https://foundation.rust-lang.org/news/2022-09-13-rust-foundation-establishes-security-team/
[socket]: https://www.socket.dev/
[socket-blog]: https://socket.dev/blog/two-malicious-rust-crates-impersonate-popular-logger-to-steal-wallet-keys