+++
layout = "post"
date = 2024-08-08
title = "Announcing Rust 1.80.1"
author = "The Rust Release Team"
release = true
+++

The Rust team has published a new point release of Rust, 1.80.1. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.80.1 is as easy as:

```
rustup update stable
```

If you don't have it already, you can [get `rustup`][rustup] from the
appropriate page on our website.

[rustup]: https://www.rust-lang.org/install.html

## What's in 1.80.1

Rust 1.80.1 fixes two regressions that were recently reported.

### Miscompilation when comparing floats

In addition to the existing optimizations performed by LLVM, rustc is growing
its own set of optimizations. Rust 1.78.0 added a new one, implementing ["jump
threading"] (merging together two adjacent branches that perform the same
comparison).

The optimization was also enabled on branches checking for floating point
equality, but it didn't implement the special rules needed for floats
comparison (`NaN != NaN` and `0.0 == -0.0`). This caused the optimization to
miscompile code performing those checks.

Rust 1.80.1 addresses the problem by preventing the optimization from being
applied to float comparisons, while retaining the optimization on other
supported types.

### False positives in the `dead_code` lint

Rust 1.80.0 contained refactorings to the `dead_code` lint. We received
multiple reports that the new lint implementation produces false positives, so
we are reverting the changes in Rust 1.80.1. We'll continue to experiment on
how to improve the accuracy of `dead_code` in future releases.

## Contributors to 1.80.1

Many people came together to create Rust 1.80.1. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.80.1/)

["jump threading"]: https://en.wikipedia.org/wiki/Jump_threading
