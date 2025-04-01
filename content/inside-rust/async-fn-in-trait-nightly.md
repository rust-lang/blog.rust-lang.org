+++
path = "inside-rust/2022/11/17/async-fn-in-trait-nightly"
title = "Async fn in trait MVP comes to nightly"
authors = ["Tyler Mandry"]
aliases = ["inside-rust/2022/11/17/async-fn-in-trait-nightly.html"]

[extra]
team = "The Rust Async Working Group"
team_url = "https://www.rust-lang.org/governance/wgs/wg-async"
+++

The async working group is excited to announce that `async fn` can now be used in traits in the nightly compiler. You can now write code like this:

```rust
#![feature(async_fn_in_trait)]

trait Database {
    async fn fetch_data(&self) -> String;
}

impl Database for MyDb {
    async fn fetch_data(&self) -> String { ... }
}
```

A full working example is available in the [playground][play-concrete-spawn]. There are some limitations we'll cover, as well as a few known bugs to be worked out, but we think it is ready for some users to try. Read on for the specifics.

## Recap: How async/await works in Rust

`async` and `.await` were a major improvement in the ergonomics of writing async code in Rust. In Rust, an `async fn` returns a `Future`, which is some object that represents an ongoing asynchronous computation.

The type of the future does not actually appear in the signature of an `async fn`. When you write an async function like this:

```rust
async fn fetch_data(db: &MyDb) -> String { ... }
```

The compiler rewrites it to something like this:

```rust
fn fetch_data<'a>(db: &'a MyDb) -> impl Future<Output = String> + 'a {
    async move { ... }
}
```

This "desugared" signature is something you can write yourself, and it's useful for examining what goes on under the hood. The `impl Future` syntax here represents some _opaque type_ that implements `Future`.

The future is a state machine responsible for knowing how to continue making progress the next time it wakes up. When you write code in an `async` block, the compiler generates a future type specific to that async block for you. This future type does not have a name, so we must instead use an opaque type in the function signature.

## The historic problem of `async fn` in trait

Traits are the fundamental mechanism of abstraction in Rust. So what happens if you want to put an async method in a trait? Each `async` block or function creates a unique type, so you would want to express that each implementation can have a different Future for the return type. Thankfully, we have associated types for this:

```rust
trait Database {
    type FetchData<'a>: Future<Output = String> + 'a where Self: 'a;
    fn fetch_data<'a>(&'a self) -> FetchData<'a>;
}
```

Notice that this associated type is generic. Generic associated types haven't been supported in the language... [until now][GATs]! Unfortunately though, even with GATs, you still can't write a trait _implementation_ that uses `async`:

```rust
impl Database for MyDb {
    type FetchData<'a> = /* what type goes here??? */;
    fn fetch_data<'a>(&'a self) -> FetchData<'a> { async move { ... } }
}
```

Since you can't name the type constructed by an async block, the only option is to use an opaque type (the `impl Future` we saw earlier). But those are not supported in associated types![^tait]

