+++
path = "2023/06/01/Rust-1.70.0"
title = "Announcing Rust 1.70.0"
authors = ["The Rust Release Team"]
aliases = [
    "2023/06/01/Rust-1.70.0.html",
    "releases/1.70.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.70.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via rustup, you can get 1.70.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.70.0](https://github.com/rust-lang/rust/releases/tag/1.70.0) on GitHub.

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.70.0 stable

### Sparse by default for crates.io

Cargo's "sparse" protocol is now enabled by default for reading the index from crates.io. This feature was previously stabilized with [Rust 1.68.0](https://blog.rust-lang.org/2023/03/09/Rust-1.68.0.html#cargos-sparse-protocol), but still required configuration to use that with crates.io. The announced plan was to make that the default in 1.70.0, and here it is!

You should see substantially improved performance when fetching information from the crates.io index. Users behind a restrictive firewall will need to ensure that access to `https://index.crates.io` is available. If for some reason you need to stay with the previous default of using the git index hosted by GitHub, the [`registries.crates-io.protocol`](https://doc.rust-lang.org/cargo/reference/config.html#registriescrates-ioprotocol) config setting can be used to change the default.

One side-effect to note about changing the access method is that this also changes the path to the crate cache, so dependencies will be downloaded anew. Once you have fully committed to using the sparse protocol, you may want to clear out the old `$CARGO_HOME/registry/*/github.com-*` paths.

### `OnceCell` and `OnceLock`

Two new types have been stabilized for one-time initialization of shared data, `OnceCell` and its thread-safe counterpart `OnceLock`. These can be used anywhere that immediate construction is not wanted, and perhaps not even possible like non-`const` data in global variables.

```rust
use std::sync::OnceLock;

static WINNER: OnceLock<&str> = OnceLock::new();

fn main() {
    let winner = std::thread::scope(|s| {
        s.spawn(|| WINNER.set("thread"));

        std::thread::yield_now(); // give them a chance...

        WINNER.get_or_init(|| "main")
    });

    println!("{winner} wins!");
}
```

Crates such as `lazy_static` and `once_cell` have filled this need in the past, but now these building blocks are part of the standard library, ported from `once_cell`'s `unsync` and `sync` modules. There are still more methods that may be stabilized in the future, as well as companion `LazyCell` and `LazyLock` types that store their initializing function, but this first step in stabilization should already cover many use cases.

### `IsTerminal`

This newly-stabilized trait has a single method, `is_terminal`, to determine if a given file descriptor or handle represents a terminal or TTY. This is another case of standardizing functionality that existed in external crates, like `atty` and `is-terminal`, using the C library `isatty` function on Unix targets and similar functionality elsewhere. A common use case is for programs to distinguish between running in scripts or interactive modes, like presenting colors or even a full TUI when interactive.

```rust
use std::io::{stdout, IsTerminal};

fn main() {
    let use_color = stdout().is_terminal();
    // if so, add color codes to program output...
}
```

### Named levels of debug information

The `-Cdebuginfo` compiler option has previously only supported numbers 0..=2 for increasing amounts of debugging information, where Cargo defaults to 2 in dev and test profiles and 0 in release and bench profiles. These debug levels can now be set by name: "none" (0), "limited" (1), and "full" (2), as well as two new levels, "line-directives-only" and "line-tables-only".

The Cargo and rustc documentation both called level 1 "line tables only" before, but it was more than that with information about all functions, just not types and variables. That level is now called "limited", and the new "line-tables-only" level is further reduced to the minimum needed for backtraces with filenames and line numbers. This may eventually become the level used for `-Cdebuginfo=1`. The other `line-directives-only` level is intended for NVPTX profiling, and is otherwise not recommended.

Note that these named options are not yet available to be used via `Cargo.toml`. Support for that will be available in the next release 1.71.

### Enforced stability in the `test` CLI

When `#[test]` functions are compiled, the executable gets a command-line interface from the `test` crate. This CLI has a number of options, including some that are not yet stabilized and require specifying `-Zunstable-options` as well, like many other commands in the Rust toolchain. However, while that's only intended to be allowed in nightly builds, that restriction wasn't active in `test` -- until now. Starting with 1.70.0, stable and beta builds of Rust will no longer allow unstable `test` options, making them truly nightly-only as documented.

There are known cases where unstable options may have been used without direct user knowledge, especially `--format json` used in IntelliJ Rust and other IDE plugins. Those projects are already adjusting to this change, and the status of JSON output can be followed in its [tracking issue](https://github.com/rust-lang/rust/issues/49359).

### Stabilized APIs

- [`NonZero*::MIN/MAX`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroI8.html#associatedconstant.MIN)
- [`BinaryHeap::retain`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.retain)
- [`Default for std::collections::binary_heap::IntoIter`](https://doc.rust-lang.org/stable/std/collections/binary_heap/struct.IntoIter.html)
- [`Default for std::collections::btree_map::{IntoIter, Iter, IterMut}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoIter.html)
- [`Default for std::collections::btree_map::{IntoKeys, Keys}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoKeys.html)
- [`Default for std::collections::btree_map::{IntoValues, Values}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoKeys.html)
- [`Default for std::collections::btree_map::Range`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.Range.html)
- [`Default for std::collections::btree_set::{IntoIter, Iter}`](https://doc.rust-lang.org/stable/std/collections/btree_set/struct.IntoIter.html)
- [`Default for std::collections::btree_set::Range`](https://doc.rust-lang.org/stable/std/collections/btree_set/struct.Range.html)
- [`Default for std::collections::linked_list::{IntoIter, Iter, IterMut}`](https://doc.rust-lang.org/stable/alloc/collections/linked_list/struct.IntoIter.html)
- [`Default for std::vec::IntoIter`](https://doc.rust-lang.org/stable/alloc/vec/struct.IntoIter.html#impl-Default-for-IntoIter%3CT,+A%3E)
- [`Default for std::iter::Chain`](https://doc.rust-lang.org/stable/std/iter/struct.Chain.html)
- [`Default for std::iter::Cloned`](https://doc.rust-lang.org/stable/std/iter/struct.Cloned.html)
- [`Default for std::iter::Copied`](https://doc.rust-lang.org/stable/std/iter/struct.Copied.html)
- [`Default for std::iter::Enumerate`](https://doc.rust-lang.org/stable/std/iter/struct.Enumerate.html)
- [`Default for std::iter::Flatten`](https://doc.rust-lang.org/stable/std/iter/struct.Flatten.html)
- [`Default for std::iter::Fuse`](https://doc.rust-lang.org/stable/std/iter/struct.Fuse.html)
- [`Default for std::iter::Rev`](https://doc.rust-lang.org/stable/std/iter/struct.Rev.html)
- [`Default for std::slice::Iter`](https://doc.rust-lang.org/stable/std/slice/struct.Iter.html)
- [`Default for std::slice::IterMut`](https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html)
- [`Rc::into_inner`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html#method.into_inner)
- [`Arc::into_inner`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#method.into_inner)
- [`std::cell::OnceCell`](https://doc.rust-lang.org/stable/std/cell/struct.OnceCell.html)
- [`Option::is_some_and`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.is_some_and)
- [`NonNull::slice_from_raw_parts`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.slice_from_raw_parts)
- [`Result::is_ok_and`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_ok_and)
- [`Result::is_err_and`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_err_and)
- [`std::sync::atomic::Atomic*::as_ptr`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicU8.html#method.as_ptr)
- [`std::io::IsTerminal`](https://doc.rust-lang.org/stable/std/io/trait.IsTerminal.html)
- [`std::os::linux::net::SocketAddrExt`](https://doc.rust-lang.org/stable/std/os/linux/net/trait.SocketAddrExt.html)
- [`std::os::unix::net::UnixDatagram::bind_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.bind_addr)
- [`std::os::unix::net::UnixDatagram::connect_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.connect_addr)
- [`std::os::unix::net::UnixDatagram::send_to_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.send_to_addr)
- [`std::os::unix::net::UnixListener::bind_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixListener.html#method.bind_addr)
- [`std::path::Path::as_mut_os_str`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.as_mut_os_str)
- [`std::sync::OnceLock`](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.70.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-170-2023-06-01), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-170).

## Contributors to 1.70.0

Many people came together to create Rust 1.70.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.70.0/)
