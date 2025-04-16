# The Rust blog

[![CI](https://github.com/rust-lang/blog.rust-lang.org/actions/workflows/main.yml/badge.svg)](https://github.com/rust-lang/blog.rust-lang.org/actions/workflows/main.yml)

This is the blog of the Rust Programming Language.

It uses [Zola](https://www.getzola.org/) and is deployed to GitHub Pages via GitHub Actions.

## Building

To serve the site locally, first install Zola: (takes a couple minutes)

```sh
# using a fork because we rely on a few patches that haven't landed yet
cargo install --locked --git https://github.com/senekor/zola --rev 620bf3c46a39b41db30b1e91756a995bbff84d3a
```

Now run `zola serve --open`.
The site will be reloaded automatically when you make any changes.

## Contributing

First of all, thank you!

Like everything in Rust, the blog is licensed MIT/Apache 2.0. See the two
`LICENSE-*` files for more details. We're also governed by the Rust
Code of Conduct, see `CODE_OF_CONDUCT.md` for more.

### Writing a new blog post

If you want to include images in your post, please store them in the repository.
You can store your main blog post in `content/<some-slug>/index.md`.
Images go into the same directory: `content/<some-slug>/my_image.png`.
Now you can reference that image with a simple relative path: `![alt text](my_image.png)`.

A post's date of publication is embedded in the `path` key of the front matter.
Keep the placeholder (`9999/12/99`) until the post is about to be merged.
You can easily do so by adding a comment containing the string `publish=today` on the PR.
Don't worry, there's a merge queue check to make sure you don't forget.

Here is an example of the front matter format:
```md
+++
path = "9999/12/99/some-slug"
title = "Title of the blog post"
authors = ["Blog post author (or on behalf of which team)"]
description = "(optional)"
aliases = ["releases/X.XX.X"] # only if the post is a release

[extra] # optional section
team = "Team Name" # if post is made on behalf of a team
team_url = "https://www.rust-lang.org/governance/teams/..." # required if team is set
release = true # (to be only used for official posts about Rust releases announcements)
+++
```

### Snapshot testing

If you're making changes to how the site is generated, you may want to check the impact your changes have on the output.
For this purpose, there is a setup to do snapshot testing over the entire output directory.

To run these tests in CI, add the string `RUN_SNAPSHOT_TESTS` to the PR description.
You can also run these tests locally for a faster feedback cycle:

- Make sure you have [cargo-insta](https://insta.rs/docs/quickstart/) installed.

- Generate the good snapshots to compare against, usually based off the master branch:
  ```sh
  cargo insta test --accept --include-ignored
  ```
  Consider making a commit with these snapshots, so you can always check the diff of your changes with git:
  ```sh
  git add --force snapshot/src/snapshots # snapshots are ignored by default
  git commit --message "WIP add good snapshots"
  ```
  Since we can't merge the snapshots to main, don't forget to drop this commit when opening a pull request.

- Compare the output of the branch you're working on with the good snapshots:
  ```sh
  cargo insta test --review --include-ignored
  ```