[^tait]: This feature is called ["type alias impl trait"](https://rust-lang.github.io/rfcs/2515-type_alias_impl_trait.html).

### Workarounds available in the stable compiler

So to write an `async fn` in a trait we need a concrete type to specify in our impl. There are a couple ways of achieving this today.

#### Runtime type erasure

First, we can avoid writing the future type by erasing it with `dyn`. Taking our example from above, you would write your trait like this:

```rust
trait Database {
    fn fetch_data(&self)
    -> Pin<Box<dyn Future<Output = String> + Send + '_>>;
}
```

This is significantly more verbose, but it achieves the goal of combining async with traits. What's more, the [async-trait] proc macro crate rewrites your code for you, allowing you to simply write

```rust
#[async_trait]
trait Database {
    async fn fetch_data(&self) -> String;
}

#[async_trait]
impl Database for MyDb {
    async fn fetch_data(&self) -> String { ... }
}
```

This is an excellent solution for the people who can use it!

Unfortunately, not everyone can. You can't use `Box` in no_std contexts. Dynamic dispatch and allocation come with overhead that can be [overwhelming][barbara-benchmark] for highly performance-sensitive code. Finally, it bakes a lot of assumptions into the trait itself: allocation with `Box`, dynamic dispatch, and the `Send`-ness of the futures. This makes it unsuitable for many libraries.

Besides, users [expect][alan-async-traits] to be able to write `async fn` in traits, and the experience of adding an external crate dependency is a papercut that gives async Rust a reputation for being difficult to use.

#### Manual `poll` implementations

Traits that need to work with zero overhead or in no_std contexts have another option: they can take the concept of polling from the [`Future` trait](https://doc.rust-lang.org/stable/std/future/trait.Future.html) and build it directly into their interface. The `Future::poll` method returns `Poll::Ready(Output)` if the future is complete and `Poll::Pending` if the future is waiting on some other event.

You can see this pattern, for example, in the current version of the unstable [AsyncIterator](https://doc.rust-lang.org/stable/std/async_iter/trait.AsyncIterator.html) trait. The signature of `AsyncIterator::poll_next` is a cross between `Future::poll` and `Iterator::next`.

```rust
pub trait AsyncIterator {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

Before async/await, it was very common to write manual `poll` implementations. Unfortunately, they proved challenging to write correctly. In the [vision document][vision-blog] process we underwent last year, we received a number of reports on how this was [extremely difficult][alan-stream] and a [source of bugs][barbara-mutex] for Rust users.

In fact, the difficulty of writing manual poll implementations was a primary reason for adding async/await to the core language in the first place.

## What's supported in nightly

We've been working to support `async fn` directly in traits, and an implementation [recently landed][initial-impl] in nightly! The feature still has some rough edges, but let's take a look at what you can do with it.

First, as you might expect, you can write and implement traits just like the above.

```rust
#![feature(async_fn_in_trait)]

trait Database {
    async fn fetch_data(&self) -> String;
}

impl Database for MyDb {
    async fn fetch_data(&self) -> String { ... }
}
```

One thing this will allow us to do is standardize new traits we've been waiting on this feature for. For example, the `AsyncIterator` trait from above is significantly more complicated than its analogue, `Iterator`. With the new support, we can simply write this instead:

```rust
#![feature(async_fn_in_trait)]

trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}
```

There's a decent chance that the trait in the standard library will end up exactly like this! For now, you can also use the one in the [`async_iterator` crate](https://docs.rs/async-iterator/latest/async_iterator/) and write generic code with it, just like you would normally.

```rust
async fn print_all<I: AsyncIterator>(mut count: I)
where
    I::Item: Display,
{
    while let Some(x) = count.next().await {
        println!("{x}");
    }
}
```

### Limitation: Spawning from generics

There is a catch! If you try to *spawn* from a generic function like `print_all`, and (like the majority of async users) you use a work stealing executor that requires spawned tasks to be `Send`, you'll hit an error which is not easily resolved.[^actual-error]

```rust
fn spawn_print_all<I: AsyncIterator + Send + 'static>(mut count: I)
where
    I::Item: Display,
{
    tokio::spawn(async move {
        //       ^^^^^^^^^^^^
        // ERROR: future cannot be sent between threads safely
        while let Some(x) = count.next().await {
            //              ^^^^^^^^^^^^
            // note: future is not `Send` as it awaits another future which is not `Send`
            println!("{x}");
        }
    });
}
```

[^actual-error]: The actual error message produced by the compiler is a bit noisier than this, but that will be improved.

You can see that we added an `I: Send` bound in the function signature, but that was not enough. To satisfy this error we need to say that the *future returned by `next()`* is `Send`. But as we saw at the beginning of this post, async functions return anonymous types. There's no way to express bounds on those types.

There are potential solutions to this problem that we'll be exploring in a follow-up post. But for now, there are a couple things you can do to get out of a situation like this.

#### Hypothesis: This is uncommon

First, you *may* be surprised to find that this situation just doesn't occur that often in practice. For example, we can spawn a task that invokes the above `print_all` function [without any problem][play-concrete-spawn]:

```rust
async fn do_something() {
    let iter = Countdown::new(10);
    executor::spawn(print_all(iter));
}
```

[play-concrete-spawn]: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=6ffde69ba43c6c5094b7fbdae11774a9

This works without any `Send` bounds whatsoever! This works because `do_something` knows the concrete type of our iterator, `Countdown`. The compiler knows that this type is `Send`, and that `print_all(iter)` therefore produces a future that is `Send`.[^auto-traits-special]

One hypothesis is that while people will hit this problem, they will encounter it relatively infrequently, because most of the time `spawn` won't be called in code that's generic over a trait with async functions.

We would like to start gathering data on people's actual experiences with this. If you have relevant experience to share, [please comment on this issue][send-bound-issue].

#### When it does happen

Eventually you probably *will* want to spawn from a context that's generic over an async trait that you call. What then!?

For now it's possible to use another new nightly-only feature, `return_position_impl_trait_in_trait`, to express the bound you need directly in your trait:

```rust
#![feature(return_position_impl_trait_in_trait)]

trait SpawnAsyncIterator: Send + 'static {
    type Item;
    fn next(&mut self) -> impl Future<Output = Option<Self::Item>> + Send + '_;
}
```

Here we've *desugared* our `async fn` to a regular function returning `impl Future + '_`, which works just like normal `async fn`s do. It's more verbose, but it gives us a place to put a `+ Send` bound! What's more, you can continue to use `async fn` in an `impl` of this trait.

The downside of this approach is that the trait becomes less generic, making it less suitable for library code. If you want to maintain two separate versions of a trait, you can do that, and (perhaps) provide macros to make it easier to implement both.

This solution is intended to be temporary. We'd like to implement a better solution in the language itself, but since this is a nightly-only feature we prefer to get more people trying it out as soon as possible.

### Limitation: Dynamic dispatch

There's one final limitation: You can't call an `async fn` with a `dyn Trait`. Designs to support this exist[^dyn-designs], but are in the earlier stages. If you need dynamic dispatch from a trait, you're better off using the `async_trait` macro for now.

## Path to stabilization

The async working group would like to get something useful in the hands of Rust users, even if it doesn't do *everything* they might want. That's why despite having some limitations, the current version of `async fn` in traits might not be far off from stabilization.[^stabilization-when] You can follow progress by watching the [tracking issue](https://github.com/rust-lang/rust/issues/91611).

[^stabilization-when]: When? Possibly sometime in the next six months or so. But don't hold me to it :)

There are two big questions to answer first:

* **Do we need to solve the "spawning from generics" (`Send` bound) problem first?** Please leave feedback on [this issue][send-bound-issue].
* **What other bugs and quality issues exist?** Please file [new issues](https://github.com/rust-lang/rust/issues/new/choose) for these. You can view [known issues here](https://github.com/rust-lang/rust/labels/F-async_fn_in_trait).

If you're an async Rust enthusiast and are willing to try experimental new features, we'd very much appreciate it if you gave it a spin!

If you use `#[async_trait]`, you can try removing it from some traits (and their impls) where you don't use dynamic dispatch. Or if you're writing new async code, try using it there. Either way, you can tell us about your experience at the links above.

## Conclusion

This work was made possible thanks to the efforts of many people, including

* Michael Goulet
* Santiago Pastorino
* Oli Scherer
* Eric Holk
* Dan Johnson
* Bryan Garza
* Niko Matsakis
* Tyler Mandry

In addition it was built on top of years of compiler work that enabled us to ship [GATs] as well as other fundamental type system improvements. We're deeply grateful to all those who contributed; this work would not be possible without you. Thank you!

To learn more about this feature and the challenges behind it, check out the [Static async fn in traits RFC][RFC] and [why async fn in traits are hard]. Also stay tuned for a follow-up post where we explore language extensions that make it possible to express `Send` bounds without a special trait.


_Thanks to Yoshua Wuyts, Nick Cameron, Dan Johnson, Santiago Pastorino, Eric Holk, and Niko Matsakis for reviewing a draft of this post._


[^auto-traits-special]: Auto traits such as `Send` and `Sync` are special in this way. The compiler knows that the return type of `print_all` is `Send` if and only if the type of its argument `Send`, and unlike with regular traits, it is allowed to use this knowledge when type checking your program.
[^dyn-designs]: See [Async fn in dyn trait](https://rust-lang.github.io/async-fundamentals-initiative/explainer/async_fn_in_dyn_trait.html) on the initiative website, as well as posts 8 and 9 in [this series](https://smallcultfollowing.com/babysteps/blog/2022/09/21/dyn-async-traits-part-9-callee-site-selection/).

[initial-impl]: https://github.com/rust-lang/rust/pull/101224
[GATs]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
[RFC]: https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
[why async fn in traits are hard]: https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/
[async-trait]: https://crates.io/crates/async-trait
[vision-blog]: https://blog.rust-lang.org/2021/03/18/async-vision-doc.html
[alan-stream]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_hates_writing_a_stream.html
[alan-async-traits]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/alan_needs_async_in_traits.html
[barbara-mutex]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_polls_a_mutex.html
[barbara-benchmark]: https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_benchmarks_async_trait.html
[send-bound-issue]: https://github.com/rust-lang/rust/issues/103854
