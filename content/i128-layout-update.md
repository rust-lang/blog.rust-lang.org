+++
path = "2024/03/30/i128-layout-update"
title = "Changes to `u128`/`i128` layout in 1.77 and 1.78"
authors = ["Trevor Gross"]
aliases = ["2024/03/30/i128-layout-update.html"]

[extra]
team = "The Rust Lang Team"
team_url = "https://www.rust-lang.org/governance/teams/lang"
+++

Rust has long had an inconsistency with C regarding the alignment of 128-bit integers
on the x86-32 and x86-64 architectures. This problem has recently been resolved, but
the fix comes with some effects that are worth being aware of.

As a user, you most likely do not need to worry about these changes unless you are:

1. Assuming the alignment of `i128`/`u128` rather than using `align_of`
1. Ignoring the `improper_ctypes*` lints and using these types in FFI

There are also no changes to architectures other than x86-32 and x86-64. If your
code makes heavy use of 128-bit integers, you may notice runtime performance increases
at a possible cost of additional memory use.

This post documents what the problem was, what changed to fix it, and what to expect
with the changes. If you are already familiar with the problem and only looking for a
compatibility matrix, jump to the [Compatibility](#compatibility) section.

# Background

Data types have two intrinsic values that relate to how they can be arranged in memory;
size and alignment. A type's size is the amount of space it takes up in memory, and its
alignment specifies which addresses it is allowed to be placed at.

The size of simple types like primitives is usually unambiguous, being the exact size of
the data they represent with no padding (unused space). For example, an `i64` always has
a size of 64 bits or 8 bytes.

Alignment, however, can vary. An 8-byte integer _could_ be stored at any memory address
(1-byte aligned), but most 64-bit computers will get the best performance if it is
instead stored at a multiple of 8 (8-byte aligned). So, like in other languages,
primitives in Rust have this most efficient alignment by default. The effects of this
can be seen when creating composite types ([playground link][composite-playground]):

```rust
use core::mem::{align_of, offset_of};

#[repr(C)]
struct Foo {
    a: u8,  // 1-byte aligned
    b: u16, // 2-byte aligned
}

#[repr(C)]
struct Bar {
    a: u8,  // 1-byte aligned
    b: u64, // 8-byte aligned
}

println!("Offset of b (u16) in Foo: {}", offset_of!(Foo, b));
println!("Alignment of Foo: {}", align_of::<Foo>());
println!("Offset of b (u64) in Bar: {}", offset_of!(Bar, b));
println!("Alignment of Bar: {}", align_of::<Bar>());
```

Output:

```
Offset of b (u16) in Foo: 2
Alignment of Foo: 2
Offset of b (u64) in Bar: 8
Alignment of Bar: 8
```

We see that within a struct, a type will always be placed such that its offset is a
multiple of its alignment - even if this means unused space (Rust minimizes this by
default when `repr(C)` is not used).

These numbers are not arbitrary; the application binary interface (ABI) says what they
should be. In the x86-64 [psABI] (processor-specific ABI) for System V (Unix & Linux),
_Figure 3.1: Scalar Types_ tells us exactly how primitives should be represented:

| C type               | Rust equivalent | `sizeof` | Alignment (bytes) |
| -------------------- | --------------- | -------- | ----------------- |
| `char`               | `i8`            | 1        | 1                 |
| `unsigned char`      | `u8`            | 1        | 1                 |
| `short`              | `i16`           | 2        | 2                 |
| **`unsigned short`** | **`u16`**       | **2**    | **2**             |
| `long`               | `i64`           | 8        | 8                 |
| **`unsigned long`**  | **`u64`**       | **8**    | **8**             |

The ABI only specifies C types, but Rust follows the same definitions both for
compatibility and for the performance benefits.

# The Incorrect Alignment Problem

If two implementations disagree on the alignment of a data type, they cannot reliably
share data containing that type. Rust had inconsistent alignment for 128-bit types:

```rust
println!("alignment of i128: {}", align_of::<i128>());
```

```
// rustc 1.76.0
alignment of i128: 8
```

```c
printf("alignment of __int128: %zu\n", _Alignof(__int128));
```

```
// gcc 13.2
alignment of __int128: 16

// clang 17.0.1
alignment of __int128: 16
```

([Godbolt link][align-godbolt]) Looking back at the [psABI], we can see that Rust has
the wrong alignment here:

| C type              | Rust equivalent | `sizeof` | Alignment (bytes) |
| ------------------- | --------------- | -------- | ----------------- |
| `__int128`          | `i128`          | 16       | 16                |
| `unsigned __int128` | `u128`          | 16       | 16                |

It turns out this isn't because of something that Rust is actively doing incorrectly:
layout of primitives comes from the LLVM codegen backend used by both Rust and Clang,
among other languages, and it has the alignment for `i128` hardcoded to 8 bytes.

Clang uses the correct alignment only because of a workaround, where the alignment is
manually set to 16 bytes before handing the type to LLVM. This fixes the layout issue
but has been the source of some other minor problems.[^f128-segfault][^va-segfault]
Rust does no such manual adjustment, hence the issue reported at
<https://github.com/rust-lang/rust/issues/54341>.

# The Calling Convention Problem

There is an additional problem: LLVM does not always do the correct thing when passing
128-bit integers as function arguments. This was a [known issue in LLVM], before its
[relevance to Rust was discovered].

When calling a function, the arguments get passed in registers (special storage
locations within the CPU) until there are no more slots, then they get "spilled" to
the stack (the program's memory). The ABI tells us what to do here as well, in the
section _3.2.3 Parameter Passing_:

> Arguments of type `__int128` offer the same operations as INTEGERs, yet they do not
> fit into one general purpose register but require two registers. For classification
> purposes `__int128` is treated as if it were implemented as:
>
> ```c
> typedef struct {
>     long low, high;
> } __int128;
> ```
>
> with the exception that arguments of type `__int128` that are stored in memory must be
> aligned on a 16-byte boundary.

We can try this out by implementing the calling convention manually. In the below C
example, inline assembly is used to call `foo(0xaf, val, val, val)` with `val` as
`0x11223344556677889900aabbccddeeff`.

x86-64 uses the registers `rdi`, `rsi`, `rdx`, `rcx`, `r8`, and `r9` to pass function
arguments, in that order (you guessed it, this is also in the ABI). Each register
fits a word (64 bits), and anything that doesn't fit gets `push`ed to the stack.

```c
/* full example at <https://godbolt.org/z/5c8cb5cxs> */

/* to see the issue, we need a padding value to "mess up" argument alignment */
void foo(char pad, __int128 a, __int128 b, __int128 c) {
    printf("%#x\n", pad & 0xff);
    print_i128(a);
    print_i128(b);
    print_i128(c);
}

int main() {
    asm(
        /* load arguments that fit in registers */
        "movl    $0xaf, %edi \n\t"                /* 1st slot (edi): padding char (`edi` is the
                                                   * same as `rdi`, just a smaller access size) */
        "movq    $0x9900aabbccddeeff, %rsi \n\t"  /* 2nd slot (rsi): lower half of `a` */
        "movq    $0x1122334455667788, %rdx \n\t"  /* 3rd slot (rdx): upper half of `a` */
        "movq    $0x9900aabbccddeeff, %rcx \n\t"  /* 4th slot (rcx): lower half of `b` */
        "movq    $0x1122334455667788, %r8  \n\t"  /* 5th slot (r8):  upper half of `b` */
        "movq    $0xdeadbeef4c0ffee0, %r9  \n\t"  /* 6th slot (r9):  should be unused, but
                                                   * let's trick clang! */

        /* reuse our stored registers to load the stack */
        "pushq   %rdx \n\t"                       /* upper half of `c` gets passed on the stack */
        "pushq   %rsi \n\t"                       /* lower half of `c` gets passed on the stack */

        "call    foo \n\t"                        /* call the function */
        "addq    $16, %rsp \n\t"                  /* reset the stack */
    );
}
```

Running the above with GCC prints the following expected output:

```
0xaf
0x11223344556677889900aabbccddeeff
0x11223344556677889900aabbccddeeff
0x11223344556677889900aabbccddeeff
```

But running with Clang 17 prints:

```
0xaf
0x11223344556677889900aabbccddeeff
0x11223344556677889900aabbccddeeff
0x9900aabbccddeeffdeadbeef4c0ffee0
//^^^^^^^^^^^^^^^^ this should be the lower half
//                ^^^^^^^^^^^^^^^^ look familiar?
```

Surprise!

This illustrates the second problem: LLVM expects an `i128` to be passed half in a
register and half on the stack when possible, but this is not allowed by the ABI.

Since the behavior comes from LLVM and has no reasonable workaround, this is a
problem in both Clang and Rust.

# Solutions

Getting these problems resolved was a lengthy effort by many people, starting with a
patch by compiler team member Simonas Kazlauskas in 2017: [D28990]. Unfortunately,
this wound up reverted. It was later attempted again in [D86310] by LLVM contributor
Harald van Dijk, which is the version that finally landed in October 2023.

Around the same time, Nikita Popov fixed the calling convention issue with [D158169].
Both of these changes made it into LLVM 18, meaning all relevant ABI issues will be
resolved in both Clang and Rust that use this version (Clang 18 and Rust 1.78 when using
the bundled LLVM).

However, `rustc` can also use the version of LLVM installed on the system rather than a
bundled version, which may be older. To mitigate the chance of problems from differing
alignment with the same `rustc` version, [a proposal] was introduced to manually
correct the alignment like Clang has been doing. This was implemented by Matthew Maurer
in [#116672].

Since these changes, Rust now produces the correct alignment:

```rust
println!("alignment of i128: {}", align_of::<i128>());
```

```
// rustc 1.77.0
alignment of i128: 16
```

As mentioned above, part of the reason for an ABI to specify the alignment of a datatype
is because it is more efficient on that architecture. We actually got to see that
firsthand: the [initial performance run] with the manual alignment change showed
nontrivial improvements to compiler performance (which relies heavily on 128-bit
integers to work with integer literals). The downside of increasing alignment is that
composite types do not always fit together as nicely in memory, leading to an increase
in usage. Unfortunately this meant some of the performance wins needed to be sacrificed
to avoid an increased memory footprint.

[a proposal]: https://github.com/rust-lang/compiler-team/issues/683
[#116672]: https://github.com/rust-lang/rust/pull/116672/
[D158169]: https://reviews.llvm.org/D158169
[D28990]: https://reviews.llvm.org/D28990
[D86310]: https://reviews.llvm.org/D86310

# Compatibility

The most important question is how compatibility changed as a result of these fixes. In
short, `i128` and `u128` with Rust using LLVM 18 (the default version starting with
1.78) will be completely compatible with any version of GCC, as well as Clang 18 and
above (released March 2024). All other combinations have some incompatible cases, which
are summarized in the table below:

| Compiler 1                         | Compiler 2          | status                              |
| ---------------------------------- | ------------------- | ----------------------------------- |
| Rust ≥ 1.78 with bundled LLVM (18) | GCC (any version)   | Fully compatible                    |
| Rust ≥ 1.78 with bundled LLVM (18) | Clang ≥ 18          | Fully compatible                    |
| Rust ≥ 1.77 with LLVM ≥ 18         | GCC (any version)   | Fully compatible                    |
| Rust ≥ 1.77 with LLVM ≥ 18         | Clang ≥ 18          | Fully compatible                    |
| Rust ≥ 1.77 with LLVM ≥ 18         | Clang \< 18         | Storage compatible, has calling bug |
| Rust ≥ 1.77 with LLVM \< 18        | GCC (any version)   | Storage compatible, has calling bug |
| Rust ≥ 1.77 with LLVM \< 18        | Clang (any version) | Storage compatible, has calling bug |
| Rust \< 1.77[^l]                   | GCC (any version)   | Incompatible                        |
| Rust \< 1.77[^l]                   | Clang (any version) | Incompatible                        |
| GCC (any version)                  | Clang ≥ 18          | Fully compatible                    |
| GCC (any version)                  | Clang \< 18         | Storage compatible with calling bug |

[^l]: Rust < 1.77 with LLVM 18 will have some degree of compatibility, this is just
      an uncommon combination.

# Effects & Future Steps

As mentioned in the introduction, most users will notice no effects of this change
unless you are already doing something questionable with these types.

Starting with Rust 1.77, it will be reasonably safe to start experimenting with
128-bit integers in FFI, with some more certainty coming with the LLVM update
in 1.78. There is [ongoing discussion] about lifting the lint in an upcoming
version, but we want to be cautious and avoid introducing silent breakage for users
whose Rust compiler may be built with an older LLVM.

[relevance to Rust was discovered]: https://github.com/rust-lang/rust/issues/54341#issuecomment-1064729606
[initial performance run]: https://github.com/rust-lang/rust/pull/116672/#issuecomment-1858600381
[known issue in llvm]: https://github.com/llvm/llvm-project/issues/41784
[psabi]: https://www.uclibc.org/docs/psABI-x86_64.pdf
[ongoing discussion]: https://github.com/rust-lang/lang-team/issues/255
[align-godbolt]: https://godbolt.org/z/h94Ge1vMW
[composite-playground]: https://play.rust-lang.org/?version=beta&mode=debug&edition=2021&gist=52f349bdea92bf724bc453f37dbd32ea
[^va-segfault]: <https://github.com/llvm/llvm-project/issues/20283>
[^f128-segfault]: <https://bugs.llvm.org/show_bug.cgi?id=50198>
