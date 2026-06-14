+++
path = "inside-rust/9999/12/31/unite-for-clippy"
title = "United for a strong Clippy"
authors = ["blyxyas"]

[extra]
team = "the Clippy team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-clippy"
+++

# United for a strong Clippy

Clippy is a very loved piece of software, with the fortune of having many
contributors willing to do high-quality pull requests with very needed features
and bug-fixes. We are extremely thankful for every single contributor who not
only took the time to learn about Clippy’s internals, but also felt comfortable
enough to show their work and maintained communication in order to ship it.

That’s why we’re addressing them with this health report of Clippy as an open
source project.

The truth is that Clippy has a reviewing capacity problem, and we’ve been having
it for the last few months. Unlike some other Rust teams, no one from the Clippy
team is currently funded to work on Clippy, not even on a part-time basis. The
burden of maintenance thus falls squarely into the maintainer’s free time,
eventually leading to burnout.

> We hope that the newly established Rust Foundation Maintainers Fund will be able
> to help secure some funding for Clippy maintainers. If you'd like to help,
> consider [donating] to the fund.

That’s why we are launching an initiative in which contributors help themselves
get their pull request reviewed and merged *faster*.

## What does this mean

Once a contributor opens a pull request, we propose them to review another
contributor’s pull request. That way each contributor reviews at least once pull
request and nobody’s contributions get stale.

The underlying mechanism looks like this: Once a contributor opens a pull request,
our dear bot [@bors](https://github.com/bors) will prompt them to look at other
pull requests (preferably ones without any other reviews). After they’ve reviewed
a pull request, the contributor gets on a special *secret* list of cool ones, and
they’ll both remove burden from the maintainers and get a grateful message from bors.

## Together, we make Clippy healthier

Clippy is no stranger to health updates of this kind, last year we had the
[Feature Freeze], which was an absolute success. In that process, we gained
knowledge, trust, and the certainty that the Rust community cared about
Clippy, and Clippy doing things *right.*

Once again, the team wants to thank and congratulate every single person,
contributor or not, that cares about Clippy. Striving for excellence is what
makes this linter (and language) so loved, and with the help of the community
(of YOU), we can help Clippy become stronger.

[donating]: https://github.com/sponsors/rustfoundation
[Feature Freeze]: https://blog.rust-lang.org/inside-rust/2025/10/22/clippys-feature-warming-up/
