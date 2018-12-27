---
layout: post
title: "Procedural Macros in Rust 2018"
author: Alex Crichton
---

Perhaps my favorite feature in the Rust 2018 edition is [procedural macros].
Procedural macros have had a long and storied history in Rust (and will continue
to have a storied future!), and now is perhaps one of the best times to get
involved with them because the 2018 edition has so dramatically improved the
experience both defining and using them.

Here I'd like to explore what procedural macros are, what they're capable of,
notable new features, and some fun use cases of procedural macros. I might even
convince you that this is Rust 2018's best feature as well!

### What is a procedural macro?

First defined over two years ago in [RFC 1566], procedural macros are, in
layman's terms, a function that takes a piece of syntax at compile time and
produces a new bit of syntax. Procedural macros in Rust 2018 come in one of
three flavors:

* **`#[derive]` mode macros** have actually been stable since [Rust 1.15]
  and bring all the goodness and ease of use of `#[derive(Debug)]` to
  user-defined traits as well, such as [Serde]'s `#[derive(Deserialize)]`.

* **Function-like macros** are newly stable to the 2018 edition and allow
  defining macros like `env!("FOO")` or `format_args!("...")` in a
  crates.io-based library. You can think of these as sort of "`macro_rules!`
  macros" on steroids.

* **Attribute macros**, my favorite, are also new in the 2018 edition
  and allow you to provide lightweight annotations on Rust functions which
  perform syntactical transformations over the code at compile time.

Each of these flavors of macros can be defined in a crate with `proc-macro =
true` [specified in its manifest][manifest]. When used, a procedural macro is
loaded by the Rust compiler and executed as the invocation is expanded. This
means that Cargo is in control of versioning for procedural macros and you can
use them with all same ease of use you'd expect from other Cargo dependencies!

### Defining a procedural macro

Each of the three types of procedural macros are [defined in a slightly different
fashion][proc-ref], and here we'll single out attribute macros. First, we'll flag
`Cargo.toml`:

```toml
[lib]
proc-macro = true
```

and then in `src/lib.rs` we can write our macro:

```rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ...
}
```

We can then write some unit tests in `tests/smoke.rs`:

```rust
#[my_crate::hello]
fn wrapped_function() {}

#[test]
fn works() {
    wrapped_function();
}
```

... and that's it! When we execute `cargo test` Cargo will compile our
procedural macro. Afterwards it will compile our unit test which loads the macro
at compile time, executing the `hello` function and compiling the resulting
syntax.

Right off the bat we can see a few important properties of procedural macros:

* The input/output is this fancy `TokenStream` type we'll talk about more in a
  bit
* We're *executing arbitrary code* at compile time, which means we can do just
  about anything!
* Procedural macros are incorporated with the module system, meaning they can
  be imported just like any other name.

Before we take a look at implementing a procedural macro, let's first dive into
some of these points.

### Macros and the module system

First stabilized in [Rust 1.30] \(noticing a trend with 1.15?\) macros are now
integrated with the module system in Rust. This mainly means that you no longer
need the clunky `#[macro_use]` attribute when importing macros! Instead of this:

```rust
#[macro_use]
extern crate log;

fn main() {
    debug!("hello, ");
    info!("world!");
}
```

you can do:

```rust
use log::info;

fn main() {
    log::debug!("hello, ");
    info!("world!");
}
```

Integration with the module system solves one of the most confusing parts about
macros historically. They're now imported and namespaced just as you would any
other item in Rust!

The benefits are not only limited to bang-style `macro_rules` macros, as you can
now transform code that looks like this:

```rust
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize)]
struct Foo {
    // ...
}
```

