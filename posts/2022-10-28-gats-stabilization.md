---
layout: post
title: Generic associated types to be stable in Rust 1.65
author: Jack Huey
description: "Generic associated types will stabilize in Rust 1.65"
team: The Types Team <https://github.com/rust-lang/types-team>
---

As of Rust 1.65, which is set to release on November 3rd, generic associated types (GATs) will be stable — over six and a half years after the original [RFC] was opened. This is truly a monumental achievement; however, as with a few of the other monumental features of Rust, like `async` or const generics, there are limitations in the initial stabilization that we plan to remove in the future.

The goal of this post is not to teach about GATs, but rather to briefly introduce them to any readers that might not know what they are and to enumerate a few of the limitations in initial stabilization that users are most likely to run into. More detailed information can be found in the [RFC], in the [GATs initiative repository][initiative_repo], in the previous [blog post][stabilization_post] during the start of the stabilization push, in the [associated items section in the nightly reference][reference], or in the [open issues on Github for GATs][gats_issues]

## What are GATs

At its core, generic associated types allow you to have *generics* (type, lifetime, or const) on *associated types*. Note that this is really just rounding out the places where you can put generics: for example, you can already have generics on freestanding type aliases and on functions in traits. Now you can just have generics on type aliases in traits (which we just call associated types). Here's an example of what a trait with a GAT would look like:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Self::Item<'a>;
}
```

Most of this should look familiar; this trait looks *very* similar to the [`Iterator`][Iterator] trait from the standard library. Fundamentally, this version of the trait allows the `next` function to return an item that *borrows* from `self`. For more detail about the example, as well as some info on what that `where Self: 'a` is for, check out the [push for stabilization post][stabilization_post].

In general, GATs provide a foundational basis for a vast range of patterns and APIs. If you really want to get a feel for how many projects have been blocked on GATs being stable, go scroll through either the [tracking issue]: you will find numerous issues from other projects linking to those threads over the years saying something along the lines of "we want the API to look like X, but for that we need GATs" (or see [this comment](https://github.com/rust-lang/rust/pull/96709#issuecomment-1173170243) that has some of these put together already). If you're interested in how GATs enable a library to do zero-copy parsing, resulting in nearly a ten-fold performance increase, you might be interested in checking out a [blog post][chumsky_blog_post] on it by Niko Matsakis.

All in all, even if *you* won't need to use GATs directly, it's very possible that the *libraries* you use will use GATs either internally or publically for ergonomics, performance, or just because that's the only way the implementation works.

## When GATs go wrong - a few current bugs and limitations

As alluded to before, this stabilization is not without its bugs and limitations. This is not atypical compared to prior large language features. We plan to fix these bugs and remove these limitations as part of ongoing efforts driven by the newly-formed types team. (Stayed tuned for more details in an official announcement soon!)

Here, we'll go over just a couple of the limitations that we've identified that users might run into.

### Implied `'static` requirement from higher-ranked trait bounds

Consider the following code:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;
}

pub struct WindowsMut<'x, T> {
    slice: &'x [T],
}

impl<'x, T> LendingIterator for WindowsMut<'x, T> {
    type Item<'a> = &'a mut [T] where Self: 'a;
}

fn print_items<I>(iter: I)
where
    I: LendingIterator,
    for<'a> I::Item<'a>: Debug,
{ ... }

fn main() {
    let mut array = [0; 16];
    let slice = &mut array;
    let windows = WindowsMut { slice };
    print_items::<WindowsMut<'_, usize>>(windows);
}
```

Here, imagine we wanted to have a `LendingIterator` where the items are overlapping slices of an array. We also have a function `print_items` that prints every item of a `LendingIterator`, as long as they implement `Debug`. This all seems innocent enough, but the above code doesn't compile — even though it should. Without going into details here, the `for<'a> I::Item<'a>: Debug` currently implies that `I::Item<'a>` must outlive `'static`.

This is not really a nice bug. And of all the ones we'll mention today, this will likely be the one that is most limiting, annoying, and tough to figure out. This pops up much more often with GATs, but can be found with code that doesn't use GATs at all. Unfortunately, fixing this requires some refactorings to the compiler that isn't a short-term project. It is on the horizon though. The good news is that, in the meantime, we are working on improving the error message you get from this code. This is what it will look like in the upcoming stabilization:

```rust
error[E0597]: `array` does not live long enough
   |
   |     let slice = &mut array;
   |                 ^^^^^^^^^^ borrowed value does not live long enough
   |     let windows = WindowsMut { slice };
   |     print_items::<WindowsMut<'_, usize>>(windows);
   |     -------------------------------------------- argument requires that `array` is borrowed for `'static`
   | }
   | - `array` dropped here while still borrowed
   |
