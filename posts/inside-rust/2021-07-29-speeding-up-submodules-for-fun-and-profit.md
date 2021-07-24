---
layout: post
title: "Speeding up submodules for fun and profit"
author: Joshua Nelson
description: "What are submodules, how did they get faster, and why did it take so long?"
---

`x.py` now has faster handling of submodules! We need your help to make x.py faster :)

## How can I opt-in to these changes?

If you're cloning the rust repository for the first time, no action is necessary. If you have an
existing clone of `rust-lang/rust`, you can almost double the speed of running `git fetch` by
running the following command:

```sh
git submodule deinit src/doc/book src/doc/edition-guide src/doc/embedded-book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/rustc-dev-guide src/llvm-project src/tools/rust-analyzer
```

You should see something similar to the following output:

```
Cleared directory 'src/doc/book'
Submodule 'src/doc/book' (https://github.com/rust-lang/book.git) unregistered for path 'src/doc/book'
...
Submodule 'src/tools/rust-analyzer' (https://github.com/rust-analyzer/rust-analyzer.git) unregistered for path 'src/tools/rust-analyzer'
```

If you see an error instead:

```
error: the following file has local modifications:
    src/doc/nomicon
(use --cached to keep the file, or -f to force removal)
fatal: Submodule work tree 'nomicon' contains local modifications; use '-f' to discard them
```

Then you can either:
1. Discard your changes permanently: `git submodule deinit -f src/doc/nomicon`
2. Choose not to clear the submodule (by removing it from the `git submodule` command you run).

## What does this do?

By default, `x.py` clones the submodules for various dependencies the first time you run it.
However, not all these submodules are necessary for most of the tasks you'll use: for example,
`src/doc/nomicon` is only necessary if you're documenting the Rustonomicon with `x.py doc`.
Some of these submodules are very large (especially `src/llvm-project`), so cloning them eagerly
slows down x.py quite a lot.

`git submodule deinit` removes your local checkouts of the submodules. You can still download them
again at any time with `git submodule update --init --recursive`, and if you run any x.py command
that requires them, they'll be cloned automatically. However, `x.py` won't remove any existing
clones you have (in case you have branches you were working on), so you need to opt-in if you have
existing clones. If you're cloning the `rust-lang/rust` repo for the first time, this is automatic
and there's no action necessary.

## Why is this only happening now?

I first came up with the idea to [avoid cloning submodules] in September 2020, almost a year ago!
It took me until January to get fed up enough to start working on it myself. From there, it turned
out that this change was quite difficult. There were several problems:

- All the existing logic for updating submodules was in [`bootstrap.py`], so I had to rewrite it into the Rust bootstrapper.
- If you have a submodule already downloaded, it should continue to be updated, or you'll end up with strange build errors.
- People have many different workflows they use to build Rust! Some workflows I had to adapt to:
    - Submodules should never be updated when `build.submodules = false`. [87049]
    - Tarballs with the source code don't have `.gitmodules` available, so it's not possible to tell what the submodules are in the first place. [82653]
    - The CI for `rust-lang/rust` actually bypasses git altogether and just downloads a zip archive, but still runs `touch .git` for each submodule. [81601]

These were especially hard to fix because it's very difficult to test changes to submodule updating
- the expected behavior is unclear and usually isn't known until people complain that you've broken
their workflow. However, all known issues have now been fixed. There are still some remaining issues -
in particular, `x.py` is only smart for submodules which aren't in the Cargo workspace. Unfortunately,
Cargo requires that we have all the Cargo.toml files available whenever it's run, even if it's not yet
compiling those projects. I hope to [extend this lazy-cloning to members of the Cargo workspace][76653] in
the future.

[avoid cloning submodules]: https://github.com/rust-lang/rust/issues/76653
[`bootstrap.py`]: https://rustc-dev-guide.rust-lang.org/building/bootstrapping.html#contributing-to-bootstrap
[76653]: https://github.com/rust-lang/rust/issues/76653
[87049]: https://github.com/rust-lang/rust/pull/87049
[82653]: https://github.com/rust-lang/rust/pull/82653
[81601]: https://github.com/rust-lang/rust/pull/81601#issuecomment-772678513
