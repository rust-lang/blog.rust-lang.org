+++
path = "inside-rust/2020/09/29/Portable-SIMD-PG"
title = "Announcing the Portable SIMD Project Group"
authors = ["Jubilee and Lokathor"]
description = "Announcing the Portable SIMD Project Group"
aliases = ["inside-rust/2020/09/29/Portable-SIMD-PG.html"]

[extra]
team = "the library team"
team_url = "https://www.rust-lang.org/governance/teams/library"
+++

We're announcing the start of the _Portable SIMD Project Group_ within the Libs team. This group is dedicated to making a portable SIMD API available to stable Rust users.

The Portable SIMD Project Group is lead by [@calebzulawski](https://github.com/calebzulawski), [@Lokathor](https://github.com/Lokathor), and [@workingjubilee](https://github.com/workingjubilee).

## What are project groups?

Rust uses [project groups](https://rust-lang.github.io/rfcs/2856-project-groups.html) to help coordinate work.
They're a place for people to get involved in helping shape the parts of Rust that matter to them.

## What is SIMD?

SIMD stands for Single Instruction, Multiple Data.
It lets the CPU apply a single instruction to a "vector" of data.
The vector is a single extra-wide CPU register made of multiple "lanes" of the same data type.
You can think of it as being *similar* to an array.
Instead of processing each lane individually, all lanes have the same operation applied *simultaneously*.
This lets you transform data much faster than with standard code.
Not every problem can be accelerated with "vectorized" code, but for multimedia and list-processing applications there can be significant gains.

## Why do you need to make it portable?

Different chip vendors offer different SIMD instructions.
Some of these are available in Rust's [`std::arch`](https://doc.rust-lang.org/core/arch/index.html) module.
You *can* build vectorized functions using that, but at the cost of maintaining a different version for each CPU you want to support.
You can also *not* write vectorized operations and hope that LLVM's optimizations will "auto-vectorize" your code.
However, the auto-vectorizer is easily confused and can fail to optimize "obvious" vector tasks.

The portable SIMD API will enable writing SIMD code just once using a high-level API.
By explicitly communicating your intent to the compiler, it's better able to generate the best possible final code.
This is still only a best-effort process.
If your target doesn't support a desired operation in SIMD, the compiler will fall back to using scalar code, processing one lane at a time.
The details of what's available depend on the build target.

We intend to release the Portable SIMD API as `std::simd`.
We will cover as many use cases as we can, but it might still be appropriate for you to use `std::arch` directly.
For that reason the `std::simd` types will also be easily convertible to `std::arch` types where needed.

## How can I get involved?

Everyone can get involved!
No previous experience necessary.
If you'd like to help make portable SIMD a reality you can visit our [GitHub repository](https://github.com/rust-lang/project-portable-simd) or reach out on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd) and say hi! :wave:
