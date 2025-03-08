+++
layout = "post"
title = "Announcing the Keyword Generics Initiative"
author = "Yoshua Wuyts"
team = "The Keyword Generics Initiative <https://github.com/rust-lang/keyword-generics-initiative>"
+++

We ([Oli], [Niko], and [Yosh]) are excited to announce the start of the [Keyword
Generics Initiative][kwi], a new initiative [^initiative] under the purview of
the language team. We're officially just a few weeks old now, and in this post
we want to briefly share why we've started this initiative, and share some
insight on what we're about.

[Oli]: https://github.com/oli-obk
[Niko]: https://github.com/nikomatsakis
[Yosh]: https://github.com/yoshuawuyts
[kwi]: https://github.com/rust-lang/keyword-generics-initiative

[^initiative]: Rust governance terminology can sometimes get confusing. An
"initiative" in Rust parlance is different from a "working group" or "team".
Initiatives are intentionally limited: they exist to explore, design, and
implement specific pieces of work - and once that work comes to a close, the
initiative will wind back down. This is different from, say, the lang team -
which essentially carries a `'static` lifetime - and whose work does
not have a clearly defined end.

## A missing kind of generic

One of Rust's defining features is the ability to write functions which are
_generic_ over their input types. That allows us to write a function once,
leaving it up to the compiler to generate the right implementations for us.

Rust allows you to be generic over types - it does not allow you to be generic
over other things that are usually specified by keywords. For example, whether a
function is async, whether a function can fail or not, whether a function is
const or not, etc.

The post ["What color is your function"][color] [^color] describes what happens
when a language introduces async functions, but with no way to be generic over
them:

> I will take async-await over bare callbacks or futures any day of the week.
> But we’re lying to ourselves if we think all of our troubles are gone. As soon
> as you start trying to write higher-order functions, or reuse code, you’re
> right back to realizing color is still there, bleeding all over your codebase.

This isn't just limited to async though, it applies to all modifier keywords -
including ones we may define in the future. So we're looking to fill that gap
by exploring something we call "keyword generics" [^name]: the ability to be
generic over keywords such as `const` and `async`.

[color]: https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/
[^color]: R. Nystrom, “What Color is Your Function?,” Feb. 01, 2015.
https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/
(accessed Apr. 06, 2022).

[^name]: The longer, more specific name would be: "keyword modifier generics".
We've tried calling it that, but it's a bit of a mouthful. So we're just
sticking with "keyword generics" for now, even if the name for this feature may
end up being called something more specific in the reference and documentation.

To give you a quick taste of what we're working on, this is roughly how we
imagine you may be able to write a function which is generic over "asyncness"
in the future:

> Please note that this syntax is entirely made up, just so we can use something
> in examples. Before we can work on syntax we need to finalize the semantics,
> and we're not there yet. This means the syntax will likely be subject to
> change over time.

```rust
async<A> trait Read {
    async<A> fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    async<A> fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... }
}

/// Read from a reader into a string.
async<A> fn read_to_string(reader: &mut impl Read * A) -> std::io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    string
}
```

This function introduces a "keyword generic" parameter into the function of `A`.
You can think of this as a flag which indicates whether the function is being
compiled in an async context or not. The parameter `A` is forwarded to the `impl
Read`, making that conditional on "asyncness" as well.

In the function body you can see a `.await` call. Because [the `.await` keyword
marks cancellation sites][cancel] we unfortunately can't just infer them
[^cancellation]. Instead we require them to be written for when the code is
compiled in async mode, but are essentially reduced to a no-op in non-async
mode.

[cancel]: https://blog.yoshuawuyts.com/async-cancellation-1/
[^cancellation]: No really, we can't just infer them - and it may not be as
simple as omitting all `.await` calls either. The Async WG is working through
the full spectrum of cancellation sites, async drop, and more. But for now we're
working under the assumption that `.await` will remain relevant going forward.
And even in the off chance that it isn't, fallibility has similar requirements
at the call site as async does.

We still have lots of details left to figure out, but we hope this at least
shows the general *feel* of what we're going for.

