---
layout: post
title: "Security advisory for the standard library"
author: The Rust Core Team
---

The Rust team was recently notified of a security vulnerability affecting
the standard library's `str::repeat` function. When passed a large number this
function has an integer overflow which can lead to an out of bounds write. If
you are not using `str::repeat`, you are not affected.

We're in the process of applying for a CVE number for this vulnerability. Fixes
for this issue have landed in the Rust repository for the stable/beta/master branches.
Nightlies and betas with the fix will be produced tonight, and 1.29.1 will be
released on 2018-09-25 with the fix for stable Rust.

You can find the full announcement on our rustlang-security-announcements mailing
list [here].

[here]: https://groups.google.com/forum/#!topic/rustlang-security-announcements/CmSuTm-SaU0
