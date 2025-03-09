+++
layout = "post"
date = 2021-01-15
title = "Rustdoc performance improvements"
author = "Jynn Nelson and Guillaume Gomez"
team = "The Rustdoc Team <https://www.rust-lang.org/governance/teams/dev-tools#rustdoc>"
+++

Hi everyone! [**@GuillaumeGomez**] recently tweeted about the rustdoc performance improvements and suggested that we write a blog post about it:

<blockquote class="twitter-tweet"><p lang="en" dir="ltr">The performance comparison between <a href="https://twitter.com/rustlang?ref_src=twsrc%5Etfw">@rustlang</a> rustdoc now and rustdoc 4 months ago are crazy! The rustdoc cleanup going on (we&#39;re still far from done!) is having a huge positive impact! Can&#39;t wait to see how much better the results will be.<br><br>Maybe I should write a blog post? <a href="https://t.co/XapdmdZ1IZ">pic.twitter.com/XapdmdZ1IZ</a></p>&mdash; Guillaume Gomez (@imperioworld_) <a href="https://twitter.com/imperioworld_/status/1349383125051305984?ref_src=twsrc%5Etfw">January 13, 2021</a></blockquote> <script async src="https://platform.twitter.com/widgets.js" charset="utf-8"></script> 

The tweet received a lot of comments approving the blog post idea so here we go!

## Performance changes

There were actually only two PRs explicitly meant to improve the performance of rustdoc:

1. Rustdoc: Cache resolved links [#77700](https://github.com/rust-lang/rust/pull/77700)

   This does what it says in the title. In particular, this sped up the time to generate intra-doc
   links for `stm32h7xx` by a whopping [90,000%]. [**@bugadani**](https://github.com/bugadani) did an
   excellent job on this, congratulations!

[90,000%]: https://github.com/rust-lang/rust/pull/77700#issuecomment-735995025

2. Don't look for blanket impls in intra-doc links [#79682](https://github.com/rust-lang/rust/pull/79682)

   This PR was very disappointing to write. The gist is that if you had

   ```rust
   trait Trait {
       fn f() {}
   }

   impl<T> Trait for T {}
   ```

   then linking to `usize::f` would not only not work, but would take longer to run than the rest of
   intra-doc links to run. This temporarily disabled blanket impls until the bug is fixed and the performance can be improved, for a similar [90x] speedup on `stm32h7xx`.

   You may be wondering why `stm32h7xx` was so slow before; see the end of the post for details.

[90x]: https://github.com/rust-lang/rust/pull/79682#issuecomment-738505531

## It's all about cleanup

With the recent growth of the rustdoc team, we finally had some time to pay down the technical debt we've been accumulating for a while. To sum it up: removing implementations in rustdoc and using the compiler types directly. First, we need to explain a bit about how rustdoc works. When we run it to generate HTML documentation, it goes through several steps:

* Run parts of the compiler to get the information we need.
* Remove the information provided by the compiler that we don't need (for example, if an item is `doc(hidden)`, we don't need it). There is a lot to say on this part so maybe we'll write another blog post to go more into details.
* The `doctree` pass which adds some extra information needed by rustdoc on some items of the compiler.
* The `clean` pass which converts the compiler types into rustdoc ones: basically, it transforms everything into "printable" content.
* The `render` pass which then generates the desired output (HTML or, on nightly, JSON).

[**@jyn514**] noticed a while ago that [most of the work in Rustdoc is duplicated](https://github.com/rust-lang/rust/issues/76382): there are actually *three* different abstract syntax trees (ASTs)! One for `doctree`, one for `clean`, and one is the original [HIR](https://rustc-dev-guide.rust-lang.org/hir.html) used by the compiler.
Rustdoc was spending quite a lot of time converting between them. Most of the speed improvements have come from getting rid of parts of the AST altogether.

### Pruning the tree

Most of the work `doctree` did was 100% unnecessary. All the information it had was already present in the [HIR], and recursively walking the crate and building up new types took quite a while to run.

[**@jyn514**]'s first stab at this was to [get rid of the pass altogether](https://github.com/rust-lang/rust/pull/78082). This went... badly. It turns out it did some useful work after all.

That said, there was a bunch of unnecessary work it didn't need to do, which was to add its own types for everything. If you look at [the types from 3 months ago](https://github.com/rust-lang/rust/blob/31d275e5877d983fecb39bbaad837f6b7cf120d3/src/librustdoc/doctree.rs) against [the types from today](https://github.com/rust-lang/rust/blob/a4cbb44ae2c80545db957763b502dc7f6ea22085/src/librustdoc/doctree.rs), the difference is really startling! It went from 300 lines of code replicating almost every type in the compiler to only 75 lines and 6 types.

## Cleaning the `clean` pass

The first and most important part of this cleanup was a PR called 'Add `Item::from_def_id_and_kind` to reduce duplication in rustdoc' ([#77820]). Before that change, every [`Item`] in rustdoc was constructed in dozens of different places - for structs, for enums, for traits, the list went on and on. This made it very hard to make changes to the [`Item`] struct, because any change would break dozens of callsites, each of which had to be fixed individually. What [#77820] did was to construct all those items in the same place, which made it far easier to change how `Item` was represented internally.

Along the way, [**@jyn514**] found several cleanups that were necessary in the compiler first:

- Calculate visibilities once in resolve [#78077](https://github.com/rust-lang/rust/pull/78077). Thanks to [**@petrochenkov**](https://github.com/petrochenkov) for tackling this!
- Fix handling of item names for HIR [#78345](https://github.com/rust-lang/rust/pull/78345)

### Deleting parts of `Item`

Once that was done, we were able to get rid of large parts of the [`Item`] type by calculating the information on-demand instead, using the compiler internals. This had two benefits:

1. Less memory usage, because the information wasn't stored longer than it was needed.
2. Less time overall, because not every item needed all the information available.

This benefited quite a lot from the [query system](https://rustc-dev-guide.rust-lang.org/query.html), which I highly encourage reading about.

Here are some example changes that calculate information on demand:

* Don't unnecessarily override attrs for Module [#80340](https://github.com/rust-lang/rust/pull/80340)
* Get rid of `clean::Deprecation` [#80041](https://github.com/rust-lang/rust/pull/80041)
* Get rid of `clean::{Method, TyMethod}` [#79125](https://github.com/rust-lang/rust/pull/79125)
* Remove duplicate `Trait::auto` field [#79126](https://github.com/rust-lang/rust/pull/79126)
* Get rid of some doctree items [#79264](https://github.com/rust-lang/rust/pull/79264)
* Get rid of `doctree::{ExternalCrate, ForeignItem, Trait, Function}` [#79335](https://github.com/rust-lang/rust/pull/79335)
* Get rid of `doctree::Impl` [79312](https://github.com/rust-lang/rust/pull/79312)
* Remove `doctree::Macro` and distinguish between `macro_rules!` and `pub macro` [#79455](https://github.com/rust-lang/rust/pull/79455)
* Pass around Symbols instead of Idents in doctree [#79623](https://github.com/rust-lang/rust/pull/79623)

As you can see, all these changes not only sped up rustdoc, but discovered bugs and duplication that had been around for years.

### Reusing compiler types

And some examples of using the existing compiler types without adding our own:

* \[rustdoc\] Switch to Symbol for `item.name` [#80044](https://github.com/rust-lang/rust/pull/80044)
* Use more symbols in rustdoc [#80047](https://github.com/rust-lang/rust/pull/80047)
* Replace String with Symbol where possible [#80091](https://github.com/rust-lang/rust/pull/80091)
* Continue String to Symbol conversion in rustdoc (1) [#80119](https://github.com/rust-lang/rust/pull/80119)
* Continue String to Symbol conversion in rustdoc (2) [#80154](https://github.com/rust-lang/rust/pull/80154)
* Get rid of custom pretty-printing in rustdoc [#80799](https://github.com/rust-lang/rust/pull/80799)

They replace `String` used for items' name to use [`Symbol`] instead. Symbols are interned strings, so we're not only preventing unnecessary conversions but also greatly improving memory usage. You can read more about Symbols in the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/appendix/glossary.html?highlight=symbol#glossary).

The interesting part is that it also allowed some [small improvements](https://github.com/rust-lang/rust/pull/80750) in the compiler itself.

With the same logic came [#80261](https://github.com/rust-lang/rust/pull/80261) (which required [#80295](https://github.com/rust-lang/rust/pull/80295) beforehand) which kept the original document attributes [`Symbol`] with the "transformation information" instead of the transformed string. If you want to know more about how rustdoc works on doc comments formatting, [**@GuillaumeGomez**] wrote a blog post about it [here](https://blog.guillaume-gomez.fr/articles/2020-11-11+New+doc+comment+handling+in+rustdoc). The idea here is once again to compute this "on demand" instead of storing the results ahead for (potential) usage.

## Why did we not rely more on rustc internals earlier?

By now, you may be wondering why rustdoc didn't rely more on rustc internals before this cleanup. The answer is actually simple: rustdoc is **old**. When it was being written, rustc internals changed very frequently (even daily), making it very complicated for the rustdoc maintainers to keep up. To allow them to work without worrying too much about these changes, they decided to abstract the compiler internals so that they could then work with those rustdoc types without having breaking changes to worry about every day.

Since then, things got improved, the 1.0 version of Rust finally got released and things slowed down. Then, focus was mostly on adding new features to make rustdoc as great as possible. With the arrival of new rustdoc team members, we were finally able to get back on this aspect. It didn't make much sense to keep all those abstractions because the internals are somewhat stable now and we can all see the results. :)

## Next Steps

As you saw from the displayed benchmarks, the results were strongly positive. However, we're still far from done. As we speak, we continue to simplify and rework a lot of the rustdoc source code.

### Remove doctree altogether

This is the "useful work" (as opposed to unnecessary complexity) that `doctree` does today:

- Detecting which items are publicly reachable. Ideally, this would just use compiler APIs, but those APIs [are broken](https://github.com/rust-lang/rust/issues/64762).
- Inlining items that are only reachable from an export. 'Inlining' is showing the full documentation for an item at a re-export (`pub use std::process::Command`) instead of just showing the `use` statement. It's used pervasively by the standard library and facade crates like `futures` to show the relevant documentation in one place, instead of spread out across many crates. **@jyn514** hopes this could be done in `clean` instead, but has no idea yet how to do it.
- Moving macros from always being at the root of the crate to the module where they're accessible. For example, this macro:

  ```rust
  #![crate_name="my_crate"]
  #![feature(decl_macro)]
  mod inner {
      pub macro m() {}
  }
  ```

  should be documented at `my_crate::inner::m`, but the compiler shows it at `my_crate::m` instead. The fix for this is an awful hack that goes through every module Rustdoc knows about to see if the name of the module matches the name of the macro's parent module. At some point in the future, it would be great to fix the compiler APIs so this is no longer necessary.

  Giant thank you to [**@danielhenrymantilla**](https://github.com/danielhenrymantilla) both for writing up the fix, and discovering and fixing several other macro-related bugs along the way!

If all these issues could be fixed, that would be an even bigger speedup - there would be no need to walk the tree in the first place!

### Continue to shrink `clean::Item`

Most of the existing cleanups have been focused on calculating info on-demand that's used for *every* item in rustdoc, since that has the greatest impact. There are still lots of other parts that are calculated ahead of time, though: in particular [`ItemKind`](https://doc.rust-lang.org/nightly/nightly-rustc/rustdoc/clean/types/enum.ItemKind.html) goes completely through `clean` before starting to render the documentation.

### Speed up `collect_blanket_impls`

One of the slowest functions in all of rustdoc is a function called
[`get_auto_trait_and_blanket_impls`](https://doc.rust-lang.org/nightly/nightly-rustc/rustdoc/clean/utils/fn.get_auto_trait_and_blanket_impls.html).
On crates with many blanket implementation, such as `stm32`-generated crates, this can take
[almost half of the *total*
time](https://github.com/rust-lang/rust/issues/79103#issuecomment-745732064) rustdoc spends on
the crate.

We are not sure yet how to speed this up, but there is definitely lots of room for improvement.
If you're interested in working on this, please reach out [on Zulip].

[on Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/247081-t-compiler.2Fperformance/topic/rustdoc.20calls.20.60for_each_relevant_impl.60.20a.20lot

Overall, rustdoc is making rapid progress in performance, but there is still a lot more work to be done.

## Errata

An earlier version of the blog post described the section on slimming `doctree` as "Burning down
the tree". The name was changed to be more environmentally friendly.

[**@jyn514**]: https://github.com/jyn514
[**@GuillaumeGomez**]: https://github.com/GuillaumeGomez
[HIR]: https://rustc-dev-guide.rust-lang.org/hir.html
[`Symbol`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustc_span/symbol/struct.Symbol.html
[`Item`]: https://doc.rust-lang.org/nightly/nightly-rustc/rustdoc/clean/types/struct.Item.html
[#77820]: https://github.com/rust-lang/rust/pull/77820
