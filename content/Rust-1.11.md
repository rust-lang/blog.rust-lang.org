+++
path = "2016/08/18/Rust-1.11"
title = "Announcing Rust 1.11"
authors = ["The Rust Core Team"]
aliases = [
    "2016/08/18/Rust-1.11.html",
    "releases/1.11.0",
]

[extra]
release = true
+++

The Rust team is happy to announce the latest version of Rust, 1.11. Rust is a
systems programming language focused on safety, speed, and concurrency.

As always, you can [install Rust 1.11][install] from the appropriate page on our
website, and check out the [detailed release notes for 1.11][notes] on GitHub.
1109 patches were landed in this release.

[install]: https://www.rust-lang.org/install.html
[notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1110-2016-08-18

### What's in 1.11 stable

Much of the work that went into 1.11 was with regards to compiler internals
that are not yet stable. We're excited about features like [MIR becoming the
default] and the beginnings of [incremental compilation], and the 1.11 release
has laid the groundwork.

[MIR becoming the default]: https://github.com/rust-lang/rust/pull/34096
[incremental compilation]: https://github.com/rust-lang/rust/pull/34956

As for user-facing changes, [last release], we talked about the new `cdylib`
crate type.

> The existing dylib dynamic library format will now be used solely for writing
> a dynamic library to be used within a Rust project, while cdylibs will be
> used when compiling Rust code as a dynamic library to be embedded in another
> language. With the 1.10 release, cdylibs are supported by the compiler, but
> not yet in Cargo. This format was defined in RFC 1510.

[Last release]: https://blog.rust-lang.org/2016/07/07/Rust-1.10.html

Well, in Rust 1.11, [support for cdylibs has landed in
Cargo](https://github.com/rust-lang/cargo/pull/2741)! By adding this to your
`Cargo.toml`:

```toml
crate-type = ["cdylib"]
```

You'll get one built.

In the standard library, the default hashing function [was
changed](https://github.com/rust-lang/rust/pull/33940), from SipHash 2-4
to SipHash 1-3. We have been thinking about this for a long time, as far
back as the original decision to go with `2-4`:

> we proposed SipHash-2-4 as a (strong) PRF/MAC, and so far no attack
> whatsoever has been found, although many competent people tried to break it.
> However, fewer rounds may be sufficient and I would be very surprised if
> SipHash-1-3 introduced weaknesses for hash tables.

See the [detailed release notes][notes] for more.

#### Library stabilizations

* The `append` method was added to
[`BinaryHeap`](https://doc.rust-lang.org/std/collections/binary_heap/struct.BinaryHeap.html#method.append),
[`BTreeMap`](https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.append),
and
[`BTreeSet`](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html#method.append).
In addition, `split_off` was added to [`BTreeMap`](https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.split_off) and [`BTreeSet::split_off`](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html#method.split_off).
* The `to_degrees` and `to_radians` methods were on the `f32` and `f64`
types in `libstd`, but they are now in `libcore` as well.
* `Iterator` has two new methods:
[`sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) and
[`product`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
* Both [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html#method.get_mut) and [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.get_mut) gained `get_mut`.
* [`assert_eq!` accepts a custom error message, like `assert!` does](https://github.com/rust-lang/rust/pull/33976).
* The main thread [is now called "main" instead of "&lt;main&gt;"](https://github.com/rust-lang/rust/pull/33803).

See the [detailed release notes][notes] for more.

#### Cargo features

* Cargo [added color support for Windows
consoles](https://github.com/rust-lang/cargo/pull/2804), and you can now
[configure the colors of stderr as well as
stdout](https://github.com/rust-lang/cargo/pull/2739).
* [Build scripts can now emit warnings](https://github.com/rust-lang/cargo/pull/2630).
* As mentioned above, support was added [for the cdylib crate type](https://github.com/rust-lang/cargo/pull/2741).
* Cargo now [prevents publishing crates when files are dirty](https://github.com/rust-lang/cargo/pull/2781).

See the [detailed release notes][notes] for more.

### Contributors to 1.11

We had 126 individuals contribute to 1.11. Thank you so much!

* Aaklo Xu
* Aaronepower
* Aleksey Kladov
* Alexander Polyakov
* Alexander Stocko
* Alex Burka
* Alex Crichton
* Alex Ozdemir
* Alfie John
* Amanieu d'Antras
* Andrea Canciani
* Andrew Brinker
* Andrew Paseltiner
* Andrey Tonkih
* Andy Russell
* Ariel Ben-Yehuda
* bors
* Brian Anderson
* Carlo Teubner
* Carol (Nichols &#124;&#124; Goulding)
* CensoredUsername
* cgswords
* cheercroaker
* Chris Krycho
* Chris Tomlinson
* Corey Farwell
* Cristian Oliveira
* Daan Sprenkels
* Daniel Firth
* diwic
* Eduard Burtescu
* Eduard-Mihai Burtescu
* Emilio Cobos Álvarez
* Erick Tryzelaar
* Esteban Küber
* Fabian Vogt
* Felix S. Klock II
* flo-l
* Florian Berger
* Frank McSherry
* Georg Brandl
* ggomez
* Gleb Kozyrev
* Guillaume Gomez
* Hendrik Sollich
* Horace Abenga
* Huon Wilson
* Ivan Shapovalov
* Jack O'Connor
* Jacob Clark
* Jake Goulding
* Jakob Demler
* James Alan Preiss
* James Lucas
* James Miller
* Jamey Sharp
* Jeffrey Seyfried
* Joachim Viide
* John Ericson
* Jonas Schievink
* Jonathan L
* Jonathan Price
* Jonathan Turner
* Joseph Dunne
* Josh Stone
* Jupp Müller
* Kamal Marhubi
* kennytm
* Léo Testard
* Liigo Zhuang
* Loïc Damien
* Luqman Aden
* Manish Goregaokar
* Mark Côté
* marudor
* Masood Malekghassemi
* Mathieu De Coster
* Matt Kraai
* Mátyás Mustoha
* M Farkas-Dyck
* Michael Necio
* Michael Rosenberg
* Michael Woerister
* Mike Hommey
* Mitsunori Komatsu
* Morten H. Solvang
* Ms2ger
* Nathan Moos
* Nick Cameron
* Nick Hamann
* Nikhil Shagrithaya
* Niko Matsakis
* Oliver Middleton
* Oliver Schneider
* Paul Jarrett
* Pavel Pravosud
* Peter Atashian
* Peter Landoll
* petevine
* Reeze Xia
* Scott A Carr
* Sean McArthur
* Sebastian Thiel
* Seo Sanghyeon
* Simonas Kazlauskas
* Srinivas Reddy Thatiparthy
* Stefan Schindler
* Steve Klabnik
* Steven Allen
* Steven Burns
* Tamir Bahar
* Tatsuya Kawano
* Ted Mielczarek
* Tim Neumann
* Tobias Bucher
* Tshepang Lekhonkhobe
* Ty Coghlan
* Ulrik Sverdrup
* Vadim Petrochenkov
* Vincent Esche
* Wangshan Lu
* Will Crichton
* Without Boats
* Wojciech Nawrocki
* Zack M. Davis
* 吴冉波
