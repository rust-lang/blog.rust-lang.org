---
layout: post
title: "Constant propagation is now on by default in nightly"
author: "Wesley Wiser"
description: "Constant propagation is now on by default in nightly"
team: the MIR Optimizations WG <https://rust-lang.github.io/compiler-team/working-groups/mir-opt/>
---

I'm pleased to announce that the [Mid-level IR][mir] (MIR) constant propagation pass has been [switched on][pr] by default on Rust nightly which will eventually become Rust 1.41!

## What is constant propagation?

Constant propagation is an optimization where the compiler recognizes code that can be run at compile time, evaluates it, and replaces the original code with the result.

For example:

```rust
const X: u32 = 2;

let y = X + X;
```

Rather than evaluating `X + X` at runtime, the compiler can recognize that the value of `X` is known at compile time and replace it with the correct value resulting in:

```rust
const X: u32 = 2;

let y = 4;
```

This optimization is opportunistic and automatically recognizes constants even when they are not declared as such:

```rust
struct Point {
  x: u32,
  y: u32,
}

let a = 2 + 2; // optimizes to 4
let b = [0, 1, 2, 3, 4, 5][3]; // optimizes to 3
let c = (Point { x: 21, y: 42 }).y; // optimizes to 42
```

## Propagation into control flow

The constant propagation pass also handles propagating into control flow.
For example:

```rust
const Foo: Option<u8> = Some(12);

let x = match Foo {
   None => panic!("no value"),
   Some(v) => v,
};
```

becomes:

```rust
const Foo: Option<u8> = Some(12);

let x = 12;
```

This is very helpful for checked math, the default in `debug` mode, which introduces additional control flow after every operation:

```rust
let x = 2 + 4 * 6;
```

actually operates like this with overflow checking enabled:

```rust
let (_tmp0, overflowed) = CheckedMultiply(4, 6);
assert!(!overflowed, "attempt to multiply with overflow");

let (_tmp1, overflowed) = CheckedAdd(_tmp0, 2);
assert!(!overflowed, "attempt to add with overflow");

let x = _temp1;
```

which adds quite a bit of control flow!
Constant propagation evaluates the math at compile time and reduces this to:

```rust
let _tmp0 = 24;
assert!(!false, "attempt to multiply with overflow");

let _tmp1 = 26;
assert!(!false, "attempt to add with overflow");

let x = 26;
```

which is further reduced to just:

```rust
let x = 26;
```

## Compiler performance

As you might have guessed, reducing the amount of control flow processed by the Rust compiler has a positive effect on compile times.
We're seeing 2-10% improvement on a variety of test cases in both debug and release mode.
Even though LLVM has its own constant propagation pass, we see improvements because our pass operates on MIR while it is still generic.
The more concrete instances of a generic function that are instantiated, the larger the payoff from this optimization.

We've suspected for a while that the verbose LLVM IR the Rust compiler generates contributes considerably to long compilation times.
By implementing optimizations like this, we believe there is significant potential to lower compile times by generating better LLVM IR.
If you'd like to get involved with the MIR Optimizations working group, stop by our [Zulip channel][zulip] and say hello!

[mir]: https://blog.rust-lang.org/2016/04/19/MIR.html
[pr]: https://github.com/rust-lang/rust/pull/66074
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/189540-t-compiler.2Fwg-mir-opt
