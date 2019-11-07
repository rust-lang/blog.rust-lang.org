---
layout: post
title: "Async-await on stable Rust!"
author: Niko Matsakis
---

**On this coming Thursday, November 7, async-await syntax hits stable
Rust, as part of the 1.39.0 release.** This work has been a long time
in development -- the key ideas for zero-cost futures, for example,
were [first proposed by Aaron Turon and Alex Crichton in
2016][zcf-rust]! -- and we are very proud of the end result. We believe
that Async I/O is going to be an increasingly important part of Rust's
story.

While this first release of "async-await" is a momentous event, it's
also only the beginning. The current support for async-await marks a
kind of "Minimum Viable Product" (MVP). We expect to be polishing,
improving, and extending it for some time.

Already, in the time since [async-await hit beta][aa-beta], we've made
a lot of great progress, including making some [key diagnostic
improvements][diag] that help to make async-await errors far more
approachable. To get involved in that work, check out
the [Async Foundations Working Group][wg]; if nothing else, you can
help us by filing bugs about polish issues or by [nominating those
bugs that are bothering you the most][nom], to help direct our
efforts.

Many thanks are due to the people who made async-await a reality. The
implementation and design would never have happened without the
leadership of cramertj and withoutboats, the implementation and polish
work from the compiler side (davidtwco, tmandry, gilescope, csmoe),
the core generator support that futures builds on (Zoxc), the
foundational work on `Future` and the `Pin` APIs (aturon,
alexcrichton, RalfJ, pythonesque), and of course the input provided by
so many community members on RFC threads and discussions.

# Major developments in the async ecosystem

Now that async-await is approaching stabilization, all the major Async
I/O runtimes are at work adding and extending their support for the
new syntax:

* the [tokio] runtime [recently announced a number of scheduler
  improvements][tokio-sched], and they are planning a stable release
  in November that supports async-await syntax;
* the [async-std] runtime [has been putting out weekly releases for the past few months][as-releases], and plans to make their
  1.0 release shortly after async-await hits stable;
* using [wasm-bindgen-futures], you can even bridge Rust Futures with
  [JavaScript promises];
* the [hyper library][hyper] has [migrated][hyper#1805] to adopt standard Rust futures;
* the newly released 0.3.0 version of the [futures-rs library][futures] includes support
  for async-await;
* finally, async-await support is starting to become available in higher-level
  [web frameworks][wf] as well, as well as other interesting applications such
  as the [`futures_intrusive`](https://docs.rs/futures-intrusive/0.2.0/futures_intrusive/)
  crate.

[futures]: https://crates.io/crates/futures
[tokio]: https://tokio.rs/
[zcf-rust]: https://aturon.github.io/blog/2016/08/11/futures/
[wasm-bindgen-futures]: https://docs.rs/crate/wasm-bindgen-futures/0.2.16
[aa-beta]: https://blog.rust-lang.org/2019/09/30/Async-await-hits-beta.html
[diag]: https://blog.rust-lang.org/inside-rust/2019/10/11/AsyncAwait-Not-Send-Error-Improvements.html
[wg]: https://rust-lang.github.io/compiler-team/working-groups/async-await/
[nom]: https://rust-lang.github.io/compiler-team/working-groups/async-await/#nominating-issues
[tokio-sched]: https://tokio.rs/blog/2019-10-scheduler/
[as-releases]: https://github.com/async-rs/async-std/releases
[0.3.0-alpha]: https://rust-lang-nursery.github.io/futures-rs/blog/2018/07/19/futures-0.3.0-alpha.1.html
[hyper]: https://hyper.rs
[hyper#1805]: https://github.com/hyperium/hyper/issues/1805
[async-std]: https://async.rs/
[wf]: https://www.arewewebyet.org/topics/frameworks/
[JavaScript promises]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Using_promises

### Async-await: a quick primer

*(This section and the next are reproduced from the ["Async-await hits
beta!"][aa-beta] post.)*

So, what is async await? Async-await is a way to write functions that
can "pause", return control to the runtime, and then pick up from
where they left off.  Typically those pauses are to wait for I/O, but
there can be any number of uses.

You may be familiar with the async-await from JavaScript or C#. Rust's
version of the feature is similar, but with a few key differences.

To use async-await, you start by writing `async fn` instead of `fn`:

```rust
async fn first_function() -> u32 { .. }
```

Unlike a regular function, calling an `async fn` doesn't have any
immediate effect. Instead, it returns a `Future`. This is a suspended
computation that is waiting to be executed. To actually *execute* the
future, use the `.await` operator:

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
errors (which, after all, are very common in I/O). You can simply
write `future.await?` to await the result of a future and propagate
errors. It also has the advantage of making method chaining painless.

### Zero-cost futures

The other difference between Rust futures and futures in JS and C# is
that they are based on a "poll" model, which makes them **zero
cost**. In other languages, invoking an async function immediately
creates a future and schedules it for execution: awaiting the future
isn't necessary for it to execute. But this implies some overhead for
each future that is created.

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
[async book]: https://github.com/rust-lang/async-book

### Summary

We believe that having async-await on stable Rust is going to be a key
enabler for a lot of new and exciting developments in Rust. If you've
tried Async I/O in Rust in the past and had problems -- particularly
if you tried the combinator-based futures of the past -- you'll find
[async-await integrates much better with Rust's borrowing
system][bc]. Moreover, there are now a number of great runtimes and
other libraries available in the ecosystem to work with.  So get out
there and build stuff!

[bc]: http://aturon.github.io/tech/2018/04/24/async-borrowing/
