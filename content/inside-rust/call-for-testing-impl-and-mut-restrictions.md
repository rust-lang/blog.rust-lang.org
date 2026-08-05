+++
path = "inside-rust/2026/08/10/call-for-testing-impl-and-mut-restrictions"
title = "Call for testing: Restricting trait implementability and field mutability"
authors = ["Ryosuke Yamano"]

[extra]
team = "The Compiler Team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

We are excited to announce that [RFC 3323](https://rust-lang.github.io/rfcs/3323-restrictions.html) "Restrictions" is ready for testing on nightly Rust. In this post, we will briefly describe the features. If you are already familiar with them, you can skip ahead to the [How can I help?](#how-can-i-help) section.

The RFC is split into two features: `impl_restriction` and `mut_restriction`.

## What is `impl_restriction`?

The `impl_restriction` feature allows explicit restriction of the scope in which a trait may be implemented.

For example, consider a trait `Foo` with a method that we want users to be able to call, while preventing downstream crates from providing their own implementations. With this feature, we can write:

```rust
#![feature(impl_restriction)]

pub impl(crate) trait Foo {
    fn method();
}

impl Foo for usize {
    fn method() {}
}
```

The `impl(crate)` restriction prevents `Foo` from being implemented outside the current crate. As with `pub`, other paths can also be specified, such as `impl(super)` or `impl(in path)`.

Without this feature, this use case is typically handled using the sealed trait pattern, which is described in the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed).

In short, this pattern defines a public `Sealed` trait inside a private module and makes it a supertrait of `Foo`. Because downstream crates cannot name `Sealed`, they cannot implement `Foo`.

```rust
pub trait Foo: private::Sealed {
    fn method();
}

// Implement `Foo` for selected types.
impl Foo for usize {
    fn method() {}
}

mod private {
    pub trait Sealed {}

    // Implement `Sealed` for those same types, but no others.
    impl Sealed for usize {}
}
```

However, this pattern requires defining an additional `Sealed` trait. The new `impl_restriction` feature provides a more direct and concise way to express the same restriction.

The feature also allows the compiler to produce a more direct error message when an implementation is attempted outside the permitted scope.

For example, the following code:

```rust
#![feature(impl_restriction)]

pub mod foo {
    pub mod bar {
        pub(crate) impl(super) trait Foo {}
    }

    // `Foo` may be implemented here.
    impl bar::Foo for i8 {}
}

// Error: `Foo` cannot be implemented here.
impl foo::bar::Foo for u8 {}
```

results in the following error:

```text
error: trait cannot be implemented outside `crate::foo`
  --> src/lib.rs:12:1
   |
12 | impl foo::bar::Foo for u8 {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: trait restricted here
  --> src/lib.rs:5:20
   |
 5 |         pub(crate) impl(super) trait Foo {}
   |                    ^^^^^^^^^^^
```

## What is `mut_restriction`?

The `mut_restriction` feature allows explicit restriction of the scope in which a field may be mutated.

For example, consider a struct `Bar` with a field `alpha` that can be read from any crate but can only be mutated within the crate where `Bar` is defined. With this feature, we can write:

```rust
#![feature(mut_restriction)]

pub struct Bar {
    pub mut(crate) alpha: u8,
}
```

The `mut(crate)` restriction prevents `alpha` from being mutated outside the current crate. As with `pub`, other paths can also be specified, such as `mut(super)` or `mut(in path)`.

This provides an alternative to getter methods by allowing read-only access through direct field access. It can also work better with the borrow checker, which can track borrows at the field level and allow disjoint fields to be borrowed independently.

When code attempts to mutate a restricted field outside the permitted scope, the compiler produces a direct error message. For example, the following code:

```rust
#![feature(mut_restriction)]

pub mod foo {
    pub struct Bar {
        pub mut(self) alpha: &'static str,
    }

    impl Bar {
        pub fn mutate_inner(&mut self) {
            // `alpha` may be mutated here.
            self.alpha = "inner";
        }
    }
}

impl foo::Bar {
    pub fn mutate_outer(&mut self) {
        // Error: `alpha` cannot be mutated here.
        self.alpha = "outer";
    }
}
```

results in the following error:

```text
error: field `alpha` cannot be mutated outside `crate::foo`
  --> src/lib.rs:19:9
   |
 5 |         pub mut(self) alpha: &'static str,
   |             --------- field restricted here
...
19 |         self.alpha = "outer";
   |         ^^^^^^^^^^^^^^^^^^^^
```

The `mut_restriction` feature can also be used with fields of enum variants and unions. For example, we can write:

```rust
pub enum Foo {
    Alpha { mut(crate) x: u8 },
    Beta(mut(crate) u8), // We can also use tuple-style
}

pub union Bar {
    pub mut(crate) i: i32,
    pub f: f32,
}

pub struct Baz(pub mut(crate) u8); // We can also use tuple-style
```

### Restricting construction with struct expressions

A common reason to use `mut`-restricted fields is to preserve an invariant. However, allowing unrestricted construction with struct expressions would let users create values that have not been validated, potentially violating that invariant.

Thus, we disallow struct expressions if any `mut`-restricted field cannot be mutated from the current scope. For example, the following code:

```rust
#![feature(mut_restriction)]

pub mod foo {
    pub struct Baz {
        pub alpha: u8,
        pub mut(self) beta: u8,
    }
}

fn main() {
    let bar = foo::Baz { alpha: 0, beta: 1 };
}
```

results in the following error:

```text
error: `Baz` cannot be constructed using a `struct` expression outside `crate::foo`
  --> src/main.rs:11:15
   |
 6 |         pub mut(self) beta: u8,
   |             --------- field restricted here
...
11 |     let bar = foo::Baz { alpha: 0, beta: 1 };
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## How can I help?

We'd love for you to try out these new features and share your feedback! To get started, use the latest Rust nightly compiler[^1] and enable `#![feature(impl_restriction)]` or `#![feature(mut_restriction)]`.

[^1]: Make sure to run `rustup update nightly` first, as these features are very new and may not be available on an older nightly toolchain.

Please share your feedback in the [dedicated feedback issue](https://github.com/rust-lang/rust/issues/160614).

In particular, the RFC lists several [unresolved questions](https://rust-lang.github.io/rfcs/3323-restrictions.html#unresolved-questions). One of the main open questions concerns the syntax, which is still open to discussion. Please try out the features and let us know what you think!

## Acknowledgements

These features were implemented as part of a [Google Summer of Code 2026 project](https://summerofcode.withgoogle.com/programs/2026/projects/xFrskRCv). I would like to thank my mentors, [Jacob Pratt](https://github.com/jhpratt) and [Urgau](https://github.com/Urgau), for their guidance and support throughout the project.
