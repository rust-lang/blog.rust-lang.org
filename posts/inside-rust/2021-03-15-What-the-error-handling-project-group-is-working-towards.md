---
layout: post
title: "What the Error Handling Project Group is Working Towards"
author: Jane Lusby
team: The Error Handling Project Group <https://www.rust-lang.org/governance/teams/library#project-error-handling>
---

This blog post is a follow up of our [previous](https://blog.rust-lang.org/inside-rust/2020/11/23/What-the-error-handling-project-group-is-working-on.html) post detailing what we're working on now. We figure it's time to take a step back and answer some of the bigger questions of why we're making these changes and how they're going to fit into the bigger picture of error-handling provided by Rust.

> Disclaimer: *This post is equal parts plan and aspiration. There are technical challenges here to sort out so the final outcome may look rather different from our initial vision, so please don't assume any of this is final.*

## Guidelines for implementing `Display::fmt` and `Error::source`

But before we get into the plans we first need to share an announcement. Project Error Handling recently created a guideline for [how to implement `Display::fmt` and `Error::source`](https://github.com/rust-lang/project-error-handling/issues/27#issuecomment-763950178). To create this guideline we first had to answer a fundamental question, _What is the primary role of the error trait?_.

As we saw it there are two possible answers.

* The error trait is primarily an interface for reporting errors.
* The error trait is primarily an interface for reacting to specific errors.

In the first case, "an interface for reporting errors" means it provides access to pieces of information about the error such as the error message, optional backtrace, optional source, and so on. A reporter then takes all this information and stitches it together into a full error report, explaining what went wrong.

If we decide to prefer the former viewpoint then we would want to prioritize not duplicating information in the reporting interfaces of the `Error` trait. That would then turn into the guidance: _Return source errors via `Error::source` unless the source error's message is included in your own error message in `Display`._

In the second case, "an interface for reacting to specific errors" is referring to the various `downcast` methods associated with the error trait. These methods can be used to test if a type erased error is a specific error variant and they're functionally equivalent to `match`.

If we decide to prefer the latter then we would want to prioritize exposing every error type in the chain of errors to ensure users can always correctly downcast to specific error variants they want to handle. In this situation the guidance would be: _Always return source errors via `Error::source`._

After some discussion we concluded that the reporting focused viewpoint was the ideal one. We prefer splitting our errors into multiple error messages with no duplication of information as opposed to having each error print itself followed by all others. This split moves the responsibility of formatting to the end user, rather than relying on many library authors concatenating error messages together with a consistent format.

This gives us more flexibility and consistency in how we render error reports, and it doesn't prevent users from structuring their error types so all sources are still correctly exposed for downcasting. If we prioritize downcasting however we have no way of untangling the error messages attached to each source.

With all of that said however, we're not in a great position to make this guidance which brings us to the first major problem we're planning on fixing.

## Problem #1: The Missing Reporter

The error `Trait` as described above is only half of a complete error reporting solution. The existence of an interface for reporting implies the existance of a reporter who uses that interface and outputs a fully formatted error report. The problem is, we don't provide any sort of reporter for actually formatting error reports. The expectation is you have to bring your own reporter, either something like `eyre::Report` or a loop printing each error message.

### Solution: Add a new `{:!}` error reporting fmt specifier

Our currently preferred solution to this problem is to build some simple reporting logic into the language's `fmt` grammar. `{:!}` would report types via the `Error` trait rather than via `Display` or `Debug`, and would implicitly iterate over sources and print them all sequentially.

We want to use `fmt::Formatter` flags to customize the output. Calling `println!("Error: {:!}", error);` would output something like this:

```
Error: error message: source's error message: source's source's ...
```

Whereas calling `println!("Error: {:#!}", error);` could print something like this (assuming `error` captured a `Backtrace`):

```
Error:
   0: error message
   1: source's error message
   2: source's source's ...

Backtrace:
   ...
```

This should provide reasonable defaults for simple error reporting cases that are encountered most of the time. Applications that need a more complex error report format can then provide their own reporter by wrapping errors in a `Display` type the same way the `Path::display` method does for paths.

## Panics cannot be made from `dyn Error`s, and instead stringifies the error

The next problem we'd like to solve is a lack of integration between recoverable and non-recoverable errors. Consider this trivial program:

```rust
struct TypeThatILike;

#[derive(Debug, thiserror::Error)]
#[error("My error's extremely important error message, do not drop this!")]
struct MyError;

fn doomed_to_fail() -> Result<TypeThatILike, MyError> {
    Err(MyError)
}

fn main() {
    // If this fails I want to panic immediately, there's no point in continuing
    // without my favorite type
    let my_extremely_important_type = doomed_to_fail().unwrap();
}
```

If I didn't know much about how rust error handling work's this is approximately what I'd hope to see in this situation:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err`', src/main.rs:14:56

Error:
    0: My error's extremely important error message, do not drop this!

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

And here's what we actually get...

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: MyError', src/main.rs:14:56
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

I definitely don't think this is what we want as a default when promoting recoverable errors to non-recoverable errors... `unwrap` and `expect` work by stringifying the error variant using it's `Debug` impl, but this is entirely the wrong operation for `Error` types! By converting the `Error` to a `String` we lose access to the pieces of context we carefully split up via the `Error` trait, in all likelyhood the `derive(Debug)` output of our error types won't even include the error messages in our `Display` impls.

So we need some way to create panics _from_ `Error` types that doesn't hide the `Error` interface. We'd like it if `PanicInfo` could store a `dyn Error + 'static` instead of, or in addition to the `dyn Any + 'static` payload it currently carries. We're thinking about adding a `std::panic::panic_error` function similar to `std::panic::panic_any`. Then, if possible, we'd like to specialize the `unwrap` and `expect` functions on `Result` to call `panic_error` instead of `panic!` when `E` implements `Error`.

We're not sure this is possible but either way we will figure out some way to integrate the two error handling systems more smoothly, though first we have to move the `Error` trait into `core`. :eyes:

## Types like `Box<dyn Error + ...>` cannot implement `Error`

The final problem is the incomplete `Error` implementation we're currently locked into on `Box`. The issue here is probably a surprise to most people based on personal experience, so let me first provide some background.

`Box` has the following implementation for the `Error` trait:

```rust
impl<T: Error> Error for Box<T> {
    /// irrelevant impl details ...
}
```

This impl looks pretty innocent, but as it is written it currently _doesn't_ apply to `Box<dyn Error>`, the proper impl would need the following change:

```diff
- impl<T: Error> Error for Box<T> {
+ impl<T: Error + ?Sized> Error for Box<T> {
```

This means that as long as the `T` type of the box is _Sized_ it will implement `Error`. If `MyError` is an error then so is `Box<MyError>`, but if we convert it to a `Box<dyn Error + ...>` it is no longer an error!

So why can't we just apply this diff and fix the `Error` impl? Here are the culprits:

```rust
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a> {
    fn from(err: E) -> Box<dyn Error + 'a> {
        Box::new(err)
    }
}

impl<T> From<T> for T {
    fn from(t: T) -> T {
        t
    }
}
```

These `From` impls interfere with our `Error` impl due to the "Overlap Rule", which states that no two generic implementations of a Trait can apply to the same type. If `Box<dyn Error>` implements the `Error` trait both of the `From` impls above would apply, and `BoxError::from(other_box_error)` becomes ambiguous. The compiler doesn't know whether it should pass the Box through directly or wrap it in another `Box`.

In theory we could fix this in the future if we ever get specialization finished, but as far as we know its highly unlikely that it would be useful in this specific case. We'd need to support [lattice specialization](https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md#the-lattice-rule) which is, by our most optimistic understanding, very far from happening if ever.

### Hope, AKA `Try` trait v2 RFC

But fear not! All is not lost. It turns out, with the [new `Try` trait RFC](https://github.com/scottmcm/rfcs/blob/do-or-do-not/text/0000-try-trait-v2.md#summary) we can solve this problem. Probably not for `Box<dyn Error>` itself, but for the rest of the ecosystem. The way this works is by moving the `From` impl to be a property of the `Result` type being used, rather than the `Error` type it is abstracting over.

If you're interested in more details we've created a gist of the proof of concept [here](https://gist.github.com/yaahc/c9348cca4aa34ba8bc92f41ba543919d).

The main difference is that we hard code the `From` impl into the new `Result` type, rather than having it be inherited from the wrapped error types the way `std::result::Result` does.

So instead of this:

```rust
impl<T, E, F> FromResidual<Result<!, E>> for Result<T, F>
where
    F: From<E> { ... }
```

We can write this:

```rust
impl<T, E> FromResidual<Result<!, E>> for MyResult<T>
where
    BoxError: From<E> { ... }
```

That way the error type associated with the custom result doesn't need to implement `From`, freeing it from the overlap rule and letting it implement `Error` instead.

Assuming this works as well as we hope it does, and doesn't introduce worse problems than we're trying to fix, the plan is to then standardize this approach to all catch-all error types across the ecosystem. Our current vague plan has three parts. Add a new `Box<dyn Error>` equivalent to std that implement's `Error` and doesn't implement `From`, add a new `Result` type to `std` that doesn't have a generic parameter for `E`, and instead always converts to `YeetError`, or whatever it's called, whenever converting from a `std::result::Result`, and finally add lints to either `rustc` or `clippy` that warn against using `Box<dyn Error>`, and encourage people to switch to the new fixed equivalent type.


### Disclaimers

Note, we almost certainly won't _start_ with adding these types to std, and will instead trial this approach in 3rd party libraries, probably in a fork of `eyre` or `anyhow`. We're particularly worried about backwards compatibility so its easy to switch to the new libraries that properly implement `Error`. We're also worried about teachability, particularly with respect to knowing which `Result` type to use in each situation, and we're looking into what kind of errors get produced when you pick the wrong Error or Result type, and how we can guide users to the correct types.

Also worth noting, this doesn't change how `std::result::Result` works. If you're happy with enum based error handling and `From` impls as necessary then you can still stick with the current `Result` type and `?` as you currently do. These changes only apply to types like `Box<dyn Error>` that implement From for all error types so they can still implement error while using `?` for convenience.

## Summary

To summarize on the above our plan is to:

1. Move the error trait into core
2. Integrate the Error trait with the panic runtime so that errors can be smoothly upgraded into panics without losing information
    * Add a `panic_error` function for creating a panic from an `Error` type
    * Expose `dyn Error` as part of `PanicInfo`'s API, similar to how you can extract the payload as a `dyn Any`
    * Update the default panic hook in `std` to iterate over source errors and prints captured backtraces when reporting errors, if the panic came from a `dyn Error`
    * specialize `unwrap`/`expect` on result to call `panic_error` instead of `panic!`
3. Add basic reporting logic for printing via the `Error` trait including source errors, possibly by integrating it directly with the `fmt` grammar
4. Support the stabilization of the `Try` trait as much as possible
5. Add an alternative to `Box<dyn Error>` to std that implements the `Error` trait
6. Add a corresponding `Result` that makes it so you can use `?` to convert any error into the new dyn error type
7. Add lints to discourage direct use of `Box<dyn Error>` in the future and nudge people towards a the `Try` based convenient propagation approach

This isn't everything we're working on, but these are our current priorities. Our hope is that with all of these changes applied there will be significantly fewer sharp edges when working with error handling.
