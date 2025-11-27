+++
path = "2025/11/27/warning-about-fn-pointer-casts"
title = "New warning: casting function pointer"
authors = ["Guillaume Gomez"]
+++

# New warning: casting function pointer

We recently added a new warning in the rust compiler when a function pointer is cast as a pointer in [#141470](https://github.com/rust-lang/rust/pull/141470). Let's dive into the reasons why we added it and what we plan to do next.

## Why?

In rust, it's possible to do:

```rust
let x = u16::max as usize;
```

Which is fine, except that `u16::max` is a function, not a constant (the constant is named `u16::MAX`). And it can become catastrophic in cases like:

```rust
let x = [0; u16::max as usize];
```

Or:

```rust
if x.len() > u16::max as usize {
    // ...
}
```

We took the second example in particular as it was the root issue of [CVE-2025-1014](https://www.cve.org/CVERecord?id=CVE-2025-1014), which happened in the Firefox web browser (reported [here](https://bugzilla.mozilla.org/show_bug.cgi?id=1940804)).

## Suggested fix

Now let's take a look at what the suggested fix looks like:

```rust
let x = u16::max as usize;
```

will suggest:

```text
warning: direct cast of function item into an integer
 --> src/main.rs:3:18
  |
3 | let x = u16::max as usize;
  |                  ^^^^^^^^
  |
  = note: `#[warn(function_casts_as_integer)]` on by default
help: first cast to a pointer `as *const ()`
  |
3 | let x = u16::max as *const () as usize;
  |                  ++++++++++++
```

So the "fixed" code would look like this:

```rust
let x = u16::max as *const () as usize;
```

Having the `as * const ()` part to be present in the code will force code readers to wonder why such a cast is there for what appears to be an integer cast. In short: to make it stand out so it doesn't go unnoticed.

However, we're not completely satisfied with this because, although it attract readers attention on this part of the code, it's not really clear what's happening just from reading it. An alternative was to force having the function declaration in the cast:

```rust
let x = u16::max as *const fn() -> u16 as usize;
```

However, we are planning to have another approach in the future to cast function pointers to integers so until ten, we decided to keep the simpler approach.

## What's coming next

We're planning to create two new traits: `FnPtr` and `FnStatic`. The one we're interested into here is `FnPtr` because it would add `as_ptr`, directly callable on functions/methods. So the previous suggested code would become:

```rust
let x = u16::max.as_ptr() as usize;
```

So once this new trait has been implemented and stabilized, this is what the warning will suggest.

You can follow progress of these new traits implementations by taking a look at its [tracking issue](https://github.com/rust-lang/rust/issues/148768).

## A bit of history

Because it's always interesting how something comes to be, we added this section.

It all started from a `clippy` lint named `confusing_method_to_numeric_cast` (implemented in [this PR](https://github.com/rust-lang/rust-clippy/pull/13979)) which checked special cases like:

```rust
let _ = u16::max as usize;
```

It was suggesting the constant equivalent. However, when we realizes that it might need to grow a lot when we added cases like `integer::max_value` or `integer::maximum`, we decided that having something more general directly in the rust compiler might be preferable.
