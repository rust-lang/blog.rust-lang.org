---
layout: post
title: "Const generics MVP hits beta!"
author: The const generics project group
---

After more than 3 years since the [original RFC for const generics](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md) was accepted, **the first version of const generics is now available in the Rust beta channel!** It will be available in the 1.51 release, which is expected to be released on **March 25th, 2021**. Const generics is one of the [most highly anticipated](https://blog.rust-lang.org/2020/12/16/rust-survey-2020.html) features coming to Rust, and we're excited for people to start taking advantage of the increased power of the language following this addition.

Even if you don't know what const generics are (in which case, read on!), you've likely been benefitting from them: const generics are already employed in the Rust standard library to improve the ergonomics of arrays and diagnostics; more on that below.

With const generics hitting beta, let's take a quick look over what's actually being stabilized, what this means practically, and what's next.

## What are const generics?

Const generics are generic arguments that range over constant values, rather than types or lifetimes. This allows, for instance, types to be parameterized by integers. In fact, there has been one example of const generic types since early on in Rust's development: the array types `[T; N]`, for some type `T` and `N: usize`. However, there has previously been no way to abstract over arrays of an arbitrary size: if you wanted to implement a trait for arrays of any size, you would have to do so manually for each possible value. For a long time, even the standard library methods for arrays were limited to arrays of length at most 32 due to this problem. This restriction was [finally lifted in Rust 1.47](https://blog.rust-lang.org/2020/10/08/Rust-1.47.html#traits-on-larger-arrays) - a change that was made possible by const generics.

Here's an example of a type and implementation making use of const generics: a type wrapping a pair of arrays of the same size.

```rust
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
}
```

### Current restrictions

The first iteration of const generics has been deliberately constrained: in other words, this version is the MVP (minimal viable product) for const generics. This decision is motivated both by the additional complexity of general const generics (the implementation for general const generics is not yet complete, but we feel const generics in 1.51 are already very useful), as well as by the desire to introduce a large feature gradually, to gain experience with any potential shortcomings and difficulties. We intend to lift these in future versions of Rust: see [what's next](#whats-next).

#### Only integral types are permitted for const generics

For now, the only types that may be used as the type of a const generic argument are the types of integers (i.e. signed and unsigned integers, including `isize` and `usize`) as well as `char` and `bool`. This covers a primary use case of const, namely abstracting over arrays. In the future, this restriction will be lifted to allow more complex types, such as `&str` and user-defined types.

#### No complex generic expressions in const arguments

Currently, const parameters may only be instantiated by const arguments of the following forms:

- A standalone const parameter.
- A literal (i.e. an integer, bool, or character).
- A concrete constant expression (enclosed by `{}`), involving no generic parameters.

For example:
```rust
fn foo<const N: usize>() {}

fn bar<T, const M: usize>() {
    foo::<M>(); // ok: `M` is a const parameter
    foo::<2021>(); // ok: `2021` is a literal
    foo::<{20 * 100 + 20 * 10 + 1}>(); // ok: const expression contains no generic parameters
    
    foo::<{ M + 1 }>(); // error: const expression contains the generic parameter `M`
    foo::<{ std::mem::size_of::<T>() }>(); // error: const expression contains the generic parameter `T`
    
    let _: [u8; M]; // ok: `M` is a const parameter
    let _: [u8; std::mem::size_of::<T>()]; // error: const expression contains the generic parameter `T`
}
```

## By-value array iterator

In addition to the language changes described above, we've also started adding methods to the standard library taking advantage of const generics. While most are not yet ready for stabilization in this version, there is one method that has been stabilized. [`array::IntoIter`](https://doc.rust-lang.org/nightly/std/array/struct.IntoIter.html) allows arrays to be iterated by value, rather than by reference, addressing a significant shortcoming. There is ongoing discussion about the possibility of implementing `IntoIterator` directly for arrays, though there are [backwards-compatibility concerns](https://github.com/rust-lang/rust/pull/65819) that still have to be addressed. `IntoIter::new` acts as an interim solution that makes working with arrays significantly simpler.

```rust
use std::array;
fn needs_vec(v: Vec<i32>) {
    // ...
}

let arr = [vec![0, 1], vec![1, 2, 3], vec![3]];
for elem in array::IntoIter::new(arr) {
    needs_vec(elem);
}
```

## What's next?

### Const generics and default arguments

Generic parameters must currently come in a specific order: lifetimes, types, consts. However, this causes difficulties when one attempts to use default arguments alongside const parameters. For the compiler to know which generic argument is which, any default arguments need to be placed last. These two constraints - "types come before consts", and "defaults come last" - conflict with each other for definitions that have default type arguments *and* const parameters.

The solution to this is to relax the ordering constraint so that const parameters may precede type arguments. However, there turn out to be subtleties involved in implementing this change, because the Rust compiler currently makes assumptions about parameter ordering that require some delicacy to remove.

In light of similar design questions around defaults for const arguments, these are also currently not supported in version 1.51. However, fixing the parameter ordering issues above will also unblock const defaults.

### Const generics for custom types

For a type to be valid, in theory, as the type of a const parameter, we must be able to compare values of that type at compile-time. Furthermore, equality of values should be well-behaved (namely, it should be deterministic, reflexive, symmetric, and transitive). To guarantee these properties, the concept of *structural equality* was introduced in the [const generics RFC](https://github.com/rust-lang/rfcs/blob/master/text/2000-const-generics.md): essentially this includes any type with `#[derive(PartialEq, Eq)]` whose members also satisfy structural equality.

There are [still some questions](https://github.com/rust-lang/rust/issues/74446) concerning precisely how structural equality should behave, and [prerequisites for implementation](https://github.com/rust-lang/compiler-team/issues/323). Primitive types are significantly simpler, which has allowed us to stabilize const generics for these types before more general types.

### Const generics with complex expressions

There are several complexities involved in supporting complex expressions. A feature flag, `feature(const_evaluatable_checked)`, is available in the Nightly channel, which enables a version of complex expression support for const generics.

One difficulty lies in the necessity of having some way to compare unevaluated constants, as the compiler does not automatically know that two syntactically identical expressions are actually equal. This involves a kind of symbolic reasoning about expressions, which is a complex problem in general.
```rust
// The two expressions `N + 1` and `N + 1` are distinct
// entities in the compiler, so we need a way to check
// if they should be considered equal.
fn foo<const N: usize>() -> [u8; N + 1] {
    [0; N + 1]
}
```

We also want a way to deal with potential errors when evaluating generic operations.
```rust
fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1]) {
    // ...
}

fn generic_function<const M: usize>(arr: [i32; M]) {
    // ...
    let (head, tail) = split_first(arr);
    // ...
}
```
Without a way to restrict the possible values of `M` here, calling `generic_function::<0>()` would cause an error when evaluating `0 - 1` that is not caught at declaration time and so may unexpectedly fail for downstream users.

There are [design questions](https://github.com/rust-lang/rust/issues/68436) about how exactly to express these kinds of bounds, which need to be addressed before stabilising complex const arguments.

## Summary

With such a major new feature, there are likely to be a few rough edges. If you encounter any problems, even if it's as minor as a confusing error message, [please open an issue](https://github.com/rust-lang/rust/issues/new/choose)! We want the user experience to be the best it can possibly be - and any issues now are likely to be even more important for the next iterations of const generics.
