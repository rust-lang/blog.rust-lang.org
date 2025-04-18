+++
path = "2018/10/30/help-test-rust-2018"
title = "Help test Rust 2018"
authors = ["The Rust Core Team"]
aliases = ["2018/10/30/help-test-rust-2018.html"]
+++

Back in July, we talked about ["Rust 2018"]. In short, we are launching a
cycle of long-term milestones called "Editions". Editions are a way to
capture the progress delivered incrementally by our ordinary six-week release
cycle -- and focus Rust libraries, tooling, and documentation cohesively
around it. Editions will be selected roughly every three years: Rust 1.0 was
"Rust 2015" and Rust 1.31 will be "Rust 2018". Each edition has a theme;
Rust 2015's was "stability", and Rust 2018's is "productivity."

We've been [testing Rust 2018 for a while already], and things are looking
pretty good! We have just under six weeks until Rust 1.31 ships, and so
we'd appreciate it if you could give the beta a try.

There's two ways to try out Rust 2018: updating an existing project, and
starting a new one. For full details, please check out the [Edition Guide],
but the rest of this post is a quickstart to make it even easier.

If anything goes wrong, or is confusing, please [file an issue] and let us
know. We want to make sure this is an extra-awesome release! Thank you for
helping us make Rust even better. <3

["Rust 2018"]: https://blog.rust-lang.org/2018/07/27/what-is-rust-2018.html
[testing Rust 2018 for a while already]: https://internals.rust-lang.org/t/rust-2018-release-schedule-and-extended-beta/8076
[Edition Guide]: https://rust-lang-nursery.github.io/edition-guide/
[file an issue]: https://github.com/rust-lang/rust/issues/new

## Setup: install Rust beta

First things first, you'll need to install the beta release channel of Rust.
With [Rustup], it's as easy as:

```
$ rustup install beta
```

To use this channel of Rust instead of your default, you can append a `+beta`
to any `rustc` or cargo commands:

```
$ rustc +beta --version
$ cargo +beta build
```

This lets you stick to stable as the default, while using beta for your
experiments.

[Rustup]: https://www.rust-lang.org/en-US/install.html

## Start a new project

To start a new project with Rust 2018:

```
$ cargo +beta new my-sample-project
```

Nothing changes! Well, something changed. Check out `Cargo.toml`:

```toml
[package]
name = "my-sample-project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

That new `edition = "2018"` key/value pair means you're working with Rust 2018.
If it doesn't exist, it's the same as `edition = "2015"`, so all
existing projects keep working.

## Convert an existing project

You can also convert an existing project to Rust 2018. Remember, none of your
dependencies need to be updated for this to work; Rust 2018 and 2015
interoperate seamlessly!

The first step is to run `cargo fix`:

```
$ cargo fix --edition
```

This will check your code, and automatically fix any issues that it can.
`cargo fix` is still pretty new, and so it can't always fix your code
automatically. If `cargo fix` can't fix something, it will print the warning
that it cannot fix to the console. If you see one of these warnings, you'll
have to update your code manually. See the corresponding section of the
edition guide for help, and if you have problems, please seek help at the
user's forums.

Keep running `cargo fix --edition` until you have no more warnings.

Congrats! Your code is now valid in both Rust 2015 and Rust 2018!

Once this is done, you can commit to Rust 2018 by updating
your `Cargo.toml`:

```toml
[package]
name = "my-sample-project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
```

See that `edition = "2018"`? That's what opts you in to the new features.
Set it, `cargo +beta build`, and you should be good to go!
