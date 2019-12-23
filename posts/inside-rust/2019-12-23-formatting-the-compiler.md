---
layout: post
title: "Formatting the compiler tree"
author: Mark Rousskov
description: "How to rebase and what happened"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

## What happened

We recently landed two PRs which together reformatted essentially all code in the compiler tree.

The first one, [#65939], contained the initial formatting infrastructure. We currently use `rustfmt`
directly, pinned to a version specified in `src/stage0.txt`. We expect to update it as needed, and
otherwise once per cycle (coinciding with the bootstrap bump, most likely).

The second one which reformatted the majority of the codebase is [#67540].

This change landed with the following rustfmt config. Note that this this configuration is subject
to change (in particular, merge_derives may be removed in the future), but should be fairly stable.
Your editor should automatically pick this configuration up inside the rust-lang/rust repository (it
is located in the `rustfmt.toml` file in the root).

```
version = "Two"
use_small_heuristics = "Max"
merge_derives = false
```

## How to use formatting

You can reformat the repository with `x.py fmt` and `x.py fmt --check` to verify formatting; these
commands are unfortunately somewhat slow today. Tidy will also currently run the latter of these two
checks (`x.py fmt --check`) internally, but this may change in the future if we can't improve the
speed of formatting the entire codebase.

## Resolving conflicts

If you have an ongoing branch, you're likely to have merge conflicts. The following should help you
resolve them:

```
# This should be the name of the remote for rust-lang/rust
git fetch upstream
# This rebases up to the bors merge right before formatting landed;
# it needs to be done manually.
git rebase -i 9b98af84c4aa66392236fff59c86da2130d46d46
# This rebases onto the formatting PR (given the previous command, only that).
# We tell git to resolve all conflicts in favor of your code (`-Xtheirs`),
# and the `--exec` command specifies that after each commit lands, it will be formatted.
# This command will fail if your PR has intermediary commits with syntax conflicts.
git rebase -i a916ac22b9f7f1f0f7aba0a41a789b3ecd765018 \
    --exec './x.py fmt && git add -u && git commit --amend` \
    # This exec is optional, and won't work if your intermediate commits don't build,
    # but it helps make sure that the formatting resolution didn't introduce any errors.
    # It's recommended to run it afterwards before pushing at least.
    --exec './x.py check' \
    -Xtheirs
```

This should mostly resolve conflicts correctly, but occasionally if you've edited something in
imports (a common case I've encountered) or otherwise this will not resolve quite right. Usually
though this will solve 99% of the problems and the rest can be fixed up manually afterwards.

[#65939]: https://github.com/rust-lang/rust/pull/65939
[#67540]: https://github.com/rust-lang/rust/pull/67540
