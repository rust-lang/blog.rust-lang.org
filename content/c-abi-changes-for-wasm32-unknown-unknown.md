+++
layout = "post"
date = 2024-03-20
title = "C ABI Changes for `wasm32-unknown-unknown`"
author = "Alex Crichton"
+++

The `extern "C"` ABI for the `wasm32-unknown-unknown` target has been using a
non-standard definition since the inception of the target in that it does not
implement the [official C ABI of WebAssembly][tool-conventions] and it
additionally [leaks internal compiler implementation details][leak-details] of
both the Rust compiler and LLVM. This will change in a future version of the
Rust compiler and the [official C ABI][tool-conventions] will be used instead.

[tool-conventions]: https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md
[leak-details]: https://github.com/rust-lang/rust/issues/115666

## History of `wasm32-unknown-unknown`'s C ABI

When the `wasm32-unknown-unknown` target [was originally added][inception] in
2017, not much care was given to the exact definition of the `extern "C"` ABI at
the time. In 2018 [an ABI definition was added just for wasm][orig-abi] and the
target is still using this definition [to this day][current-abi]. This
definitions has become more and more problematic over time and while some issues
have been fixed, the root cause still remains.

Notably this ABI definition does not match the [tool-conventions] definition of
the C API, which is the current standard for how WebAssembly toolchains should
talk to one another. Originally this non-standard definition was used for all
WebAssembly based targets except Emscripten, but [this changed in 2021][fix-wasi]
where the WASI targets for Rust use a corrected ABI definition. Still, however,
the non-standard definition remained in use for `wasm32-unknown-unknown`.

The time has now come to correct this historical mistake and the Rust compiler
will soon be using a correct ABI definition for the `wasm32-unknown-unknown`
target. This means, however, that generated WebAssembly binaries will be
different than before.

## What is a WebAssembly C ABI?

The definition of an ABI answers questions along the lines of:

* What registers are arguments passed in?
* What registers are results passed in?
* How is a 128-bit integers passed as an argument?
* How is a `union` passed as a return value?
* When are parameters passed through memory instead of registers?
* What is the size and alignment of a type in memory?

For WebAssembly these answers are a little different than native platforms.
For example, WebAssembly does not have physical registers and functions must all
be annotated with a type. What WebAssembly does have is types such as `i32`,
`i64`, `f32`, and `f64`. This means that for WebAssembly an ABI needs to define
how to represent values in these types.

This is where the [tool-conventions] document comes in. That document provides a
definition for how to represent primitives in C in the WebAssembly format, and
additionally how function signatures in C are mapped to function signatures in
WebAssembly. For example a Rust `u32` is represented by a WebAssembly `i32` and
is passed directly as a parameter as a function argument. If the Rust structure
`#[repr(C)] struct Pair(f32, f64)` is returned from a function then a return
pointer is used which must have alignment 8 and size of 16 bytes.

In essence, the WebAssembly C ABI is acting as a bridge between C's type system
and the WebAssembly type system. This includes details such as in-memory layouts
and translations of a C function signature to a WebAssembly function signature.

## How is `wasm32-unknown-unknown` non-standard?

Despite the ABI definition today being non-standard, many aspects of it are
still the same as what [tool-conventions] specifies. For example, size/alignment
of types is the same as it is in C. The main difference is how function
signatures are calculated. An example (where you can follow along on [godbolt])
is:

```rust
#[repr(C)]
pub struct Pair {
    x: u32,
    y: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn pair_add(pair: Pair) -> u32 {
    pair.x + pair.y
}
```

This will generate the following WebAssembly function:

```wasm
(func $pair_add (param i32 i32) (result i32)
  local.get 1
  local.get 0
  i32.add
)
```

Notably you can see here that the struct `Pair` was "splatted" into its two
components so the actual `$pair_add` function takes two arguments, the `x` and
`y` fields. The [tool-conventions], however [specifically says] that "other
struct[s] or union[s]" are passed indirectly, notably through memory. We can see
this by compiling this C code:

```c
struct Pair {
    unsigned x;
    unsigned y;
};

unsigned pair_add(struct Pair pair) {
    return pair.x + pair.y;
}
```

which yields the generated function:

```wasm
(func (param i32) (result i32)
  local.get 0
  i32.load offset=4
  local.get 0
  i32.load
  i32.add
)
```

Here we can see, sure enough, that `pair` is passed in linear memory and this
function only has a single argument, not two. This argument is a pointer into
linear memory which stores the `x` and `y` fields.

The Diplomat project has [compiled a much more comprehensive overview][quirks]
than this and it's recommended to check that out if you're curious for an even
deeper dive.

## Why hasn't this been fixed long ago already?

For `wasm32-unknown-unknown` it was well-known at the time in 2021 when WASI's
ABI was updated that the ABI was non-standard. Why then has the ABI not been
fixed like with WASI?
The main reason originally for this was the [wasm-bindgen
project][wasm-bindgen].

In `wasm-bindgen` the goal is to make it easy to integrate Rust into a web
browser with WebAssembly. JavaScript is used to interact with host APIs and the
Rust module itself. Naturally, this communication touches on a lot of ABI
details! The problem was that `wasm-bindgen` relied on the above example,
specifically having `Pair` "splatted" across arguments instead of passed
indirectly. The generated JS wouldn't work correctly if the argument was passed
in-memory.

At the time this was discovered it was found to be significantly difficult to
fix `wasm-bindgen` to not rely on this splatting behavior. At the time it also
wasn't though to be a widespread issue nor was it costly for the compiler to
have a non-standard ABI. Over the years though the pressure has mounted. The
Rust compiler is carrying an [ever-growing list of hacks][leak-details] to work
around the non-standard C ABI on `wasm32-unknown-unknown`. Additionally more
projects have started to rely on this "splatting" behavior and the risk has
gotten greater that there are more unknown projects relying on the non-standard
behavior.

