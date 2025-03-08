+++
layout = "post"
date = 2023-02-23
title = "Keyword Generics Progress Report: February 2023"
author = "Yoshua Wuyts"
team = "The Keyword Generics Initiative <https://github.com/rust-lang/keyword-generics-initiative>"
+++

## Introduction

About 9 months ago [we announced][announce] the creation of the Keyword Generics
Initiative; a group working under the lang team with the intent to solve the 
[function coloring problem][color] [^color] through the type system not just for
`async`, but for `const` and all current and future function modifier keywords
as well.

We're happy to share that we've made a lot of progress over these last several
months, and we're finally ready to start putting some of our designs forward through
RFCs. Because it's been a while since our last update, and because we're excited
to share what we've been working on, in this post we'll be going over some of the things
we're planning to propose.

[announce]: https://blog.rust-lang.org/inside-rust/2022/07/27/keyword-generics.html
[color]: https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/

[^color]: To briefly recap this problem: you can't call an `async fn` from a
non-async fn. This makes the "async" notation go viral, as every function that
calls it also needs to be async. But we believe possibly more importantly: it
requires a duplication of most stdlib types and ecosystem libraries. Instead we
suspected we might be able to overcome this issue by introducing a new kind of
generic which would enable functions and types to be "generic" over whether
they're async or not, const or not, etc.

## An async example

In our [previous post][announce] we introduced the placeholder `async<A>` syntax to describe the
concept of a "function which is generic over its asyncness". We always knew we
wanted something that felt lighter weight than that, so in for our current design
we've chosen to drop the notion of a generic parameter for the end-user syntax,
and instead picked the `?async` notation. We've borrowed this from the trait
system, where for example `+ ?Sized` indicates that something may or may not
implement the `Sized` trait. Similarly `?async` means a function may or may not be
async. We also refer to these as "maybe-async" functions.

Time for an example. Say we took the [`Read` trait][read] and the
[read_to_string_methods][rts]. In the stdlib their implementations look somewhat
like this today:

```rust
trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... }
}

/// Read from a reader into a string.
fn read_to_string(reader: &mut impl Read) -> std::io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    Ok(string)
}
```

Now, what if we wanted to make these async in the future? Using `?async`
notation we could change them to look like this:

```rust
trait ?async Read {
    ?async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    ?async fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... }
}

/// Read from a reader into a string.
?async fn read_to_string(reader: &mut impl ?async Read) -> std::io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    Ok(string)
}
```

The way this would work is that `Read` and `read_to_string` would become generic over
their "asyncness". When compiled for an `async` context, they will behave
asynchronously. When compiled in a non-async context, they will behave
synchronously. The `.await` in the `read_to_string` function body is necessary
to mark the cancellation point in case the function is compiled as async; but
when not async would essentially become a no-op [^always-async-maybe]:

[^always-async-maybe]: One restriction `?async` contexts have is that they can
only call other `?async` and non-`async` functions. Because if we could call an
always-`async` function, there would be no clear right thing to do when compiled
in non-async mode. So things like async concurrency operations won't directly
work in always-async contexts. But we have a way out of this we talk about later
in the post: `if is_async() .. else ..`. This allows you to branch the body of a
`?async fn` based on which mode it's being compiled in, and will allow you to
write different logic for async and non-async modes. This means you can choose
to use async concurrency in the async version, but keep things sequential in the
non-async version.

```rust
// `read_to_string` is inferred to be `!async` because
// we didn't `.await` it, nor expected a future of any kind.
#[test]
fn sync_call() {
    let _string = read_to_string("file.txt")?;
}

// `read_to_string` is inferred to be `async` because
// we `.await`ed it.
#[async_std::test]
async fn async_call() {
    let _string = read_to_string("file.txt").await?;
}
```

We expect `?async` notation would be most useful for library code which doesn't
do anything particularly specific to async Rust. Think: most of the stdlib, and
ecosystem libraries such as parsers, encoders, and drivers. We expect most
applications to choose to be compiled either as async or non-async, making them
mostly a consumer of `?async` APIs.

## A const example

A main driver of the keywords generics initiative has been our desire to make the
different modifier keywords in Rust feel consistent with one another. Both the
const WG and the async WG were thinking about introducing keyword-traits at the
same time, and we figured we should probably start talking with each other to make
sure that what we were going to introduce felt like it was part of the same
language - and could be extended to support more keywords in the future.

So with that in mind, it may be unsurprising that for the maybe-`const` trait
bounds and declarations we're going to propose using the `?const` notation.
A common source of confusion with `const fn` is that it actually doesn't
guarantee compile-time execution; it only means that it's *possible* to evaluate
in a `const` compile-time context. So in a way `const fn` has always been a way
of declaring a "maybe-const" function, and there isn't a way to declare an
"always-const" function. More on that later in this post.

Taking the `Read` example we used earlier, we could imagine a "maybe-const" version
of the `Read` trait to look very similar:

```rust
trait ?const Read {
    ?const fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    ?const fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { ... }
}
```

Which we could then use use as a bound in the const `read_to_string` function,
like this:

```rust
const fn read_to_string(reader: &mut impl ?const Read) -> std::io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    Ok(string)
}
```

Just like with `?async` traits, `?const` traits would also need to be labeled as
`?const` when used as a bound. This is important to surface at the trait level,
because it's allowed to pass non-const bounds to maybe-const functions, as long
as no trait methods are called in the function body. This means we need to
distinguish between "never-const" and "maybe-const".

You may have noticed the `?const` on the trait declaration and the extra
`?const` on the trait methods. This is on purpose: it keeps the path open to
potentially add support for "always-const" or "never-const" methods on traits as
well. In `?async` we know that even if the entire trait is `?async`, some
methods (such as `Iterator::size_hint`) will never be async. And this would
make `?const` and `?async` traits behave similarly using the same rules.

[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[rts]: https://doc.rust-lang.org/std/io/fn.read_to_string.html

## Combining const and async

We've covered `?async`, and we've covered `?const`. Now what happens if we were
to use them together? Let's take a look at what the `Read` trait would look like
when if we extended it using our designs for `?const` and `?async`:

```rust
trait ?const ?async Read {
    ?const ?async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    ?const ?async fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { .. }
}

/// Read from a reader into a string.
const ?async fn read_to_string(reader: &mut impl ?const ?async Read) -> io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    Ok(string)
}
```

That's sure starting to feel like a lot of keywords, right? We've accurately
described exactly what's going on, but there's a lot of repetition. We know that
if we're dealing with a `const ?async fn`, most arguments probably will
want to be `?const ?async`. But under the syntax rules we've proposed so far,
you'd end up repeating that everywhere. And it probably gets worse once we start
adding in more keywords. Not ideal!

So we're very eager to make sure that we find a solution to this. And we've been
thinking about a way we could get out of this, which we've been calling
`effect/.do`-notation. This would allow you to mark a function as "generic over
all modifier keywords" by annotating it as `effect fn`, and it would allow the
compiler to insert all the right `.await`, `?`, and `yield` keywords in the
function body by suffixing function calls with `.do`.

Just to set some expectations: this is the least developed part of our proposal,
and we don't intend to formally propose this until after we're done with some of
the other proposals. But we think it's an important part of the entire vision,
so we wanted to make sure we shared it here. And with that out of the way,
here's the same example we had above, but this time using the `effect/.do`-notation:

```rust
trait ?effect Read {
    ?effect fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    ?effect fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { .. }
}

/// Read from a reader into a string.
?effect fn read_to_string(reader: &mut impl ?effect Read) -> std::io::Result<String> {
    let mut string = String::new();
    reader.read_to_string(&mut string).do;  // note the singular `.do` here
    string
}
```

One of the things we would like to figure out as part of `effect/.do` is a way
to enable writing conditional effect-bounds. For example: there may be a
function which is always async, may never panic, and is generic over the
remainder of the effects. Or like we're seeing with APIs such as
[`Vec::reserve`] and [`Vec::try_reserve`]: the ability to panic xor return an
error. This will take more time and research to figure out, but we believe it
is something which can be solved.

[`Vec::reserve`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve
[`Vec::try_reserve`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve

## Adding support for types

Something we're keen on doing is not just adding support for `?async` and to
apply to functions, traits, and trait bounds. We would like `?async` to be
possible to use with types as well. This would enable the ecosystem to stop
having to provide both sync and async versions of crates. It would also enable
the stdlib to gradually "asyncify" just like we have been with const.

The challenge with async types, especially in the stdlib, is that their behavior
will often have to be different when used in async and non-async contexts. At
the very lowest level async system calls work a bit differently from non-async
system calls. But we think we may have a solution for that too in the form of
the `is_async` compiler built-in method.

Say we wanted to implement `?async File` with a single `?async open` method. The
way we expect this to look will be something like this:

```rust
/// A file which may or may not be async
struct ?async File {
    file_descriptor: std::os::RawFd,  // shared field in all contexts
    async waker: Waker,               // field only available in async contexts
    !async meta: Metadata,            // field only available in non-async contexts
}

impl ?async File {
    /// Attempts to open a file in read-only mode.
    ?async fn open(path: Path) -> io::Result<Self> {
        if is_async() {   // compiler built-in function
            // create an async `File` here; can use `.await`
        } else {
            // create a non-async `File` here
        }
    }
}
```

This would enable authors to use different fields depending on whether they're
compiling for async or not, while still sharing a common core. And within
function bodies it would be possible to provide different behaviors depending on
the context as well. The function body notation would work as a generalization
of the currently unstable [`const_eval_select`][eval-select] intrinsic, and at
least for the function bodies we expect a similar `is_const()` compiler built-in
to be made available as well.

[eval-select]: https://doc.rust-lang.org/std/intrinsics/fn.const_eval_select.html
[connect]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect

## Consistent syntax

As we alluded to earlier in the post: one of the biggest challenges we see in
language design is adding features in a way that makes them feel like they're in
harmony with the rest of the language - and not something which stands out as
noticably different. And because we're touching on something core to Rust, the
way we do keywords, we have to pay extra close attention here to make sure Rust
keeps feeling like a single language.

Luckily Rust has the ability to make surface-level changes to the
language through the edition system. There are many things this doesn't let us
do, but it does allow us to require syntax changes. A possibility we're
exploring is leveraging the edition system to make some minor changes to `const`
and `async` so they feel more consistent with one another, and with `?const` and
`?async`.

For `const` this means there should be a syntactic distinction between `const`
declarations and `const` uses. Like we mentioned earlier in the post, when you
write `const fn` you get a function which can be evaluated both at runtime and
during compilation. But when you write `const FOO: () = ..;` the meaning of
`const` there guarantees compile-time evaluation. One keyword, different
meanings. So for that reason we're wondering whether perhaps it would make more
sense if we changed `const fn` to `?const fn`.  This would make it clear that
it's a function which *may* be const-evaluated, but doesn't necessarily have to -
and can also be called from non-`const` contexts.

```rust
//! Define a function which may be evaluated both at runtime and during
//! compilation.

// Current
const fn meow() -> String { .. }

// Proposed
?const fn meow() -> String { .. }
```

For `async` we're considering some similar surface-level changes.  The Async WG
is in the process of expanding the "async functions in traits" design into an
design covering "async traits" entirely, largely motivated by the desire to be
able to add `+ Send` bound to anonymous futures. There are more details about
this in ["Lightweight, Predictable Async Send Bounds"][bounds-post] by Eric
Holk. But it would roughly become the following notation:

[bounds-post]: https://blog.theincredibleholk.org/blog/2023/02/16/lightweight-predictable-async-send-bounds/

```rust
struct File { .. }
impl async Read for File {                                                // async trait declaration
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { .. }  // async method
}

async fn read_to_string(reader: &mut impl async Read) -> io::Result<String> { // async trait bound
    let mut string = String::new();
    reader.read_to_string(&mut string).await?;
    Ok(string)
}
```

This would make `impl ?async Read` and `impl async Read` consistent with each
other. And it would open the door for `trait ?async` traits to be passed to
`impl async Read` and be guaranteed to be always interpreted as `trait async`.
Which is another nice consistency gain.

The final thing we're looking at is `async`-notation for types. To implement
inherent `?async` methods on types, our current design requires the type to also
be marked as `?async`. In order to bring `?async` and `async` closer together,
we're exploring whether it might also make sense to require types to be marked
as `async` as well:

```rust
//! Proposed: define a method on a maybe-async type
struct ?async File { .. }
impl ?async File {
    ?async fn open(path: PathBuf) -> io::Result<Self> { .. }
}

//! Current: define a method on an always-async type
struct File { .. }
impl File {
    async fn open(path: PathBuf) -> io::Result<Self> { .. }
}

//! Proposed: define a method on an always-async type
struct async File { .. }
impl async File {
    async fn open(path: PathBuf) -> io::Result<Self> { .. }
}
```

We already have something similar going on for "always-const" arguments via the
const-generics system. These look something like this:

```rust
fn foo<const N: usize>() {}
```

Every "always-const" argument to the function must always be marked by `const`,
so it wouldn't be entirely without precedent for every "always-async" type to
always require to be marked using `async`. So we're exploring some of what might
be possible here.

## The tentative plan

We plan to initially focus our efforts on the `async` and `const` keywords only.
We're feeling ready to start converting some of our designs into RFCs, and start
putting them out for review. In the coming months we expect to start writing
the following proposals (in no particular order):

- `?async fn` notation without trait bounds, including an `is_async` mechanism.
- `trait async`  declarations and bounds.
- `trait ?async` declarations and bounds, `trait ?const` declarations and bounds.
- `?const fn` notation without trait bounds.
- `struct async` notation and `struct ?const` notation.

We'll be working closely with the Lang Team, Const WG, and Async WG on these
proposals, and in some cases (such as `trait async`) we may even take an
advising role with a WG directly driving the RFC. As usual, these will be going
through the RFC-nightly-stabilization cycle. And only once we're fully confident
in them will they become available on stable Rust.

We're intentionally not yet including `effect/.do` notation on this roadmap. We
expect to only be able to start this work once we have `?async` on nightly,
which we don't yet have. So for now we'll continue work on designing it within
the initiative, and hold off on making plans to introduce it quiet yet.

## Conclusion

And that concludes the 9-month progress report of the Keyword Generics
Initiative. We hope to be able to provide more exact details about things such
as desugarings, semantics, and alternatives in the RFCs. We're pretty stoked with the
progress we've made in these past few months! Something which I don't think
we've mentioned yet, but is probably good to share: we've actually prototyped
much of the work in this post already; so we're feeling fairly confident all of
this may actually *actually* work. And that is something we're
incredibly excited for!
