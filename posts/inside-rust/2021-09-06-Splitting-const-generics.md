+++
layout = "post"
title = "Splitting the const generics features"
author = "lcnr"
description = "Splitting the const generics features"
team = "The Const Generics Project Group <https://rust-lang.github.io/project-const-generics/>"
+++

After the stabilization of the const generics MVP in version 1.51, the const generics project group has continued to
work on const generics. Large parts of this work were gated behind the feature gates `const_generics` and `const_evaluatable_checked`. As time went on, the
`const_generics` feature became fairly useless on its own while the name of
`const_evaluatable_checked` didn't really capture what this feature was intended to do.

To improve this, we have recently removed the features `const_generics`, `lazy_normalization_consts`, and `const_evaluatable_checked`. They have been replaced by `feature(adt_const_params)` and `feature(generic_const_exprs)`.

As there is a lot going on with const generics, here's a quick overview of the new - and preexisting - features and how much still needs to be done for them to get stabilized:

### `feature(adt_const_params)`

On stable, only integers, `char` and `bool` are allowed as the types of const parameters. This feature allows additional types, such as `&'static str` and user defined types.
```rust
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
enum ImageFormat {
    Rgb8,
    Rgba8,
    // ...c
}

struct Image<const FORMAT: ImageFormat> {
    // ...
}

impl Image<{ ImageFormat::Rgba }> {
    fn alpha(&self, pixel: PixelLocation) -> u8 {
        // ...
    }
}
```
Note that even with this feature, generic const parameter types, such as `struct Foo<T, const N: T> { ... }`, are forbidden.
While allowing such things is desired, it adds additional complications exceeding our current capacity.

There are still two major blockers for stabilization: 

The first being the [transition to valtrees](https://github.com/rust-lang/rust/pull/83234). Valtrees are a representation of values as trees with integer nodes, simplifiying the way we interact with more complex types.

Additionally, we have to figure out which types we *even want* to allow as const parameter types. This ties into the discussion
about ["structural match"](https://github.com/rust-lang/rust/issues/74446), which is still ongoing.

While the issues mentioned above are definitely not trivial, it is definitely possible for this to be ready for stabilization in a few months.

### `feature(generic_const_exprs)`

Without any unstable features, const arguments must either be a fully concrete expression or a generic parameter by itself, so constants like `N + 1` are forbidden. With this feature, expressions using generic parameters are possible. 

```rust
#![feature(generic_const_exprs)]

fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1]) {
    // ...
}

struct BitSet<const SIZE: usize>
where
    [u8; (SIZE + 7) / 8]: Sized,
{
    storage: [u8; (SIZE + 7) / 8],
}
```

We currently require the user to add bounds asserting that generic constants evaluate successfully. For all constants visible in the API of an item, these bounds are added implicitly. 

If the constant expression `expr` of type `Foo` would otherwise not be used in the `where`-clauses or function signature, we add an otherwise irrelevant bound mentioning `expr` to the `where`-clauses of our item. For this one can define a `struct Evaluatable<const N: Foo>;` and use `Evaluatable<{ expr }>:` as a bound. If `expr` is of type `usize` we tend to use `[u8; expr]:`
or `[u8; expr]: Sized` for this. While it is highly likely that we will add a dedicated syntax for these bounds in the future, we are waiting with this until the rest of this feature is more mature.

This feature is still far from being stable and has some [**major** unsolved issues](https://github.com/rust-lang/project-const-generics/blob/master/design-docs/anon-const-substs.md). Especially for constants inside of `where`-bounds there are a lot of subtle bugs and backwards incompatibilities we have to fix before we can even think about how to stabilize this.

### `feature(const_generics_defaults)`

Similar to type parameter defaults, this feature adds the ability to declare default values for const parameters.

```rust
#![feature(const_generics_defaults)]

struct ArrayStorage<T, const N: usize = 2> {
    arr: [T; N],
}

impl<T> ArrayStorage<T> {
    fn new(a: T, b: T) -> ArrayStorage<T> {
        ArrayStorage {
            arr: [a, b],
        }
    }
}
```
To allow type parameter defaults in the same listing as const parameters we also intend to remove the ordering restriction for
type and const parameters, allowing `struct Foo<const N: usize, T = [u32; N]> { ... }`.

This feature is pretty much ready for stabilization and is currently blocked on figuring out any potential edge cases for the
stabilization report.

### `feature(generic_arg_infer)`

While it is already possible to use a wildcard `_` for type arguments inside of bodies, this is not the case for const arguments.
This feature adds this capability for constants.

```rust
#![feature(generic_arg_infer)]
fn array_from<T, U, const N: usize>(arr: [T; N]) -> [U; N]
where
    U: From<T>,
{
    arr.map(From::from)
}

fn main() {
    let x = ["this", "is", "a", "six", "element", "array"];
    // using `_` for the parameter `N` lets
    // the compiler infer the correct value
    let _y = array_from::<_, String, _>(x);
}
```

This feature is not yet ready for stabilization, though there aren't any known big blockers here.
To confidently stabilize this we are probably in need of some large refactorings though, as the current setup
feels fairly fragile in some areas.

