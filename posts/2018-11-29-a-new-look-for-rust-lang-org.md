---
layout: post
title: "A new look for rust-lang.org"
author: The Rust Core Team
---

Before 1.0, Rust had a reputation for changing the language on a near-daily
basis. By contrast, the website has looked pretty much the same. Here’s the
first version of rust-lang.org, seven years ago (courtesy of [the WayBack
Machine](https://web.archive.org/)):

![rust website in 2011](/images/rust-www1.png)

In 2014, three years later:

![rust website in 2014](/images/rust-www2.png)

If you visit <https://rust-lang.org> today, you'll see this:

![rust website in 2018](/images/rust-www3.png)

Over time, we’ve grown to love it. It’s simple. Minimal. Familiar.

## Improving the content

But we can always do better. For example, the website suffers from what we
call “the fireflower problem.” First formulated by [Kathy
Sierra](http://seriouspony.com/), and made into an image by Samuel Hulick:

![the fireflower](/images/fireflower.png)

We want Mario to use Rust, the fireflower, and turn into the ever-awesome
Fire Mario. But there’s a corollary here: it’s better to say “we will make
you into Fire Mario” than it is “we sell fire flowers.”

(As an aside, we had a [community discussion on this
topic](https://brson.github.io/fireflowers/) back in 2016.)

In other words, this list:

- zero-cost abstractions
- move semantics
- guaranteed memory safety
- threads without data races
- trait-based generics
- pattern matching
- type inference
- minimal runtime
- efficient C bindings

doesn’t explain what you can *do* with Rust, which leads people to say “Rust
seems neat, but I don’t know what I would actually use it for.”

## Improving the style

We also like the minimalist style of the current site, but it also may be
*too* minimal. Furthermore, it has no room to grow; we have more than just
rust-lang.org these days. We wanted a style that we could use to unify all of
the websites that we maintain in the Rust project; crates.io being a big one.
Its “pool table” design feels extremely different than rust-lang.org, which
is confusing.

Doing this requires care, as we don’t want to make the website huge and
complicated, but at the same time, using more than black and blue might be
nice.

## The beta

Today, we’d like to announce a beta of the new rust-lang.org. If you go to
<https://beta.rust-lang.org>, you’ll see this:

![beta rust website](/images/rust-www4.png)

Its fresh visual design gives us a lot more flexibility in how we get
information across. It retains the minimalist spirit of the old site, while
adding some bold color and visual variety.

We hope you like it as much as we do!

### Some highlights

The new site puts the “why Rust?” question front-and-center, and includes
dedicated pages for the four application domains we targeted in 2018:

- Embedded devices
- WebAssembly
- CLI apps
- Network services

We have also revised the slogan. Historically, it has been:

> Rust is a systems programming language that runs blazingly fast, prevents
> segfaults, and guarantees thread safety.

Like the bullet list of features, this doesn't convey what you can *do* with
Rust. So we've updated the slogan:

> Rust: The programming language that empowers everyone to become a systems
> programmer.

We're still not sure we love the term "systems programming," as it seems like
it means something different to everyone, but this iteration is significantly
better than the old one. Even if people have different ideas about what
"systems programming" means, they at least have some idea. "guarantees thread
safety," not so much.

## Future work

There’s still more work to do:

- Some information on the old site, has not yet been ported over.
- Translations have regressed. We’re working on adding the proper
  infrastructure here, and hope to be able to start accepting translations by
  the end of the year.
- We need more polish and testing in a general sense.

Please [file an
issue](https://github.com/rust-lang/beta.rust-lang.org/issues/new/choose) with any
feedback you have! We’re also looking for people with abilities of all kinds
to help maintain the site, and especially people with design, CSS, and
marketing skills. If you’d like to get involved, please [email
us](mailto:www@rust-lang.org)!

We’d like to ship this new site on December 6, with the release of Rust 2018.
Thank you for giving it a try before then, so we can work out any bugs we
find!