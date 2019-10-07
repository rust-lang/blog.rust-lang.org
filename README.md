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

From there, the generated HTML will be in a `site` directory. You can use
any web server to check it out in your browser:

```console
> cd site
> python3 -m http.server
```

The site is now available at <0.0.0.0:8000>.

## Contributing

First of all, thank you!

Like everything in Rust, the blog is licensed MIT/Apache 2.0. See the two
`LICENSE-*` files for more details. We're also governed by the Rust
Code of Conduct, see `CODE_OF_CONDUCT.md` for more.

Please send pull requests to the master branch. If you're trying to do
something big, please open an issue before working on it, so we can make sure
that it's something that will eventually be accepted.