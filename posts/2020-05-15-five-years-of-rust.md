---
layout: post
title: "Five Years of Rust"
author: The Rust Core Team
---

With all that's going on in the world you'd be forgiven for forgetting that as
of today, it has been five years since we released 1.0 in 2015! Rust has changed
a lot these past five years, so we wanted reflect back on all of our
contributors' work since the stabilization of the language.

Rust is a general purpose programming language empowering everyone to build
reliable and efficient software. Rust can be built to run anywhere in the stack,
whether as the kernel for your operating system or your next web app. It is built
entirely by an open and diverse community of individuals, primarily volunteers who
generously donate their time and expertise to help make Rust what it is.

## Major Changes since 1.0

#### 2015

**[1.2] — Parallel Codegen:** Compile time improvements are large theme to every
release of Rust, and it's hard to imagine that there was a short time where
Rust had no parallel code generation at all.

**[1.3] — The Rustonomicon:** Our first release of the fantastic "Rustonomicon", a
book that explores Unsafe Rust and its surrounding topics has become a great
resource for anyone looking to learn and understand one of the hardest aspects
of the language.

**[1.4] — Windows MSVC Tier 1 Support:** The first tier 1 platform promotion was
bringing native support for 64-bit Windows using the Microsoft Visual C++ toolchain
(MSVC). Before 1.4 you needed to also have MinGW (a third party GNU environment)
installed in order to use and compile your Rust programs. Rust's Windows support
is one of the biggest improvements these past five years. Just recently
Microsoft [announced a public preview of their official Rust support for the
WinRT API!][winrt] Now it's easier than ever build top quality native and cross
platform apps.

[winrt]: https://blogs.windows.com/windowsdeveloper/2020/04/30/rust-winrt-public-preview/

**[1.5] — Cargo Install:** The addition of being able to build Rust binaries
alongside cargo's pre-existing plugin support has given birth to an entire
ecosystem of apps, utilities, and developer tools that the community has come
to love and depend on. Quite a few of the commands cargo has today were first
plugins that the community built and shared on crates.io!

#### 2016

**[1.6] — Libcore:** `libcore` is a subset of the standard library that only
contains APIs that don't require allocation or operating system level features.
The stabilization of libcore brought the ability to compile Rust with no allocation
or operating system dependency was one of the first major steps towards Rust's
support for embedded systems development.

**[1.10] — C ABI Dynamic Libraries:** The `cdylib` crate type allows Rust to be
compiled as a C dynamic library, enabling you to embed your Rust projects in
any system that supports the C ABI. Some of Rust's biggest success stories
among users is being able to write a small critical part of their system in
Rust and seamlessly integrate in the larger codebase, and it's now easier
than ever.

**[1.12] — Cargo Workspaces:** Workspaces allow you to organise multiple rust
projects and share the same lockfile. Workspaces have been invaluable in
building large multi-crate level projects.

**[1.13] — The Try Operator:** The first major syntax addition was the `?` or
the "Try" operator. The operator allows you to easily propagate your error
through your call stack. Previously you had to use the `try!` macro, which
required you to wrap the entire expression each time you encountered a result,
now you can easily chain methods with `?` instead.

```rust
try!(try!(expression).method()); // Old
expression?.method()?;           // New
```

**[1.14] — Rustup 1.0:** Rustup is Rust's Toolchain manager, it allows you to
seamlessly use any version of Rust or any of its tooling. What started as a
humble shell script has become what the maintainers affectionately call a
*"chimera"*. Being able to provide first class compiler version management across
Linux, macOS, Windows, and the dozens of target platforms would have been a
myth just five years ago.

#### 2017

**[1.15] — Derive Procedural Macros:** Derive Macros allow you to create powerful
and extensive strongly typed APIs without all the boilerplate. This was the
first version of Rust you could use libraries like `serde` or `diesel`'s
derive macros on stable.

**[1.17] — Rustbuild:** One of the biggest improvements for our contributors to
the language was moving our build system from the initial `make` based system
to using cargo. This has opened up `rust-lang/rust` to being a lot easier for
members and newcomers alike to build and contribute to the project.

**[1.20] — Associated Constants:** Previously constants could only be associated
with a module. In 1.20 we stabilised associating constants on struct, enums,
and importantly traits. Making it easier to add rich sets of preset values for
types in your API, such as common IP addresses or interesting numbers.

#### 2018

**[1.24] — Incremental Compilation:** Before 1.24 when you made a change in your
library rustc would have to re-compile all of the code. Now rustc is a lot
smarter about caching as much as possible and only needing to re-generate
what's needed.

**[1.26] — impl Trait:** The addition of `impl Trait` gives you expressive
dynamic APIs with the benefits and performance of static dispatch.

