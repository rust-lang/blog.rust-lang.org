+++
layout = "post"
title = "WebAssembly targets: change in default target-features"
author = "Alex Crichton"
team = "The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>"
+++

The Rust compiler has [recently upgraded to using LLVM 19][llvm19] and this
change accompanies some updates to the default set of target features enabled
for WebAssembly targets of the Rust compiler. Beta Rust today, which will
become Rust 1.82 on 2024-10-17, reflects all of these changes and can be
used for testing.

WebAssembly is an evolving standard where extensions are being added over
time through a [proposals process][proposals]. WebAssembly proposals reach
maturity, get merged into the specification itself, get implemented in engines,
and remain this way for quite some time before producer toolchains (e.g. LLVM)
update to **enable these sufficiently-mature proposals by default**. In LLVM 19
this has happened with the [multi-value and reference-types
proposals][llvmenable] for the LLVM/Rust target features `multivalue` and
`reference-types`. These are now enabled by default in LLVM and transitively
means that it's enabled by default for Rust as well.

WebAssembly targets for Rust now [have improved
documentation](https://github.com/rust-lang/rust/pull/128511) about WebAssembly
proposals and their corresponding target features. This post is going to review
these changes and go into depth about what's changing in LLVM.

## WebAssembly Proposals and Compiler Target Features

WebAssembly proposals are the formal means by which the WebAssembly standard
itself is evolved over time. Most proposals need toolchain integration in one
form or another, for example new flags in LLVM or the Rust compiler. The
`-Ctarget-feature=...` mechanism is used to implement this today. This is a
signal to LLVM and the Rust compiler which WebAssembly proposals are enabled or
disabled.

There is a loose coupling between the name of a proposal (often the name of the
github repository of the proposal) and the feature name LLVM/Rust use. For
example there is the [multi-value
proposal](https://github.com/webAssembly/multi-value) but a `multivalue`
feature.

The lifecycle of the implementation of a feature in Rust/LLVM typically looks
like:

1. A new WebAssembly proposal is created in a new repository, for example
   WebAssembly/foo.
2. Eventually Rust/LLVM implement the proposal under `-Ctarget-feature=+foo`
3. Eventually the upstream proposal is merged into the specification, and
   WebAssembly/foo becomes an archived repository
4. Rust/LLVM enable the `-Ctarget-feature=+foo` feature by default but typically
   retain the ability to disable it as well.

The `reference-types` and `multivalue` target features in Rust are at step (4)
here now and this post is explaining the consequences of doing so.

## Enabling Reference Types by Default

The [reference-types proposal to
WebAssembly](https://github.com/webAssembly/reference-types) introduced a few
new concepts to WebAssembly, notably the `externref` type which is a
host-defined GC resource that WebAssembly cannot access but can pass around.
Rust does not have support for the WebAssembly `externref` type and LLVM 19 does
not change that. WebAssembly modules produced from Rust will continue to not use
the `externref` type nor have a means of being able to do so. This may be
enabled in the future (e.g. a hypothetical `core::arch::wasm32::Externref` type
or similar), but it will mostly likely only be done on an opt-in basis
and will not affect preexisting code by default.

Also included in the reference-types proposal, however, was the ability to have
multiple WebAssembly tables in a single module. In the original version of the
WebAssembly specification only a single table was allowed and this restriction
was relaxed with the reference-types proposal. WebAssembly tables are used by
LLVM and Rust to implement indirect function calls. For example function
pointers in WebAssembly are actually table indices and indirect function calls
are a WebAssembly `call_indirect` instruction with this table index.

With the reference-types proposal the binary encoding of `call_indirect`
instructions was updated. Prior to the reference-types proposal `call_indirect`
was encoded with a fixed zero byte in its instruction (required to be exactly
0x00). This fixed zero byte was relaxed to a 32-bit [LEB] to indicate which
table the `call_indirect` instruction was using. For those unfamiliar [LEB] is a
way of encoding multi-byte integers in a smaller number of bytes for smaller
integers. For example the 32-bit integer 0 can be encoded as `0x00` with a
[LEB]. [LEB]s are flexible to additionally allow "overlong" encodings so the
integer 0 can additionally be encoded as `0x80 0x00`.

LLVM's support of separate compilation of source code to a WebAssembly binary
means that when an object file is emitted it does not know the final index of
the table that is going to be used in the final binary. Before reference-types
there was only one option, table 0, so `0x00` was always used when encoding
`call_indirect` instructions. After reference-types, however, LLVM will emit an
over-long [LEB] of the form `0x80 0x80 0x80 0x80 0x00` which is the maximal
length of a 32-bit [LEB]. This [LEB] is then filled in by the linker with a
relocation to the actual table index that is used by the final module.

When putting all of this together, it means that with LLVM 19, which has
the `reference-types` feature enabled by default, any WebAssembly module with an
indirect function call (which is almost always the case for Rust code) will
produce a WebAssembly binary that cannot be decoded by engines and tooling that
do not support the reference-types proposal. It is expected that this change
will have a low impact due to the age of the reference-types proposal and
breadth of implementation in engines. Given the multitude of WebAssembly
engines, however, it's recommended that any WebAssembly users test out
Rust 1.82 beta and see if the produced module still runs on their engine of
choice.

### LLVM, Rust, and Multiple Tables

One interesting point worth mentioning is that despite the reference-types
proposal enabling multiple tables in WebAssembly modules this is not actually
taken advantage of at this time by either LLVM or Rust. WebAssembly modules
emitted will still have at most one table of functions. This means that the
over-long 5-byte encoding of index 0 as `0x80 0x80 0x80 0x80 0x00` is not
actually necessary at this time. LLD, LLVM's linker for WebAssembly, wants to
process all [LEB] relocations in a similar manner which currently forces this
5-byte encoding of zero. For example when a function calls another function the
`call` instruction encodes the target function index as a 5-byte [LEB] which is
filled in by the linker. There is quite often more than one function so the
5-byte encoding enables all possible function indices to be encoded.

In the future LLVM might start using multiple tables as well. For example LLVM
may have a mode in the future where there's a table-per-function type instead of
a single heterogenous table. This can enable engines to implement
`call_indirect` more efficiently. This is not implemented at this time, however.

For users who want a minimally-sized WebAssembly module (e.g. if you're in a web
context and sending bytes over the wire) it's recommended to use an optimization
tool such as [`wasm-opt`] to shrink the size of the output of LLVM. Even before
this change with reference-types it's recommended to do this as [`wasm-opt`] can
typically optimize LLVM's default output even further. When optimizing a module
through [`wasm-opt`] these 5-byte encodings of index 0 are all shrunk to a
single byte.

## Enabling Multi-Value by Default

The second feature enabled by default in LLVM 19 is `multivalue`. The
[multi-value proposal to WebAssembly][multi-value] enables functions to have
more than one return value for example. WebAssembly instructions are
additionally allowed to have more than one return value as well. This proposal
is one of the first to get merged into the WebAssembly specification after the
original MVP and has been implemented in many engines for quite some time.

The consequences of enabling this feature by default in LLVM are more minor for
Rust, however, than enabling the `reference-types` feature by default. LLVM's
default C ABI for WebAssembly code is not changing even when `multivalue` is
enabled.  Additionally Rust's `extern "C"` ABI for WebAssembly is not changing
either and continues to match LLVM's (or strives to, [differences to
LLVM](https://github.com/rust-lang/rust/issues/115666) are considered bugs to
fix). Despite this though the change has the possibility of still affecting
Rust users.

Rust for some time has supported an `extern "wasm"` ABI on Nightly which was an
experimental means of exposing the ability of defining a function in Rust which
returned multiple values (e.g. used the multi-value proposal). Due to
infrastructural changes and refactorings in LLVM itself this feature of Rust has
[been removed](https://github.com/rust-lang/rust/pull/127605) and is no longer
supported on Nightly at all. As a result there is no longer any possible method
of writing a function in Rust that returns multiple values at the WebAssembly
function type level.

In summary this change is expected to not affect any Rust code in the wild
unless you were using the Nightly feature of `extern "wasm"` in which case
you'll be forced to drop support for that and use `extern "C"` instead.
Supporting WebAssembly multi-return functions in Rust is a broader topic than
this post can cover, but at this time it's an area that's ripe for contribution
from suitably motivated contributors.

### Aside: ABI Stability and WebAssembly

While on the topic of ABIs and the `multivalue` feature it's perhaps worth
also going over a bit what ABIs mean for WebAssembly. The current definition of
the `extern "C"` ABI for WebAssembly is documented in the [tool-conventions
repository](https://github.com/WebAssembly/tool-conventions/blob/main/BasicCABI.md)
and this is what Clang implements for C code as well. LLVM implements enough
support for lowering to WebAssembly as well to support all of this. The `extern
"Rust` ABI is not stable on WebAssembly, as is the case for all Rust targets,
and is subject to change over time. There is no reference documentation at this
time for what `extern "Rust"` is on WebAssembly.

The `extern "C"` ABI, what C code uses by default as well, is difficult to
change because stability is often required across different compiler versions.
For example WebAssembly code compiled with LLVM 18 might be expected to work
with code compiled by LLVM 20. This means that changing the ABI is a daunting
task that requires version fields, explicit markers, etc, to help prevent
mismatches.

The `extern "Rust"` ABI, however, is subject to change over time. A great
example of this could be that when the `multivalue` feature is enabled the
`extern "Rust"` ABI could be redefined to use the multiple-return-values that
WebAssembly would then support. This would enable much more efficient returns
of values larger than 64-bits. Implementing this would require support in LLVM
though which is not currently present.

This all means that actually using multiple-returns in functions, or the
WebAssembly feature that the `multivalue` enables, is still out on the horizon
and not implemented. First LLVM will need to implement complete lowering support
to generate WebAssembly functions with multiple returns, and then `extern
"Rust"` can be change to use this when fully supported. In the yet-further-still
future C code might be able to change, but that will take quite some time due to
its cross-version-compatibility story.

## Enabling Future Proposals to WebAssembly

This is not the first time that a WebAssembly proposal has gone from
off-by-default to on-by-default in LLVM, nor will it be the last. For example
LLVM already enables the [sign-extension proposal][sign-ext] by default which
MVP WebAssembly did not have. It's expected that in the not-too-distant future
the
[nontrapping-fp-to-int](https://github.com/WebAssembly/nontrapping-float-to-int-conversions)
proposal will likely be enabled by default. These changes are currently not made
with strict criteria in mind (e.g. N engines must have this implemented for M
years), and there may be breakage that happens.

If you're using a WebAssembly engine that does not support the modules emitted
by Rust 1.82 beta and LLVM 19 then your options are:

* Try seeing if the engine you're using has any updates available to it. You
  might be using an older version which didn't support a feature but a newer
  version supports the feature.
* Open an issue to raise awareness that a change is causing breakage. This could
  either be done on your engine's repository, the Rust repository, or the
  WebAssembly
  [tool-conventions](https://github.com/WebAssembly/tool-conventions)
  repository. It's recommended to first search to confirm there isn't already an
  open issue though.
* Recompile your code with features disabled, more on this in the next section.

The general assumption behind enabling new features by default is that it's a
relatively hassle-free operation for end users while bringing performance
benefits for everyone (e.g. nontrapping-fp-to-int will make float-to-int
conversions more optimal). If updates end up causing hassle it's best to flag
that early on so rollout plans can be adjusted if needed.

## Disabling on-by-default WebAssembly proposals

For a variety of reasons you might be motivated to disable on-by-default
WebAssembly features: for example maybe your engine is difficult to update or
doesn't support a new feature. Disabling on-by-default features is unfortunately
not the easiest task. It is notably not sufficient to use
`-Ctarget-features=-sign-ext` to disable a feature for just your own project's
compilation because the Rust standard library, shipped in precompiled form, is
still compiled with the feature enabled.

To disable on-by-default WebAssembly proposal it's required that you use Cargo's
[`-Zbuild-std`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std)
feature. For example:

```shell
$ export RUSTFLAGS=-Ctarget-cpu=mvp
$ cargo +nightly build -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown
```

This will recompiled the Rust standard library in addition to your own code with
the "MVP CPU" which is LLVM's placeholder for all WebAssembly proposals
disabled. This will disable sign-ext, reference-types, multi-value, etc.

[llvm19]: https://github.com/rust-lang/rust/pull/127513
[proposals]: https://github.com/WebAssembly/proposals
[llvmenable]: https://github.com/llvm/llvm-project/pull/80923
[LEB]: https://en.wikipedia.org/wiki/LEB128
[`wasm-opt`]: https://github.com/WebAssembly/binaryen
[multi-value]: https://github.com/webAssembly/multi-value
[sign-ext]: https://github.com/webAssembly/sign-extension-ops
