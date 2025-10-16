+++
path = "inside-rust/2025/10/16/renaming-the-default-branch-of-rust-lang-rust"
title = "Renaming the default branch of rust-lang/rust"
authors = ["Jake Goulding"]
aliases = ["inside-rust/2025/10/16/renaming-the-default-branch-of-rust-langrust"]

[extra]
team = "the Infra team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-infra"
+++

We will be renaming the default branch of the [rust-lang/rust repository](https://github.com/rust-lang/rust) from `master` to `main` on 2025-11-10. We've chosen `main` specifically as it's the [new default in git][git-change] and the renaming will leverage the [GitHub tooling][github-tooling] built to make this easier.

If you maintain a tool that currently assumes the default branch of `rust-lang/rust` is named `master`, using `HEAD` instead will work both before and after the rename.

After the rename, contributors will need to run the following commands in their local checkout of the repository, assuming that your remote registry is called `origin`:

```bash
git branch -m master main
git fetch origin
git branch -u origin/main main
git remote set-head origin -a

# optional
git remote prune origin
```

If you have a fork of the `rust-lang/rust` repository on GitHub and would like to rename your default branch to match, you can follow [GitHub's instructions][github-how-to-rename].

[git-change]: https://lore.kernel.org/git/pull.656.v4.git.1593009996.gitgitgadget@gmail.com/

[github-tooling]: https://github.com/github/renaming

[github-how-to-rename]: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-branches-in-your-repository/changing-the-default-branch
