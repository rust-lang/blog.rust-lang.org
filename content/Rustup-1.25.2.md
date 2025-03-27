+++
layout = "post"
date = 2023-02-01
title = "Announcing Rustup 1.25.2"
author = "The rustup working group"
+++

The rustup working group is announcing the release of rustup version 1.25.2.
Rustup is the recommended tool to install Rust, a programming language that is
empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.25.2 is as
easy as stopping any programs which may be using Rustup (e.g. closing your IDE)
and running:

```
rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain
update:

```
rustup update
```

If you don't have it already, you can [get rustup](https://rustup.rs/) from the
appropriate page on our website.

## What's new in rustup 1.25.2

This version of rustup fixes a warning incorrectly saying that signature
verification failed for Rust releases. The warning was due to a dependency of
Rustup including a time-based check preventing the use of SHA-1 from February
1st, 2023 onwards.

Unfortunately Rust's release signing key uses SHA-1 to sign its subkeys, which
resulted in all signatures being marked as invalid. Rustup 1.25.2 temporarily
fixes the problem by allowing again the use of SHA-1.

## Why is signature verification failure only a warning?

Signature verification is currently an experimental and incomplete feature
included in rustup, as it's still missing crucial features like key rotation.
Until the feature is complete and ready for use, its outcomes are only
displayed as warnings without a way to turn them into errors.

This is done to avoid potentially breaking installations of rustup. Signature
verification will error out on failure only after the design and implementation
of the feature will be finished.

## Thanks

Thanks again to all the contributors who made rustup 1.25.2 possible!

 * Daniel Silverstone (kinnison)
 * Pietro Albini (pietroalbini)