**[1.28] — Global Allocators:** Previously you were restricted to using the
allocator that rust provided. With the global allocator API you can now
customise your allocator to one that suits your needs. This was an important
step in enabling the creation of the `alloc` library, another subset of the
standard library containing only the parts of std that need an allocator like
`Vec` or `String`. Now it's easier than ever to use even more parts of the
standard library on a variety of systems.

**[1.31] — 2018 edition:** The release of the 2018 edition was easily our biggest
release since 1.0, adding a collection of syntax changes and improvements to
writing Rust written in a completely backwards compatible fashion, allowing
libraries built with different editions to seamlessly work together.

- **Non-Lexical Lifetimes** A huge improvement to Rust's borrow checker,
  allowing it to accept more verifiable safe code.
- **Module System Improvements** Large UX improvements to how we define and
  use modules.
- **Const Functions** Const functions allow you to run and evaluate Rust code
  at compile time.
- **Rustfmt 1.0** A new code formatting tool built specifically for Rust.
- **Clippy 1.0** Rust's linter for catching common mistakes. Clippy makes it a lot
  easier to make sure that your code is not only safe but correct.
- **Rustfix** With all the syntax changes, we knew we wanted to provide the
  tooling to make the transition as easy as possible. Now when changes are
  required to Rust's syntax they're just a `cargo fix` away from being resolved.

#### 2019

**[1.34] — Alternative Crate Registries:** As Rust is used more and more in
production, there is a greater need to be able to host and use your projects
in non-public spaces, while cargo has always allowed remote git dependencies,
with Alternative Registries your organisation can easily build and share your
own registry of crates that can be used in your projects like they were
on crates.io.

**[1.39] — Async/Await:** The stabilisation of the async/await keywords for
handling Futures was one of the major milestones to making async programming
in Rust a first class citizen. Even just six months after its release the
async programming has blossomed into a diverse and performant ecosystem.

#### 2020

**[1.42] — Subslice patterns:** While not the biggest change, the addition
  of the `..` (rest) pattern has been a long awaited quality of life
  feature that greatly improves the expressivity of pattern matching
  with slices.

