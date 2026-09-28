+++
path = "inside-rust/2026/10/01/generic-const-args-and-you"
title = "Generic Const Args and You"
authors = ["BoxyUwU"]

[extra]
team = "The Const Generics Project Group"
team_url = "https://rust-lang.org/governance/teams/lang/#team-project-const-generics"
+++

# Generic Const Args and You

Back in June of 2024 at RustFest Zürich, the Const Generics project group first discussed a new design for supporting uses of generic parameters in const generic arguments. Since then, we've continued to refine the initial design and have implemented the new design as a family of features dubbed "Generic Const Arguments" (GCA for short).

These features are intended to replace the existing `generic_const_exprs` feature which has existed in some form or another since `min_const_generics` was stabilized back in 2021. Even though GCA obviates `generic_const_exprs` it was still incredibly valuable to have invested the time into it that we did as the design and implementation of GCA was informed quite significantly by `generic_const_exprs`.

Each feature in the GCA family introduces support for a specific set of expressions to the type system. This post will go over all of the features part of the GCA family and explain their design and how to use them.

---

A huge thanks to [@camelid](https://www.github.com/camelid) for doing almost all of the initial implementation work for the GCA prototype. Without him GCA would have stayed just a vague concept rather than anything tangible.

Additionally, a huge thanks to [@khyperia](https://github.com/khyperia) who has done a significant amount of the implementation and design work required to go from the original GCA prototype to the fully featured family that it is now.

Finally there have been a tonne of other people who have made contributions to getting GCA to where it is today. For a full list of everyone and all of their contributions, see the [Full Const Generics](https://github.com/rust-lang/goals/issues/100) project goal.

## What is GCA

Generic Const Arguments is a family of features introducing support for more kinds of expressions to Const Generics. For example, the `gca_adts` feature adds support for Struct Expressions to Const Generics:
```rust
#![feature(adt_const_params, gca_adts)]
use core::gca;

/* some details omitted */

struct Bar<const N: Struct>;
type BarWrapper<const N: usize> = Bar<gca!(Struct { field: N })>;
```

In this example the `gca!(Struct { field: N })` is the new functionality introduced by `gca_adts`. The `adt_const_params` feature is separate and instead allows defining the `const N: Struct` generic parameter.

All GCA features require any newly supported expressions to be written inside of a `gca!` macro call. The `gca_macroless_*` features lift this restriction and will be talked more about later, as well as why we have this restriction in the first place.

### ADT Generic Const Args

Support for constructing arrays, tuples, and ADTs are all lumped into the `gca_adts` feature. We might split this into multiple features at some point but for now it's just the one.

Constructing enums and structs is supported regardless of the syntax they're defined with (i.e. unit, tuple or struct syntax): 
```rust
#![feature(gca_adts, adt_const_params)]
use core::gca;

#[derive(ConstParamTy, Eq, PartialEq)]
struct TupleStruct(usize);
fn accepts<const N: TupleStruct>() {}

fn example<const N: usize>() {
    accepts::<gca!(TupleStruct(N))>();
}
```

```rust
#![feature(gca_adts, adt_const_params)]
use core::gca;

#[derive(ConstParamTy, Eq, PartialEq)]
enum MyEnum {
    Record { x: usize },
}
fn accepts<const E: MyEnum>() {}

fn example<const N: usize>() {
    accepts::<gca!(MyEnum::Record { x: N })>();
}
```

And lastly support for constructing arrays and tuples:
```rust
#![feature(gca_adts, adt_const_params)]
use core::gca;

fn accepts_arrays<const N: [usize; 2]>() {}

fn example<const N1: usize>() {
    accepts_arrays::<gca!([N1, 12])>();
}
```

```rust
#![feature(gca_adts, adt_const_params)]
use core::gca;

fn accepts_tuple<const N: (usize, usize)>() {}

fn example<const N1: usize>() {
    accepts_tuple::<gca!((N1, 12))>();
}
```

We currently do not support array repeat expressions but do intend to support this at some point:
```rust
#![feature(gca_adts, adt_const_params)]
use core::gca;

fn accepts_arrays<const N: [usize; 2]>() {}

fn example<const N1: usize>() {
    // currently disallowed :(
    accepts_arrays::<gca!([N1; 2])>();
}
```

### Const Item Generic Const Args

Support for using const items in the type system is part of the `gca_const_items` feature.

By allowing arbitrary const items to be used in the type system, we are allowing arbitrary expressions to *indirectly* be used in the type system. For example `gca!(N + 1)` cannot be used in the type system, but a const item defined as `const FOO: usize = N + 1;` could be.

Using an associated constant as a const argument looks like the following:
```rust
#![feature(gca_const_items)]
use core::gca;

trait Trait {
    const ASSOC: usize
}

fn example<T: Trait>() -> [u8; gca!(T::ASSOC)] {
    [0x1; _]
}
```

Using a free or inherent associated const item looks similarly:
```rust
#![feature(gca_const_items, generic_const_items, inherent_associated_types)]
use core::gca;

const FREE<T>: usize = size_of::<T>();
fn example_free<T>() -> [u8; gca!(FREE::<T>)] {
    [0x6; _]
}

struct Foo<T>(T);
impl<T> Foo<T> {
    const INHERENT: usize = size_of::<T>();
}
fn example_inherent<T>(foo: Foo<T>) -> [u8; gca!(Foo<T>::INHERENT)] {
    [0x7; _]
}
```

Note that the `inherent_associated_types` feature gate has also been enabled in the example. Using inherent associated constants in the `gca!(..)` macro requires an additional feature gate as there are equivalent challenges for supporting this as there are to supporting inherent associated types.

When defining a const item, the right hand side can be a `gca!(..)` expression, indicating that the type system can reason about the exact form of the expression. Without this the constant is treated "opaquely" and the type system has limited ability to reason about generic uses of it:
```rust
#![feature(gca_const_items, generic_const_items)]
use core::gca;

const FREE_OPAQUE<const N: usize>: usize = N;
fn make_array1<const N: usize>() -> [u8; FREE_OPAQUE::<N>] {
    // ERROR: `FREE_OPAQUE::<N>` and `N` are not equal
    [0; N]
}

const FREE_GCA<const N: usize>: usize = gca!(N);
fn make_array2<const N: usize>() -> [u8; FREE_GCA::<N>] {
    // OK :3
    [0; N]
}
```

In the above example, the compiler is unable to determine that the type `[u8; N]` and the type `[u8; FREE_OPAQUE::<N>]` are the same. This is because the constant `FREE_OPAQUE` is not defined as having a `gca!(..)`  right hand side and so the compiler cannot see that it is equal to `N`.

On the other hand, the compiler *can* tell that the type `[u8; N]` and the type `[u8; FREE_GCA::<N>]` are the same. This is because unlike `FREE_OPAQUE`, the constant `FREE_GCA` is defined as having a `gca!(..)` right hand side.

With support for const items in the type system present, `gca_const_items` also supports associated const bindings and traits with associated constants being dyn compatible:

```rust
#![feature(gca_const_items)]

trait Trait {
    const ASSOC: usize;
}

fn make_dyn<const N: usize, T: Trait<ASSOC = { N }>>(
    val: T,
) -> Box<dyn Trait<ASSOC = { N }> {
    Box::new(val)
}
```

On stable, the above example would fail to compile for two reasons. Firstly, unlike associated types we don't support bounding associated constants (e.g. `T: Trait<ASSOC = { N }`). Secondly, unlike associated types, we don't allow trait objects for traits which have associated consts. With `gca_const_items` enabled both of these are supported.

### Minimal Const Item Generic Const Args

We also have a minimal version of the `gca_const_items` feature, `gca_min_const_items`. It supports much the same as the full feature, except that instead of supporting *all* const items, only ones defined as `gca!(..)` expressions are supported.

```rust
#![feature(gca_min_const_items, generic_const_args)]

const BAD<const N: usize>: usize = N;
const GOOD<const N: usize>: usize = gca!(N);

fn foo<const N: usize>() {
    // not OK! `BAD` isn't equal to a `gca!(..)` expression
    let _: [u8; BAD::<N>];
    
    // OK :3
    let _: [u8; GOOD::<N>];
}
```

Unlike with the full `gca_const_items` feature which would accept the above example, this is rejected under `gca_min_const_items` as `BAD` does not use a `gca!(..)` right hand side.

To support trait associated constants a `rustc_always_gca`[^1] attribute exists which enforces that an associated constant is always implemented as a `gca!(..)` expression.

```rust
#![feature(gca_min_const_items)]

trait Trait<const N: usize> {
    #[rustc_always_gca]
    const ASSOC: usize;
}

impl<const N: usize> Trait<N> {
    // Not OK! not a `gca!(..)` expression
    const ASSOC: usize = N;
    
    // OK!
    const ASSOC: usize = gca!(N);
}
```

Requiring all const items in the type system to be defined as a `gca!(..)` expression allows us to limit the expressiveness of Const Generics in some desirable ways.

For example, without the full `gca_const_items` feature it is not (at the time of writing) possible to get a post-mono error from Const Generics, however it would be with `gca_const_items`. With `gca_min_const_items` we're able to retain this property at the cost of expressiveness.

As another example, the full `gca_const_items` feature can have quite confusing errors where two constants are considered unequal even though we as humans can tell that they're obviously equal. With `gca_min_const_items` there are fairly straight forward rules for determining when two constants are equal without any big surprises.

Finally, it's also significantly easier to implement `gca_min_const_items` than `gca_const_items` which means that it should be possible to get this minimal version up to the quality required for stabilization much sooner than it would be for `gca_const_items`.

## What is Macroless

While the GCA family of features currently requires all new syntax to be placed within a `gca!(..)` expression, this limitation is undesirable in the long term as it results in significant ergonomic issues in Const Generics heavy code. 

However, there are a number of design and implementation complexities for *not* having the `gca!(..)` macro (though we won't talk about them in this blog post). To let us handle these complexities independently, there are separate feature gates for removing the need for the `gca!(..)` macro, rather than having it part of the main GCA features.

There are currently two features relating to the removal of the `gca!(..)` macro, `gca_macroless_args` and `gca_macroless_items`. Each feature allows omitting the `gca!(..)` macro in different positions where it's currently required.

### Macroless Const Arguments

`feature(gca_macroless_args)` allow arguments to const generics to be written without the use of the `gca!(..)` macro:
```rust
#![feature(
    gca_macroless_args,
    gca_const_items,
)]

// OK even without an explicit `gca!(..)`
fn make_array<T: Trait>() -> [u8; T::ASSOC] {
    // ...
}
```

Without this feature enabled the compiler would require the return type of `make_array` to be written as `[u8; gca!(T::ASSOC)]`.

### Macroless Const Items

`feature(gca_macroless_items)` allows const items under `gca_const_items` and `gca_min_const_items` to be written without the `gca!(..)` macro:
```rust
#![feature(
    gca_macroless_items,
    gca_min_const_items,
)]

trait Trait {
    #[rustc_always_gca]
    const ASSOC: usize;
}

impl<const N: usize> Trait for [u8; N] {
    // OK even without an explicit `gca!(..)`
    const ASSOC: usisze = N;
}
```

Without this feature enabled the compiler would require the right hand side of `const ASSOC` to be written as `gca!(N)` to satisfy the `rustc_always_gca` attribute in the trait definition.

## Concluding it

We're not currently ready to stabilize any of the features talked about in this post. It will be a while before any of them have reached a level of polish where we would feel comfortable proposing the design as an RFC.

We would very much like to hear about any issues you run into with any of the GCA features. Whether that be compiler crashes, design issues making it hard to write code that you want to write, or if you're just struggling to get your code working with these features.

The best way to reach us would be to either open an issue on the [project-const-generics github repo](https://github.com/rust-lang/project-const-generics/issues/new/), or open a thread in the [project-const-generics zulip channel](https://rust-lang.zulipchat.com/#topics/channel/260443-project-const-generics).

[^1]: Introducing new attributes is technically a breaking change unless it has a `rustc_` prefix so we are using the name `rustc_always_gca` until stabilization at which point it would be renamed to `always_gca`.
