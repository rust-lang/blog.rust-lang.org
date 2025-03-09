+++
layout = "post"
date = 2022-02-03
title = "Async Rust in 2022"
author = "Niko Matsakis and Tyler Mandry"
description = "The async working group's goals in 2022"
team = "Async Working Group <https://www.rust-lang.org/governance/wgs/wg-async>"
+++

Almost a year ago, the Async Working Group[^name] [embarked on a collaborative effort][ce] to write a [shared async vision document][avd]. As we enter 2022, we wanted to give an update on the results from that process along with the progress we are making towards realizing that vision.

[^name]: We used to be called the Async Foundations Working Group, or wg-async-foundations. wg-async is much easier to type. The [focus][charter] of the working group being on the "foundations" of async, namely the language and standard library, hasn't changed.

[charter]: https://rust-lang.github.io/wg-async/CHARTER.html
[ce]: https://blog.rust-lang.org/2021/03/18/async-vision-doc.html
[avd]: https://rust-lang.github.io/wg-async/vision.html

## Writing an async issue aggregator in Rust 2024

To set the scene, imagine it's Rust 2024, and you've decided to build your first project in Rust. You're working on a project that uses GitHub and you'd like a tool that will walk over all the issues on your repository and do some automatic triage. You decide to use async Rust for this. You pull out the Rust book and thumb over to the Async I/O section. In there, it shows you the basic structure of an async Rust application. Like any Rust program, it begins with `main`, but this time with an `async fn`...

```rust
async fn main() {
    ...
}
```

Thumbing over to crates.io, you search for "github" and find that there is a nifty crate `crabbycat` for navigating github issues. You import it and sit down to work. The first thing you need to do is to to iterate over all the issues:

```rust
async fn main() {
    for await? issue in crabbycat::issues("https://github.com/rust-lang/rust") {
        if meets_criteria(&issue) {
            println!("{issue:?}");
        }
    }
}
```

Your crate seems to be working well and you happily tweet about it. Soon enough you find yourself with some users and one of them opens a PR to extend it to to support GitLab. To do this, they introduce a trait that allows you to write code that is generic over the issue provider. This trait has one method, `issues` which returns an iterator (in this case, an async iterator):

```rust
trait IssueProvider {
    async fn issues(&mut self, url: &str)
        -> impl AsyncIterator<Item = Result<Issue, Err>>;
}

#[derive(Debug)]
struct Issue {
    number: usize,
    header: String,
    assignee: String,
}
```

Now they are able to refactor the main loop into a function that is generic over the `IssueProvider`. They decide to use a `dyn` trait to avoid monomorphizing many times.

```rust
fn process_issues(provider: &mut dyn IssueProvider) {
    for await? issue in provider.issues("https://github.com/rust-lang/rust") {
        if meets_criteria(&issue) {
            println!("{issue:?}");
        }
    }
}
```

You happily hit merge on the PR and everything works great. Later on, somebody wants to port your system to run on the Chartreuse operating system. Chartreuse is based on an actor model and uses its own custom async runtime -- but luckily for you, you don't care. All your code is seamlessly able to switch the underlying runtime implementation over to the Chartreuse async runtime.

## Meanwhile, in 2022...

Of course, the year is still 2022, and the vision we just painted is not reality -- at least not yet. There is a lot of work to do yet in terms of RFCing and implementing the features that will let us write the code we talked about:

* Writing the `IssueProvider` trait requires async fns in traits.
* Taking an `&mut dyn IssueProvider` argument requires supporting dynamic dispatch in traits that have async functions
    * and returning `impl AsyncIterator`!
* The code used a `for await?` loop, which permitted easy iteration over iterators in async code.
* The trait for async iteration in the standard library (`Stream`) has a different name and is not stabilized; its definition is likely to change, too, once we have strong support for async fns in traits.
* Writing `async fn main` and changing to an alternate runtime requires portability across runtimes.

As this work proceeds we can expect plenty of changes in the details along the way, and we might decide some pieces aren't worth it; if nothing else, the syntax for generators is a hotly contested topic. **What won't change is the the overall vision:** that writing async Rust code should be as easy as writing sync code, apart from the occasional `async` and `await` keyword.

## How we get there

We've organized the Async working group into a number of distinct initiatives, each of which is pursuing one part of the vision. What follows is a list of the currently active groups and some of the things they've done over the last few months since they got up and running.

### [Async fundamentals][afi] initiative

Led by [tmandry], currently focused on supporting `async fn` in traits.

* We have been coordinating and supporting the [generic associated types](https://rust-lang.github.io/generic-associated-types-initiative/) and [impl trait](https://rust-lang.github.io/impl-trait-initiative/) initiatives.
* We have also landed the [static async fn in traits](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html) RFC and [drafted an RFC for return position impl trait][rpit] (which still needs a few tweaks before it's ready to land).
* We've been working on a design for dynamic dispatch, the most recent iteration of which is described in [this blog post][dyn7].
* We've also been spinning off efforts, such as the proposal for [context and capabilities][cac] that tmandry blogged about.

### [Async iteration][aii] initiative

Led by [estebank], exploring generators and async generators.

* Estebank has [prototyped a procedural macro for generators](https://estebank.github.io/rust-iterator-item-syntax.html) and put out a call for discussion about the syntax and other details.

### [Portability][pi] initiative

Led by [nrc], exploring what it takes to make code easily portable across runtimes, beginning with standardized traits for things like `AsyncRead` and `AsyncWrite`.

* [nrc] posted a [blog post](https://www.ncameron.org/blog/portable-and-interoperable-async-rust/) laying out a vision.

### [Polish][polish] initiative

Led by [eholk], focused on improving the existing capabilities via smaller changes that collectively make a big difference.
* We've got a [pending PR][#91032] that will improve the generator's capture analysis when variables are moved before a yield point, as well as [another PR][#92508] that tightens temporary scopes to further avoid unnecessary generator captures.
* [Gus Wynn] made significant progress towards a [`must_not_suspend`][#88865] lint that catches cases where values are live across an await point that should not be.
* We are starting to look at ways to make [async stack traces][stack-traces] more readable and helpful.

### [Tooling][ti] initiative

Led by [pnkfelix], working to support folks in the async ecosystem who are building interesting tooling to support async Rust others.
* Michael Woerister is exploring [async crashdump recovery](https://github.com/rust-lang/async-crashdump-debugging-initiative), offering a mechanism to recover and inspect the state of an async Rust program based on a crashdump.
* Eliza Weisman and [many others](https://tokio.rs/blog/2021-12-announcing-tokio-console#thanks-to) recently [announced their 0.1 release](https://tokio.rs/blog/2021-12-announcing-tokio-console) of [tokio console](https://github.com/tokio-rs/console). Tokio Console is a diagnostics and debugging tool for asynchronous Rust programs. It gives you a live view onto the state of the async runtime, and can also signal warnings  when it detects suspicious behavior that might indicate a bug or performance problem.

You can find the complete set of work that we have planned on our [roadmap page](https://rust-lang.github.io/wg-async/vision/roadmap.html), which also links to various deliverables we're working toward.

[cac]: https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/
[#88865]: https://github.com/rust-lang/rust/pull/88865
[#91032]: https://github.com/rust-lang/rust/issues/91032
[#92508]: https://github.com/rust-lang/rust/pull/92508
[estebank]: https://github.com/estebank
[nrc]: https://github.com/nrc
[eholk]: https://github.com/eholk
[Gus Wynn]: https://github.com/guswynn
[pnkfelix]: https://github.com/pnkfelix
[afi]: https://rust-lang.github.io/async-fundamentals-initiative/
[aii]: https://estebank.github.io/rust-iterator-item-syntax.html
[pi]: https://www.ncameron.org/blog/portable-and-interoperable-async-rust/
[polish]: https://rust-lang.github.io/wg-async/vision/roadmap/polish.html
[dyn7]: http://smallcultfollowing.com/babysteps//blog/2022/01/07/dyn-async-traits-part-7/
[stack-traces]: https://rust-lang.github.io/wg-async/design_docs/async_stack_traces.html
[rpit]: https://github.com/rust-lang/rfcs/pull/3193
[ti]: https://nikomatsakis.github.io/wg-async/vision/deliverables/tooling.html
[tmandry]: https://github.com/tmandry

## Want to help?

If you're interested in helping out, a good place to start is the [How to help] section of the polish initiative page. There is also a weekly [polish triage meeting](https://rust-lang.github.io/wg-async/triage.html) which you may want to attend.

[How to help]: https://rust-lang.github.io/wg-async/vision/roadmap/polish.html#-how-to-help
