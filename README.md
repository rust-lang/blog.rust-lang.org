# The Rust blog

[![CI](https://github.com/rust-lang/blog.rust-lang.org/actions/workflows/main.yml/badge.svg)](https://github.com/rust-lang/blog.rust-lang.org/actions/workflows/main.yml)

This is the blog of the Rust Programming Language.

It's implemented as a small static site generator, that's deployed to GitHub
Pages via GitHub Actions.

## Building

To build the site locally:

```console
$ git clone https://github.com/rust-lang/blog.rust-lang.org
$ cd blog.rust-lang.org
$ cargo run
```

You could do it in release mode if you'd like, but it's pretty fast in debug.

From there, the generated HTML will be in a `site` directory.
Open `site/index.html` in your web browser to view the site.

```console
$ firefox site/index.html
```

You can also run a server, if you need to preview your changes on a different machine:

```console
$ cargo run -p serve
Serving on: http://192.168.123.45:8000
```

## Contributing

First of all, thank you!

Like everything in Rust, the blog is licensed MIT/Apache 2.0. See the two
`LICENSE-*` files for more details. We're also governed by the Rust
Code of Conduct, see `CODE_OF_CONDUCT.md` for more.

Please send pull requests to the master branch. If you're trying to do
something big, please open an issue before working on it, so we can make sure
that it's something that will eventually be accepted.

When writing a new blog post, keep in mind the file headers:
```md
+++
layout = "post"
date = 2015-03-15
title = "Title of the blog post"
author = "Blog post author (or on behalf of which team)"
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
  git add --force src/snapshots # snapshots are ignored by default
  git commit --message "WIP add good snapshots"
  ```
  Since we can't merge the snapshots to main, don't forget to drop this commit when opening a pull request.

- Compare the output of the branch you're working on with the good snapshots:
  ```sh
  cargo insta test --review --include-ignored
  ```
