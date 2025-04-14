+++
path = "inside-rust/2025/03/05/inferred-const-generic-arguments"
title = "Inferred const generic arguments: Call for Testing!"
authors = ["BoxyUwU"]
aliases = ["inside-rust/2025/03/05/inferred-const-generic-arguments.html"]

[extra]
team = "The Const Generics Project Group"
team_url = "https://rust-lang.github.io/project-const-generics/"
+++

We are excited to announce that `feature(generic_arg_infer)` is nearing the point of stabilization. In this post we'd like to talk a bit about what this feature does, and what comes next for it.

## What is `feature(generic_arg_infer)`

When `feature(min_const_generics)` was [stabilized in early 2021](https://github.com/rust-lang/rust/pull/79135) it did not include the ability to use `_` as an explicit const argument:
```rust
fn foo() {
  // This errors due to `_` as an array length being unsupported
  let a: [u8; _] = [Default::default()];
  // This is legal as `_` is permitted as a type argument
  let b: [_; 1] = a;
}
```

This is entirely a syntactic limitation; it is possible to entirely elide generic argument listings that may involve const arguments:
```rust
fn foo<const N: usize>(_: [u8; N]) {}

fn bar() {
  // This errors due to `_` as a const argument being unsupported
  foo::<_>([1]);
  // This is legal as even though the const argument is *inferred*
  // there is no explicit `_` written.
  foo([1]);
}
```

The compiler has always been able to infer values for const generic parameters, only the ability to explicitly ask for a const argument to be inferred is unstable.

It is currently also not possible to the infer the length of a repeat expression. Doing so would require moving the expression into a separate function generic over the array length.

```rust
fn foo() {
    // This errors due to `_` as a repeat count being unsupported
    let a: [_; 1] = [String::new(); _];
}
```

With `feature(generic_arg_infer)` all of the previous examples compile. This should hopefully feel like something that should "obviously" be supported by Rust.

## What comes next

We have [significantly reworked the implementation](https://github.com/rust-lang/rust/pull/135272) of this recently and it should now be ready for stabilization. We'd love for you to try it out on a recent nightly and report any issues you encounter.

## Acknowledgements

My recent push to make this feature ready for testing would not have been possible without the help of many others.

A big thank you to [@lcnr][lcnr] and [@JulianKnodt][JulianKnodt] for the initial implementation of `generic_arg_infer`, [@camelid][camelid] for refactoring our representation of const generic arguments to be more flexible, [@voidc][voidc] for helping unify the way we operate on array lengths and const generic arguments, [@lcnr][lcnr] for design work on abstracting away differences between inferred type/const/generic arguments, and finally [@compiler-errors][compiler-errors] for reviewing many PRs and implementation decisions made as part of work on this feature.

[lcnr]: https://github.com/lcnr
[JulianKnodt]: https://github.com/JulianKnodt
[camelid]: https://github.com/camelid
[voidc]: https://github.com/voidc
[compiler-errors]: https://github.com/compiler-errors