[1.2]: https://blog.rust-lang.org/2015/08/06/Rust-1.2.html
[1.3]: https://blog.rust-lang.org/2015/09/17/Rust-1.3.html
[1.4]: https://blog.rust-lang.org/2015/10/29/Rust-1.4.html
[1.5]: https://blog.rust-lang.org/2015/12/10/Rust-1.5.html
[1.6]: https://blog.rust-lang.org/2016/01/21/Rust-1.6.html
[1.10]: https://blog.rust-lang.org/2016/07/07/Rust-1.10.html
[1.12]: https://blog.rust-lang.org/2016/09/29/Rust-1.12.html
[1.13]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
[1.14]: https://blog.rust-lang.org/2016/12/22/Rust-1.14.html
[1.15]: https://blog.rust-lang.org/2017/02/02/Rust-1.15.html
[1.17]: https://blog.rust-lang.org/2017/04/27/Rust-1.17.html
[1.20]: https://blog.rust-lang.org/2017/08/31/Rust-1.20.html
[1.24]: https://blog.rust-lang.org/2018/02/15/Rust-1.24.html
[1.26]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html
[1.28]: https://blog.rust-lang.org/2018/08/02/Rust-1.28.html
[1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[1.34]: https://blog.rust-lang.org/2019/04/11/Rust-1.34.0.html
[1.39]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html

## Error Diagnostics

One thing that we haven't mentioned much is how much Rust's error messages and
diagnostics have improved since 1.0. Looking at older error messages now feels
like looking at a different language.

We’ve highlighted a couple of examples that best showcase just how much we’ve
improved showing users where they made mistakes and importantly help them
understand why it doesn’t work and teach them how they can fix it.

##### First Example (Traits)
```rust
use std::io::Write;

fn trait_obj(w: &Write) {
    generic(w);
}

fn generic<W: Write>(_w: &W) {}
```


<details>
 <summary>1.2.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (file:///Users/usr/src/rust/error-messages)
src/lib.rs:6:5: 6:12 error: the trait `core::marker::Sized` is not implemented for the type `std::io::Write` [E0277]
src/lib.rs:6     generic(w);
                 ^~~~~~~
src/lib.rs:6:5: 6:12 note: `std::io::Write` does not have a constant size known at compile-time
src/lib.rs:6     generic(w);
                 ^~~~~~~
error: aborting due to previous error
Could not compile `error-messages`.

To learn more, run the command again with --verbose.
```

</details>

![A terminal screenshot of the 1.2.0 error message.][trait-error-1.2.0]

<details>
 <summary>1.43.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (/Users/ep/src/rust/error-messages)
error[E0277]: the size for values of type `dyn std::io::Write` cannot be known at compilation time
 --> src/lib.rs:6:13
  |
6 |     generic(w);
  |             ^ doesn't have a size known at compile-time
...
9 | fn generic<W: Write>(_w: &W) {}
  |    ------- -       - help: consider relaxing the implicit `Sized` restriction: `+  ?Sized`
  |            |
  |            required by this bound in `generic`
  |
  = help: the trait `std::marker::Sized` is not implemented for `dyn std::io::Write`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-messages`.

To learn more, run the command again with --verbose.
```

</details>

![A terminal screenshot of the 1.43.0 error message.][trait-error-1.43.0]

##### Second Example (help)
```rust
fn main() {
    let s = "".to_owned();
    println!("{:?}", s.find("".to_owned()));
}
```

<details>
 <summary>1.2.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (file:///Users/ep/src/rust/error-messages)
src/lib.rs:3:24: 3:43 error: the trait `core::ops::FnMut<(char,)>` is not implemented for the type `collections::string::String` [E0277]
src/lib.rs:3     println!("{:?}", s.find("".to_owned()));
                                    ^~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
src/lib.rs:3:5: 3:45 note: expansion site
src/lib.rs:3:24: 3:43 error: the trait `core::ops::FnOnce<(char,)>` is not implemented for the type `collections::string::String` [E0277]
src/lib.rs:3     println!("{:?}", s.find("".to_owned()));
                                    ^~~~~~~~~~~~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
src/lib.rs:3:5: 3:45 note: expansion site
error: aborting due to 2 previous errors
Could not compile `error-messages`.

To learn more, run the command again with --verbose.

```

</details>

![A terminal screenshot of the 1.2.0 error message.][help-error-1.2.0]

<details>
 <summary>1.43.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (/Users/ep/src/rust/error-messages)
error[E0277]: expected a `std::ops::FnMut<(char,)>` closure, found `std::string::String`
 --> src/lib.rs:3:29
  |
3 |     println!("{:?}", s.find("".to_owned()));
  |                             ^^^^^^^^^^^^^
  |                             |
  |                             expected an implementor of trait `std::str::pattern::Pattern<'_>`
  |                             help: consider borrowing here: `&"".to_owned()`
  |
  = note: the trait bound `std::string::String: std::str::pattern::Pattern<'_>` is not satisfied
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `std::string::String`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-messages`.

To learn more, run the command again with --verbose.
```

</details>

![A terminal screenshot of the 1.43.0 error message.][help-error-1.43.0]

##### Third Example (Borrow checker)
```rust
fn main() {
    let mut x = 7;
    let y = &mut x;

    println!("{} {}", x, y);
}
```

<details>
 <summary>1.2.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (file:///Users/ep/src/rust/error-messages)
src/lib.rs:5:23: 5:24 error: cannot borrow `x` as immutable because it is also borrowed as mutable
src/lib.rs:5     println!("{} {}", x, y);
                                   ^
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
src/lib.rs:5:5: 5:29 note: expansion site
src/lib.rs:3:18: 3:19 note: previous borrow of `x` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `x` until the borrow ends
src/lib.rs:3     let y = &mut x;
                              ^
src/lib.rs:6:2: 6:2 note: previous borrow ends here
src/lib.rs:1 fn main() {
src/lib.rs:2     let mut x = 7;
src/lib.rs:3     let y = &mut x;
src/lib.rs:4
src/lib.rs:5     println!("{} {}", x, y);
src/lib.rs:6 }
             ^
error: aborting due to previous error
Could not compile `error-messages`.

To learn more, run the command again with --verbose.
```

</details>

![A terminal screenshot of the 1.2.0 error message.][borrow-error-1.2.0]

<details>
 <summary>1.43.0 Error Message</summary>

```
   Compiling error-messages v0.1.0 (/Users/ep/src/rust/error-messages)
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
 --> src/lib.rs:5:23
  |
3 |     let y = &mut x;
  |             ------ mutable borrow occurs here
4 |
5 |     println!("{} {}", x, y);
  |                       ^  - mutable borrow later used here
  |                       |
  |                       immutable borrow occurs here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
error: could not compile `error-messages`.

To learn more, run the command again with --verbose.
```

</details>

![A terminal screenshot of the 1.43.0 error message.][borrow-error-1.43.0]

[borrow-error-1.2.0]:  /images/2020-05-15-five-years-of-rust/borrow-error-1.2.0.png
[borrow-error-1.43.0]: /images/2020-05-15-five-years-of-rust/borrow-error-1.43.0.png
[help-error-1.2.0]:    /images/2020-05-15-five-years-of-rust/help-error-1.2.0.png
[help-error-1.43.0]:   /images/2020-05-15-five-years-of-rust/help-error-1.43.0.png
[trait-error-1.2.0]:   /images/2020-05-15-five-years-of-rust/trait-error-1.2.0.png
[trait-error-1.43.0]:  /images/2020-05-15-five-years-of-rust/trait-error-1.43.0.png

## Quotes from the teams
Of course we can't cover every change that has happened. So we reached out and
asked some of our teams what changes they are most proud of:

> For rustdoc, the big things were:
> * The automatically generated documentation for blanket implementations
> * The search itself and its optimizations (last one being to convert it into JSON)
> * The possibility to test more accurately doc code blocks "compile_fail,
>   should_panic, allow_fail"
> * Doc tests are now generated as their own seperate binaries.
>
> — Guillaume Gomez ([rustdoc])


> Rust now has baseline IDE support! Between IntelliJ Rust, RLS and
> rust-analyzer, I feel that most users should be able to find "not horrible"
> experience for their editor of choice. Five years ago, "writing Rust" meant
> using old school Vim/Emacs setup.
>
> — Aleksey Kladov ([IDEs and editors][ides])


> For me that would be: Adding first class support for popular embedded
> architectures and achieving a striving ecosystem to make micro controller
> development with Rust an easy and safe, yet fun experience.
>
> — Daniel Egger ([Embedded WG][ewg])


> The release team has only been around since (roughly) early 2018, but even in
> that time, we've landed ~40000 commits just in rust-lang/rust without any
> significant regressions in stable.
>
> Considering how quickly we're improving the compiler and standard libraries, I
> think that's really impressive (though of course the release team is not the
> sole contributor here). Overall, I've found that the release team has done an
> excellent job of managing to scale to the increasing traffic on issue
> trackers, PRs being filed, etc.
>
> — Mark Rousskov ([Release][release])


> Within the last 3 years we managed to turn [Miri][miri-repo] from an experimental
> interpreter into a practical tool for exploring language design and finding
> bugs in real code—a great combination of PL theory and practice.  On the
> theoretical side we have [Stacked Borrows], the most concrete proposal for a
> Rust aliasing model so far. On the practical side, while initially only a 
> few key libraries were checked in Miri by us, recently we saw a great uptake
> of people using Miri to [find and fix bugs] in their own crates and
> dependencies, and a similar uptake in contributors improving Miri e.g. by
> adding support for file system access, unwinding, and concurrency.
>
> — Ralf Jung ([Miri])

[miri-repo]: https://github.com/rust-lang/miri
[stacked borrows]: https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md
[find and fix bugs]:  https://github.com/rust-lang/miri/#bugs-found-by-miri

> If I had to pick one thing I'm most proud of, it was the work on non-lexical
> lifetimes (NLL). It's not only because I think it made a big difference in
> the usability of Rust, but also because of the way that we implemented it by
> forming the NLL working group. This working group brought in a lot of great
> contributors, many of whom are still working on the compiler today. Open
> source at its best!
>
> — Niko Matsakis ([Language])

[rustdoc]: https://www.rust-lang.org/governance/teams/rustdoc
[ides]: https://www.rust-lang.org/governance/teams/ides
[ewg]: https://www.rust-lang.org/governance/wgs/embedded
[release]: https://www.rust-lang.org/governance/teams/release
[miri]: https://www.rust-lang.org/governance/teams/compiler#miri
[language]: https://www.rust-lang.org/governance/teams/lang


## The Community

As the language has changed and grown a lot in these past five years so has its
community. There's been so many great projects written in Rust, and Rust's
presence in production has grown exponentially. We wanted to share some
statistics on just how much Rust has grown.

- Rust has been voted ["Most Loved Programming"][mlp] every year in the past
  four Stack Overflow developer surveys since it went 1.0.
- We have served over 2.25 Petabytes (1PB = 1,000 TB) of different versions of the
  compiler, tooling, and documentation this year alone!
- In the same time we have served over 170TB of crates to roughly 1.8 billion
  requests on crates.io, doubling the monthly traffic compared to last year.

[mlp]: https://insights.stackoverflow.com/survey/2019#most-loved-dreaded-and-wanted

When Rust turned turned 1.0 you could count the number of companies that were
using it in production on one hand. Today, it is being used by hundreds of
tech companies with some of the largest tech companies such as Apple, Amazon,
Dropbox, Facebook, Google, and Microsoft choosing to use Rust for its performance,
reliability, and productivity in their projects.

## Conclusion
Obviously we couldn't cover every change or improvement to Rust that's happened
since 2015. What have been your favourite changes or new favourite Rust
projects? Feel free to post your answer and discussion on [our
Discourse forum][urlo].

[urlo]: <TODO: CREATE FORUM POST BEFORE MERGE>

Lastly, we wanted to thank everyone who has to contributed to the Rust, whether
you contributed a new feature or fixed a typo, your work has made Rust the
amazing it is today. We can't wait to see how Rust and its community will
continue to grow and change, and see what you all will build with Rust in the
coming decade!
