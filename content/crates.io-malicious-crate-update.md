+++
path = "2026/02/13/crates.io-malicious-crate-update"
title = "crates.io: an update to the malicious crate notification policy"
authors = ["Adam Harvey"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-crates-io"
+++

The crates.io team will no longer publish a blog post each time a malicious crate is detected or reported. In the vast majority of cases to date, these notifications have involved crates that have no evidence of real world usage, and we feel that publishing these blog posts is generating noise, rather than signal.

We will always publish a [RustSec][rustsec] advisory when a crate is removed for containing malware. You can subscribe to the [RustSec advisory RSS feed][rss] to receive updates.

Crates that contain malware _and_ are seeing real usage or exploitation will still get both a blog post and a RustSec advisory. We may also notify via additional communication channels (such as social media) if we feel it is warranted.

## Recent crates

Since we are announcing this policy change now, here is a retrospective summary of the malicious crates removed since [our last blog post][last-post] and today:

- `finch_cli_rust`, `finch-rst`, and `sha-rst`: the Rust security response working group was notified on December 9th, 2025 by Matthias Zepper of [National Genomics Infrastructure Sweden][ngi-sweden] that these crates were attempting to exfiltrate credentials by impersonating the `finch` and `finch_cli` crates. Advisories: [RUSTSEC-2025-0150][advisory-finch-rst], [RUSTSEC-2025-0151][advisory-sha-rst], [RUSTSEC-2025-0152][advisory-finch-cli-rust].
- `polymarket-clients-sdk`: we were notified on February 6th by [Socket][socket] that this crate was attempting to exfiltrate credentials by impersonating the `polymarket-client-sdk` crate. Advisory: [RUSTSEC-2026-0010][advisory-polymarket].
- `polymarket-client-sdks`: we were notified on February 13th that this crate was attempting to exfiltrate credentials by impersonating the `polymarket-client-sdk` crate. Advisory: [RUSTSEC-2026-0011][advisory-polymarket-deux].

In all cases, the crates were deleted, the user accounts that published them were immediately disabled, and reports were made to upstream providers as appropriate.

## Thanks

Once again, our thanks go to Matthias, Socket, and the reporter of `polymarket-client-sdks` for their reports. We also want to thank Dirkjan Ochtman from the secure code working group, Emily Albini from the security response working group, and Walter Pearce from the [Rust Foundation][foundation] for aiding in the response.

[advisory-finch-cli-rust]: https://rustsec.org/advisories/RUSTSEC-2025-0152.html
[advisory-finch-rst]: https://rustsec.org/advisories/RUSTSEC-2025-0150.html
[advisory-sha-rst]: https://rustsec.org/advisories/RUSTSEC-2025-0151.html
[advisory-polymarket]: https://rustsec.org/advisories/RUSTSEC-2026-0010.html
[advisory-polymarket-deux]: https://rustsec.org/advisories/RUSTSEC-2026-0011.html
[foundation]: https://foundation.rust-lang.org/
[last-post]: https://blog.rust-lang.org/2025/12/05/crates.io-malicious-crates-finch-rust-and-sha-rust/
[ngi-sweden]: https://ngisweden.scilifelab.se/
[rss]: https://rustsec.org/feed.xml
[rustsec]: https://rustsec.org/
[socket]: https://socket.dev/
