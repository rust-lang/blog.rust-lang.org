+++
path = "2019/10/15/Rustup-1.20.0"
title = "Announcing Rustup 1.20.0"
authors = ["The Rustup Working Group"]
aliases = ["2019/10/15/Rustup-1.20.0.html"]
+++

The rustup working group is happy to announce the release of rustup version 1.20.0. [Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

If you have a previous version of rustup installed, getting rustup 1.20.0 is as easy as:

```
rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

[rust]: https://www.rust-lang.org
[install]: https://rustup.rs

## What's new in rustup 1.20.0

The highlights of this release are profiles support, the ability to get the latest available nightly with all the components you need, and improvements to the `rustup doc` command. You can also check out [the changelog][changelog] for a list of all the changes included in this release.

[changelog]: https://github.com/rust-lang/rustup.rs/blob/master/CHANGELOG.md

### Profiles

Previous versions of rustup installed a few components by default along with each toolchain: the compiler (`rustc`), the package manager (`cargo`), the standard library (`rust-std`), and offline documentation (`rust-docs`). While this approach is fine while developing software locally, some of the components (like `rust-docs`) slowed down the installation, either because they're not used on build servers, or on Windows due to the large amount of installed files.

To address this problem, rustup 1.20.0 introduces the concept of "profiles". They are groups of components you can choose to download while installing a new Rust toolchain. The profiles available at this time are `minimal`, `default`, and `complete`:

* The **minimal** profile includes as few components as possible to get a working compiler (`rustc`, `rust-std`, and `cargo`). It's recommended to use this component on Windows systems if you don't use local documentation, and in CI.
* The **default** profile includes all the components previously installed by default (`rustc`, `rust-std`, `cargo`, and `rust-docs`) plus `rustfmt` and `clippy`. This profile will be used by rustup by default, and it's the one recommended for general use.
* The **complete** profile includes all the components available through rustup, including `miri` and IDE integration tools (`rls` and `rust-analysis`).

To change the rustup profile you can use the `rustup set profile` command. For example, to select the minimal profile you can use:

```
rustup set profile minimal
```

It's also possible to choose the profile when installing rustup for the first time, either interactively by choosing the "Customize installation" option or programmatically by passing the `--profile=<name>` flag. Profiles will only affect newly installed toolchains: as usual it will be possible to install individual components later with: `rustup component add`.

### Installing the latest compatible nightly

While most components are guaranteed to be present on stable releases of [tier 1 platforms][tiers], the same guarantee doesn't apply to nightly builds. Frequently, tools such as `rustfmt`, `clippy`, or `rls` are missing in the latest nightly. If you depend on these tools, that makes updating nighties hard, as rustup will prevent the upgrade if a component you previously installed is missing.

Starting from rustup 1.20.0, if a component you previously installed is missing in the latest nightly, `rustup update` will walk backwards in time to find the most recent release with all the components you need. If there are no new nightlies with all the components you need you'll either need to wait or remove some of them.

Along with this change, rustup 1.20.0 introduces the `--component`/`-c` and `--target`/`-t` options to the `rustup toolchain install` command, allowing you to add components and targets as the toolchain is installed. These flags will also search past nightlies if the current one does not feature all the requested components. 

[tiers]: https://forge.rust-lang.org/release/platform-support.html

### Improvements to `rustup doc`

The `rustup doc` command opens the locally installed documentation on your browser, without any Internet connection required. rustup 1.20.0 enhances the command allowing you to open directly the API documentation of a specific item. For example to look at the documentation of [`Iterator`] you can use:

```
rustup doc std::iter::Iterator
```

This works for traits, structs/enums, macros, and modules, and can take you to the `std`, `alloc`, and `core` crates. Note, however, that this functionality will only work if you have the `rust-docs` component installed in your toolchain. We will be improving the command's UX over time, so if you have ideas, please do let us know!

[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

## Thanks

Thanks to all the contributors who made rustup 1.20.0 possible!

- Andy McCaffrey
- Artem Borisovskiy
- Benjamin Chen
- Daniel Silverstone
- Jon Gjengset
- Lzu Tao
- Matt Kantor
- Mitchell Hynes
- Nick Cameron
- PicoJr
- Pietro Albini
