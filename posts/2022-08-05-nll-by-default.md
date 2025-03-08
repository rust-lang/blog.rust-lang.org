+++
layout = "post"
title = "Non-lexical lifetimes (NLL) fully stable"
author = "Niko Matsakis"
team = "the NLL working group <https://www.rust-lang.org/governance/teams/compiler#Non-Lexical%20Lifetimes%20(NLL)%20working%20group>"
release = false
+++

As of Rust 1.63 (releasing next week), the "non-lexical lifetimes" (NLL) work will be enabled by default. NLL is the second iteration of Rust's borrow checker. The [RFC] actually does quite a nice job of highlighting some of the motivating examples. "But," I hear you saying, "wasn't NLL included in [Rust 2018]?" And yes, yes it was! But at that time, NLL was only enabled for Rust 2018 code, while Rust 2015 code ran in "migration mode". When in "migration mode," the compiler would run both the old *and* the new borrow checker and compare the results. This way, we could give warnings for older code that should never have compiled in the first place; we could also limit the impact of any bugs in the new code. Over time, we have limited migration mode to be closer and closer to just running the new-style borrow checker: in the next release, that process completes, and all Rust code will be checked with NLL. 

[RFC]: https://rust-lang.github.io/rfcs/2094-nll.html
[Rust 2018]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

## How does removing the old borrow checker affect users?

At this point, we have almost completely merged "migration mode" and "regular mode", so switching to NLL will have very little impact on the user experience. A number of diagnostics changed, mostly for the better -- [Jack Huey gives the full details in his blog post](https://jackh726.github.io/rust/2022/06/10/nll-stabilization.html).

## Credit where credit is due

The work to remove the old borrow checker has been going on for years. It's been a long, tedious, and largely thankless process. We'd like to take a moment to highlight the various people involved and make sure they are recognized for their hard work:

* [Jack Huey](https://github.com/jackh726/) ([sponsorship page](https://github.com/sponsors/jackh726)), for driving the final details of stabilization (diagnostics, reconciling differences in behavior).
* [Élie Roudninski](https://github.com/marmeladema), for refactoring the diagnostics code and doing the painstaking work (along with Jack) of checking each regressed case, one by one.
* [Aaron Hill](https://github.com/Aaron1011), for numerous improvements to bring NLL diagnostics up to par.
* [Matthew Jasper](https://github.com/matthewjasper), for reconciling errors around higher-ranked lifetimes and other core diagnostics improvements.
* [Rémy Rakic](https://github.com/lqd), for rebasing Matthew's PR as well as doing other independent diagnostics work.
* [Christopher Vittal](https://github.com/chrisvittal), for removing "compare" mode (don't ask).
* [Centril](https://github.com/centril), for doing a lot of early work reconciling migration mode and the regular mode.
* And of course the members of the NLL working group (myself, [Felix Klock](https://github.com/pnkfelix), [Santiago Pastorino](https://github.com/spastorino) ([sponsorship page](https://github.com/sponsors/spastorino)), [Matthew Jasper](https://github.com/matthewjasper), [David Wood](https://github.com/davidtwco), [Rémy Rakic](https://github.com/lqd), [Paul Daniel Faria](https://github.com/nashenas88), [Nick Nethercote](https://github.com/nnethercote)) who delivered the feature in the first place.

Jack's blog post includes a [detailed narrative](https://jackh726.github.io/rust/2022/06/10/nll-stabilization.html#how-did-we-get-here) of all the work involved if you'd like more details! It's a fun read.

## Looking forward: what can we expect for the "borrow checker of the future"?

The next frontier for Rust borrow checking is taking the [polonius](https://github.com/rust-lang/polonius) project and moving it from research experiment to production code. Polonius is a next-generation version of the borrow checker that was "spun off" from the main NLL effort in 2018, as we were getting NLL ready to ship in production. Its most important contribution is fixing a known limitation of the borrow checker, demonstrated by the following example:

```rust
fn last_or_push<'a>(vec: &'a mut Vec<String>) -> &'a String {
    if let Some(s) = vec.last() { // borrows vec
        // returning s here forces vec to be borrowed
        // for the rest of the function, even though it
        // shouldn't have to be
        return s; 
    }
    
    // Because vec is borrowed, this call to vec.push gives
    // an error!
    vec.push("".to_string()); // ERROR
    vec.last().unwrap()
}
```

This example doesn't compile today ([try it for yourself](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=517ac32f0aab076faa32b9065783bbb4)), though there's not a good reason for that. You can often workaround the problem by editing the code to introduce a redundant `let` ([as shown in this example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d9b25963e83201902ecf5c02d79cbc13)), but with polonius, it will compile as is. If you'd like to learn more about how polonius (and the existing borrow checker) works[^name], you can [watch my talk from Rust Belt Rust](https://www.youtube.com/watch?v=_agDeiWek8w).

[^name]: Or where the name "polonius" comes from!