note: due to current limitations in the borrow checker, this implies a `'static` lifetime
   |
   |     for<'a> I::Item<'a>: Debug,
   |                          ^^^^
```

It's not perfect, but it's something. It might not cover all cases, but if have a `for<'a> I::Item<'a>: Trait` bound somewhere and get an error that says something doesn't live long enough, you might be running into this bug. We're actively working to fix this. However, this error doesn't actually come up as often as you might expect while reading this (from our experience), so we feel the feature is still immensely useful enough even with it around.

### Traits with GATs are not object safe

So, this one is a simple one. Making traits with GATs object safe is going to take a little bit of design work for its implementation. To get an idea of the work left to do here, let's start with a bit of code that you could write on stable today:

```rust
fn takes_iter(_: &dyn Iterator) {}
```

Well, you can write this, but it doesn't compile:

```rust
error[E0191]: the value of the associated type `Item` (from trait `Iterator`) must be specified
 --> src/lib.rs:1:23
  |
1 | fn takes_iter(_: &dyn Iterator) {}
  |                       ^^^^^^^^ help: specify the associated type: `Iterator<Item = Type>`
```

For a trait object to be well-formed, it must specify a value for all associated types. For the same reason, we don't want to accept the following:

```rust
fn no_associated_type(_: &dyn LendingIterator) {}
```

However, GATs introduce an extra bit of complexity. Take this code:

```rust
fn not_fully_generic(_: &dyn LendingIterator<Item<'static> = &'static str>) {}
```

So, we've specified the value of the associated type for *one* value of of the `Item`'s lifetime (`'static`), but not for *any* value, like this:

```rust
fn fully_generic(_: &dyn for<'a> LendingIterator<Item<'a> = &'a str>) {}
```

While we have a solid idea of how to implement requirement in some *future* iterations of the trait solver (that uses more logical formulations), implementing it in the current trait solver is more difficult. Thus, we've chosen to hold off on this for now.

### The borrow checker isn't perfect and it shows

Keeping with the `LendingIterator` example, let's start by looking at two methods on `Iterator`: `for_each` and `filter`:

```rust
trait Iterator {
    type Item;

    fn for_each<F>(self, f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item);
    
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool;
}
```

Both of these take a function as an argument. Closures are often used these. Now, let's look at the `LendingIterator` definitions:

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>);

    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item<'_>) -> bool;
}
```

Looks simple enough, but if it really was, would it be here? Let's start by looking at what happens when we try to use `for_each`:

```rust
fn iterate<T, I: for<'a> LendingIterator<Item<'a> = &'a T>>(iter: I) {
    iter.for_each(|_: &T| {})
}
```

```rust
error: `I` does not live long enough
   |
   |     iter.for_each(|_: &T| {})
   |                   ^^^^^^^^^^
```

Well, that isn't great. Turns out, this is pretty closely related to the first limitation we talked about earlier, even
though the borrow checker does play a role here.

On the other hand, let's look at something that's very clearly a borrow checker problem, by looking at an implementation
of the `Filter` struct returned by the `filter` method:

```rust
impl<I: LendingIterator, P> LendingIterator for Filter<I, P>
where
    P: FnMut(&I::Item<'_>) -> bool, // <- the bound from above, a function
{
    type Item<'a> = I::Item<'a> where Self: 'a; // <- Use the underlying type

    fn next(&mut self) -> Option<I::Item<'_>> {
        // Loop through each item in the underlying `LendingIterator`...
        while let Some(item) = self.iter.next() {
            // ...check if the predicate holds for the item...
            if (self.predicate)(&item) {
                // ...and return it if it does
                return Some(item);
            }
        }
        // Return `None` when we're out of items
        return None;
    }
}
```

Again, the implementation here shouldn't seem surprising. We, of course, run into a borrow checker error:

```rust
error[E0499]: cannot borrow `self.iter` as mutable more than once at a time
  --> src/main.rs:28:32
   |
