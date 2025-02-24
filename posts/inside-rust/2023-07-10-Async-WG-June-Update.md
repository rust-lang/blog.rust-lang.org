---
layout: post
title: "Async Working Group July update"
author: Niko Matsakis and Tyler Mandry
team: The Rust Async Working Group <https://www.rust-lang.org/governance/wgs/wg-async>
---

This is the July update from the Async Working Group of the Rust Programming Language. In May, we [laid out our plans for the year][may]. Over the last two months, we have gotten a clearer picture of the stabilization strategy. The current plan is to stabilize the full async function in trait MVP in two phases:

[may]: https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html

* Phase 1: support `async fn` and `-> impl Trait` in traits and impls (target: Rust 1.73, Oct 5).
* Phase 2: support "send bounds", likely (but not necessarily) through ["return type notation"][rtn] (target: Rust 1.74, Nov 16).

[rtn]: https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html#mvp-part-2-send-bounds-and-associated-return-types

The first phase covers the ability to write `async fn` and `-> impl Trait` in both traits and impls, roughly as described in [RFC 3185] and [RFC 3425]. These features have been implemented for some time and they are not controversial, though we are still working to do a final triage of bugs and blockers. This first phase provides functionality roughly equivalent to the existing `#[async_trait]` macro and, as such, it's enough for a great many use cases.

[RFC 3185]: https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
[RFC 3425]: https://github.com/rust-lang/rfcs/pull/3425

The second phase will add support for the ability to put bounds on the futures returned by async functions. Our case studies found that this was a common need as well, particularly for traits which would appear in the standard library, as such traits do not want to return futures to always be `Send`. We have an experimental implementation of ["return type notation"][rtn] on master, but the best solution for the sends bound problem is still under some debate. 

## When to stabilize async functions in traits

One of the key questions we were considering was whether to hold off on stabilizing async functions in traits until we were ready to stabilize a solution for the "send bounds" problem as well. On the one hand, our [case studies][cs] showed that send bounds would be important to many users. Moreover, the stabilization of async functions in traits is going to be a big deal and many users will try using them. We don't want them to get frustrated with missing functionality.

[cs]: https://github.com/rust-lang/async-fundamentals-initiative/tree/master/evaluation/case-studies

On the other hand, a true "send bounds" solution is only needed when you wish to have a trait that *may* or *may not* return `Send` futures. Many users can get by with traits that either *always* require `Send` or *never* require `Send`, as demonstrated by the `#[async_trait]` macro.

Rust has a tradition of making new functionality available incrementally rather than waiting for perfection, and it's always served us well (async functions themselves are an example of this, as is `-> impl Trait` in other functions). Moreover, while it is possible to emulate `async fn` using `-> impl Future`, the translation can be subtle. We feared that releasing `-> impl Trait` support without `async fn` would lead to people misusing the former to emulate the latter. On balance, we felt that it would be better to release async functions as quickly as possible.

## What Phase 1 enables: using Async Functions in Traits

Phase 1 provides enough functionality to make it possible to write traits that contain async functions that are either *always* `Send` or *never* `Send`. This makes it roughly equivalent to what you can do with the `#[async_trait]` macro, but potentially more efficient, since you can avoid boxing when using static dispatch. It is also possible to fill in support for `dyn` dispatch using a macro. We expect that many async trait users in these early phases will wind up using some sort of macros to remove some boilerplate, and are discussing shipping a rust-lang crate that provides these common utilities.

For example, consider the `HealthCheck` trait shown here:

```rust
trait HealthCheck {
    async fn check(&mut self);
}

impl HealthCheck for MyType {
    async fn check(&mut self) { /* ... */ }
}
```

As written, this `HealthCheck` trait is not guaranteed to return `Send` futures. Using explicit `-> impl Future` notation, however, it is possible to make a variant of the trait that *does* guarantee returning `Send` futures; better yet, it's still possible to write `async fn` in the impl:

```rust
trait SendHealthCheck {
    fn check(&mut self) -> impl Future<Output = ()> + Send + '_;
}

impl SendHealthCheck for MyType {
    async fn check(&mut self) { /* ... */ }
}
```

If the async fn is not `Send`, the [compiler would error](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=0948f287cd7fae0c8ddad3a586b698f4).

## What Phase 2 enables: the "send bound" problem

Phase 1 is great, but it doesn't support writing traits that only *sometimes* require `Send` bounds. This is really needed for traits that will be used in highly generic contexts, like the standard library. To support those kinds of cases, we really want a solution to the "send bounds" problem. We currently have an experimental implementation of one proposal, return type notation, and this month a draft RFC was [posted to Zulip](https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async/topic/associated.20return.20types.20draft.20RFC/near/356796689) and then reviewed in a [lang team design meeting](https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2023-05-24-return-type-notation.md). That conversation turned up a few other ideas that hadn't been considered yet, and we are talking about them before settling on a final direction. The goal is to open an RFC over the next few months.

## Timeline and Roadmap

To provide an overview of our progress and the expected timeline, here is our updated roadmap for stabilizing `-> impl Trait` in traits, async functions in traits (AFIT), and addressing the "send bounds" problem (SBP).

- [x] MVP implementation
- [x] Case study evaluations complete
- [x] Accepted RFC for `-> impl Trait` in traits (target: 2023-05-31)
- [x] Triage issues and identify blocks (target: 2023-07-07)
- [ ] Evaluation period and bug fixing (target: 2023-07-30)
- [ ] Stabilization report for AFIT authored (target: 2023-08-01)
- [ ] Stabilization complete for 1.73.0 (target: 2023-08-15, deadline 2023-08-24)
- [ ] **AFIT/RPITIT available on stable as part of 1.73.0 (release date 2023-10-05)**
- [ ] Authored RFC addressing SBP (target: 2023-08-15)
- [ ] Stabilization report for SBP solution authored (target: 2023-08-29)
- [ ] Stabilization complete for SBP solution 1.75.0 (target: 2023-10-30)
- [ ] **SBP solution available on stable as part of 1.75.0 (release date 2023-12-28)**

## Conclusion

The Async Working Group is focused on stabilizing RPITIT and async functions in traits together for the Rust 1.73.0 release. We encourage you to test these features, provide feedback, and report any issues you encounter. The "send bounds" problem is also being actively discussed, and we expect to propose a solution soon. Thank you for your support and contributions as we strive to shape the future of asynchronous programming in Rust.

Stay tuned for further updates from the Async Working Group as we progress towards the Rust 1.73.0 release. Together, let's make Rust an even better language for async programming!
