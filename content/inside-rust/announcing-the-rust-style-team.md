+++
path = "inside-rust/2022/09/29/announcing-the-rust-style-team"
title = "Announcing the Rust Style Team"
authors = ["Josh Triplett"]
aliases = ["inside-rust/2022/09/29/announcing-the-rust-style-team.html"]

[extra]
team = "The Rust Style Team"
team_url = "https://www.rust-lang.org/governance/teams/lang#Style team"
+++

Rust has a standardized style, and an implementation of that style in the
`rustfmt` tool. The standardized style helps Rust developers feel comfortable
and at home in many different projects, and the tooling support from `rustfmt`
makes it easy to maintain and to incorporate in continuous integration.
`rustfmt` also provides many options to customize the style, but the style
guide defines the defaults, and most projects use those defaults.

The standard Rust style resulted from development and discussion within the
Rust style team, between 2016 and 2018. After publishing the style guide, the
Rust style team concluded its active work, by design.

However, as the Rust language develops, we have a regular need for improvements
to the style guide, such as to support new language constructs. This includes
minor language changes, as well as highly anticipated new features such as
`let`-chaining (RFC 2497) and `let`-`else` (RFC 3137). New constructs like
these, by default, get ignored and not formatted by rustfmt, and subsequently
need formatting added. Some of this work has fallen to the rustfmt team in
recent years, but the rustfmt team would prefer to implement style
determinations made by another team rather than making such determinations
itself.

In addition, rustfmt maintains backwards compatibility guarantees: code that
has been correctly formatted with rustfmt won't get formatted differently with
a future version of rustfmt. This avoids churn, and avoids creating CI failures
when people use rustfmt to check style in CI. However, this also prevents
evolving the Rust style to take community desires into account and improve
formatting over time. rustfmt provides various configuration options to change
its default formatting, and many of those options represent changes that many
people in the community would like enabled by default.

For instance, many people prefer to format their `use` lines in three blocks:
imports from the standard library, imports from external crates, and then
imports from modules within the same project. `rustfmt` supports this via the
option `group_imports = StdExternalCrate`, but cannot make this the default
without causing CI failures in existing projects. We need a way to evolve the
default Rust style compatibly, similar in spirit to the mechanisms we use for
Rust editions: allowing existing style to continue working, and allowing people
to opt into new style.

To solve both of these problems, [RFC
3309](https://rust-lang.github.io/rfcs/3309-style-team.html) has revived the
Rust style team, with three goals:

- Making determinations about styling for new Rust constructs
- Evolving the existing Rust style
- Defining mechanisms to evolve the Rust style while taking backwards
  compatibility into account

We don't plan to make any earth-shattering style changes; the look and feel of
Rust will remain largely the same. Evolutions to the default Rust style will
largely consist of established `rustfmt` options people already widely enable,
or would enable if they were stable.

We expect that the initial work of the style team will focus on clearing a
backlog of new language constructs that lack formatting guidance. Afterwards,
we will look towards defining and implementing the mechanisms to evolve the
default Rust style, and then begin introducing style improvements.
