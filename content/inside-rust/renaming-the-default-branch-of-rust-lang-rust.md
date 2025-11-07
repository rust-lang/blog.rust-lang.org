+++
path = "inside-rust/2025/10/16/renaming-the-default-branch-of-rust-lang-rust"
title = "Renaming the default branch of rust-lang/rust"
authors = ["Jake Goulding"]
aliases = ["inside-rust/2025/10/16/renaming-the-default-branch-of-rust-langrust"]

[extra]
team = "the Infra team"
team_url = "https://www.rust-lang.org/governance/teams/infra#team-infra"
+++

We will be renaming the default branch of the [rust-lang/rust repository](https://github.com/rust-lang/rust) from `master` to `main` on 2025-11-10. We've chosen `main` specifically as it's the default for newly-created repositories [in GitHub][github-change] and the renaming will leverage the [GitHub tooling][github-tooling] built to make this easier.

If you maintain a tool that currently assumes the default branch of `rust-lang/rust` is named `master`, using `HEAD` instead will work both before and after the rename.

After the rename, contributors will need to run a few git commands in their local checkout of the repository to update. Note that the specific commands that should be executed might differ based on the way you use git and how your local checkout is configured. We provide a guide below that we think should work for most use-cases, but your mileage may vary.

Please try to follow the guide step-by-step, and if you run into any problems, feel free to ask in this [Zulip channel][infra-help-zulip].

## Renaming your fork's default branch

If you have a fork of the `rust-lang/rust` repository on GitHub, you should first update its default branch name before continuing. If you do not rename it and later run `git checkout master`, git will create a `master` branch based on your fork's outdated `master` branch, which can be confusing. Some of the following git commands in this post might also not work as expected.

Here is how you can update your fork's default branch:

1) Go to `https://github.com/<your-username>/rust/settings`
2) Find the default branch section, click on the "Rename branch" button (pencil icon) and rename the branch to `main`.

## Updating your local git checkout

Execute the following git commands in your local checkout of the `rust` repository.

Note that the instructions below make two assumptions:
- You have a git remote called `upstream` that points to the `rust-lang/rust` repository and a remote called `origin` that points to your `<username>/rust` fork. Please update the commands accordingly if you use a different setup.
  - You can find out which remotes you have configured using the `git remote -v` command.
- Your local `master` branch tracks the default branch of your *fork*, not the default branch of the *upstream* `rust-lang/rust` repository. If that is not the case, execute the commented `git branch` command instead.
  - You can find which remote your local `master` branch tracks by using `git branch -vv --contains master` (the tracked branch should be shown in square brackets).

```bash
# Update the local branch name
git branch --move master main

# Update local references to your <username>/rust fork
git fetch origin
git branch --set-upstream-to=origin/main main # If your main branch tracks your fork
git remote set-head origin --auto

# Update local references relevant to the upstream rust-lang/rust repository
git fetch upstream
#git branch --set-upstream-to=upstream/main main # If your main branch tracks the upstream repo
git remote set-head upstream --auto

# Remove old branch names (optional, but recommended)
git remote prune origin
git remote prune upstream
```

[github-change]: https://github.blog/changelog/2020-10-01-the-default-branch-for-newly-created-repositories-is-now-main/

[github-tooling]: https://github.com/github/renaming

[infra-help-zulip]: (https://rust-lang.zulipchat.com/#narrow/channel/242791-t-infra/topic/Help.20with.20updating.20rust-lang.2Frust.20default.20branch/with/554127642)
