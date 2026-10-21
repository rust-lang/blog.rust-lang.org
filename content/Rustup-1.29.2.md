+++
path = "2026/10/20/Rustup-1.29.2"
title = "Announcing rustup 1.29.2"
authors = ["The Rustup Team"]

[taxonomies]
tags = ["rustup"]
+++

The rustup team is happy to announce the release of rustup version 1.29.2.

[Rustup][install] is the recommended tool to install [Rust][rust], a
programming language that empowers everyone to build reliable and efficient
software.

## What's new in rustup 1.29.2

This is a small release which only updates the rustls-webpki and
webpki-root-certs dependencies to address the problematic eMudhra
intermediate certificates issue described in [CVE-2026-111943].

See also [RUSTSEC-2026-0382].

[CVE-2026-111943]: https://www.cve.org/CVERecord?id=CVE-2026-111943
[RUSTSEC-2026-0382]: https://rustsec.org/advisories/RUSTSEC-2026-0382.html

Further details are available in the [changelog]!

## How to update

If you have a previous version of rustup installed, getting the new one is as easy as stopping
any programs which may be using rustup (e.g. closing your IDE) and running:

```
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
$ rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

Rustup's documentation is also available in [the rustup book][book].

## Caveats

Rustup releases can come with problems not caused by rustup itself but just due to having a new release.

In particular, anti-malware scanners might block rustup or stop it from creating or copying
files, especially when installing `rust-docs` which contains many small files.

Issues like this should be automatically resolved in a few weeks when the anti-malware scanners are updated
to be aware of the new rustup release.

## Thanks

Thanks again to all the [contributors] who made this rustup release possible!

[issues]: https://github.com/rust-lang/rustup/issues
[book]: https://rust-lang.github.io/rustup/
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[contributors]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md#detailed-changes
[install]: https://rustup.rs
[rust]: https://www.rust-lang.org
