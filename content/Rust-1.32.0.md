+++
path = "2019/01/17/Rust-1.32.0"
title = "Announcing Rust 1.32.0"
authors = ["The Rust Release Team"]
aliases = [
    "2019/01/17/Rust-1.32.0.html",
    "releases/1.32.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.32.0. Rust is a
programming language that is empowering everyone to build reliable and
efficient software.

If you have a previous version of Rust installed via rustup, getting Rust
1.32.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.32.0][notes] on GitHub.

> As a small side note, `rustup` has seen some new releases lately! To update
> `rustup` itself, run `rustup self update`.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1320-2019-01-17

## What's in 1.32.0 stable

Rust 1.32.0 has a few quality of life improvements, switches the default
allocator, and makes additional functions `const`. Read on for a few
highlights, or see the [detailed release notes][notes] for additional
information.

#### The `dbg` macro

First up, a quality of life improvement. Are you a "print debugger"? If you are, and
you've wanted to print out some value while working on some code, you have to do this:

```rust
let x = 5;

println!("{:?}", x);

// or maybe even this
println!("{:#?}", x);
```

This isn't the *largest* speed bump, but it is a lot of stuff to simply show the value of `x`.
Additionally, there's no context here. If you have several of these `println!`s, it can be hard
to tell which is which, unless you add your own context to each invocation, requiring even more work.

In Rust 1.32.0, [we've added a new macro,
`dbg!`](https://github.com/rust-lang/rust/pull/56395/), for this purpose:

```rust
fn main() {
    let x = 5;

    dbg!(x);
}
```

If you run this program, you'll see:

```
[src/main.rs:4] x = 5
```

You get the file and line number of where this was invoked, as well as the
name and value. Additionally, `println!` prints to the standard output, so
you really should be using `eprintln!` to print to standard error. `dbg!`
does the right thing and goes to `stderr`.

It even works in more complex circumstances. Consider this factorial example:

```rust
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        n * factorial(n - 1)
    }
}
```

If we wanted to debug this, we might write it like this with `eprintln!`:

```rust
fn factorial(n: u32) -> u32 {
    eprintln!("n: {}", n);

    if n <= 1 {
        eprintln!("n <= 1");

        n
    } else {
        let n = n * factorial(n - 1);

        eprintln!("n: {}", n);

        n
    }
}
```

We want to log `n` on each iteration, as well as have some kind of context
for each of the branches. We see this output for `factorial(4)`:

```
n: 4
n: 3
n: 2
n: 1
n <= 1
n: 2
n: 6
n: 24
```

This is servicable, but not particularly great. Maybe we could work on how we
print out the context to make it more clear, but now we're not debugging our code,
we're figuring out how to make our debugging code better.

Consider this version using `dbg!`:

```rust
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}
```

We simply wrap each of the various expressions we want to print with the macro. We
get this output instead:

```
[src/main.rs:3] n <= 1 = false
[src/main.rs:3] n <= 1 = false
[src/main.rs:3] n <= 1 = false
[src/main.rs:3] n <= 1 = true
[src/main.rs:4] 1 = 1
[src/main.rs:5] n * factorial(n - 1) = 2
[src/main.rs:5] n * factorial(n - 1) = 6
[src/main.rs:5] n * factorial(n - 1) = 24
[src/main.rs:11] factorial(4) = 24
```

Because the `dbg!` macro returns the value of what it's debugging, instead of
`eprintln!` which returns `()`, we need to make *no* changes to the structure
of our code. Additionally, we have *vastly* more useful output.

That's a lot to say about a little macro, but we hope it improves your
debugging experience! We are contining to work on support for `gdb` and
friends as well, of course.

#### `jemalloc` is removed by default

Long, long ago, Rust had a large, Erlang-like runtime. We chose to use
[jemalloc] instead of the system allocator, because it often improved
performance over the default system one. Over time, we shed more and more of
this runtime, and eventually almost all of it was removed, but jemalloc
was not. We didn't have a way to choose a custom allocator, and so we
couldn't really remove it without causing a regression for people who do need
jemalloc.

Also, saying that `jemalloc` was always the default is a bit UNIX-centric,
as it was only the default on *some* platforms. Notably, the MSVC target on
Windows has shipped the system allocator for a long time.

Finally, while jemalloc *usually* has great performance, that's not always
the case. Additionally, it adds about 300kb to every Rust binary. We've also
had a host of [other
issues](https://github.com/rust-lang/rust/issues/36963#issuecomment-252029017)
with jemalloc in the past. It has also felt a little strange that a systems
language does not default to the system's allocator.

For all of these reasons, once [Rust 1.28 shipped a way to choose a global
allocator](https://blog.rust-lang.org/2018/08/02/Rust-1.28.html#whats-in-1.28.0-stable),
we started making plans to switch the default to the system allocator, and
allow you to use `jemalloc` via a crate. In Rust 1.32, we've finally finished
this work, and by default, you will get the system allocator for your
programs.

If you'd like to continue to use jemalloc, use [the jemallocator crate]. In
your `Cargo.toml`:

```toml
jemallocator = "0.1.8"
```

And in your crate root:

```rust
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
```

That's it! If you don't need jemalloc, it's not forced upon you, and if
you do need it, it's a few lines of code away.

[jemalloc]: http://jemalloc.net/
[the jemallocator crate]: https://crates.io/crates/jemallocator

#### Final module improvements

In the past two releases, we announced several improvements to the module
system. We have one last tweak landing in 1.32.0 and the 2018 edition.
Nicknamed ["uniform
paths"](https://github.com/rust-lang/rust/pull/56759#issuecomment-450051210),
it permits previously invalid import path statements to be resolved exactly
the same way as non-import paths. For example:

```rust
enum Color { Red, Green, Blue }

use Color::*;
```

This code did *not* previously compile, as `use` statements had to start with
`super`, `self`, or `crate`. Now that the compiler supports uniform paths,
this code will work, and do what you probably expect: import the variants of
the `Color` enum defined above the `use` statement.

With this change in place, we've completed our efforts at revising the module
system. We hope you've been enjoying the simplified system so far!


#### Macro improvements

A few improvements to macros have landed in Rust 1.32.0. First, [a new
`literal` matcher](https://github.com/rust-lang/rust/pull/56072/) was added:

```rust
macro_rules! m {
    ($lt:literal) => {};
}

fn main() {
    m!("some string literal");
}
```

`literal` matches against literals of any type; string literals, numeric literals, `char` literals.

In the 2018 edition, `macro_rules` macros can also use `?`, like this:

```rust
macro_rules! bar {
    ($(a)?) => {}
}
```

The `?` will match zero or one repetitions of the pattern, similar to the
already-existing `*` for "zero or more" and `+` for "one or more."

### Library stabilizations

We talked above about the `dbg!` macro, which is a big library addition.
Beyond that, 19 functions were made `const fn`s, and all integral numeric
primitives now provide conversion functions to and from byte-arrays with
specified endianness. These six functions are named `to_<endian>_bytes` and
`from_<endian>_bytes`, where `<endian>` is one of:

* `ne` - native endianness
* `le` - little endian
* `be` - big endian

See the [detailed release notes][notes] for more details.

### Cargo features

Cargo gained [`cargo c` as an alias for `cargo
check`](https://github.com/rust-lang/cargo/pull/6218/), and now [allows
usernames in registry URLs](https://github.com/rust-lang/cargo/pull/6242/).

See the [detailed release notes][notes] for more.

## Contributors to 1.32.0

Many people came together to create Rust 1.32.0. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.32.0)
