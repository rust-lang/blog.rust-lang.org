+++
path = "2019/12/19/Rust-1.40.0"
title = "Announcing Rust 1.40.0"
authors = ["The Rust Release Team"]
aliases = [
    "2019/12/19/Rust-1.40.0.html",
    "releases/1.40.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.40.0. Rust is a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, getting Rust 1.40.0 is as easy as:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the appropriate page on our website, and check out the [detailed release notes for 1.40.0][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1400-2019-12-19

## What's in 1.40.0 stable

The highlights of Rust 1.40.0 include `#[non_exhaustive]` and improvements to `macros!()` and `#[attribute]`s. Finally, borrow-check migration warnings have become hard errors in Rust 2015. See the [detailed release notes][notes] for additional information.

### `#[non_exhaustive]` structs, enums, and variants

[`Ordering`]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
[report_non_exhaustive]: https://github.com/rust-lang/rust/issues/44109#issuecomment-533356866

Suppose you're a library author of a crate `alpha`, that has a `pub struct Foo`. You would like to make `alpha::Foo`'s fields `pub` as well, but you're not sure whether you might be adding more fields to `Foo` in future releases. So now you have a dilemma: either you make the fields private, with the drawbacks that follow, or you risk users depending on the exact fields, breaking their code when you add a new one. Rust 1.40.0 introduces a way to break the logjam: `#[non_exhaustive]`.

The attribute `#[non_exhaustive]`, when attached to a `struct` or the variant of an `enum`, will prevent code outside of the crate defining it from constructing said `struct` or variant. To avoid future breakage, other crates are also prevented from exhaustively matching on the fields. The following example illustrates errors in `beta` which depends on `alpha`:

```rust
// alpha/lib.rs:

#[non_exhaustive]
struct Foo {
    pub a: bool,
}

enum Bar {
    #[non_exhaustive]
    Variant { b: u8 }
}

fn make_foo() -> Foo { ... }
fn make_bar() -> Bar { ... }

// beta/lib.rs:

let x = Foo { a: true }; //~ ERROR
let Foo { a } = make_foo(); //~ ERROR

// `beta` will still compile when more fields are added.
let Foo { a, .. } = make_foo(); //~ OK


let x = Bar::Variant { b: 42 }; //~ ERROR
let Bar::Variant { b } = make_bar(); //~ ERROR
let Bar::Variant { b, .. } = make_bar(); //~ OK
                   // -- `beta` will still compile...
```

What happens behind the scenes is that the visibility of the constructors for a `#[non_exhaustive]` `struct` or `enum` variant is lowered to `pub(crate)`, preventing access outside the crate defining it.

A perhaps more important aspect of `#[non_exhaustive]` is that it can also be attached to `enum`s themselves. An example, taken from the standard library, is [`Ordering`]:

```rust
#[non_exhaustive]
pub enum Ordering { Relaxed, Release, Acquire, AcqRel, SeqCst }
```

The purpose of `#[non_exhaustive]` in this context is to ensure that more variants can be added over time. This is achieved by preventing other crates from exhaustively pattern `match`-ing on `Ordering`. That is, the compiler would reject:

```rust
match ordering {
    // This is an error, since if a new variant is added,
    // this would suddenly break on an upgrade of the compiler.
    Relaxed | Release | Acquire | AcqRel | SeqCst => {
        /* logic */
    }
}
```

Instead, other crates need to account for the possibility of more variants by adding a wildcard arm using e.g. `_`:
```rust
match ordering {
    Relaxed | Release | Acquire | AcqRel | SeqCst => { /* ... */ }
    // OK; if more variants are added, nothing will break.
    _ => { /* logic */ }
}
```

For more details on the `#[non_exhaustive]` attribute, see the [stabilization report][report_non_exhaustive].

### Macro and attribute improvements

[pr_bang_proc_type]: https://github.com/rust-lang/rust/pull/63931/#issuecomment-526362396
[pr_bang_extern]: https://github.com/rust-lang/rust/pull/63931/#issuecomment-526362396
[ref_extern_block]: https://doc.rust-lang.org/nightly/reference/items/external-blocks.html
[pr_mr_proc]: https://github.com/rust-lang/rust/pull/64035#issuecomment-533890826
[pr_meta]: https://github.com/rust-lang/rust/pull/63674
[pr_modern_syn]: https://github.com/rust-lang/rust/pull/57367#issuecomment-457882109

In 1.40.0, we have introduced several improvements to macros and attributes, including:

- [Calling procedural macros `mac!()` in type contexts.][pr_bang_proc_type]

  For example, you may write `type Foo = expand_to_type!(bar);` where `expand_to_type` would be a procedural macro.

- [Macros in `extern { ... }` blocks.][pr_bang_extern]

  This includes `bang!()` macros, for example:
  ```rust
  macro_rules! make_item { ($name:ident) => { fn $name(); } }

  extern {
      make_item!(alpha);
      make_item!(beta);
  }
  ```

  Procedural macro attributes on items in [`extern { ... }` blocks][ref_extern_block] are now also supported:
  ```rust
  extern "C" {
      // Let's assume that this expands to `fn foo();`.
      #[my_identity_macro]
      fn foo();
  }
  ```

- [Generating `macro_rules!` items in procedural macros.][pr_mr_proc]

  Function-like (`mac!()`) and attribute (`#[mac]`) macros can both now generate `macro_rules!` items. For details on hygiene, please refer to the attached stabilization report.

- [The `$m:meta` matcher][pr_meta] supports [arbitrary token-stream values][pr_modern_syn].

  That is, the following is now valid:

  ```rust
  macro_rules! accept_meta { ($m:meta) => {} }
  accept_meta!( my::path );
  accept_meta!( my::path = "lit" );
  accept_meta!( my::path ( a b c ) );
  accept_meta!( my::path [ a b c ] );
  accept_meta!( my::path { a b c } );
  ```

### Borrow check migration warnings are hard errors in Rust 2015

[rel-1350]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html#nll-for-rust-2015
[rel-1310]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes
[err-2018]: https://github.com/rust-lang/rust/pull/63565
[err-2015]: https://github.com/rust-lang/rust/pull/64221
[rip-ast-borrowck]: https://github.com/rust-lang/rust/pull/64790
[niko-blog-nll]: https://blog.rust-lang.org/2019/11/01/nll-hard-errors.html

In the 1.35.0 release, [we announced][rel-1350] that NLL had come to Rust 2015 after first being released for the 2018 edition in [Rust 1.31][rel-1310].

As we noted back then, the old borrow checker had some bugs which would allow memory unsafety, and the NLL borrow checker fixed them. As these fixes break some stable code, we decided to gradually phase in the errors, by checking if the old borrow checker would accept the program and the NLL checker would reject it. In those cases, the errors would be downgraded to warnings.

The previous release, Rust 1.39.0, changes these warnings into errors for code using the [2018 edition][err-2018]. Rust 1.40.0 applies the same change for users of the [2015 edition][err-2015], closing those soundness holes for good. This also allows us to [clean up the old code from the compiler][rip-ast-borrowck].

If your build breaks due to this change, or you want to learn more, check out [Niko Matsakis's blog post][niko-blog-nll].

### More `const fn`s in the standard library

[pr_is_power_of_two]: https://github.com/rust-lang/rust/pull/65092
[`is_power_of_two`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_power_of_two

With Rust 1.40.0, the following function became `const fn`:

- [`is_power_of_two`] for [unsigned integers][pr_is_power_of_two]

### Additions to the standard library

[`todo!()`]: https://doc.rust-lang.org/std/macro.todo.html
[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html
[`slice::repeat`]: https://doc.rust-lang.org/std/primitive.slice.html#method.repeat
[`BTreeMap::get_key_value`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get_key_value
[`HashMap::get_key_value`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_key_value
[`Option::flatten`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.flatten
[`Option::as_deref`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref
[`Option::as_deref_mut`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref_mut
[`UdpSocket::peer_addr`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.peer_addr
[`{f32,f64}::to_be_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.to_be_bytes
[`{f32,f64}::to_le_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.to_le_bytes
[`{f32,f64}::to_ne_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.to_ne_bytes
[`{f32,f64}::from_be_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.from_be_bytes
[`{f32,f64}::from_le_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.from_le_bytes
[`{f32,f64}::from_ne_bytes`]: https://doc.rust-lang.org/std/primitive.f32.html#method.from_ne_bytes

[`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
[`Cell::take`]: https://doc.rust-lang.org/std/cell/struct.Cell.html#method.take
[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`unimplemented!()`]: https://doc.rust-lang.org/std/macro.unimplemented.html
[`Option::flatten`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.flatten
[`Option::as_ref`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_ref
[`Option::as_mut`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut
[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`Iterator::flatten`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten

In Rust 1.40.0 the following functions and macros were stabilized:

- [`todo!()`]

  A macro, which is a shorter, more memorable, and convenient version of [`unimplemented!()`].

- [`slice::repeat`]

  Creates a `Vec<T>` by repeating a slice `n` times.

- [`mem::take`]

  This function `take`s the value out of a mutable reference and replaces it with the type's default. This is similar to [`Option::take`] and [`Cell::take` ] and provides a convenient short-hand for [`mem::replace(&mut dst, Default::default())`][`mem::replace`].

- [`BTreeMap::get_key_value`] and [`HashMap::get_key_value`]

  Returns the key-value pair corresponding to the supplied key.

- [`Option::as_deref`], [`Option::as_deref_mut`]

  These work similarly to [`Option::as_ref`] and [`Option::as_mut`] but also use [`Deref`] and [`DerefMut`] respectively, so that `opt_box.as_deref()` and `opt_box.as_deref_mut()`, where `opt_box: Option<Box<T>>`, produce an `Option<&T>` and `Option<&mut T>` respectively.

- [`Option::flatten`]

  This function flattens an `Option<Option<T>>` to `Option<T>` producing `Some(x)` for `Some(Some(x))` and `None` otherwise. The function is similar to [`Iterator::flatten`].

- [`UdpSocket::peer_addr`]

  Returns the socket address of the remote peer this socket was connected to.

- [`{f32,f64}::to_be_bytes`], [`{f32,f64}::to_le_bytes`],[`{f32,f64}::to_ne_bytes`], [`{f32,f64}::from_be_bytes`], [`{f32,f64}::from_le_bytes`], and [`{f32,f64}::from_ne_bytes`]

  Return the memory representation of the floating point number as a byte array in big-endian (network), little-endian, and native-endian byte order.

### Other changes

[relnotes-cargo]: https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-140-2019-12-19
[relnotes-clippy]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-140
[compat-notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#compatibility-notes

There are other changes in the Rust 1.40.0 release: check out what changed in [Rust][notes], [Cargo][relnotes-cargo], and [Clippy][relnotes-clippy].

Please also see the [compatibility notes][compat-notes] to check if you're affected by those changes.

## Contributors to 1.40.0

Many people came together to create Rust 1.40.0. We couldn't have done it
without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.40.0/)
