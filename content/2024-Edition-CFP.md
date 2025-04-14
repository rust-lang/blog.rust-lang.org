+++
path = "2023/12/15/2024-Edition-CFP"
title = "A Call for Proposals for the Rust 2024 Edition"
authors = ["Ben Striegel on behalf of the Edition 2024 Project Group"]
aliases = ["2023/12/15/2024-Edition-CFP.html"]
+++

The year 2024 is soon to be upon us, and as long-time Rust aficionados know,
that means that a new Edition of Rust is on the horizon!

## What is an Edition?

You may be aware that a new *version* of Rust is released every six weeks.
New versions of the language can both add things as well as change things,
but only in backwards-compatible ways, according to Rust's
[1.0 stability guarantee][stability].

[stability]: https://blog.rust-lang.org/2014/10/30/Stability.html

But does that mean that Rust can *never* make backwards-incompatible changes?
Not quite! This is what an Edition is:
Rust's mechanism for introducing backwards-incompatible changes in a backwards-compatible way.
If that sounds like a contradiction,
there are three key properties of Editions that preserve the stability guarantee:

1. Editions are *opt-in*;
crates only receive breaking changes if its authors explicitly ask for them.

2. Crates that use older editions *never get left behind*;
a crate written for the original Rust 2015 Edition is still supported by every Rust release,
and can still make use of all the new goodies that accompany each new version,
e.g. new library APIs, compiler optimizations, etc.

3. An Edition *never splits the library ecosystem*;
crates using new Editions can depend on crates using old Editions (and vice-versa!),
so nobody ever has to worry about Edition-related incompatibility.

In order to keep churn to a minimum, a new Edition of Rust is only released once every three years.
We've had the [2015 Edition][2015], the [2018 Edition][2018], the [2021 Edition][2021],
and soon, the 2024 Edition. And we could use your help!

[2015]: https://doc.rust-lang.org/edition-guide/rust-2015/index.html

[2018]: https://doc.rust-lang.org/edition-guide/rust-2018/index.html

[2021]: https://doc.rust-lang.org/edition-guide/rust-2021/index.html

## A call for proposals for the Rust 2024 Edition

We know how much you love Rust, but let's be honest, no language is perfect,
and Rust is no exception.
So if you've got ideas for how Rust could be better if only that pesky stability guarantee
weren't around, now's the time to share!
Also note that potential Edition-related changes aren't just limited to the language itself:
we'll also consider changes to both Cargo and rustfmt as well.

Please keep in mind that the following criteria determine the sort of changes we're looking for:

1. A change must be possible to implement without violating the strict properties
listed in the prior section.
Specifically, the ability of crates to have cross-Edition dependencies imposes restrictions
on changes that would take effect across crate boundaries, e.g. the signatures of public APIs.
However, we will occasionally discover that an Edition-related change
[that was once thought to be impossible actually turns out to be feasible][change],
so hope is not lost if you're not sure if your idea meets this standard;
propose it just to be safe!

[change]: https://doc.rust-lang.org/edition-guide/rust-2021/IntoIterator-for-arrays.html

2. We strive to ensure that *nearly all* Edition-related changes can be applied
to existing codebases automatically (via tools like `cargo fix`),
in order to make upgrading to a new Edition as painless as possible.

3. Even if an Edition *could* make any given change, [that doesn't mean that it should][jeff].
We're not looking for hugely-invasive changes or things that would fundamentally
alter the character of the language.
Please focus your proposals on things like fixing obvious bugs, changing annoying behavior,
unblocking future feature development, and making the language easier and more consistent.

[jeff]: https://www.youtube.com/watch?v=9nazm3_OXac

To spark your imagination, here's a real-world example.
In the 2015 and 2018 Editions, iterating over a fixed-length array via `[foo].into_iter()`
will yield *references* to the iterated elements;
this is is surprising because, on other types, calling `.into_iter()` produces an iterator
[that yields owned values rather than references][iters].
This limitation existed because older versions of Rust lacked the ability to implement
traits for all possible fixed-length arrays in a generic way.
Once Rust finally [became able to express this][notes],
*all* Editions at last gained the ability to iterate over owned values in fixed-length arrays;
however, in the specific case of `[foo].into_iter()`,
altering the existing behavior would have broken lots of code in the wild.
Therefore, we used the 2021 Edition to fix this inconsistency
for the specific case of `[foo].into_iter()`,
allowing us to address [this long-standing issue][25725] while
preserving Rust's stability guarantees.

[iters]: https://doc.rust-lang.org/std/iter/#the-three-forms-of-iteration

[notes]: https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html#intoiterator-for-arrays

[25725]: https://github.com/rust-lang/rust/issues/25725

## How to contribute

Just like other changes to Rust, Edition-related proposals follow the RFC process,
as documented in [the Rust RFCs repository][rfcs].
Please follow the process documented there, and please consider [publicizing a draft of your RFC][rfcs2] to collect preliminary feedback before officially submitting it, in order to expedite the RFC process once you've filed it for real! (And in addition to the venues mentioned in the prior link, please feel free to announce your pre-RFC to [our Zulip channel][zulip].)

[rfcs]: https://github.com/rust-lang/rfcs/#rust-rfcs---rfc-book---active-rfc-list

[rfcs2]: https://github.com/rust-lang/rfcs/#before-creating-an-rfc

Please file your RFCs as soon as possible!
Our goal is to release the 2024 Edition in the second half of 2024,
which means we would like to get everything *implemented*
(not only the features themselves, but also all the Edition-related migration tooling)
by the end of May, which means that RFCs should be accepted by the end of February.
And since RFCs take time to discuss and consider,
we strongly encourage you to have your RFC filed by the end of December,
or the first week of January at the very latest.

We hope to have periodic updates on the ongoing development of the 2024 Edition.
In the meantime, if you have any questions or
if you would like to help us make the new Edition a reality,
we invite you to come chat in [the `#edition` channel in the Rust Zulip][zulip].

[zulip]: https://rust-lang.zulipchat.com/#narrow/stream/268952-edition
