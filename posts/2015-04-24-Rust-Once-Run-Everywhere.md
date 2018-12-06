---
layout: post
title: "Rust Once, Run Everywhere"
author: Alex Crichton
description: "Zero-cost and safe FFI in Rust"
---

Rust's quest for world domination was never destined to happen overnight, so
Rust needs to be able to interoperate with the existing world just as easily as
it talks to itself. For this reason, **Rust makes it easy to communicate with C
APIs without overhead, and to leverage its ownership system to provide much
stronger safety guarantees for those APIs at the same time**.

To communicate with other languages, Rust provides a *foreign function
interface* (FFI). Following Rust's design principles, the FFI provides a
**zero-cost abstraction** where function calls between Rust and C have identical
performance to C function calls. FFI bindings can also leverage language
features such as ownership and borrowing to provide a **safe interface** that
enforces protocols around pointers and other resources. These protocols usually
appear only in the documentation for C APIs -- at best -- but Rust makes them
explicit.

In this post we'll explore how to encapsulate unsafe FFI calls to C in safe,
zero-cost abstractions. Working with C is, however, just an example; we'll also
see how Rust can easily talk to languages like Python and Ruby just as
seamlessly as with C.

### Rust talking to C

Let's start with a simple example of calling C code from Rust and then
demonstrate that Rust imposes no additional overhead. Here's a C program which
will simply double all the input it's given:

```c
int double_input(int input) {
    return input * 2;
}
```

To call this from Rust, you might write a program like this:

```rust
extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
}
```

And that's it! You can try this out for yourself by
[checking out the code on GitHub][rust2c] and running `cargo run` from that
directory. **At the source level we can see that there's no burden in calling an
external function beyond stating its signature, and we'll see soon that the
generated code indeed has no overhead, either.** There are, however, a few
subtle aspects of this Rust program, so let's cover each piece in detail.

[rust2c]: https://github.com/alexcrichton/rust-ffi-examples/tree/master/rust-to-c

First up we see `extern crate libc`. [The libc crate][libc] provides many useful
type definitions for FFI bindings when talking with C, and it makes it easy to
ensure that both C and Rust agree on the types crossing the language boundary.

[libc]: https://crates.io/crates/libc

This leads us nicely into the next part of the program:

```rust
extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}
```

In Rust this is a **declaration** of an externally available function. You can
think of this along the lines of a C header file. Here's where the compiler
learns about the inputs and outputs of the function, and you can see above that
this matches our definition in C. Next up we have the main body of the program:

```rust
fn main() {
    let input = 4;
    let output = unsafe { double_input(input) };
    println!("{} * 2 = {}", input, output);
}
```

We see one of the crucial aspects of FFI in Rust here, the `unsafe` block. The
compiler knows nothing about the implementation of `double_input`, so it must
assume that memory unsafety *could* happen whenever you call a foreign function.
The `unsafe` block is how the programmer takes responsibility for ensuring
safety -- you are promising that the actual call you make will not, in fact,
violate memory safety, and thus that Rust's basic guarantees are upheld.  This
may seem limiting, but Rust has just the right set of tools to allow consumers
to not worry about `unsafe` (more on this in a moment).

Now that we've seen how to call a C function from Rust, let's see if we can
verify this claim of zero overhead. Almost all programming languages can call
into C one way or another, but it often comes at a cost with runtime type
conversions or perhaps some language-runtime juggling. To get a handle on what
Rust is doing, let's go straight to the assembly code of the above `main`
function's call to `double_input`:

```
mov    $0x4,%edi
callq  3bc30 <double_input>
```

And as before, that's it! Here we can see that calling a C function from Rust
involves precisely one call instruction after moving the arguments into place,
exactly the same cost as it would be in C.

### Safe Abstractions

Most features in Rust tie into its core concept of ownership, and the FFI is no
exception. When binding a C library in Rust you not only have the benefit of zero
overhead, but you are also able to make it *safer* than C can! **Bindings can
leverage the ownership and borrowing principles in Rust to codify comments
typically found in a C header about how its API should be used.**

For example, consider a C library for parsing a tarball. This library will
expose functions to read the contents of each file in the tarball, probably
something along the lines of:

```c
// Gets the data for a file in the tarball at the given index, returning NULL if
// it does not exist. The `size` pointer is filled in with the size of the file
// if successful.
const char *tarball_file_data(tarball_t *tarball, unsigned index, size_t *size);
```

This function is implicitly making assumptions about how it can be used,
however, by assuming that the `char*` pointer returned cannot outlive the input
tarball. When bound in Rust, this API might look like this instead:

```rust
pub struct Tarball { raw: *mut tarball_t }

impl Tarball {
    pub fn file(&self, index: u32) -> Option<&[u8]> {
        unsafe {
            let mut size = 0;
            let data = tarball_file_data(self.raw, index as libc::c_uint,
                                         &mut size);
            if data.is_null() {
                None
            } else {
                Some(slice::from_raw_parts(data as *const u8, size as usize))
            }
        }
    }
}
```

