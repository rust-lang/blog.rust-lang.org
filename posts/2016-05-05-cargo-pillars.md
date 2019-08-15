---
layout: post
title: "Cargo: predictable dependency management"
author: Yehuda Katz
description: "Cargo makes dependency management in Rust easy and predictable"
---

Cargo's goal is to make modern application package management a core value of
the Rust programming language.

In practice, this goal translates to being able to build a new browser engine
like [Servo](https://github.com/servo/servo) out of 247 community-driven
libraries&mdash;and counting. Servo's build system is a thin wrapper around
Cargo, and after a fresh checkout, you're only one command away from seeing the
whole dependency graph built:

```
   Compiling num-complex v0.1.32
   Compiling bitflags v0.6.0
   Compiling angle v0.1.0 (https://github.com/emilio/angle?branch=servo#eefe3506)
   Compiling backtrace v0.2.1
   Compiling smallvec v0.1.5
   Compiling browserhtml v0.1.4 (https://github.com/browserhtml/browserhtml?branch=gh-pages#0ca50842)
   Compiling unicase v1.4.0
   Compiling fnv v1.0.2
   Compiling heapsize_plugin v0.1.4
   ...
```

Why do these granular dependencies matter?

Concretely, they mean that Servo's URL library (and many components like
it) is not a deeply nested part of Servo's main tree, but rather an
[external library](https://crates.io/crates/url) that anyone in the
ecosystem can use. This makes it possible for other Rust libraries, like web
frameworks, to easily use a browser-grade URL library, sharing the costs
and benefits of maintenance. And it flows both ways: recently, a new
[Rust-based text editor](https://github.com/google/xi-editor) was announced,
and happened to provide a fast line-breaking library. Within days, that library
[replaced Servo's old custom linebreaker](https://twitter.com/mbrubeck/status/726791246014877696),
decreasing Servo's maintenance burden and increasing sharing in the Rust
ecosystem.

## The core concerns of dependency management

To make this all work at the scale of an app like Servo, you need a dependency
management approach with good answers to a number of thorny questions:

1. How easy is it to add an external library, like a new linebreaker, to Servo?

2. If I build Servo on a different machine, for a different architecture,
   in CI or for release, am I building from the same source code?

3. If I build Servo for testing, will its indirect dependencies be compiled
   with debug symbols? If I build Servo for release, will its indirect
   dependencies be compiled with maximum optimizations? How can I be sure?

4. If someone published a new version of one of Servo's dependencies after I
   commit to Servo, will my CI environment use the same source code as my
   machine? My production environment?

5. If I add a new dependency (or upgrade one), can that break the build? Can it
   affect unrelated dependencies? Under what conditions?

**All of these questions (and many more like them) have one thing in common:
predictability**. One solution to this problem, common in the systems space, is
vendoring dependencies&mdash;forking them directly into an application's
repository&mdash;and then managing them manually. But this comes at a
substantial per-project cost, since there's more to manage and configure. It
also comes at an ecosystem-wide cost, since the work involved cannot easily be
shared between libraries; it has to be redone instead for each application that
brings a set of libraries together. And making sure you can answer all of the
questions above, all of the time, is hard work indeed.

Package managers for higher-level languages have shown that by turning
dependency management over to a shared tool, you can have predictability, easy
workflows that operate over the entire dependency graph, and increased sharing
and robustness across the ecosystem. When we started planning Rust 1.0, we knew
we wanted to bring these ideas to a systems setting, and making Cargo a central
part of the way people use Rust was a big part of that.

## Pillars of Cargo

Cargo is built on three major pillars:

1. Building, testing, and running projects should be predictable across
   environments and over time.

2. To the extent possible, indirect dependencies should be invisible to
   application authors.

3. Cargo should provide a shared workflow for the Rust ecosystem that aids the
   first two goals.

We'll look at each of these pillars in turn.

## Predictability

Cargo's predictability goals start with a simple guarantee: **once a project
successfully compiles on one machine, subsequent compiles across machines and
environments will use exactly the same source code**.

This guarantee is accomplished without incorporating the source code for
dependencies directly into a project repository. Instead, Cargo uses several
strategies:

1. The first time a build succeeds, Cargo emits a `Cargo.lock` file, which
   contains a manifest of precisely which source code was used in the
   build. (more on "precise" in a bit)

2. Cargo manages the entire workflow, from running tests and benchmarks, to
   building release artifacts, to running executables for debugging. This allows
   Cargo to ensure that all dependencies (direct and indirect) are downloaded
   and properly configured for these use-cases without the user having to do
   anything extra.

3. Cargo standardizes important environment configuration, like optimization
   level, static and dynamic linking, and architecture. Combined with the
   `Cargo.lock`, this makes the results of building, testing and executing Cargo
   projects highly predictable.

### Predictability By Example

To illustrate these strategies, let's build an example crate using Cargo. To
keep things simple, we'll create a small `datetime` crate that exposes date and
time functionality.

First, we'll use `cargo new` to start us out:

```
$ cargo new datetime
$ cd datetime
$ ls
Cargo.toml src
$ cat Cargo.toml
[package]
name = "datetime"
version = "0.1.0"
authors = ["Yehuda Katz <wycats@gmail.com>"]

[dependencies]
```

We don't want to build the date or time functionality from scratch, so let's
edit the `Cargo.toml` and add the `time` crate from [crates.io](https://crates.io/):

```diff
  [package]
  name = "datetime"
  version = "0.1.0"
  authors = ["Yehuda Katz <wycats@gmail.com>"]

  [dependencies]
+ time = "0.1.35"
```

Now that we've added the `time` crate, let's see what happens if we ask Cargo to
build our package:

```console
cargo build
   Compiling winapi v0.2.6
   Compiling libc v0.2.10
   Compiling winapi-build v0.1.1
   Compiling kernel32-sys v0.2.2
   Compiling time v0.1.35
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
```

Whoa! That's a lot of crates. **The biggest part of Cargo's job is to provide
enough predictability to allow functionality like the `time` crate to be broken
up into smaller crates that do one thing and do it well**.

Now that we successfully built our crate, what happens if we try to build it again?

```console
cargo build
```

Nothing happened at all. Why's that? We can always ask Cargo to give us more
information through the `--verbose` flag, so let's do that:

```console
cargo build --verbose
       Fresh libc v0.2.10
       Fresh winapi v0.2.6
       Fresh winapi-build v0.1.1
       Fresh kernel32-sys v0.2.2
       Fresh time v0.1.35
       Fresh datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
```

Cargo isn't bothering to recompile packages that it knows are "fresh", like
`make`, but without having to write the `Makefile`.

But how does Cargo know that everything is fresh? When Cargo builds a crate, it
emits a file called `Cargo.lock` that contains the precise versions of all of
its resolved dependencies:

```toml
[root]
name = "datetime"
version = "0.1.0"
dependencies = [
 "libc 0.2.10 (registry+https://github.com/rust-lang/crates.io-index)",
 "time 0.1.35 (registry+https://github.com/rust-lang/crates.io-index)",
]

[[package]]
name = "kernel32-sys"
version = "0.2.2"
source = "registry+https://github.com/rust-lang/crates.io-index"
dependencies = [
 "winapi 0.2.6 (registry+https://github.com/rust-lang/crates.io-index)",
 "winapi-build 0.1.1 (registry+https://github.com/rust-lang/crates.io-index)",
]

...
```

The `Cargo.lock` contains a serialized version of the entire **resolved**
dependency graph, including precise versions of all of the source code included
in the build. In the case of a package from crates.io, Cargo stores the name and
version of the dependency. This is enough information to uniquely identify
source code from [crates.io](https://crates.io/), because the registry is
*append only* (no changes to already-published packages are allowed).

In addition, the metadata for the registry is stored in a
[separate git repository](https://github.com/rust-lang/crates.io-index/), and
includes checksum for the relevant package. Before Cargo ever unpacks a crate it
downloads, it first validates the checksum.

### Collaborating

Now for the real test. Let's push our code up to GitHub and develop it on a
different machine. Ideally, we would like to be able to pick up right where we
left off, with the exact same source code for all of our dependencies.

To do this, we check in our `Cargo.lock` and clone the repository on our new
machine. Then, we run `cargo build` again.

```console
cargo build
   Compiling libc v0.2.10
   Compiling winapi v0.2.6
   Compiling winapi-build v0.1.1
   Compiling kernel32-sys v0.2.2
   Compiling time v0.1.35
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
```

As expected, **because we checked in our `Cargo.lock` we get exactly the same
versions of all dependencies as before**. And if we wanted to start collaborating
with other developers on GitHub (or with other members of our team at work), we
would continue to get the same level of predictability.

### Common conventions: examples, tests, and docs

Now that we've written our snazzy new `datetime` crate, we'd love to write an
example to show other developers how it should be used. We create a new file
called `examples/date.rs` that looks like this:

```
extern crate datetime;

fn main() {
    println!("{}", datetime::DateTime::now());
}
```

To run the example, we ask Cargo to build and run it:

```console
cargo run --example date
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
     Running `target/debug/examples/date`
26 Apr 2016 :: 05:03:38
```

Because we put our code in the conventional location for examples, Cargo knew
how to do the right thing, no sweat.

In addition, once you start writing a few tests, `cargo test` will automatically
build your examples as well, which prevents them from getting out of sync with
your code, and ensures they continue to compile as long as your tests are
passing.

Similarly, the `cargo doc` command will automatically compile not just your
code, but that of your dependencies as well. The upshot is that the API docs it
automatically produces include the crates you depend on, so if your APIs mention
types from those crates, your clients can follow those links.

These are just a few examples of a general point: **Cargo defines a common set of
conventions and workflows that operate precisely the same way across the entire
Rust ecosystem**.

### Updating

All of this means that your application won't change if you don't make any
changes to your dependencies, but what happens when you need to change them?

Cargo adds another layer of protection with *conservative updates*. This means
that if you modify your `Cargo.toml`, Cargo attempts to minimize the changes
made to the `Cargo.lock`. The intuition of conservative updates is: **if the
change you made was unrelated to another dependency, it shouldn't change**.

Let's say that after developing the library for a little while, we decide that
we want to add support for time zones. First, let's add in the `tz` dependency
to our package:

```
  [package]
  name = "datetime"
  version = "0.1.0"
  authors = ["Yehuda Katz <wycats@gmail.com>"]

  [dependencies]
  time = "0.1.35"
+ tz = "0.2.1"
```

After using the crate in our library, let's run `cargo build` again:

```console
cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading tz v0.2.1
 Downloading byteorder v0.5.1
   Compiling byteorder v0.5.1
   Compiling tz v0.2.1
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
```

Cargo downloaded `tz` (and its dependency `byteorder`) and compiled them, but it
didn't touch the packages we were already using (`kernel32-sys`, `libc`,
`time`, `winapi` and `winapi-build`). Even if one of those package authors
published an update in the meantime, you can be sure that adding new crates
won't mess with unrelated ones.

Conservative updates attempt to significantly reduce unexpected changes to your
source code. It stands in stark contrast to 'rebuild the world', which allows a
small change to dependencies to rebuild the entire graph, wreaking havoc in its
wake.

**As a rule, Cargo attempts to minimize the effects of intentional changes to
  direct dependencies.**

## Indirect Dependencies "Just Work"

One of the most basic goals of an application package manager is separating
direct dependencies, which are required by the application, and indirect
dependencies, which those dependencies need in order to work.

As we've seen in the `datetime` crate we built, we only needed to specify
dependencies on `time` and `tz`, and Cargo automatically created the entire
graph of dependencies needed to make that work. It also serialized that graph
for future predictability.

Since Cargo manages your dependencies for you, it can also make sure that it
compiles all of your dependencies (whether you knew about them directly or not)
appropriately for the task at hand.

### Testing, Benchmarking, Releasing, Oh My

Historically, people have shied away from the kinds of granular dependencies
we've seen here because of the configuration needed for each new dependency.

For example, when running tests or type-checking your code, you would like to
compile the code as fast as possible to keep the feedback loop fast. On the
other hand, when benchmarking or releasing your code, you are willing to spend
plenty of time waiting for the compiler to optimize your code if it produces a
fast binary.

It's important to compile not only your own code or your direct dependencies,
but all indirect dependencies with the same configuration.

Cargo manages that process for you automatically. Let's add a benchmark to our
code:

```
#[bench]
fn bench_date(b: &mut Bencher) {
    b.iter(|| DateTime::now());
}
```
If we then run `cargo bench`:

```console
cargo bench
   Compiling winapi v0.2.6
   Compiling libc v0.2.10
   Compiling byteorder v0.5.1
   Compiling winapi-build v0.1.1
   Compiling kernel32-sys v0.2.2
   Compiling tz v0.2.1
   Compiling time v0.1.35
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
     Running target/release/datetime-2602656fcee02e68

running 1 test
test bench_date ... bench:         486 ns/iter (+/- 56)
```

Notice that we're re-compiling all of our dependencies. This is because `cargo
bench` defaults to release mode, which uses maximum optimizations. `cargo build
--release` similarly builds in optimized mode by default.

> As an aside, the default behavior of each command is configurable through
> [profiles](http://doc.crates.io/manifest.html#the-profile-sections) in the
> `Cargo.toml`. This allows you to configure things like the optimization level,
> whether to include debug symbols and more. Rather than forcing you to use a
> custom workflow if something doesn't precisely meet your needs, the profiles
> feature allows you to customize the existing workflows and stay within Cargo's
> flows.

### Platforms and Architectures

Similarly, applications are often built for different architectures, operating
systems, or even operating system version. They can be compiled for maximum
portability or to make maximum use of available platform features.

Libraries can be compiled as static libraries or dynamic libraries. And even
static libraries might want to do some dynamic linking (for example, against the
system version of `openssl`).

**By standardizing what it means to build and configure a package, Cargo can apply
all of these configuration choices to your direct dependencies *and* indirect
dependencies**.

### Shared Dependencies

So far, we've looked at packages and their dependencies. But what if two
packages that your application depends on share a third dependency?

For example, let's say that I decide to add the `nix` crate to my `datetime`
library for Unix-specific functionality.

```
  [package]
  name = "datetime"
  version = "0.1.0"
  authors = ["Yehuda Katz <wycats@gmail.com>"]

  [dependencies]
  time = "0.1.35"
  tz = "0.2.1"
+ nix = "0.5.0"
```

As before, when I run `cargo build`, Cargo *conservatively* adds `nix` and its dependencies:

```console
cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading nix v0.5.0
 Downloading bitflags v0.4.0
   Compiling bitflags v0.4.0
   Compiling nix v0.5.0
   Compiling datetime v0.1.0 (file:///Users/ykatz/Code/datetime)
```

But if we look a little closer, we'll notice that `nix` has a dependency on
`bitflags` *and* `libc`. It now *shares* the dependency on `libc` with the
`date` package.

If my `datetime` crate gets `libc` types from `time` and hands them off to
`nix`, they better be the same `libc` or my program won't compile (and we
wouldn't want it to!)

Today, Cargo will automatically share dependencies between crates if they depend
on the same *major* version (or minor version before 1.0), since Rust uses
[semantic versioning](http://semver.org/). This means that if `nix` and `datetime`
both depend on some version of `libc 0.2.x`, they will get the same version. In
this case, they do, and the program compiles.

While this policy works well (and in fact is the same policy that system package
managers use), it doesn't always do exactly what people expect, especially when
it comes to coordinating a major version bump across the ecosystem. (In many
cases, it would be preferable for Cargo to hard-error than assume that a
dependency on `0.2.x` is simply unrelated to another dependency on `0.3.x`.)

This problem is especially pernicious when multiple major versions of the same
package expose global symbols (using `#[no_mangle]` for example, or by including
other statically linked C libraries).

We have some thoughts on ways to improve Cargo to handle these cases better,
including the ability for a package to more explicitly express when a dependency
is used purely internally and is not shared through its public interface. Those
packages could be more readily duplicated if needed, while dependencies that are
used in a package's public interface must not be.

You should expect to see more on this topic in the months ahead.

## Workflow

As we've seen, **Cargo is not just a dependency manager, but Rust's primary
workflow tool**.

This allows Rust to have a rich ecosystem of interconnected dependencies,
eliminating the need for applications to manually manage large dependency
graphs. Applications can benefit from a vibrant ecosystem of small packages that
do one thing and do it well, and let Cargo handle the heavy lifting of keeping
everything up to date and compiling correctly.

Even a small crate like the `datetime` example we built has a few dependencies
on small, targeted crates, and each of those crates has some dependencies of its
own.

By defining shared, well-known workflows, like "build", "test", "bench", "run",
and "doc", Cargo provides Rust programmers with a way to think about what
they're trying to accomplish at a high level, and not have to worry about what
each of those workflows mean for indirect dependencies.

This allows us to get closer to the holy grail of making those indirect
dependency graphs "invisible", empowering individuals to do more on their hobby
projects, small teams to do more on their products, and large teams to have a
high degree of confidence in the output of their work.

With a workflow tool that provides predictability, even in the face of many
indirect dependencies, we can all build higher together.
