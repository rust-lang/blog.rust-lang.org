---
layout: post
title: crates.io security advisory
author: Rust Security Response WG
---

This is a cross-post of [the official security advisory][ml]. The official post
contains a signed version with our PGP key, as well.

---

The Rust Security Response Working Group was recently notified of a security
issue affecting token generation in the [crates.io] web application, and while
investigated that issue we discovered an additional vulnerability affecting
crates.io API tokens.

We have no evidence of this being exploited in the wild, but out of an
abundance of caution we opted to revoke all existing API keys. You can generate
a new one at [crates.io/me].

## Overview

Until recently, API keys for [crates.io] were generated using the PostgreSQL
random function, which is not a cryptographically secure random number
generator. This means that in theory, an attacker could observe enough random
values to determine the internal state of the random number generator, and use
this information to determine previously created API keys up to the last
database server reboot.

As part of the investigation for this, we also found that API keys were being
stored in plain text. This would mean if our database were somehow compromised
the attacker would be have API access for all current tokens.

## Mitigations

We deployed a code change to production to use a cryptographically secure
random number generator, and we implemented hashing for storing tokens in the
database.

Exploiting either issue would be incredibly impractical in practice, and we've
found no evidence of this being exploited in the wild. However, out of an
abundance of caution, we've opted to revoke all existing API keys. You can
generate a new API key by visiting [crates.io/me]. We apologize for any
inconvenience this causes.

## Acknowledgements

Thanks to [Jacob Hoffman-Andrews] for responsibly disclosing the random number
generator issue according to [our security policy][policy]. Thanks to [Siân
Griffin] and [Justin Geibel] from the crates.io team for helping the Security
Response WG addressing both of the issues. Thanks to [Pietro Albini] from the
Security Response WG for coordinating the work on this vulnerability.

## Timeline of events

All times are listed in UTC.

- 2020-07-11 17:43 - The issue is reported to [security@rust-lang.org]
- 2020-07-11 20:56 - The issue is acknowledged, the leads of the crates.io team
  are looped in
- 2020-07-11 23:48 - The issue is confirmed and a planned fix is agreed on
- 2020-07-13 08:00 - The development of the fix is started
- 2020-07-14 12:53 - The fix is tested on the staging environment
- 2020-07-14 19:03 - The fix is deployed, existing tokens are revoked, and the
  issue is disclosed publicly

[ml]: https://groups.google.com/forum/?oldui=1#!topic/rustlang-security-announcements/wc5d_Qq35RA
[policy]: https://www.rust-lang.org/policies/security
[security@rust-lang.org]: mailto:security@rust-lang.org
[crates.io]: https://crates.io
[crates.io/me]: https://crates.io/me
[Jacob Hoffman-Andrews]: https://github.com/jsha
[Siân Griffin]: https://github.com/sgrif
[Justin Geibel]: https://github.com/jtgeibel
[Pietro Albini]: https://github.com/pietroalbini
