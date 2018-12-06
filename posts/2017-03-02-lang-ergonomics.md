---
layout: post
title: "Rust's language ergonomics initiative"
author: Aaron Turon
description: "Ergonomics, learnability, and the fact that sometimes implicit is better"
---

To help bring our [2017 vision for Rust] to fruition, the Rust subteams are
launching initiatives targeted at specific roadmap goals. **This post covers the
language team's major initiative: improving the ergonomics of the core
language.** The aim is to improve productivity and bring down the learning curve
by streamlining features and glossing over irrelevant details.

[2017 vision for Rust]: https://blog.rust-lang.org/2017/02/06/roadmap.html

## Ergonomics

**Ergonomics is a measure of the friction you experience when trying to get
things done with a tool**. You want to achieve a state of "flow", in which ideas
and intuitions are steadily transformed into working code with a minimum of
fuss. (And, with Rust, we want that code to be reliable and fast as well.) The
great threat to this experience is **interruptions**, which come in many forms:
looking things up, switching contexts, going through a large amount of ceremony,
or dealing with reams of errors where the compiler pedantically points out tiny
things you forgot. Anything that takes your attention away from the problem at
hand and puts it on details that don't really matter (or don't matter *just yet*).

A corollary is that ergonomics is rarely about raw character count. When you
have good momentum, the difference between typing `pos` and `position` isn't so
significant; **what matters much more is how easy it is to remember the right
one to type**. Similarly, *typing* `ref` in a `match` pattern, or `*` to
dereference an `&i32`, is easy; knowing or remembering to type them, on the
other hand...

The same things you have to remember as an expert are things you have to learn
as a newbie. Ergonomic improvements make life better for everyone.

Often, the heart of the matter is the question of **what to make implicit**. In
the rest of this post, I'll present a basic framework for thinking about this
question, and then apply that framework to three areas of Rust, analyzing both
the current design and some streamlining we may want to consider this year.

### Implicit vs explicit

**Information is implicit when it is *implied* but not *expressed*.** The
potential ergonomic wins are pretty easy to see: forcing you to write down
information that's obvious (because it's already implied) is a pain, because it
adds distracting noise and is easy to forget. Allowing you to leave it implicit
reduces that friction. Yet implicitness gets a bad rap, with Python going so far
as to state "explicit is better than implicit" as a core design rule. Why?

Implicitness can be very powerful. After all, the compiler knows a *lot* about
your code, and it's possible to leverage that to inject behavior in all kinds of
subtle ways. If taken too far, these techniques can impair readability, or
worse: introduce surprising behavior that can be tricky to track down, because
it's the result of a subtle chain of inference. If you've had first-hand
experience with these pitfalls, it's easy to come away with the sense that
implicitness itself is to blame.

But this, in my opinion, is a misdiagnosis of the problem, one that throws out
the baby with the bathwater. The root issue is instead: **how much information
do you need to confidently understand what a particular line of code is doing,
and how hard is that information to find?** Let's call this the *reasoning
footprint* for a piece of code. The pitfalls above come from the reasoning
footprint getting out of hand, rather than implicitness per se.

Does readability then demand that we *minimize* the reasoning footprint? I don't
think so: make it too small, and code becomes hopelessly verbose, making it
difficult to read by forcing too much information into view at all times. What
we want is a sweet spot, where routine or easy to find details can be left out,
but relevant or surprising information is kept front and center.

### How to analyze and manage the reasoning footprint

There are three dimensions of the reasoning footprint for implicitness:

- **Applicability**. Where are you allowed to elide implied information? Is
  there any heads-up that this might be happening?

- **Power**. What influence does the elided information have? Can it radically
  change program behavior or its types?

- **Context-dependence**. How much of do you have to know about the rest of the
  code to know what is being implied, i.e. how elided details will be filled in?
  Is there always a clear place to look?

**The basic thesis of this post is that implicit features should balance these
three dimensions**. If a feature is large in one of the dimensions, it's best to
strongly limit it in the other two.

