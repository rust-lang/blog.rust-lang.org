---
layout: post
title: "A crates.io typosquatting retrospective"
author: Adam Harvey
team: the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>
---

About two months ago, the crates.io team [agreed to run an experiment][typomania-pr] for about two months where all new crates would be checked for potential typosquatting as they were published, after which a decision would be made on whether these checks had sufficient value to retain them.

When potential typosquatting was found during this experiment, two members of Rust Foundation staff were notified: myself, and Walter Pearce[^security-initiative]. When notified, one or both of us checked the crate, and then malicious crates had appropriate action[^action] taken in collaboration with the [crates.io team][crates-io-team] and the [security response working group][security-response-wg].

## Results

Between the deployment of the typosquatting checks on November 14, 2023, and the writing of this blog on January 22, 2024, 6,799 new crates were published. 106 (1.6%) triggered one or more typosquatting checks, of which 2 (1.9% of those crates) turned out to be malicious. We also found an additional 2 malicious crates as a result of these checks that did not trigger typosquatting checks[^also].

Additionally, two malicious crates were reported[^additional-malware] during this period that did not use names that triggered the typosquatting checks. The typosquatting checks were adjusted after this incident in a way that would have caught those crates.

While the false positive rate is higher than is ideal, we now have detailed data on which checks have proven more valuable than others so far, and will be adjusting the typosquatting check configuration accordingly. Based on the data we have, we expect this will reduce the false positive rate by approximately one third.

A deeper dive into individual check performance and false positive trends will be posted on the Rust Foundation blog in the near future.

## Decision

The crates.io team is excited to incorporate these checks to help protect Rust users, and they are now a fully supported features of crates.io.

A few steps will be taken to improve the typosquatting check functionality as it becomes a permanent part of crates.io:

- As noted above, check configuration will be adjusted to reduce the false positive rate.
- Reports will now go to a public Zulip stream, rather than just to Rust Foundation staff.[^continued]
- A new process will be added to the crates.io ops guide to formalise what happens when a malicious crate is found.
- Typosquatting functionality will be more deeply integrated into crates.io, particularly around configuration, to make it more maintainable in the long term.[^separation]

These changes will be implemented by the end of January.

## Thanks

On a personal level, I'd like to thank the following people for helping with this project:

- The [crates.io team][crates-io-team]: Justin, Tobias, Carol, Rustin, Yuki, and Matthew, for being willing to let us run this experiment and being open to making part of crates.io moving forward.
- The [security response working group][security-response-wg]: Josh, Manish, and Pietro, for being available to advise and help when malicious crates are found.
- The [typogard][typogard] team led by Matthew Taylor, who did the original research this project was built on.
- Dan Gardner at AWS, who adapted typogard to work with crates.io, which provided the initial data suggesting that this would be a useful experiment.
- The [Rust Foundation][rust-foundation] for sponsoring this work.

[^action]: This action always includes deleting the crates involved. Further actions vary depending on what is found — they can include, but are not limited to, reporting malware to other hosts, reporting accounts to identity providers, analysing crates.io logs to ensure that no users have inadvertently been exposed, and liaising with local law enforcement as appropriate.

[^additional-malware]: These were detected at the same time by other malware checks being run by the Rust Foundation, and were from the same user, so were considered the same incident once reported.

[^also]: These crates belonged to the same users that had published crates that did trigger typosquatting checks — once a user has published one malicious crate, we check their other crates as a matter of course.

[^continued]: The Rust Foundation will continue triaging these reports as part of its ongoing [Security Initiative][security-initiative-intro], but also welcomes any community members who would like to help!

[^security-initiative]: This work is funded as part of the Rust Foundation's [Security Initiative][security-initiative-intro], an ongoing effort to improve security within the Rust ecosystem by working with the Rust Project and other stakeholders.

[^separation]: The typosquatting checks were kept intentionally isolated from crates.io as much as possible to make it easier to remove them if the experiment was unsuccessful.

[crates-io-team]: https://www.rust-lang.org/governance/teams/crates-io
[rust-foundation]: https://rustfoundation.org/
[security-initiative-intro]: https://foundation.rust-lang.org/news/2022-09-13-rust-foundation-establishes-security-team/
[security-response-wg]: https://www.rust-lang.org/governance/wgs/wg-security-response
[typogard]: https://github.com/mt3443/typogard
[typomania-pr]: https://github.com/rust-lang/crates.io/pull/7206
