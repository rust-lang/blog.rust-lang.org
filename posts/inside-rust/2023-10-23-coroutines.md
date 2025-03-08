+++
layout = "post"
title = "Generators are dead, long live coroutines, generators are back"
author = "oli-obk"
+++

We have renamed the unstable `Generator` trait to `Coroutine` and adjusted all terminology accordingly.
If you want to see all 3800 modified lines of code, you can find the PR [here](https://github.com/rust-lang/rust/pull/116958).

Our `Generator` trait was effectively a coroutine already, so this change was long overdue.
All nightly users using the `Generator` trait must now update their code to refer to the new feature gate and trait names.

## What is the difference?

A generator is just a convenient way to write `Iterator` implementations.
This means it's a coroutine that has no arguments and no return type.
Instead when it returns, that means iteration is over, and the `Iterator::next` method returns `None`.

## Coming full circle

This change is directly motivated for *reintroducing* generators, this time with simpler (`async`/`await` style)
syntax for creating `Iterator`s. Some discussion about this can be found in the [`gen fn` rfc](https://github.com/rust-lang/rfcs/pull/3513).

Of course such a massive change would be incomplete without the PR to immediately reintroduce a new concept with the name that was just removed,
so [here](https://github.com/rust-lang/rust/pull/116447) you can see the MVP for `gen` blocks that has very weird diagnostics and may panic on you if you poke it too much.