Here the `*mut tarball_t` pointer is *owned by* a `Tarball`, which is
responsible for any destruction and cleanup, so we already have rich knowledge
about the lifetime of the tarball's memory. Additionally, the `file` method
returns a **borrowed slice** whose lifetime is implicitly connected to the
lifetime of the source tarball itself (the `&self` argument). This is Rust's way
of indicating that the returned slice can only be used within the lifetime of
the tarball, statically preventing dangling pointer bugs that are easy to
make when working directly with C. (If you're not familiar with this kind of
borrowing in Rust, have a look at Yehuda Katz's [blog post on ownership].)

[blog post]: http://blog.skylight.io/rust-means-never-having-to-close-a-socket/

A key aspect of the Rust binding here is that it is a safe function, meaning
that callers do not have to use `unsafe` blocks to invoke it! Although it has an
`unsafe` *implementation* (due to calling an FFI function), the *interface* uses
borrowing to guarantee that no memory unsafety can occur in any Rust code that
uses it. That is, due to Rust's static checking, it's simply not possible to
cause a segfault using the API on the Rust side. And don't forget, all of this
is coming at zero cost: the raw types in C are representable in Rust with no
extra allocations or overhead.

Rust's amazing community has already built some substantial safe bindings around
existing C libraries, including [OpenSSL][rust-openssl], [libgit2][git2-rs],
[libdispatch][dispatch], [libcurl][curl-rust], [sdl2][sdl2], [Unix APIs][nix],
and [libsodium][sodiumoxide]. This list is also growing quite rapidly on
[crates.io][crates-io], so your favorite C library may already be bound or will
be bound soon!

[blog post on ownership]: http://blog.skylight.io/rust-means-never-having-to-close-a-socket/
[rust-openssl]: https://crates.io/crates/openssl
[git2-rs]: https://crates.io/crates/git2
[curl-rust]: https://crates.io/crates/curl
[dispatch]: https://crates.io/crates/dispatch
[sdl2]: https://crates.io/crates/sdl2
[nix]: https://crates.io/crates/nix
[sodiumoxide]: https://crates.io/crates/sodiumoxide
[crates-io]: https://crates.io

### C talking to Rust

**Despite guaranteeing memory safety, Rust does not have a garbage collector or
runtime, and one of the benefits of this is that Rust code can be called from C
with no setup at all.** This means that the zero overhead FFI not only applies
when Rust calls into C, but also when C calls into Rust!

Let's take the example above, but reverse the roles of each language. As before,
all the code below is [available on GitHub][c2rust]. First we'll start off with
our Rust code:

[c2rust]: https://github.com/alexcrichton/rust-ffi-examples/tree/master/c-to-rust

```rust
#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}
```

As with the Rust code before, there's not a whole lot here but there are some
subtle aspects in play. First off, we've labeled our function definition with a
`#[no_mangle]` attribute. This instructs the compiler to not mangle the symbol
name for the function `double_input`. Rust employs name mangling similar to C++
to ensure that libraries do not clash with one another, and this attribute
means that you don't have to guess a symbol name like
`double_input::h485dee7f568bebafeaa` from C.

Next we've got our function definition, and the most interesting part about
this is the keyword `extern`. This is a specialized form of specifying the [ABI
for a function][abi-fn] which enables the function to be compatible with a C
function call.

[abi-fn]: http://doc.rust-lang.org/reference.html#extern-functions

Finally, if you [take a look at the `Cargo.toml`][cargo-toml] you'll see that
this library is not compiled as a normal Rust library (rlib) but instead as a
static archive which Rust calls a 'staticlib'. This enables all the relevant
Rust code to be linked statically into the C program we're about to produce.

[cargo-toml]: https://github.com/alexcrichton/rust-ffi-examples/blob/master/c-to-rust/Cargo.toml#L8

Now that we've got our Rust library squared away, let's write our C program
which will call Rust.

```c
#include <stdint.h>
#include <stdio.h>

extern int32_t double_input(int32_t input);

int main() {
    int input = 4;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    return 0;
}
```

Here we can see that C, like Rust, needs to declare the `double_input` function
that Rust defined. Other than that though everything is ready to go! If you run
`make` from the [directory on GitHub][c2rust] you'll see these examples getting
compiled and linked together and the final executable should run and print
`4 * 2 = 8`.

Rust's lack of a garbage collector and runtime enables this seamless transition
from C to Rust. The external C code does not need to perform any setup on Rust's
behalf, making the transition that much cheaper.

### Beyond C

Up to now we've seen how FFI in Rust has zero overhead and how we can use Rust's
concept of ownership to write safe bindings to C libraries. If you're not using
C, however, you're still in luck! These features of Rust enable it to also be
called from [Python][py2rust], [Ruby][rb2rust], [JavaScript][js2rust], and many
more languages.

[py2rust]: https://github.com/alexcrichton/rust-ffi-examples/tree/master/python-to-rust
[rb2rust]: https://github.com/alexcrichton/rust-ffi-examples/tree/master/ruby-to-rust
[js2rust]: https://github.com/alexcrichton/rust-ffi-examples/tree/master/node-to-rust

When writing code in these languages, you sometimes want to speed up some
component that's performance critical, but in the past this often required
dropping all the way to C, and thereby giving up the memory safety, high-level
abstractions, and ergonomics of these languages.

The fact that Rust can talk to easily with C, however, means that it is also
viable for this sort of usage. One of Rust's first production users,
[Skylight](https://www.skylight.io), was able to improve the performance and
memory usage of their data collection agent almost instantly by just using Rust,
and the Rust code is all published as a Ruby gem.

Moving from a language like Python and Ruby down to C to optimize performance is
often quite difficult as it's tough to ensure that the program won't crash in a
difficult-to-debug way. Rust, however, not only brings zero cost FFI, but *also*
makes it possible to retain the same safety guarantees as the original source
language. In the long run, this should make it much easier for programmers in
these languages to drop down and do some systems programming to squeeze out
critical performance when they need it.

FFI is just one of many tools in the toolbox of Rust, but it's a key component
to Rust's adoption as it allows Rust to seamlessly integrate with existing code
bases today. I'm personally quite excited to see the benefits of Rust reach as
many projects as possible!
