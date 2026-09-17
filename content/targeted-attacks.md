+++
path = "2026/09/17/targeted-attacks"
title = "Be alert: targeted attacks on prominent Rustaceans"
authors = ["Adam Harvey"]

[extra]
team = "the crates.io team and security response working group"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

We believe that there is an ongoing campaign targeting rust-lang members and
owners of popular crates that is attempting to compromise devices and accounts
in order to use them to publish malware.

## What we've seen

A video call is set up for something positive — maybe for a job, maybe for a
project, maybe for a contract opportunity — and then that's used as a vector to
either get the target to install something on their computer (such as a
purportedly missing audio codec) or execute another command (for example, via
putting a command on the clipboard).

These attackers are setting up new but legitimate seeming company profiles,
including plausible LinkedIn presences, in order to pass cursory inspection.

A [previous attack of this form][grack] targeted many prominent Rust developers in
June, and, last month, the [`arrayref` crate was briefly compromised through similar
attacks][arrayref]. At this moment we do not know if these are all a part of the same
campaign.

This attack style is [known to be used by the DPRK][dprk], and has been [seen outside of the Rust community as well][ashishb].

## What you can do

Please take extra care in the near term. Be appropriately suspicious of cold
outreaches, and ensure that any calls you have with new people are on platforms
you trust — ideally, try to be the one who sets up the call on a platform you
already use.

Please also re-check that your accounts look normal: MFA enabled,
no unexpected logins on platforms that can track that, and so on.

If you have any concerns about your accounts, please reach out to
[help@crates.io](mailto:help@crates.io) (for crates.io account concerns) and/or
[security@rust-lang.org](mailto:security@rust-lang.org) (for any other
concerns). We're very happy to help.


[arrayref]: https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/
[ashishb]: https://ashishb.net/security/contagious-interview/
[dprk]: https://kudelskisecurity.com/research/how-dprks-contagious-interview-campaign-targets-developers
[grack]: https://grack.com/blog/2026/06/25/dissecting-a-failed-nation-state-attack/
