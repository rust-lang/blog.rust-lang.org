---
layout: post
title: "Bisecting Rust Compiler Regressions with cargo-bisect-rustc"
author: Santiago Pastorino
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

Let's say that you've just updated the Rust compiler version and have
tried to compile your application and see a failure that wasn't there
before. That's likely due to a regression in the compiler.  We've just
released
[`cargo-bisect-rustc`](https://github.com/rust-lang/cargo-bisect-rustc),
a tool that makes it super easy to find exactly when the regression
happened.

`cargo-bisect-rustc` automatically downloads rustc artifacts and tests
them against a project you provide until it finds the regression. At
minimum, it will identify the nightly release which triggered the
regression; but if the regression occurred in the last 168 days, it will
even figure out the exact PR, which is often very useful in helping us
fix the problem.

`cargo-bisect-rustc` was created originally by Mark Rousskov. I extended
it recently to make it easier to use.

To install the tool run:

```sh
cargo install cargo-bisect-rustc
```

## Finding a regression

We are going to use [this "old" reported rustc
regression](https://github.com/rust-lang/rust/issues/64945) as an
example:

Our application consists only of this file:

```rust
pub struct Slice<'a, T>(&'a [T]);

impl<'a, T: 'a> Slice<'a, T> {
    pub const EMPTY: Self = Slice ({
        let v: &[T] = &[];
        v
    });
}

fn main() {
    let s = Slice(&[1, 2]);
    assert!(s.0 != Slice::EMPTY.0);
}
```

Then we run `cargo bisect-rustc --end=2019-10-02`.

Since this bug was fixed on 2019-10-03, we're using 2019-10-02 as the
end We need to provide the end point for this particular example, given
that this bug was fixed on 2019-10-03, we're using 2019-10-02 as the end
point. If you don't provide an ending point it assumes that the end
point is today's nightly or your currently installed nightly. If you
don't provide a start point as we're doing it tries to find one by
regressing in time.  If you know if a failing starting point it would be
faster if you just provide that one.

By default it will run `cargo build` in the project and check whether or
not it fails. After finding a nightly that has regressed it is going to
automatically search for the commit that introduced the regression.

Let's see the tool in action:

The tool starts by downloading various nightly compilers, trying to find
a date when the program worked ...

```
checking nightly-2019-10-02
std for x86_64-unknown-linux-gnu: 172.87 MB / 172.87 MB [===============================================================================================================================================================] 100.00 % 10.67 MB/s uninstalling nightly-2019-10-02
checking nightly-2019-09-30
...
```

Once it has one failing and working point it starts bisecting ...
```
std for x86_64-unknown-linux-gnu: 173.43 MB / 173.43 MB [===============================================================================================================================================================] 100.00 % 12.82 MB/s uninstalling nightly-2019-09-29
tested nightly-2019-09-29, got No
searched toolchains nightly-2019-09-28 through nightly-2019-09-30
regression in nightly-2019-09-30
```

Once it finds a nightly, it starts to search the PRs that went into that
nightly build ...
```
looking for regression commit between 2019-09-30 and 2019-09-29
fetching commits from 488381ce9ef0ceabe83b73127c659e5d38137df0 to 8431f261dd160021b6af85916f161a13dd101ca0
...
searched toolchains 488381ce9ef0ceabe83b73127c659e5d38137df0 through 8431f261dd160021b6af85916f161a13dd101ca0
regression in 0bbab7d99dde8620604fb265706dc8bff20345a7
```

Finally, when it finds the PR that broke the compiler, it generates a
bug report that you can copy and paste!

````
==================================================================================
= Please open an issue on Rust's github repository                               =
= https://github.com/rust-lang/rust/issues/new                                   =
= Below you will find a text that would serve as a starting point of your report =
==================================================================================

# Regression found in the compiler

searched nightlies: from nightly-2019-09-28 to nightly-2019-09-30
regressed nightly: nightly-2019-09-30
searched commits: from https://github.com/rust-lang/rust/commit/488381ce9ef0ceabe83b73127c659e5d38137df0 to https://github.com/rust-lang/rust/commit/8431f261dd160021b6af85916f161a13dd101ca0
regressed commit: https://github.com/rust-lang/rust/commit/0bbab7d99dde8620604fb265706dc8bff20345a7
source code: URL OF A REPOSITORY THAT REPRODUCES THE ERROR

## Instructions

Please give the steps for how to build your repository (platform, system dependencies, etc.)
## Error

<details><summary>COLLAPSIBLE ERROR STACKTRACE</summary>
<p>

```bash
Paste the error the compiler is giving
```

</p></details>
````

This tells us that the regression started with
[`0bbab7d99dde8620604fb265706dc8bff20345a7`](https://github.com/rust-lang/rust/commit/0bbab7d99dde8620604fb265706dc8bff20345a7)
and you can look at the git log to find the PR. In this case is
[#64470](https://github.com/rust-lang/rust/pull/64470).

## Call for action: try the tool

Please, give this tool a try and if you find yourself updating your
application and it stops building, it's likely that you're hitting a
regression. As you can see at the end of the execution of the tool, if a
regression is found the tool gives you a report that you can paste on a
github issue on the [Rust repo](https://github.com/rust-lang/rust).

## Call for action: get involved in the development of cargo-bisect-rustc

There are also a lot of things to improve in the tool and a lot of bugs
to fix. There are a bunch of reported issues that are easy to fix,
[check them
out](https://github.com/rust-lang/cargo-bisect-rustc/issues).  You can
also, reach us out. You can find me and the rest of the compiler
contributors and members in [Zulip's #t-compiler/cargo-bisect-rustc
stream](https://rust-lang.zulipchat.com/#narrow/stream/217417-t-compiler.2Fcargo-bisect-rustc).
Sign up there if you haven't already and do not hesitate to ask
questions or even to send me a direct message if you don't know where to
start.
