+++
path = "2025/09/12/crates-io-phishing-campaign"
title = "crates.io phishing campaign"
authors = ["Rust Security Response WG", "crates.io team"]
aliases = []
+++

We received multiple reports of a phishing campaign targeting crates.io users
(from the `rustfoundation.dev` domain name), mentioning a compromise of our
infrastructure and asking users to authenticate to limit damage to their crates.

These emails are malicious and come from a domain name not controlled by  the
Rust Foundation (nor the Rust Project), seemingly with the purpose of stealing
your GitHub credentials. We have no evidence of a compromise of the crates.io
infrastructure.

We are taking steps to get the domain name taken down and to monitor for
suspicious activity on crates.io. Do not follow any links in these emails if you
receive them, and mark them as phishing with your email provider.

If you have any further questions please reach out to <security@rust-lang.org>
and <help@crates.io>.
