+++
layout = "post"
title = "The rustc-guide is now rustc-dev-guide"
author = "mark-i-m"
description = "the guide has been renamed"
team = "the rustc dev guide working group <https://www.rust-lang.org/governance/teams/compiler#wg-rustc-dev-guide>"
+++

You may or may not be aware of two similarly named resources:
- [The rustc book](https://doc.rust-lang.org/rustc/index.html)
- [The rustc-guide](https://rustc-dev-guide.rust-lang.org/)

What's the difference? The "rustc book" is intended as a guide for _users_ of the
compiler. The "rustc-guide" is intended as a guide for _developers_ of the
compiler. However, the names have always been confusingly similar.

**For that reason, we have decided to rename the `rustc-guide` to
`rustc-dev-guide`. You can now find it at [https://rustc-dev-guide.rust-lang.org/][guide].**

[guide]: https://rustc-dev-guide.rust-lang.org/

We have put significant work into finding and updating links around the
`rust-lang/*` repos. We have also put up a website in place of the former
`rustc-guide` website that redirects to the new one.

To update your git clone of the (former) `rustc-guide` repo, you can do the following:

```
git remote set-url origin https://github.com/rust-lang/rustc-dev-guide.git
```

This will change where git thinks the `origin` remote repo is to
`rust-lang/rustc-dev-guide`.

You can find more information in [this issue][gh]. If you have any questions or
concerns, please feel free to contact us either by opening an [issue on the
rustc-dev-guide repo][repo] or on our [zulip channel][zulip].

[gh]: https://github.com/rust-lang/rustc-dev-guide/issues/602
[repo]: https://github.com/rust-lang/rustc-dev-guide
[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-rustc-dev-guide
