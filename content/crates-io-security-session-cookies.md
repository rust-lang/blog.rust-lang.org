+++
path = "2025/04/11/crates-io-security-session-cookies"
title = "crates.io security incident: improperly stored session cookies"
authors = ["Adam Harvey"]
aliases = ["2025/04/11/crates-io-security-session-cookies.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

Today the crates.io team discovered that the contents of the `cargo_session`
cookie were being persisted to our error monitoring service,
[Sentry](https://sentry.io/welcome/), as part of event payloads sent when an
error occurs in the crates.io backend. The value of this cookie is a signed
value that identifies the currently logged in user, and therefore these cookie
values could be used to impersonate any logged in user.

Sentry access is limited to a trusted subset of the crates.io team, Rust
infrastructure team, and the crates.io on-call rotation team, who already have
access to the production environment of crates.io. There is no evidence that
these values were ever accessed or used.

Nevertheless, out of an abundance of caution, we have taken these actions
today:

1. We have [merged and deployed a change to redact all cookie values from all
   Sentry events](https://github.com/rust-lang/crates.io/pull/10991).
2. We have invalidated all logged in sessions, thus making the cookies stored
   in Sentry useless. In effect, this means that every crates.io user has been
   logged out of their browser session(s).

Note that API tokens are **not** affected by this: they are transmitted using
the `Authorization` HTTP header, and were already properly redacted before
events were stored in Sentry. All existing API tokens will continue to work.

We apologise for the inconvenience. If you have any further questions, please
contact us on
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io) or
[GitHub](https://github.com/rust-lang/crates.io/discussions).
