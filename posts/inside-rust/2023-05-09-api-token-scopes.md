+++
layout = "post"
title = "API token scopes"
author = "Tobias Bieniek"
team = "the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>"
+++

Roughly three years ago [Pietro Albini](https://github.com/pietroalbini) opened an RFC called ["crates.io token scopes"](https://github.com/rust-lang/rfcs/pull/2947). This RFC described an improvement to the existing API tokens, that everyone is using to publish crates to the [crates.io](https://crates.io/) package registry. The proposal was to make it possible to restrict API tokens to 1) certain operations and 2) certain crates.

Unfortunately, the crates.io team members were quite busy at the time, so it took a while for this proposal to get accepted. To be precise, during the [EuroRust](https://eurorust.eu) conference in October 2022 we talked about the RFC again and after a few modifications the RFC was moved into FCP status and then finally merged.

The implementation was started soon after, but was paused again due to other priorities at the time. Fortunately, I was lucky enough to get one of the software engineering jobs at the [Rust Foundation](https://rustfoundation.org/), so in early April the development continued, and I am happy to report:

**API token scopes on crates.io are now in a public beta testing period!**

For details on what these token scopes are and how they are supposed to work, I recommend reading through the [RFC](https://github.com/rust-lang/rfcs/pull/2947). If you want to try them out, you can go to <https://crates.io/settings/tokens/new> and create a new API token scoped to the operations and crates you want:

![Screenshot of the "New API Token" page](../../../../images/inside-rust/2023-05-09-api-token-scopes/screenshot.png)

Please note that this page is currently not reachable from the regular user interface, you have to access it via URL directly while we test it out.

Finally, if you notice any issues, or if you have any questions don't hesitate to find us on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/token.20scopes) or open an issue on [GitHub](https://github.com/rust-lang/crates.io/issues/new/choose).
