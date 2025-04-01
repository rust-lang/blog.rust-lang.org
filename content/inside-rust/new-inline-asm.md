+++
path = "inside-rust/2020/06/08/new-inline-asm"
title = "New inline assembly syntax available in nightly"
authors = ["Josh Triplett"]
description = "Rust has a new inline assembly syntax in nightly, please test"
aliases = ["inside-rust/2020/06/08/new-inline-asm.html"]

[extra]
team = "the language team"
team_url = "https://www.rust-lang.org/governance/teams/lang"
+++

In the course of optimization, OS or embedded development, or other kinds of
low-level programming, you may sometimes need to write native assembly code for
the processor you're running on. "Inline assembly" provides a simple way to
integrate some assembly instructions into a Rust program, feeding Rust
expressions in as input registers, and getting output directly into Rust
variables. We've introduced a new syntax for inline assembly in nightly Rust,
and we're seeking feedback on it; we believe this new syntax has a path to
stabilization in the future.

Nightly Rust has had a syntax for "inline assembly" (`asm!`) for a long time;
however, this syntax just exposed a very raw version of LLVM's assembly
construct, with no safeguards to help developers use it. Getting any detail of
this syntax even slightly wrong tended to produce an Internal Compiler Error
(ICE) rather than the kind of friendly error message you've come to expect from
rustc. This syntax was also error-prone for another reason: it looks similar to
GCC's inline assembly syntax, but has subtle differences (such as the names in
register constraints). This syntax also had little to no hope of being
supported on any non-LLVM backend. As a result of all these limitations, the
`asm!` syntax was highly unlikely to ever graduate from nightly to stable Rust,
despite being one of the most requested features.

In an effort to improve `asm!` and bring it to more users, [Amanieu
d'Antras](https://github.com/Amanieu) designed and implemented a new,
friendlier syntax for `asm!`. This syntax has had a long road from concept to
compiler implementation:
- The proposal first started as a [pre-RFC on
  internals](https://internals.rust-lang.org/t/pre-rfc-2-inline-assembly/11310).
- Inline assembly became one of the language team's first [project
  groups](https://github.com/rust-lang/rfcs/blob/master/text/2836-project-asm.md),
  and iteratively designed RFCs in [the project group
  repository](https://github.com/rust-lang/project-inline-asm/).
- [RFC 2873](https://github.com/rust-lang/rfcs/pull/2873) (still under
  discussion) provides a specification for the syntax and its interaction with
  the Rust language.
- We [renamed the existing `asm!` to
  `llvm_asm!`](https://github.com/rust-lang/rust/pull/68404), so that people
  currently using inline assembly on nightly can continue to use the existing
  syntax for now. (We plan to remove this syntax eventually, given its fragile
  ICE-happy nature, but while evaluating the new syntax we want the old syntax
  available for comparison and alternatives.)
- [PR 69171](https://github.com/rust-lang/rust/pull/69171) (also by Amanieu)
  implemented the new `asm!` syntax in nightly.

Here's an example of using the new inline assembly syntax, to print a message
to standard output using a direct [`write`
syscall](https://man7.org/linux/man-pages/man2/write.2.html) on x86-64 Linux:

```rust
#![feature(asm)]

fn main() {
    let buf = "Hello from asm!\n";
    let ret: i32;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1, // syscall number
            in("rdi") 1, // fd (stdout)
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            out("rcx") _, // clobbered by syscalls
            out("r11") _, // clobbered by syscalls
            lateout("rax") ret,
        );
    }
    println!("write returned: {}", ret);
}
```

(You can [try this example on the
playground](https://play.rust-lang.org/?version=nightly&mode=release&edition=2018&gist=e983a5f5cffa51f4320f1176465d3a56).)

The example above specifies the exact inputs, outputs, and clobbers required by
the Linux syscall calling convention. You can also provide inputs and outputs
via arbitrary registers, and the compiler will select appropriate registers for
you. The following example uses [bit manipulation
instructions](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets)
to compute the bit numbers of all set bits in a value, and stores them in a
slice of memory:

```rust
#![feature(asm)]

fn main() {
    let mut bits = [0u8; 64];
    for value in 0..=1024u64 {
        let popcnt;
        unsafe {
            asm!(
                "popcnt {popcnt}, {v}",
                "2:",
                "blsi rax, {v}",
                "jz 1f",
                "xor {v}, rax",
                "tzcnt rax, rax",
                "stosb",
                "jmp 2b",
                "1:",
                v = inout(reg) value => _,
                popcnt = out(reg) popcnt,
                out("rax") _, // scratch
                inout("rdi") bits.as_mut_ptr() => _,
            );
        }
        println!("bits of {}: {:?}", value, &bits[0..popcnt]);
    }
}
```

(You can [try this example on the
playground](https://play.rust-lang.org/?version=nightly&mode=release&edition=2018&gist=894a407f0fe858559aa378edf6ec4801).
Note that this code serves to demonstrate inline assembly, not to demonstrate
an efficient implementation of any particular algorithm.)

Notice that `value` and `popcnt` have registers selected for them, while
`bits.as_mut_ptr()` must go in the `rdi` register for use with the `stosb`
instruction.

Also, note that on x86 platforms, `asm!` uses Intel syntax by default; however,
you can use AT&T syntax with `option(att_syntax)`. You may find this useful
when translating existing inline assembly code to the new `asm!` syntax.

For full details on the new `asm!` syntax, see [RFC
2873](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md).
Please try it out (including translating existing inline assembly to the new
syntax), and [report any bugs via the rust issue
tracker](https://github.com/rust-lang/rust/issues/) with the tag `F-asm`. You
can also discuss inline assembly by creating a topic on [the project-inline-asm
stream in
Zulip](https://rust-lang.zulipchat.com/#narrow/stream/216763-project-inline-asm).
