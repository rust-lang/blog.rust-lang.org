# The Rust blog

[![Build Status](https://travis-ci.com/rust-lang/blog.rust-lang.org.svg?branch=master)](https://travis-ci.com/rust-lang/blog.rust-lang.org)

This is the blog of the Rust Programming Language.

It's implemented as a small static site generator, that's deployed to GitHub
Pages via Travis.

## Building

To build the site locally:

```console
> git clone https://github.com/rust-lang/blog.rust-lang.org
> cd blog.rust-lang.org
> cargo run
```

You could do it in release mode if you'd like, but it's pretty fast in debug.

From there, the generated HTML will be in a `site` directory.
Open `site/index.html` in your web browser to view the site.

```console
> firefox site/index.html
```

You can also run a server, if you need to preview your changes on a different machine:

```console
> cargo run --bin serve
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
```
---
layout: post
title: Title of the blog post
author: Blog post author (or on behalf of which team)
release: true (to be only used for official posts about Rust releases announcements)
---
```
