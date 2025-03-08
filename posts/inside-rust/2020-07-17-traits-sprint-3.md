+++
layout = "post"
title = "Traits working group 2020 sprint 3 summary"
author = "Jack Huey"
team = "The Traits WG <https://rust-lang.github.io/wg-traits/>"
+++

Again? It feels like we just had one of these...6 weeks ago 😉. Anyways, much of this sprint was a continuation of the previous two: working towards making Chalk feature-complete and eventually using it in rustc for trait solving.

For those who haven't seen one of these before, if you're curious about the traits working group, you can find a summary [here](https://rust-lang.github.io/wg-traits/).

### Credit where credit is due

As always, a big thanks to everyone who participated in this sprint:

* [Wilco Kusee]
* [Florian Diebold]
* [Jack Huey]
* [Charles Lew]
* [Niko Matsakis]
* [Nathan Whitaker]
* [Adam Bratschi-Kaye]
* [super-tuple]
* [David Ross]
* [Zahari Dichev]
* [Mikhail Babenko]
* [Mark Drobnak]
* [Aaron Hill]
* [Pavan Kumar Sunkara]
* [Nathan Corbyn]
* [Matthew Jasper]
* [Bastian Kauschke]

[Wilco Kusee]: https://github.com/detrumi
[Charles Lew]: https://github.com/crlf0710
[Niko Matsakis]: https://github.com/nikomatsakis
[Jack Huey]: https://github.com/jackh726
[Florian Diebold]: https://github.com/flodiebold
[Nathan Whitaker]: https://github.com/nathanwhit
[Adam Bratschi-Kaye]: https://github.com/adamrk
[super-tuple]: https://github.com/super-tuple
[David Ross]: https://github.com/daboross
[Zahari Dichev]: https://github.com/zaharidichev
[Mikhail Babenko]: https://github.com/Areredify
[Mark Drobnak]: https://github.com/Mcat12
[Aaron Hill]: https://github.com/Aaron1011
[Pavan Kumar Sunkara]: https://github.com/pksunkara
[Nathan Corbyn]: https://github.com/doctorn
[Matthew Jasper]: https://github.com/matthewjasper
[Bastian Kauschke]: https://github.com/lcnr

### Chalk crate cleanups and weekly publish

Since its inception, Chalk has undergone a fair number of changes in regards to the structure of its crates, as any reasonably sized project might. During this sprint, we took the time to clean up the crate structure a bit. It's probably easiest to just give a brief overview of what we ended up with. A more comprehensive overview can be found in the [Chalk book](http://rust-lang.github.io/chalk/book/what_is_chalk/crates.html).
- `chalk-derive` - Defines the derive proc macros
- `chalk-ir` - A basic "type library", which might someday be shared between rustc, Chalk, and rust-analyzer
- `chalk-solve` - Defines the Rust semantics of the types from `chalk-ir`
- `chalk-engine` - Implements the SLG solver
- `chalk-recursive` - Implements the recursive solver
- `chalk-parse` - Used for testing, parses a Rust-like syntax into `chalk-ir` and `chalk-solve` types
- `chalk-integration` - Used for testing, provides types useful for testing
- `chalk` - Used for testing, provides a REPL

Also during this sprint, we set up regular weekly releases of the Chalk crates. While at the moment these are all `0.*.0` patch releases, it sets up the infrastructure for future stable releases and provides published crates to be used in rustc and rust-analyzer. In the future, when Chalk development is more stable, we want to switch this to be manual. We also plan to set up bors to ensure that `master` always builds and passes tests.

### Work towards GAT support in rustc

Chalk has had support for GATs for [a while now](https://github.com/rust-lang/chalk/pull/145); in Chalk terms, GATs are a natural extension of everything else. [GATs in rustc](https://github.com/rust-lang/rust/issues/44265), however, are a bit more difficult, and have been fairly stagnant over the past couple of years, with the primary focus going towards getting Chalk ready. Recently, however, some work has been done in rustc to get GATs working under the current rustc trait system.

### Extracting a shared library representing types

As a long term goal, we hope to one day have a shared type library between Chalk and rustc. Moreover, this type library could be used for other projects, such as rust-analyzer. On the Chalk side, more types — such as closures and enums — and more traits — such as the `Fn` family and `Unsize` — were added. Additionally, some work has been done to go the *other direction*: to move rustc closer to Chalk, such as [interning `Predicate`s](https://github.com/rust-lang/rust/pull/72055), and [introducing `ForAll` predicates](https://github.com/rust-lang/rust/pull/73503).

### Writing a `.chalk` file for debugging

As part of Chalk tests, we can write Rust-like "programs" that get parsed into Chalk types. Importantly, these programs are much more succint than the types they get lowered to. As part of an effort to better enable debugging, we implemented a system to go in the opposite direction: to be able to generate the Rust-like programs from the underlying types. This is extremely useful to, for example, debug a bug for a given bit of code that rustc tries to compile. Additionally, this could be used to generate programs for cases with performance problems.

### Improving `impl Trait` support

In the last sprint, we landed initial `impl Trait` support, to handle the simple case of something like `type Foo<T> = impl Bar`. During this sprint, we began work on adding more support for more complex cases such as `type Foo<T>: Debug = impl Bar where T: Debug`. Additionally, some design work was done to support checking that these are well-formed.

### Extend Chalk to support Rust semantics

This goal overlaps a bit with "extracting a shared type library", but is less about representing types themselves and more about expressing the semantics of those types. For example, consider the following program:

```
trait Foo: Sized {
    fn foo(self) {}
}
impl Foo for u32 {}
impl Foo for String {}

fn main() {
  let x = 0;
  x.foo();
}
```
Prior to the current sprint, Chalk wouldn't be able to handle this properly; it wouldn't know that you can call `foo` on `0`. In fact, to be able to compile this program correctly, the compiler has to know that `0` can never be a `String`. Consider what would happen if you changed to impl for `String` to `u64`: rustc wouldn't know if you wanted `0` to be a `u32` or an `u64`. That's essentially how Chalk would have seen this program prior to this sprint. However, Chalk now correctly handles this situation.

### Handle lifetime constraints in rustc integration

So, as part of trait solving Chalk and rustc may sometimes find one lifetime needs to outlive another, or that a type must outlive a lifetime. For example,

```
trait Foo<'a, 'b, T> where 'a: 'b, T: 'a  {}
```
Chalk doesn't solve these lifetime constraints on its own, instead it passes them back to rustc. During this sprint, we added the ability to express these where clauses in Chalk.

### Other work

There was a bunch of smaller stuff done during this sprint that doesn't really fit into separate goals. Chalk got a little bit smarter in its suggestions. We introduced `tracing` for logging. We did some design work for the recursive solver. And of course, a fair amount of internal refactoring and cleanup. And of course, the rustc integration has been kept up with and updated for newer Chalk features.

## Summer vacation

It's been a busy year so far! Since the first sprint started in early February, we've made tons of progress. However, unlike the past couple sprints, we aren't going to immediately jump into our next one. Instead, we're taking a couple months for vacation, and we'll start back up in September. Until then, we won't have have weekly meetings on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/144729-wg-traits) nor will we have any defined goals. This is in part since some members might be taking a vacation. But also, code burnout is very real and a break every now and then can be helpful. In the meantime, there are a few items left to finish/cleanup.

If you're interested in helping, don't be discouraged! Zulip should still be fairly active, so feel free to drop by!