## A peek at the past: horrors before const

Rust didn't always have `const fn` as part of the language. A long long long long
long time ago (2018) we had to write a regular function for runtime computations
and associated const of generic type logic for compile-time computations. As an
example, to add the number `1` to a constant provided to you, you had to write
([playground]):

[playground]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=50e818b79b8af322ed4384d3c33e9773

```rust
trait Const<T> {
    const VAL: T;
}

/// `42` as a "const" (type) generic:
struct FourtyTwo;
impl Const<i32> for FourtyTwo {
    const VAL: i32 = 42;
}

/// `C` -> `C + 1` operation:
struct AddOne<C: Const<i32>>(C);
impl<C: Const<i32>> Const<i32> for AddOne<C> {
    const VAL: i32 = C::VAL + 1;
}

AddOne::<FourtyTwo>::VAL
```

Today this is as easy as writing a `const fn`:

```rust
const fn add_one(i: i32) -> i32 {
    i + 1
}

add_one(42)
```

The interesting part here is that you can also just call this function in
runtime code, which means the implementation is shared between both `const`
(CTFE[^ctfe]) and non-`const` (runtime) contexts.

[^ctfe]: CTFE stands for "Compile Time Function Execution": `const` functions
can be evaluated during compilation, which is implemented using a Rust
interpreter (miri).

## Memories of the present: async today

People write duplicate code for async/non-async with the only difference being
the `async` keyword. A good example of that code today is [`async-std`], which
duplicates and translates a large part of the stdlib's API surface to be async
[^async-std]. And because the Async WG has made it an explicit goal to [bring
async Rust up to par with non-async Rust][async-vision], the issue of code
duplication is particularly relevant for the Async WG as well. Nobody on the
Async WG seems particularly keen on proposing we add a second instance of just
about every API currently in the stdlib.

[`async-std`]: https://docs.rs/async-std/latest/async_std/
[async-vision]: https://rust-lang.github.io/wg-async/vision/how_it_feels.html
[^async-std]: Some limitations in `async-std` apply: async Rust is missing async
`Drop`, async traits, and async closures. So not all APIs could be duplicated.
Also `async-std` explicitly didn't reimplement any of the collection APIs to be
async-aware, which means users are subject to the "sandwich problem". The
purpose of `async-std` was to be a proving ground to test whether creating
an async mirror of the stdlib would be possible: and it's proven that it is, as
far as was possible with missing language features.

