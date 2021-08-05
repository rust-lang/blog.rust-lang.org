---
layout: post
title: "Introducing Disjoint Capture in Closures"
author: The RFC 2229 working group <https://www.rust-lang.org/governance/teams/compiler#wg-rfc-2229>
---

One of the major features of the [upcoming Rust 2021 Edition](https://blog.rust-lang.org/2021/05/11/edition-2021.html)) is a change to how Rust closures work. This change makes closure captures more precise, eliminating common borrow check errors. As a result, it involves some (minor) changes to Rust semantics, which is why it is tied to the Rust 2021 edition.

Like any Edition migration, we have also created lints that will warn you of upcoming changes and suggest precise edits to preserve the semantics of your code. This blog post will explain the new feature and also tell you how you can try it out today on nightly if you like.


## What are closures?
Closures are anonymous functions you can save in a variable or pass as an argument to other functions. They are referred to as ”lambda functions” in other programming languages.
    
This is a basic closure that prints "Hello, World!":

```rust
let c = || println!("Hello, World!"); 
```


Closures can capture variables from the scope they are defined in.
 ```rust
let my_string = "Hello, World";
let c = || println!("{}", my_string); // closure captures (& my_string)
```


If you're new to Rust and haven't used closures, we recommend you check out the [Rust Book](https://doc.rust-lang.org/book/ch13-01-closures.html) and [Rust by Example](https://doc.rust-lang.org/rust-by-example/fn/closures.html) to learn how to use closures.

For those who would like a quick recap, we recommend reading the [Rust Reference](https://doc.rust-lang.org/reference/types/closure.html).


## What is disjoint capture?

"Disjoint capture" is the ability of a closure to capture only part of a variable rather than the variable in its entirety (the behavior in Rust 2018). The goal with this feature is to unify the semantics of closure captures with those of borrowing and move operations.

Consider a `Point` `p` with fields `x` and `y` corresponding to the x and y coordinates of the point.

```rust
let p = Point { x: 10, y: 20 };

let c = || println!("{}", p.x);
```

`c` in Rust 2018 will capture `p` completely, but in Rust 2021 it will only capture `p.x`. This now matches what gets borrowed when someone writes `println!("{}", p.x)`.

Disjoint capture was proposed as part of [RFC 2229](https://github.com/rust-lang/rfcs/blob/master/text/2229-capture-disjoint-fields.md), and we suggest reading the RFC to better understand motivations behind the feature.

The precise path that gets captured is typically the full path that is used in the closure, but there are cases where we will only capture a prefix of the path. See this pending PR to the [Rust Reference](https://github.com/rust-lang/reference/blob/6b88e48ffebc46be91884ef6237accb947e5f6f3/src/types/closure.md) for the full details.

The feature also includes (minor) breaking changes to the Rust semantics which are also documented in this pending PR to the [Rust Reference](https://github.com/rust-lang/reference/blob/6b88e48ffebc46be91884ef6237accb947e5f6f3/src/types/closure.md).

## How to use the feature?

Interested in testing this out? You can now try disjoint capture in Rust closures on rust nightly using `#![feature(capture_disjoint_fields)]`. 

If you would like to be warned of semantics change that may impact your code, you can follow migration instructions provided in the [2021 Edition Guide](https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html#migration).

## How to submit bugs?

To submit a bug simply [open an issue](https://github.com/rust-lang/rust/issues/new/choose) and tag the RFC 2229 working group using `@rust-lang/wg-rfc-2229`. We hope to create the best experince posible so no issue is too small, even a confusing error messages is worth reporting.
