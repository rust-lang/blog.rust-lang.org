+++
path = "2025/11/20/switching-to-v0-mangling-on-nightly"
title = "Switching to Rust's own mangling scheme on nightly"
authors = ["David Wood"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

**TL;DR:** rustc will use its own "v0" mangling scheme by default on nightly
versions instead of the previous default, which re-used C++'s mangling
scheme, starting in `nightly-2025-11-21`

#### Context

When Rust is compiled into object files and binaries, each item (functions,
statics, etc) must have a globally unique  "symbol" identifying it.

In C, the symbol name of a function is just the name that the function was
defined with, such as `strcmp`. This is straightforward and easy to
understand, but requires that each item have a globally unique name
that doesn't overlap with any symbols from libraries that it is linked
against. If two items had the same symbol then when the linker tried to resolve
a symbol to an address in memory (of a function, say), then it wouldn't know
which symbol is the correct one.

Languages like Rust and C++ define "symbol mangling schemes", leveraging information
from the type system to give each item a unique symbol name. Without this, it would be
possible to produce clashing symbols in a variety of ways - for example, every
instantiation of a generic or templated function (or an overload in C++), which all
have the same name in the surface language would end up with clashing symbols; or
the same name in different modules, such as `a::foo` and `b::foo` would have clashing
symbols.

Rust originally used a symbol mangling scheme based on the
[Itanium ABI's name mangling scheme][itanium-mangling] used by C++ (sometimes). Over
the years, it was extended in an inconsistent and ad-hoc way to support Rust
features that the mangling scheme wasn't originally designed for. Rust's current legacy
mangling scheme has a number of drawbacks:

- Information about generic parameter instantiations is lost during mangling
- It is internally inconsistent - some paths use an Itanium ABI-style encoding
  but some don't
- Symbol names can contain `.` characters which aren't supported on all platforms
- Symbol names include an opaque hash which depends on compiler internals and
  can't be easily replicated by other compilers or tools
- There is no straightforward way to differentiate between Rust and C++ symbols

If you've ever tried to use Rust with a debugger or a profiler and found it hard
to work with because you couldn't work out which functions were which, it's probably
because information was being lost in the mangling scheme.

Rust's compiler team started working on our own mangling scheme back in 2018
with [RFC 2603][rfcs#2603] (see the ["v0 Symbol Format"][v0-mangling] chapter in
rustc book for our current documentation on the format). Our "v0" mangling scheme has
multiple advantageous properties:

- An unambiguous encoding for everything that can end up in a binary's symbol table
- Information about generic parameters are encoded in a reversible way
- Mangled symbols are decodable such that it should be possible to identify concrete
  instances of generic functions
- It doesn't rely on compiler internals
- Symbols are restricted to only `A-Z`, `a-z`, `0-9` and `_`, helping ensure
  compatibility with tools on varied platforms
- It tries to stay efficient and avoid unnecessarily long names and
  computationally-expensive decoding 

However, rustc is not the only tool that interacts with Rust symbol names: the
aforementioned debuggers, profilers and other tools all need to be updated to
understand Rust's v0 symbol mangling scheme so that Rust's users can continue
to work with Rust binaries using all the tools they're used to without having
to look at mangled symbols. Furthermore, all of those tools need to have new
releases cut and then those releases need to be picked up by distros. This takes
time!

Fortunately, the compiler team now believe that support for our v0 mangling
scheme is now sufficiently widespread that it can start to be used by default by
rustc.

#### Benefits

Reading Rust backtraces, or using Rust with debuggers, profilers and other
tools that operate on compiled Rust code, will be able to output much more
useful and readable names. This will especially help with async code,
closures and generic functions.

It's easy to see the new mangling scheme in action, consider the following
example:

```rust
fn foo<T>() {
    panic!()
}

fn main() {
    foo::<Vec<(String, &[u8; 123])>>();
}
```

With the legacy mangling scheme, all of the useful information about the generic
instantiation of `foo` is lost in the symbol `f::foo`..

```
thread 'main' panicked at f.rs:2:5:
explicit panic
stack backtrace:
  0: std::panicking::begin_panic
    at /rustc/d6c...582/library/std/src/panicking.rs:769:5
  1: f::foo
  2: f::main
  3: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

..but with the v0 mangling scheme, the useful details of the generic instantiation
are preserved with  `f::foo::<alloc::vec::Vec<(alloc::string::String, &[u8; 123])>>`:

```
thread 'main' panicked at f.rs:2:5:
explicit panic
stack backtrace:
  0: std::panicking::begin_panic
    at /rustc/d6c...582/library/std/src/panicking.rs:769:5
  1: f::foo::<alloc::vec::Vec<(alloc::string::String, &[u8; 123])>>
  2: f::main
  3: <fn() as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

#### Possible drawbacks

Symbols using the v0 mangling scheme can be larger than symbols with the
legacy mangling scheme, which can result in a slight increase in linking
times and binary sizes if symbols aren't stripped (which they aren't by default).
Fortunately this impact should be minor, especially with modern linkers like
lld, which Rust [will now default to on some targets][switch-to-lld].

Some old versions of tools/distros or niche tools that the compiler team are
unaware of may not have had support for the v0 mangling scheme added. When
using these tools, the only consequence is that users may encounter mangled
symbols. [rustfilt] can be used to demangle Rust symbols if a tool does not.

In any case, using the new mangling scheme can be disabled if any problem
occurs: use the `-Csymbol-mangling-version=legacy -Zunstable-options` flag
to revert to using the legacy mangling scheme.

Explicitly enabling the legacy mangling scheme requires nightly, it is not
intended to be stabilised so that support can eventually be removed.

#### Adding v0 support in your tools

If you maintain a tool that interacts with Rust symbols and does not
support the v0 mangling scheme, there are Rust and C implementations
of a v0 symbol demangler available in the [rust-lang/rustc-demangle]
repository that can be integrated into your project.

#### Summary

rustc will use our "v0" mangling scheme on nightly for all targets
starting in tomorrow's rustup nightly (`nightly-2025-11-21`).

Let us know if you encounter problems, by [opening an
issue](https://github.com/rust-lang/rust/issues/new/choose) on GitHub.

If that happens, you can use the legacy mangling scheme with
the `-Csymbol-mangling-version=legacy -Zunstable-options` flag.
Either by adding it to the usual `RUSTFLAGS` environment variable, or to a
project's [`.cargo/config.toml`] configuration file, like so:

```toml
[build]
rustflags = ["-Csymbol-mangling-version=legacy", "-Zunstable-options"]
```

If you like the sound of the new symbol mangling version and would
like to start using it on stable or beta channels of Rust, then you can
similarly use the `-Csymbol-mangling-version=v0` flag today via
`RUSTFLAGS` or [`.cargo/config.toml`]:

```toml
[build]
rustflags = ["-Csymbol-mangling-version=v0"]
```

[`.cargo/config.toml`]: https://doc.rust-lang.org/cargo/reference/config.html
[rfcs#2603]: https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html
[itanium-mangling]: https://refspecs.linuxbase.org/cxxabi-1.86.html#mangling
[v0-mangling]: https://doc.rust-lang.org/nightly/rustc/symbol-mangling/v0.html
[switch-to-lld]: https://blog.rust-lang.org/2025/09/01/rust-lld-on-1.90.0-stable/
[rustfilt]: https://github.com/luser/rustfilt
[rust-lang/rustc-demangle]: https://github.com/rust-lang/rustc-demangle