We're in a similar situation with `async` today as `const` was prior to 2018.
Duplicating entire interfaces and wrapping them in `block_on` calls is the
approach taken by e.g. the `mongodb`
[[async](https://docs.rs/mongodb/latest/mongodb/index.html),
[non-async](https://docs.rs/mongodb/latest/mongodb/sync/index.html)], `postgres`
[[async](https://docs.rs/tokio-postgres/latest/tokio_postgres/index.html),
[non-async](https://docs.rs/postgres/latest/postgres/)], and `reqwest`
[[async](https://docs.rs/reqwest/latest/reqwest/),
[non-async](https://docs.rs/reqwest/latest/reqwest/blocking/index.html)] crates:
```rust
// Async functionality like this would typically be exposed from a crate "foo":
async fn bar() -> Bar { 
    // async implementation goes here
}
```

```rust
// And a sync counterpart would typically be exposed from a crate
// named "blocking_foo" or a submodule on the original crate as
// "foo::blocking". This wraps the async code in a `block_on` call:
fn bar() -> Bar {
    futures::executor::block_on(foo::bar())
}
```

This situation is not ideal. Instead of using the host's synchronous syscalls,
we're now going through an async runtime to get the same results - something
which is often not zero-cost. But more importantly, it's rather hard to
keep both a sync and async API version of the same crate in, err, sync with each
other. Without automation it's really easy for the two APIs to get out of sync,
leading to mismatched functionality.

The ecosystem has come up with some solutions to this, perhaps most notably the
proc-macro based [`maybe-async` crate][maybe-async].  Instead of writing two
separate copies of `foo`, it generates a sync and async variant for you:

```rust
#[maybe_async]
async fn foo() -> Bar { ... }
```

[maybe-async]: https://docs.rs/maybe-async/0.2.6/maybe_async/

While being useful, the macro has clear limitations with respect to diagnostics
and ergonomics. That's absolutely not an issue with the crate, but an inherent
property of the problem it's trying to solve. Implementing a way to be generic
over the `async` keyword is something which will affect the language in many
ways, and a type system + compiler will be better equipped to handle it than
proc macros reasonably can.

## A taste of trouble: the sandwich problem

A pervasive issue in existing Rust is the _sandwich_ problem. It occurs when a
type passed into an operation wants to perform control flow not supported by the
type it's passed into. Thus creating a _sandwich_ [^dilemma] The classic example
is a `map` operation:

[^dilemma]: Not to be confused with the higher-order _sandwich dilemma_ which is
when you look at the sandwich problem and attempt to determine whether the
sandwich is two slices of bread with a topping in between, or two toppings with
a slice of bread in between. Imo the operation part of the problem feels more
_bready_, but that would make for a weird-looking sandwich. Ergo: sandwich
dilemma. (yes, you can ignore all of this.)

```rust
enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    fn map<J>(self, f: impl FnOnce(T) -> J) -> Option<J> { ... }
}

my_option.map(|x| x.await)
```

This will produce a compiler error: the closure `f` is not an async context, so
`.await` cannot be used within it. And we can't just convert the closure to be
`async` either, since `fn map` doesn't know how to call async functions. In
order to solve this issue, we could provide a new `async_map` method which
_does_ provide an async closure. But we may want to repeat those for more
effects, and that would result in a combinatorial explosion of effects. Take for
example "can fail" and "can be async":

|                | not async    | async              |
| -------------- | ------------ | ------------------ |
| __infallible__ | `fn map`     | `fn async_map`     |
| __fallible__   | `fn try_map` | `fn async_try_map` |

That's a lot of API surface for just a single method, and __that problem
multiplies across the entire API surface in the stdlib__. We expect that once we
start applying "keyword generics" to traits, we will be able to solve the
sandwich problem. The type `f` would be marked generic over a set of effects,
and the compiler would choose the right variant during compilation.

## Affecting all effects

Both `const` and `async` share a very similar issue, and we expect that other
"effects" will face the same issue. "fallibility" is particularly on our mind here,
but it isn't the only effect. In order for the language to feel consistent we
need consistent solutions.

## FAQ

### Q: Is there an RFC available to read?

Rust initiatives are intended for _exploration_. The announcement of the Keyword
Generics Initiative marks the _start_ of the exploration process. Part of
exploring is not knowing what the outcomes will be. Right now we're in the
"pre-RFC" phase of design. What we hope we'll achieve is to enumerate the
full problem space, design space, find a balanced solution, and eventually
summarize that in the form of an RFC. Then after the RFC is accepted: implement
it on nightly, work out the kinks, and eventually move to stabilize.  But we may
at any point during this process conclude that this initiative is actually
infeasible and start ramping down.

But while we can't make any _assurances_ about the outcome of the initiative,
what we can share is that we're pretty optimistic about the initiative overall.
We wouldn't be investing the time we are on this if we didn't think we'd be
actually be able to see it through to completion.

### Q: Will this make the language more complicated?

The goal of keyword generics is not to minimize the complexity of the Rust
programming language, but to _minimize the complexity of programming in Rust._
These two might sound similar, but they're not. Our reasoning here is that by
_adding_ a feature, we will actually be able to significantly reduce the surface
area of the stdlib, crates.io libraries, and user code - leading to a more
streamlined user experience.

Choosing between sync or async code is a fundamental choice which needs to be
made. This is complexity which cannot be avoided, and which needs to exist
somewhere. Currently in Rust that complexity is thrust entirely on users of
Rust, making them responsible for choosing whether their code should support
async Rust or not. But other languages have made diferent choices. For example
Go doesn't distinguish between "sync" and "async" code, and has a runtime which
is able to remove that distinction.

In today's Rust application authors choose whether their application will be sync
or async, and even after the introduction of keyword generics we don't really
expect that to change. All generics eventually need to have their types known,
and keyword generics are no different. What we're targeting is the choice made
by _library_ authors whether their library supports is sync or async. With
keyword generics library authors will be able to support both with the help of
the compiler, and leave it up to application authors to decide how they want to
compile their code.

### Q: Are you building an effect system?

The short answer is: kind of, but not really. "Effect systems" or "algebraic
effect systems" generally have a lot of surface area. A common example of what
effects allow you to do is implement your own `try/catch` mechanism. What we're
working on is intentionally limited to built-in keywords only, and wouldn't
allow you to implement anything like that at all.

What we do share with effect systems is that we're integrating modifier keywords
more directly into the type system. Modifier keywords like `async` are often
referred to as "effects", so being able to be conditional over them in
composable ways effectively gives us an "effect algebra". But that's very
different from "generalized effect systems" in other languages.

### Q: Are you looking at other keywords beyond `async` and `const`?

For a while we were referring to the initiative as "modifier generics" or
"modifier keyword generics", but it never really stuck. We're only really
interested in keywords which modify how types work. Right now this is `const`
and `async` because that's what's most relevant for the const-generics WG and
async WG. But we're designing the feature with other keywords in mind as well.

The one most at the top of our mind is a future keyword for fallibility. There
is talk about introducing `try fn() {}` or `fn () -> throws` syntax. This could
make it so methods such as `Iterator::filter` would be able to use `?` to break
out of the closure and short-circuit iteration.

Our main motivation for this feature is that without it, it's easy for Rust to
start to feel _disjointed_. We sometimes joke that Rust is actually 3-5
languages in a trenchcoat. Between const rust, fallible rust, async rust, unsafe
rust - it can be easy for common APIs to only be available in one variant of the
language, but not in others. We hope that with this feature we can start to
systematically close those gaps, leading to a more consistent Rust experience
for _all_ Rust users.

### Q: What will the backwards compatibility story be like?

Rust has pretty strict backwards-compatibility guarantees, and any feature we
implement needs to adhere to this. Luckily we have some wiggle room because of
the edition mechanism, but our goal is to shoot for maximal backwards compat. We
have some ideas of how we're going to make this work though, and we're
cautiously optimistic we might actually be able to pull this off.

But to be frank: this is by far one of the hardest aspects of this feature, and
we're lucky that we're not designing any of this just by ourselves, but have the
support of the language team as well.

### Q: Aren't implementations sometimes fundamentally different?

Const Rust can't make any assumptions about the host it runs on, so it can't do
anything platform-specific. This includes using more efficient instructions of
system calls which are only available in one platform but not another. In order
to work around this, the [`const_eval_select`][ces] intrinsic in the standard
library enables `const` code to detect whether it's executing during CTFE or
runtime, and execute different code based on that.

[ces]: https://doc.rust-lang.org/std/intrinsics/fn.const_eval_select.html

For async we expect to be able to add a similar intrinsic, allowing library
authors to detect whether code is being compiled as sync or async, and do
something different based on that. This includes: using internal concurrency, or
switching to a different set of system calls. We're not sure whether an
intrinsic is the right choice for this though; we may want to provide a more
ergonomic API for this instead. But because keyword generics is being designed
as a consistent feature, we expect that whatever we end up going with can be used
consistently by _all_ modifier keywords.

## Conclusion

In this post we've introduced the new keyword generics initiative, explained why
it exists, and shown a brief example of what it might look like in the future.

The initiative is active on the Rust Zulip under
[`t-lang/keyword-generics`][zulip] - if this seems interesting to you, please
pop by!

_Thanks to everyone who's helped review this post, but in particular:
[fee1-dead][fd], [Daniel Henry-Mantilla][dhm], and [Ryan Levick][rylev]_

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/328082-t-lang.2Fkeyword-generics
[fd]: https://github.com/fee1-dead
[dhm]: https://github.com/danielhenrymantilla
[rylev]: https://github.com/rylev
