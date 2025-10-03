+++
path = "inside-rust/2020/03/28/traits-sprint-1"
title = "Traits working group 2020 sprint 1 summary"
authors = ["Jack Huey"]
aliases = ["inside-rust/2020/03/28/traits-sprint-1.html"]

[extra]
team = "The Traits WG"
team_url = "https://rust-lang.github.io/wg-traits/"
+++

This Tuesday, the traits working group finished our first sprint of 2020, last 6 weeks from February 11th through March 24th. The last sprint was about a year ago, but we decided to resurrect the format in order to help push forward traits-related work in [Chalk] and rustc.

## What is wg-traits and what do we do?

### Goal: An efficient, extensible, and reusable crate for the Rust trait system

The overarching goal of the [traits working group][wg-traits] is to create a performant, extensible, and clean implementation of Rust's trait system. This implementation should scale not only to existing Rust features but also to new and upcoming features, such as:

* Implied bounds ([RFC](https://rust-lang.github.io/rfcs/2089-implied-bounds.html))
* Const generics ([RFC](https://rust-lang.github.io/rfcs/2000-const-generics.html))
* Generic associated types (GATs) ([RFC](https://rust-lang.github.io/rfcs/1598-generic_associated_types.html))
* Inherent associated items ([tracking issue](https://github.com/rust-lang/rust/issues/8995))

[wg-traits]: https://rust-lang.github.io/wg-traits/

As if that weren't enough, we'd like the implementation to be **reusable**, too -- meaning that it can be used by rustc, yes, but also rust-analyzer and potentially other contexts as well. 

This effort is part of one of the big, longer term goals for the compiler team: **library-ification**. This refers to the idea of breaking apart the compiler into independent libraries that can be learned, tested, and developed independently.

In order to achieve these and future features, our work is split into two parts: 1) Improving rustc's existing trait solver. 2) Design and implement the [Chalk] trait solver, work towards integration into rustc. The Chalk trait solver, briefly, is a logic-based trait solver, designed to be independent of rustc internals. In addition to it being more powerful than the current rustc trait solving implementation, Chalk can be used as a library for compiler-related work, such as IDE integration (e.g. [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer)).

Coming into 2020, we — the traits working group — knew we wanted to get more organized and start to push more on getting Chalk fully integrated into rustc, by cleaning up the Chalk codebase itself, fixing bugs, implementing new features, and ultimately integrating Chalk into rustc itself. In addition, we are committed to documenting design considerations and decisions for better accessibility now and in the future. For example, we now publish a Chalk [book] which, while incomplete, attempts to document the Chalk internals somewhat akin to the [rustc dev guide](https://rustc-dev-guide.rust-lang.org/).

#### A note about Chalk integration in rustc
An experimental integration of Chalk was in rustc (under the `-Z chalk` flag) for over a year, but since its initial implementation, little work had been done while much work had been done on Chalk itself. This ultimately meant that the initial implementation based on the older Chalk version looks very different from what an implementation based on the current Chalk would and should look like. Under this reasoning, that experimental implementation has been removed.

## 2020 sprint 1

Ok, with the background finished, that brings us to the actual 2020 sprint 1. Going into this, we didn't *quite* know what our goals would be. In this post, we'll share an overview of each of the things that were accomplished during this sprint, which actually was quite a lot!

### Credit where credit is due

A big thank you :hearts: to the folks who participated in this sprint:

* [David Barsky]
* [detrumi]
* [Florian Diebold]
* [Jack Huey]
* [Charles Lew]
* [Jane Lusby]
* [Niko Matsakis]

[David Barsky]: https://github.com/davidbarsky
[detrumi]: https://github.com/detrumi
[Jane Lusby]: https://github.com/yaahc
[Charles Lew]: https://github.com/crlf0710
[Niko Matsakis]: https://github.com/nikomatsakis
[Jack Huey]: https://github.com/jackh726
[Florian Diebold]: https://github.com/flodiebold

### wg-traits skill tree

Our "[skill tree]" is how we track our [development roadmap]. It shows some of the major goals we are working towards (e.g., having chalk be usable as a standalone library) along with some of the major tasks that we have to complete along the way. You can click on the tasks to be taken to a github issue or other explanation. We try to update it after every meeting so that we have some idea of what we're doing and why.

The skill tree structure was inspired by [this blog post about WebAssembly][wasm], which in turn borrowed the term from games. Sadly, the current tool that generates the skill tree doesn't yet make anything as beautiful as the hand-drawn art in the WASM post. If anybody is interested in improving the tool's output, that is on the list of 'stretch goals' for this coming sprint!

[skill tree]: https://rust-lang.github.io/wg-traits/roadmap/skill-tree.html
[development roadmap]: https://rust-lang.github.io/wg-traits/roadmap.html
[wasm]: https://hacks.mozilla.org/2018/10/webassemblys-post-mvp-future/

### Chalk book `chalk-engine` chapter

As mentioned before, in our effort to document Chalk internals, we started publishing a [book] late last year. Near the beginning of this sprint, we added a whole chapter about [`chalk-engine`](http://rust-lang.github.io/chalk/book/engine.html) itself. This is the core crate of Chalk that solves a given set of `Goal`s. While there is always more that can be documented, we hope this at least is a start in helping people, potentially newcomers, to understand how Chalk works internally.

### Work on basic support for `impl Trait`

In Rust, there are a few places, currently and in the future, where you may specify `impl Trait` instead of a specific struct. For example, the signature of a function may be `fn foo() -> impl Debug`. Another place where you may use the `impl Trait` syntax in the future is with `type Foo = impl Trait` (currently under the [`type_alias_impl_trait`](https://github.com/rust-lang/rust/issues/63063) feature). This would allow you to use `Foo` as if it was a concrete type. During this sprint, we made significant progress in allowing both of these to work with Chalk. We'll be doing follow-up work on this in the upcoming sprint, and hopefully landing support.


### Creating a proposal for a shared type library

Currently, rustc, rust-analyzer, and chalk each represent Rust types using a different set of structs. This means that when rustc or rust-analyzer wish to invoke chalk functions, we have to convert the representation of Rust types back and forth. This is fine for the time being, but eventually we would like to be having everyone use the same representation, so that no interconversion is required. This is a bit tricky, though, because the requirements of rustc (a batch compiler) and rust-analyzer (an IDE) are somewhat different. During this sprint, we wrote up a proposal for a shared type library, and led a design meeting on the topic. You can find the [record of that meeting here](https://rust-lang.github.io/compiler-team/minutes/design-meeting/2020-03-12-shared-library-for-types/), which also includes the proposal.

During this upcoming sprint, we'll be following up on this design by starting to do some of the preliminary refactorings in rustc.

### Refactoring for passing `Interner` around

One of the requirements for a shared type library is that it needs to support interning and arena allocation of types. *Interning* a type means to re-use the same memory each time you have an equivalent type, rather than allocating multiples copies. *Arena allocation* is a memory management strategy where you allocate all the memory in an ever-growing pool and then free the entire pool at once, rather than tracking and freeing individual allocations. 

Chalk's existing type library was implemented with simplicity in mind, however, and couldn't support either of these use-cases. The problem was that to support interning and arena allocation, you need to track around an *interner* variable that contains the hash-maps, arenas, and other supporting data structures, and chalk's APIs didn't have any space for that. This sprint, we fixed that, so that we now pass along an `interner` value throughout chalk, meaning we can bridge to rustc and rust-analyzer more easily.

### Refactoring how chalk represents bound types and lifetimes

Some of the details of how chalk represented types with *bound variables* (e.g., the `'a` in `for<'a> fn(&'a u32)`) differed from how rustc was handling such types. This made bridging from rustc to chalk much harder.  We found that the design we ultimately want was a hybrid of what rustc and and chalk have. During this sprint, we did most of the chalk refactoring, and in the upcoming sprint, we'll work on the rustc side of the work.

### Work on adding `tracing` support to Chalk

The [`tracing`](https://crates.io/crates/tracing) crate provides a framework for collecting event-based diagnostic information. Currently, in Chalk, we only have basic logging support. By adding `tracing` support, we expect to get more fine-grained control of Chalk diagnostics. Initial support is nearly finished and hopefully will get merged soon.
### Exploratory rustc integration MVP

As mentioned before, the previous experimental Chalk integration was removed from rustc since it was outdated. Since there are quite a few design differences between Chalk and rustc's current trait solver, some subtle, it's not always clear what *exactly* needs to be modified to makes things work correctly. We have [started](https://github.com/rust-lang/rust/pull/69406) writing the experimental Chalk integration. The goal, at least to start, is to create a minimal implementation as a basis for future work. While the pull request is not *quite* there, it's close and has been tremendously helpful in uncovering blocking issues in Chalk that hold up progress.

### Exploratory recursive solver

One of the interesting aspects of chalk's design is that it separates out the **solver strategy** from other parts of the trait system implementation. In addition to our existing solver, the so-called ["on demand slg solver"], we are exploring a ["recursive solver"] design. We began by resurrecting an older version of this code that was removed and have been exploring adapting it to the newer ideas.

["on demand slg solver"]: http://smallcultfollowing.com/babysteps/blog/2018/01/31/an-on-demand-slg-solver-for-chalk/
["recursive solver"]: https://gist.github.com/nikomatsakis/bfbdbe588d6fc61ecb09e3b51847fb7c

### Minor Chalk cleanups

Over the last sprint, there have been a couple smaller cleanups to Chalk to mention. It can build rustc again, passing rustc's lints. We removed an unneeded dependency (well, it's technically there for tests). Finally, we also made Chalk a bit more panic-safe.

## 2020 sprint 2

We plan to begin the next sprint of 2020 next Tuesday, March 31st. We'll briefly cover a few goals:

### How to get involved

If you'd like to get involved, please drop in on the [rust-lang Zulip] in the `#wg-traits` stream. We also have a [weekly design meeting] (held on Zulip) that we use to sync up on progress and discuss tricky issues.

[rust-lang Zulip]: https://rust-lang.zulipchat.com/
[weekly design meeting]: https://calendar.google.com/event?action=TEMPLATE&tmeid=b2hhbXZ2YzcxNzhsMTZqNHFibGxpMmZubjRfMjAyMDAzMzFUMjAwMDAwWiA2dTVycnRjZTZscnR2MDdwZmkzZGFtZ2p1c0Bn&tmsrc=6u5rrtce6lrtv07pfi3damgjus%40group.calendar.google.com&scp=ALL

### Chalk rustc-integration MVP

It will be a bit of a stretch, but we hope that this sprint we can complete work on a "MVP" of chalk-rustc integration that we can use to drive further development. This MVP will be unsound and incomplete (for example, it will likely not enforce borrow checker rules correctly), but it will help us to uncover corner cases and to validate the design of the chalk solver. Towards this end, we have a number of concrete tasks:

* [Extending chalk with support for builtin traits like `Sized`, `Copy`, and `Clone`][chalk#363]
    * There are several traits for which the precise rules are not expressed as ordinary impls, but rather require special integration in the library itself. Chalk doesn't currently have any support for these traits, so we need to extend it.
* Land existing branch 
* Converting rustc types into chalk types
    * Eventually, we hope to have rustc and chalk sharing the same type library, so that no bridging is needed between them. But creating such a library will take a while. So, in the interim, we will write code that converts rustc types into chalk types on demand. (Some of the other sprint goals, meanwhile, will be adapting rustc types so that we are also moving towards our eventual goal.)

[chalk#363]: https://github.com/rust-lang/chalk/issues/363

### Design meeting for `const` integration

As mentioned in the previous section, our initial Chalk rustc-integration MVP won't have support for `const`. During this sprint, we plan on [scheduling a design meeting][wg-traits#15] to specifically flesh out some of the design about what `const` *would* look like. Actual implementation will be left for a later sprint.

[wg-traits#15]: https://github.com/rust-lang/wg-traits/issues/15

### Move towards aligning rustc and Chalk types

During this sprint, we plan to start working towards extracting a shared library for Rust types, as discussed in the [design meeting] mentioned previously. This will involve work on refactoring rustc as well as changes to chalk. ([Tracking issue.](https://github.com/rust-lang/wg-traits/issues/16))

[design meeting]: https://rust-lang.github.io/compiler-team/minutes/design-meeting/2020-03-12-shared-library-for-types/

### Land basic support for `impl Trait`

We expect to land basic support for `impl Trait` fairly early in the next sprint. However, there is some [followup work][chalk#335] to be done to further refine the implementation.

[chalk#335]: https://github.com/rust-lang/chalk/issues/335

### Exploratory implementations and research

In addition to the more concrete goals, there is also some exploratory work being done:
* [Implementing a recursive solver][chalk#351]
* [Converting semantic to syntactic equality][chalk#364]
* [Outputting a file for reproducing bugs][chalk#365]

[chalk#351]: https://github.com/rust-lang/chalk/issues/351
[chalk#364]: https://github.com/rust-lang/chalk/issues/364
[chalk#365]: https://github.com/rust-lang/chalk/issues/365

### Chalk performance work

Most of the work on Chalk has been focused on design, and *not much* has been done to optimize performance. While the particular "end goal" isn't clear here, we hope to start by creating a set of memory, cpu, and time benchmarks for Chalk. With this framework, we can diagnose specific performance issues and monitor future changes for regressions. Part of this will be to [land][chalk#337] `tracing` support.

[chalk#337]: https://github.com/rust-lang/chalk/issues/337

### Improving the skill tree

The skill tree has been a useful tool for helping us organize our work and track our status and overall plan. However, the current output is not exactly self explanatory, nor is it particularly attractive. The ultimate goal is to generate pictures similar to Lin's [hand drawn artwork][wasm]. There are also some missing features. If there is someone out there interested in taking a stab at improving the quality of the output, or adding features, that would be great! skill-tree lives in its own [github repo](https://github.com/nikomatsakis/skill-tree), but just drop by the `#wg-traits` stream on Zulip to chat about it.

[book]: http://rust-lang.github.io/chalk/book/
[Chalk]: https://github.com/rust-lang/chalk
