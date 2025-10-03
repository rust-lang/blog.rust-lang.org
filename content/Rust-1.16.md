+++
path = "2017/03/16/Rust-1.16"
title = "Announcing Rust 1.16"
authors = ["The Rust Core Team"]
aliases = [
    "2017/03/16/Rust-1.16.html",
    "releases/1.16.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.16.0. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed, getting Rust 1.16 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.16.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1160-2017-03-16

### What's in 1.16.0 stable

The largest addition to Rust 1.16 is `cargo check`. This new subcommand should speed up the development
workflow in many cases.

What does it do? Let's take a step back and talk about how `rustc` compiles your code. Compilation has many
"passes", that is, there are many distinct steps that the compiler takes on the road from your source code
to producing the final binary. You can see each of these steps (and how much time and memory they take)
by passing `-Z time-passes` to a nightly compiler:

 ```
 rustc +nightly hello.rs -Z time-passes
time: 0.003; rss: 16MB  parsing
time: 0.000; rss: 16MB  recursion limit
time: 0.000; rss: 16MB  crate injection
time: 0.000; rss: 16MB  plugin loading
time: 0.000; rss: 16MB  plugin registration
time: 0.049; rss: 34MB  expansion
<snip>
 ```

There's a lot of them. However, you can think of this process in two big steps: first, `rustc` does
all of its safety checks, makes sure your syntax is correct, all that stuff. Second, once it's satisfied
that everything is in order, it produces the actual binary code that you end up executing.

It turns out that that second step takes a lot of time. And most of the time, it's not necessary. That is,
when you're working on some Rust code, many developers will get into a workflow like this:

 1. Write some code.
 2. Run `cargo build` to make sure it compiles.
 3. Repeat 1-2 as needed.
 4. Run `cargo test` to make sure your tests pass.
 5. GOTO 1.

In step two, you never actually run your code. You're looking for feedback from the compiler, not to
actually run the binary. `cargo check` supports exactly this use-case: it runs all of the compiler's
checks, but doesn't produce the final binary.

So how much speedup do you actually get? Like most performance related questions, the answer is "it
depends." Here are some very un-scientific benchmarks:

|                 | thanks  | cargo   | diesel |
|-----------------|--------:|--------:|-------:|
| initial build   | 134.75s | 236.78s | 15.27s |
| initial check   | 50.88s  | 148.52s | 12.81s |
| speedup         | 2.648   | 1.594   | 1.192  |
| secondary build | 15.97s  | 64.34s  | 13.54s |
| secondary check | 2.9s    | 9.29s   | 12.3s  |
| speedup         | 5.506   | 6.925   | 1.100  |

The 'initial' categories are the first build after cloning down a project. The 'secondary' categories
involved adding one blank line to the top of `src\lib.rs` and running the command again. That's why
the initial ones are more dramatic; they involve also doing this for all dependencies, as well as
the crate itself. As you can see, larger projects with many dependencies see a big improvement, but
smaller ones see much more modest gains.

We are still working on improving compile-times generally as well, though we don't have anything
in particular to highlight at this time.

#### Other improvements

To support `cargo check`, `rustc` has [learned to emit] a new kind of file: `.rmeta`. This file
will contain only the metadata about a particular crate. `cargo check` needs this for your
dependencies, to let the compiler check types and such from them. It's also useful for the
[Rust Language Server], and possibly more tools in the future.

[learned to emit]: https://github.com/rust-lang/rust/pull/38571
[Rust Language Server]: https://github.com/rust-lang-nursery/rls

Another large change is the removal of a long-standing diagnostic: `consider using an explicit
lifetime parameter`. This diagnostic would kick in whenever you had an incorrect lifetime annotation,
and the compiler thought that you might have meant something else. Consider this code:

```rust
use std::str::FromStr;

pub struct Name<'a> {
    name: &'a str,
}

impl<'a> FromStr for Name<'a> {
    type Err = ();

    fn from_str(s: &str) -> Result<Name, ()> {
        Ok(Name { name: s })
    }
}
```

Here, Rust isn't sure what to do with the lifetimes; as written, the code doesn't guarantee that `s`
will live as long as `Name`, which is required for `Name` to be valid. Let's try to compile this
code with Rust 1.15.1:

```bash
$ rustc +1.15.1 foo.rs --crate-type=lib
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
  --> .\foo.rs:10:5
   |
10 |       fn from_str(s: &str) -> Result<Name, ()> {
   |  _____^ starting here...
11 | |         Ok(Name { name: s })
12 | |     }
   | |_____^ ...ending here
   |
help: consider using an explicit lifetime parameter as shown: fn from_str(s: &'a str) -> Result<Name, ()>
  --> .\foo.rs:10:5
   |
10 |       fn from_str(s: &str) -> Result<Name, ()> {
   |  _____^ starting here...
11 | |         Ok(Name { name: s })
12 | |     }
   | |_____^ ...ending here
```

The compiler explains the issue, and gives a helpful suggestion. So let's try it: modify the code to add in
the `'a`, and compile again:

```bash
$ rustc +1.15.1 .\foo.rs --crate-type=lib
error[E0308]: method not compatible with trait
  --> .\foo.rs:10:5
   |
10 |       fn from_str(s: &'a str) -> Result<Name, ()> {
   |  _____^ starting here...
11 | |         Ok(Name { name: s })
12 | |     }
   | |_____^ ...ending here: lifetime mismatch
   |
<snip>
help: consider using an explicit lifetime parameter as shown: fn from_str(s: &'a str) -> Result<Name<'a>, ()>
  --> .\foo.rs:10:5
   |
10 |       fn from_str(s: &'a str) -> Result<Name, ()> {
   |  _____^ starting here...
11 | |         Ok(Name { name: s })
12 | |     }
   | |_____^ ...ending here
```

It still doesn't work. That help message was not actually helpful. It does suggest adding another
lifetime, this time on `Name`. If we do that...

```bash
$ rustc +1.15.1 .\foo.rs --crate-type=lib
<snip>
help: consider using an explicit lifetime parameter as shown: fn from_str(s: &'a str) -> Result<Name<'a>, ()>
  --> .\foo.rs:10:5
```

... that's what we already have, compiler!

This diagnostic was well-intentioned, but when it's wrong, it was *very* wrong, as you can see here. Sometimes
it wouldn't even suggest valid Rust syntax! Furthermore, more advanced Rust users didn't really need the
suggestion, but new Rustaceans would take them to heart, and then be led down this bad path. As such, we decided
that for now, [we should remove the help message entirely]. We may bring it back in the future, but only if we can
limit false positives.

[we should remove the help message entirely]: https://github.com/rust-lang/rust/pull/37057

> An aside: the above implementation is not possible; `Name` would need to use `String`, not `&str`.

In other diagnostic changes, previous versions of Rust would helpfully attempt to suggest fixes for typos:

```rust
let foo = 5;

println!("{}", ffo);
```

Would give this error:

```
error[E0425]: cannot find value `ffo` in this scope
 --> foo.rs:4:20
  |
4 |     println!("{}", ffo);
  |                    ^^^ did you mean `foo`?
```

However, this would only happen in certain circumstances: sometimes in local variables, and for fields in
structs. [This now happens nearly everywhere]. When combined with [some other related improvements], this
results in a significant improvement in these sorts of diagnostics.

[This now happens nearly everywhere]: https://github.com/rust-lang/rust/pull/38927
[some other related improvements]: https://github.com/rust-lang/rust/pull/38154

See the [detailed release notes][notes] for more.

#### Library stabilizations

21 new bits of API were stabilized this release:

* [`VecDeque::truncate`]
* [`VecDeque::resize`]
* [`String::insert_str`]
* [`Duration::checked_add`]
* [`Duration::checked_sub`]
* [`Duration::checked_div`]
* [`Duration::checked_mul`]
* [`str::replacen`]
* [`str::repeat`]
* [`SocketAddr::is_ipv4`]
* [`SocketAddr::is_ipv6`]
* [`IpAddr::is_ipv4`]
* [`IpAddr::is_ipv6`]
* [`Vec::dedup_by`]
* [`Vec::dedup_by_key`]
* [`Result::unwrap_or_default`]
* [`<*const T>::wrapping_offset`]
* [`<*mut T>::wrapping_offset`]
* `CommandExt::creation_flags`
* [`File::set_permissions`]
* [`String::split_off`]

[`<*const T>::wrapping_offset`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset
[`<*mut T>::wrapping_offset`]: https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset
[`Duration::checked_add`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_add
[`Duration::checked_div`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_div
[`Duration::checked_mul`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_mul
[`Duration::checked_sub`]: https://doc.rust-lang.org/std/time/struct.Duration.html#method.checked_sub
[`File::set_permissions`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.set_permissions
[`IpAddr::is_ipv4`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_ipv4
[`IpAddr::is_ipv6`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_ipv6
[`Result::unwrap_or_default`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default
[`SocketAddr::is_ipv4`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html#method.is_ipv4
[`SocketAddr::is_ipv6`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html#method.is_ipv6
[`String::insert_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.insert_str
[`String::split_off`]: https://doc.rust-lang.org/std/string/struct.String.html#method.split_off
[`Vec::dedup_by_key`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key
[`Vec::dedup_by`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by
[`VecDeque::resize`]:  https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#method.resize
[`VecDeque::truncate`]: https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html#method.truncate
[`str::repeat`]: https://doc.rust-lang.org/std/primitive.str.html#method.repeat
[`str::replacen`]: https://doc.rust-lang.org/std/primitive.str.html#method.replacen

In addition, a number of small improvements to existing functions landed. For example [`writeln!` now has
a single-argument form], just like `println!` has. This ends up writing only a newline, but is a nice bit
of symmetry.

[`writeln!` now has a single-argument form]: https://github.com/rust-lang/rust/pull/38469

All structs in the standard library [now implement `Debug`].

[now implement `Debug`]: https://github.com/rust-lang/rust/pull/38006

When slicing a `&str`, [you'll see better errors]. For example, this code:

```rust
&"abcαβγ"[..4]
```

Is incorrect. It generates this error:

```
thread 'str::test_slice_fail_boundary_1' panicked at 'byte index 4 is not
a char boundary; it is inside 'α' (bytes 3..5) of `abcαβγ`'
```

The part after the `;` is new.

[you'll see better errors]: https://github.com/rust-lang/rust/pull/38066

See the [detailed release notes][notes] for more.

#### Cargo features

In addition to `cargo check`, Cargo and crates.io have some new polish added. For example,
[`cargo build`] and [`cargo doc`] now take a `--all` flag for building and documenting every
crate in your workspace with one command.

[`cargo build`]: https://github.com/rust-lang/cargo/pull/3511
[`cargo doc`]: https://github.com/rust-lang/cargo/pull/3515

Cargo now has a [`--version --verbose` flag], mirroring `rustc`.

[`--version --verbose` flag]: https://github.com/rust-lang/cargo/pull/3604

Crates.io now [can show off your TravisCI or AppVeyor badges] on your crate's page.

[can show off your TravisCI or Appveyor badges]: https://github.com/rust-lang/cargo/pull/3546

In addition, both Cargo and crates.io [understand categories]. Unlike keywords, which are free-form,
categories are curated. In addition, keywords are used for searching, but categories are not. In other
words, categories are intended to assist browsing, and keywords are intended to assist searching.

You can browse crates by category [here](https://crates.io/categories).

[understand categories]: https://github.com/rust-lang/cargo/pull/3301

See the [detailed release notes][notes] for more.

### Contributors to 1.16.0

Last release, we introduced [thanks.rust-lang.org](https://thanks.rust-lang.org).
We have been doing some behind-the-scenes refactoring work to allow for more projects
than only Rust itself; we're hoping to introduce that in the next release.

We had 135 individuals contribute to Rust 1.16.
[Thanks!](https://thanks.rust-lang.org/rust/1.16.0)
