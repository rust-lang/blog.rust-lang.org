+++
path = "2025/06/26/Rust-1.88.0"
title = "Announcing Rust 1.88.0"
authors = ["The Rust Release Team"]
aliases = ["releases/1.88.0"]

[extra]
release = true
+++

The Rust team is happy to announce a new version of Rust, 1.88.0. Rust is a programming language empowering everyone to build reliable and efficient software.

If you have a previous version of Rust installed via `rustup`, you can get 1.88.0 with:

```
$ rustup update stable
```

If you don't have it already, you can [get `rustup`](https://www.rust-lang.org/install.html) from the appropriate page on our website, and check out the [detailed release notes for 1.88.0](https://doc.rust-lang.org/stable/releases.html#version-1880-2025-06-26).

If you'd like to help us out by testing future releases, you might consider updating locally to use the beta channel (`rustup default beta`) or the nightly channel (`rustup default nightly`). Please [report](https://github.com/rust-lang/rust/issues/new/choose) any bugs you might come across!

## What's in 1.88.0 stable

### Cargo automatic cache cleaning

Starting in 1.88.0, Cargo will automatically run garbage collection on the cache in its home directory!

When building, Cargo downloads and caches crates needed as dependencies. Historically, these downloaded files would never be cleaned up, leading to an unbounded amount of disk usage in Cargo's home directory. In this version, Cargo introduces a garbage collection mechanism to automatically clean up old files (e.g. `.crate` files). Cargo will remove files downloaded from the network if not accessed in 3 months, and files obtained from the local system if not accessed in 1 month. Note that this automatic garbage collection will not take place if running offline (using `--offline` or `--frozen`).

Cargo 1.78 and newer track the access information needed for this garbage collection. This was introduced well before the actual cleanup that's starting now, in order to reduce cache churn for those that still use prior versions. If you regularly use versions of Cargo even older than 1.78, in addition to running current versions of Cargo, and you expect to have some crates accessed exclusively by the older versions of Cargo and don't want to re-download those crates every ~3 months, you may wish to set `cache.auto-clean-frequency = "never"` in the Cargo configuration, as described in the [docs](https://doc.rust-lang.org/nightly/cargo/reference/config.html#cache).

For more information, see the original [unstable announcement](https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning/) of this feature. Some parts of that design remain unstable, like the `gc` subcommand tracked in [cargo#13060](https://github.com/rust-lang/cargo/issues/13060), so there's still more to look forward to!

### Let chains

This feature allows `&&`-chaining `let` statements inside `if` and `while` conditions, even intermingling with boolean expressions, so there is less distinction between `if`/`if let` and `while`/`while let` anymore. The patterns inside the `let` sub-expressions can be irrefutable or refutable, and bindings are usable in later parts of the chain as well as the body.

For example, [this actual snippet](https://github.com/rust-lang/rust/blob/28f1c807911c63f08d98e7b468cfcf15a441e34b/compiler/rustc_ast_passes/src/feature_gate.rs#L327-L338) from the compiler combines multiple conditions which would have required nesting `if let` and `if` blocks before:

```rust
    fn visit_generic_args(&mut self, args: &'a ast::GenericArgs) {
        if let ast::GenericArgs::Parenthesized(generic_args) = args
            && let ast::FnRetTy::Ty(ref ty) = generic_args.output
            && matches!(ty.kind, ast::TyKind::Never)
        {
            gate!(&self, never_type, ty.span, "the `!` type is experimental");
        }
        visit::walk_generic_args(self, args);
    }
```

Let chains are only available in the Rust 2024 edition, as this feature depends on the [`if let` temporary scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html) change for more consistent drop order. Earlier efforts tried to work with all editions, but some difficult edges cases threatened the integrity of the implementation. 2024 made it feasible, so please upgrade your crate's edition if you'd like to use this feature!

### Naked functions

Rust now supports writing naked functions with no compiler-generated epilogue and prologue, allowing full control over the generated assembly for a particular function. This is a more ergonomic alternative to defining functions in a `global_asm!` block. A naked function is marked with the `#[unsafe(naked)]` attribute, and its body consists of a single `naked_asm!` call, for example:

```rust
#[unsafe(naked)]
pub unsafe extern "sysv64" fn wrapping_add(a: u64, b: u64) {
    // Equivalent to `a.wrapping_add(b)`.
    core::arch::naked_asm!(
        "add rax, rdi, rsi",
        "ret"
    );
}
```

The handwritten assembly block defines the _entire_ function body: unlike non-naked functions, the compiler does not add any special handling for arguments or return values. Naked functions are used in low-level settings like Rust's [`compiler-builtins`](https://github.com/rust-lang/compiler-builtins), operating systems, and embedded applications.

Look for a more detailed post soon on [Inside Rust](https://blog.rust-lang.org/inside-rust/)!

### Boolean configuration

The `cfg` predicate language now supports boolean literals, `true` and `false`, acting as a configuration that is always enabled or disabled, respectively. This works in Rust [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html) with `cfg` and `cfg_attr` attributes and the built-in `cfg!` macro, and also in Cargo `[target]` tables in both [configuration](https://doc.rust-lang.org/cargo/reference/config.html#target) and [manifests](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#platform-specific-dependencies).

If you are familiar with set theory, you might already be using empty sets like `cfg(all())` for enabled and `cfg(any())` for disabled, but this meaning is rather implicit and easy to get backwards. The boolean literals offer a more direct way to say what you mean.

See [RFC 3695](https://rust-lang.github.io/rfcs/3695-cfg-boolean-literals.html) for more!

### Stabilized APIs

- [`Cell::update`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.update)
- [`impl Default for *const T`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#impl-Default-for-*const+T)
- [`impl Default for *mut T`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#impl-Default-for-*mut+T)
- [`HashMap::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.extract_if)
- [`HashSet::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html#method.extract_if)
- [`proc_macro::Span::line`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.line)
- [`proc_macro::Span::column`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.column)
- [`proc_macro::Span::start`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.start)
- [`proc_macro::Span::end`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.end)
- [`proc_macro::Span::file`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.file)
- [`proc_macro::Span::local_file`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.local_file)

These previously stable APIs are now stable in const contexts:

- [`NonNull<T>::replace`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.replace)
- [`<*mut T>::replace`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.replace)
- [`std::ptr::swap_nonoverlapping`](https://github.com/rust-lang/rust/pull/137280)
- [`Cell::{replace, get, get_mut, from_mut, as_slice_of_cells}`](https://github.com/rust-lang/rust/pull/137928)

### Other changes

Check out everything that changed in [Rust](https://github.com/rust-lang/rust/releases/tag/1.88.0), [Cargo](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-188-2025-06-26), and [Clippy](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#rust-188).

## Contributors to 1.88.0

Many people came together to create Rust 1.88.0. We couldn't have done it without all of you. [Thanks!](https://thanks.rust-lang.org/rust/1.88.0/)
