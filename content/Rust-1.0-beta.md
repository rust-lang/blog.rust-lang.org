+++
path = "2015/04/03/Rust-1.0-beta"
title = "Announcing Rust 1.0 Beta"
authors = ["The Rust Core Team"]
aliases = ["2015/04/03/Rust-1.0-beta.html"]
+++

Today we are excited to announce the [release of Rust 1.0 beta][ru]!
The beta release marks a very significant "state transition" in the
move towards 1.0. In particular, with the beta release, **all
libraries and language features that are planned to be stable for 1.0
have been marked as stable**. As such, the beta release represents an
accurate preview of what Rust 1.0 will include.

To see what has changed since 1.0-alpha2, please see the [release notes][rn].

[rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-100-beta-april-2015

The Beta release also marks a turning point in our
[approach to stability][as]. During the alpha cycle, the use of
unstable APIs and language features was permitted, but triggered a
warning. As of the Beta release, the use of unstable APIs will become
an error (unless you are using Nightly builds or building from
source).

The Rust ecosystem continues to grow. The
[crates.io](https://crates.io/) repository just passed 1 million
downloads and has over 1,700 crates available. Many of the top crates
in [crates.io](https://crates.io/) can now be built using only stable
Rust, and efforts to port the remainder are underway. Therefore, we
are now recommending that new users start with the Beta release,
rather than the Nightly builds, and the [rustup script][ru] will be
modified to install Beta by default. (However, it is easy to switch to
the Nightly build if some of your dependencies aren't updated yet. See
the [install page][ru] for details.)

[ru]: https://www.rust-lang.org/tools/install
[as]: https://blog.rust-lang.org/2014/10/30/Stability.html

### What happens during the beta cycle?

**The final Rust 1.0 release is scheduled for May 15th -- exactly six
weeks from now.** In the interim, we expect to put most of our effort
into fixing bugs, improving documentation and error messages, and
otherwise improving the end-user experience. We don't plan on making
functional changes to stable content, though naturally we may make
minor corrections or additions to the library APIs if shortcomings or
problems are uncovered (but the bar for such changes is relatively
high).

While we don't expect to add any new features (or major new APIs) for
the 1.0 release, that doesn't mean we're going to stop working on them
altogether. In fact, quite the opposite! Per [the train model][tm],
the plan is to continue development on new features on the master
branch, in parallel with the beta. And of course, we'll be issuing the
beta for 1.1 release at the same time as we issue the final 1.0
release, so you shouldn't have to wait long to start putting that work
to use.

To help ensure that we don't accidentally introduce breakage as we add
new features, we've also been working on an exciting new CI
infrastructure to allow us to monitor which packages are building with
the Nightly builds and detect regressions across the entire Rust
ecosystem, not just our own test base. This infrastructure is still in
the development phase, but you can see a [sample report][sr] here.

[tm]: https://blog.rust-lang.org/2014/12/12/1.0-Timeline.html
[sr]: https://gist.github.com/brson/a30a77836fbec057cbee

### A community achievement

As always, this Rust release is the achievement of the fantastic Rust
community at large. Thanks to everyone who has participated in the RFC
process, and a particular thank you to the 170 contributors for this
release:

- `Aaron Turon`
- `Aaron Weiss`
- `Adam Jacob`
- `Adenilson Cavalcanti`
- `Adolfo Ochagavía`
- `Ahmed Charles`
- `Alan Cutter`
- `Alex Crichton`
- `Alexander Bliskovsky`
- `Alexander Campbell`
- `Alexander Chernyakhovsky`
- `Alexis`
- `Alexis Beingessner`
- `Amol Mundayoor`
- `Anders Kaseorg`
- `Andrew Hobden`
- `Andrew Paseltiner`
- `Angus Lees`
- `awlnx`
- `Barosl Lee`
- `bcoopers`
- `Björn Steinbrink`
- `bombless`
- `Brian Anderson`
- `Brian Brooks`
- `Brian Leibig`
- `Camille Roussel`
- `Camille TJHOA`
- `Carol Nichols`
- `Caspar Krieger`
- `Ches Martin`
- `Chloe`
- `Chris Wong`
- `Cody P Schafer`
- `Corey Farwell`
- `Corey Richardson`
- `Dabo Ross`
- `Dan Burkert`
- `Dan Connolly`
- `Dan W.`
- `Daniel Lobato García`
- `Darin Morrison`
- `Darrell Hamilton`
- `Dave Huseby`
- `David Creswick`
- `David King`
- `David Mally`
- `defuz`
- `Denis Defreyne`
- `Drew Crawford`
- `Dzmitry Malyshau`
- `Eduard Bopp`
- `Eduard Burtescu`
- `Eduardo Bautista`
- `Edward Wang`
- `Emeliov Dmitrii`
- `Eric Platon`
- `Erick Tryzelaar`
- `Eunji Jeong`
- `Falco Hirschenberger`
- `Felix S. Klock II`
- `Fenhl`
- `Flavio Percoco`
- `Florian Hahn`
- `Florian Hartwig`
- `Florian Zeitz`
- `FuGangqiang`
- `Gary M. Josack`
- `Germano Gabbianelli`
- `GlacJAY`
- `Gleb Kozyrev`
- `Guillaume Gomez`
- `GuillaumeGomez`
- `Huachao Huang`
- `Huon Wilson`
- `inrustwetrust`
- `Ivan Petkov`
- `Ivan Radanov Ivanov`
- `Jake Goulding`
- `Jakub Bukaj`
- `James Miller`
- `Jessy Diamond Exum`
- `Jihyun Yu`
- `Johannes Oertel`
- `John Hodge`
- `John Zhang`
- `Jonathan Reem`
- `Jordan Woehr`
- `Jorge Aparicio`
- `Joseph Crail`
- `JP-Ellis`
- `Julian Orth`
- `Julian Viereck`
- `Junseok Lee`
- `Kang Seonghoon`
- `Keegan McAllister`
- `Kevin Ballard`
- `Kevin Butler`
- `Kevin Yap`
- `kgv`
- `kjpgit`
- `Lai Jiangshan`
- `Leonids Maslovs`
- `Liam Monahan`
- `Liigo Zhuang`
- `Łukasz Niemier`
- `lummax`
- `Manish Goregaokar`
- `Markus Siemens`
- `Markus Unterwaditzer`
- `Marvin Löbel`
- `Matt Brubeck`
- `Matt Cox`
- `mdinger`
- `Michael Woerister`
- `Michał Krasnoborski`
- `Mihnea Dobrescu-Balaur`
- `Mikhail Zabaluev`
- `Ms2ger`
- `Murarth`
- `Nicholas Bishop`
- `Nicholas Mazzuca`
- `Nicholas`
- `Nick Cameron`
- `Niko Matsakis`
- `nwin`
- `Oliver Schneider`
- `Or Neeman`
- `Pascal Hertleif`
- `Patrick Walton`
- `Paul ADENOT`
- `Paul Osborne`
- `Peter Elmers`
- `Phil Dawes`
- `Philip Munksgaard`
- `Piotr Czarnecki`
- `Pyry Kontio`
- `Raphael Nestler`
- `ray glover`
- `Ricardo Martins`
- `Richard Diamond`
- `Richo Healey`
- `Ruud van Asseldonk`
- `Ryan Prichard`
- `Sae-bom Kim`
- `Scott Olson`
- `Sean McArthur`
- `Sébastien Marie`
- `Seo Sanghyeon`
- `Simonas Kazlauskas`
- `Stepan Koltsov`
- `Steve Klabnik`
- `Steven Crockett`
- `Steven Fackler`
- `Tamir Duberstein`
- `Tero Hänninen`
- `Tiago Nobrega`
- `Tobias Bucher`
- `Tom Jakubowski`
- `Trent Nadeau`
- `Tshepang Lekhonkhobe`
- `Ulrik Sverdrup`
- `Vadim Chugunov`
- `Vadim Petrochenkov`
- `Valerii Hiora`
- `Vladimir Pouzanov`
- `Vojtech Kral`
- `Wangshan Lu`
- `Wesley Wiser`
- `York Xiang`
