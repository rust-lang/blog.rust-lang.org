+++
layout = "post"
date = 2020-03-04
title = "Recent and future pattern matching improvements"
author = 'Mazdak "Centril" Farrokhzad'
description = "Reviewing recent pattern matching improvements"
team = "the language team <https://www.rust-lang.org/governance/teams/lang>"
+++

[ch_6]: https://doc.rust-lang.org/book/ch06-00-enums.html
[ch_18]: https://doc.rust-lang.org/book/ch18-00-patterns.html
[ref_match]: https://doc.rust-lang.org/reference/expressions/match-expr.html
[ref_pat]: https://doc.rust-lang.org/reference/patterns.html
[ref_place]: https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions

Much of writing software revolves around checking if some data has some shape ("pattern"), extracting information from it, and then reacting if there was a match. To facilitate this, many modern languages, Rust included, support what is known as "pattern matching".

> If you are new to Rust or want to refresh your knowledge, you may first want to read chapters [6, Enums and Pattern Matching][ch_6] and [18, Patterns and Matching][ch_18] in the book, or read more about [`match` expressions][ref_match] and [patterns][ref_pat] in the reference.

Pattern matching in Rust works by checking if a [*place*][ref_place] in memory (the "data") matches a certain *pattern*. In this post, we will look at some recent improvements to patterns soon available in stable Rust as well as some more already available in nightly.

If you are familiar with the nightly features discussed and would like to help out with the efforts to drive them to stable, jump ahead to [*How can I help?](#how-can-i-help?).

## Subslice patterns, `[head, tail @ ..]`

[fixed_slice]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#basic-slice-patterns
[recover_attrs_no_item]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_parse/parser/struct.Parser.html#method.recover_attrs_no_item
[pr_subslice]: https://github.com/rust-lang/rust/pull/67712

Lists are one of the most basic and common data structures found in software. In Rust, lists are usually a contiguous sequence of elements in memory, or a *slice*.

Since slices are so commonplace, it is important that working with them is easy. To that end, we stabilized [*fixed-length slice patterns* in Rust 1.26.0][fixed_slice]. So now it is possible to e.g., write `let [a, b, c] = my_array;` to destructure an array of 3 elements. Oftentimes, however, we're working with a slice of unknown length, so given only fixed-length slice patterns, we have to provide a fallback `match` arm with e.g. `_` as the pattern.

In Rust 1.42.0, [we are stabilizing *subslice patterns*][pr_subslice]. To introduce a subslice pattern, we use `..` which denotes a variable-length gap, matching as many elements as possible not matched by the patterns before and after the `..`. For example, in a parser, we would like to error when a list of attributes, `attrs`, is not followed by an item, [so we write][recover_attrs_no_item]:

```rust
/// Recover if we parsed attributes and expected an item but there was none.
fn recover_attrs_no_item(&mut self, attrs: &[Attribute]) -> PResult<'a, ()> {
    let (start, end) = match attrs {
        [] => return Ok(()),
        [x0] => (x0, x0),
        [x0, .., xn] => (x0, xn),
    };
    let msg = if end.is_doc_comment() {
        "expected item after doc comment"
    } else {
        "expected item after attributes"
    };
    let mut err = self.struct_span_err(end.span, msg);
    if end.is_doc_comment() {
        err.span_label(end.span, "this doc comment doesn't document anything");
    }
    if let [.., penultimate, _] = attrs {
        err.span_label(start.span.to(penultimate.span), "other attributes here");
    }
    Err(err)
}
```

Here we have two subslice patterns, the first one being `[x0, .., xn]`. In this case, the pattern binds `x0`, the first element, and `xn`, the last element, and ignores everything in the middle, matching a slice with at least two elements in total. Meanwhile, `[]` and `[x0]` match cases with fewer than two elements, so the compiler knows that we have covered all possibilities. In the latter case, we extract the `penultimate` element of the slice, which, as the name suggests, also requires that the slice has at least two elements.

We can also bind a subslice to a variable. For example, suppose we want to disallow `...` in all but the last parameter of a function. If so, we can write:

```rust
match &*fn_decl.inputs {
    ... // other arms
    [ps @ .., _] => {
        for Param { ty, span, .. } in ps {
            if let TyKind::CVarArgs = ty.kind {
                self.err_handler().span_err(
                    *span,
                    "`...` must be the last argument of a C-variadic function",
                );
            }
        }
    }
}
```

Here, `ps @ ..` will bind the initial elements of the slice to `ps` and ignore the last element.

After more than 7 years of baking in nightly, with many twists and turns, subslice patterns will finally be stable. To get here, we've had to redesign the feature, plug soundness holes in the borrow checker, and substantially refactor the exhaustiveness checker. For more on how we got here, [read the stabilization report][pr_subslice], [Thomas Hartmann's blog post][thomas_subslice], and stay tuned for the 1.42.0 release announcement  on the 12th of March.

[thomas_subslice]: https://thomashartmann.dev/blog/feature(slice_patterns)/

## Nested OR-patterns

[tracking_or_pats]: https://github.com/rust-lang/rust/issues/54883

When pattern matching on an `enum`, the logic for some of the variants may be exactly the same. To avoid repeating ourselves, the `|` separator in `match`, `if let`, or `while let` expressions can be used to say that the branch should be taken if any of the `|`-separated patterns match. For example, we may write:

```rust
// Any local node that may call something in its body block should be explored.
fn should_explore(tcx: TyCtxt<'_>, hir_id: hir::HirId) -> bool {
    match tcx.hir().find(hir_id) {
        Some(Node::Item(..))
        | Some(Node::ImplItem(..))
        | Some(Node::ForeignItem(..))
        | Some(Node::TraitItem(..))
        | Some(Node::Variant(..))
        | Some(Node::AnonConst(..))
        | Some(Node::Pat(..)) => true,
        _ => false,
    }
}
```

