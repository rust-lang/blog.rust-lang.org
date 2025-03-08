+++
layout = "post"
title = "What the Error Handling Project Group is Working On"
author = "Sean Chen"
team = "the library team <https://www.rust-lang.org/governance/teams/library>"
+++

The Rust community takes its error handling seriously. There’s already a strong culture in place for emphasizing helpful error handling and reporting, with multiple libraries each offering their own take (see Jane Lusby’s thorough [survey][error_ecosystem_vid] of Rust error handling/reporting libraries).

But there’s still room for improvement. The main focus of the group is carrying on error handling-related work that was in progress before the group's formation. To that end, we're working on systematically addressing error handling-related issues, as well as eliminating blockers that are holding up stalled RFCs.  

Our first few meetings saw us setting a number of short- and long-term goals. These goals fall into one of three themes: making the `Error` trait more universally accessible, improving error handling ergonomics, and authoring additional learning resources. 

## One Standardized `Error` Trait 

The `Error` trait has been around since 1.0, and exposed two methods: `Error::description` and `Error::cause`. As it was originally constructed, it was too restictive for a number of reasons<sup>1</sup>. The `Failure` crate addressed many of the `Error` trait's shortcomings by exporting the `Fail` trait, which informs many of changes that are being made to improve the `Error` trait. 

On that note, bolstering the `std::error::Error` trait such that it could be adopted across the Rust community as _the_ `Error` trait has been an ongoing process since [RFC 2504][rfc2504] was merged in August 2018. 

This process also involves stabilizing many `Error` trait APIs and crates that are, as of this writing, on nightly only. These include the `backtrace` and `chain` methods, which are both extremely useful for working with error types. If you’re interested in following or contributing to this work, take a look at [this issue][core_error_issue].

Another related initiative is migrating the `Error` trait to `core` so that it’s more widely accessible to different use cases (such as in FFI or embedded contexts).

## More Ways to Access Error Contexts

Rust’s language semantics already provide a decently ergonomic error handling experience, what with the `Result` type and the `?` operator. The error handling group has identified a few additional features to further improve the error handling user experience. 

### Adding the Capability to Iterate Through the `Backtrace` Type

As of this writing, the `backtrace` type only implements the `Display` and `Debug` traits. This means that the only way to work with the `backtrace` type is to print it out, which is less than ideal. An iterator API that provided the ability to iterate through stack frames would give users the ability to control how their backtraces are formatted, which is a necessary step adding `std::backtrace::Backtrace` support to crates like `color-backtrace`.

Upon researching strategies for how to tackle this, we found that the `backtrace` crate already has a `frames` method that would work nicely for implementing the `Iterator` API. It should be a relatively straightforward ordeal to expose an identical method in `std`. 

A [PR][backtrace_frames_pr] for this has been opened for anyone who would like to check it out. 

### Generic Member Access

Currently, when we want to fetch some additional context related to an error, there are specific methods that need to be called in order to fetch that context. For example, to see the backtrace for an error, we’d call the `backtrace` method: `let backtrace = some_error.backtrace();`. The problem with this approach is that it's not possible to support types that are defined outside of `std`. Even for types that exist within `std`, a method to access each respective type needs to be defined, which makes things cumbersome and harder to maintain. 

As the name implies, generic member access, when it gets implemented, is a type-agnostic way to access different pieces of context from an `Error` trait object. The analogy that clicked for me is when you’re parsing a string into a number, with something like:

```rust
let ten = "10".parse::<i32>();
```

Or when you’re collecting the contents yielded by an iterator:

```rust
use std::collections::HashSet;

let a_to_z_set = ('a'..='z').collect::<HashSet<_>>();
```

In a similar vein, you’d be able to access some piece of context from an error by specifying its type ID:

```rust
let span_trace = some_error.context::<&SpanTrace>();
```

This could be used to fetch other pieces of context related to the error such as its backtrace, the error’s sources, status codes, alternate formatting representations (such as `&dyn Serialize`).

This feature will enable other features we plan on adding down the line, such as exposing a way to report back all of the locations from which errors originated from in a program, as well as exposing a more consistent error reporting format besides just `Display` and `Debug`. 

Jane has been putting in a lot of work on pushing these ideas forward. You can check out the associated [RFC][gma_rfc].

## Authoring a Book on Rust Error Handling Best Practices

Last but not least, there’s a lot of interest in the group around authoring [_The Rust Error Book_][error_book]. The aim of the book would be to codify and communicate different error handling best practices based on the respective use-case. This could include FFI use-cases, or best practices around returning error codes from programs.

This is an ongoing effort that will see a lot of progress in the coming weeks and months!

## In Summary

We're excited by the opportunities to continue to iterate on and improve Rust's error handling ergonomics and culture! If you're interested in helping out and/or joining in on the conversation, please come by and introduce yourself in our [Zulip stream][zulip]. You can also keep track of our progress via our [GitHub repo][peh_repo].

Lastly, we'll be presenting some forthcoming news about a universally consistent error reporting format in our next update, so stay tuned for that!

### Footnotes

<sup>1</sup>The `Error::description` method only supported string slices, which meant that it was not straightforward to create dynamic error messages that included additional context. This method was deprecated in favor of `Display`. The `Error::cause` method, now known as `Error::source`, doesn't enforce errors having a `'static` lifetime, which means that downcasting error sources is impossible, making it much more difficult to handle errors using dynamic error handlers.

[rfc2504]: https://github.com/rust-lang/rfcs/pull/2504
[core_error_issue]: https://github.com/rust-lang/project-error-handling/issues/3
[error_ecosystem_vid]: https://youtu.be/rAF8mLI0naQ
[gma_rfc]: https://github.com/rust-lang/rfcs/pull/2895
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/257204-project-error-handling
[peh_repo]: https://github.com/rust-lang/project-error-handling
[backtrace_frames_pr]: https://github.com/rust-lang/rust/pull/78299
[error_book]: https://github.com/rust-lang/project-error-handling/tree/master/the-rust-error-book
