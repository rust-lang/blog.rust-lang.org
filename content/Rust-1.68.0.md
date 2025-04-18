+++
path = "2023/03/09/Rust-1.68.0"
title = "Announcing Rust 1.68.0"
authors = ["The Rust Release Team"]
aliases = [
    "2023/03/09/Rust-1.68.0.html",
    "releases/1.68.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.68.0. Rust is a
programming language empowering everyone to build reliable and efficient
software.

If you have a previous version of Rust installed via rustup, you can get 1.68.0
with:

```
$ rustup update stable
```

If you don't have it already, you can [get
`rustup`](https://www.rust-lang.org/install.html) from the appropriate page on
our website, and check out the [detailed release notes for
1.68.0](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1680-2023-03-09)
on GitHub.

If you'd like to help us out by testing future releases, you might consider
updating locally to use the beta channel (`rustup default beta`) or the nightly
channel (`rustup default nightly`). Please
[report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you
might come across!

## What's in 1.68.0 stable

### Cargo's sparse protocol

Cargo's "sparse" registry protocol has been stabilized for reading the index of
crates, along with infrastructure at `https://index.crates.io/` for those
published in the primary crates.io registry. The prior git protocol (which is
still the default) clones a repository that indexes _all_ crates available in
the registry, but this has started to hit scaling limitations, with noticeable
delays while updating that repository. The new protocol should provide a
significant performance improvement when accessing crates.io, as it will only
download information about the subset of crates that you actually use.

To use the sparse protocol with crates.io, set the environment variable
`CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse`, or edit your
[`.cargo/config.toml` file](https://doc.rust-lang.org/cargo/reference/config.html)
to add:

```toml
[registries.crates-io]
protocol = "sparse"
```

The sparse protocol is currently planned to become the default for crates.io in
the 1.70.0 release in a few months. For more information, please see the prior
[announcement](https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html)
on the Inside Rust Blog, as well as
[RFC 2789](https://rust-lang.github.io/rfcs/2789-sparse-index.html)
and the current
[documentation](https://doc.rust-lang.org/stable/cargo/reference/registry-index.html#sparse-protocol)
in the Cargo Book.

### Local `Pin` construction

The new [`pin!`](https://doc.rust-lang.org/stable/std/pin/macro.pin.html) macro
constructs a `Pin<&mut T>` from a `T` expression, anonymously captured in local
state. This is often called stack-pinning, but that "stack" could also be the
captured state of an `async fn` or block. This macro is similar to some crates,
like [`tokio::pin!`](https://docs.rs/tokio/1/tokio/macro.pin.html), but the
standard library can take advantage of `Pin` internals and [temporary lifetime
extension](https://doc.rust-lang.org/stable/reference/destructors.html#temporary-lifetime-extension)
for a more expression-like macro.

```rust
/// Runs a future to completion.
fn block_on<F: Future>(future: F) -> F::Output {
    let waker_that_unparks_thread = todo!();
    let mut cx = Context::from_waker(&waker_that_unparks_thread);
    // Pin the future so it can be polled.
    let mut pinned_future = pin!(future);
    loop {
        match pinned_future.as_mut().poll(&mut cx) {
            Poll::Pending => thread::park(),
            Poll::Ready(result) => return result,
        }
    }
}
```

In this example, the original `future` will be moved into a temporary local,
referenced by the new `pinned_future` with type `Pin<&mut F>`, and that pin is
subject to the normal borrow checker to make sure it can't outlive that local.

### Default `alloc` error handler

When allocation fails in Rust, APIs like `Box::new` and `Vec::push` have no way
to indicate that failure, so some divergent execution path needs to be taken.
When using the `std` crate, the program will print to `stderr` and abort.
As of Rust 1.68.0, binaries which include `std` will continue to have
this behavior. Binaries which do not include `std`, only including `alloc`, will now `panic!`
on allocation failure, which may be further adjusted via a `#[panic_handler]` if desired.

In the future, it's likely that the behavior for `std` will also be changed to match that of `alloc`-only binaries.

### Stabilized APIs

- [`{core,std}::pin::pin!`](https://doc.rust-lang.org/stable/std/pin/macro.pin.html)
- [`impl From<bool> for {f32,f64}`](https://doc.rust-lang.org/stable/std/primitive.f32.html#impl-From%3Cbool%3E-for-f32)
- [`std::path::MAIN_SEPARATOR_STR`](https://doc.rust-lang.org/stable/std/path/constant.MAIN_SEPARATOR_STR.html)
- [`impl DerefMut for PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#impl-DerefMut-for-PathBuf)

These APIs are now stable in const contexts:

- [`VecDeque::new`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.new)

### Other changes

* As [previously announced](https://blog.rust-lang.org/2023/01/09/android-ndk-update-r25.html),
  Android platform support in Rust is now targeting NDK r25, which corresponds to
  a minimum supported API level of 19 (KitKat).

Check out everything that changed in
[Rust](https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1680-2023-03-09),
[Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-168-2023-03-09),
and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-168).

### Contributors to 1.68.0

Many people came together to create Rust 1.68.0.
We couldn't have done it without all of you.
[Thanks!](https://thanks.rust-lang.org/rust/1.68.0/)
