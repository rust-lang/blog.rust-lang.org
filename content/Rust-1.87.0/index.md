+++
path = "2025/05/15/Rust-1.87.0"
title = "Announcing Rust 1.87.0 and ten years of Rust!"
authors = ["The Rust Release Team"]
aliases = [
    "2025/05/15/Rust-1.87.0.html",
    "releases/1.87.0",
]

[extra]
release = true
+++

Live from the [10 Years of Rust celebration](https://2025.rustweek.org/celebration/) in Utrecht, Netherlands,
the Rust team is happy to announce a new version of Rust, 1.87.0!

![picture of Rustaceans at the release party](party.jpg)

Today's release day happens to fall exactly on the 10 year anniversary of
[Rust 1.0](https://blog.rust-lang.org/2015/05/15/Rust-1.0/)!

Thank you to the myriad contributors who have worked on Rust, past and present.
Here's to many more decades of Rust! 🎉

---

As usual, the new version includes all the changes that have been part of the beta version in the
past six weeks, following the consistent regular release cycle that we have followed since Rust 1.0.

If you have a previous version of Rust installed via `rustup`, you can get 1.87.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.87.0](https://doc.rust-lang.org/stable/releases.html#version-1870-2025-04-03).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.87.0 stable

### Anonymous pipes

1.87 adds access to anonymous pipes to the standard library. This includes
integration with `std::process::Command`'s input/output methods. For example,
joining the stdout and stderr streams into one is now relatively
straightforward, as shown below, while it used to require either extra threads
or platform-specific functions.

```rust
use std::process::Command;
use std::io::Read;

let (mut recv, send) = std::io::pipe()?;

let mut command = Command::new("path/to/bin")
    // Both stdout and stderr will write to the same pipe, combining the two.
    .stdout(send.try_clone()?)
    .stderr(send)
    .spawn()?;

let mut output = Vec::new();
recv.read_to_end(&mut output)?;

// It's important that we read from the pipe before the process exits, to avoid
// filling the OS buffers if the program emits too much output.
assert!(command.wait()?.success());
```

### Safe architecture intrinsics

Most `std::arch` intrinsics that are unsafe only due to requiring target
features to be enabled are now callable in safe code that has those features
enabled. For example, the following toy program which implements summing an array using
manual intrinsics can now use safe code for the core loop.

```rust
#![forbid(unsafe_op_in_unsafe_fn)]

use std::arch::x86_64::*;

fn sum(slice: &[u32]) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // SAFETY: We have detected the feature is enabled at runtime,
            // so it's safe to call this function.
            return unsafe { sum_avx2(slice) };
        }
    }

    slice.iter().sum()
}

#[target_feature(enable = "avx2")]
#[cfg(target_arch = "x86_64")]
fn sum_avx2(slice: &[u32]) -> u32 {
    // SAFETY: __m256i and u32 have the same validity.
    let (prefix, middle, tail) = unsafe { slice.align_to::<__m256i>() };
    
    let mut sum = prefix.iter().sum::<u32>();
    sum += tail.iter().sum::<u32>();
    
    // Core loop is now fully safe code in 1.87, because the intrinsics require
    // matching target features (avx2) to the function definition.
    let mut base = _mm256_setzero_si256();
    for e in middle.iter() {
        base = _mm256_add_epi32(base, *e);
    }
    
    // SAFETY: __m256i and u32 have the same validity.
    let base: [u32; 8] = unsafe { std::mem::transmute(base) };
    sum += base.iter().sum::<u32>();
    
    sum
}
```

### `asm!` jumps to Rust code  

Inline assembly (`asm!`) can now jump to labeled blocks within Rust code. This
enables more flexible low-level programming, such as implementing optimized
control flow in OS kernels or interacting with hardware more efficiently. 

- The `asm!` macro now supports a label operand, which acts as a jump target.  
- The label must be a block expression with a return type of `()` or `!`.  
- The block executes when jumped to, and execution continues after the `asm!` block.  
- Using output and label operands in the same `asm!` invocation remains [unstable](https://github.com/rust-lang/rust/issues/119364).  

```rust
unsafe {
    asm!(
        "jmp {}",
        label {
            println!("Jumped from asm!");
        }
    );
}
```

For more details, please consult the [reference](https://doc.rust-lang.org/nightly/reference/inline-assembly.html#r-asm.operand-type.supported-operands.label).

### Precise capturing (`+ use<...>`) in `impl Trait` in trait definitions

This release stabilizes specifying the specific captured generic types and
lifetimes in trait definitions using `impl Trait` return types. This allows
using this feature in trait definitions, expanding on the stabilization for
non-trait functions in
[1.82](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0/#precise-capturing-use-syntax).

Some example desugarings:

```rust
trait Foo {
    fn method<'a>(&'a self) -> impl Sized;
    
    // ... desugars to something like:
    type Implicit1<'a>: Sized;
    fn method_desugared<'a>(&'a self) -> Self::Implicit1<'a>;
    
    // ... whereas with precise capturing ...
    fn precise<'a>(&'a self) -> impl Sized + use<Self>;
    
    // ... desugars to something like:
    type Implicit2: Sized;
    fn precise_desugared<'a>(&'a self) -> Self::Implicit2;
}
```

### Stabilized APIs


- [`Vec::extract_if`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.extract_if)
- [`vec::ExtractIf`](https://doc.rust-lang.org/stable/std/vec/struct.ExtractIf.html)
- [`LinkedList::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html#method.extract_if)
- [`linked_list::ExtractIf`](https://doc.rust-lang.org/stable/std/collections/linked_list/struct.ExtractIf.html)
- [`<[T]>::split_off`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off)
- [`<[T]>::split_off_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off_mut)
- [`<[T]>::split_off_first`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off_first)
- [`<[T]>::split_off_first_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off_first_mut)
- [`<[T]>::split_off_last`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off_last)
- [`<[T]>::split_off_last_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_off_last_mut)
- [`String::extend_from_within`](https://doc.rust-lang.org/stable/alloc/string/struct.String.html#method.extend_from_within)
- [`os_str::Display`](https://doc.rust-lang.org/stable/std/ffi/os_str/struct.Display.html)
- [`OsString::display`](https://doc.rust-lang.org/stable/std/ffi/struct.OsString.html#method.display)
- [`OsStr::display`](https://doc.rust-lang.org/stable/std/ffi/struct.OsStr.html#method.display)
- [`io::pipe`](https://doc.rust-lang.org/stable/std/io/fn.pipe.html)
- [`io::PipeReader`](https://doc.rust-lang.org/stable/std/io/struct.PipeReader.html)
- [`io::PipeWriter`](https://doc.rust-lang.org/stable/std/io/struct.PipeWriter.html)
- [`impl From<PipeReader> for OwnedHandle`](https://doc.rust-lang.org/stable/std/os/windows/io/struct.OwnedHandle.html#impl-From%3CPipeReader%3E-for-OwnedHandle)
- [`impl From<PipeWriter> for OwnedHandle`](https://doc.rust-lang.org/stable/std/os/windows/io/struct.OwnedHandle.html#impl-From%3CPipeWriter%3E-for-OwnedHandle)
- [`impl From<PipeReader> for Stdio`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html)
- [`impl From<PipeWriter> for Stdio`](https://doc.rust-lang.org/stable/std/process/struct.Stdio.html#impl-From%3CPipeWriter%3E-for-Stdio)
- [`impl From<PipeReader> for OwnedFd`](https://doc.rust-lang.org/stable/std/os/fd/struct.OwnedFd.html#impl-From%3CPipeReader%3E-for-OwnedFd)
- [`impl From<PipeWriter> for OwnedFd`](https://doc.rust-lang.org/stable/std/os/fd/struct.OwnedFd.html#impl-From%3CPipeWriter%3E-for-OwnedFd)
- [`Box<MaybeUninit<T>>::write`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.write)
- [`impl TryFrom<Vec<u8>> for String`](https://doc.rust-lang.org/stable/std/string/struct.String.html#impl-TryFrom%3CVec%3Cu8%3E%3E-for-String)
- [`<*const T>::offset_from_unsigned`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from_unsigned)
- [`<*const T>::byte_offset_from_unsigned`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.byte_offset_from_unsigned)
- [`<*mut T>::offset_from_unsigned`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from_unsigned-1)
- [`<*mut T>::byte_offset_from_unsigned`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.byte_offset_from_unsigned-1)
- [`NonNull::offset_from_unsigned`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.offset_from_unsigned)
- [`NonNull::byte_offset_from_unsigned`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.byte_offset_from_unsigned)
- [`<uN>::cast_signed`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.cast_signed)
- [`NonZero::<uN>::cast_signed`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.cast_signed-5).
- [`<iN>::cast_unsigned`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.cast_unsigned).
- [`NonZero::<iN>::cast_unsigned`](https://doc.rust-lang.org/stable/std/num/struct.NonZero.html#method.cast_unsigned-5).
- [`<uN>::is_multiple_of`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.is_multiple_of)
- [`<uN>::unbounded_shl`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.unbounded_shl)
- [`<uN>::unbounded_shr`](https://doc.rust-lang.org/stable/std/primitive.usize.html#method.unbounded_shr)
- [`<iN>::unbounded_shl`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.unbounded_shl)
- [`<iN>::unbounded_shr`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.unbounded_shr)
- [`<iN>::midpoint`](https://doc.rust-lang.org/stable/std/primitive.isize.html#method.midpoint)
- [`<str>::from_utf8`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.from_utf8)
- [`<str>::from_utf8_mut`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.from_utf8_mut)
- [`<str>::from_utf8_unchecked`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.from_utf8_unchecked)
- [`<str>::from_utf8_unchecked_mut`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.from_utf8_unchecked_mut)

These previously stable APIs are now stable in const contexts:

- [`core::str::from_utf8_mut`](https://doc.rust-lang.org/stable/std/str/fn.from_utf8_mut.html)
- [`<[T]>::copy_from_slice`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.copy_from_slice)
- [`SocketAddr::set_ip`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.set_ip)
- [`SocketAddr::set_port`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.set_port),
- [`SocketAddrV4::set_ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.set_ip)
- [`SocketAddrV4::set_port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.set_port),
- [`SocketAddrV6::set_ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.set_ip)
- [`SocketAddrV6::set_port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.set_port)
- [`SocketAddrV6::set_flowinfo`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.set_flowinfo)
- [`SocketAddrV6::set_scope_id`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.set_scope_id)
- [`char::is_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_digit)
- [`char::is_whitespace`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_whitespace)
- [`<[[T; N]]>::as_flattened`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_flattened)
- [`<[[T; N]]>::as_flattened_mut`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.as_flattened_mut) 
- [`String::into_bytes`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.into_bytes)
- [`String::as_str`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_str)
- [`String::capacity`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.capacity)
- [`String::as_bytes`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_bytes)
- [`String::len`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.len)
- [`String::is_empty`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.is_empty)
- [`String::as_mut_str`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_mut_str)
- [`String::as_mut_vec`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_mut_vec)
- [`Vec::as_ptr`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.as_ptr)
- [`Vec::as_slice`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.as_slice)
- [`Vec::capacity`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.capacity)
- [`Vec::len`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.len)
- [`Vec::is_empty`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.is_empty)
- [`Vec::as_mut_slice`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.as_mut_slice)
- [`Vec::as_mut_ptr`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html#method.as_mut_ptr)

### `i586-pc-windows-msvc` target removal

The Tier 2 target `i586-pc-windows-msvc` has been removed. `i586-pc-windows-msvc`'s difference to the much more popular Tier 1 target `i686-pc-windows-msvc` is that `i586-pc-windows-msvc` does not require SSE2 instruction support. But Windows 10, the minimum required OS version of all `windows` targets (except the `win7` targets), requires SSE2 instructions itself.

All users currently targeting `i586-pc-windows-msvc` should migrate to `i686-pc-windows-msvc`.

You can check the [Major Change Proposal](https://github.com/rust-lang/compiler-team/issues/840) for more information.

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.87.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-187-2025-05-15), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-187).

## Contributors to 1.87.0

Many people came together to create Rust 1.87.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.87.0/)