This is serviceable, but `Some(_)` is still repeated several times. With [`#![feature(or_patterns)]`][tracking_or_pats], which recently became usable on nightly, this repetition can be avoided:

```rust
// Any local node that may call something in its body block should be explored.
fn should_explore(tcx: TyCtxt<'_>, hir_id: hir::HirId) -> bool {
    match tcx.hir().find(hir_id) {
        Some(
            Node::Item(..)
            | Node::ImplItem(..)
            | Node::ForeignItem(..)
            | Node::TraitItem(..)
            | Node::Variant(..)
            | Node::AnonConst(..)
            | Node::Pat(..),
        ) => true,
        _ => false,
    }
}
```

Previously, when using `|` in a `match` expression, the `|` syntax was part of `match` itself. With `or_patterns`, this is now part of patterns themselves, so you can nest OR-patterns arbitrarily, and use them in `let` statements too:

```rust
let Ok(x) | Err(x) = foo();
```

An OR-pattern covers the *union* of all the `|`-ed ("or-ed") patterns. To ensure that whatever alternative matched, all bindings are consistent and initialized, each or-ed pattern must include the exact same set of bindings, with the same types, and the same binding modes.

## Bindings after `@`

[#16053]: https://github.com/rust-lang/rust/pull/16053
[MIR]: https://rustc-dev-guide.rust-lang.org/mir/index.html
[rip_ast_borrowck]: https://github.com/rust-lang/rust/pull/64790
[tracking_at]: https://github.com/rust-lang/rust/issues/65490

When matching on a certain substructure, you sometimes want to hold on to the whole. For example, given `Some(Expr { .. })`, you would like to bind the outer `Some(_)` layer. In Rust, this can be done using e.g., `expr @ Some(Expr { .. })`, which binds the matched place to `expr` while also ensuring that it matches `Some(Expr { .. })`.

Suppose also that `Expr` has a field `span` that you would also use. In ancient times, that is before Rust 1.0, this was possible, but today, it results in an error:

```rust
error[E0303]: pattern bindings are not allowed after an `@`
 --> src/lib.rs:L:C
  |
L |         bar @ Some(Expr { span }) => {}
  |                           ^^^^ not allowed after `@`
```

This was turned into an error in [#16053], mainly due to the difficulties of encoding borrow checking rules in a sound way in the old AST based borrow checker.

Since then, we have [removed the old borrow checker][rip_ast_borrowck] in favor of one based on [MIR], which is a simpler, and more appropriate data structure for borrow checking. Specifically, in the case of a statement like `let ref x @ ref y = a;`, we would get roughly the same MIR as if we had used `let x = &a; let y = &a;`.

So now that having bindings to the right of `@` is handled uniformly and correctly by the borrow checker (e.g., the compiler won't allow `ref x @ ref mut y`), we have decided to allow them under [`#![feature(bindings_after_at)]`][tracking_at], now available on nightly. With the feature gate enabled, you may for example write:

```rust
#![feature(bindings_after_at)]

fn main() {
    if let x @ Some(y) = Some(0) {
        dbg!(x, y);
    }
}
```

Our hope is that with providing this feature, we remove one surprising corner of the language.

## Combining by-move and by-`ref` bindings

[tracking_move_ref]: https://github.com/rust-lang/rust/issues/68354

For similar reasons as noted in the case of bindings after `@`, Rust does not currently allow you to combine normal by-move bindings with those that are by-`ref`. For example, should you write...:

```rust
fn main() {
    let tup = ("foo".to_string(), 0);
    let (x, ref y) = tup;
}
```

... you would get an error:

```rust
error[E0009]: cannot bind by-move and by-ref in the same pattern
 --> src/main.rs:3:10
  |
3 |     let (x, ref y) = tup;
  |          ^  ----- by-ref pattern here
  |          |
  |          by-move pattern here
```

At the same time, however, the compiler is perfectly happy to allow...:

```rust
fn main() {
    let tup = ("foo".to_string(), 0);
    let x = tup.0;
    let ref y = tup.1;
}
```

... even though there is no semantic difference between these programs.

Now that we have moved to the new borrow checker, as outlined in the previous section, we have relaxed this restriction on nightly as well, so under [`#![feature(move_ref_pattern)]`][tracking_move_ref], you may write:

```rust
#![feature(move_ref_pattern)]

fn main() {
    let tup = ("foo".to_string(), 0);
    let (x, ref y) = tup;
}
```

## How can I help?

[F-or_patterns]: https://github.com/rust-lang/rust/labels/F-or_patterns

To recap, we have three unstable features, all improving pattern matching in different ways:

- [`#![feature(or_patterns)]`][tracking_or_pats], which allows you to arbitrarily nest or-patterns e.g. `Some(Foo | Bar)`
- [`#![feature(bindings_after_at)]`][tracking_at], which allows e.g., `ref x @ Some(ref y)` 
- [`#![feature(move_ref_pattern)]`][tracking_move_ref], which allows e.g., `(x, ref y)` where `x` is by-move and `y` is by-reference

To help us transition these features over to stable Rust, we need your help to ensure that they meet the expected quality standards. To help out, consider:

- Using the features in your code where applicable, if a nightly compiler is something you are OK with, and reporting any bugs, problems, deficiencies in diagnostics, etc. as issues.
- Looking through the reported issues under the feature gate labels (e.g., [`F-or_patterns`][F-or_patterns]) and seeing if you can help out with any of them.
    - In particular, if you can help out with writing tests, that is appreciated.

Thanks for reading, and happy pattern matching!
