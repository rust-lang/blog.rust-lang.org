---
layout: post
title: "std::simd available in nightly"
author: Caleb Zulawski and Jubilee Young
team: the Portable SIMD Project Group <https://blog.rust-lang.org/inside-rust/2020/09/29/Portable-SIMD-PG.html>
---

The [*Portable SIMD Project Group*](https://blog.rust-lang.org/inside-rust/2020/09/29/Portable-SIMD-PG.html) has been working on the initial implementation of the `std::simd` module over the past year, and it is now ready for community testing in nightly Rust.

To enable the module, use `#![feature(portable_simd)]`.

## How it works

Most of the module's functionality is contained in the `std::simd::Simd` type:

```rust
struct Simd<T, const LANES: usize>(..);
```

This type is similar to `[T; LANES]`, but uses SIMD registers and instructions when possible.

Today, the valid types of `T` are all primitive integer and floating point number types, excluding `u128` and `i128`.  In the future, it's expected to also accept raw pointers.

This design allows SIMD vectors to be treated like containers, much like arrays or `Vec`, yet allow easy manipulation of their contained values. `Simd` implements functions and operators that mirror those used on the scalar values, expanding to work simultaneously on each element in the vector.

## What is "portable", really?

SIMD instructions are typically made available by vendor intrinsic functions (the `std::arch` module in Rust).  Intrinsic functions can allow low-level access to the CPU and may result in much faster code, but they come with a few downsides:
* Most vendor intrinsic functions are `unsafe`
* They can be cumbersome for simple tasks
* Your carefully optimized code will only work on one target platform!

SIMD instruction sets are exceptionally complicated, but they share many basic operations.  Instead of providing particular instructions, `std::simd` contains types and functions that implement this common ground.  Everything in `std::simd` works on every target, which means any particular function may compile to one or several SIMD instructions, or non-SIMD instructions if there is no SIMD equivalent for the target.

Occasionally it may be desirable to stray beyond the common ground and use target-specific instructions.  For those situations, `std::simd` vectors can be converted to `std::arch` vectors and vice versa using `From` and `Into`.

## An example

With `std::arch`, you can already use SIMD like this:

```rust
#[cfg(target_arch = "x86")]
use std::arch::x86::{_mm_add_ps, _mm_loadu_ps, _mm_setzero_ps};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_add_ps, _mm_loadu_ps, _mm_setzero_ps};

/// Can only be called if SSE2 is available (most x86_64 and some x86).
#[cfg(all(
    target_feature = "sse2",
    any(target_arch = "x86", target_arch = "x86_64")
))]
fn sum(values: &[f32]) -> f32 {
    assert!(values.len() % 4 == 0);

    // SAFETY: This function will only compile for an x86 architecture
    // which has SSE2 available and enabled.
    unsafe {
        // Compute parallel sums
        let mut sums = _mm_setzero_ps();
        for chunk in values.chunks(4) {
            // _mm_load_ps can be faster, but we don't know our alignment,
            // and violating alignment requirements would be UB!
            sums = _mm_add_ps(sums, _mm_loadu_ps(chunk.as_ptr()));
        }

        // Sum the sums!
        let sums: [f32; 4] = std::mem::transmute(sums);
        sums.iter().copied().sum()
    }
}
```

With Portable SIMD, you can write a similar function that uses no `unsafe` and works on every target:

```rust
#![feature(portable_simd)]
use std::simd::f32x4; // equivalent to Simd<f32, 4>

fn sum(values: &[f32]) -> f32 {
    assert!(values.len() % 4 == 0);
    
    // Compute parallel sums
    let mut sums = f32x4::splat(0.);
    for chunk in values.chunks(4) {
        sums += f32x4::from_slice(chunk);
    }
        
    // Sum the sums!
    sums.horizontal_sum()
}
```

## Setting target features for best results

If you want to make sure that the instructions emitted include the latest SIMD instructions your target supports, make sure to set `RUSTFLAGS`! On Linux, Mac, or other Unix-like platforms, this can be done with:

```bash
env RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```

On Windows PowerShell, you can use:

```powershell
$Env:RUSTFLAGS="-Ctarget-cpu=native"
cargo build --release
```

You can also use `"-Ctarget-feature=+${FEATURE1}, +${FEATURE2}"` instead, so your code is portable to all processors which support at least those features.

Functions using this API are **not** implicitly multiversioned for every single CPU feature an architecture may possess, as rustc at the moment does not know what differences are relevant. Instead, multiversioning can be done by a programmer using the `is_x86_feature_detected!` macro (or your target's variant thereof) and `#[target_feature(enable)]`. Community crates like `multiversion` offer a way to do this more easily.

## Getting involved

We'd love to hear any comments, suggestions, or bug reports.  If you'd like to contribute, you can visit our [GitHub repository](https://github.com/rust-lang/portable-simd) or reach out on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd) and say hi!
