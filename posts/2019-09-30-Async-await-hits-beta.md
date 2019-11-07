---
layout: post
title: "Async-await hits beta!"
author: Niko Matsakis
---

Big news! As of this writing, **syntactic support for async-await is
available in the Rust beta channel!** It will be available in the 1.39
release, which is expected to be released on **November 7th, 2019**.
Once async-await hits stable, that will mark the culmination of a
**multi-year effort to enable efficient and ergonomic asynchronous I/O
in Rust**. It will not, however, mark the end of the road: there is
still more work to do, both in terms of polish (some of the error
messages we get today are, um, [not great]) and in terms of feature
set ([async fn in traits], anyone?).

[not great]: https://github.com/rust-lang/rust/issues/64130
[async fn in traits]: https://github.com/dtolnay/async-trait

(If you're not familiar with what async-await is, don't despair!
There's a primer and other details later on in this post!)

### Async-await support in the ecosystem growing rapidly

But async-await has never been the entire story. To make good use of
async-await, you also need strong libraries and a vibrant ecosystem.
**Fortunately, you've got a lot of good choices, and they keep getting
better:** 

- the async runtime [tokio], for example, recently announced an [alpha
  release][] based on async-await;
- the [recently announced][] [async-std][] library was built from the
  start on the new async-await syntax;
- using [wasm-bindgen-futures], you can even bridge Rust Futures with
  [JavaScript promises];
- the [hyper library][hyper] has [migrated][hyper#1805] to adopt standard Rust futures;
- the 0.3.0 version of the [futures-rs library][futures] will support
  async-await and will be released by the time async-await hits stable
  (you can use the [0.3.0-alpha][] releases now);
- finally, async-await support is starting to become available in higher-level
  [web frameworks][wf] as well.
  
[futures]: https://crates.io/crates/futures-preview
[0.3.0-alpha]: https://rust-lang-nursery.github.io/futures-rs/blog/2018/07/19/futures-0.3.0-alpha.1.html
[wasm-bindgen-futures]: https://docs.rs/crate/wasm-bindgen-futures/0.2.16
[tokio]: https://tokio.rs/
[actix]: https://actix.rs/
[alpha release]: https://tokio.rs/blog/2019-08-alphas/
[adding support]: https://github.com/actix/actix-web/issues/955#issuecomment-523266936
[async-std]: https://async.rs/
[recently announced]: https://async.rs/blog/announcing-async-std/
[wf]: https://www.arewewebyet.org/topics/frameworks/
[JavaScript promises]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_promises
[hyper]: https://hyper.rs
[hyper#1805]: https://github.com/hyperium/hyper/issues/1805

### Restructuring Async I/O in the Rust org

Now that async-await is approaching stable, we are taking the
opportunity to make some changes to our Rust team structure. The
current structure includes two working groups: the [Async Foundations
WG], focused on building up core language support, and the [Async
Ecosystem WG], focused on supporting the ecosystem develop.

**In light of all the activity going on in the ecosystem, however,
there it not as much need for the [Async Ecosystem WG], and as such
we've decided to spin it down.** We'll be deprecating the [rustasync
github org]. The [areweasyncyet.rs] and [arewewebyet.org] websites
will move to the main [rust-lang org], but the fate of the other
projects will be decided by the people who built them. A few will
likely be deprecated, and the remainder will be moving out to be
maintained independently.

[areweasyncyet.rs]: https://areweasyncyet.rs/
[arewewebyet.org]: https://www.arewewebyet.org/
[rustasync github org]: https://github.com/rustasync/
[rust-lang org]: https://github.com/rust-lang/
[Async Foundations WG]: https://rust-lang.github.io/compiler-team/working-groups/async-await/
[Async Ecosystem WG]: https://github.com/rustasync/team
[async book]: https://github.com/rust-lang/async-book

**The [Async Foundations WG], meanwhile, will continue, but with a
shift in focus.** Now that async-await is en route to stabilization,
the focus will be on polish, such as improving diagnostics, fixing
smaller bugs, and improving documentation such as the [async
book]. Once progress is made on that, we'll be considering what
features to implement next.

(An aside: this is the first time that we've ever opted to spin *down*
a working group, and we realized that we don't have a formal policy
for that. We've [created an issue][gov25] with the [governance working
group][gov-wg] to look into that for the future.)

[gov25]: https://github.com/rust-lang/wg-governance/issues/25
[gov-wg]: https://github.com/rust-lang/wg-governance/

### Async await: a quick primer

So, what is async await? Async-await is a way to write functions that
can "pause", return control to the runtime, and then pick up from
where they left off.  Typically those pauses are to wait for I/O, but
there can be any number of uses.

You may be familiar with the async-await from other languages, such as
JavaScript or C#. Rust's version of the feature is similar, but with a
few key differences.

To use async-await, you start by writing `async fn` instead of `fn`:

```rust
async fn first_function() -> u32 { .. }
```

Unlike a regular function, calling an `async fn` doesn't do anything
to start -- instead, it returns a `Future`. This is a suspended
computation that is waiting to be executed. To actually *execute*
the future, you have to use the `.await` operator:

```rust
async fn another_function() {
    // Create the future:
    let future = first_function();
    
    // Await the future, which will execute it (and suspend
    // this function if we encounter a need to wait for I/O): 
    let result: u32 = future.await;
    ...
}
```

This example shows the first difference between Rust and other
languages: we write `future.await` instead of `await future`. This
syntax integrates better with Rust's `?` operator for propagating
errors (which, after all, are very common in I/O). One can simply
write `future.await?` to await the result of a future and propagate
errors. It also has the advantage of making method chaining painless.

### Zero-cost futures

The other difference between Rust futures and futures in other
languages is that they are based on a "poll" model, which makes them
**zero cost**. In other languages, invoking an async function
immediately creates a future and schedules it for execution: awaiting
the future isn't really necessary for it to execute. But this implies
some overhead for each future that is created. 

In contrast, in Rust, calling an async function does not do any
scheduling in and of itself, which means that we can compose a complex
nest of futures without incurring a per-future cost. As an end-user,
though, the main thing you'll notice is that **futures feel "lazy"**:
they don't do anything until you await them.

If you'd like a closer look at how futures work under the hood, take a
look at [the executor section] of the [async book], or watch the
[excellent talk][video] that [withoutboats] gave at [Rust LATAM 2019]
on the topic.

[the executor section]: https://rust-lang.github.io/async-book/02_execution/04_executor.html
[video]: https://www.youtube.com/watch?v=skos4B5x7qE
[Rust LATAM 2019]: https://rustlatam.org/
[withoutboats]: https://github.com/withoutboats

### Summary

In summary, if you've an interest in using Async I/O in Rust, this is
a very exciting time! With async-await syntax hitting stable in
November, it's going to be easier than ever to write futures (in
particular, if you tried using the combinator-based futures in the
past, you'll find [async-await integrates much better with Rust's
borrowing system][bc]). Moreover, there are now a number of great
runtimes and other libraries available in the ecosystem to work with.
So get out there and build stuff! 

(Oh, yeah, and please file bugs when you hit confusing or surprising
problems, so we can improve the user experience!)

[bc]: http://aturon.github.io/tech/2018/04/24/async-borrowing/
