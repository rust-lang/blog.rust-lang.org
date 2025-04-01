+++
path = "inside-rust/2019/12/23/formatting-the-compiler"
title = "Formatting the compiler tree"
authors = ["Mark Rousskov"]
description = "How to rebase and what happened"
aliases = ["inside-rust/2019/12/23/formatting-the-compiler.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

## What happened

We recently landed two PRs which together reformatted essentially all code in the compiler tree.

The first one, [#65939], contained the initial formatting infrastructure. We currently use `rustfmt`
directly, pinned to a version specified in `src/stage0.txt`. We expect to update it as needed, and
otherwise once per cycle (coinciding with the bootstrap bump, most likely).

The second one which reformatted the majority of the codebase is [#67540].

This change landed with the following rustfmt config. Note that this configuration is subject
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

```bash
#!/bin/bash

set -xeo pipefail

if [ "$1" = "from-rebase" ] ; then
	git rev-parse HEAD > /tmp/commit
	git rev-parse HEAD >> /tmp/old.shas
	./x.py fmt
	git commit -a --amend --no-edit
	git rev-parse HEAD >> /tmp/new.shas
	git reset --hard $(cat /tmp/commit)
else
	rm -f /tmp/old.shas /tmp/commit /tmp/new.shas
	git rebase 8eb7c58dbb7 --exec '../format.sh from-rebase'
	branch=$(git rev-parse --abbrev-ref HEAD) # get branch name
	git reset --hard 8eb7c58dbb7
	for sha in $(cat /tmp/new.shas); do
		git cherry-pick $sha -Xtheirs
	done
  # put yourself atop the format the world PR
  git rebase -Xtheirs a916ac22b9f7f1f0f7aba0a41a789b3ecd765018
fi
```

This script should be saved to `format.sh` in the parent directory of your Rust
checkout, and then run `git fetch upstream && ../format.sh`. `upstream` should
be the name of the rust-lang/rust remote.

Once the script runs, you will be based on the `a916ac22b9f7f` commit. You
likely want to then run `git rebase -i upstream/master` or so to finish, but the
script above gets you past the formatting PR at least.

This should mostly resolve conflicts correctly, but occasionally if you've edited something in
imports (a common case I've encountered) or otherwise this will not resolve quite right. Usually
though this will solve 99% of the problems and the rest can be fixed up manually afterwards.

[#65939]: https://github.com/rust-lang/rust/pull/65939
[#67540]: https://github.com/rust-lang/rust/pull/67540
