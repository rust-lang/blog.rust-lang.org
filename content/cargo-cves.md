+++
layout = "post"
date = 2022-09-14
title = "Security advisories for Cargo (CVE-2022-36113, CVE-2022-36114)"
author = "The Rust Security Response WG"
+++

> This is a cross-post of [the official security advisory][advisory]. The
> official advisory contains a signed version with our PGP key, as well.

[advisory]: https://groups.google.com/g/rustlang-security-announcements/c/ldvsemwk_VY

The Rust Security Response WG was notified that Cargo did not prevent
extracting some malformed packages downloaded from alternate registries. An
attacker able to upload packages to an alternate registry could fill the
filesystem or corrupt arbitary files when Cargo downloaded the package.

These issues have been assigned CVE-2022-36113 and CVE-2022-36114. The severity
of these vulnerabilities is "low" for users of alternate registries. Users
relying on crates.io are not affected.

Note that **by design** Cargo allows code execution at build time, due to build
scripts and procedural macros. The vulnerabilities in this advisory allow
performing a subset of the possible damage in a harder to track down way. Your
dependencies must still be trusted if you want to be protected from attacks, as
it's possible to perform the same attacks with build scripts and procedural
macros.

## Arbitrary file corruption (CVE-2022-36113)

After a package is downloaded, Cargo extracts its source code in the `~/.cargo`
folder on disk, making it available to the Rust projects it builds. To record
when an extraction is successfull, Cargo writes "ok" to the `.cargo-ok` file at
the root of the extracted source code once it extracted all the files.

It was discovered that Cargo allowed packages to contain a `.cargo-ok`
*symbolic link*, which Cargo would extract. Then, when Cargo attempted to write
"ok" into `.cargo-ok`, it would actually replace the first two bytes of the
file the symlink pointed to with `ok`. This would allow an attacker to corrupt
one file on the machine using Cargo to extract the package.

## Disk space exhaustion (CVE-2022-36114)

It was discovered that Cargo did not limit the amount of data extracted from
compressed archives. An attacker could upload to an alternate registry a
specially crafted package that extracts way more data than its size (also known
as a "zip bomb"), exhausting the disk space on the machine using Cargo to
download the package.

## Affected versions

Both vulnerabilities are present in all versions of Cargo. Rust 1.64, to be
released on September 22nd, will include fixes for both of them.

Since these vulnerabilities are just a more limited way to accomplish what a
malicious build scripts or procedural macros can do, we decided not to publish
Rust point releases backporting the security fix. Patch files for Rust 1.63.0
are available [in the wg-security-response repository][1] for people building
their own toolchains.

## Mitigations

We recommend users of alternate registries to excercise care in which package
they download, by only including trusted dependencies in their projects. Please
note that even with these vulnerabilities fixed, by design Cargo allows
arbitrary code execution at build time thanks to build scripts and procedural
macros: a malicious dependency will be able to cause damage regardless of these
vulnerabilities.

crates.io implemented server-side checks to reject these kinds of packages
years ago, and there are no packages on crates.io exploiting these
vulnerabilities. crates.io users still need to excercise care in choosing their
dependencies though, as the same concerns about build scripts and procedural
macros apply here.

## Acknowledgements

We want to thank Ori Hollander from JFrog Security Research for responsibly
disclosing this to us according to the [Rust security policy][2].

We also want to thank Josh Triplett for developing the fixes, Weihang Lo for
developing the tests, and Pietro Albini for writing this advisory. The
disclosure was coordinated by Pietro Albini and Josh Stone.

[1]: https://github.com/rust-lang/wg-security-response/tree/master/patches
[2]: https://www.rust-lang.org/policies/security
