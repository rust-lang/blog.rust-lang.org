+++
path = "2021/05/11/edition-2021"
title = "The Plan for the Rust 2021 Edition"
authors = ["Mara Bos"]
aliases = ["2021/05/11/edition-2021.html"]

[extra]
team = "The Rust 2021 Edition Working Group"
team_url = "https://www.rust-lang.org/governance/teams/core#project-edition-2021"
+++

We are happy to announce that the third edition of the Rust language, Rust 2021,
is scheduled for release in October.
Rust 2021 contains a number of small changes that are
nonetheless expected to make a significant improvement to how Rust feels in practice.

## What is an Edition?

The release of Rust 1.0 established
["stability without stagnation"](https://blog.rust-lang.org/2014/10/30/Stability.html)
as a core Rust deliverable.
Ever since the 1.0 release,
the rule for Rust has been that once a feature has been released on stable,
we are committed to supporting that feature for all future releases.

There are times, however, when it is useful to be able to make small changes
to the language that are not backwards compatible.
The most obvious example is introducing a new keyword,
which would invalidate variables with the same name.
For example, the first version of Rust did not have the `async` and `await` keywords.
Suddenly changing those words to keywords in a later version would've broken code like `let async = 1;`.

**Editions** are the mechanism we use to solve this problem.
When we want to release a feature that would otherwise be backwards incompatible,
we do so as part of a new Rust *edition*.
Editions are opt-in, and so existing crates do
not see these changes until they explicitly migrate over to the new edition.
This means that even the latest version of Rust will still *not* treat `async` as a keyword,
unless edition 2018 or later is chosen.
This choice is made *per crate* [as part of its `Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-edition-field).
New crates created by `cargo new` are always configured to use the latest stable edition.

### Editions do not split the ecosystem

The most important rule for editions is that crates in one edition can
interoperate seamlessly with crates compiled in other editions. This ensures
that the decision to migrate to a newer edition is a "private one" that the
crate can make without affecting others.

The requirement for crate interoperability implies some limits on the kinds of
changes that we can make in an edition.
In general, changes that occur in an edition tend to be "skin deep".
All Rust code, regardless of edition,
is ultimately compiled to the same internal representation within the compiler.

### Edition migration is easy and largely automated

Our goal is to make it easy for crates to upgrade to a new edition.
When we release a new edition,
we also provide [tooling to automate the migration](https://doc.rust-lang.org/cargo/commands/cargo-fix.html).
It makes minor changes to your code necessary to make it compatible with the new edition.
For example, when migrating to Rust 2018, it changes anything named `async` to use the equivalent
[raw identifier syntax](https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html): `r#async`.

The automated migrations are not necessarily perfect:
there might be some corner cases where manual changes are still required.
The tooling tries hard to avoid changes
to semantics that could affect the correctness or performance of the code.

In addition to tooling, we also maintain an Edition Migration Guide that covers
the changes that are part of an edition.
This guide will describe the change and give pointers to where people can learn more about it.
It will also cover any corner cases or details that people should be aware of.
The guide serves both as an overview of the edition,
but also as a quick troubleshooting reference
if people encounter problems with the automated tooling.

## What changes are planned for Rust 2021?

Over the last few months, the Rust 2021 Working Group has
gone through a number of proposals for what to include in the new edition.
We are happy to announce the final list of edition changes.
Each feature had to meet two criteria to make this list.
First, they had to be approved by the appropriate Rust team(s).
Second, their implementation had to be far enough along that we had
confidence that they would be completed in time for the planned milestones.

### Additions to the prelude

The [prelude of the standard library](https://doc.rust-lang.org/stable/std/prelude/index.html)
is the module containing everything that is automatically imported in every module.
It contains commonly used items such as `Option`, `Vec`, `drop`, and `Clone`.

The Rust compiler prioritizes any manually imported items over those
from the prelude, to make sure additions to the prelude will not break any existing code.
For example, if you have a crate or module called `example` containing a `pub struct Option;`,
then `use example::*;` will make `Option` unambiguously refer to the one from `example`;
not the one from the standard library.

However, adding a *trait* to the prelude can break existing code in a subtle way.
A call to `x.try_into()` using a `MyTryInto` trait might become ambiguous and
fail to compile if `std`'s `TryInto` is also imported,
since it provides a method with the same name.
This is the reason we haven't added `TryInto` to the prelude yet,
since there is a lot of code that would break this way.

As a solution, Rust 2021 will use a new prelude.
It's identical to the current one, except for three new additions:

- [`std::convert::TryInto`](https://doc.rust-lang.org/stable/std/convert/trait.TryInto.html)
- [`std::convert::TryFrom`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)
- [`std::iter::FromIterator`](https://doc.rust-lang.org/stable/std/iter/trait.FromIterator.html)

### Default Cargo feature resolver

Since Rust 1.51.0, Cargo has opt-in support for a [new feature resolver][4]
which can be activated with `resolver = "2"` in `Cargo.toml`.

Starting in Rust 2021, this will be the default.
That is, writing `edition = "2021"` in `Cargo.toml` will imply `resolver = "2"`.

The new feature resolver no longer merges all requested features for
crates that are depended on in multiple ways.
See [the announcement of Rust 1.51][5] for details.

[4]: https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2
[5]: https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html#cargos-new-feature-resolver

### IntoIterator for arrays

Until Rust 1.53, only *references* to arrays implement `IntoIterator`.
This means you can iterate over `&[1, 2, 3]` and `&mut [1, 2, 3]`,
but not over `[1, 2, 3]` directly.

```rust
for &e in &[1, 2, 3] {} // Ok :)

for e in [1, 2, 3] {} // Error :(
```

This has been [a long-standing issue][25], but the solution is not as simple as it seems.
Just [adding the trait implementation][20] would break existing code.
`array.into_iter()` already compiles today because that implicitly calls
`(&array).into_iter()` due to [how method call syntax works][22].
Adding the trait implementation would change the meaning.

Usually we categorize this type of breakage
(adding a trait implementation) 'minor' and acceptable.
But in this case there is too much code that would be broken by it.

It has been suggested many times to "only implement `IntoIterator` for arrays in Rust 2021".
However, this is simply not possible.
You can't have a trait implementation exist in one edition and not in another,
since editions can be mixed.

Instead, we decided to add the trait implementation in *all* editions (starting in Rust 1.53.0),
but add a small hack to avoid breakage until Rust 2021.
In Rust 2015 and 2018 code, the compiler will still resolve `array.into_iter()`
to `(&array).into_iter()` like before, as if the trait implementation does not exist.
This *only* applies to the `.into_iter()` method call syntax.
It does not affect any other syntax such as `for e in [1, 2, 3]`, `iter.zip([1, 2, 3])` or
`IntoIterator::into_iter([1, 2, 3])`.
Those will start to work in *all* editions.

While it's a shame that this required a small hack to avoid breakage,
we're very happy with how this solution keeps the difference between
the editions to an absolute minimum.
Since the hack is only present in the older editions,
there is no added complexity in the new edition.

[25]: https://github.com/rust-lang/rust/issues/25725
[20]: https://github.com/rust-lang/rust/pull/65819
[22]: https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator

### Disjoint capture in closures

[Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
automatically capture anything that you refer to from within their body.
For example, `|| a + 1` automatically captures a reference to `a` from the surrounding context.

Currently, this applies to whole structs, even when only using one field.
For example, `|| a.x + 1` captures a reference to `a` and not just `a.x`.
In some situations, this is a problem.
When a field of the struct is already borrowed (mutably) or moved out of,
the other fields can no longer be used in a closure,
since that would capture the whole struct, which is no longer available.

```rust
let a = SomeStruct::new();

drop(a.x); // Move out of one field of the struct

println!("{}", a.y); // Ok: Still use another field of the struct

let c = || println!("{}", a.y); // Error: Tries to capture all of `a`
c();
```

Starting in Rust 2021, closures will only capture the fields that they use.
So, the above example will compile fine in Rust 2021.

This new behavior is only activated in the new edition,
since it can change the order in which fields are dropped.
As for all edition changes, an automatic migration is available,
which will update your closures for which this matters.
It can insert `let _ = &a;` inside the closure to force the entire
struct to be captured as before.

### Panic macro consistency

The `panic!()` macro is one of Rust's most well known macros.
However, it has [some subtle surprises](https://github.com/rust-lang/rfcs/blob/master/text/3007-panic-plan.md)
that we can't just change due to backwards compatibility.

```rust
panic!("{}", 1); // Ok, panics with the message "1"
panic!("{}"); // Ok, panics with the message "{}"
```

The `panic!()` macro only uses string formatting when it's invoked with more than one argument.
When invoked with a single argument, it doesn't even look at that argument.

```rust
let a = "{";
println!(a); // Error: First argument must be a format string literal
panic!(a); // Ok: The panic macro doesn't care
```

(It even accepts non-strings such as `panic!(123)`, which is uncommon and rarely useful.)

This will especially be a problem once
[implicit format arguments](https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html)
are stabilized.
That feature will make `println!("hello {name}")` a short-hand for `println!("hello {}", name)`.
However, `panic!("hello {name}")` would not work as expected,
since `panic!()` doesn't process a single argument as format string.

To avoid that confusing situation, Rust 2021 features a more consistent `panic!()` macro.
The new `panic!()` macro will no longer accept arbitrary expressions as the only argument.
It will, just like `println!()`, always process the first argument as format string.
Since `panic!()` will no longer accept arbitrary payloads,
[`panic_any()`](https://doc.rust-lang.org/stable/std/panic/fn.panic_any.html)
will be the only way to panic with something other than a formatted string.

In addition, `core::panic!()` and `std::panic!()` will be identical in Rust 2021.
Currently, there are some historical differences between those two,
which can be noticeable when switching `#![no_std]` on or off.

### Reserving syntax

To make space for some new syntax in the future,
we've decided to reserve syntax for prefixed identifiers and literals:
`prefix#identifier`, `prefix"string"`, `prefix'c'`, and `prefix#123`,
where `prefix` can be any identifier.
(Except those that already have a meaning, such as `b'…'` and `r"…"`.)

This is a breaking change, since macros can currently accept `hello"world"`,
which they will see as two separate tokens: `hello` and `"world"`.
The (automatic) fix is simple though. Just insert a space: `hello "world"`.

<!--
The original plan was to reserve only `k#` and `f""` for future use,
but reserving *all* possible prefixes did not have many downsides.
It leaves more space for new syntax which would otherwise need to wait for another edition.
-->

Other than turning these into a tokenization error,
[the RFC][10] does not attach a meaning to any prefix yet.
Assigning meaning to specific prefixes is left to future proposals,
which will&mdash;thanks to reserving these prefixes now&mdash;not be breaking changes.

These are some new prefixes you might see in the future:

- `f""` as a short-hand for a format string.
  For example, `f"hello {name}"` as a short-hand for the equivalent `format_args!()` invocation.

- `c""` or `z""` for null-terminated C strings.

- `k#keyword` to allow writing keywords that don't exist yet in the current edition.
  For example, while `async` is not a keyword in edition 2015,
  this prefix would've allowed us to accept `k#async` as an alternative in edition 2015
  while we waited for edition 2018 to reserve `async` as a keyword.

[10]: https://github.com/rust-lang/rfcs/pull/3101

### Promoting two warnings to hard errors

Two existing lints are becoming hard errors in Rust 2021.
These lints will remain warnings in older editions.

* `bare-trait-objects`:
  The use of the `dyn` keyword to identify [trait objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
  will be mandatory in Rust 2021.

* `ellipsis-inclusive-range-patterns`:
  The [deprecated `...` syntax](https://doc.rust-lang.org/stable/reference/patterns.html#range-patterns)
  for inclusive range patterns is no longer accepted in Rust 2021.
  It has been superseded by `..=`, which is consistent with expressions.

### Or patterns in macro_rules

Starting in Rust 1.53.0, [patterns](https://doc.rust-lang.org/stable/reference/patterns.html)
are extended to support `|` nested anywhere in the pattern.
This enables you to write `Some(1 | 2)` instead of `Some(1) | Some(2)`.
Since this was simply not allowed before, this is not a breaking change.

However, this change also affects [`macro_rules` macros](https://doc.rust-lang.org/stable/reference/macros-by-example.html).
Such macros can accept patterns using the `:pat` fragment specifier.
Currently, `:pat` does *not* match `|`, since before Rust 1.53,
not all patterns (at all nested levels) could contain a `|`.
Macros that accept patterns like `A | B`,
such as [`matches!()`](https://doc.rust-lang.org/1.51.0/std/macro.matches.html)
use something like `$($_:pat)|+`.
Because we don't want to break any existing macros,
we did *not* change the meaning of `:pat` in Rust 1.53.0 to include `|`.

Instead, we will make that change as part of Rust 2021.
In the new edition, the `:pat` fragment specifier *will* match `A | B`.

Since there are times that one still wishes to match a single pattern
variant without `|`, the fragment specified `:pat_param` has been added
to retain the older behavior.
The name refers to its main use case: a pattern in a closure parameter.

## What comes next?

Our plan is to have these changes merged and fully tested by September,
to make sure the 2021 edition makes it into Rust 1.56.0.
Rust 1.56.0 will then be in beta for six weeks,
after which it is released as stable on October 21st.

However, note that Rust is a project run by volunteers.
We prioritize the personal well-being of everyone working on Rust
over any deadlines and expectations we might have set.
This could mean delaying the edition a version if necessary,
or dropping a feature that turns out to be too difficult or stressful to finish in time.

That said, we are on schedule and many of the difficult problems are already tackled,
thanks to all the people contributing to Rust 2021! 💛

---

You can expect another announcement about the new edition in July.
At that point we expect all changes and automatic migrations to be implemented
and ready for public testing.

We'll be posting some more details about the process and rejected proposals on
the "Inside Rust" blog soon. (_Correction: This did not end up happening due
to lack of bandwidth_)

<!--
If you really can't wait, many features are already available on
Rust [Nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html)
with `-Zunstable-options --edition=2021`.
-->
