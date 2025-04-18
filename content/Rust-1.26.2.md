+++
path = "2018/06/05/Rust-1.26.2"
title = "Announcing Rust 1.26.2"
authors = ["The Rust Core Team"]
aliases = [
    "2018/06/05/Rust-1.26.2.html",
    "releases/1.26.2",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.26.2. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.26.2 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.26.2][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1262-2018-06-05

## What's in 1.26.2 stable

This patch release fixes a bug in the borrow checker verification of `match` expressions. This bug
was introduced in 1.26.0 with the stabilization of [match ergonomics]. Specifically, it permitted
code which took two mutable borrows of the `bar` path at the same time.

```rust
let mut foo = Some("foo".to_string());
let bar = &mut foo;
match bar {
    Some(baz) => {
        bar.take(); // Should not be permitted, as baz has a unique reference to the bar pointer.
    },
    None => unreachable!(),
}
```

1.26.2 will reject the above code with this error message:

```
error[E0499]: cannot borrow `*bar` as mutable more than once at a time
 --> src/main.rs:6:9
  |
5 |     Some(baz) => {
  |          --- first mutable borrow occurs here
6 |         bar.take(); // Should not be permitted, as baz has a ...
  |         ^^^ second mutable borrow occurs here
...
9 | }
  | - first borrow ends here

error: aborting due to previous error
```

The Core team decided to issue a point release to minimize the window of time in which this bug in
the Rust compiler was present in stable compilers.

[match ergonomics]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings
