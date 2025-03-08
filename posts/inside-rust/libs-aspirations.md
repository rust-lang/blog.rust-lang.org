+++
layout = "post"
date = 2022-04-20
title = "Rust Library Team Aspirations"
author = "Mara Bos"
description = "Rust Library Team Aspirations"
team = "The Rust Library Team <https://www.rust-lang.org/governance/teams/library>"
+++

Over the past years, Rust has grown from a language used by a few dedicated users
into a well-known language used by lots of highly visible projects and
successful companies.
As the Rust user base, the community, and the ecosystem continues to grow,
we need to look forward and consider how we need to scale to adapt
to the needs of the ever expanding Rust ecosystem.

Recently, the compiler team shared [their blog post](https://blog.rust-lang.org/inside-rust/2022/02/22/compiler-team-ambitions-2022.html)
detailing their ambitions for 2022,
and the language team published [a roadmap](https://blog.rust-lang.org/inside-rust/2022/04/04/lang-roadmap-2024.html)
with their vision for Rust in 2024.

In this blog post, we, the Rust library team, will share our perspective
on the future of the Rust standard library and the library ecosystem.

It's important to note that
the role of the team is to coordinate changes and to guide, review and decide.
The majority of the work itself is done by contributors, like yourself,
both in and outside the Rust team.
While we often also participate in design and implementation work,
we do so as contributors, like everyone else.

What follows is an (incomplete) summary of topics we think
are important and would like to coordinate and guide;
things that we'd love to see happen,
as an invitation and source of inspiration.

### Scalability

As mentioned above, the Rust language, standard library, and ecosystem
is growing and getting more mature.
We need to invest in ways to keep evolving these smoothly.

**Evolvability of the standard library and fixing mistakes**

The stability guarantee of Rust makes it hard to evolve the standard library.
Unlike most crates, we cannot release a new major version, as that would effectively be releasing a 'Rust 2.0'.
So, once an API is stable, we have to keep it there forever, meaning that we have to be extremely careful
when stabilizing anything new.

While we are very careful with adding new APIs, mistakes can still happen.
There are a few things we would do differently if we could go back in time, based on current experience.
There are not a lot of these cases, but over time these can still accumulate to the point that
it'd be useful to have a mechanism to correct past mistakes.

The Rust language has the concept of [editions](https://doc.rust-lang.org/edition-guide/editions/index.html)
to be able to make breaking changes in the language itself, without disrupting Rust users.
The library, however, can make very limited use of editions to correct mistakes.
We have used them for the [`panic!()` macro](https://doc.rust-lang.org/edition-guide/rust-2021/panic-macro-consistency.html)
and [the prelude](https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html).
However, in general, it's extremely tricky to make use of the edition mechanism for backwards incompatible
library changes, as crates of different editions can be mixed, while all using the same standard library.

There are also cases when adding a new API can subtly break existing Rust code,
even when not changing any existing API.
This can happen, for example, when a type gets a new method that was already available through
a popular trait, such as `itertools::Itertools` or `num::Integer`.
Adding a method to the standard library can result in an existing method call resolving differently,
potentially breaking code.
This is usually considered 'acceptable' breakage, but as Rust usage grows,
the impact of such breakage increases, often making such breakage unacceptable in practice.

So, in order to keep evolving the standard library, we'd like to collaborate on language features
that alleviate these issues:

- [Edition based method disambiguation](https://github.com/rust-lang/rfcs/pull/3240)
- A way to fix the `Range` types, such that `1..2` can be `Copy`.
- A way to remove or improve lock poisoning without breaking existing code.
- General mechanisms to provide for the library what editions provide for the language.

**People and collaboration**

The most important thing to keep Rust and the ecosystem scalable,
are the people: Rust team members, maintainers of crates in the ecosystem,
reviewers, contributors, and so on.
It's important we keep working on how we collaborate and make it
as easy as possible for everyone to get involved in a way that works for them.

Concretely, we want to work on:

- Better and more complete guidelines for contributors and reviewers; and
- More interaction with the rest of the ecosystem.

**Making `std` less special / Empowering other crates in the ecosystem**

The standard library uses a
[huge amount of unstable language features](https://github.com/rust-lang/rust/issues/94970)
that other crates in the ecosystem cannot (or should not) use.
While this is unavoidable for `core`, because it contains everything related
to Rust's built-in types, we should be able to make `alloc` and `std` less
dependent on unstable features.
Maybe some day these libraries could be no different than any other
popular crate in the ecosystem.

A big part of the work here will be in collaboration with the language team,
to help move the unstable language features we need towards a state where
they can be stabilized.

**Adapting to different platforms**

As Rust's popularity increases, it is used on an increasingly wider variety of platforms.
The Rust standard library does an okay job at abstracting away some of the
differences between popular platforms like Linux and Windows,
through things like `File` and `TcpStream`,
but we don't do a great job for targets that do not look like those,
such as Wasm or kernel modules.

For example, `core` includes `f32` and `f64`, even if the platform doesn't support floating point operations,
and `std` includes `File::open`, even if it isn't implemented and always fails on the specific platform you're targeting.

In order to better support the ever growing diversity of platforms Rust is used on,
we would like to collaborate with the language and compiler teams to make it easier
for the standard library to properly support targets with very different needs,
without it becoming hugely inconvenient for maintainers, contributors, or users:

- Make it easier to port std to a new platform, possibly allowing the relevant code
  to live outside of the `rust-lang/rust` repository for less popular platforms.
- A better way to allow only parts of `std` to be available, depending on the platform.
  For example, a `where Platform: Unix` bound, or something like a [`#[cfg]` portability lint](https://rust-lang.github.io/rfcs/1868-portability-lint.html).
- A way to allow non-portable functionality to be available when on platforms
  that would support it, such as allowing infallible conversion between `u64` and
  `usize` in code that declares it only runs on 64-bit platforms.
- Make the standard library more modular, allowing to disable e.g. floating point support
  or file system support on certain platforms.

### Improving and adding new APIs

A main focus of the library team is and will always be the public interface of the standard library.
As of last year, we even have a separate team to make the final calls for API changes and additions:
the [library API team](https://www.rust-lang.org/governance/teams/library#Library%20API%20team).

Rust purposely has a minimal standard library. Lots of commonly used functionality is
found in other crates in the ecosystem, rather than the standard library.

Where exactly we draw the line between things that should and shouldn't go in the standard library
can be tricky to define and is somewhat flexible, but there are a few categories we're most interested in.

**Ergonomics**

A lot of additions to the standard library are very small ones that increase ergonomics.
Quite often, these are things that were already possible in some way, just not in an ergonomic way.
Some recent examples are:

- `abs_diff()`
- `Path::is_symlink`
- `iter::from_fn`
- `NonZero*::saturating_add`

While we always have to consider the trade-off for niche features to the already large interface
on some types and traits, additions like these continue to happen regularly.

**Standardizing some bigger features the ecosystem needs**

As Rust grows into new territories, there is more and more a need
for certain features to be included in the standard library.
This is especially true for things where a consistent, standard, interface is
important.
Some of the bigger examples are:

- Async traits and functions
- Allocators and fallible allocation
- Error and panic handling
- Portable SIMD
- Benchmarking and custom test/bench frameworks

**Reducing and improving unsafe code**

By providing the right low level APIs and abstractions, we can greatly minimize
the amount of complex unsafe code that users need to write. Tools like
`MaybeUninit` guide users to correct unsafe code that's easy to follow and
prove correct. Even better, some APIs can entirely remove the need for unsafe
code in many situations.
This includes situations where users tend to reach for `unsafe` for performance reasons.

- `std::arch`
- `std::simd`
- Scoped threads
- More atomic primitives
- '`Iterator`' with static length for arrays
- Improving `MaybeUninit` and related methods
- Extending `NonNull` and pointer methods
- A more complete interface to `OsString`, `Path`, and `CString`
- Documentation for `Pin` and other 'unsafe' types
- File descriptors (`OwnedFd`, `AsFd`, etc) and handles (`OwnedHandle`, `AsHandle`, etc)

### Improving implementations of things within the standard library

Historically, the implementation details of the standard library got less attention
than its public API.
Recently, however, we're seeing more and more contributions towards improving the implementation
of various parts of the standard library.

These are some parts that we're especially interested in seeing improvements in:

- `core::fmt` and the implementation of `format_args!()` and `fmt::Arguments`
- [Synchronization primitives like `Mutex`, `RwLock`, and `Condvar`](https://github.com/rust-lang/rust/issues/93740)
- Cleanups in platform-specific code in `std::sys`
- Avoiding allocations wherever possible, [such as when calling functions in `std::fs`](https://github.com/rust-lang/rust/pull/93668)
- Making widely used types [such as `std::io::Error`](https://github.com/rust-lang/rust/pull/87869) more light-weight
- Cleaning up all unnecessary `SeqCst` memory ordering
- Optimizing thread local variables

### Conclusion

We hope this summary provides a healthy amount of inspiration and excitement,
and gives you an idea of what direction the library team is headed.
If you want to help out, whether you want to work on implementation work,
design, documentation, organisation, or any other kind of helpful work,
you're warmly invited to [get involved](https://rust-lang.zulipchat.com/#narrow/stream/219381-t-libs)!
