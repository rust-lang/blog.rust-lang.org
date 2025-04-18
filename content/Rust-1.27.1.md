+++
path = "2018/07/10/Rust-1.27.1"
title = "Announcing Rust 1.27.1"
authors = ["The Rust Core Team"]
aliases = [
    "2018/07/10/Rust-1.27.1.html",
    "releases/1.27.1",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.27.1. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.27.1 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.27.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1271-2018-07-10

## What's in 1.27.1 stable

This patch release fixes a bug in the borrow checker verification of `match` expressions.
This bug was introduced in 1.26.0 with the stabilization of [match ergonomics]. We are
uncertain that this specific problem actually indicated unsoundness in the borrow checker,
but suspected that it might be a possibility, so decided to issue a point release. The
code sample below caused a panic inside the compiler prior to this patch.

```rust
fn main() {
    let a = vec!["".to_string()];
    a.iter().enumerate()
            .take_while(|(_, &t)| false)
            .collect::<Vec<_>>();
}
```

1.27.1 will reject the above code with this error message:

```
error[E0507]: cannot move out of borrowed content
    --> src/main.rs:4:30
    |
  4 |             .take_while(|(_, &t)| false)
    |                              ^-
    |                              ||
    |                              |hint: to prevent move, use `ref t` or `ref mut t`
    |                              cannot move out of borrowed content

error: aborting due to previous error
```

Alongside the match ergonomics fix, a [security vulnerability] was also found in rustdoc,
the standard documentation generator for Rust projects. That vulnerability is addressed by
the second patch contained in this release, by removing the default search path for
rustdoc plugins. This functionality will be entirely removed in Rust 1.28.0. This plugin
infrastructure predates Rust 1.0 and has never been usable on stable, and has been
unusable on nightly for many months. Expect to hear more about the removal in the next
release: the current patch removes the default search path (instead, users must specify it
explicitly), while the next release will remove the functionality entirely.

[security vulnerability]: https://blog.rust-lang.org/2018/07/06/security-advisory-for-rustdoc.html
[match ergonomics]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings
