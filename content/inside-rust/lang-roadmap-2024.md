+++
path = "inside-rust/2022/04/04/lang-roadmap-2024"
title = "Rust Lang Roadmap for 2024"
authors = ["Josh Triplett, Niko Matsakis"]
description = "The language team's concrete initiatives and hopeful aspirations for the Rust 2024 edition."
aliases = ["inside-rust/2022/04/04/lang-roadmap-2024.html"]

[extra]
team = "The Rust Lang Team"
team_url = "https://www.rust-lang.org/governance/teams/lang"
+++

Note: this blog post is a snapshot of the living roadmap at
<https://lang-team.rust-lang.org/roadmaps/roadmap-2024.html>. Subsequent
changes may occur in that version, but not in this blog post. Please see that
page for the most up-to-date version.

# Lang Team Roadmap 2024

Rust 1.0 was released in 2015. Since that time, we've seen Rust grow from a
small language used for a handful of prominent projects into a mainstay in use
at virtually every major tech company.

As we work towards Rust 2024, it's natural to ask what's next for the language.
This roadmap provides insight into that question by describing what we, as
members of the lang team with input from other Rust teams, would like to
prioritize.

We have two goals with this roadmap:

- to give people a sense for what to expect in Rust over the next few years;
- for those who would like to contribute to Rust, to help provide "starting
  points" for how to get involved, and a sense for what kind of projects we are
  looking for.

Also see the [Rust Compiler Ambitions for
2022](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html),
for plans from the Rust Compiler team, and watch the Inside Rust blog for the
upcoming roadmap from the Rust Library team.

## Rust 2024: scaling empowerment

Rust's goal is to **empower everyone to build reliable and efficient
software**. Success requires not only designing and implementing a great
language with great libraries and great tools, but also maintaining a great and
supportive community.

Our focus for Rust 2024 is to **scale empowerment** in many different ways. As
we grow, we face increasing challenges in how we can scale the ways in which we
empower people to an increasing number of people. This roadmap presents three
general themes we plan to focus on:

