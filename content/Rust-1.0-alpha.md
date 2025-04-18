+++
path = "2015/01/09/Rust-1.0-alpha"
title = "Announcing Rust 1.0 Alpha"
authors = ["The Rust Core Team"]
aliases = ["2015/01/09/Rust-1.0-alpha.html"]
+++

Today, we're excited to [release](https://www.rust-lang.org/install.html) the alpha version of Rust 1.0, a systems programming language with a focus on safety, performance and concurrency.

This release marks a huge milestone for Rust and its community:

* **The language is feature-complete for 1.0**. While we plan to make many usability
  improvements before the final release, all 1.0 language features are now in place
  and we do not expect major breaking changes to them.

* **The core libraries are feature-complete for 1.0**. API conventions have been
  firmly established, and core functionality -- basic types, traits, data
  structures and concurrency primitives -- has all been
  [stabilized](https://blog.rust-lang.org/2014/10/30/Stability.html). Here again
  we do not expect major breaking changes before 1.0.

The Rust community really rolled up their sleeves to help push this release over the finish line; thanks to everyone who participated, and especially to the [friends of the tree](https://github.com/rust-lang/rust/wiki/Doc-friends-of-the-tree)!

While we've come a long way in [the last four months](https://blog.rust-lang.org/2014/09/15/Rust-1.0.html), there's a lot left to do before Rust hits 1.0 final. Read on for more.

### What happens during the alpha cycle?

If you're already a Rust user, the first thing you'll notice during the alpha cycle is a *dramatic* drop in the pace of breaking changes.

Most features and modules that will ship with Rust 1.0 are in place and will change in only limited ways during this cycle, as detailed later in this post. Only a few modules -- the key ones being path manipulation and I/O -- are still stabilizing.

We need your help to iterate on those final libraries before the beta release, and to hammer out any remaining issues on the parts of Rust we have already marked as stable.  **During the alpha cycle, we recommend using the nightly releases, which will continue to evolve as these APIs take their final shape.**

The alpha release is part of our [transition](https://blog.rust-lang.org/2014/12/12/1.0-Timeline.html) into [stability guarantees](https://blog.rust-lang.org/2014/10/30/Stability.html). While we're not ready to make full stability guarantees yet, this release cycle moves
us much closer to that 1.0 goal. When 1.0-beta1 is released [six weeks from now](https://blog.rust-lang.org/2014/12/12/1.0-Timeline.html), these important remaining APIs will be stable. Code that compiles on the beta release should do so with minimal changes on 1.0 final as well.

### What's shipping in alpha?

Since the [previous release](https://mail.mozilla.org/pipermail/rust-dev/2014-October/011267.html), we've made an enormous amount of progress. We'll cover a
few of the highlights here; the full details are in [the release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md).

#### Language features

* **Dynamically-sized types (DSTs):** [Types whose size is only known at runtime](https://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/) (such as array slices and trait objects) are now largely integrated into the language, including basic integration with user-defined smart pointers. *Implemented by Nick Cameron; rolled out by Jorge Aparicio*.

* **Multidispatch traits:** Trait implementations can now be selected via [multiple types](https://github.com/rust-lang/rfcs/pull/195) (not just `Self`), which has opened the door to [many](https://github.com/rust-lang/rfcs/pull/201) [interesting](https://github.com/rust-lang/rfcs/pull/235) [APIs](https://github.com/rust-lang/rfcs/pull/529). *Implemented by Niko Matsakis; rolled out by Jorge Aparicio and Aaron Turon*.

* **Associated types:** Traits can now have [associated types](https://github.com/rust-lang/rfcs/pull/195), which cuts down on verbosity with generics and aids type inference. *Implemented by Patrick Walton, Niko Matsakis, and Nick Cameron; rolled out by Jorge Aparicio*.

* **Where clauses:** A powerful new way of specifying trait bounds, [where clauses](https://github.com/rust-lang/rfcs/pull/135) have landed and enabled some [significant](https://github.com/rust-lang/rust/pull/20398) [simplifications](https://github.com/rust-lang/rust/pull/20560) in the standard library. *Implemented by Niko Matsakis, Patrick Walton, and Jared Roesch; rolled out by Jorge Aparicio and Aaron Turon.*

* **"Unboxed" closures:** Closures are now [just another way of using the trait system](https://github.com/rust-lang/rfcs/pull/114), which required adding [higher-ranked lifetimes](https://github.com/rust-lang/rfcs/pull/387); this allows much greater flexibility for both ownership and choosing static or dynamic dispatch. *Implemented by Patrick Walton, Brian Koropoff and Niko Matsakis; rolled out by Jorge Aparicio.*

* **Macros:** The `macro_rules` system has been [significantly](https://github.com/rust-lang/rfcs/pull/453) [revised](https://github.com/rust-lang/rfcs/pull/550), cleaning up a number of rough edges and future-proofing in various ways. While macros suffer from some significant deficiencies, the macro system is a powerful and important part of Rust programming that will ship with 1.0. We plan to build a second-generation macro system after the 1.0 release. *Implemented by Keegan McAllister and Corey Richardson.*

* **Integer type changes:** The long-running debate about integer types was [recently resolved](https://github.com/rust-lang/rfcs/pull/544#issuecomment-68760871): `int` and `uint` are now known as `isize` and `usize`, and no longer suggest the role of "default integer" in Rust. *Credit goes to the entire community for the vigorous debate.*

* **Opt-in builtin traits**: The marker traits that Rust uses to classify data as copyable, thread-safe, and so forth have been [significantly refactored to have safer, more predicable defaults](https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md). *Implemented by Patrick Walton and Flavio Percoco Premoli.*

All of these language features have been rolled out throughout the standard library.

#### Library features

* **Consistent conventions:**
[A](https://github.com/rust-lang/rfcs/pull/199)
[large](https://github.com/rust-lang/rfcs/pull/236)
[number](https://github.com/rust-lang/rfcs/pull/240)
[of](https://github.com/rust-lang/rfcs/pull/344)
[conventions](https://github.com/rust-lang/rfcs/pull/430)
[RFCs](https://github.com/rust-lang/rfcs/pull/445)
have been approved, and are largely summarized [in the Rust Guidelines](https://aturon.github.io/); these conventions have been rolled out through all `#[stable]` parts of `std`. *Led by Aaron Turon*.

* **Stable core types and traits:** All of the primitive types and basic building blocks (like `char`, `String`, `Vec`, `Box`, `Arc`, `RefCell` and so on) are now `#[stable]`. *Stabilized by Aaron Turon, Alex Crichton, Brian Anderson, Brendan Zabarauskas and Huon Wilson*.

* **Iterators and collections:** A [series](https://github.com/rust-lang/rfcs/pull/216) [of](https://github.com/rust-lang/rfcs/pull/235) [RFCs](https://github.com/rust-lang/rfcs/pull/509) have been implemented to revamp and stabilize the collections APIs; iterators are also `#[stable]`. *RFCs by Aaron Turon and Alexis Beingessner, stabilized together with Chase Southwood, Piotr Czarnecki, Félix Raimundo, Clark Gaebel and others.*

* **Concurrency primitives:** Rust's concurrency modules have seen an overhaul, including
[thread-local storage](https://github.com/rust-lang/rfcs/pull/461), [synchronization primitives](https://github.com/rust-lang/rust/pull/19274/), and a new [thread API](https://github.com/rust-lang/rust/pull/20615) that will soon allow child threads to share data on their parent's stack. These APIs are more efficient and idiomatic than previous versions, and most are already `#[stable]`. *Implemented by Alex Crichton and Aaron Turon*.

* **Runtime freedom:** Rust's runtime system and green-threading model has been [entirely removed](https://github.com/rust-lang/rfcs/pull/230), which cut the static binary size of "hello world" in half and has [opened the door](https://github.com/rust-lang/rust/pull/19169) to lower-level hooks into the standard library. *Implemented by Aaron Turon*.

#### Documentation

* **The Rust Programming Language:** The previous split between "The Guide" and "The guides" has been rectified by combining them into ["The book"](https://github.com/rust-lang/rust/pull/19897). *Led by Steve Klabnik*.

* **Rust by Example:** The lovely https://rustbyexample.com/ introduction to Rust is now part of the official documentation. *Initiated by Jorge Aparicio*.

* **Additional API documentation:** A lot of work has gone into improving API documentation, both by expanding the overview text and adding examples throughout. *Credit goes to the entire community, who worked tirelessly to improve these docs*.

### What remains to be done?

A detailed list of possible breaking changes to *stable* features/APIs can be found [here](https://github.com/rust-lang/rust/wiki/Anticipated-breaking-changes-during-1.0-alpha). Below is a list of major areas of improvement for the alpha cycle:

* **Improvements to [associated types](https://github.com/rust-lang/rust/issues/17307) and [unboxed closures](https://github.com/rust-lang/rust/issues/18101):** Both of these features are functional, but we plan to make significant ergonomic improvements during the beta cycle, such as [more sophisticated capture-clause inference](https://github.com/rust-lang/rfcs/blob/master/text/0231-upvar-capture-inference.md) and more [uniform support for shorthands like `X::Type`](https://github.com/rust-lang/rust/issues/20300).

* **Improvements to generics:** We will explore [avenues](https://smallcultfollowing.com/babysteps/blog/2014/07/06/implied-bounds/) to cut down on the number of redundant trait bounds that are currently required.

* **Path reform:** The `path` module will soon be [reformed](https://github.com/rust-lang/rfcs/pull/474) to make use of DST, to give a better account of platform differences, and to more closely align with the [new C++ standard](http://web.archive.org/web/20150427231422/http://article.gmane.org/gmane.comp.lib.boost.devel/256220). Implementation work is largely complete, and the rollout should occur soon after alpha.

* **IO reform:** An [overhaul of the IO APIs](https://github.com/rust-lang/rfcs/pull/517/) is being planned; please join in the conversation! These changes will be landing throughout the alpha cycle.

* **Sync/Send changes:** We plan to tweak the definitions of the `Sync` and `Send` markers so that [threads can share stack data](https://github.com/rust-lang/rfcs/pull/458). This may cause some minor breakage.

* **Integer type auditing:** During the alpha cycle, we will finalize formal conventions for choosing integer types and re-audit the use of these types in `std`. This auditing may cause some breakage to `#[stable]` APIs.

* **Documentation for all features:** Some recent feature additions, such as associated types and unboxed closures, have their RFC as the only documentation. User-facing documentation for these features will be added during the alpha cycle. The [reference](https://doc.rust-lang.org/reference.html) will likewise be brought up to date and improved.

* **Complete API documentation:** Everything marked stable will have _at least_ a usage example, and hopefully complete textual explanation, before beta.

And, of course, we will continue to fix bugs and add polish throughout the alpha cycle.

### Contributors to Rust 1.0.0-alpha

This alpha release could not have happened without the help of Rust's enthusiastic community. Thanks go to:

* `Aaron Friel`
* `Aaron Liblong`
* `Aaron Turon`
* `Aaron Weiss`
* `Adam Szkoda`
* `Adolfo Ochagavía`
* `Adrien Tétar`
* `Aidan Cully`
* `A.J. Gardner`
* `Akos Kiss`
* `Aleksandr Koshlo`
* `Alexander Light`
* `Alex Crichton`
* `Alex Gaynor`
* `Alexis Beingessner`
* `Alex Lyon`
* `Alfie John`
* `Andrea Canciani`
* `Andrew Cann`
* `Andrew Paseltiner`
* `Andrew Wagner`
* `areski`
* `Ariel Ben-Yehuda`
* `Artem`
* `Arthur Liao`
* `arturo`
* `Austin Bonander`
* `Barosl Lee`
* `Ben Foppa`
* `Ben Gamari`
* `Ben S`
* `Bheesham Persaud`
* `Björn Steinbrink`
* `bluss`
* `Boris Egorov`
* `bors`
* `Brandon Sanderson`
* `Brendan Zabarauskas`
* `Brian Anderson`
* `Brian J Brennan`
* `Brian Koropoff`
* `Carol Nichols`
* `Chase Southwood`
* `Chris Morgan`
* `Chris Wong`
* `Clark Gaebel`
* `Cody P Schafer`
* `Colin Sherratt`
* `Corey Farwell`
* `Corey Ford`
* `Corey Richardson`
* `crhino`
* `Cristi Burcă`
* `Damien Radtke`
* `Dan Burkert`
* `dan@daramos.com`
* `Daniel Hofstetter`
* `Daniel Micay`
* `Dan Luu`
* `David Creswick`
* `Davis Silverman`
* `Diego Giagio`
* `Dirk Gadsden`
* `Dylan Ede`
* `Earl St Sauver`
* `Eduard Burtescu`
* `Eduardo Bautista`
* `elszben`
* `Eric Allen`
* `Eric Kidd`
* `Erick Tryzelaar`
* `Erwan`
* `Fabrice Desré`
* `FakeKane`
* `Falco Hirschenberger`
* `Felix Raimundo`
* `Felix S. Klock II`
* `Flavio Percoco`
* `Florian Gilcher`
* `Florian Hahn`
* `Florian Wilkens`
* `gamazeps`
* `Gil Cottle`
* `Gleb Kozyrev`
* `Graham Fawcett`
* `Guillaume Pinot`
* `Huon Wilson`
* `Ian Connolly`
* `inrustwetrust`
* `Ivan Petkov`
* `Ivan Ukhov`
* `Jacob Edelman`
* `Jake Goulding`
* `Jakub Bukaj`
* `James Miller`
* `Jared Roesch`
* `Jarod Liu`
* `Jashank Jeremy`
* `Jauhien Piatlicki`
* `jbranchaud`
* `Jeff Parsons`
* `Jelte Fennema`
* `jfager`
* `Jim Apple`
* `jmu303`
* `Johannes Hoff`
* `Johannes Muenzel`
* `John Albietz`
* `John Gallagher`
* `John Kåre Alsaker`
* `John Kleint`
* `Jonathan Reem`
* `Jonathan S`
* `Jon Haddad`
* `JONNALAGADDA Srinivas`
* `Joonas Javanainen`
* `Jorge Aparicio`
* `Joseph Crail`
* `Joseph Rushton Wakeling`
* `Josh Haberman`
* `Josh Stone`
* `Joshua Yanovski`
* `jrincayc`
* `Julian Orth`
* `juxiliary`
* `jxv`
* `Kang Seonghoon`
* `Kasey Carrothers`
* `Keegan McAllister`
* `Ken Tossell`
* `Kevin Ballard`
* `Kevin Mehall`
* `Kevin Yap`
* `klutzy`
* `kulakowski`
* `Laurence Tratt`
* `Liigo Zhuang`
* `Lionel Flandrin`
* `Luke Metz`
* `Luqman Aden`
* `Manish Goregaokar`
* `Markus Siemens`
* `Martin Pool`
* `Marvin Löbel`
* `Matej Lach`
* `Mathieu Poumeyrol`
* `Matt McPherrin`
* `Matt Murphy`
* `Matt Windsor`
* `Maxime Quandalle`
* `Maximilian Haack`
* `Maya Nitu`
* `mchaput`
* `mdinger`
* `Michael Gehring`
* `Michael Neumann`
* `Michael Sproul`
* `Michael Woerister`
* `Mike Dilger`
* `Mike Pedersen`
* `Mike Robinson`
* `Ms2ger`
* `Mukilan Thiyagarajan`
* `Murarth`
* `Nafis`
* `Nathan Zadoks`
* `Neil Pankey`
* `Nicholas Bishop`
* `Nick Cameron`
* `Nick Howell`
* `Niels Egberts`
* `Niko Matsakis`
* `NODA, Kai`
* `OGINO Masanori`
* `oli-obk`
* `Oliver Schneider`
* `olivren`
* `P1start`
* `Pascal Hertleif`
* `Patrick Walton`
* `Paul Collier`
* `Pedro Larroy`
* `Peter Atashian`
* `Peter Elmers`
* `Phil Dawes`
* `Philip Munksgaard`
* `Philipp Gesang`
* `Piotr Czarnecki`
* `Piotr Jawniak`
* `Piotr Szotkowski`
* `qwitwa`
* `Raphael Speyer`
* `Ray Clanan`
* `Richard Diamond`
* `Richo Healey`
* `Ricky Taylor`
* `rjz`
* `Robin Gloster`
* `Robin Stocker`
* `Rohit Joshi`
* `Rolf Timmermans`
* `Rolf van de Krol`
* `Roy Crihfield`
* `Ruud van Asseldonk`
* `Sean Collins`
* `Sean Gillespie`
* `Sean Jensen-Grey`
* `Sean McArthur`
* `Sean T Allen`
* `Seo Sanghyeon`
* `Seth Pollack`
* `sheroze1123`
* `Simonas Kazlauskas`
* `Simon Sapin`
* `Simon Wollwage`
* `Son`
* `Stefan Bucur`
* `Stepan Koltsov`
* `Steve Klabnik`
* `Steven Fackler`
* `Stuart Pernsteiner`
* `Subhash Bhushan`
* `Tamir Duberstein`
* `Taylor Hutchison`
* `th0114nd`
* `thiagopnts`
* `Timon Rapp`
* `Titouan Vervack`
* `Tobias Bucher`
* `Tom Jakubowski`
* `tshakah`
* `Tshepang Lekhonkhobe`
* `Ulysse Carion`
* `Utkarsh Kukreti`
* `Vadim Chugunov`
* `Vadim Petrochenkov`
* `Valerii Hiora`
* `Victor Berger`
* `Victor van den Elzen`
* `Viktor Dahl`
* `ville-h`
* `Vitali Haravy`
* `Vladimir Matveev`
* `Vladimir Smola`
* `we`
* `whataloadofwhat`
* `YawarRaza7349`
* `York Xiang`
* `Zbigniew Siciarz`
* `Ziad Hatahet`
