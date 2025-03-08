+++
layout = "post"
title = "What the Error Handling Project Group is Working Towards"
author = "Jane Lusby"
team = "the library team <https://www.rust-lang.org/governance/teams/library>"
+++

This blog post is a follow up of our [previous](https://blog.rust-lang.org/inside-rust/2020/11/23/What-the-error-handling-project-group-is-working-on.html) post detailing what we're working on now. We've been iterating for a while now on some of the challenges that we see with error handling today and have reached the point where we want to describe some of the new changes we're working towards. But first we need to describe the main challenges we've identified.

> Disclaimer: *This post is equal parts plan and aspiration. There are technical challenges here to sort out so the final outcome may look rather different from our initial vision, so please don't assume any of this is final.*

## Error Handling Today

The first challenge we'd like to solve is that it's easy to lose context accidentally when reporting errors. There are a couple of places this can happen, either when printing an error and forgetting to print sources, when returning an error from main, or when converting a recoverable error into a non recoverable error.

Consider this example:

```rust
use std::fmt;

// We have a program that loads a config and expects that
// loading the config will always succeed.
fn main() {
    let _config = load_config()
        .expect("config is always valid and exists");
}

// We have a dummy implementation of load_config which
// always errors, since we're just focusing on diagnostics
// here.
fn load_config() -> Result<(), Error> {
    Err(Error(SourceError))
}

// And we have an error type that just prints "invalid
// config" and has a source error which just prints "config
// file does not exist"
#[derive(Debug)]
struct Error(SourceError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid config")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
struct SourceError;

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("config file does not exist")
    }
}

impl std::error::Error for SourceError {}
```

When we run this we would like to see output somewhat like this:

```
$ cargo run
thread 'main' panicked at 'config is always valid and exists', src/main.rs:4:33

Error:
    0: invalid config
    1: config file does not exist

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this error message we can see that we exited because of a panic. We can see what invariant we violated that was supposed to prevent this panic from happening. We can see the location where the panic was produced. And we can see the error message of each error in the chain of errors accessible via `source`.

That's what we would like, at least in the version of Rust that the error handling project group wants to see, but what we actually get is this...

```
$ cargo run
thread 'main' panicked at 'config is always valid and exists: Error(SourceError)', main.rs:4:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Now, I definitely don't think this is what we want as a default when promoting recoverable errors to non-recoverable errors! `unwrap` and `expect` work by stringifying the error variant using its `Debug` impl, but this is often the wrong operation for types that implement the `Error` trait. By converting the `Error` to a `String` we lose access to the pieces of context we carefully split up via the `Error` trait, and in all likelihood the `derive(Debug)` output of our error types won't even include the error messages in our `Display` impls.

Rust's panic infrastructure doesn't provide a method for converting an `Error` type into a panic, it only supports converting `Debug` types into panics, and we feel that this is a major issue. Similarly, there's no convenient tools provided by the language for printing an error and all of its source's error messages.

```rust
fn main() {
    let result = load_config();
    let _config = match result {
        Ok(config) => config,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
}
```

When we run this program we'd like to see output that looks something like this:

```
$ cargo run
Error: invalid config: config file does not exist
```

Here we can see the header we provided to indicate we're printing an error, followed by each error message in the chain of sources separated by colons.

But instead all we get is this:

```
$ cargo run
Error: invalid config
```

By default all of the source's error messages are lost. This arises from the fact that we used `Display` as the interface to an individual error message. If we could go back we'd currently propose instead adding `fn message(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result` to the `Error` trait, but that ship has sailed.

The way that libraries work around this today is by abusing the `Debug` trait. Types like [`eyre`](https://docs.rs/eyre/0.6.5/eyre/trait.EyreHandler.html#tymethod.debug), [`anyhow`](https://docs.rs/anyhow/1.0.40/src/anyhow/fmt.rs.html#19), and even sometimes [`custom error enums`](https://www.lpalmieri.com/posts/error-handling-rust/#error_chain_fmt) use their `Debug` output to print the full chain of errors in a human readable report.

This has the advantage of making it easy to print a full error report and makes it so `unwrap`, `expect`, and return from main all print the full error report. But doing so prevents us from accessing the derived `Debug` format of our errors, potentially hiding internal details that might be needed for debugging but which aren't part of the error messages intended for users to read.

## Error Handling Tomorrow

Eventually we'd like to get to a place where the default tools you reach for when error handling in Rust all do the right thing and fully leverage the `Error` trait's design. Unwrapping a type that implements the `Error` trait will preserve the original error as a `dyn Error` which is then available in the panic hook. Printing a full error report will be easy to do and obvious. With these changes in place it will hopefully be quite difficult to accidentally discard information when reporting errors.

Our plan to fix these issues is two-fold:

### 1. Error Trait + Panic Runtime Integration

First we need to integrate the Error trait and the panic runtime, and the first step to doing so will be moving the `Error` trait into `core`. This is necessary because the panic runtime is part of `core` and the language itself, where as the `Error` trait currently resides in `std`. We're pretty excited about this change which we hope will have other positive downstream effects, particularly in the embedded ecosystem.

Once we've gotten to the point where the `Error` trait is usable in `core` APIs the next step will be to add an interface for creating a panic from an `Error` type. We're currently planning on adding a `panic_error` function, similar to the `panic_any` function that is already available in `std`. This function will give the panic handler access to errors via a `dyn Error`.

Once panic handlers are able to process `Error` types the next step will be to update the default panic hook provided by `std` to actually report panics via the `Error` trait if they're exposed as such. It should iterate over sources and print the backtrace captured by the error itself if one is available, or possibly capture one itself otherwise.

Finally, we need to specialize `expect` and `unwrap` to use these new `Error` aware panic interfaces when unwrapping types that implement the `Error` trait. To do this we first need to work around a [soundness issue](https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls/#the-soundness-problem) with specialization for trait impls that are conditional based on lifetimes, though thankfully we already have a good idea of [how to fix this](https://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls).

### 2. Error Reporter

We would also like to provide a basic error reporter in `std`, and some facilities for making it easy to use, or easy to replace with your own preferred error reporter. Printing an error and its sources is a fundamental operation in Rust, so we want the language to provide a pit of success for reporting, where the easiest thing to do is the right thing. We can't get there completely because we use `Display` for individual error messages, and we can't change that in a backwards compatible fashion, but we hope that adding a convenient method for printing a full chain of errors and some clever lints will relieve most of the pressure.

We plan on fixing this by first adding a `Report` type to the standard library that wraps a `&dyn Error` and implements `Display` such that it prints each source as desired. We would like the output of `Report`'s display method to support the styles of error concatenation that are most common in the Rust ecosystem.

Either one line with each error message concatenated with colons:

```rust
println!("Error: {}", Report::from(error));

// Outputs:
// Error: outermost error: second error: root error
```

Or multiple lines with each error message on its own line :


```rust
println!("Error: {:#}", Report::from(error))

// Outputs:
// Error: outermost error
//
// Caused by:
//    0: second error
//    1: root error
```

The first single line format is useful for log output or inlined error messages, whereas the alternate multi line format is useful for user facing output such as a CLI interface or a GUI popup.

We also want to add a method to the error trait for conveniently wrapping up any error in the `Report` type so that reporting an error is as simple as `println!("Error: {}", error.report());`

We expect the report method will look something like this:

```rust
fn report(&self) -> impl Display + '_
where
    Self: Sized,
{
    Report::from(self)
}
```

We want the return type here to be generic rather than hard coded to `Report` so that individual error types can provide their own report format if desired. We expect that derive macros may leverage this to customize error reporting format defaults. This will work well with composition because the reporter from the outermost type will be used to format the full chain of errors.

For now we can't implement this method as described because `impl Trait` isn't allowed in return types on trait methods, but we're working to find a way to add this to the error trait backwards compatibly.

## Duplicate Information Issue

With these fixes in place it will become easy to chain errors and report them completely and consistently. However there is a hazard that `Error` implementors need to be aware of in this system: duplicate information.

Imagine an error like the one in the previous example, except instead of each error printing its own message and returning the next error via `source`, they also include their source's error message after their own. That way when we print the outer error's `Display` output we see all of the error messages, not just the first in the chain.

```rust
println!("Error: {}", error);

// Outputs:
// Error: outermost error: second error: root error
```

Now, what happens we then print this same error type with `Report` expecting that we need to iterate over the sources and print them too?

```rust
println!("Error: {:#}", error.report());

// Outputs
// Error: outermost error: second error: root error
//
// Caused by:
//    0: second error: root error
//    1: root error
```

The source error messages get duplicated! With the multi-line output of `anyhow` and `eyre` we get this nice little triangle shape to the error report, which you've probably encountered if you've ever used these libraries before. We can no longer separate the error messages of the individual errors in the chain of errors because this error type concatenates the sources manually and returns them via the `source` function. This also restricts how we can format our error reports. If we want a consistent report format and we have a dependency that concatenates errors in a single line we are forced to do so as well ourselves throughout our entire application. If, on the other hand, we have two dependencies that concatenate errors in different ways, well, we're out of luck.

So how do we avoid this? We adopt a consistent separation for `Display` and `source` implementations.

## Guidelines for implementing `Display::fmt` and `Error::source`

To resolve this issue, project error handling recently created a guideline for [how to implement `Display::fmt` and `Error::source`](https://github.com/rust-lang/project-error-handling/issues/27#issuecomment-763950178). In it we make the following recommendation:

**An error type with a source error should either return that error via `source` or include that source's error message in its own `Display` output, but never both.**

We figure the default will be to return errors via source. That way source errors can be reacted to via `downcast` when appropriate. This is particularly important for libraries that are changing existing public error types. For these libraries removing an error from `source` is a breaking change that isn't detected at compile time, making a major version bump likely insufficient. Changing the `Display` output is also a breaking change, though a less dangerous one. To help with this we've drafted a suggested migration plan: [rust-lang/project-error-handling#44](https://github.com/rust-lang/project-error-handling/issues/44).

In coming up with this recommendation we had to figure out what the `Error` trait's primary role is in Rust. After discussing it with the library team we concluded that reporting should be treated as the primary role, and that reacting via `downcast` should come second when designing error types. Generally these needs are not in conflict, but it is possible for issues to come up. For example, when working with transparent error types that forward all methods to an inner error type. When these types follow this guideline the inner error type is skipped over and is never made available for `downcast`ing.

This recommendation only applies for error types that are exposed as part of library APIs. Internal errors in libraries or and applications can do whatever they want, but as soon as they need to be integrated into other crates by 3rd party users it's important that errors follow a consistent style. If you're interested in our rationale or have any comments please check out our github issue on the topic: [rust-lang/project-error-handling#27](https://github.com/rust-lang/project-error-handling/issues/27).

## Conclusion

We hope that these changes will significantly improve error handling experience provided by Rust. Error reporting will be more consistent and flexible and let the final application developer define how to format error reports for their specific use-case. It will be a lot harder to accidentally lose information when reporting errors. The tools for reporting errors will be more tightly integrated into the standard library and the language itself and we hope this will have extra benefits to the embedded ecosystem by more universally standardizing on the `Error` trait.

So that's the plan for now, it's not the full plan of all the changes we'd like to make, but we think its the best first step. However, this is by no means set in stone and we're interested in getting feedback from the rest of the community so we can refine our design. So if you have thoughts please let us know, our project group repo is https://github.com/rust-lang/project-error-handling. Please feel free to [open an issue](https://github.com/rust-lang/project-error-handling/issues) or hop in our [zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/257204-project-error-handling) and create a new topic to let us know what you think of this plan.
