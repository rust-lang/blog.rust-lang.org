---
layout: post
title: Improved API tokens for crates.io
author: Tobias Bieniek
team: the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>
---

If you created a new API token on crates.io lately, you may have noticed our new
API token creation page and some of the new features it supports.

Previously, when you clicked the "New Token" button on <https://crates.io/settings/tokens>
you would only get the option to choose a name for the token, but nothing else.
We knew that we wanted to offer our users more choices, but in the previous user
interface that would have been difficult, so our first step was to build a
proper "New API Token" page.

Our next two features on the to-do list were both described as "token scopes".
One part is allowing you to restrict API tokens to certain operations, e.g.
only allowing a token to publish new versions of existing crates, but not any
new crates. The second part is an optional restriction for the token to only
work with certain crate names. If you want to read more about how these features
were planned and implemented you can take a look at our corresponding
[tracking issue](https://github.com/rust-lang/crates.io/issues/5443).

The remaining piece to making crates.io API tokens more secure was implementing
expiration dates for them. Since we had already touched most of the
token-related code this was relatively straight-forward, and we are happy to
announce that our "New API Token" page now supports endpoint scopes, crate
scopes and expiration dates:

![Screenshot of the "New API Token" page](/images/2023-06-23-improved-api-tokens-for-crates-io/new-api-token-page.png)

Similar to when you create an API token on github.com, you can choose to not
have an expiration date, use one of the presets, or even choose a custom
expiration date.

If you notice any issues, or if you have any questions don't hesitate to find us
on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/token.20scopes)
or open an issue on [GitHub](https://github.com/rust-lang/crates.io/issues/new/choose).

Finally, the crates.io team would like to thank the [OpenSSF's Alpha-Omega Initiative](https://openssf.org/community/alpha-omega/)
and [JFrog](https://jfrog.com/blog/jfrog-joins-rust-foundation-as-platinum-member/)
for funding the [Rust Foundation](https://rustfoundation.org) security
initiative, which enabled us to implement these features and perform a lot of
other security-related work on the crates.io codebase in the past couple of months!