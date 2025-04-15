+++
path = "2015/02/20/Rust-1.0-alpha2"
title = "Announcing Rust 1.0.0.alpha.2"
authors = ["Steve Klabnik"]
description = "Rust 1.0.0.alpha.2 has been released."
aliases = ["2015/02/20/Rust-1.0-alpha2.html"]
+++

Today, we are happy to announce the release of Rust 1.0.0.alpha.2! Rust is a
systems programming language pursuing the trifecta: safe, fast, and concurrent.

In accordance with our [status report](/2015/02/13/Final-1.0-timeline/)
last week, this is a second alpha release, rather than a first beta release.
The beta release will be six weeks from today, with the final coming six weeks
after that.

We’ve managed to land almost all of the features previously expected for this
cycle. **The big headline here is that all major API revisions are finished**:
path and IO reform have landed. At this point, all modules shipping for 1.0 are
in what we expect to be their final form, modulo minor tweaks during the alpha2
cycle. See the [previous post](/2015/02/13/Final-1.0-timeline/) for more
details.

This coming release cycle is crucial to Rust, as this will be the last cycle
that we recommend nightly builds for most users. Part of the way through the
cycle, around March 9th, we expect to have all major functionality we expect in
1.0 marked as stable; we will fill in stability gaps between then and beta on
March 31st. The beta release will fully introduce the "stable channel", which
will not allow use of unstable features but will guarantee
backwards-compatibility (after 1.0). Unstable features will only be available
in nightly.

For more detail, please see the [Release
notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-100-alpha2-february-2015). 

Thank you to all 201 contributors for this release:

* `Aaron Turon`
* `Adam Roben`
* `Adolfo Ochagavía`
* `Ahmed Charles`
* `Aidan Hobson Sayers`
* `Akos Kiss`
* `Alexander Bliskovsky`
* `Alexander Korolkov`
* `Alexander Light`
* `Alex Crichton`
* `Alexis`
* `Alfie John`
* `Andrea Canciani`
* `Andrew Barchuk`
* `Andrew Paseltiner`
* `Ariel Ben-Yehuda`
* `Armin Preiml`
* `Artem`
* `Barosl Lee`
* `Benjamin Peterson`
* `Ben S`
* `Björn Steinbrink`
* `blackbeam`
* `Brian Anderson`
* `Brian Leibig`
* `caipre`
* `Cam Jackson`
* `Carl Lerche`
* `Carol Nichols`
* `Carter Hinsley`
* `CarVac`
* `Caspar Krieger`
* `Chase Southwood`
* `Chris Morgan`
* `Chris Thorn`
* `Chris Wong`
* `Clifford Caoile`
* `Corey Farwell`
* `Corey Richardson`
* `Daniel Griffen`
* `Daniel Grunwald`
* `Daniel Raloff`
* `Daniil Smirnov`
* `Dan Yang`
* `David Creswick`
* `Diggory Blake`
* `Dominik Inführ`
* `Duane Edwards`
* `Duncan Regan`
* `Dzmitry Malyshau`
* `Earl St Sauver`
* `Eduard Burtescu`
* `Edward Wang`
* `Elantsev Serj`
* `emanueLczirai`
* `Erick Rivas`
* `Erick Tryzelaar`
* `Eunji Jeong`
* `Felix S. Klock II`
* `Fenhl`
* `Filip Szczepański`
* `Flavio Percoco`
* `Florian Hahn`
* `Garrett Heel`
* `Geoffrey Thomas`
* `Greg Chapple`
* `Guillaume Gomez`
* `Guillaume Pinot`
* `Henrik Schopmans`
* `Hugo van der Wijst`
* `Huon Wilson`
* `Ignacio Corderi`
* `Ingo Blechschmidt`
* `Jake Goulding`
* `James Miller`
* `Jared Roesch`
* `Jason Fager`
* `jatinn`
* `Jay True`
* `Jeff Belgum`
* `John Hodge`
* `John Kåre Alsaker`
* `Jonathan Reem`
* `JONNALAGADDA Srinivas`
* `Jorge Aparicio`
* `Jorge Israel Peña`
* `Jormundir`
* `Joseph Crail`
* `JP Sugarbroad`
* `Julian Orth`
* `Junseok Lee`
* `Kang Seonghoon`
* `Keegan McAllister`
* `Ken Tossell`
* `KernelJ`
* `Kevin Ballard`
* `Kevin Butler`
* `Kevin Yap`
* `Kim Røen`
* `klutzy`
* `Kostas Karachalios`
* `kud1ing`
* `Lai Jiangshan`
* `Lauri Lehmijoki`
* `Leo Testard`
* `Liigo Zhuang`
* `Logan Chien`
* `Loïc Damien`
* `Luca Bruno`
* `Luke Francl`
* `Luke Steensen`
* `madmalik`
* `Manish Goregaokar`
* `Markus Siemens`
* `Marvin Löbel`
* `Matt Roche`
* `Mátyás Mustoha`
* `mdinger`
* `Michael Budde`
* `Michael Neumann`
* `Michael Pankov`
* `Michael Sproul`
* `Michael Woerister`
* `Mike English`
* `Mikhail Zabaluev`
* `Ms2ger`
* `NAKASHIMA, Makoto`
* `nathan dotz`
* `Nathaniel Theis`
* `Nathan Stoddard`
* `Nelson Chen`
* `Nick Cameron`
* `Nick Howell`
* `Nick Sarten`
* `Niko Matsakis`
* `NODA, Kai`
* `Oliver 'ker' Schneider`
* `Oliver Schneider`
* `Orpheus Lummis`
* `P1start`
* `Pascal Hertleif`
* `Paul Collier`
* `Paul Crowley`
* `Peter Atashian`
* `Peter Schuller`
* `Pierre Baillet`
* `Piotr Czarnecki`
* `posixphreak`
* `Potpourri`
* `Pyfisch`
* `Raul Gutierrez S`
* `Renato Alves`
* `Renato Zannon`
* `Richo Healey`
* `Robin Stocker`
* `Rohit Joshi`
* `Ryan Levick`
* `Sean Collins`
* `Sean Gillespie`
* `Sean Patrick Santos`
* `Sean T Allen`
* `Sebastian Gesemann`
* `Sebastian Rasmussen`
* `Sébastien Marie`
* `Seo Sanghyeon`
* `Seth Faxon`
* `Simonas Kazlauskas`
* `Stepan Koltsov`
* `Steve Klabnik`
* `Steven Allen`
* `Steven Crockett`
* `Steven Fackler`
* `Strahinja Val Markovic`
* `Thiago Carvalho`
* `Tim Brooks`
* `Tim Cuthbertson`
* `Tim Dumol`
* `Tim Parenti`
* `Tobias Bucher`
* `Toby Scrace`
* `Tom Chittenden`
* `Tom Jakubowski`
* `Toni Cárdenas`
* `Travis Watkins`
* `Tristan Storch`
* `Tshepang Lekhonkhobe`
* `Tyler Thrailkill`
* `Ulrik Sverdrup`
* `Vadim Chugunov`
* `Vadim Petrochenkov`
* `Valerii Hiora`
* `Victory`
* `visualfc`
* `Vojtech Kral`
* `Volker Mische`
* `Wangshan Lu`
* `we`
* `Willson Mock`
* `Will`
* `wonyong kim`
* `York Xiang`