In late 2023 [the wasm-bindgen project fixed bindings generation][wbgfix] to be
unaffected by the transition to the standard definition of `extern "C"`. In the following months
a [future-incompat lint was added to rustc][fcw1] to specifically migrate users
of old `wasm-bindgen` versions to a "fixed" version. This was in anticipation of
changing the ABI of `wasm32-unknown-unknown` once enough time had passed. Since
early 2025 users of old `wasm-bindgen` versions [will now receive a hard
error][hard-error] asking them to upgrade.

Despite all this heroic effort done by contributors, however, it has now come to
light that there are more projects than `wasm-bindgen` relying on this
non-standard ABI definition. Consequently this blog post is intended to serve as
a notice to other users on `wasm32-unknown-unknown` that the ABI break is
upcoming and projects may need to be changed.

## Am I affected?

If you don't use the `wasm32-unknown-unknown` target, you are not affected by
this change. If you don't use `extern "C"` on the `wasm32-unknown-unknown`
target, you are also not affected. If you fall into this bucket, however, you
may be affected!

To determine the impact to your project there are a few tools at your disposal:

* A new [future-incompat warning][fcw2] has been added to the Rust compiler
  which will issue a warning if it detects a signature that will change when the
  ABI is changed.
* In 2023 a [`-Zwasm-c-abi=(legacy|spec)` flag was added][specflag] to the Rust
  compiler. This defaults to `-Zwasm-c-abi=legacy`, the non-standard definition.
  Code can use `-Zwasm-c-abi=spec` to use the standard definition of the C ABI
  for a crate to test out if changes work.

The best way to test your crate is to compile with the most recent Nightly
compiler, ensure there are no warnings, and then test your project still works
with `-Zwasm-c-abi=spec`. If all that passes then you're good to go and the
upcoming change to the C ABI will not affect your project.

## I'm affected, now what?

So you're using `wasm32-unknown-unknown`, you're using `extern "C"`, and the
nightly compiler is giving you warnings. Additionally your project is broken
when compiled with` -Zwasm-c-abi=spec`. What now?

At this time this will unfortunately be a somewhat rough transition period for
you. There are a few options at your disposal but they all have their downsides:

1. Pin your Rust compiler version to the current stable, don't update until the
   ABI has changed. This means that you won't get any compiler warnings (as old
   compilers don't warn) and additionally you won't get broken when the ABI
   changes (as you're not changing compilers). Eventually when you update to a
   stable compiler with `-Zwasm-c-abi=spec` as the default you'll have to port
   your JS or bindings to work with the new ABI.

2. Update to Rust nightly as your compiler and pass `-Zwasm-c-abi=spec`. This is
   front-loading the work required in (1) for your target. You can get your
   project compatible with `-Zwasm-c-abi=spec` today. The downside of this
   approach is that your project will only work with a nightly compiler and
   `-Zwasm-c-abi=spec` and you won't be able to use stable until the default is
   switched.

3. Update your project to not rely on the non-standard behavior of
   `-Zwasm-c-abi=legacy`. This involves, for example, not passing
   structs-by-value in parameters. You can pass `&Pair` above, for example,
   instead of `Pair`. This is similar to (2) above where the work is done
   immediately to update a project but has the benefit of continuing to work on
   stable Rust. The downside of this, however, is that you may not be able to
   easily change or update your C ABI in some situations.

If you have uncertainties, questions, or difficulties, feel free to reach out on
[the tracking issue for the future-incompat warning][tracking] or on Zulip.

## Timeline of ABI changes

At this time there is not an exact timeline of how the default ABI is going to
change. It's expected to take on the order of 3-6 months, however, and will look
something roughly like this:

* 2025 March: (soon) - a [future-incompat warning][fcw2] will be added to the
  compiler to warn projects if they're affected by this ABI change.
* 2025-05-15: this future-incompat warning will reach the stable Rust channel as
  1.87.0.
* 2025 Summer: (ish) - the `-Zwasm-c-abi` flag will be removed from the compiler
  and the `legacy` option will be entirely removed.

Exactly when `-Zwasm-c-abi` is removed will depend on feedback from the
community and whether the future-incompat warning triggers much. It's hoped that
soon after the Rust 1.87.0 is stable, though, that the old legacy ABI behavior
can be removed.

[wbgfix]: https://github.com/rustwasm/wasm-bindgen/pull/3595
[specflag]: https://github.com/rust-lang/rust/pull/117919
[fcw1]: https://github.com/rust-lang/rust/pull/117918
[fcw2]: https://github.com/rust-lang/rust/pull/138601
[hard-error]: https://github.com/rust-lang/rust/pull/133951
[inception]: https://github.com/rust-lang/rust/pull/45905
[orig-abi]: https://github.com/rust-lang/rust/pull/48959
[current-abi]: https://github.com/rust-lang/rust/blob/78948ac259253ce89effca1e8bb64d16f4684aa4/compiler/rustc_target/src/callconv/wasm.rs#L76-L114
[fix-wasi]: https://github.com/rust-lang/rust/pull/79998
[godbolt]: https://godbolt.org/z/fExj4M4no
[conventions-struct]: https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md#function-arguments-and-return-values
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[tracking]: https://github.com/rust-lang/rust/issues/138762
[quirks]: https://github.com/rust-diplomat/diplomat/blob/main/docs/wasm_abi_quirks.md
