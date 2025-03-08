+++
layout = "post"
title = "2024 Edition Update"
author = "Eric Huss"
team = "Edition 2024 Project Group <https://github.com/rust-lang/team/blob/15e99829ee2124b07f740b8befd41c55a46fee91/teams/project-edition-2024.toml>"
+++

This is a reminder to the teams working on the 2024 Edition that implementation work should be **finished by the end of May**. If you have any questions, please let us know on the [`#edition`][zulip] Zulip stream.

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/268952-edition

## What we are tracking for the Edition

The following is a list of changes we are tracking for inclusion in the Edition. This list is not final, and may change in the future.

- Change the [`unsafe_op_in_unsafe_fn`] lint to be warn-by-default. This is implemented, see [docs][docs-unsafe].
- Remove `RustcEncodable` & `RustcDecodable` from the 2024 prelude. This is waiting for the implementation to be approved, and for automatic migration. See [#116016](https://github.com/rust-lang/rust/pull/116016).
- Include `Future` and `IntoFuture` in the 2024 prelude. This is implemented, and waiting for automatic migration support. See [docs][docs-future] and [#121042](https://github.com/rust-lang/rust/issues/121042#issuecomment-1942181209).
- Reserve the `gen` keyword. This is waiting for final approval, and the implementation review. See [RFC #3513](https://github.com/rust-lang/rfcs/pull/3513).
- RPIT lifetime capture. This is partially implemented, but is waiting on final design for precise capturing. See [RFC #3498](https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html) and [#117587](https://github.com/rust-lang/rust/issues/117587).
- Macro matcher fragment specifier policy. This is a policy change and does not have an implementation. See [RFC #3531](https://rust-lang.github.io/rfcs/3531-macro-fragment-policy.html).
- Disallow references to `static mut`. This is implemented, though there is uncertainty about how migration should work, how to communicate to users how to update their code, and whether or not this should cover hidden references. See [docs][docs-static-mut] and [#114447](https://github.com/rust-lang/rust/issues/114447).
- New range types. This is waiting on the design considerations for migration. See [RFC #3550](https://github.com/rust-lang/rfcs/pull/3550).
- Cargo: Remove implicit features. Development of this is underway. See [RFC #3491](https://rust-lang.github.io/rfcs/3491-remove-implicit-features.html).
- Public/private dependencies. This is partially implemented, though there are some changes needed on the rules for determining visibility. See [RFC #3516](https://rust-lang.github.io/rfcs/3516-public-private-dependencies.html).
- Rustfmt: Enable `overflow_delimited_expr` by default. This is not yet implemented. See [#114764](https://github.com/rust-lang/rust/pull/114764).

[docs-static-mut]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
[docs-future]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/prelude.html
[docs-unsafe]: https://doc.rust-lang.org/nightly/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html
[`unsafe_op_in_unsafe_fn`]: https://doc.rust-lang.org/nightly/rustc/lints/listing/allowed-by-default.html#unsafe-op-in-unsafe-fn

## What is an Edition?

Editions are a mechanism to introduce changes that would otherwise be backwards incompatible while still retaining compatibility with older releases. Editions are opt-in and designed to allow projects on different Editions to remain compatible with one another. More information may be found in the [Edition Guide].

[Edition Guide]: https://doc.rust-lang.org/nightly/edition-guide/editions/index.html
