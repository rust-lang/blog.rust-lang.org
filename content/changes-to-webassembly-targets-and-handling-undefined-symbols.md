+++
path = "2026/04/02/changes-to-webassembly-targets-and-handling-undefined-symbols"
title = "Changes to WebAssembly targets and handling undefined symbols"
authors = ["Alex Crichton"]
+++

Rust's WebAssembly targets are soon going to experience a change which has a
risk of breaking existing projects, and this post is intended to notify users of
this upcoming change, explain what it is, and how to handle it. Specifically, all
WebAssembly targets in Rust have been linked using the `--allow-undefined` flag
to `wasm-ld`, and this flag is being removed.

## What is `--allow-undefined`?

WebAssembly binaries in Rust today are all created by linking with `wasm-ld`.
This serves a similar purpose to `ld`, `lld`, and `mold`, for example; it
takes separately compiled crates/object files and creates one final binary.
Since the first introduction of WebAssembly targets in Rust, the
`--allow-undefined` flag has been passed to `wasm-ld`. This flag is documented
as:

```
  --allow-undefined       Allow undefined symbols in linked binary. This options
                          is equivalent to --import-undefined and
                          --unresolved-symbols=ignore-all
```

The term "undefined" here specifically means with respect to symbol resolution in `wasm-ld` itself. Symbols used by `wasm-ld` correspond relatively closely to what native platforms use, for example all Rust functions have a symbol associated with them. Symbols can be referred to in Rust through `extern "C"` blocks, for example:

```rust
unsafe extern "C" {
    fn mylibrary_init();
}

fn init() {
    unsafe {
        mylibrary_init();
    }
}
```

The symbol `mylibrary_init` is an undefined symbol. This is typically defined by
a separate component of a program, such as an externally compiled C library,
which will provide a definition for this symbol. By passing `--allow-undefined`
to `wasm-ld`, however, it means that the above would generate a WebAssembly
module like so:

```wasm
(module
    (import "env" "mylibrary_init" (func $mylibrary_init))

    ;; ...
)
```

This means that the undefined symbol was ignored and ended up as an imported
symbol in the final WebAssembly module that is produced.

The precise history here is somewhat lost to time, but the current understanding
is that `--allow-undefined` was effectively required in the very early days of
introducing `wasm-ld` to the Rust toolchain. This historical workaround stuck
around till today and hasn't changed.

## What's wrong with `--allow-undefined`?

By passing `--allow-undefined` on all WebAssembly targets, rustc is introducing
diverging behavior between other platforms and WebAssembly. The main risk of
`--allow-undefined` is that misconfiguration or mistakes in building can
result in broken WebAssembly modules being produced, as opposed to compilation
errors. This means that the proverbial can is kicked down the road and lengthens
the distance from where the problem is discovered to where it was introduced.
Some example problematic situations are:

* If `mylibrary_init` was typo'd as `mylibraryinit` then the final binary would
  import the `mylibraryinit` symbol instead of calling the linked
  `mylibrary_init` C symbol.

* If `mylibrary` was mistakenly not compiled and linked into a final
  application then the `mylibrary_init` symbol would end up imported rather than
  producing a linker error saying it's undefined.

* If external tooling is used to process a WebAssembly module, such as `wasm-bindgen` or `wasm-tools component new`, these tools don't know what to do with `"env"` imports by default and they are likely to provide an error message of some form that isn't clearly connected back to the original source code and where the symbols was imported from.

* For web users if you've ever seen an error along the lines of `Uncaught TypeError: Failed to resolve module specifier "env". Relative references must start with either "/", "./", or "../".` this can mean that `"env"` leaked into the final module unexpectedly and the true error is the undefined symbol error, not the lack of `"env"` items provided.

All native platforms consider undefined symbols to be an error by default, and
thus by passing `--allow-undefined` rustc is introducing surprising behavior on
WebAssembly targets. The goal of the change is to remove this surprise and
behave more like native platforms.



## What is going to break, and how to fix?

In theory, not a whole lot is expected to break from this change. If the final
WebAssembly binary imports unexpected symbols, then it's likely that the binary
won't be runnable in the desired embedding, as the desired embedding probably
doesn't provide the symbol as a definition. For example, if you compile an
application for `wasm32-wasip1` if the final binary imports `mylibrary_init`
then it'll fail to run in most runtimes because it's considered an unresolved
import. This means that most of the time this change won't break users, but
it'll instead provide better diagnostics.

The reason for this post, however, is that it's possible users could be
intentionally relying on this behavior. For example your application might have:

```rust
unsafe extern "C" {
    fn js_log(n: u32);
}

// ...
```

And then perhaps some JS code that looks like:

```js
let instance = await WebAssembly.instantiate(module, {
    env: {
        js_log: n => console.log(n),
    }
});
```

Effectively it's possible for users to explicitly rely on the behavior of
`--allow-undefined` generating an import in the final WebAssembly binary.

If users encounter this then the code can be fixed through a `#[link]` attribute
which explicitly specifies the `wasm_import_module` name:

```rust
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn js_log(n: u32);
}

// ...
```

This will have the same behavior as before and will no longer be considered an
undefined symbol to `wasm-ld`, and it'll work both before and after this change.

Affected users can also compile with `-Clink-arg=--allow-undefined` as well to
quickly restore the old behavior.

## When is this change being made?

Removing `--allow-undefined` on wasm targets is being done in
[rust-lang/rust#149868]. That change is slated to land in nightly soon, and will then get released with Rust 1.96 on 2026-05-28. If you see any issues as a
result of this fallout please don't hesitate to file an issue on
[rust-lang/rust].

[rust-lang/rust#149868]: https://github.com/rust-lang/rust/pull/149868
[rust-lang/rust]: https://github.com/rust-lang/rust
