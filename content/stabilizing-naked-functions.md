+++
path = "2025/07/03/stabilizing-naked-functions"
title = "Stabilizing naked functions"
authors = ["Folkert de Vries"]
+++

Rust [1.88.0](/2025/06/26/Rust-1.88.0/) stabilizes the `#[unsafe(naked)]` attribute and the `naked_asm!` macro which are used to define naked functions.

A naked function is marked with the `#[unsafe(naked)]` attribute, and its body consists of a single `naked_asm!` call. For example:

```rust
/// SAFETY: Respects the 64-bit System-V ABI.
#[unsafe(naked)]
pub extern "sysv64" fn wrapping_add(a: u64, b: u64) -> u64 {
    // Equivalent to `a.wrapping_add(b)`.
    core::arch::naked_asm!(
        "lea rax, [rdi + rsi]",
        "ret"
    );
}
```

What makes naked functions special — and gives them their name — is that the handwritten assembly block defines the _entire_ function body. Unlike non-naked functions, the compiler does not add any special handling for arguments or return values.

This feature is a more ergonomic alternative to defining functions using `global_asm!`. Naked functions are used in low-level settings like Rust's [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins), operating systems, and embedded applications.

## Why use naked functions?

But wait, if naked functions are just syntactic sugar for `global_asm!`, why add them in the first place?

To see the benefits, let's rewrite the `wrapping_add` example from the introduction using `global_asm!`:

```rust
// SAFETY: `wrapping_add` is defined in this module,
// and expects the 64-bit System-V ABI.
unsafe extern "sysv64" {
    safe fn wrapping_add(a: u64, b: u64) -> u64
}

core::arch::global_asm!(
    r#"
        // Platform-specific directives that set up a function.
        .section .text.wrapping_add,"ax",@progbits
        .p2align 2
        .globl wrapping_add
        .type wrapping_add,@function

wrapping_add:
        lea rax, [rdi + rsi]
        ret

.Ltmp0:
        .size wrapping_add, .Ltmp0-wrapping_add
    "#
);
```

The assembly block starts and ends with the directives (`.section`, `.p2align`, etc.) that are required to define a function. These directives are mechanical, but they are different between object file formats. A naked function will automatically emit the right directives.

Next, the `wrapping_add` name is hardcoded, and will not participate in Rust's name mangling. That makes it harder to write cross-platform code, because different targets have different name mangling schemes (e.g. x86_64 macOS prefixes symbols with `_`, but Linux does not). The unmangled symbol is also globally visible — so that the `extern` block can find it — which can cause symbol resolution conflicts. A naked function's name does participate in name mangling and won't run into these issues.

A further limitation that this example does not show is that functions defined using global assembly cannot use generics. Especially const generics are useful in combination with assembly.

Finally, having just one definition provides a consistent place for (safety) documentation and attributes, with less risk of them getting out of date. Proper safety comments are essential for naked functions. The `naked` attribute is unsafe because the ABI (`sysv64` in our example), the signature, and the implementation have to be consistent.

## How did we get here?

Naked functions have been in the works for a long time.

The [original RFC](https://github.com/rust-lang/rfcs/pull/1201) for naked functions is from 2015. That RFC was superseded by [RFC 2972](https://github.com/rust-lang/rfcs/pull/2972) in 2020. Inline assembly in Rust had changed substantially at that point, and the new RFC limited the body of naked functions to a single `asm!` call with some additional constraints. And now, 10 years after the initial proposal, naked functions are stable.

Two additional notable changes helped prepare naked functions for stabilization:

##### Introduction of the `naked_asm!` macro

The body of a naked function must be a single `naked_asm!` call. This macro is a blend between `asm!` (it is in a function body) and `global_asm!` (only some [operand types](https://doc.rust-lang.org/1.88.0/reference/inline-assembly.html#r-asm.operand-type) are accepted).

The initial implementation of RFC 2972 added lints onto a standard `asm!` call in a naked function. This approach made it hard to write clear error messages and documentation. With the dedicated `naked_asm!` macro the behavior is much easier to specify.

##### Lowering to `global_asm!`

The initial implementation relied on LLVM to lower functions with the `naked` attribute for code generation. This approach had two issues:

- LLVM would sometimes add unexpected additional instructions to what the user wrote.
- Rust has non-LLVM code generation backends now, and they would have had to implement LLVM's (unspecified!) behavior.

The implementation that is stabilized now instead converts the naked function into a piece of global assembly. The code generation backends can already emit global assembly, and this strategy guarantees that the whole body of the function is just the instructions that the user wrote.

## What's next for assembly?

We're working on further assembly ergonomics improvements. If naked functions are something you are excited about and (may) use, we'd appreciate you testing these new features and providing feedback on their designs.

##### `extern "custom"` functions

Naked functions usually get the `extern "C"` calling convention. But often that calling convention is a lie. In many cases, naked functions don't implement an ABI that Rust knows about. Instead they use some custom calling convention that is specific to that function.

The [`abi_custom`](https://github.com/rust-lang/rust/issues/140829) feature adds `extern "custom"` functions and blocks, which allows us to correctly write code like this example from [compiler-builtins](https://github.com/rust-lang/compiler-builtins/blob/267ae1fa43785448bfb0aebafc4e352c936dd4cf/compiler-builtins/src/arm.rs#L52-L63):

```rust
#![feature(abi_custom)]

/// Division and modulo of two numbers using Arm's nonstandard ABI.
///
/// ```c
/// typedef struct { int quot; int rem; } idiv_return;
///  __value_in_regs idiv_return __aeabi_idivmod(int num, int denom);
/// ```
// SAFETY: The assembly implements the expected ABI, and "custom"
// ensures this function cannot be called directly.
#[unsafe(naked)]
pub unsafe extern "custom" fn __aeabi_idivmod() {
    core::arch::naked_asm!(
        "push {{r0, r1, r4, lr}}", // Back up clobbers.
        "bl {trampoline}",         // Call an `extern "C"` function for a / b.
        "pop {{r1, r2}}",
        "muls r2, r2, r0",         // Perform the modulo.
        "subs r1, r1, r2",
        "pop {{r4, pc}}",          // Restore clobbers, implicit return by setting `pc`.
        trampoline = sym crate::arm::__aeabi_idiv,
    );
}
```

A consequence of using a custom calling convention is that such functions cannot be called using a Rust call expression; the compiler simply does not know how to generate correct code for such a call. Instead the compiler will error when the program does try to call an `extern "custom"` function, and the only way to execute the function is using inline assembly.

##### `cfg` on lines of inline assembly

The [`cfg_asm`](https://github.com/rust-lang/rust/issues/140364) feature adds the ability to annotate individual lines of an assembly block with `#[cfg(...)]` or `#[cfg_attr(..., ...)]`. Configuring specific sections of assembly is useful to make assembly depend on, for instance, the target, target features, or feature flags. For example:

```rust
#![feature(cfg_asm)]

global_asm!(
    // ...

    // If enabled, initialise the SP. This is normally
    // initialised by the CPU itself or by a bootloader, but
    // some debuggers fail to set it when resetting the
    // target, leading to stack corruptions.
    #[cfg(feature = "set-sp")]
    "ldr r0, =_stack_start
     msr msp, r0",

     // ...
)
```

This example is from the [cortex-m](https://github.com/rust-embedded/cortex-m/blob/c3d664bba1148cc2d0f963ebeb788aa347ba81f7/cortex-m-rt/src/lib.rs#L528-L636) crate that currently has to use a custom macro that duplicates the whole assembly block for every use of `#[cfg(...)]`. With `cfg_asm`, that will no longer be necessary.
