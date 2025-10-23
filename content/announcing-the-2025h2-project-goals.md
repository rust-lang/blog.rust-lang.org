+++
path = "9999/12/31/announcing-the-2025h2-project-goals"
title = "Project goals for 2025H2"
authors = ["Niko Matsakis"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

On Sep 9, we merged [RFC 3849](https://github.com/rust-lang/rfcs/pull/3849), declaring our goals for the "second half" of 2025H2 -- well, the last 3 months, at least, since "yours truly" ran a bit behind getting goals program organized.

## Flagship themes

In prior goals programs we had a few major flagship goals, but since many of these goals were multi-year programs, it made it hard to see what progress had been made. This time we decided to organize things a bit differently. We established four flagship *themes*, each of which covers a number of more specific goals. These themes cover the goals we expect to be the most impactful and constitute our major focus as a project for the remainder of the year. The four themes identified in the RFC are as follows:

* **Beyond the `&`**, making it possible to create user-defined smart pointers that are as ergonomic as Rust's built-in references `&`.
* **Unblocking dormant traits**, extending the core capabilities of Rust's trait system to unblock long-desired features for language interop, lending iteration, and more.
* **Flexible, fast(er) compilation**, making it faster to build Rust programs and improving support for specialized build scenarios like embedded usage and sanitizers.
* **Higher-level Rust**, making higher-level usage patterns in Rust easier.

### "Beyond the `&`"

| Goal                                                                         | Point of contact | Team(s) and Champion(s)                      |
| :--                                                                          | :--          | :--                                          |
| [Reborrow traits](https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html)                                    | [Aapo Alasuutari][]    | [compiler] ([Oliver Scherer][]), [lang] ([Tyler Mandry][])     |
| [Design a language feature to solve Field Projections](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html) | [Benno Lossin][] | [lang] ([Tyler Mandry][])                            |
| [Continue Experimentation with Pin Ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html)            | [Frank King][]  | [compiler] ([Oliver Scherer][]), [lang] ([TC][]) |


One of Rust's core value propositions is that it's a "library-based language"—libraries can build abstractions that feel built-in to the language even when they're not. Smart pointer types like `Rc` and `Arc` are prime examples, implemented purely in the standard library yet feeling like native language features. However, Rust's built-in reference types (`&T` and `&mut T`) have special capabilities that user-defined smart pointers cannot replicate. This creates a "second-class citizen" problem where custom pointer types can't provide the same ergonomic experience as built-in references.

The "Beyond the `&`" initiative aims to share `&`'s special capabilities, allowing library authors to create smart pointers that are truly indistinguishable from built-in references in terms of syntax and ergonomics. This will enable more ergonomic smart pointers for use in cross-language interop (e.g., references to objects in other languages like C++ or Python) and for low-level projects like Rust for Linux which use smart pointers to express particular data structures.

### "Unblocking dormant traits"

| Goal                                                    | Point of contact | Team(s) and Champion(s)                                        |
| :--                                                     | :--       | :--                                                            |
| [Evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/evolving-traits.html)        | [Taylor Cramer][] | [compiler], [lang] ([Taylor Cramer][]), [libs-api], [types] ([Oliver Scherer][]) |
| [In-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html)   | [Alice Ryhl][] | [lang] ([Taylor Cramer][])                                             |
| [Next-generation trait solver](https://rust-lang.github.io/rust-project-goals/2025h2/next-solver.html)          | [lcnr][]     | [types] ([lcnr][])                                                |
| [Stabilizable Polonius support on nightly](https://rust-lang.github.io/rust-project-goals/2025h2/polonius.html) | [Rémy Rakic][]      | [types] ([Jack Huey][])                                            |
| [SVE and SME on AArch64](https://rust-lang.github.io/rust-project-goals/2025h2/scalable-vectors.html) | [David Wood][]       | [compiler] ([David Wood][]), [lang] ([Niko Matsakis][]), [libs] ([Amanieu d'Antras][]), [types]           |


Rust's trait system is one of its most powerful features, but it has a number of longstanding limitations that are preventing us from adopting new patterns. The goals in this category unblock a number of new capabilities:

* [Polonius](https://rust-lang.github.io/rust-project-goals/2025h2/./polonius.html) will enable new borrowing patterns, and in particular [unblock "lending iterators"](https://github.com/rust-lang/rust/issues/92985). Over the last few goal periods we have identified an "alpha" version of polonius that addresses the most important cases while being relatively simple and optimizable. Our goal for 2025H2 is to implement this algorithm in a form that is ready for stabilization in 2026.
* The [next gen trait solver](https://rust-lang.github.io/rust-project-goals/2025h2/./next-solver.html) is a refactored trait solver that unblocks better support for numerous language features (implied bounds, negative impls, the list goes on) in addition to closing a number of existing bugs and unsoundnesses. Over the last few goal periods, the trait solver went from early prototype to being production use in coherence. The goal for 2025H2 is to prepare it for stabilization.
* The work on [evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/./evolving-traits.html) will make it possible to refactor some parts of an existing trait out into a new supertrait so they can be used on their own. This unblocks a number of features where the existing trait is insufficiently general, in particular stabilizing support for custom receiver types, a prior project goal that wound up blocking on this refactoring. This will also make it safer to provide stable traits in the standard library, while preserving the ability to evolve them in the future.
* The work to [expand Rust's `Sized` hierarchy](https://rust-lang.github.io/rust-project-goals/2025h2/./scalable-vectors.html) will permit us to express types that are neither `Sized` nor `?Sized`, such as extern types (which have no size) or Arm's Scalable Vector Extension (which have a size that is known at runtime, but not compilation time). This goal builds on [RFC #3729](https://github.com/rust-lang/rfcs/pull/3729) and [RFC #3838](https://github.com/rust-lang/rfcs/pull/3838), authored in previous project goal periods.
* [In-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/./in-place-initialization.html) allows creating structs and values that are tied to a particular place in memory. While useful directly for projects doing advanced C interop, it also unblocks expanding `dyn Trait` to support for `async fn` and `-> impl Trait` methods, as compiling such methods requires the ability for the callee to return a future whose size is not known to the caller.

### "Flexible, fast(er) compilation"

| Goal                                                                | Point of contact | Team(s) and Champion(s)                                      |
| :--                                                                 | :--         | :--                                                          |
| [build-std](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html)                                           | [David Wood][]  | [cargo] ([Eric Huss][]), [compiler] ([David Wood][]), [libs] ([Amanieu d'Antras][]) |
| [Promoting Parallel Front End](https://rust-lang.github.io/rust-project-goals/2025h2/parallel-front-end.html)               | [Sparrow Li][] | [compiler]                                                   |
| [Production-ready cranelift backend](https://rust-lang.github.io/rust-project-goals/2025h2/production-ready-cranelift.html) | [Folkert de Vries][] | [compiler], [wg-compiler-performance]                        |


The "Flexible, fast(er) compilation" initiative focuses on improving Rust's build system to better serve both specialized use cases and everyday development workflows:

* We are improving compilation performance through (1) [parallel compilation in the compiler front-end](https://rust-lang.github.io/rust-project-goals/2025h2/./parallel-front-end.html), which delivers 20-30% faster builds, and (2) [making the Cranelift backend production-ready for development use](https://rust-lang.github.io/rust-project-goals/2025h2/./production-ready-cranelift.html), offering roughly 20% faster code generation compared to LLVM for debug builds.
* We are working to [stabilize a core MVP of the `-Zbuild-std` feature](https://rust-lang.github.io/rust-project-goals/2025h2/./build-std.html), which allows developers to rebuild the standard library from source with custom compiler flags. This unblocks critical use cases for embedded developers and low-level projects like Rust for Linux, while also enabling improvements like using sanitizers with the standard library or building `std` with debug information.

### "Higher-level Rust"

| Goal                                                                | Point of contact | Team(s) and Champion(s)                                                           |
| :--                                                                 | :--           | :--                                                                               |
| [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)                           | [Ed Page][]        | [cargo] ([Ed Page][]), [compiler], [lang] ([Josh Triplett][]), [lang-docs] ([Josh Triplett][]) |
| [Ergonomic ref-counting: RFC decision and preview](https://rust-lang.github.io/rust-project-goals/2025h2/ergonomic-rc.html) | [Niko Matsakis][] | [compiler] ([Santiago Pastorino][]), [lang] ([Niko Matsakis][])                                  |


People generally start using Rust for foundational use cases, where the requirements for performance or reliability make it an obvious choice. But once they get used to it, they often find themselves turning to Rust even for higher-level use cases, like scripting, web services, or even GUI applications. Rust is often "surprisingly tolerable" for these high-level use cases -- except for some specific pain points that, while they impact everyone using Rust, hit these use cases particularly hard. We plan two flagship goals this period in this area:

* We aim to stabilize [cargo script](https://rust-lang.github.io/rust-project-goals/2025h2/./cargo-script.html), a feature that allows single-file Rust programs that embed their dependencies, making it much easier to write small utilities, share code examples, and create reproducible bug reports without the overhead of full Cargo projects.
* We aim to finalize the design of [ergonomic ref-counting](https://rust-lang.github.io/rust-project-goals/2025h2/./ergonomic-rc.html) and to finalize the experimental impl feature so it is ready for beta testing. Ergonomic ref counting makes it less cumbersome to work with ref-counted types like `Rc` and `Arc`, particularly in closures.

## What to expect next

For the remainder of 2025 you can expect monthly blog posts covering the major progress on the project goals.

Looking at the broader picture, we have now done 3 iterations of the goals program and we want to judge how it should be run going forward. To start, Nandini Sharma from CMU has been conducting interviews with various project members to help us see what's working with the goals program and what could be improved. We expect to spend some time discussing what we should do and to be launching the next iteration of the goals program next year. Whatever form that winds up taking, Tomas Sedovic, the [Rust program manager](https://blog.rust-lang.org/inside-rust/2025/06/30/program-management-update-2025-06/) hired by the Leadership Council will join me in running the program.

 # Appendix: Full list of project goals.

<details>
<summary>Read the full slate of Rust project goals.</summary>

The full slate of project goals are as follows. These goals all have identified points of contact who will drive the work forward as well as a viable work plan. 

**Invited goals.** Some goals of the goals below are "invited goals", meaning that for that goal to happen we need someone to step up and serve as a point of contact. To find the invited goals, look for the **"Help wanted"** badge in the table below. Invited goals have reserved capacity for teams and a mentor, so if you are someone looking to help Rust progress, they are a great way to get involved.

| Goal                                                                                                       | Point of contact | Team(s) and Champion(s)                                                               |
| :--                                                                                                        | :--              | :--                                                                                   |
| [Develop the capabilities to keep the FLS up to date](https://rust-lang.github.io/rust-project-goals/2025h2/FLS-up-to-date-capabilities.html)                      | [Pete LeVasseur][]      | [bootstrap] ([Jakub Beránek][]), [lang] ([Niko Matsakis][]), [opsem], [spec] ([Pete LeVasseur][]), [types] |
| [Getting Rust for Linux into stable Rust: compiler features](https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-compiler.html)                   | [Tomas Sedovic][]    | [compiler] ([Wesley Wiser][])                                                             |
| [Getting Rust for Linux into stable Rust: language features](https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-language.html)                   | [Tomas Sedovic][]    | [lang] ([Josh Triplett][]), [lang-docs] ([TC][])                                    |
| [Borrow checking in a-mir-formality](https://rust-lang.github.io/rust-project-goals/2025h2/a-mir-formality.html)                                                   | [Niko Matsakis][]    | [types] ([Niko Matsakis][])                                                               |
| [Reborrow traits](https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html)                                                                  | [Aapo Alasuutari][]        | [compiler] ([Oliver Scherer][]), [lang] ([Tyler Mandry][])                                              |
| [build-std](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html)                                                                                  | [David Wood][]       | [cargo] ([Eric Huss][]), [compiler] ([David Wood][]), [libs] ([Amanieu d'Antras][])                          |
| [Prototype Cargo build analysis](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-analysis.html)                                                  | [Weihang Lo][]       | [cargo] ([Weihang Lo][])                                                                  |
| [Rework Cargo Build Dir Layout](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-build-dir-layout.html)                                                 | [Ross Sullivan][]     | [cargo] ([Weihang Lo][])                                                                  |
| [Prototype a new set of Cargo "plumbing" commands](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-plumbing.html)                                      | ![Help Wanted][] | [cargo]                                                                               |
| [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)                                                                  | [Ed Page][]           | [cargo] ([Ed Page][]), [compiler], [lang] ([Josh Triplett][]), [lang-docs] ([Josh Triplett][])     |
| [Continue resolving `cargo-semver-checks` blockers for merging into cargo](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-semver-checks.html)         | [Predrag Gruevski][]      | [cargo] ([Ed Page][]), [rustdoc] ([Alona Enraght-Moony][])                                          |
| [Emit Retags in Codegen](https://rust-lang.github.io/rust-project-goals/2025h2/codegen_retags.html)                                                                | [Ian McCormack][]        | [compiler] ([Ralf Jung][]), [opsem] ([Ralf Jung][])                                           |
| [Comprehensive niche checks for Rust](https://rust-lang.github.io/rust-project-goals/2025h2/comprehensive-niche-checks.html)                                       | [Bastian Kersting][]          | [compiler] ([Ben Kimock][]), [opsem] ([Ben Kimock][])                                           |
| [Const Generics](https://rust-lang.github.io/rust-project-goals/2025h2/const-generics.html)                                                                        | [Boxy][]         | [lang] ([Niko Matsakis][])                                                                |
| [Ergonomic ref-counting: RFC decision and preview](https://rust-lang.github.io/rust-project-goals/2025h2/ergonomic-rc.html)                                        | [Niko Matsakis][]    | [compiler] ([Santiago Pastorino][]), [lang] ([Niko Matsakis][])                                      |
| [Evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/evolving-traits.html)                                                           | [Taylor Cramer][]        | [compiler], [lang] ([Taylor Cramer][]), [libs-api], [types] ([Oliver Scherer][])                        |
| [Design a language feature to solve Field Projections](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html)                               | [Benno Lossin][]     | [lang] ([Tyler Mandry][])                                                                     |
| [Finish the std::offload module](https://rust-lang.github.io/rust-project-goals/2025h2/finishing-gpu-offload.html)                                                 | [Manuel Drehwald][]          | [compiler] ([Manuel Drehwald][]), [lang] ([TC][])                                           |
| [Run more tests for GCC backend in the Rust's CI](https://rust-lang.github.io/rust-project-goals/2025h2/gcc-backend-tests.html)                                    | [Guillaume Gomez][]  | [compiler] ([Wesley Wiser][]), [infra] ([Marco Ieni][])                                       |
| [In-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html)                                                      | [Alice Ryhl][]        | [lang] ([Taylor Cramer][])                                                                    |
| [C++/Rust Interop Problem Space Mapping](https://rust-lang.github.io/rust-project-goals/2025h2/interop-problem-map.html)                                           | [Jon Bauman][]         | [compiler] ([Oliver Scherer][]), [lang] ([Tyler Mandry][]), [libs] ([David Tolnay][]), [opsem]                  |
| [Finish the libtest json output experiment](https://rust-lang.github.io/rust-project-goals/2025h2/libtest-json.html)                                               | [Ed Page][]           | [cargo] ([Ed Page][]), [libs-api], [testing-devex]                                         |
| [MIR move elimination](https://rust-lang.github.io/rust-project-goals/2025h2/mir-move-elimination.html)                                                            | [Amanieu d'Antras][]         | [compiler], [lang] ([Amanieu d'Antras][]), [opsem], [wg-mir-opt]                                  |
| [Next-generation trait solver](https://rust-lang.github.io/rust-project-goals/2025h2/next-solver.html)                                                             | [lcnr][]            | [types] ([lcnr][])                                                                       |
| [Implement Open API Namespace Support](https://rust-lang.github.io/rust-project-goals/2025h2/open-namespaces.html)                                                 | ![Help Wanted][] | [cargo] ([Ed Page][]), [compiler] ([b-naber][]), [crates-io] ([Carol Nichols][])                 |
| [Promoting Parallel Front End](https://rust-lang.github.io/rust-project-goals/2025h2/parallel-front-end.html)                                                      | [Sparrow Li][]      | [compiler]                                                                            |
| [Continue Experimentation with Pin Ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html)                                          | [Frank King][]      | [compiler] ([Oliver Scherer][]), [lang] ([TC][])                                          |
| [Stabilizable Polonius support on nightly](https://rust-lang.github.io/rust-project-goals/2025h2/polonius.html)                                                    | [Rémy Rakic][]             | [types] ([Jack Huey][])                                                                   |
| [Production-ready cranelift backend](https://rust-lang.github.io/rust-project-goals/2025h2/production-ready-cranelift.html)                                        | [Folkert de Vries][]      | [compiler], [wg-compiler-performance]                                                 |
| [Stabilize public/private dependencies](https://rust-lang.github.io/rust-project-goals/2025h2/pub-priv.html)                                                       | ![Help Wanted][] | [cargo] ([Ed Page][]), [compiler]                                                          |
| [Expand the Rust Reference to specify more aspects of the Rust language](https://rust-lang.github.io/rust-project-goals/2025h2/reference-expansion.html)           | [Josh Triplett][]    | [lang-docs] ([Josh Triplett][]), [spec] ([Josh Triplett][])                                   |
| [reflection and comptime](https://rust-lang.github.io/rust-project-goals/2025h2/reflection-and-comptime.html)                                                      | [Oliver Scherer][]         | [compiler] ([Oliver Scherer][]), [lang] ([Scott McMurray][]), [libs] ([Josh Triplett][])                     |
| [Relink don't Rebuild](https://rust-lang.github.io/rust-project-goals/2025h2/relink-dont-rebuild.html)                                                             | [Jane Lusby][]           | [cargo], [compiler]                                                                   |
| [Rust Vision Document](https://rust-lang.github.io/rust-project-goals/2025h2/rust-vision-doc.html)                                                                 | [Niko Matsakis][]    | [leadership-council]                                                                  |
| [rustc-perf improvements](https://rust-lang.github.io/rust-project-goals/2025h2/rustc-perf-improvements.html)                                                      | [James][]    | [compiler], [infra]                                                                   |
| [Stabilize rustdoc `doc_cfg` feature](https://rust-lang.github.io/rust-project-goals/2025h2/rustdoc-doc-cfg.html)                                                  | [Guillaume Gomez][]  | [rustdoc] ([Guillaume Gomez][])                                                           |
| [Add a team charter for rustdoc team](https://rust-lang.github.io/rust-project-goals/2025h2/rustdoc-team-charter.html)                                             | [Guillaume Gomez][]  | [rustdoc] ([Guillaume Gomez][])                                                           |
| [SVE and SME on AArch64](https://rust-lang.github.io/rust-project-goals/2025h2/scalable-vectors.html)                                                              | [David Wood][]       | [compiler] ([David Wood][]), [lang] ([Niko Matsakis][]), [libs] ([Amanieu d'Antras][]), [types]           |
| [Rust Stabilization of MemorySanitizer and ThreadSanitizer Support](https://rust-lang.github.io/rust-project-goals/2025h2/stabilization-of-sanitizer-support.html) | [Jakob Koschel][]       | [bootstrap], [compiler], [infra], [project-exploit-mitigations]                       |
| [Type System Documentation](https://rust-lang.github.io/rust-project-goals/2025h2/typesystem-docs.html)                                                            | [Boxy][]         | [types] ([Boxy][])                                                                    |
| [Unsafe Fields](https://rust-lang.github.io/rust-project-goals/2025h2/unsafe-fields.html)                                                                          | [Jack Wrenn][]         | [compiler] ([Jack Wrenn][]), [lang] ([Scott McMurray][])                                             |

</details>

<!-- Github usernames -->

[all]: https://www.rust-lang.org/governance/teams
[alumni]: https://www.rust-lang.org/governance/teams
[android]: https://www.rust-lang.org/governance/teams
[apple]: https://www.rust-lang.org/governance/teams
[arewewebyet]: https://www.rust-lang.org/governance/teams
[arm]: https://www.rust-lang.org/governance/teams
[arm-maintainers]: https://www.rust-lang.org/governance/teams
[book]: https://github.com/rust-lang/book
[bootstrap]: https://github.com/rust-lang/rust
[cargo]: https://github.com/rust-lang/cargo
[clippy]: https://github.com/rust-lang/rust-clippy
[clippy-contributors]: https://github.com/rust-lang/rust-clippy
[cloud-compute]: https://www.rust-lang.org/governance/teams
[codegen-c-maintainers]: https://github.com/rust-lang/rustc_codegen_c
[community]: https://github.com/rust-community/team
[community-content]: https://github.com/rust-community/content-team
[community-events]: https://github.com/rust-community/events-team
[community-localization]: https://github.com/rust-lang/community-localization
[community-rustbridge]: https://github.com/rustbridge/team
[community-survey]: https://github.com/rust-lang/surveys
[compiler]: http://github.com/rust-lang/compiler-team
[compiler-fcp]: http://github.com/rust-lang/compiler-team
[compiler-ops]: https://www.rust-lang.org/governance/teams
[content]: https://github.com/rust-lang/content-team
[cookbook]: https://github.com/rust-lang-nursery/rust-cookbook/
[council-librarians]: https://www.rust-lang.org/governance/teams
[crate-maintainers]: https://www.rust-lang.org/governance/teams
[crates-io]: https://github.com/rust-lang/crates.io
[crates-io-admins]: https://www.rust-lang.org/governance/teams
[crates-io-infra-admins]: https://www.rust-lang.org/governance/teams
[crates-io-on-call]: https://www.rust-lang.org/governance/teams
[devtools]: https://github.com/rust-dev-tools/dev-tools-team
[docker]: https://github.com/rust-lang/docker-rust/
[docs-rs]: https://github.com/rust-lang/docs.rs
[docs-rs-reviewers]: https://github.com/rust-lang/docs.rs
[edition]: http://github.com/rust-lang/edition-team
[emacs]: https://www.rust-lang.org/governance/teams
[emscripten]: https://www.rust-lang.org/governance/teams
[expect-test]: https://www.rust-lang.org/governance/teams
[foundation-board-project-directors]: https://www.rust-lang.org/governance/teams
[foundation-email-redirects]: https://www.rust-lang.org/governance/teams
[fuchsia]: https://www.rust-lang.org/governance/teams
[goal-owners]: https://www.rust-lang.org/governance/teams
[goals]: https://github.com/rust-lang/rust-project-goals
[gsoc-contributors]: https://www.rust-lang.org/governance/teams
[hiring]: https://www.rust-lang.org/governance/teams
[infra]: https://github.com/rust-lang/infra-team
[infra-admins]: https://www.rust-lang.org/governance/teams
[infra-bors]: https://github.com/rust-lang/bors
[inside-rust-reviewers]: https://www.rust-lang.org/governance/teams
[lang]: http://github.com/rust-lang/lang-team
[lang-advisors]: https://www.rust-lang.org/governance/teams
[lang-docs]: https://www.rust-lang.org/governance/teams
[lang-ops]: https://www.rust-lang.org/governance/teams
[launching-pad]: https://www.rust-lang.org/governance/teams
[leadership-council]: https://github.com/rust-lang/leadership-council
[leads]: https://www.rust-lang.org/governance/teams
[libs]: https://github.com/rust-lang/libs-team
[libs-api]: https://www.rust-lang.org/governance/teams
[libs-contributors]: https://www.rust-lang.org/governance/teams
[loongarch]: https://www.rust-lang.org/governance/teams
[mentors]: https://www.rust-lang.org/governance/teams
[mentorship]: https://www.rust-lang.org/governance/teams
[miri]: https://github.com/rust-lang/miri
[mods]: https://github.com/rust-lang/moderation-team
[mods-discourse]: https://www.rust-lang.org/governance/teams
[mods-venue]: https://www.rust-lang.org/governance/teams
[opsem]: https://github.com/rust-lang/opsem-team
[ospp]: https://www.rust-lang.org/governance/teams
[ospp-contributors]: https://www.rust-lang.org/governance/teams
[project-async-crashdump-debugging]: https://github.com/rust-lang/async-crashdump-debugging-initiative
[project-const-generics]: https://github.com/rust-lang/project-const-generics
[project-const-traits]: https://github.com/rust-lang/project-const-traits
[project-dyn-upcasting]: https://github.com/rust-lang/dyn-upcasting-coercion-initiative
[project-exploit-mitigations]: https://github.com/rust-lang/project-exploit-mitigations
[project-generic-associated-types]: https://github.com/rust-lang/generic-associated-types-initiative
[project-goal-reference-expansion]: https://www.rust-lang.org/governance/teams
[project-group-leads]: https://www.rust-lang.org/governance/teams
[project-impl-trait]: https://github.com/rust-lang/impl-trait-initiative
[project-keyword-generics]: https://github.com/rust-lang/keyword-generics-initiative
[project-negative-impls]: https://github.com/rust-lang/negative-impls-initiative
[project-portable-simd]: https://www.rust-lang.org/governance/teams
[project-stable-mir]: https://github.com/rust-lang/project-stable-mir
[project-trait-system-refactor]: https://github.com/rust-lang/types-team
[project-vision-doc-2025]: https://github.com/rust-lang/vision-doc-2025
[regex]: https://github.com/rust-lang/regex
[release]: https://github.com/rust-lang/release-team
[release-publishers]: https://github.com/rust-lang/release-team
[relnotes-interest-group]: https://www.rust-lang.org/governance/teams
[risc-v]: https://www.rust-lang.org/governance/teams
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[rust-analyzer-contributors]: https://github.com/rust-lang/rust-analyzer
[rust-by-example]: https://github.com/rust-lang/rust-by-example
[rust-for-linux]: https://www.rust-lang.org/governance/teams
[rustconf-emails]: https://www.rust-lang.org/governance/teams
[rustdoc]: https://github.com/rust-lang/rust
[rustdoc-frontend]: https://www.rust-lang.org/governance/teams
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustlings]: https://github.com/rust-lang/rustlings/
[rustup]: https://github.com/rust-lang/rustup
[social-media]: https://www.rust-lang.org/governance/teams
[spec]: https://github.com/rust-lang/spec
[spec-contributors]: https://github.com/rust-lang/spec
[style]: https://github.com/rust-lang/style-team
[team-repo-admins]: https://www.rust-lang.org/governance/teams
[testing-devex]: https://www.rust-lang.org/governance/teams
[triagebot]: https://github.com/rust-lang/triagebot
[twir]: https://github.com/rust-lang/this-week-in-rust
[twir-reviewers]: https://github.com/rust-lang/this-week-in-rust
[types]: https://github.com/rust-lang/types-team
[types-fcp]: https://github.com/rust-lang/types-team
[vim]: https://www.rust-lang.org/governance/teams
[wasi]: https://www.rust-lang.org/governance/teams
[wasm]: https://www.rust-lang.org/governance/teams
[web-presence]: https://www.rust-lang.org/governance/teams
[website]: https://github.com/rust-lang/www.rust-lang.org/
[wg-allocators]: https://github.com/rust-lang/wg-allocators
[wg-async]: https://github.com/rust-lang/wg-async
[wg-bindgen]: https://github.com/rust-lang/rust-bindgen
[wg-cli]: https://www.rust-lang.org/governance/teams
[wg-compiler-performance]: https://github.com/rust-lang/rustc-perf
[wg-const-eval]: https://github.com/rust-lang/const-eval
[wg-diagnostics]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-embedded]: https://github.com/rust-embedded/wg
[wg-embedded-arm]: https://www.rust-lang.org/governance/teams
[wg-embedded-core]: https://www.rust-lang.org/governance/teams
[wg-embedded-hal]: https://www.rust-lang.org/governance/teams
[wg-embedded-infra]: https://www.rust-lang.org/governance/teams
[wg-embedded-libs]: https://www.rust-lang.org/governance/teams
[wg-embedded-linux]: https://www.rust-lang.org/governance/teams
[wg-embedded-msp430]: https://www.rust-lang.org/governance/teams
[wg-embedded-resources]: https://www.rust-lang.org/governance/teams
[wg-embedded-riscv]: https://www.rust-lang.org/governance/teams
[wg-embedded-tools]: https://www.rust-lang.org/governance/teams
[wg-embedded-triage]: https://www.rust-lang.org/governance/teams
[wg-ffi-unwind]: https://github.com/rust-lang/project-ffi-unwind
[wg-gamedev]: https://github.com/rust-gamedev
[wg-gcc-backend]: https://github.com/rust-lang/rustc_codegen_gcc
[wg-inline-asm]: https://github.com/rust-lang/project-inline-asm
[wg-leads]: https://www.rust-lang.org/governance/teams
[wg-llvm]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-macros]: https://github.com/rust-lang/wg-macros
[wg-mir-opt]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-parallel-rustc]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-polonius]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-rustc-dev-guide]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-safe-transmute]: https://github.com/rust-lang/project-safe-transmute
[wg-secure-code]: https://github.com/rust-secure-code/wg
[wg-security-response]: https://github.com/rust-lang/wg-security-response
[wg-triage]: https://www.rust-lang.org/governance/teams
[windows]: https://www.rust-lang.org/governance/teams


[Bastian Kersting]: https://github.com/1c3t3a
[Amanieu d'Antras]: https://github.com/Amanieu
[Benno Lossin]: https://github.com/BennoLossin
[Boxy]: https://github.com/BoxyUwU
[Alice Ryhl]: https://github.com/Darksonn
[Guillaume Gomez]: https://github.com/GuillaumeGomez
[James]: https://github.com/Jamesbarford
[Pete LeVasseur]: https://github.com/PLeVasseur
[Ralf Jung]: https://github.com/RalfJung
[Sparrow Li]: https://github.com/SparrowLii
[Wesley Wiser]: https://github.com/WesleyWiser
[Manuel Drehwald]: https://github.com/ZuseZ4
[Aapo Alasuutari]: https://github.com/aapoalas
[Alona Enraght-Moony]: https://github.com/adotinthevoid
[b-naber]: https://github.com/b-naber
[Jon Bauman]: https://github.com/baumanj
[Boxy]: https://github.com/boxyuwu
[Carol Nichols]: https://github.com/carols10cents
[Taylor Cramer]: https://github.com/cramertj
[David Wood]: https://github.com/davidtwco
[Ding Xiang Fei]: https://github.com/dingxiangfei2009
[David Tolnay]: https://github.com/dtolnay
[Eric Huss]: https://github.com/ehuss
[Ed Page]: https://github.com/epage
[Folkert de Vries]: https://github.com/folkertdev
[Frank King]: https://github.com/frank-king
[Ian McCormack]: https://github.com/icmccorm
[Jack Huey]: https://github.com/jackh726
[Jakob Koschel]: https://github.com/jakos-sec
[Josh Triplett]: https://github.com/joshtriplett
[Jack Wrenn]: https://github.com/jswrenn
[Jakub Beránek]: https://github.com/kobzol
[lcnr]: https://github.com/lcnr
[Rémy Rakic]: https://github.com/lqd
[Marco Ieni]: https://github.com/marcoieni
[Niko Matsakis]: https://github.com/nikomatsakis
[Predrag Gruevski]: https://github.com/obi1kenobi
[Oliver Scherer]: https://github.com/oli-obk
[Vadim Petrochenkov]: https://github.com/petrochenkov
[Ross Sullivan]: https://github.com/ranger-ross
[Ben Kimock]: https://github.com/saethlin
[Scott McMurray]: https://github.com/scottmcm
[Santiago Pastorino]: https://github.com/spastorino
[Tyler Mandry]: https://github.com/tmandry
[Tomas Sedovic]: https://github.com/tomassedovic
[TC]: https://github.com/traviscross
[Weihang Lo]: https://github.com/weihanglo
[Jane Lusby]: https://github.com/yaahc


[Complete]: https://img.shields.io/badge/Complete-green
[Help wanted]: https://img.shields.io/badge/Help%20wanted-yellow
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