into

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Foo {
    // ...
}
```

and you don't even need to explicitly depend on `serde_derive` in `Cargo.toml`!
All you need is:

```toml
[dependencies]
serde = { version = '1.0.82', features = ['derive'] }
```

### What's inside a `TokenStream`?

This mysterious `TokenStream` type comes from the [compiler-provided
`proc_macro` crate][pm]. When it was first added all you could do with a
[`TokenStream`] was call convert it to or from a string using `to_string()` or `parse()`.
As of Rust 2018, you can act on the tokens in a [`TokenStream`] directly.

A [`TokenStream`] is effectively "just" an iterator over [`TokenTree`]. All
syntax in Rust falls into one of these four categories, the four variants of
[`TokenTree`]:

* `Ident` is any identifier like `foo` or `bar`. This also contains keywords
  such as `self` and `super`.
* `Literal` include things like `1`, `"foo"`, and `'b'`. All literals are one
  token and represent constant values in a program.
* `Punct` represents some form of punctuation that's not a delimiter. For
  example `.` is a `Punct` token in the field access of `foo.bar`.
  Multi-character punctuation like `=>` is represented as two `Punct` tokens,
  one for `=` and one for `>`, and the `Spacing` enum says that the `=` is
  adjacent to the `>`.
* `Group` is where the term "tree" is most relevant, as `Group` represents a
  delimited sub-token-stream. For example `(a, b)` is a `Group` with parentheses
  as delimiters, and the internal token stream is `a, b`.

While this is conceptually simple, this may sound like there's not much we can
do with this! It's unclear, for example, how we might parse a function from a
`TokenStream`. The minimality of `TokenTree` is crucial, however, for
stabilization. It would be infeasible to stabilize the Rust AST because that
means we could never change it. (imagine if we couldn't have added the `?`
operator!)

By using `TokenStream` to communicate with procedural macros, the compiler is
able to add new language syntax while also being able to compile
and work with older procedural macros. Let's see now, though, how we can
actually get useful information out of a `TokenStream`.

### Parsing a `TokenStream`

If `TokenStream` is just a simple iterator, then we've got a long way to go from
that to an actual parsed function. Although the code is already lexed for us
we still need to write a whole Rust parser! Thankfully though the community has
been hard at work to make sure writing procedural macros in Rust is as smooth as
can be, so you need look no further than the [`syn` crate][syn].

With the [`syn`][syn] crate we can parse any Rust AST as a one-liner:

```rust
#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;
    let abi = &input.abi;
    // ...
}
```

The [`syn`][syn] crate not only comes with the ability to parse built-in syntax
but you can also easily write a recursive descent parser for your own syntax.
The [`syn::parse` module][spm] has more information about this capability.

### Producing a `TokenStream`

Not only do we take a `TokenStream` as input with a procedural macro, but we
also need to produce a `TokenStream` as output. This output is typically
required to be valid Rust syntax, but like the input it's just list of tokens
that we need to build somehow.

Technically the only way to create a `TokenStream` is via its `FromIterator`
implementation, which means we'd have to create each token one-by-one and
collect it into a `TokenStream`. This is quite tedious, though, so let's take a
look at [`syn`][syn]'s sibling crate: [`quote`].

The [`quote`] crate is a quasi-quoting implementation for Rust which primarily
provides a convenient macro for us to use:

```rust
use quote::quote;

#[proc_macro_attribute]
pub fn hello(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        fn #name() -> u32 { 42 }
    };
    result.into()
}
```

The [`quote!` macro] allows you to write mostly-Rust syntax and interpolate
variables quickly from the environment with `#foo`. This removes much of the
tedium of creating a `TokenStream` token-by-token and allows quickly cobbling
together various pieces of syntax into one return value.

### Tokens and `Span`

Perhaps the greatest feature of procedural macros in Rust 2018 is the ability to
customize and use [`Span`] information on each token, giving us the ability for
amazing syntactical error messages from procedural macros:

```
error: expected `fn`
 --> src/main.rs:3:14
  |
3 | my_annotate!(not_fn foo() {});
  |              ^^^^^^
```

as well as completely custom error messages:

```
error: imported methods must have at least one argument
  --> invalid-imports.rs:12:5
   |
12 |     fn f1();
   |     ^^^^^^^^
```

A [`Span`] can be thought of as a pointer back into an original source file,
typically saying something like "the `Ident` token` foo` came from file
`bar.rs`, line 4, column 5, and was 3 bytes long". This information is
primarily used by the compiler's diagnostics with warnings and error messages.

In Rust 2018 each [`TokenTree`] has a [`Span`] associated with it. This means that
if you preserve the [`Span`] of all input tokens into the output then even
though you're producing brand new syntax the compiler's error messages are still
accurate!