The [`?` operator](https://blog.rust-lang.org/2016/11/10/Rust-1.13.html) in Rust
is a good example of this kind of tradeoff. It explicitly (but concisely) marks
a point where you will bail out of the current context on an error, possibly
doing an implicit conversion on the way. The fact that it's marked means the
feature has strongly limited applicability: you'll never be surprised that it's
coming into play. On the other hand, it's fairly powerful, and somewhat
context-dependent, since the conversion can depend on the type where `?` is
used, and the type expected in the scope it's jumping to. Altogether, this
careful balance makes error handling in Rust feels as ergonomic as working with
exceptions while avoiding some of their well-known downsides.

By contrast, a feature like unrestricted implicit conversion rightfully has a
bad reputation, because it's universally applicable, quite powerful, *and*
context-dependent. If we were to expand implicit conversions in Rust, we would
likely limit their power (by, say, restricting them to `AsRef`-style coercions,
which can do very little).

One route for strongly limiting context-dependence is employing *conventions*,
in which the compiler is simply assuming a default unless told otherwise. Often
such conventions are universal and well-known, meaning that you don't need to
know anything about the rest of the code to know what they are. A good example
of this technique in Rust is the the fact that `mod foo;` looks for `foo.rs` (or
`foo/mod.rs`) by default.

One final point. "Implicitness" is often relative to where the language is
today, something that seems radical at first—like type inference!—but then
quickly disappears into the background, to the point that it no longer feels
implicit at all (see [Stroustrup's rule]). But sometimes a bit of implicitness
really is a bad idea. The key is to carefully consider the impact on the
reasoning footprint.

[Stroustrup's rule]: https://thefeedbackloop.xyz/stroustrups-rule-and-layering-over-time/

## Example: type annotations

One bit of ergonomics that is increasingly taken for granted is *type
inference*. In the days of yore, you'd have to annotate every local variable
with its type, a practice that seems wildly verbose now—but at the time, type
inference seemed wildly implicit.

Type inference in Rust is quite powerful, but we limit the other two dimensions:

- Applicability: type inference happens only for variable bindings; data
  types and functions must include complete, explicit signatures.

- Context-dependence: because data types and functions are annotated, it's easy
  to determine the information that's influencing the outcome of inference. You
  only need to look *shallowly* at code outside of the current function. Another
  way of saying this is that type inference is performed modularly, one function
  body at a time.

By and large, the amount of type inference we do in Rust seems to be a good
match for what you can hold in your head.

The type system also provides a good example of using conventions for
ergonomics: [lifetime elision]. That feature allows you to leave off lifetimes
from function signatures in the vast majority of cases (check out the RFC—we
measured!). **Lifetime elision greatly aids learnability, because it allows you
to work at an intuitive level with borrowing before you grapple with explicit
lifetimes.**

- Applicability: lifetime elision applies to a broad class of locations—any
  function signature—but is limited to those cases for which the lifetimes are
  *strongly* implied.

- Power: limited; elision is just a shorthand for a use of lifetime parameters,
  and if you get this wrong, the compiler will complain.

- Context-dependence: here, we overshot. The fact that elision applies to types
  other than `&` and `&mut`, means that to even know whether reborrrowing is
  happening in a signature like `fn lookup(&self) -> Ref<T>`, you need to know
  whether `Ref` has a lifetime parameter that's being left out. For something as
  common as function signatures, this is too much context. We've been considering
  pushing in the direction of a small but explicit marker to say that a lifetime
  is being elided for `Ref`, a strategy similar to the one for `?` mentioned
  earlier.

There's also been some extensions to the original elision proposal, again
carefully crafted to follow these rules, like the [lifetimes in statics] RFC.

[lifetime elision]: https://github.com/rust-lang/rfcs/pull/141
[lifetimes in statics]: https://github.com/rust-lang/rfcs/pull/1623

### Idea: implied bounds

One papercut with Rust today is the fact that, for certain data structures, you
end up having to repeat the same set of trait bounds over and over. `HashMap` is
a good example; it takes a key type which, in practice, must satisfy the `Hash`
and `Eq` traits. So the question is, how should we understand a signature like
the following?

```rust
fn use_map<K, V>(map: HashMap<K, V>) { ... }
```

Right now, such a signature would be accepted, but if you tried to use any of
`map`'s methods, you'd get an error that `K` needs to be `Hash` and `Eq`, and
have to go back and add those bounds. That's an example of the compiler being
pedantic in a way that can interrupt your flow, and doesn't *really* add
anything; the fact that we're using `K` as a hashmap key essentially forces some
additional assumptions about the type. But the compiler is making us spell out
those assumptions explicitly in the signature. This situation seems ripe for an
ergonomic improvement.

It's
[straightforward](http://smallcultfollowing.com/babysteps/blog/2014/07/06/implied-bounds/)
to assume bounds that are "implied" by the type, like assuming that `K` must be
`Hash` and `Eq` above, by tying it to the type definition:

```rust
struct HashMap<K: Hash + Eq, V> { ... }
```

What's the impact on the reasoning footprint? It means that to completely
understand a signature like

```rust
fn use_map<K, V>(map: HashMap<K, V>) { ... }
```

you need to be aware of the bounds on any type constructor like `HashMap` that
you're applying to a type variable like `K`. So in particular, if you're trying
to invoke `use_map`, you need to know that there are some unstated constraints
on `K`.

- Applicability: very broad; applies to any use of generics.

- Power: very limited; the bounds will almost always be needed anyway, and in
  any case adding bounds is not very risky.

- Context-dependence: fairly limited; it draws from the bounds on all type
  constructors that are applied to type variables (like `HashMap<K,
  V>`). Usually you will be well aware of these bounds anyway, and when *using*
  a function like `use_map`, you're generally going to be passing in an existing
  `HashMap`, which by construction will ensure that the bounds already hold.
  The compiler can reliably also produce an error pointing directly to the
  type(s) imposing unfulfilled bounds.

## Example: ownership

A lot of work went into making Rust's ownership system ergonomic, and that work
entailed judicious use of "implicit" features. It's particularly instructive to
look at the places where borrowing is explicit, and places where it's not:

- Borrowing is implicit for the receiver when invoking a method.
- Borrowing is explicit for normal function arguments and in other expressions.

Ownership is important in Rust, and reasoning locally about it is vital. So why
did we end up with this particular mix of implicit and explicit ownership
tracking?

- Applicability: common, but narrowly-described: it applies only to the receiver
  of method calls.

- Power: moderately powerful, since it can determine whether the receiver can be
  mutated (by mutably borrowing it). That's mitigated to some degree by borrow
  checking, which will at least ensure that it's *permitted* to do such a borrow.

- Context-dependence: in principle, you need to know how the method is resolved,
  and then its signature. In practice, the style of `self` borrowing is almost
  always implied by the method name (e.g. `push()` versus `len()`). Notably, this
  point does *not* apply to function arguments.

**This design also aids learnability, by often just doing "the obvious thing"
for borrowing, and thereby limiting the situations in which newcomers have to
grapple with choices about it**.

### Ideas: implied borrows

Nevertheless, there are some pain points around borrowing in Rust today. To wit:

**Discarding ownership**. Sometimes you have ownership of a value, like a
`String`, and want to pass it to a function that only needs a borrow (say,
`&str`), after which you no longer need the value. Today, you *must* borrow the
value in the argument:

```rust
fn read_config(path: &Path) -> Config { ... }

let mut path = PathBuf::new(src_dir);
path.push("Config.toml");

// we have to borrow `path` with `&` even though we're done with it
let config = read_config(&path);
```

But we could easily allow you to write `read_config(path)`, implicitly borrowing
`path` for `read_config` and then *dropping it* immediately afterwards. That
would retain one's ability to reason locally about ownership, since ownership of
`path` is indeed fully relinquished from the caller's perspective (and the
buffer is destroyed at the end of the call to `read_config`). But it allows you
to gloss over the unimportant detail that the callee happened to only need a
borrow. And again, if you just forgot to borrow, and try to use `path`
afterward, the compiler will catch it, just as it does today. This is an example
of a not terribly powerful bit of inference (it's only introducing a shared
borrow for an object about to be dropped) that we'd allow to occur virtually
everywhere.

**Borrowing in match patterns**. One stumbling block when learning Rust is the
interaction between pattern matching and borrowing. In particular, when you're
pattern matching against borrowed data, you often have to do a little
reborrowing dance:

```rust
match *foo {
    Some(ref contents) => { ... }
    None => { ... }
}
```

Here we're using `*` to dereference an `Option`, and then `ref` to *re*reference
its contents. Beginners and experienced Rustaceans alike tend to miss one or
both of these markers, in part because it's usually *the only thing you could be
doing*. Thus, we could consider inferring these markers from context:

- Infer the need for dereferencing based on the type of the expression being
  matched and the arms of the match. That's a very limited amount of context
  that will already be front and center in the programmer's head.

- Infer the need for `ref` (or `ref mut`) based on the borrowing usage in the
  match arm, much like we do for closures already. That expands the reasoning
  footprint a bit, since you can't tell at a glance from the pattern what kind
  of borrow is being taken. But examining code blocks to determine the borrows
  *they* take is something Rust programmers do all the time, and as explained in
  the ownership section, the borrowing system is designed to make that easy to
  do. And in any case, it's still quite *local* context. As usual, if you get
  this wrong, the borrow checker will catch it.

In addition to that story for context-dependence, the feature would be only
narrowly applicable (only to `match`) and only moderately powerful (since,
again, the borrower checker will catch mistakes).

Both of these changes would expand the reasoning footprint slightly, but in a
very controlled way. They remove the need to write down annotations which are
essentially already forced by nearby code. And that in turn lowers the learning
curve for `match`.

## Example: the module system

Finally, let's take the module system. In the most common usage, modules are
defined like so:

```rust
mod some_module;
```

where `some_module.rs` is a file at an appropriate place in the source tree. You
can specify an explicit path if you prefer, so this is a case of implicitness
through convention. But while this bit of implicitness helps, the module system
still makes a number of fine distinctions that trip up newcomers and require
redundancy that even old hands can forget.

### Idea: eliminate the need for `extern crate`, and maybe `mod` too

The clearest-cut case is the `extern crate` declaration, which is used to bring
an external crate into scope. The vast majority of Rust projects use Cargo for
dependency management, and already specify the crates they depend on in their
`Cargo.toml` file. Hence, `extern crate` is usually redundant, and it's easy to
forget to add it after updating `Cargo.toml`. New users often complain about
baroque distinctions between `mod`, `use`, `extern crate`, and entries in
`Cargo.toml`; maybe we could improve matters by obviating the need for `extern
crate`.  What does that mean for the reasoning footprint?

It means that to know what crates are in scope at the root, you need to consult
the `Cargo.toml`, which becomes the sole source of truth for this
concern. That's a pretty limited context: it's single place to look, and in many
cases you already need some level of awareness of its contents, to know *which
version* of the crate is being assumed. Inferring `extern crate` also fares well
on the applicability front: only root modules are affected, so it's easy to
know precisely when you need to consult `Cargo.toml`.

Thinking along similar, but more radical lines, an argument could be made about
the need for `mod` itself. After all, if we're usually just writing `mod
some_module` to tell Rust to pull in a file at a canonical location with the
same name, we're being forced to duplicate information that was already readily
available. You could instead imagine the filesystem hierarchy directly informing
the module system hierarchy. The concerns about limited context and
applicability work out pretty much the same way as with `Cargo.toml`, and the
learnability and ergonomic gains are significant.

Now, both of these proposals assume your code follows the *typical* patterns,
not making use of extra, non-default flexibility. There are a lot of questions
about the fine details and expressiveness. But, at least from an *implicitness*
perspective, neither of these changes set off any alarm bells for the reasoning
footprint.

## The initiative

With those aims and that design philosophy in mind, how do we plan to proceed?

First off, we'll be using the [roadmap tracker] to help organize ideas for
ergonomic improvements. The tracker is already populated with some of the ideas
the language team has been mulling over, but we'll keep it updated as proposals
emerge on the [the internals forum] and elsewhere. The language team is eager
to mentor, so if one of the ideas catches your eye and you'd like guidance
working toward a full-blown RFC, log your interest on the tracker! And similarly
for implementation, once the RFC has merged.

Digging deeper, there's a vital cross-cutting concern: *empathy*. The goal here
is to try to imagine and evaluate ways that Rust could be different. To do this
well, we need to be able to put ourselves back in the shoes of a newcomer. Of
someone who prefers a different workflow. We need to be able to come to Rust
fresh, shedding our current habits and mental models and trying on new ones.

And, perhaps most importantly, we need empathy for each other. Transformative
insights can be fragile; they can start out embedded in ideas that have lots of
problems. If we're too quick to shut down a line of thought based on those
problems, we risk foreclosing on avenues to something better. We've got to have
the patience to sit with ideas that are foreign and uncomfortable, and gain some
new perspective from them. We've got to trust that we all want to make Rust
better, and that good faith deliberation is the way to
[make productivity a core value, without sacrificing the others][2017 vision for Rust].

Let's do this!

[roadmap tracker]: https://github.com/rust-lang/rust-roadmap/issues/17
[the internals forum]: https://internals.rust-lang.org/