27 |     fn next(&mut self) -> Option<I::Item<'_>> {
   |             - let's call the lifetime of this reference `'1`
28 |         while let Some(item) = self.iter.next() {
   |                                ^^^^^^^^^^^^^^^^ `self.iter` was mutably borrowed here in the previous iteration of the loop
29 |             if (self.predicate)(&item) {
30 |                 return Some(item);
   |                        ---------- returning this value requires that `self.iter` is borrowed for `'1`
```

This is a known limitation in the current borrow checker and should be solved in some future iteration (like [Polonius](https://github.com/rust-lang/polonius)).

### Non-local requirements for where clauses on GATs

The last limitation we'll talk about today is a bit different than the others; it's not a bug and it shouldn't prevent any programs from compiling. But it all comes back to that `where Self: 'a` clause you've seen in several parts of this post. As mentioned before, if you're interested in digging a bit into why that clause is required, see the [push for stabilization post][stabilization_post].

There is one not-so-ideal requirement about this clause: you must write it on the trait. Like with where clauses on functions, you cannot add clauses to associated types in impls that aren't there in the trait. However, if you *didn't* add this clause, a large set of potential impls of the trait would be disallowed.

To help users not fall into the pitfall of accidentally forgetting to add this (or similar clauses that end up with the same effect for a different set of generics), we've implemented a set of rules that must be followed for a trait with GATs to compile. Let's first look at the error without writing the clause:

```rust
trait LendingIterator {
    type Item<'a>;

    fn next<'a>(&'a mut self) -> Self::Item<'a>;
}
```

```rust
error: missing required bound on `Item`
 --> src/lib.rs:2:5
  |
2 |     type Item<'a>;
  |     ^^^^^^^^^^^^^-
  |                  |
  |                  help: add the required where clause: `where Self: 'a`
  |
  = note: this bound is currently required to ensure that impls have maximum flexibility
  = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
```

This error should hopefully be helpful (you can even `cargo fix` it!). But, what exactly are these rules? Well, ultimately, they end up being somewhat simple: *for methods that use the GAT, any bounds that can be proven must also be present on the GAT itself*.

Okay, so how did we end up with the required `Self: 'a` bound. Well, let's take a look at the `next` method. It returns `Self::Item<'a>`, and we have an argument `&'a mut self`. We're getting a bit into the details of the Rust language, but because of that argument, we know that `Self: 'a` must hold. So, we require that bound.

We're requiring these bounds now to leave room in the future to potentially imply these automatically (and of course because it should help users write traits with GATs). They shouldn't interfere with any real use-cases, but if you do encounter a problem, check out the issue mentioned in the error above. And if you want to see a fairly comprehensive testing of different scenarios on what bounds are required and when, check out the relevant [test file](https://github.com/rust-lang/rust/blob/f2702e922ba31e49d6167f5651d4545646dcf22d/src/test/ui/generic-associated-types/self-outlives-lint.rs).

## Conclusion

Hopefully the limitations brought up here and explanations thereof don't detract from overall excitement of GATs stabilization. Sure, these limitations do, well, *limit* the number of things you can do with GATs. *However*, we would not be stabilizing GATs if we didn't feel that GATs are not still *very* useful. Additionally, we wouldn't be stabilizing GATs if we didn't feel that the limitations weren't solvable (and in a backwards-compatible manner).

To conclude things, all the various people involved in getting this stabilization to happen deserve the utmost thanks. As said before, it's been 6.5 years coming and it couldn't have happened without everyone's support and dedication. Thanks all!


[RFC]: https://github.com/rust-lang/rfcs/pull/1598
[initiative_repo]: https://rust-lang.github.io/generic-associated-types-initiative/index.html
[stabilization_post]: https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html
[reference]: https://doc.rust-lang.org/nightly/reference/items/associated-items.html
[gats_issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AF-generic_associated_types
[Iterator]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
[tracking issue]: https://github.com/rust-lang/rust/issues/44265
[stabilization pull request]: https://github.com/rust-lang/rust/pull/96709
[chumsky_blog_post]: https://smallcultfollowing.com/babysteps/blog/2022/06/27/many-modes-a-gats-pattern/
