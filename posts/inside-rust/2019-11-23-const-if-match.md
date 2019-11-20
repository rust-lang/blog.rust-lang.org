---
layout: post
title: "`if` and `match` in constants on nightly rust"
author: Dylan MacKenzie
team: WG const-eval <https://github.com/rust-lang/const-eval>
---

**TLDR; `if` and `match` are now usable in constants on the latest nightly.** 

As a result, you can now write code like the following and have it execute at
compile-time:

```rust
static PLATFORM: &str = if cfg!(unix) {
    "unix"
} else if cfg!(windows) {
    "windows"
} else {
    "other"
};

const fn gcd(a: u32, b: u32) -> u32 {
    match (a, b) {
        (x, 0) | (0, x) => x,

        (x, y) if x % 2 == 0 && y % 2 == 0 => 2*gcd(x/2, y/2),
        (x, y) | (y, x) if x % 2 == 0 => gcd(x/2, y),

        (x, y) if x < y => gcd((y-x)/2, x),
        (x, y) => gcd((x-y)/2, y),
    }
}

const _: () = assert!(std::mem::size_of::<usize>() == 8, "Only 64-bit platforms are supported");
```

## What exactly is going on here?

The following expressions,
- `match`
- `if` and `if let`
- `&&` and `||`

can now appear in any of the following contexts,
- `const fn` bodies
- `const` and associated `const` initializers
- `static` and `static mut` initializers
- array initializers
- const generics (EXPERIMENTAL)

if `#![feature(const_if_match)]` is enabled for your crate.

You may have noticed that the short-circuiting logic operators, `&&` and
`||`, were already legal in a `const` or `static`. This was accomplished by
translating them to their non-short-circuiting equivalents, `&` and `|`
respectively. Enabling the feature gate will turn off this hack and make `&&`
and `||` behave as you would expect.

As a side-effect of these changes, the `assert` and `debug_assert` macros
become usable in a const context if `#![feature(const_panic)]` is also
enabled. However, the other assert macros (e.g., `assert_eq`,
`debug_assert_ne`) remain forbidden, since they need to call `Debug::fmt` on
their arguments.

Also forbidden are looping constructs, `while`, `for`, and `loop`, which will
be [feature-gated separately][52000], and the `?` operator, which calls
`From::from` on the value inside the `Err` variant. The design for
`const` trait methods is still being discussed.

[52000]: https://github.com/rust-lang/rust/issues/52000

## What's next?

This change will allow a great number of standard library functions to be
made `const`. If you like, you can help with this process! There's a [list of
numeric functions][const-int] that can be constified with little effort. Be
aware that things move slowly in the standard library, and it's always best
to have a concrete use case that demonstrates why this *particular* function
needs to be callable in a const context.

Personally, I've looked forward to this feature for a long time, and I can't
wait to start playing with it. If you feel the same, I would greatly
appreciate if you tested the limits of this feature! Try to sneak `Cell`s and
types with `Drop` impls into places they shouldn't be allowed, blow up the
stack with poorly implemented recursive functions (see `gcd` above), and let
us know if something goes horribly wrong.

[const-int]: https://github.com/rust-lang/rust/issues/53718

## What took you so long?

[Miri], which rust uses under the hood for compile-time function evaluation,
has been capable of this for a while now. However, rust needs to statically
guarantee certain properties about variables in a `const`, such as whether
they allow for interior mutability or whether they have a `Drop`
implementation that needs to be called. For example, we must reject the
following code since it would result in a `const` being mutable at runtime!

[Miri]: https://github.com/rust-lang/miri

```rust
const CELL: &std::cell::Cell<i32> = &std::cell::Cell::new(42); // Not allowed...

fn main() {
    CELL.set(0);
    println!("{}", CELL.get()); // otherwise this could print `0`!!!
}
```

However, it is sometimes okay for a `const` to contain a *type* that allows
interior mutability, as long as we can prove that the actual *value* of that
type does not. This is particularly useful for `enum`s with a "unit variant"
(e.g., `Option::None`).

```rust
const NO_CELL: Option<&std::cell::Cell<i32>> = None; // OK
```

A more detailed (but non-normative) treatment of the rules [for `Drop`][drop]
and [for interior mutability][interior-mut] in a const context can be found
on the [`const-eval`] repo.

It is not trivial to guarantee properties about the value of a variable when
complex control flow such as loops and conditionals is involved. Implementing
this feature required extending the existing dataflow framework in rust so
that we could properly track the value of each local across the control-flow
graph. At the moment, the analysis is very conservative, especially when values are
moved in and out of compound data types. For example, the following will not
compile, even when the feature gate is enabled.

```rust
const fn imprecise() -> Vec<i32> {
    let tuple: (Vec<i32>) = (Vec::new(),);
    tuple.0
}
```

Even though the `Vec` created by `Vec::new` will never actually be dropped
inside the `const fn`, we don't detect that all fields of `tuple` have been moved
out of, and thus conservatively assume that the drop impl for `tuple` will run.
While this particular case is trivial, there are other, more complex ones that
would require a more expensive solution. It is an open question how precise we
want to be here.

[`const-eval`]: https://github.com/rust-lang/const-eval
[drop]: https://github.com/rust-lang/const-eval/blob/master/static.md#drop
[interior-mut]:
https://github.com/rust-lang/const-eval/blob/master/const.md#2-interior-mutability

## Acknowledgements

TODO