- **[Flatten the (learning) curve](#Theme-Flatten-the-learning-curve)**:
  scaling to new users and new use cases
    - Make Rust more accessible to new and existing users alike, and make
      solving hard problems easier.
- **[Help Rust's users help each other](#Theme-Help-users-help-each-other)**:
  scaling the ecosystem
    - Empower library authors so they can---in turn---empower their users.
- **[Help the Rust project scale](#Theme-Help-the-Rust-project-scale)**:
  scaling the project
    - Develop processes to scale to the needs and use cases of a growing number
      of users; evaluate and finish projects we've started.

For each theme, we'll describe our goals for Rust 2024, and give a few examples
of the kinds of things that we're working on right now, as well as the kinds of
things we would like to do over the next few years.

This roadmap is a starting point. Our intent is to highlight those areas that
will have the biggest impact on Rust's success. Specific examples will change
over time, whether because they're finished or because new proposals arise. As
2023 approaches, we will revisit these themes to see how much progress we have
made, and whether we wish to adjust the list.

## Theme: Flatten the (learning) curve

### The vision

Thanks to a consistent focus on ergonomics, Rust has become considerably easier
to use over the last few years. Companies building large teams of Rust users
report that the typical onboarding time for a Rust engineer is around 3-6
months. Once folks learn Rust, they typically love it. Even so, many people
report a sense of high "cognitive overhead" in using it, and "learning curve"
remains the most common reason not to use Rust. The fact is that, even after
you learn how the Rust borrow checker works, there remain a lot of "small
details" that you have to get just right to get your Rust program to compile.

For Rust 2024, we will identify and eliminate many of those patterns and
idiosyncracies that one must learn to use Rust; our goal is to let you focus
squarely on the "inherent complexity" of your problem domain and avoid
"accidental complexity" from Rust as much as possible.

Async and embedded Rust are areas of particular interest. We have made a lot of
strides to support those areas, and they are growing rapidly. Nonetheless, Rust
lacks many core capabilities that would be required to make working in those
domains not only *possible* but *straightforward and delightful*. For Rust
2024, we will close that gap.

Our plan for achieving this vision is to focus on four high-level goals (in
order from broad to precise):

- **More precise analyses, less rigamarole:** Make the compiler better able to
  recognize when code is correct via improvements to the borrow checker, type
  inference, and so forth. Identify and eliminate "boilerplate" patterns like
  having to copy-and-paste the same set of where clauses everywhere.
- **Express yourself more easily:** Where necessary, extend the language so you
  can express what you want your code to do more directly. In some cases this
  takes the form of syntactic sugar (such as
  [let-else](https://github.com/rust-lang/rust/issues/87335)) but in other
  cases it may mean extending the type system to be able to describe new
  patterns (such as [generic associated
  types](https://rust-lang.github.io/generic-associated-types-initiative/)).
- **Improve async support:** Extend our async-await support beyond the current
  "MVP" to include features like async fns in traits, async drop, and other
  features needed to support the [async vision
  document](https://rust-lang.github.io/wg-async/vision/roadmap.html) roadmap.
- **Make `dyn Trait` more usable:** Broaden the set of traits that can be used
  with `dyn` and make working with `dyn` closer to working with generics.

### How you can help

Join the rust-lang Zulip, and either start a thread in the
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
stream, or send a private message to nikomatsakis if you'd like to discuss
privately first.

### The plan (so far)

Current active [initiatives](https://lang-team.rust-lang.org/initiatives.html)
in each category include:

- **More precise analyses, less rigamarole:**
    - Non-lexical lifetimes were a big stride forward, but the [polonius
      project](https://github.com/rust-lang/polonius/) promises to improve the
      borrow check's precision even more.
    - [Implied bounds](https://github.com/rust-lang/rust/issues/44491) promise
      to remove a lot of copy-and-pasting of where clauses.
- **Express yourself more easily:**
    - [let-else](https://github.com/rust-lang/rust/issues/87335) directly
      express the "match this variant or `return`/`continue`/etc" pattern.
    - [let-chains](https://github.com/rust-lang/rust/issues/53667) allow you to
      express iterative refinement with a series of pattern-matches and
      conditionals
    - ["Type alias" impl
      Trait](https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html)
      permits APIs to name previously unnameable types. This is part of a
      larger effort to [expand impl
      Trait](https://rust-lang.github.io/impl-trait-initiative/).
    - [Generic associated
      types](https://rust-lang.github.io/generic-associated-types-initiative/)
      allow traits to express a number of patterns (like "iterable") that the
      current trait system cannot handle. They are a particularly important
      foundational piece for async programming.
- **Improve async support:**
    - We are working to support [async fns in
      traits](https://rust-lang.github.io/async-fundamentals-initiative/explainer/async_fn_in_traits.html),
      including both static dispatch and dyn dispatch.
- **Make `dyn Trait` more usable:**
    - [Dyn upcasting coercion
      initiative](https://github.com/rust-lang/dyn-upcasting-coercion-initiative/issues/6):
      Allow upcasting `dyn trait` objects from `&dyn Subtrait` to `&dyn
      Supertrait`.
    - The [async fn in
      traits](https://rust-lang.github.io/async-fundamentals-initiative/explainer/async_fn_in_traits.html)
      initiative is also extending dyn trait to support async fns and "return
      position impl Trait".

### Looking forward

Looking beyond the initiatives that are in progress, there's a lot of room for
more improvement. Here are some other ideas we'd like to see. **For many of
these ideas, the main thing they need is someone to own the design!** If you
might be interested in giving that a try, come to
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
to discuss, or send a private message to nikomatsakis.

- **More precise analyses, less rigamarole:**
    - [Deref patterns](https://github.com/rust-lang/lang-team/issues/88):
      Permit matching types with patterns they can dereference to, such as
      matching a `String` with a `"str"`.
    - Perfect derive: determine the precise conditions for generic type
      parameters based on the types of a struct fields. For instance,
      `#[derive(Clone)] struct MyStruct<T>(Rc<T>)` would not require `T: Clone`,
      because `Rc<T>` can be cloned without it.
    - Autoref, operators, and clones: Generic methods that operate on
      references sometimes necessitate types like `&u32`; since `u32` is
      `Copy`, we could automatically make it a reference. We've historically
      had some hesitance to add more reference-producing operations, because it
      can lead to types the user doesn't expect (such as `&&&str`). We have
      some ideas to simplify those cases and avoid unnecessary
      double-references.
- **Express yourself more easily:**
    - [Generators](https://github.com/rust-lang/lang-team/issues/137), allowing
      users to write iterators (async and otherwise) using custom syntax.
- **Improve async support:**
    - After adding async fn in traits, we intend to add support for async drop,
      async closures, and potentially other features.
- **Make `dyn Trait` more usable:**
    - Make more patterns "object safe" and thus usable in `dyn Trait` objects,
      including passing `self` by value and handling `impl Trait` in argument
      position ([see this post for more
      information](https://smallcultfollowing.com/babysteps/blog/2022/01/07/dyn-async-traits-part-7/)).

## Theme: Help users help each other

### The vision

Rust's combination of ownership and borrowing, low-level systems control, and
powerful extensibility mechanisms like procedural macros makes it a great
language for writing libraries. And, thanks to Cargo, using a library in your
program only takes a few lines of code. Nonetheless, there are a number of
things that library authors *can't* do, or can't do easily -- for example, they
can't control the error messages you see or deploy an "unstable" feature that
requires special opt-in to use. For Rust 2024, we want to build features that
empower library authors to better serve their users, either by helping to
manage the feature lifecycle or by expanding the capabilities of what a library
can do.

We encourage people to experiment and explore in the library ecosystem,
building new functionality for people to use. Sometimes, that new functionality
becomes a foundation for others to build on, and standardizing it simplifies
further development atop it, letting the cycle continue at another level.
However, some aspects of the Rust language (notably coherence) make it harder
to extend the Rust standard library or well-established crates from separate
libraries, discouraging experimentation. Other features (such as aspects of
method resolution) make it hard to promote best-in-class functionality into the
standard library or into well-established crates without breaking users of the
crates that first developed that functionality. For Rust 2024, we want to
pursue changes that enable more exploration in the ecosystem, and enable stable
migration of code from the ecosystem into the standard library.

Our plan for achieving this vision is to focus on four categories of work:

- **Feature lifecycle**: Help library authors support features as they move
  from experimental to finalized. Help library authors manage their development
  lifecycle and evolution.
- **Richer abstractions**: Extend the language to let library authors express
  richer abstractions.
- **Custom developer experience**: Permit library authors can tailor the
  developer experience, for example by tailoring the error messages a user gets
  when traits are not implemented or introducing custom lints.
- **Interoperability**: The library ecosystem can easily coordinate, making
  libraries work together without tying them together. Library authors can
  write code that is portable across many environments or specific to one, as
  they please.

### How you can help

Join the rust-lang Zulip, and either start a thread in the
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
stream, or send a private message to Josh Triplett if you'd like to discuss
privately first.

### The plan (so far)

Current active [initiatives](https://lang-team.rust-lang.org/initiatives.html)
in each category include:

- **Feature lifecycle:**
    - RFC 3240 proposes [edition-based method
      disambiguation](https://github.com/rust-lang/rfcs/pull/3240), to support
      moving extension methods from external crates into the standard library.
- **Richer abstractions:**
    - There are numerous core extensions to Rust's type system that permit
      richer traits to be developed. Often the lack of these features prohibits
      people from writing general purpose libraries because they can't get
      sufficient reuse:
        - [Async fn in
          traits](https://rust-lang.github.io/async-fundamentals-initiative/)
        - [Const generics](https://github.com/rust-lang/lang-team/issues/51)
          and [constant
          evaluation](https://github.com/rust-lang/lang-team/issues/22)
        - [Type alias impl
          Trait](https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html)
        - [Generic associated
          types](https://rust-lang.github.io/generic-associated-types-initiative/)
- **Custom developer experience:**
    - We are not currently doing any coordinated initiatives here, though there
      are ongoing efforts that help lay groundwork for this.
- **Interoperability:**
    - Support "global capabilities" like allocators or async runtimes, perhaps
      via an approach like [RFC
      2492](https://github.com/rust-lang/rfcs/pull/2492), and perhaps extending
      to something like [scoped contexts and
      capabilities](https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/).
    - [Negative impls in
      coherence](https://rust-lang.github.io/negative-impls-initiative/) allows
      for more flexibility in the coherence check by permitting crates to
      explicitly declare that a given type will never implement a given trait.
    - The async working group's [portability
      initiative](https://www.ncameron.org/blog/portable-and-interoperable-async-rust/)
      (which builds on the work to support [async fn in
      traits](https://rust-lang.github.io/async-fundamentals-initiative/)) will
      help the async ecosystem to grow by enabling more interoperability.

### Looking forward

Looking beyond the initiatives that are in progress, there's a lot of room for
more improvement. Here are some other ideas we'd like to see. **For many of
these ideas, the main thing they need is someone to own the design!** If you
might be interested in giving that a try, come to
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
to discuss, or send a private message to Josh Triplett.

- **Feature lifecycle**:
    - All ecosystem crates can have "release trains", with the equivalent of
      "nightly features" that require a stability opt-ins. Top-level crates
      retain control over whether any of their dependencies may use nightly
      features.
- **Richer abstractions**:
    - Allow libraries to implement the `Fn` traits to define callable objects.
    - Variadic tuples and variadic generics would address a common pain point
      of "implement this trait for tuples of any arity".
- **Custom developer experience**:
    - Allow libraries to provide custom lints for their users.
    - Allow libraries to control or customize Rust diagnostics, especially for
      trait resolution failures.
- **Interoperability**:
    - Revive the stalled [portability
      lint](https://github.com/rust-lang/rfcs/pull/1868) or pursue an
      alternative design (a recent suggestion is that the "platform" might be a
      global service, similar to [RFC
      2492](https://github.com/rust-lang/rfcs/pull/2492), permitting one to use
      where clauses to designate portable code)
    - The coherence rules make it hard to implement interoperability traits; we
      should find a way to lift this restriction, while preserving coherence's
      key benefits.
    - Adopt a standard way to write performance benchmarks (perhaps simply
      adopt `criterion` officially).
    - Better support for dynamic linking, with richer and safer types than the
      C ABI. For instance, implement an `extern "safe"` providing a subset of
      Rust's rich types.

## Theme: **Help the Rust project scale**

### The vision

The Rust repo is a blizzard of activity. This is great, but it can be
overwhelming, particularly if you are trying to figure out the status of some
particular thing that you are interested in or would like to contribute to.

To ship Rust 2024 and make Rust all that it can be, we need a system that makes
it easy for people to find out what's going on and how they can help. We want
to scale our language development through delegation, empowering developers to
own and drive the work that they are passionate about. Lang team liaisons and
frequent lang team check-in will help ensure quality, stability, and overall
coherence. The team itself will have a clear "path to membership" that helps us
to maintain our membership and make sure we have the expertise we need.

Our plan for achieving this vision is to focus on four categories of work:

- **See the status at a glance:** We want it to be easy to identify what things
  the lang-team is actively working on and how far those designs have come. We
  want every tracking issue to clearly identify what "next steps" are needed to
  push that particular feature over the finish line and make sure that those
  steps are clearly documented for would-be contributors.
- **Clear owners and clear communication:** Rust operates by consensus, but
  that doesn't mean that everybody has to know all the details of everything.
  We need a system that has clear owners for the work to be done, and ideally,
  owners that are not on the lang team. Simply dividing work though can lead to
  conflicts later on, so we also need frequent communication and updates to
  ensure that everyone is keeping abreast of the overall direction things are
  going, and to surface concerns early.
- **Efficient, open processes with tooling support:** We are always looking for
  ways to improve how we operate to help us stay on top of what is going on in
  the Rust project and to reach conclusions more quickly. One thing we've
  noticed is that processes that are supported by bots or other tooling tend to
  work much better.

### How you can help

Join the rust-lang Zulip, and either start a thread in the
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
stream, or send a private message to Josh Triplett and nikomatsakis if you'd
like to discuss privately first.

### The plan (so far)

Current active [initiatives](https://lang-team.rust-lang.org/initiatives.html)
in each category include:

- **See the status at a glance:**
    - The [initiative project
      board](https://github.com/orgs/rust-lang/projects/16/) tracks all the
      currently active initiatives that we are focusing on. For each one, it
      shows their [current
      stage](https://lang-team.rust-lang.org/initiatives/process/stages.html)
      along with their
      [owners](https://lang-team.rust-lang.org/initiatives/process/roles/owner.html)
      and [lang-team
      liaisons](https://lang-team.rust-lang.org/initiatives/process/roles/liaison.html).
    - During the [backlog
      bonanza](https://lang-team.rust-lang.org/meetings/backlog-bonanza.html)
      meetings, we are going through each older tracking issue and identifying
      what kinds of work is needed to move it forward (needs a summary, needs
      design work, etc).
    - We're taking the time to stabilize features that people are using, and
      remove incomplete features as well as features people are not using, with
      the eventual goal of treating everything open as "in-flight" rather than
      "unknown". We will also reduce the total number of in-flight features.
- **Clear owners and clear communication:**
    - The [initiative system](https://lang-team.rust-lang.org/initiatives.html)
      assigns each task an owner, who drives the design, as well as a lang-team
      liaison, who helps ensure alignment with the team. More work is needed to
      get this system up and running smoothly.
    - We are launching a [formality
      team](https://hackmd.io/@nikomatsakis/rJ3h_-kJc) that will take ownership
      of ensuring Rust's type soundness and diving into the details. This will
      help to grow the set of people with expertise in that area while also
      allowing the main lang team to consult as needed.
- **Efficient, open processes with tooling support:**
    - We have designed a new [consensus decision
      process](https://lang-team.rust-lang.org/decision_process.html) that is
      designed to overcome some of the shortcomings we've seen with rfcbot; it
      needs to be implemented. This will help us make easily reversible
      decisions easier, enable more experimentation, make it smoother to raise
      and resolve concerns, and keep track of potential issues from proposal
      through to stabilization

### Looking forward

Looking beyond the initiatives that are in progress, there's a lot of room for
more improvement. Here are some other ideas we'd like to see. If you might be
interested in giving that a try, come to
[`#t-lang/roadmap-2024`](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)
to discuss, or send a private message to Josh Triplett and nikomatsakis.

- **See the status at a glance:**
    - Find ways to integrate the older tracking issues with active initiatives;
      reduce the manual updates required to keep the project board in sync.
    - Improve the visualization of projects and blockers to something more
      compelling and easier to follow.
- **Clear owners and clear communication:**
    - Beyond the type system, there are other areas where forming specialized
      teams could be useful.
- **Efficient, open processes with tooling support:**
    - Generally improve rustbot to make meetings more efficient.
    - Improve and automate the process of going from initiative proposal to
      tracked initiative.

## Conclusion

We hope that this post has given you a taste for what we plan to focus on over
the next few years. If you'd like to help us reach these goals, please [get
involved](https://rust-lang.zulipchat.com/#narrow/stream/318377-t-lang.2Froadmap-2024)!
We've listed a number of active initiatives for each point, but we've also
included a lot of ideas that are looking for an owner. Whether you prefer to
code, design, document, or organize, there's work for you to do. And if the
only thing you want to do with Rust 2024 is to use it, we welcome that too.
Happy Rust hacking to y'all!
