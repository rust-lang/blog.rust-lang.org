+++
path = "2018/03/01/Rust-1.24.1"
title = "Announcing Rust 1.24.1"
authors = ["The Rust Core Team"]
aliases = [
    "2018/03/01/Rust-1.24.1.html",
    "releases/1.24.1",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.24.1. Rust is a
systems programming language focused on safety, speed, and concurrency.

If you have a previous version of Rust installed via rustup, getting Rust
1.24.1 is as easy as:

```bash
$ rustup update stable
```

If you don't have it already, you can [get `rustup`][install] from the
appropriate page on our website, and check out the [detailed release notes for
1.24.1][notes] on GitHub.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1241-2018-03-01

## What's in 1.24.1 stable

Several minor regressions were found in 1.24.0 which collectively merited a
release.

A quick summary of the changes:

* Do not abort when unwinding through FFI (this reverts behavior added in 1.24.0)
* Emit UTF-16 files for linker arguments on Windows
* Make the error index generator work again
* Cargo will warn on Windows 7 if an update is needed.

If your code is continuing to build, then the only issue that may affect you is
the unwinding issue. We plan on bringing this behavior back in 1.25 or 1.26,
depending on how smoothly the new strategy goes.

With that, let's dig into the details!

### Do not abort when unwinding through FFI

TL;DR: the new behavior in 1.24.0 broke the `rlua` crate, and is being
reverted. If you have since changed your code to take advantage of the behavior
in 1.24.0, you'll need to revert it for now.  While we still plan to introduce
this behavior eventually, we will be rolling it out more slowly and with a new
implementation strategy.

Quoting [the 1.24 announcement](https://blog.rust-lang.org/2018/02/15/Rust-1.24.html):

> There’s one other change we’d like to talk about here: undefined behavior.
> Rust generally strives to minimize undefined behavior, having none of it in
> safe code, and as little as possible in unsafe code. One area where you could
> invoke UB is when a panic! goes across an FFI boundary. In other words, this:

```rust
extern "C" fn panic_in_ffi() {
    panic!("Test");
}
```

> This cannot work, as the exact mechanism of how panics work would have to
> be reconciled with how the "C" ABI works, in this example, or any other ABI
> in other examples.
>
> In Rust 1.24, this code will now abort instead of producing undefined behavior.

As mentioned above, this caused breakage. It started with [a bug filed against
the `rlua` crate](https://github.com/chucklefish/rlua/issues/71). `rlua` is a
package that provides high level bindings between Rust and the [Lua programming
language](https://www.lua.org/).

> Side note: `rlua` is maintained by [Chucklefish](https://chucklefish.org/),
> a game development studio from London that's using Rust. Lua is a very
> popular language to use for extending and scripting games. We care deeply about
> production Rust users, and so handling this was a very high priority for the
> Rust team.

On Windows, and only on Windows, any attempt to handle errors from Lua would
simply abort. This makes `rlua` unusable, as any error of any kind within Lua
causes your program to die.

After digging in, the culpurit was found: `setjmp`/`longjmp`. These functions
are provided by the C standard library as a means of handling errors. You
first call `setjmp`, and then, at some later point in time, call `longjmp`.
When you do, control flow returns to where you had previously called
`setjmp`. This is often used as a way to implement exceptions, and sometimes,
even coroutines. Lua's implementation uses `setjmp`/`longjmp` [to implement
exceptions](https://www.lua.org/pil/24.3.html):

> Unlike C++ or Java, the C language does not offer an exception handling
> mechanism. To ameliorate this difficulty, Lua uses the setjmp facility from
> C, which results in a mechanism similar to exception handling. (If you
> compile Lua with C++, it is not difficult to change the code so that it uses
> real exceptions instead.)

The issue is this: what happens when some C code `setjmp`/`longjmp`'s through
Rust stack frames? Because drop checking and borrow checking know nothing
about this style of control flow, if you `longjmp` across a Rust stack
frame that has any type that's not `Copy` on its stack, undefined
behavior will result. However, if the jump happens entirely in C, this
should work just fine. This is how `rlua` was managing it: every call
into Lua is [wrapped with `lua_pcall`](https://www.lua.org/pil/24.3.2.html):

> When you write library functions for Lua, however, there is a standard way
> to handle errors. Whenever a C function detects an error, it simply calls
> `lua_error`, (or better yet `luaL_error`, which formats the error message and
> then calls `lua_error`). The `lua_error` function clears whatever needs to be
> cleared in Lua and jumps back to the `lua_pcall` that originated that
> execution, passing along the error message.

So, the question becomes: Why does this break? And why does it break on
Windows?

When we talked about `setjmp`/`longjmp` initially, a key phrase here wasn't
highlighted. Here it is:

> After digging in, the culpurit was found: `setjmp`/`longjmp`. These functions
> are *provided by the C standard library* as a means of handling errors.

These functions aren't part of the C language, but part of the standard
library. That means that platform authors implement these functions, and
their implementations may differ.

Windows has a concept called SEH, short for ["Structured Exception
Handling"](https://msdn.microsoft.com/en-us/library/windows/desktop/ms680657(v=vs.85).aspx).
Windows uses SEH to implement `setjmp`/`longjmp`, as the whole idea of SEH
is to have uniform error handling. For similar reasons, C++ exceptions use
SEH, as do Rust panics.

Before we can sort the exact details of what's happening, let's look at how `rlua`
works. `rlua` has an internal function, `protect_lua_call`, used to call into
Lua. Using it looks like this:

```rust
protect_lua_call(self.state, 0, 1, |state| {
    ffi::lua_newtable(state);
})?;
```

That is, `protect_lua_call` takes some arguments, one of which is a closure. This
closure is passed to `lua_pcall`, which catches any `longjmp`s that may be thrown
by the code passed to it, aka, that closure.

Consider the code above, and imagine that `lua_newtable` here could call
`longjmp`. Here's what should happen:

1. `protect_lua_call` takes our closure, and passes it to `lua_pcall`.
2. `lua_pcall` calls `setjmp` to handle any errors, and invokes our closure.
2. Inside our closure, `lua_newtable` has an error, and calls `longjmp`.
3. The initial `lua_pcall` catches the `longjmp` with the `setjmp` it called earlier.
4. Everyone is happy.

However, the implementation of `protect_lua_call` converts our closure to an
`extern fn`, since that's what Lua needs. So, with the changes in 1.24.0, it
sets up a panic handler that will cause an abort. In other words, the code
sorta looks something like this pseudo code now:

```rust
protect_lua_call(self.state, 0, 1, |state| {
    let result = panic::catch_unwind(|| {
        ffi::lua_newtable(state);
    });

    if result.is_err() {
        process::abort();
    }
})?;
```

Earlier, when discussing `setjmp`/`longjmp`, we said that the issue with it in
Rust code is that it doesn't handle Rust destructors. So, on every platform but
Windows, the above `catch_unwind` shenanigans is effectively ignored, so
everything works. However, on Windows, since both `setjmp`/`longjmp` and Rust
panics use SEH, the `longjmp` gets "caught", and runs the new abort code!

The [solution here](https://github.com/rust-lang/rust/pull/48572) is to
generate the abort handler, but in a way that `longjmp` won't trigger it.  It's
not 100% clear if this will make it into Rust 1.25; if the landing is smooth,
we may backport, otherwise, this functionality will be back in 1.26.

### Emit UTF-16 files for linker arguments on Windows

TL;DR: `rustc` stopped working for some Windows users in edge-case situations.
If it's been working for you, you were not affected by this bug.

In contrast with the previous bug, which is very complex and tough to understand,
this bug's impact is simple: if you have non-ASCII paths in the directory where
you invoke `rustc`, in 1.24, it would incorrectly error with a message like

> fatal error LNK1181: cannot open input file

The PR that caused it, [#47507](https://github.com/rust-lang/rust/pull/47507),
has a good explanation of the behavior that ended up causing the problem:

> When spawning a linker rustc has historically been known to blow OS limits
> for the command line being too large, notably on Windows. This is especially
> true of incremental compilation where there can be dozens of object files per
> compilation. The compiler currently has logic for detecting a failure to
> spawn and instead passing arguments via a file instead, but this failure
> detection only triggers if a process actually fails to spawn.

However, when generating that file, we were doing it incorrectly. As [the
docs state](https://docs.microsoft.com/en-gb/cpp/build/reference/unicode-support-in-the-compiler-and-linker#linker-response-files-and-def-files):

> Response files and DEF files can be either UTF-16 with a BOM, or ANSI.

We were providing a UTF-8 encoded file, with no
[BOM](https://en.wikipedia.org/wiki/Byte_order_mark). The fix is therefore
straightforward: produce a UTF-16 file with a BOM.

### Make the error index generator work again

TL;DR: building Rust 1.24.0 with Rust 1.24.0 broke in some circumstances.
If you weren't building Rust yourself, you were not affected by this bug.

When packaging Rust for various Linux distros, it was found that [building
1.24 with 1.24 fails](https://github.com/rust-lang/rust/issues/48308).
This was caused by an incorrect path, causing certain metadata to not
be generated properly.

As this issue is not particularly interesting, and only affects a small
number of people, all of whom should be aware of it by now, we won't go
into the details further. To learn more, please check out the issue and
the resulting discussion.

### Cargo will warn on Windows 7 if an update is needed.

TL;DR: Cargo couldn't fetch the index from crates.io if you were using an older
Windows without having applied security fixes. If you are using a newer
Windows, or a patched Windows, you are not affected by this bug.

In February of 2017, [GitHub announced that they were dropping support for
weak cryptographic
standards](https://githubengineering.com/crypto-deprecation-notice/). One
year later, in February of 2018, [the deprecation period is over, and support
is
removed](https://blog.github.com/2018-02-23-weak-cryptographic-standards-removed/).
In general, this is a great thing.

Cargo uses GitHub to store the index of Crates.io, our package repository.
It also uses `libgit2` for `git` operations. `libgit2` uses
[WinHTTP](https://msdn.microsoft.com/en-us/library/windows/desktop/aa382925(v=vs.85).aspx)
for making HTTP calls. As part of the OS, its feature set depends on the OS you're using.

> This section uses "Windows 7" to mean "Windows 7, Windows Server 2018, and Windows Server 2012",
> because it's much shorter. The following applies to all three of these editions of Windows,
> however.

Windows 7 [received an update](https://support.microsoft.com/en-us/help/3140245/update-to-enable-tls-1-1-and-tls-1-2-as-a-default-secure-protocols-in)
in June of 2016 regarding TLS. Before the patch, Windows 7 would use TLS 1.0 by default. This update
allows for applications to use TLS 1.1 or 1.2 natively instead.

If your system has not received that update, then you'd still be using TLS 1.0. This means
that accessing GitHub would start to fail.

`libgit2` [created a fix](https://github.com/libgit2/libgit2/pull/4550), using the `WinHTTP` API
to request TLS 1.2. On master, we've [updated to fix this](https://github.com/rust-lang/cargo/pull/5091),
but for 1.24.1 stable, we're [issuing a warning](https://github.com/rust-lang/cargo/pull/5069),
suggesting that they upgrade their Windows version. Although the `libgit2` fix
could have been backported, we felt that the code change footprint was too
large for the point release, especially since the issue does not affect patched
systems.

## Contributors to 1.24.1

[Thanks!](https://thanks.rust-lang.org/rust/1.24.1)
