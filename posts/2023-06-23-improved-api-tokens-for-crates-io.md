+++
layout = "post"
title = "Improved API tokens for crates.io"
author = "Tobias Bieniek"
team = "the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>"
+++

If you recently generated a new API token on crates.io, you might have noticed
our new API token creation page and some of the new features it now supports.

Previously, when clicking the "New Token" button on <https://crates.io/settings/tokens>,
you were only provided with the option to choose a token name, without any
additional choices. We knew that we wanted to offer our users more flexibility,
but in the previous user interface that would have been difficult, so our first
step was to build a proper "New API Token" page.

Our roadmap included two essential features known as "token scopes". The first
of them allows you to restrict API tokens to specific operations. For instance,
you can configure a token to solely enable the publishing of new versions for
existing crates, while disallowing the creation of new crates. The second one
offers an optional restriction where tokens can be limited to only work for
specific crate names. If you want to read more about how these features
were planned and implemented you can take a look at our corresponding
[tracking issue](https://github.com/rust-lang/crates.io/issues/5443).

To further enhance the security of crates.io API tokens, we prioritized the
implementation of expiration dates. Since we had already touched most of the
token-related code this was relatively straight-forward. We are delighted to
announce that our "New API Token" page now supports endpoint scopes, crate
scopes and expiration dates:

![Screenshot of the "New API Token" page](/images/2023-06-23-improved-api-tokens-for-crates-io/new-api-token-page.png)

Similar to the API token creation process on github.com, you can choose to not
have any expiration date, use one of the presets, or even choose a custom
expiration date to suit your requirements.

If you come across any issues or have questions, feel free to reach out to us on
[Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/topic/token.20scopes)
or open an issue on [GitHub](https://github.com/rust-lang/crates.io/issues/new/choose).

Lastly, we, the crates.io team, would like to express our gratitude to the
[OpenSSF's Alpha-Omega Initiative](https://openssf.org/community/alpha-omega/)
and [JFrog](https://jfrog.com/blog/jfrog-joins-rust-foundation-as-platinum-member/)
for their contributions to the [Rust Foundation](https://rustfoundation.org)
security initiative. Their support has been instrumental in enabling us to
implement these features and undertake extensive security-related work on the
crates.io codebase over the past few months.
