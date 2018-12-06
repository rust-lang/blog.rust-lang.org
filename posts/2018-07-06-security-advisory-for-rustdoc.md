---
layout: post
title: "Security Advisory for rustdoc"
author: "The Rust Core Team"
---

## Quick overview

The Rust team was recently notified of a security vulnerability affecting
rustdoc plugins. If you are not using rustdoc plugins, you are not affected.
We're not aware of any usage of this feature. The associated CVE is [CVE-2018-1000622].

You can find the full announcement on our `rustlang-security-announcements`
mailing list [here](https://groups.google.com/forum/#!topic/rustlang-security-announcements/4ybxYLTtXuM).

[CVE-2018-1000622]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=%20CVE-2018-1000622

## Announcement

On Tuesday July 3rd, Red Hat reported a security vulnerability in `rustdoc` to
us. The problem was in rustdoc’s obscure plugin functionality, consisting of
its loading plugins from a path that is globally writable on most platforms,
`/tmp/rustdoc/plugins`. This feature permitted a malicious actor to write a
dynamic library into this path and have another user execute that code.  The
security issue only happens if you're actively using the feature, and so this
behavior will be removed from rustdoc in the near future, with patches landing
for each channel over the next week. The plugin infrastructure predates 1.0 and
is not usable on stable or nightly Rust today. Its removal should not impact
any Rust users.

As Rust’s first official CVE, this is somewhat of a milestone for us. The fix
will be out in 1.27.1 on Tuesday July 10th. Because there's no embargo, we are
filing for a CVE now, and will update this post with the number once we are
assigned one.

Despite the acknowledge low impact and severity of this bug, the Rust team
decided to follow the full procedure we have for security bugs. We know of no
one who uses this functionality, so we felt comfortable discussing it publicly
ahead of the patch release. The impact is limited due to the plugin
functionality being long deprecated and being unusable on all current versions
of Rust, as the required library is not shipped to users. However, since the
bug can potentially cause problems for users, we decided to include this in the
1.27.1 stable release.

It’s worth noting that while Rust does prevent a lot of issues in your code at
compile time, they’re issues that result from memory unsafety. This bug is a
logic error. Rust code is not inherently secure, or bug-free. Sometimes, people
get enthusiastic and make overly-broad claims about Rust, and this incident is
a good demonstration of how Rust’s guarantees can’t prevent all bugs.

Thank you to Red Hat for responsibly disclosing the problem and working with us
to ensure that the fix we plan to ship is correct.