For example, a small macro like:

```rust
#[proc_macro]
pub fn make_pub(item: TokenStream) -> TokenStream {
    let result = quote! {
        pub #item
    };
    result.into()
}
```

when invoked as:

```rust
my_macro::make_pub! {
    static X: u32 = "foo";
}
```

is invalid because we're returning a string from a function that should return a
`u32`, and the compiler will helpfully diagnose the problem as:

```
error[E0308]: mismatched types
 --> src/main.rs:1:37
  |
1 | my_macro::make_pub!(static X: u32 = "foo");
  |                                     ^^^^^ expected u32, found reference
  |
  = note: expected type `u32`
             found type `&'static str`

error: aborting due to previous error

```

And we can see here that although we're generating brand new syntax, the
compiler can preserve span information to continue to provide targeted
diagnostics about code that we've written.

### Procedural Macros in the Wild

Ok up to this point we've got a pretty good idea about what procedural macros
can do and the various capabilities they have in the 2018 edition. As such a
long-awaited feature, the ecosystem is already making use of these new
capabilities! If you're interested, some projects to keep your eyes on are:

* [`syn`][syn], [`quote`], and [`proc-macro2`] are your go-to libraries for
  writing procedural macros. They make it easy to define custom parsers, parse
  existing syntax, create new syntax, work with older versions of Rust, and much
  more!

* [Serde] and its derive macros for `Serialize` and `Deserialize` are likely the
  most used macros in the ecosystem. They sport an [impressive amount of
  configuration][serde-attr] and are a great example of how small annotations
  can be so powerful.

* The [`wasm-bindgen` project][wbg] uses attribute macros to easily define
  interfaces in Rust and import interfaces from JS. The `#[wasm_bindgen]`
  lightweight annotation makes it easy to understand what's coming in and out,
  as well as removing lots of conversion boilerplate.

* The [`gobject_gen!` macro][gnome-class] is an experimental IDL for the GNOME
  project to define GObject objects safely in Rust, eschewing manually writing
  all the glue necessary to talk to C and interface with other GObject
  instances in Rust.

* The [Rocket framework][rocket] has recently switched over to procedural
  macros, and showcases some of nightly-only features of procedural macros like
  custom diagnostics, custom span creation, and more. Expect to see these
  features stabilize in 2019!

That's just a *taste* of the power of procedural macros and some example usage
throughout the ecosystem today. We're only 6 weeks out from the original release
of procedural macros on stable, so we've surely only scratched the surface as
well! I'm really excited to see where we can take Rust with procedural macros by
empowering all kinds of lightweight additions and extensions to the language!

[procedural macros]: https://doc.rust-lang.org/reference/procedural-macros.html
[RFC 1566]: https://github.com/rust-lang/rfcs/blob/master/text/1566-proc-macros.md
[Rust 1.15]: https://blog.rust-lang.org/2017/02/02/Rust-1.15.html
[Serde]: https://serde.rs
[manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[proc-ref]: https://doc.rust-lang.org/stable/reference/procedural-macros.html
[pm]: https://doc.rust-lang.org/proc_macro/
[`TokenStream`]: https://doc.rust-lang.org/stable/proc_macro/struct.TokenStream.html
[`TokenTree`]: https://doc.rust-lang.org/stable/proc_macro/enum.TokenTree.html
[Rust 1.30]: https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html
[syn]: https://crates.io/crates/syn
[spm]: https://docs.rs/syn/0.15/syn/parse/index.html
[`quote`]: https://docs.rs/quote/0.6/quote/
[`quote!` macro]: https://docs.rs/quote/0.6/quote/macro.quote.html
[`Span`]: https://doc.rust-lang.org/proc_macro/struct.Span.html
[`proc-macro2`]: https://docs.rs/proc-macro2/0.4/proc_macro2/
[serde-attr]: https://serde.rs/attributes.html
[wbg]: https://github.com/rustwasm/wasm-bindgen
[gnome-class]: https://gitlab.gnome.org/federico/gnome-class
[rocket]: https://rocket.rs/
