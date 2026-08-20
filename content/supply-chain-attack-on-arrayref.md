+++
path = "2026/08/20/supply-chain-attack-on-arrayref"
title = "Supply chain attack on arrayref"
authors = ["Manish Goregaokar"]

[extra]
team = "security-response"
team_url = "https://www.rust-lang.org/governance/teams/#team-security-response"
+++


## What happened

On 2026-08-20 at 7:15 UTC we got a report that the `proc-macro1` crate was malicious.

The Rust Security Response Team verified this to be the case: the crate had a build script that was downloading a malicious payload.

This crate `proc-macro1` and others like it (`proc-macro-en`, `aovine`, `arone`, `aronenao`, `tinymember`) have been deleted.

Furthermore, we discovered that the popular [`arrayref`](https://crates.io/crates/arrayref) crate had recently been republished and made to depend on this crate, with the most recent versions yanked. We have removed the malicious version and unyanked the maliciously-yanked versions. Other crates by that author ([`internment`](https://crates.io/crates/internment), [`append-only-vec`](https://crates.io/crates/append-only-vec)) were also affected so we have done the same for those, and locked the account as a precaution. We do not believe the author of `arrayref` to be acting maliciously, but their computer or credentials are likely compromised, and we are attempting to contact them.

## What you need to do

We recommend you check your local dependencies to ensure these crates were not pulled in.
Here are the malicious versions that we deleted from crates.io:

* `append-only-vec@0.1.9`.
* `arrayref@0.3.10`: published at `2026-08-20T07:15:00Z`, deleted at `2026-08-20T08:41:40Z`. Online for 86 minutes.
* `internment@0.8.7`.
* `proc-macro1`, `proc-macro-en`, `aovine`, `arone`, `aronenao`, `tinymember` (any versions).

You can quickly check if these crates have been used locally by going through `~/.cargo/registry/cache` with this command:

```bash
find ~/.cargo/registry/cache -type f \( \
  -name 'append-only-vec-0.1.9.crate' -o \
  -name 'arrayref-0.3.10.crate' -o \
  -name 'internment-0.8.7.crate' -o \
  -name 'proc-macro1-*.crate' -o \
  -name 'proc-macro-en-*.crate' -o \
  -name 'aovine-*.crate' -o \
  -name 'arone-*.crate' -o \
  -name 'aronenao-*.crate' -o \
  -name 'tinymember-*.crate' \
\) -print
```

## Thanks

We'd like to thank the Research Team at Nextron Systems GmbH for initially discovering this and reporting it to us. We'd also like to thank Emily Albini, Manish Goregaokar, Marco Ieni, Tobias Bieniek, Ubiratan Soares, and Walter Pearce for participating in the response here.
