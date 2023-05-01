---
layout: post
title: "Stabilizing async fn in traits in 2023"
author: Niko Matsakis and Tyler Mandry
team: The Rust Async Working Group <https://www.rust-lang.org/governance/wgs/wg-async>
---

The async working group's headline goal for 2023 is to stabilize a "minimum viable product" (MVP) version of async functions in traits. We are currently targeting Rust 1.74 for stabilization. This post lays out the features we plan to ship and the status of each one.

In November, we [blogged about nightly support for async fn in trait][pp] and identified some key next steps, most importantly [support for send bounds to allow spawning in generic functions](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html#limitation-spawning-from-generics). Since then we've done a significant amount of design exploration and collected a set of case studies evaluating how well the current code works in practice.

As of now, all of the functionality described in this blog post is implemented and available in some form on the nightly compiler. This was done to prove out the viability, but not all of it has been formally RFC'd. We'll link to playground examples and RFCs where appropriate so you can try it for yourself or read about the details.

[pp]: https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html

## MVP Part 1: Core support for "async functions in traits"

The easiest way to explain what we are going to stabilize is to use a code example. To start, we will permit the use of `async fn` in trait definitions...

```rust
trait HealthCheck {
    async fn check(&mut self) -> bool;
}
```

...and you can then use `async fn` in the corresponding impl:

```rust
impl HealthCheck for MyHealthChecker {
    async fn check(&mut self) -> bool {
        do_async_op().await
    }
}
```

Traits with async functions can then be used as you normally would:

```rust
async fn do_health_check(hc: impl HealthCheck) {
    if !hc.check().await {
        log_health_check_failure().await;
    }
}
```

**Status:** This functionality was described in [RFC 3185], merged on Dec 7, 2021, and was covered in detail in our [previous blog post][pp].

[RFC 3185]: https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html


## MVP Part 2: Send bounds and associated return types

There is one complication that arises when using async functions in traits that doesn't arise with sync functions. Many async runtimes -- notably including the default configurations of [Tokio] and [async-std] -- use a workstealing thread scheduler. This means that futures may move between worker threads dynamically to achieve load balancing. As a result, the future must only capture `Send` data.

[Tokio]: https://tokio.rs/

[async-std]: https://async.rs/

If you author a generic async function that spawns tasks on one of those runtimes, however, you will start to get compilation errors ([playground](XXX)):

```rust
async fn do_health_check_par(hc: impl HealthCheck) {
    tokio::task::spawn(async move {
        if !hc.check().await {
            log_health_check_failure().await;
        }
    });
}
```

The problem is that the future returned by `hc.check()` isn't guaranteed to be `Send`. It might access non-Send data. The solution is to add a `Send` bound, but given that this is an async function, it's not obvious how to do that. How do we talk about the future returned by a call to `hc.check()`? Associated return types provide the answer. We can convert the above function to use an explicit type parameter `HC` (instead of `impl HealthCheck`) and then add a new bound, `HC::check(): Send`. This says "the value returned by `HC::check` must be of `Send` type":

```rust
async fn do_health_check_par<HC>(hc: HC)
where
    HC: HealthCheck + Send + 'static,
    HC::check(): Send, // <-- associated return type
{
    tokio::task::spawn(async move {
        if !hc.check().await {
            log_health_check_failure().await;
        }
    });
}
```

Of course, it's kind of unfortunate that we had to rewrite from taking an `impl HealthCheck` to an explicit `HC` type parameter in order to use this notation. [RFC 2289](https://github.com/rust-lang/rfcs/pull/2289), "associated type bounds", introduced a compact notation to address this problem. That RFC is not part of this MVP, but if it were stabilized, then one could simply write:

```rust
async fn do_health_check_par(hc: impl HealthCheck<check(): Send> + Send + 'static) {
    //                                            -------------
    tokio::task::spawn(async move {
        if !hc.check().await {
            log_health_check_failure().await;
        }
    });
}
```

In our [previous post][pp], we [hypothesized](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html#hypothesis-this-is-uncommon) that this problem might not occur often in practice. However, our case studies found that it comes up quite frequently, and so we decided that a solution is needed. We explored a number of solutions and concluded that associated return types are the most practical.
 
**Status:** Associated return types have an experimental implementation and we are currently drafting an RFC. There are several open bugs that will need to be fixed.

## MVP Part 3: "impl trait in traits" (return position)

All across Rust, an async function is "syntactic sugar" for a function that returns an `impl Future` -- and async functions in traits are no exception. As part of the MVP, we plan to stabilize the use of `-> impl Trait` notation in traits.

Impl trait in traits has all kinds of uses, but one common one for async programming is to avoid capturing all of the function arguments by doing some amount of sync work and then returning a future for the rest. For example, this `LaunchService` trait declares a `launch` function that does not capture `self` (similar to the existing Tower [`Service`] trait):

[`Service`]: https://docs.rs/tower/latest/tower/trait.Service.html

```rust
trait LaunchService {
    fn launch(
        &mut self, 
        request: Request,
    ) -> impl Future<Output = u32>;
    //   -------------------------
    //   Does not capture `self` as it does
    //   not include a `+ '_`.
}
```

Even though the need for "impl trait in traits" comes up a lot in async, they are a general feature that will be useful in many contexts having nothing to do with async (for example, returning iterators from trait methods).

**Status:** Return-position impl trait in traits have an experimental implementation and are described in the recently opened [RFC 3425].

[RFC 3425]: https://github.com/rust-lang/rfcs/pull/3425

## Evaluating the MVP

To evaluate the utility of this MVP, the working group collected [five case studies][] covering the [builder-provider pattern used in the AWS SDK](https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/builder-provider-api.html#dynamic-dispatch-behind-the-api); the potential use of async function in traits in [tower][cst] and the actual use in [embassy][cse], the [Fuchsia networking stack][] and [an internal Microsoft tool][]. These studies validated that the above functionality is sufficient to use async function in traits for all kinds of things, though some situations require workarounds (hence the "MVP" title).

[Fuchsia networking stack]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/socket-handler.html

[an internal Microsoft tool]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/microsoft.html

[cst]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/tower.html

[cse]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/embassy.html

[five case studies]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies.html

## What the MVP will not support or won't support well

The case studies revealed two situations that the MVP doesn't support very well, but both of them have workarounds available. These workarounds are mechanical and once the MVP is available on stable it will be possible to automate them via a custom derive or other crates on crates.io.

### Modeling dynamic dispatch

In the MVP, traits that use async functions are not "dyn safe", meaning that they don't support dynamic dispatch. So e.g. given the `HealthCheck` trait we saw earlier, one could not write `Box<dyn HealthCheck>`. At first, this seems like a crucial limitation, since many of the use cases require dynamic dispatch! But it turns out that there is a workaround. One can define an "erased" trait internally to your crate that enables dynamic dispatch. The process was pioneered by crates like [erased serde][] and is explained in detail in the [builder-provider case study][].

In the future, async fn should work with `dyn Trait` directly.

[erased serde]: https://github.com/dtolnay/erased-serde
[builder-provider case study]: https://rust-lang.github.io/async-fundamentals-initiative/evaluation/case-studies/builder-provider-api.html#dynamic-dispatch-behind-the-api

### Send bounds are verbose, especially for traits with lots of methods

The associated return type proposal works great for traits with a single method, but it can be annoying for traits that have lots of methods. One convenient solution is to use the "trait alias pattern" (if [RFC 1733](https://github.com/rust-lang/rust/issues/41517) were stabilized, this would be easier):

```rust
trait SendHealthCheck
where
    Self: HealthCheck + Send,
    Self::check(): Send,
{}

impl<T> SendHealthCheck for T
where
    T: HealthCheck + Send,
    T::check(): Send,
{}
```

Using a pattern like this means you can write `T: SendHealthCheck`. In the future, something like [trait transformers][] may provide a more concise syntax.

[trait transformers]: https://smallcultfollowing.com/babysteps/blog/2023/03/03/trait-transformers-send-bounds-part-3/

## Timeline and roadmap

Our goal is to stabilize the MVP for Rust 1.74, which will be released on 2023-11-16. The branch window for this feature opens on July 14 and closes on August 24. To actually stabilize in 1.74, we want to leave room for bug fixes that may arise before the release branch is cut. The key milestones for this goal are as follows:

* [x] MVP implementation 
* [x] Case study evaluations complete
* [ ] Accepted RFC for return-position impl trait (target: 2023-05-31)
* [ ] Accepted RFC for associated return types (target: 2023-06-15)
* [ ] Evaluation period and bug fixing (target: 2023-06-30)
* [ ] Stabilization report authored (target: 2023-07-01)
* [ ] Stabilization complete for 1.74.0 (target: 2023-07-21)

You can find the [complete timeline in our github project][timeline].

[timeline]: https://github.com/orgs/rust-lang/projects/28/views/2

## What comes next?

So, once this MVP is done, what next? Our next immediate goals are to ship **dynamic dispatch** and **async closures** support in 2024. Together this will complete a solid foundation to tackle future async problems, such as support for async drop, easy async iterators, or portability across runtimes.
