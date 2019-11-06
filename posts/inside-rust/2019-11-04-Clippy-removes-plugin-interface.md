---
layout: post
title: "Clippy is removing its plugin interface"
author: Philipp Krones
description: "Now that compiler plugins are deprecated, Clippy is removing its deprecated plugin interface"
team: the Dev tools team (Clippy) <https://www.rust-lang.org/governance/teams/dev-tools#clippy>
---

Today, we're announcing that Clippy will completely remove its plugin interface.
Using the plugin interface has been deprecated for about one and a half year now
([rust-lang/rust-clippy#2712]). Since then, an unsilenceable warning has been
emitted. Now that compiler plugins are officially deprecated
([rust-lang/rust#64675]), Clippy will remove its support for the plugin
interface completely ([rust-lang/rust-clippy#4714]). This change will hit stable
with version 1.41.0.

[rust-lang/rust-clippy#2712]: https://github.com/rust-lang/rust-clippy/pull/2712
[rust-lang/rust#64675]: https://github.com/rust-lang/rust/pull/64675
[rust-lang/rust-clippy#4714]: https://github.com/rust-lang/rust-clippy/pull/4714

### Does this post affect me?

Most likely, no. This post only affects you, if you're still using Clippy
through its plugin interface. If you do so, you get the warning

```
warning: the clippy plugin is being deprecated, please use cargo clippy or rls with the clippy feature
```

when compiling your crate. If you don't see this warning, nothing will change
for you.

### How do I migrate from the plugin interface?

If you are still using the Clippy plugin interface, here are some steps you can
take to migrate to `cargo clippy`.

1. `Cargo.toml`: Remove every occurrence of the `clippy` dependency and the
   `clippy` feature.
2. Completely remove every occurrence of `feature(plugin)` and `plugin(clippy)`.
3. Replace every occurrence of `feature = "clippy"` with `feature =
   "cargo-clippy"`. The `cargo-clippy` feature is automatically enabled when
   running `cargo clippy`.
4. CI: You now have to install Clippy via rustup, with `rustup component add
   clippy`. Once installed, you can run `cargo clippy` (for more usage
   instructions, see the [Clippy `README`]). Note that Clippy is not included in
   every nightly, but you can check its availability on the [rustup components
   history] page.

[Clippy `README`]: https://github.com/rust-lang/rust-clippy#usage
[rustup components history]: https://rust-lang.github.io/rustup-components-history/index.html

### Where should I go if I have more questions?

If you need help with migrating from the plugin interface, you can contact us
via [Discord] or open an issue on [GitHub].

[Discord]: https://discord.gg/vNNtpyD
[GitHub]: https://github.com/rust-lang/rust-clippy/issues/new
