+++
path = "inside-rust/2026/08/10/overloading-experiment"
title = "Rust Function Overloading - Call for Experimentation"
authors = ["teor"]

[extra]
team = "The Language Team"
team_url = "https://www.rust-lang.org/governance/teams/lang"
+++

In partnership with the [Rust Foundation's Rust-C++ Interop Initiative][interop-initiative],
the Rust Project has been experimenting with
[function overloading for FFI bindings][overloading-goal]. This experiment is now at a stage
where compiler and interop tool developers can start exploring function overloading.

[interop-initiative]: https://rustfoundation.org/interop-initiative/
[overloading-goal]: https://rust-lang.github.io/rust-project-goals/2026/overloading-for-ffi.html

Stable Rust already supports [a form of overloading using tuples and traits][stable-example], but
calling these overloaded functions looks strange, because the overloaded arguments have to be
passed as a single tuple argument, like this: `hypot((2.0, 3.0, 6.0))`. Stable Rust also allows
overloading of built-in operators with user-defined types, via traits like
[`Add` (the `+` operator)][add-docs] and [`Neg` (the `-` value negation operator)][neg-docs].

[stable-example]: https://internals.rust-lang.org/t/pre-pre-rfc-splatting-for-named-arguments-and-function-overloading/24012
[add-docs]: https://doc.rust-lang.org/std/ops/trait.Add.html
[neg-docs]: https://doc.rust-lang.org/std/ops/trait.Neg.html

We are running an unstable nightly Rust language experiment to answer questions like:

- How much overloading can we do with Rust’s existing trait system?
- Could this help us call C++ from Rust ergonomically?

In the tradition of [yeet][yeet-issue] (to [avoid bikeshedding][bikeshed-defn]), we are using basic
syntax in the first stage of the experiment: the `#[rustc_splat]` attribute.
[Alternative syntaxes][splat-defn] can be considered later, if the experiment generates useful
outcomes.

[yeet-issue]: https://github.com/rust-lang/compiler-team/issues/501
[bikeshed-defn]: https://en.wiktionary.org/wiki/bikeshedding
[splat-defn]: https://en.wiktionary.org/wiki/splat#Noun_2:~:text=An%20operator%20indicating%20a%20variable-length%20argument%20list

## Experimental Function Overloading

Rust nightly builds [from 2026-07-31 onwards][rustup-components] have experimental support for
more ergonomic function and method overloading, using the incomplete "splat" compiler feature;
if you're a compiler or interop tool developer, we encourage you to experiment with it!

[This experiment][splat-tracking] lets overloaded functions be called with separate arguments,
like this: `hypot(2.0, 3.0, 6.0)`. No double parentheses required! But type inference and type
checking still happen as they would in a stable Rust overload.

We are experimenting with splat to get a feel for the complexity of the implementation, and to see
if it solves some language interoperability use cases. Like most
[Rust language experiments][lang-exp-howto], this nightly feature has no RFC, and can change or be
removed at any time.

[rustup-components]: https://rust-lang.github.io/rustup-components-history/
[splat-tracking]: https://github.com/rust-lang/rust/issues/153629
[lang-exp-howto]: https://lang-team.rust-lang.org/how_to/experiment.html

### Experiment Design

We expect the feature to change significantly in future, or to be replaced by a more ergonomic
interface. In this spirit, [Ajay Singh][gh-ajay], a
[Rust Project Outreachy intern][ajay-blog], is working on a macro to make splat-based
overloading more ergonomic. You can find his work in the
[rust-foundation/overloading-macros][overloading-macros] repository on GitHub.

The design axioms for this feature are:

- **"keep Rust nice"**
- make calling overloaded FFI functions easy
- preserve foreign language maintainability
- select the overload most developers would expect

This could be a challenging design, because different programming languages have different overload
resolution rules.

[gh-ajay]: https://github.com/ajay-singh1
[overloading-macros]: https://github.com/rustfoundation/overloading-macros/tree/main/splat-overload
[ajay-blog]: https://ajay-singh1.github.io/posts/outreachy/

### Example Usage

Rust overloading aims to support a range of programming languages. This example uses C++ because it
is well known, and has significant existing interop tooling.

Here is some Rust code that calls the overloaded C++ `hypot` (hypotenuse) function, using "splat"
to create corresponding overloads in Rust.

```rust
#![feature(splat, tuple_trait)]
#![expect(incomplete_features)]

use cpp::cpp;
use std::{ffi::c_double, marker::Tuple};

cpp! {{ #include <cmath> }}

/// The arguments of an overloaded C++ `hypot` function.
trait HypotArgs: Tuple {
    type Output;
    fn call_hypot(self) -> Self::Output;
}

/// Calls the overloaded C++ `std::hypot` function with the given arguments.
fn hypot<Args: HypotArgs>(#[rustc_splat] args: Args) -> <Args as HypotArgs>::Output {
    args.call_hypot()
}

/// A 2-argument `hypot` overload.
impl HypotArgs for (c_double, c_double) {
    type Output = c_double;
    fn call_hypot(self) -> c_double {
        let (x, y) = self;
        unsafe {
            cpp!([x as "double", y as "double"] -> c_double as "double" {
                // This is C++ code!
                return std::hypot(x, y);
            })
        }
    }
}

/// A 3-argument `hypot` overload.
impl HypotArgs for (c_double, c_double, c_double) {
    type Output = c_double;
    fn call_hypot(self) -> c_double {
        let (x, y, z) = self;
        unsafe {
            cpp!([x as "double", y as "double", z as "double"] -> c_double as "double" {
                return std::hypot(x, y, z);
            })
        }
    }
}

fn main() {
    println!("|(3, 4)|    = {}", hypot(3.0, 4.0));
    println!("|(2, 3, 6)| = {}", hypot(2.0, 3.0, 6.0));
}
```

This example uses [the cpp crate][cpp-crate] to inline C++ code in a Rust file. A full runnable
example [is available on GitHub][overloading-examples]. You can also run a minimal Rust-only
example online, [in the Rust playground][playground-hypot].

[Nadrieril’s original "Overloading at Home" code][pre-pre-rfc] from his recent write-up of "splat",
can also be run [in the Rust playground][playground-at-home].

[cpp-crate]: https://crates.io/crates/cpp
[overloading-examples]: https://github.com/rustfoundation/overloading-examples
[playground-hypot]: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=d9494d84fa32271b3517fa293d3215ff
[pre-pre-rfc]: https://internals.rust-lang.org/t/pre-pre-rfc-splatting-for-named-arguments-and-function-overloading/24012
[playground-at-home]: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=fcf1ead0e431166177fd9f05d5e33fd0

### Limitations

As stated above, splat is currently an incomplete compiler feature, and is only available in the
nightly Rust compiler. Splat is also unergonomic, and we want to change that as part of the next
design phase.

Support for splatted function arguments in rustdoc [just merged on 12 August][rustdoc-pr]. Splatted
arguments display as an ellipsis (`…`), rather than the argument name. This syntax is unstable,
currently only used for display in rustdoc, and can change at any time. For example:

```rust
fn example(#[rustc_splat] args: (u32, String));
```

Is displayed as:

```rust
fn example(…: (u32, String));
```

We've also recently merged [splat support for function pointers][fnptr-pr], if you're seeing
[an internal compiler error][fnptr-bug], please update to the latest nightly.
If you want overloading support for function pointers, please let us know about your use case in
the [#t-lang/interop channel on Zulip][t-lang-interop].

And there is an ongoing [Rust standard library experiment][variadic-tracking] using splat for
variable-argument `smallest` and `greatest` functions. Hopefully they will be
[available soon on nightly][variadic-pr].

Experimenters will find other bugs – [some have already been found in the last few months][f-splat]
– and some overloading functionality is out of scope for now. If you think you've found a bug or
limitation, ask us about it in the [#t-lang/interop channel on Zulip][t-lang-interop].

[rustdoc-pr]: https://github.com/rust-lang/rust/pull/160882
[fnptr-bug]: https://github.com/rust-lang/rust/issues/158603
[fnptr-pr]: https://github.com/rust-lang/rust/pull/158645
[variadic-tracking]: https://github.com/rust-lang/rust/issues/160728
[variadic-pr]: https://github.com/rust-lang/rust/pull/160687
[t-lang-interop]: https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop
[f-splat]: https://github.com/rust-lang/rust/issues?q=label:F-splat

### Credits

This work would not be possible without Google’s generous funding and support of the Rust
Foundation’s Rust-C++ [Interop Initiative][interop-initiative].

It is driven by the [Nightly support for function overloading in FFI bindings][overloading-goal]
Rust Project goal, and it is an outcome of the
[C++/Rust Interop Problem Space Mapping][interop-goal] Rust Project goal.

[interop-goal]: https://rust-lang.github.io/rust-project-goals/2026/interop-problem-map.html

Thank you also to everyone who has contributed to the overloading work, including [Oli][gh-oli],
[Ajay][gh-ajay], [Nadrieril][gh-nadri], [Scott][gh-scott], [Taylor][gh-taylor], [Tyler][gh-tyler],
[Matthias][gh-matthias], [Tim][gh-tim], [Devin][gh-devin], [Ralf][gh-ralf], [Jacob][gh-jacob],
[Zachary][gh-zachary] and the many project members who have given suggestions, feedback, testing,
bug reports, and reviews.

[gh-oli]: https://github.com/oli-obk
[gh-nadri]: https://github.com/nadrieril
[gh-scott]: https://github.com/scottmcm
[gh-taylor]: https://github.com/cramertj
[gh-tyler]: https://github.com/tmandry
[gh-matthias]: https://github.com/matthiaskrgr
[gh-tim]: https://github.com/theemathas
[gh-devin]: https://github.com/ssbr
[gh-ralf]: https://github.com/RalfJung
[gh-jacob]: https://github.com/programmerjake
[gh-zachary]: https://github.com/bushrat011899

## Future Work

### Overloading, In The Shiny Future

```rust
#[overload]
impl f64 {
    /// Returns the 2-dimensional distance from the origin.
    fn hypot(self, y: f64) -> f64 { … }

    /// Returns the 3-dimensional distance from the origin.
    fn hypot(self, y: f64, z: f64) -> f64 { … }
}
```

_(This example is a variant of a design shared by [Taylor Cramer][gh-taylor].)_

In the shiny future, Rust might have an `#[overload]` attribute that "just works" to call
overloaded foreign functions with the same name. No traits, tuples, or `#[rustc_splat]` required,
the compiler handles it all for you. Since this work is targeted at interop, overloading might be
limited to `extern` blocks to start with. Extending it to native Rust could be a separate feature,
stabilised on a longer timeframe (or not at all).

This might happen through macros that hide compiler implementation internals, or it might not need
any macros. It's hard to guess what the final shape of the feature will be: we’ve only just started
the first experiment.

### Future Designs

We have a lot of work to do before we can discuss overloading designs in detail. The "splat"
experiment will help us find the limits of the Rust type system, how it handles typical foreign
language overloads, the diagnostics needed to guide overloading users, and any design gaps for
future work.

We’ll also need to find a syntax for overloading, if it is accepted. Naming things is hard, as many
users of function overloading have discovered 😅

Stay tuned for future interop updates on our blog. You can see the
[full list of Project Goals here][goals-2026], many of which are
[working towards better interop][interop-goals] with a range of programming languages.

[goals-2026]: https://rust-lang.github.io/rust-project-goals/2026/goals.html
[interop-goals]: https://docs.google.com/document/d/1pFOUJXcs3bZKsCwrMOVHaCbjSe8XEb5ZtHux_4nTwp0/edit?usp=sharing
