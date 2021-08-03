---
layout: post
title: "The push for GATs stabilization"
author: Jack Huey
team: the Traits Working Group <https://www.rust-lang.org/governance/teams/compiler#wg-traits>
---

# The push for GATs stabilization

Where to start, where to start...

Let's begin by saying: this is a very exciting post. Some people reading this will be overwhelmingly thrilled; some will have no idea what GATs (generic associated types) are; others might be in disbelief. The [RFC] for this feature did get opened in April of *2016* (and merged about a year and a half later). In fact, this RFC even predates const generics (which an MVP of was [recently stabilized][min_const_generics]). Don't let this fool you though: it is a powerful feature; and the reactions to the tracking issue on Github should maybe give you an idea of its popularity (it is *the* most upvoted issue on the Rust repository):
![GATs reactions](/images/2021-08-03-GATs-stabilization-push/gats-reactions.png)

If you're not familiar with GATs, they allow you to define type, lifetime, or const generics on associated types. Like so:

```rust
trait Foo {
    type Bar<'a>;
}
```

Now, this may seem underwhelming, but I'll go into more detail later as to *why* this really is a powerful feature.

But for now: what exactly *is* happening? Well, nearly four years after its RFC was merged, the `generic_associated_types` feature is no longer "incomplete."

*crickets chirping*

Wait...that's it?? Well, yes! I'll go into a bit of detail later in this blog post as to why this *is* a big deal. But, long story short, there have been a good amount of changes that have had to have been made to the compiler to get GATs to work. And, while there are still a few small remaining diagnostics issues, the feature is finally in a space that we feel comfortable making it no longer "incomplete".

So, what does that mean? Well, all it *really* means is that when you use this feature on nightly, you'll no longer get the "`generic_associated_types` is incomplete" warning. However, the real reason this is a big deal: we want to stabilize this feature. But we need your help. We need you to test this feature, to file issues for any bugs you find or for potential diagnostic improvements. Also, we'd love for you to just tell us about some interesting patterns that GATs enable over on [Zulip]!

Without making promises that we aren't 100% sure we can keep, we have high hopes we can stabilize this feature within the next couple months. But, we want to make sure we aren't missing glaringly obvious bugs or flaws. We want this to be a smooth stabilization.

Okay. Phew. That's the main point of this post and the most exciting news. But as I said before, I think it's also reasonable for me to explain *what* this feature is, what you can do with it, and some of the background and how we got here.

## So what are GATs?

**Note: this will only be a brief overview. The [RFC] contains many more details.**

GATs (generic associated types) were originally proposed in [RFC 1598][RFC]. As said before, they allow you to define type, lifetime, or const generics on associated types. If you're familiar with languages that have "higher-kinded types", then you could call GATs *type constructors on traits*. Perhaps the easiest way for you to get a sense of how you might use GATs is to jump into an example.

Here is a popular use case: a `LendingIterator` (formerly known as a `StreamingIterator`):

```rust
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

Let's go through one implementation of this, a hypothetical `<[T]>::windows_mut`, which allows for iterating through overlapping mutable windows on a slice. If you were to try to implement this with `Iterator` today like

```rust
struct WindowsMut<'t, T> {
    slice: &'t mut [T],
    start: usize,
    window_size: usize,
}

impl<'t, T> Iterator for WindowsMut<'t, T> {
    type Item = &'t mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        let retval = self.slice[self.start..].get_mut(..self.window_size)?;
        self.start += 1;
        Some(retval)
    }
}
```

then you would get an error.

```rust
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in function call due to conflicting requirements
  --> src/lib.rs:9:22
   |
9  |         let retval = self.slice[self.start..].get_mut(..self.window_size)?;
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the method body at 8:13...
  --> src/lib.rs:8:13
   |
8  |     fn next<'a>(&'a mut self) -> Option<Self::Item> {
   |             ^^
note: ...so that reference does not outlive borrowed content
  --> src/lib.rs:9:22
   |
9  |         let retval = self.slice[self.start..].get_mut(..self.window_size)?;
   |                      ^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime `'t` as defined on the impl at 6:6...
  --> src/lib.rs:6:6
   |
6  | impl<'t, T: 't> Iterator for WindowsMut<'t, T> {
   |      ^^
```

Put succinctly, this error is essentially telling us that in order for us to be able to return a reference to `self.slice`, it must live as long as `'a`, which would require a `'a: 't` bound (which we can't provide). Without this, we could call `next` while already holding a reference to the slice, creating overlapping mutable references. However, it does compile fine if you were to implement this using the `LendingIterator` trait from before:

