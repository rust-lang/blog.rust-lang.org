+++
path = "2026/09/03/Rust-1.98.1"
title = "Announcing Rust 1.98.1"
authors = ["The Rust Release Team"]
aliases = ["releases/1.98.1"]

[extra]
release = true
+++

The Rust team has published a new point release of Rust, 1.98.1. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.98.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.98.1

Rust 1.98.1 fixes a [miscompilation in vtable generation](https://github.com/rust-lang/rust/issues/161441).

In Rust 1.98.0, in some circumstances, rustc would incorrectly generate a trait object vtable with a null pointer
where a function pointer should be. This leads to undefined behavior in the
emitted code. In some cases this may 'just' cause segfaults due to the null
pointer being loaded, but it is possible for it to be justification for
arbitrary effects (as is typical for UB).

If you'd like to help us out by testing future releases, you might consider
running your code's CI or locally using the beta channel (`rustup default beta`) or the nightly
channel (`rustup default nightly`). Please
[report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you
might come across!

### Contributors to 1.98.1

Many people came together to create Rust 1.98.1. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.98.1/)
