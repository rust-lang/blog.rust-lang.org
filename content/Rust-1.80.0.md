+++
path = "2024/07/25/Rust-1.80.0"
title = "Announcing Rust 1.80.0"
authors = ["The Rust Release Team"]
aliases = [
    "2024/07/25/Rust-1.80.0.html",
    "releases/1.80.0",
]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.80.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.80.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.80.0](https://doc.rust-lang.org/nightly/releases.html#version-1800-2024-07-25).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.80.0 stable

### `LazyCell` and `LazyLock`

These "lazy" types delay the initialization of their data until first access. They are similar to the `OnceCell` and `OnceLock` types [stabilized in 1.70](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html#oncecell-and-oncelock), but with the initialization function included in the cell. This completes the stabilization of functionality adopted into the standard library from the popular [`lazy_static`](https://crates.io/crates/lazy-static) and [`once_cell`](https://crates.io/crates/once_cell) crates.

`LazyLock` is the thread-safe option, making it suitable for places like `static` values. For example, both the `spawn` thread and the main `scope` will see the exact same duration below, since `LAZY_TIME` will be initialized once, by whichever ends up accessing the static first. Neither use has to know *how* to initialize it, unlike they would with `OnceLock::get_or_init()`.

```rust
use std::sync::LazyLock;
use std::time::Instant;

static LAZY_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

fn main() {
    let start = Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            println!("Thread lazy time is {:?}", LAZY_TIME.duration_since(start));
        });
        println!("Main lazy time is {:?}", LAZY_TIME.duration_since(start));
    });
}
```

`LazyCell` does the same thing without thread synchronization, so it doesn't implement `Sync`, which is needed for `static`, but it can still be used in `thread_local!` statics (with distinct initialization per thread). Either type can also be used in other data structures as well, depending on thread-safety needs, so lazy initialization is available everywhere!

### Checked `cfg` names and values

In 1.79, `rustc` stabilized a [`--check-cfg` flag](https://doc.rust-lang.org/rustc/check-cfg.html), and now Cargo 1.80 is enabling those checks for all `cfg` names and values that it knows (in addition to the [well known names and values](https://doc.rust-lang.org/rustc/check-cfg.html#well-known-names-and-values) from `rustc`). This includes feature names from `Cargo.toml` as well as new `cargo::rustc-check-cfg` output from build scripts.

Unexpected cfgs are reported by the warn-by-default `unexpected_cfgs` lint, which is meant to catch typos or other misconfiguration. For example, in a project with an optional `rayon` dependency, this code is configured for the wrong `feature` value:

```rust
fn main() {
    println!("Hello, world!");

    #[cfg(feature = "crayon")]
    rayon::join(
        || println!("Hello, Thing One!"),
        || println!("Hello, Thing Two!"),
    );
}
```

```
warning: unexpected `cfg` condition value: `crayon`
 --> src/main.rs:4:11
  |
4 |     #[cfg(feature = "crayon")]
  |           ^^^^^^^^^^--------
  |                     |
  |                     help: there is a expected value with a similar name: `"rayon"`
  |
  = note: expected values for `feature` are: `rayon`
  = help: consider adding `crayon` as a feature in `Cargo.toml`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default
```

The same warning is reported regardless of whether the actual `rayon` feature is enabled or not.

The `[lints]` table in the `Cargo.toml` manifest can also be used to extend the list of known names and values for custom `cfg`. `rustc` automatically provides [the syntax](https://doc.rust-lang.org/rustc/check-cfg.html#specifying-expected-names-and-values) to use in the warning.

```toml
[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(foo, values("bar"))'] }
```

You can read more about this feature in a [previous blog post](https://blog.rust-lang.org/2024/05/06/check-cfg.html) announcing the availability of the feature on nightly.

### Exclusive ranges in patterns

Rust ranged patterns can now use exclusive endpoints, written `a..b` or `..b` similar to the `Range` and `RangeTo` expression types. For example, the following patterns can now use the same constants for the end of one pattern and the start of the next:

```rust
pub fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 10u32.pow(3);
    const M: u32 = 10u32.pow(6);
    const G: u32 = 10u32.pow(9);
    match n {
        ..K => "",
        K..M => "k",
        M..G => "M",
        G.. => "G",
    }
}
```

Previously, only inclusive (`a..=b` or `..=b`) or open (`a..`) ranges were allowed in patterns, so code like this would require separate constants for inclusive endpoints like `K - 1`.

Exclusive ranges have been implemented as an unstable feature for a long time, but the blocking concern was that they might add confusion and increase the chance of off-by-one errors in patterns. To that end, exhaustiveness checking has been enhanced to better detect gaps in pattern matching, and new lints `non_contiguous_range_endpoints` and `overlapping_range_endpoints` will help detect cases where you might want to switch exclusive patterns to inclusive, or vice versa.

### Stabilized APIs

- [`impl Default for Rc<CStr>`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3CCStr%3E)
- [`impl Default for Rc<str>`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3Cstr%3E)
- [`impl Default for Rc<[T]>`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html#impl-Default-for-Rc%3C%5BT%5D%3E)
- [`impl Default for Arc<str>`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3Cstr%3E)
- [`impl Default for Arc<CStr>`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3CCStr%3E)
- [`impl Default for Arc<[T]>`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#impl-Default-for-Arc%3C%5BT%5D%3E)
- [`impl IntoIterator for Box<[T]>`](https://doc.rust-lang.org/stable/alloc/boxed/struct.Box.html#impl-IntoIterator-for-Box%3C%5BI%5D,+A%3E)
- [`impl FromIterator<String> for Box<str>`](https://doc.rust-lang.org/stable/alloc/boxed/struct.Box.html#impl-FromIterator%3CString%3E-for-Box%3Cstr%3E)
- [`impl FromIterator<char> for Box<str>`](https://doc.rust-lang.org/stable/alloc/boxed/struct.Box.html#impl-FromIterator%3Cchar%3E-for-Box%3Cstr%3E)
- [`LazyCell`](https://doc.rust-lang.org/stable/core/cell/struct.LazyCell.html)
- [`LazyLock`](https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html)
- [`Duration::div_duration_f32`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.div_duration_f32)
- [`Duration::div_duration_f64`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.div_duration_f64)
- [`Option::take_if`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.take_if)
- [`Seek::seek_relative`](https://doc.rust-lang.org/stable/std/io/trait.Seek.html#method.seek_relative)
- [`BinaryHeap::as_slice`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.as_slice)
- [`NonNull::offset`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.offset)
- [`NonNull::byte_offset`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.byte_offset)
- [`NonNull::add`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.add)
- [`NonNull::byte_add`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.byte_add)
- [`NonNull::sub`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.sub)
- [`NonNull::byte_sub`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.byte_sub)
- [`NonNull::offset_from`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.offset_from)
- [`NonNull::byte_offset_from`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.byte_offset_from)
- [`NonNull::read`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.read)
- [`NonNull::read_volatile`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.read_volatile)
- [`NonNull::read_unaligned`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.read_unaligned)
- [`NonNull::write`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.write)
- [`NonNull::write_volatile`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.write_volatile)
- [`NonNull::write_unaligned`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.write_unaligned)
- [`NonNull::write_bytes`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.write_bytes)
- [`NonNull::copy_to`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.copy_to)
- [`NonNull::copy_to_nonoverlapping`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.copy_to_nonoverlapping)
- [`NonNull::copy_from`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.copy_from)
- [`NonNull::copy_from_nonoverlapping`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.copy_from_nonoverlapping)
- [`NonNull::replace`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.replace)
- [`NonNull::swap`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.swap)
- [`NonNull::drop_in_place`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.drop_in_place)
- [`NonNull::align_offset`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.align_offset)
- [`<[T]>::split_at_checked`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_at_checked)
- [`<[T]>::split_at_mut_checked`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_at_mut_checked)
- [`str::split_at_checked`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at_checked)
- [`str::split_at_mut_checked`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_at_mut_checked)
- [`str::trim_ascii`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.trim_ascii)
- [`str::trim_ascii_start`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.trim_ascii_start)
- [`str::trim_ascii_end`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.trim_ascii_end)
- [`<[u8]>::trim_ascii`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.trim_ascii)
- [`<[u8]>::trim_ascii_start`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.trim_ascii_start)
- [`<[u8]>::trim_ascii_end`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.trim_ascii_end)
- [`Ipv4Addr::BITS`](https://doc.rust-lang.org/stable/core/net/struct.Ipv4Addr.html#associatedconstant.BITS)
- [`Ipv4Addr::to_bits`](https://doc.rust-lang.org/stable/core/net/struct.Ipv4Addr.html#method.to_bits)
- [`Ipv4Addr::from_bits`](https://doc.rust-lang.org/stable/core/net/struct.Ipv4Addr.html#method.from_bits)
- [`Ipv6Addr::BITS`](https://doc.rust-lang.org/stable/core/net/struct.Ipv6Addr.html#associatedconstant.BITS)
- [`Ipv6Addr::to_bits`](https://doc.rust-lang.org/stable/core/net/struct.Ipv6Addr.html#method.to_bits)
- [`Ipv6Addr::from_bits`](https://doc.rust-lang.org/stable/core/net/struct.Ipv6Addr.html#method.from_bits)
- [`Vec::<[T; N]>::into_flattened`](https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html#method.into_flattened)
- [`<[[T; N]]>::as_flattened`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.as_flattened)
- [`<[[T; N]]>::as_flattened_mut`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.as_flattened_mut)

These APIs are now stable in const contexts:

- [`<[T]>::last_chunk`](https://doc.rust-lang.org/stable/core/primitive.slice.html#method.last_chunk)
- [`BinaryHeap::new`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.new)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.80.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-180-2024-07-25), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-180).

## Contributors to 1.80.0

Many people came together to create Rust 1.80.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.80.0/)