```rust
impl<'t, T> LendingIterator for WindowsMut<'t, T> {
    type Item<'a> where Self: 'a = &'a mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        let retval = self.slice[self.start..].get_mut(..self.window_size)?;
        self.start += 1;
        Some(retval)
    }
}
```

As an aside, there's one thing to note about this trait and impl that you might be curious about: the `where Self: 'a` clause on `Item`. Briefly, this allows us to use `&'a mut [T]`; without this where clause, someone could try to return `Self::Item<'static>` and extend the lifetime of the slice. We understand that this is a point of confusion sometimes and are considering potential alternatives, such as always assuming this bound or implying it based on usage within the trait (see [this issue][#87479]). We definitely would love to hear about your use cases here, particularly when assuming this bound would be a hindrance.

As another example, imagine you wanted a struct to be generic over a pointer to a specific type. You might write the following code:

```rust
trait PointerFamily {
    type Pointer<T>: Deref<Target = T>;

    fn new<T>(value: T) -> Self::Pointer<T>;
}

struct ArcFamily;
struct RcFamily;

impl PointerFamily for ArcFamily {
    type Pointer<T> = Arc<T>;
    ...
}
impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;
    ...
}

struct MyStruct<P: PointerFamily> {
    pointer: P::Pointer<String>,
}
```

We won't go in-depth on the details here, but this example is nice in that it not only highlights the use of types in GATs, but also shows that you can still use the trait bounds that you already can use on associated types.

These two examples only scratch the surface of the patterns that GATs support. If you find any that seem particularly interesting or clever, we would love to hear about them over on [Zulip]!

## Why has it taken so long to implement this?

So what has caused us to have taken nearly four years to get to the point that we are now? Well, it's hard to put into words how much the existing trait solver has had to change and adapt; but, consider this: for a while, it was thought that to support GATs, we would have to transition rustc to use [Chalk], a potential future trait solver that uses logical predicates to solve trait goals (though, while some progress has been made, it's still very experimental even now).

For reference, here are some various implementation additions and changes that have been made that have furthered GAT support in some way or another:
- Parsing GATs in AST ([#45904])
- Resolving lifetimes in GATs ([#46706])
- Initial trait solver work to support lifetimes ([#67160])
- Validating projection bounds (and making changes that allow type and const GATs) ([#72788])
- Separating projection bounds and predicates ([#73905])
- Allowing GATs in trait paths ([#79554])
- Partially replace leak check with universes ([#65232])
- Move leak check to later in trait solving ([#72493])
- Replacing bound vars in GATs with placeholders when projecting ([#86993])

And to further emphasize the work above: many of these PRs are large and have considerable design work behind them. There are also several smaller PRs along the way. *But*, we made it. And I just want to congratulate everyone who's put effort into this one way or another. You rock.

## What limitations are there currently?

Ok, so now comes the part that nobody likes hearing about: the limitations. Fortunately, in this case, there's really only one GAT limitation: traits with GATs are not object safe. This means you won't be able to do something like

```rust
fn takes_iter(_: &mut dyn for<'a> LendingIterator<Item<'a> = &'a i32>) {}
```

The biggest reason for this decision is that there's still a bit of design and implementation work to actually make this usable. And while this is a nice feature, adding this in the future would be a backward-compatible change. We feel that it's better to get *most* of GATs stabilized and then come back and try to tackle this later than to block GATs for even longer. Also, GATs without object safety are still very powerful, so we don't lose much by defering this.

As was mentioned earlier in this post, there are still a couple remaining diagnostics [issues](https://github.com/rust-lang/rust/labels/F-generic_associated_types). If you do find bugs though, please file issues!

[RFC]: https://github.com/rust-lang/rfcs/pull/1598
[min_const_generics]: https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#const-generics-mvp
[Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/144729-wg-traits
[Chalk]: https://github.com/rust-lang/chalk
[#87479]: https://github.com/rust-lang/rust/issues/87479
[#45904]: https://github.com/rust-lang/rust/pull/45904
[#46706]: https://github.com/rust-lang/rust/pull/46706
[#67160]: https://github.com/rust-lang/rust/pull/67160
[#72788]: https://github.com/rust-lang/rust/pull/72788
[#73905]: https://github.com/rust-lang/rust/pull/73905
[#79554]: https://github.com/rust-lang/rust/pull/79554
[#65232]: https://github.com/rust-lang/rust/pull/65232
[#72493]: https://github.com/rust-lang/rust/pull/72493
[#86993]: https://github.com/rust-lang/rust/pull/86993
