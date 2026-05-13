+++
path = "9999/12/31/project-goals-2026-04"
title = "Project goals update — April 2026 (end of 2025H2)"
authors = ["Nurzhan Saken"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

The 2025H2 Project Goal period has now concluded. Over these months, the Rust Project pursued [41 Project Goals](https://rust-lang.github.io/rust-project-goals/2025h2/goals.html), 13 of which were designated as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h2/goals.html#flagship-goals). This post contains curated updates on our progress since the [last post](https://blog.rust-lang.org/2026/01/05/project-goals-2025-december-update/) and the final status for each of the goals (many of which continue as part of the 2026 period). Full details for any particular goal are available in its [tracking issue](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20label%3Aex-2025h2).

Thanks to everyone who contributed! <3

## Table of contents

- [Flagship: Beyond the `&`](#flagship-beyond-the)
  - [Continue Experimentation with Pin Ergonomics](#continue-experimentation-with-pin-ergonomics)
  - [Design a language feature to solve Field Projections](#design-a-language-feature-to-solve-field-projections)
  - [Reborrow traits](#reborrow-traits)
- [Flagship: Flexible, fast(er) compilation](#flagship-flexible-fast-er-compilation)
  - [build-std](#build-std)
  - [Production-ready cranelift backend](#production-ready-cranelift-backend)
  - [Promoting Parallel Front End](#promoting-parallel-front-end)
  - [Relink don't Rebuild](#relink-don-t-rebuild)
- [Flagship: Higher-level Rust](#flagship-higher-level-rust)
  - [Ergonomic ref-counting: RFC decision and preview](#ergonomic-ref-counting-rfc-decision-and-preview)
  - [Stabilize cargo-script](#stabilize-cargo-script)
- [Flagship: Unblocking dormant traits](#flagship-unblocking-dormant-traits)
  - [Evolving trait hierarchies](#evolving-trait-hierarchies)
  - [In-place initialization](#in-place-initialization)
  - [Next-generation trait solver](#next-generation-trait-solver)
  - [Stabilizable Polonius support on nightly](#stabilizable-polonius-support-on-nightly)
- [Other goal updates](#other-goal-updates)
  - [Add a team charter for rustdoc team](#add-a-team-charter-for-rustdoc-team)
  - [Borrow checking in a-mir-formality](#borrow-checking-in-a-mir-formality)
  - [C++/Rust Interop Problem Space Mapping](#c-rust-interop-problem-space-mapping)
  - [Comprehensive niche checks for Rust](#comprehensive-niche-checks-for-rust)
  - [Const Generics](#const-generics)
  - [Continue resolving `cargo-semver-checks` blockers for merging into cargo](#continue-resolving-cargo-semver-checks-blockers-for-merging-into-cargo)
  - [Develop the capabilities to keep the FLS up to date](#develop-the-capabilities-to-keep-the-fls-up-to-date)
  - [Emit Retags in Codegen](#emit-retags-in-codegen)
  - [Expand the Rust Reference to specify more aspects of the Rust language](#expand-the-rust-reference-to-specify-more-aspects-of-the-rust-language)
  - [Finish the libtest json output experiment](#finish-the-libtest-json-output-experiment)
  - [Finish the std::offload module](#finish-the-std-offload-module)
  - [Getting Rust for Linux into stable Rust: compiler features](#getting-rust-for-linux-into-stable-rust-compiler-features)
  - [Getting Rust for Linux into stable Rust: language features](#getting-rust-for-linux-into-stable-rust-language-features)
  - [Implement Open API Namespace Support](#implement-open-api-namespace-support)
  - [MIR move elimination](#mir-move-elimination)
  - [Prototype a new set of Cargo "plumbing" commands](#prototype-a-new-set-of-cargo-plumbing-commands)
  - [Prototype Cargo build analysis](#prototype-cargo-build-analysis)
  - [reflection and comptime](#reflection-and-comptime)
  - [Rework Cargo Build Dir Layout](#rework-cargo-build-dir-layout)
  - [Run more tests for GCC backend in the Rust's CI](#run-more-tests-for-gcc-backend-in-the-rust-s-ci)
  - [Rust Stabilization of MemorySanitizer and ThreadSanitizer Support](#rust-stabilization-of-memorysanitizer-and-threadsanitizer-support)
  - [Rust Vision Document](#rust-vision-document)
  - [rustc-perf improvements](#rustc-perf-improvements)
  - [Stabilize public/private dependencies](#stabilize-public-private-dependencies)
  - [Stabilize rustdoc `doc_cfg` feature](#stabilize-rustdoc-doc-cfg-feature)
  - [SVE and SME on AArch64](#sve-and-sme-on-aarch64)
  - [Type System Documentation](#type-system-documentation)
  - [Unsafe Fields](#unsafe-fields)

---

## Flagship: Beyond the `&`

### [Continue Experimentation with Pin Ergonomics](https://github.com/rust-lang/rust-project-goals/issues/389)

- **People involved:** **[Frank King]**
- **Champions:** [compiler] ([Oliver Scherer]), [lang] ([TC])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/pin-ergonomics.html)

3 detailed updates available.

- **[Frank King]** — [comment from 2026-02-26](https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-3963321984)

  <details>

  > (Just come back from the Spring Festival)
  >
  > - (locally, no PR yet): design and implement the borrow checking algorithms of `&pin`
  > - Reviewed [Add `Drop::pin_drop` for pinned drops](https://github.com/rust-lang/rust/pull/144537), to update the submodule `book`
  > - Reviewed [Implement coercions between `&pin (mut|const) T` and `&(mut) T` when `T: Unpin`](https://github.com/rust-lang/rust/pull/149130), to do some refactors according to the reviewed messages.

  </details>

- **[Frank King]** — [comment from 2026-03-16](https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-4064499322)

  <details>

  > - Merged [Implement coercions between `&pin (mut|const) T` and `&(mut) T` when `T: Unpin`](https://github.com/rust-lang/rust/pull/149130).
  > - Opened draft PR [Implement borrowck for `&pin mut|const $place`](https://github.com/rust-lang/rust/pull/153693). The implementation needs to be refined and self-reviewed before the community reviews.

  </details>

- **[Frank King]** — [comment from 2026-04-16](https://github.com/rust-lang/rust-project-goals/issues/389#issuecomment-4258655696)

  <details>

  > Self-reviewed [Implement borrowck for `&pin mut|const $place`](https://github.com/rust-lang/rust/pull/153693). Found that the current approach of handling pinned borrows may be incorrect, as it failed to distinguish a pinned borrow from a coercion of a normal-to-pinned reference. The latter doesn't prevent a `T: Unpin` type from being moved, but the former does, which breaks the pin coercion test.

  </details>

### [Design a language feature to solve Field Projections](https://github.com/rust-lang/rust-project-goals/issues/390)

- **People involved:** **[Benno Lossin]**
- **Champions:** [lang] ([Tyler Mandry])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/field-projections.html)

5 detailed updates available.

- **[Benno Lossin]** — [comment from 2026-01-01](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3703190328)

  <details>

  > - At the beginning of December, we set out to [answer five important questions](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3621913656) regarding the virtual places approach. We discussed four questions and arrived at answers for three.
  >   - The first question we looked at was question 3 [Canonical Projections](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3644702112).
  >   - Next we looked at question 4 [Non-Indirected Containers](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3659055067).
  >   - As the final question we answered, we looked at question 1 [Field-by-Field Projections vs One-Shot Projections](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3677624396).
  >   - At the moment, we are investigating question 2 and I wrote a [blog post](https://bennolossin.github.io/blog/field-projections/virtual-places-and-borrowck.html) with a potential solution that still needs feedback.
  > - We started a [Wiki Project](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3690808729) to consolidate our knowledge in one place.
  >   - We [implemented an algorithm](https://github.com/rust-lang/beyond-refs/pull/9) to determine the type of a place expression.
  > - Our plan is to continue this project goal in the next goal period.

  </details>

- **[Benno Lossin]** — [comment from 2026-01-25](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3796971577)

  <details>

  > Earlier this month, [Nadrieril] [Ding Xiang Fei] and I held a meeting on autoref and method resolution in a world with field projections. This meeting resulted in a new page for the wiki on [autoref](https://rust-lang.github.io/beyond-refs/autoref.html).

  </details>

- **[Benno Lossin]** — [comment from 2026-02-28](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-3977436014)

  <details>

  > The first pull request of the lang experiment has just been merged: rust-lang/rust#152730
  >
  > This PR enables the use of the `field_of!` macro to obtain a unique type for each field of a struct, enum variant, tuple, or union. We call these types field representing types (FRTs). When the base type is a struct that is not `repr(packed)`, only contains `Sized` fields, this type automatically implements the `Field` trait that exposes some information about the field to the type system. The offset in bytes from the start of the struct, the type of the field and the type of the base type.
  >
  > The feature is still incomplete and highly experimental. We also want to tackle the limitations in future PRs. For the moment this is enough to give us the ability to experiment with library versions of field projections and write functions that are generic over the fields of structs. For example one can write code like this:
  >
  > ```rust
  > #![feature(field_projections)]
  >
  > use std::field::{Field, field_of};
  > use std::ptr;
  >
  > fn project_ref<'a, T, F: Field<Base = T>>(r: &'a T) -> &'a F::Type {
  >     // SAFETY: the `Field` trait guarantees that this is sound.
  >     unsafe { &*ptr::from_ref(r).byte_add(F::OFFSET).cast() }
  > }
  >
  > struct Struct {
  >     field: i32,
  >     other: u32,
  > }
  >
  > fn main() {
  >     let s = Struct { field: 42, other: 24 };
  >     let r = &s;
  >     let field = project_ref::<_, field_of!(Struct, field)>(r);
  >     let other = project_ref::<_, field_of!(Struct, other)>(r);
  >     println!("field: {field}"); // prints 42
  >     println!("other: {other}"); // prints 24
  > }
  > ```
  >
  > A very important feature of the types returned by `field_of!` is that you can implement traits for them if you own the base type. This allows anointing fields with information by extending the `Field` trait. For example, this allows encoding the property of being a structurally pinned field:
  >
  > ```rust
  > use std::pin::Pin;
  >
  > unsafe trait PinnableField: Field {
  >     type StructuralRefMut<'a>
  >     where
  >         Self::Type: 'a,
  >         Self::Base: 'a;
  >
  >     fn project_mut<'a>(base: Pin<&'a mut Self::Base>) -> Self::StructuralRefMut<'a>
  >     where
  >         Self::Type: 'a,
  >         Self::Base: 'a;
  > }
  >
  > fn project_pinned<'a, T, F>(r: Pin<&'a mut T>) -> <F as PinnableField>::StructuralRefMut<'a>
  > where
  >     F: PinnableField<Base = T>,
  > {
  >     F::project_mut(r)
  > }
  > ```
  >
  > We can then implement this extra trait for all of the fields of our struct (and automate that with a proc-macro):
  >
  > ```rust
  > unsafe impl PinnableField for field_of!(Struct, field) {
  >     type StructuralRefMut<'a> = &'a mut i32;
  >
  >     fn project_mut<'a>(base: Pin<&'a mut Self::Base>) -> Self::StructuralRefMut<'a>
  >     where
  >         Self::Type: 'a,
  >         Self::Base: 'a,
  >     {
  >         let base = unsafe { Pin::into_inner_unchecked(base) };
  >         &mut base.field
  >     }
  > }
  >
  > unsafe impl PinnableField for field_of!(Struct, other) {
  >     type StructuralRefMut<'a> = Pin<&'a mut u32>;
  >     // u32 is `Unpin`, so this isn't doing anything special, but it highlights the pattern.
  >
  >     fn project_mut<'a>(base: Pin<&'a mut Self::Base>) -> Self::StructuralRefMut<'a>
  >     where
  >         Self::Type: 'a,
  >         Self::Base: 'a,
  >     {
  >         let base = unsafe { Pin::into_inner_unchecked(base) };
  >         unsafe { Pin::new_unchecked(&mut base.other) }
  >     }
  > }
  > ```
  >
  > Now you can safely obtain a pinned mutable reference to `other` and a normal mutable reference to `field` by calling the `project_pinned` function and supplying the correct FRT.
  >
  > ([playground link](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024&gist=5b9494bd8f88aa4adf054f70abe16d9d))

  </details>

- **[Benno Lossin]** — [comment from 2026-03-20](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-4099385451)

  <details>

  > ### Plan for 2026
  >
  > We have an updated plan for this goal in 2026 consisting of three major steps:
  >
  > - `a-mir-formality`,
  > - Implementation,
  > - Experimentation.
  >
  > Some of their subtasks depend on other subtasks for other steps. You can find the details in the [updated tracking issue](https://github.com/rust-lang/rust/issues/145383). Here is a short rundown of each:
  >
  > **`a-mir-formality`:** we want to create a formal model of the borrow checker changes we're proposing to ensure correctness. We also want to create a document explaining our model in a more human-friendly language. To really get started with this, we're blocked on the new expression based syntax in development by Niko.
  >
  > **Implementation:** at the same time, we can start implementing more parts in the compiler. We will continue to improve FRTs, while keeping in mind that we might remove them if they end up being unnecessary. They still pose for a useful feature, but they might be orthogonal to field projections. We plan to make small and incremental changes, starting with library additions. We also want to begin exploring potential desugarings, for which we will add some manual and low level macros. When we have that figured out, we can fast-track syntax changes. When we have a sufficiently mature formal model of the borrow checker integration, we will port it to the compiler. After further evaluation, we can think about removing the `incomplete_feature` flag.
  >
  > **Experimentation:** after each compiler or standard library change, we look to several projects to stress-test our ideas in real code. I will take care of experimentation in the Linux kernel, while [Tyler Mandry] will be taking a look at testing field projections with `crubit`. [Josh Triplett] also has expressed eagerness of introducing them in the standard library; I will coordinate with him and the rest of t-libs-api to experiment there.

  </details>

- **[Benno Lossin]** — [comment from 2026-04-02](https://github.com/rust-lang/rust-project-goals/issues/390#issuecomment-4178384770)

  <details>

  > Yesterday, we held a t-lang design meeting on our current approach. [Nadrieril] and I authored a [design document](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg) with the feedback of [Tyler Mandry], [Ding Xiang Fei], [Alice Ryhl], and [Gary Guo]. In this document, we provided the motivation for this feature, what the look and feel of a solution fitting into the existing features of Rust is, and a comprehensive + compact introduction to our current approach based on virtual places.
  >
  > The general reception was extremely positive. To give some concrete quotes from the meeting:
  >
  > - Josh:
  >   > I adore this! I love how orthogonal it is, and how impactful and universal it is. I anticipate this becoming a beloved, _pervasive_ feature of Rust.
  >   >
  >   > Places and projection seem important enough to me that they're worth giving one of our precious remaining ASCII sigils to, and `@` is nicely evocative of a place (something is _at_ a place). So to the extent the final syntax benefits from a sigil, :+1: for giving this `@`. (See some feedback below on the details, though.)
  > - TC:
  >   > Love it. High concept. As I said in the last meeting:
  >   >
  >   > > I particularly like language features that reduce the need for library surface area, and this is one of those.
  >   >
  >   > There are, of course, many details to resolve and understand further, e.g., with respect to migration issues, interaction with `const`, `async`, and other effect-like things, etc. I'm looking forward to seeing the formalization work.
  > - tmandry:
  >   > What I love about this direction is how effectively it builds on what Rust already has. I love to see designs that reinforce our existing concepts while pushing them in directions that make them more expressive.
  > - Jack:
  >   > Whoo boy. This is great. There's so much here that I'm not exactly sure where to begin and what to comment on. I think this is the type of thing that we will only _really_ be able to figure out the nitty gritty details and ergonomics only after some amount of experimentation.
  >
  > There are a few takeaways from this meeting:
  >
  > - Mark [raised the concern](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#From-author-Concerns-about-moving-forward) that t-libs should be more involved in reviewing the experimental traits that we intend to add. Ensuring that we don't accidentally stabilize or expose some behavior, have sufficient documentation on our experimental traits, and that t-libs is in the loop of this feature in general.
  >   - Mark offered to review PRs and I will be tagging him in those.
  > - Jack [raised the concern](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Jack) that increasing the cognitive load for the 95% use-case should be avoided. Making the right choice between `@` and `&` might be challenging for users.
  >   - We [discussed this point more in the meeting](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#From-author-Ergonomics-of-raw-pointers) and concluded with that we need to do some experimentation, possibly utilizing the user research team. We will of course keep this in mind and revisit it later when we have a partially working implementation.
  > - TC requested that we publish our fine-grained design axioms, essentially the list of things we go through when considering a modification of our proposal.
  >   - I will write an update on this issue explaining exactly those.
  >
  > Aside from the concerns and directly actionable items, the meeting also covered design questions/comments that we want to take a look at in the coming weeks/months:
  >
  > - [Can we support reads/writes of different types?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Support-ReadWrite-of-non-PlaceTarget)
  > - [Can we support re-assembly of wrapper types, so going from `Cell<[T]>` to `[Cell<T>]`?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Re-assembling)
  > - [The `PlaceDiscriminant` trait needs to be carefully designed](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Trait-for-reading-discriminant-for-pattern-matching)
  > - [How do we handle naming conflicts & ensure SemVer evolution of library types implementing our traits?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Name-conflicts)
  > - [Can we support projecting through `Option`, so e.g. `&Option<Struct>` to `Option<&Field>`?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Projecting-through-Option-and-Result)
  > - [Can we support a pointer that carries alignment information & which is updated on projections?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Projecting-through-Option-and-Result)
  > - [What compatibility with effects do we need or want to support?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#Projecting-through-Option-and-Result)
  > - [What doors on future ergonomic improvements of pointers are we closing by having field projections?](https://hackmd.io/H5d2-83ER2ymNPZVIWCYWg?view#What-doors-are-we-closing)
  >
  > Thanks to everyone who participated in the meeting!

  </details>

### [Reborrow traits](https://github.com/rust-lang/rust-project-goals/issues/399)

- **People involved:** **[Aapo Alasuutari]**
- **Champions:** [compiler] ([Oliver Scherer]), [lang] ([Tyler Mandry])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/reborrow-traits.html)

1 detailed update available.

- **[Aapo Alasuutari]** — [comment from 2026-02-28](https://github.com/rust-lang/rust-project-goals/issues/399#issuecomment-3977652772)

  <details>

  > [PR](https://github.com/rust-lang/rust/pull/151753) open to get the first working version of the `Reborrow` and `CoerceShared` traits merged.
  >
  > ### Blockers
  >
  > Currently "blocked" on PR review, and of course my (and Ding's) work to fix all review issues.
  >
  > The review has brought up an opportunity to replace `Rvalue::Ref` / `ExprKind::Ref` with a more generalised variant that could encompass both references and user-defined references. This would be powerful, but it would be a very big and scary change. If this turns out to be a blocking issue for reviewers, then this will block the goal for the foreseeable future as the PR then starts on a massive refactoring.
  >
  > ### Help wanted
  >
  > The PR currently does not include derive traits, but we'd really want them. Instead of these:
  >
  > ```rust
  > impl<'a> Reborrow for CustomMarker<'a> {}
  > impl<'a> CoerceShared<CustomMarkerRef<'a>> for CustomMarker<a'> {}
  >
  > impl<'a, T> Reborrow for CustomMut<'a, T> {}
  > impl<'a, T> CoerceShared<CustomRef<'a, T>> for CustomMut<'a, T> {}
  > ```
  >
  > we'd prefer to have something like this:
  >
  > ```rust
  > #[derive(Reborrow, CoerceShared(CustomMarkerRef))]
  > struct CustomMarker<'a> { ... }
  >
  > #[derive(Reborrow, CoerceShared(CustomRef))]
  > struct CustomMut<'a, T> { ... }
  > ```
  >
  > If anyone feels like picking up this thread, that'd be awesome: the derive macros do not need to really perform any validity checking, as the trait itself will do that.
  >
  > If the PR merges soon, then public testing and exploration of the traits will be the next big thing. Likely concurrently with that the massive refactoring to generalise `Rvalue::Ref` / `ExprKind::Ref`.

  </details>

## Flagship: Flexible, fast(er) compilation

### [build-std](https://github.com/rust-lang/rust-project-goals/issues/274)

- **People involved:** **[David Wood]**, [Adam Gemmell]
- **Champions:** [cargo] ([Eric Huss]), [compiler] ([David Wood]), [libs] ([Amanieu d'Antras])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/build-std.html)

4 detailed updates available.

- **[David Wood]** — [comment from 2026-01-15](https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3754440280)

  <details>

  > [rust-lang/rfcs#3873](https://github.com/rust-lang/rfcs/pull/3873) has been merged and an FCP has been started on [rust-lang/rfcs#3874](https://github.com/rust-lang/rfcs/pull/3874) and [rust-lang/rfcs#3875](https://github.com/rust-lang/rfcs/pull/3875) - those both have some feedback for me to respond to that I'll get to as soon as I can.

  </details>

- **[David Wood]** — [comment from 2026-02-17](https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3914113179)

  <details>

  > No major updates this cycle - we're still working through feedback on [rust-lang/rfcs#3874](https://github.com/rust-lang/rfcs/pull/3874) and [rust-lang/rfcs#3875](https://github.com/rust-lang/rfcs/pull/3875) and prototyping the implementation to be prepared.

  </details>

- **[David Wood]** — [comment from 2026-03-17](https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-4073778569)

  <details>

  > Update this cycle is the same as last time - [rust-lang/rfcs#3874](https://github.com/rust-lang/rfcs/pull/3874) and [rust-lang/rfcs#3875](https://github.com/rust-lang/rfcs/pull/3875) are progressing, with feedback being addressed and checkboxes checked, and we're still working out what the implementation would look like.

  </details>

- **[David Wood]** — [comment from 2026-04-14](https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-4245212089)

  <details>

  > [rust-lang/rfcs#3874](https://github.com/rust-lang/rfcs/pull/3874) has finished FCP and is due to be merged any day now. I'm working on resolving the remaining open comments on [rust-lang/rfcs#3875](https://github.com/rust-lang/rfcs/pull/3875) and then intend to nudge the reviewers to have a look and check their boxes or leave concerns.
  >
  > [Adam Gemmell] has opened [rust-lang/cargo#16675](https://github.com/rust-lang/cargo/pull/16675) with an early sketch of some of the core changes that build-std would require and is working with the Cargo team to address feedback and work out how to proceed with the implementation.

  </details>

### [Production-ready cranelift backend](https://github.com/rust-lang/rust-project-goals/issues/397)

- **People involved:** **[Folkert de Vries]**, [bjorn3], [Trifecta Tech Foundation]
- **Champions:** [compiler] ([bjorn3])
- **Status:** Not completed ([lack of funding](https://github.com/rust-lang/rust-project-goals/issues/397#issuecomment-3597627406))

### [Promoting Parallel Front End](https://github.com/rust-lang/rust-project-goals/issues/121)

- **People involved:** **[Sparrow Li]**
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/parallel-front-end.html)

### [Relink don't Rebuild](https://github.com/rust-lang/rust-project-goals/issues/400)

- **People involved:** **[Jane Lusby]**, [@dropbear32](https://github.com/dropbear32), [@osiewicz](https://github.com/osiewicz)
- **Champions:** [cargo] ([Weihang Lo]), [compiler] ([Oliver Scherer])
- **Status:** Not completed ([note](https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/2025H2.20Goal.20Review/near/536084528))

## Flagship: Higher-level Rust

### [Ergonomic ref-counting: RFC decision and preview](https://github.com/rust-lang/rust-project-goals/issues/107)

- **People involved:** **[Niko Matsakis]**, [Santiago Pastorino]
- **Champions:** [compiler] ([Santiago Pastorino]), [lang] ([Niko Matsakis])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/ergonomic-rc.html)

### [Stabilize cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119)

- **People involved:** **[Ed Page]**
- **Champions:** [cargo] ([Ed Page]), [lang] ([Josh Triplett]), [lang-docs] ([Josh Triplett])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/cargo-script.html)

3 detailed updates available.

- **[Ed Page]** — [comment from 2026-01-14](https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3750134602)

  <details>

  > [#146377](https://github.com/rust-lang/rust/pull/146377) has been decided and merged.
  >
  > ### Blockers
  >
  > - T-lang discussing CR / text direction feedback: [comment](https://github.com/rust-lang/rust/pull/148051#issuecomment-3638326490)
  > - T-rustdoc deciding on and implementing how they want frontmatter handled in doctests

  </details>

- **[Ed Page]** — [comment from 2026-02-13](https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3897507244)

  <details>

  > - FCP has ended on [frontmatter support](https://github.com/rust-lang/rust/pull/148051), just awaiting merge
  > - [Cargo script](https://github.com/rust-lang/cargo/pull/16569) has entered FCP
  >
  > ### Blockers
  >
  > - Potential issues around edition, see [Cargo script edition policy (lang/edition aspects)](https://github.com/rust-lang/rust/issues/152254).

  </details>

- **[Ed Page]** — [comment from 2026-03-16](https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-4067313342)

  <details>

  > Cargo's FCP has ended.
  >
  > ### Blockers
  >
  > - [Cargo script edition policy (lang/edition aspects)](https://github.com/rust-lang/rust/issues/152254)

  </details>

## Flagship: Unblocking dormant traits

### [Evolving trait hierarchies](https://github.com/rust-lang/rust-project-goals/issues/393)

- **People involved:** **[Taylor Cramer]** and others
- **Champions:** [lang] ([Taylor Cramer]), [types] ([Oliver Scherer])
- **Status:** Superseded by the [Implement Supertrait `auto impl`](https://rust-lang.github.io/rust-project-goals/2026/supertrait-auto-impl.html) and [Arbitrary Self Types](https://rust-lang.github.io/rust-project-goals/2026/arbitrary-self-types.html) 2026 goals

### [In-place initialization](https://github.com/rust-lang/rust-project-goals/issues/395)

- **People involved:** **[Alice Ryhl]**, [Benno Lossin], [Michael Goulet], [Taylor Cramer], [Josh Triplett], [Gary Guo], [Yoshua Wuyts]
- **Champions:** [lang] ([Taylor Cramer])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/in-place-init.html)

1 detailed update available.

- **[Alice Ryhl]** — [comment from 2026-01-31](https://github.com/rust-lang/rust-project-goals/issues/395#issuecomment-3828006712)

  <details>

  > A [proposal](https://github.com/rust-lang/rust-project-goals/pull/477) to continue this goal in the next goal period was merged.

  </details>

### [Next-generation trait solver](https://github.com/rust-lang/rust-project-goals/issues/113)

- **People involved:** **[lcnr]**, [Boxy], [Michael Goulet]
- **Champions:** [types] ([lcnr])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/next-solver.html)

1 detailed update available.

- **[lcnr]** — [comment from 2026-01-19](https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3767714713)

  <details>

  > There hasn't been too much progress over the last few weeks and I've been mostly taking a Christmas break. [Nicholas Nethercote] has been looking into the performance of the new trait solver, cleaning up canonicalization and slightly improving its performance: [PR 1](https://github.com/rust-lang/rust/pull/150748) and [PR 2](https://github.com/rust-lang/rust/pull/150859).
  >
  > [Shoyu Vanilla] looked into [ICE from mir validation on unsizing in opendal](https://github.com/rust-lang/trait-system-refactor-initiative/issues/251) and uncovered the underlying bug there. While this issue also affects the old solver and the proper fix for it requires where-bounds on binders, we can work around this bug in the trait solver for now and intend to do so.
  >
  > We've started another crater run with all our recent changes and [adwin] has started to triage it, uncovering one new issue up until now. Intend to continue going through that over the next few weeks.
  >
  > There's also a lot in-progress work going on. I am collaborating with [Niko Matsakis] to specify and later RFC the cycle semantics of Rust. [León Orell Valerian Liehr] is working on a [replacement for the rustdoc's auto trait impl synthesis](https://github.com/rust-lang/rust/pull/149019). [tiif] is working on a fix a [MIR borrowck unsoundness](https://github.com/rust-lang/trait-system-refactor-initiative/issues/159). [Shoyu Vanilla] and I are improving the way we propagate inference constraints from the expected return type to function arguments, fixing [this issue](https://github.com/rust-lang/trait-system-refactor-initiative/issues/259).

  </details>

### [Stabilizable Polonius support on nightly](https://github.com/rust-lang/rust-project-goals/issues/118)

- **People involved:** **[Rémy Rakic]**, [Amanda Stjerna], [Niko Matsakis]
- **Champions:** [types] ([Jack Huey])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/polonius.html)

2 detailed updates available.

- **[Rémy Rakic]** — [comment from 2026-01-30](https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3824711064)

  <details>

  > This month's update:
  >
  > - [tiif] is making progress on [normalizing opaques while computing implied bounds](https://github.com/rust-lang/trait-system-refactor-initiative/issues/159)
  > - we discussed how to investigate and fix the remaining correctness issues in Tage's work, to be able to evaluate it more accurately: in particular around variance and bidirectional edges, and without the reliance on NLL (having computed region values / errors)
  > - we've tried to see if it'd be possible to remove the cfg region elements
  > - Amanda is still working on her two papers, one about the current borrow checker and one about the work on Polonius. Her major PR for the restructuring of placeholder handling during region inference is stalled due to a conflict with further trait solver developments and may have to be abandoned. Work with the larger types team is ongoing and smaller patches/refactorings/improvements are being landed in the meantime.
  > - [#149639](https://github.com/rust-lang/rust/pull/149639) has now landed, and [#150551](https://github.com/rust-lang/rust/pull/150551) is still in review
  > - I've also fixed more small inefficiencies (computing boring/relevant locals on-demand in diagnostics, removed conversions between locations and points, etc) building on top of the previous PRs (so they need to be reviewed first)
  > - I've looked at crates.io again with the alpha, to find functions that are slower than with NLLs. AFAICT the worst case there is 60% for a 5KLOC function with 42K loans, 255K statements, and 125K outlives constraints. I'll see what we can do with this. Small composable functions is still good advice.
  > - there seem to be optimization opportunities to 1. limit propagation to the smaller number of blocks that could be affected by bidirectional edges, 2. for unifying invariant lifetimes of live locals that are assigned at most once (à la use-def chains), 3. for invalidations that are just the activation of a reservation
  > - we discussed possible plans to gather actual statistics, using the infrastructure that was created for the Metrics project
  > - we're also preparing the new project goal for this year, where we'll want to stabilize the alpha 🤞

  </details>

- **[Rémy Rakic]** — [comment from 2026-02-28](https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3977551969)

  <details>

  > We had a bit less time this month, the update will be shorter, but still meaningful I hope:
  >
  > - [#150551](https://github.com/rust-lang/rust/pull/150551) has landed, and it feels stabilizable. To me, this part of the goal is achieved.
  > - still, "stabilizable" is not _stable_, and there is more work to do. We plan to stabilize this year, and the [project goal proposal for 2026](https://rust-lang.github.io/rust-project-goals/2026/polonius.html) tracks how.
  > - [tiif] is still deep in [#152051](https://github.com/rust-lang/rust/pull/152051), and `a-mir-formality` work with Niko and I.
  > - Amanda has opened a few cleanup PRs ([#152438](https://github.com/rust-lang/rust/pull/152438), and [#152579](https://github.com/rust-lang/rust/pull/152579)), and [#151863](https://github.com/rust-lang/rust/pull/151863) has landed already. She also has started looking into Tage's old PR to see if we can fix it, benchmark it more accurately, and see the cool parts there that we could be using.
  > - Jack is possibly going to have some time to work with us this year! His help will be very welcome, especially as I will have less time available myself.
  > - we'll be tracking the opaque type region liveness soundness issue in [#153215](https://github.com/rust-lang/rust/issues/153215), and I've added [a couple tests](https://github.com/rust-lang/rust/pull/153216), in case [tiif]'s PR or anything that impacts them lands.
  > - some of the tiny cleanups I mentioned last time have also landed in [#152587](https://github.com/rust-lang/rust/pull/152587).

  </details>

## Other goal updates

### [Add a team charter for rustdoc team](https://github.com/rust-lang/rust-project-goals/issues/387)

- **People involved:** **[Guillaume Gomez]**
- **Champions:** [rustdoc] ([Guillaume Gomez])
- **Status:** Completed

### [Borrow checking in a-mir-formality](https://github.com/rust-lang/rust-project-goals/issues/122)

- **People involved:** **[Niko Matsakis]**, [tiif]
- **Champions:** [types] ([Niko Matsakis])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/a-mir-formality.html)

### [C++/Rust Interop Problem Space Mapping](https://github.com/rust-lang/rust-project-goals/issues/388)

- **People involved:** **[Joel Marcey]**
- **Champions:** [compiler] ([Oliver Scherer]), [lang] ([Tyler Mandry]), [libs] ([David Tolnay])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/interop-problem-map.html)

5 detailed updates available.

- **[Joel Marcey]** — [comment from 2026-01-20](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3774860319)

  <details>

  > The Rust Foundation is opening up a short-term, approximately 3-month, contracting role to assist in our [Rust/C++ Interop](https://rustfoundation.org/interop-initiative/) initiative. The primary work and deliverables for the role will be to make substantial progress on the [Problem Space Mapping Rust Project Goal](https://rust-lang.github.io/rust-project-goals/2025h2/interop-problem-map.html) by collecting discrete problem statements and offering up recommendations on the work that should follow based upon the problems that you found.
  >
  > If you are interested in how programming languages interoperate, are curious in understanding the problems therein, and are have a passion to think about how those problems may be resolved for the betterment of interop, then this work may be for you.
  >
  > An ideal candidate will have experience with Rust programming. Having experience in C++ is strongly preferred as well. If you have direct experience with actual engineering that required interoperating between Rust and C++ codebases, that's even better.
  >
  > If you are interested, please [email me](https://github.com/JoelMarcey) (email address found in my GitHub profile) or contact me directly on Zulip by Tuesday, January 27 and we can take it from there to see if there may be a potential fit for further discussion.
  >
  > Thank you.

  </details>

- **[Joel Marcey]** — [comment from 2026-01-31](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3827224576)

  <details>

  > The effort to fill the [contracting role ](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3774860319) to support this project goal is in the process winding down. The interview and discussion process is nearly complete. We expect to make a final decision for the role in early February.

  </details>

- **[teor]** — [comment from 2026-02-27](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-3970777014)

  <details>

  > Hi, I'm the new contractor on the interop problem space mapping project goal.
  >
  > In the last week and a half, I've:
  >
  > - added [some draft high-level problem statement summaries](https://github.com/rustfoundation/interop-initiative/tree/main/problem-space)
  > - started mapping out [interop use cases](https://github.com/rustfoundation/interop-initiative/issues)
  > - added relationships between problems/use cases and [existing project goals & unstable compiler features](https://github.com/rustfoundation/interop-initiative/pull/10)
  >
  > Next step is prioritising a few of the use cases, then working on related problem statements in more detail.
  >
  > ### Blockers
  >
  > Nothing at the moment, still working through the high level mapping of the problem space.
  >
  > ### Help wanted
  >
  > Suggestions for more interop use cases would be very welcome, just open a discussion in [t-lang/interop](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop) and I'll turn it into a ticket. Or go ahead and [open a use case ticket directly](https://github.com/rustfoundation/interop-initiative/issues).
  >
  > I'll post an update here every few weeks, you can follow [more detailed weekly updates on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/427678-t-lang.2Finterop/topic/Interop.20Problem.20Space.20Mapping.20-.20Weekly.20Updates/with/576165231).

  </details>

- **[teor]** — [comment from 2026-03-30](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-4151397648)

  <details>

  > In the last month, I've:
  >
  > - met with the lang team, Crubit team, and `cxx` author, and Joel and Mara have met with the C++ standards working group
  > - expanded [some draft high-level problem statement summaries](https://github.com/rustfoundation/interop-initiative/tree/main/problem-space), and added code examples
  > - added 6 new [interop use cases](https://github.com/rustfoundation/interop-initiative/issues)
  > - added more relationships between problems/use cases and existing project goals & unstable compiler features
  > - prepared for the Rust All Hands, and started mentoring for Outreachy
  >
  > Specifically, the last month we've identified and prioritised two high-priority use cases for more detailed work:
  >
  > - [calling an overloaded C++ function from Rust](https://github.com/rustfoundation/interop-initiative/issues/14), with a [Rust lang experiment](https://github.com/rust-lang/rust/pull/153697) - [implementation discussion](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/On.20overloading/near/579590336)
  > - [adding Rust to an existing C++ build system](https://github.com/rustfoundation/interop-initiative/issues/13), this currently works for basic cases, but the tooling could be improved on the Rust side
  >
  > And I [analysed the problems / use cases we've collected so far](https://github.com/rustfoundation/interop-initiative/issues/3#issuecomment-4151337653), with priorities, responsible language, and a split into semantics or tooling changes.
  >
  > Next step is continuing to work on overloading and build systems in more detail. If you have specific Rust/C/C++ build system blockers, please open a chat or ticket.
  >
  > ### Blockers
  >
  > Nothing at the moment, everyone has been extremely helpful, and I'm getting good feedback on use cases, problems, priorities, and Rust language experiments.

  </details>

- **[teor]** — [comment from 2026-05-01](https://github.com/rust-lang/rust-project-goals/issues/388#issuecomment-4357141346)

  <details>

  > In the last month, I've:
  >
  > - prepared for RustWeek and the All Hands, where I will be [giving a Rust Project track talk](https://2026.rustweek.org/schedule/wednesday/#teor) and running an All Hands interop session (schedule TBC)
  > - added new [interop use cases and problem statements](https://github.com/rustfoundation/interop-initiative/issues), and continued categorising them using GitHub tags
  > - continued to expand the [draft high-level problem statement summaries](https://github.com/rustfoundation/interop-initiative/tree/main/problem-space)
  > - added interop code examples, including many examples from Outreachy applicants
  > - continued mentoring Outreachy applicants
  > - continued working on the Overloading Rust language experiment
  >
  > Specifically, the last month we've made detailed progress on two high-priority use cases:
  >
  > - [calling an overloaded C++ function from Rust](https://github.com/rustfoundation/interop-initiative/issues/14), with a [Rust lang experiment](https://github.com/rust-lang/rust/issues/153629) - [implementation discussion](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/On.20overloading/near/579590336)
  >   - we've [merged a refactor](https://github.com/rust-lang/rust/pull/155223) to prepare for this experiment, which gave some nice perf wins
  >   - the initial [overloading experiment PR](https://github.com/rust-lang/rust/pull/153697) has been through two rounds of review, and is waiting for my revisions and rebasing
  > - [adding Rust to an existing C++ build system](https://github.com/rustfoundation/interop-initiative/issues/13)
  >   - Outreachy applicants wrote [interop example code PRs](https://github.com/rustfoundation/interop-initiative/pulls?q=is%3Apr)
  >   - these interop user experiences are waiting for analysis, so they can be summarised in the build system and overloading problem statements
  >   - this will likely happen after RustWeek and the All Hands
  >
  > Next step is continuing to work on the overloading experiment, along with RustWeek/All Hands preparation, and collecting feedback during the conference.
  >
  > ### Blockers
  >
  > Nothing at the moment. There is a steady stream of new use cases, problems, code examples and Rust language experiment feedback.

  </details>

### [Comprehensive niche checks for Rust](https://github.com/rust-lang/rust-project-goals/issues/262)

- **People involved:** **[Bastian Kersting]**, [Jakob Koschel]
- **Champions:** [compiler] ([Ben Kimock]), [opsem] ([Ben Kimock])
- **Status:** Not completed

### [Const Generics](https://github.com/rust-lang/rust-project-goals/issues/100)

- **People involved:** **[Boxy]**, [Noah Lev]
- **Champions:** [lang] ([Niko Matsakis])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/const-generics.html)

6 detailed updates available.

- **[Niko Matsakis]** — [comment from 2026-01-27](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3806305532)

  <details>

  > [Boxy] and I have established a regular time to check-in on formalizing this within a-mir-formality. Today we mostly worked on the "model" of const values, starting with this
  >
  > ```rust
  > #[term]
  > pub enum ConstData {
  >     // Sort of equivalent to `ValTreeKind::Branch`
  >     #[cast]
  >     RigidValue(RigidConstData),
  >
  >     // Sort of equivalent to `ValTreeKind::Leaf`
  >     #[cast]
  >     Scalar(ScalarValue),
  >
  >     #[variable(ParameterKind::Const)]
  >     Variable(Variable),
  > }
  >
  >
  >
  > #[term]
  > pub enum ScalarValue {
  >     #[grammar(u8($v0))]
  >     U8(u8),
  >     #[grammar(u16($v0))]
  >     U16(u16),
  >     #[grammar(u32($v0))]
  >     U32(u32),
  >     #[grammar(u64($v0))]
  >     U64(u64),
  >     #[grammar(i8($v0))]
  >     I8(i8),
  >     #[grammar(i16($v0))]
  >     I16(i16),
  >     #[grammar(i32($v0))]
  >     I32(i32),
  >     #[grammar(i64($v0))]
  >     I64(i64),
  >     #[grammar($v0)]
  >     Bool(bool),
  >     #[grammar(usize($v0))]
  >     Usize(usize),
  >     #[grammar(isize($v0))]
  >     Isize(isize),
  > }
  >
  >
  > #[term($name $<parameters> { $,values })]
  > pub struct RigidConstData {
  >     pub name: RigidName,
  >     pub parameters: Parameters,
  >     pub values: Vec<Const>,
  > }
  > ```
  >
  > i.e., a const value can be a scalar value (as today) or a struct literal like `Foo { ... }` (which would also cover tuples and things). We got the various tests passing. Huzzah!

  </details>

- **[Boxy]** — [comment from 2026-01-30](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3823289633)

  <details>

  > In addition to what niko posted previously there's been a lot of other stuff happening. A lot of people have opened PRs to improve mGCA this month: [León Orell Valerian Liehr] [Noah Lev] @enthropy7 [Kivooeo] [mu001999] @Human9000-bit [Redddy] @Keith-Cancel @AprilNEA
  >
  > A rough list of things that have been improved for mGCA:
  >
  > - Lots of new expressions now supported by mGCA: const constructors, tuple constructor calls, array expressions, tuple expression, literals
  > - `associated_const_equality` has been merged into `min_generic_const_args`. the former was effectively dependent on the latter already so this just makes it nicer to use the former :)
  > - traits can now be dyn compatible if all associated constants are type consts and are specified in the trait object (e.g. `dyn Trait<ASSOC = 10>`)
  > - type consts are enforced to be non-generic
  > - a bunch of ICEs have been fixed
  > - camelid has been working on "non-min" version of mGCA which will allow arbitrary expressions to be used in the type system (a blog post with more detail will be published once this actually lands)
  >
  > In non-mGCA updates, as niko says, we've been meeting regularly to make progress on modelling const generics in a-mir-formality. I've also been spending time thinking about the interactions between `adt_const_params` and ADTs with privacy/safety invariants and I think I know how to structure the RFC in this area so can make progress on that again
  >
  > There's some more detail about the various bits of work people have done and who did what here: [#project-const-generics > perfectly adequately sized wins @ 💬](https://rust-lang.zulipchat.com/#narrow/channel/260443-project-const-generics/topic/perfectly.20adequately.20sized.20wins/near/566850721)

  </details>

- **[Niko Matsakis]** — [comment from 2026-02-13](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3897552648)

  <details>

  > [Boxy] and I have met (and continue to meet) and work on modeling const generics in a-mir-formality. We're still working on laying the groundwork.
  >
  > There is a proposed [project goal](https://rust-lang.github.io/rust-project-goals/2026/const-generics.html) for next year.

  </details>

- **[Boxy]** — [comment from 2026-02-28](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3977295413)

  <details>

  > There's been a lot of miscellaneous fixes for mGCA this month. I've also started drafting some blog posts to explain what's going on with mGCA/oGCA as well as soliciting use cases/experience reports for them and `adt_const_params`. I also talked with some folks at Rust Nation this month about const generics and what features would be useful for them and why.

  </details>

- **[Boxy]** — [comment from 2026-04-02](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-4179111762)

  <details>

  > Late on the update :') niko and i continue to meet to discuss const generics. we've made some progress on figuring out problems around privacy/safety in const generics. we've also been discussing the big picture stuff for const generics and where we're "heading".

  </details>

- **[Boxy]** — [comment from 2026-05-01](https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-4359546137)

  <details>

  > started running weekly meetings about const generics to make it easier to keep up to date with all the people who are working on const generics stuff. i think `min_adt_const_params` is now at the point of what the RFC is going to specify.
  >
  > GCA is making good progress thanks to ashley's work. i also met with lcnr where we talked about whether there was some version of mGCA that is stabilizeable in the near future or not (maybe!)

  </details>

### [Continue resolving `cargo-semver-checks` blockers for merging into cargo](https://github.com/rust-lang/rust-project-goals/issues/104)

- **People involved:** **[Predrag Gruevski]**
- **Champions:** [cargo] ([Ed Page]), [rustdoc] ([Alona Enraght-Moony])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/cargo-semver-checks.html)

1 detailed update available.

- **[Predrag Gruevski]** — [comment from 2026-01-17](https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-3764211880)

  <details>

  > I posted a ["year in review" for cargo-semver-checks](https://predr.ag/blog/cargo-semver-checks-2025-year-in-review/).
  >
  > It has a section on [how I think we should move forward in 2026 and beyond](https://predr.ag/blog/cargo-semver-checks-2025-year-in-review/#the-path-forward-for-2026-and-beyond).

  </details>

### [Develop the capabilities to keep the FLS up to date](https://github.com/rust-lang/rust-project-goals/issues/391)

- **People involved:** **[Pete LeVasseur]**, `t-spec`, and contributors from [Ferrous Systems](https://ferrous-systems.com/)
- **Champions:** [bootstrap] ([Jakub Beránek]), [lang] ([Niko Matsakis]), [spec] ([Pete LeVasseur])
- **Status:** Superseded by the [Stabilize FLS Release Cadence](https://rust-lang.github.io/rust-project-goals/2026/stabilize-fls-releases.html) 2026 goal

2 detailed updates available.

- **[Pete LeVasseur]** — [comment from 2026-03-04](https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-3999761277)

  <details>

  > We have a Project Goal in 2026 that we'll take on: [Stabilize FLS Release Cadence](https://rust-lang.github.io/rust-project-goals/2026/stabilize-fls-releases.html). Progress towards 1.93.1 looks good, most issues are [closed](https://github.com/rust-lang/fls/issues?q=is%3Aissue%20state%3Aopen%20%5BChange%5D%3A%20%5B1.93%5D).
  >
  > ### Help wanted
  >
  > We'd love more folks from the safety-critical community to contribute to picking up [issues](https://github.com/rust-lang/fls/issues) or opening an issue if you notice something is missing.

  </details>

- **[Pete LeVasseur]** — [comment from 2026-04-02](https://github.com/rust-lang/rust-project-goals/issues/391#issuecomment-4178437764)

  <details>

  > Trying to prepare FLS releases earlier:
  >
  > - since we completed the 1.94.0 release of the FLS a bit early this time, we checked into the stretch part of our goal this year to look at 1.95.0 early
  > - we learned a bit more of the release notes process thanks to tips from [Eric Huss] and [TC]
  > - [Tshepang Mbambo] and I attended the t-release meeting last week where we chatted about working a little "upstream" with them on generating the release notes a bit earlier
  > - tomorrow in our t-fls meeting we'll discuss our interest with engaging over there; at a minimum I'll get engaged with t-release
  >
  > Glossary and main-body text harmonization:
  >
  > - the first PR landed from [Tshepang Mbambo] removing IDs from the glossary
  > - further steps planned, we have a tracking issue for it
  >
  > Developer guide:
  >
  > - akin to how the Reference now has a developer's guide now for contributing we'll do the same in the FLS
  > - [Hristian Kirtchev] has been working on this

  </details>

### [Emit Retags in Codegen](https://github.com/rust-lang/rust-project-goals/issues/392)

- **People involved:** **[Ian McCormack]**
- **Champions:** [compiler] ([Ralf Jung]), [opsem] ([Ralf Jung])
- **Status:** Superseded by the [BorrowSanitizer](https://rust-lang.github.io/rust-project-goals/2026/borrowsanitizer.html) 2026 goal

4 detailed updates available.

- **[Ian McCormack]** — [comment from 2026-01-09](https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-3730419629)

  <details>

  > Here's our January status update!
  >
  > - Yesterday, we posted [an MCP](https://github.com/rust-lang/compiler-team/issues/958) for our retag intrinsics. While that's in progress, we'll start adapting our current prototype to remove our dependence on MIR-level retags. Once that's finished, we'll be ready to submit a PR.
  > - We published our first monthly [blog post](https://borrowsanitizer.com/status/january_2026.html) about BorrowSanitizer.
  > - Our overall goal for 2026 is to transition from a research prototype to a functional tool. Three key features have yet to be implemented: garbage collection, error reporting, and support for atomic memory accesses. Once these are complete, we'll be able to start testing real-world libraries and auditing our results against Miri.

  </details>

- **[Ian McCormack]** — [comment from 2026-02-24](https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-3955255197)

  <details>

  > We just posted our [February status update](https://borrowsanitizer.com/status/february_2026.html) for BorrowSanitizer. TL;DR:
  >
  > - We provide detailed error messages for aliasing violations, which look _almost_ like Miri's do!
  > - We have two forms of retag intrinsic: `__rust_retag_mem` and `__rust_retag_reg`. We no longer require a compiler plugin to determine the permission associated with a retag, which will make it possible to use BorrowSanitizer by providing a single `-Zsanitizer=borrow` flag to rustc. You can check out our [MCP](https://github.com/rust-lang/compiler-team/issues/958) for more detailed design updates.
  > - We are starting to have a better understanding of how BorrowSanitizer performs in practice, but we do not have enough data yet to be certain. From one test case, it seems like we are somewhat faster but still in the same category of performance as Miri when we compare against other sanitizers. Expect more detailed results to come as we scale up our benchmarking pipeline.
  > - We have a tentative plan for upstreaming BorrowSanitizer in 2026, starting with its LLVM components. We intend to start the RFC process on the LLVM side this spring, once our API is stable.

  </details>

- **[Ian McCormack]** — [comment from 2026-03-30](https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-4158528895)

  <details>

  > We just posted our [March status update](https://borrowsanitizer.com/status/march_2026.html) for BorrowSanitizer. TL;DR:
  >
  > - We added hundreds more relevant tests from Miri's test suite. At the moment, 80% pass.
  > - We improved our cargo plugin (`cargo-bsan`) to better support multilanguage libraries. This will let us start to recreate the bugs from our [earlier evaluation](https://dl.acm.org/doi/10.1109/ICSE55347.2025.00167).
  >
  > Our goal for April is to continue expanding our test suite, finish an initial version of the LLVM components of BorrowSanitizer, and hopefully start the RFC process on the LLVM side.

  </details>

- **[Ian McCormack]** — [comment from 2026-04-29](https://github.com/rust-lang/rust-project-goals/issues/392#issuecomment-4346250277)

  <details>

  > We have some exciting news: our talk on BorrowSanitizer was accepted [at RustConf](https://rustconf2026.sched.com/event/2KHtb) this year! We’re grateful for the opportunity and looking forward to sharing our results with the broader community this September.
  >
  > We just posted [our April status update](https://borrowsanitizer.com/status/april_2026.html). It’s a bit of a technical one. Here’s the TL;DR:
  >
  > - BorrowSanitizer now uses a shadow stack to track metadata at runtime - this is a significantly different strategy than other LLVM sanitizers, and it will help us support garbage collection.
  > - We are now ready to start sending in PRs for our retag intrinsics. It will take a little time to split our changes up into meaningful, reviewable chunks—you can expect to see these throughout the next week.
  >
  > The RFC for our LLVM components is taking a little longer than expected, but it was worth taking the extra time to test out compiler changes and make sure that we had the core parts of the instrumentation pass settled. We’ll be drafting the RFC throughout the next few weeks.

  </details>

### [Expand the Rust Reference to specify more aspects of the Rust language](https://github.com/rust-lang/rust-project-goals/issues/394)

- **People involved:** **[Josh Triplett]**, [Amanieu d'Antras], [Guillaume Gomez], [Jack Huey], [lcnr], [Mara Bos], [Vadim Petrochenkov], [Jane Lusby]
- **Champions:** [lang-docs] ([Josh Triplett]), [spec] ([Josh Triplett])
- **Status:** Superseded by the [Experimental language specification](https://rust-lang.github.io/rust-project-goals/2026/experimental-language-specification.html) 2026 goal

1 detailed update available.

- **[Josh Triplett]** — [comment from 2026-04-14](https://github.com/rust-lang/rust-project-goals/issues/394#issuecomment-4245135894)

  <details>

  > This work is now continuing into a [new goal](https://github.com/rust-lang/rust-project-goals/pull/490) by [Jack Huey].

  </details>

### [Finish the libtest json output experiment](https://github.com/rust-lang/rust-project-goals/issues/255)

- **People involved:** **[Ed Page]**
- **Champions:** [cargo] ([Ed Page])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/libtest-json.html)

### [Finish the std::offload module](https://github.com/rust-lang/rust-project-goals/issues/109)

- **People involved:** **[Manuel Drehwald]**, LLVM offload/GPU contributors
- **Champions:** [compiler] ([Manuel Drehwald]), [lang] ([TC])
- **Status:** Superseded by the [High-Level ML optimizations](https://rust-lang.github.io/rust-project-goals/2026/high-level-ml.html) 2026 goal

2 detailed updates available.

- **[Manuel Drehwald]** — [comment from 2026-01-16](https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3762217620)

  <details>

  > `std::autodiff` is moving closer to nightly, and `std::offload` is gaining various performance, feature, and hardware support improvements.
  >
  > #### autodiff
  >
  > [Jakub Beránek], [sgasho], and I continued working on enabling autodiff in nightly. We have a PR up that builds autodiff in CI, and verified that the artifacts can be installed and work on Linux. For apple however, we noticed that any autodiff usage hangs. After some investigation, it turns out that we ended up embedding two LLVM copies, one in rustc, and one in Enzyme. It should be comparably easy to get rid of the second one. Once we verified that this fixes the build, we'll merge the PR to enable autodiff on both targets in nightly.
  >
  > #### offload
  >
  > A lot of interesting updates on the performance, feature, and hardware support side.
  >
  > 1. [Marcelo Domínguez], @kevinsala, @jdoerfert, and I started implementing the first benchmarks, since that's generally the best way to find missing features or performance issues. We were positively surprised by how good the out-of-the-box performance was. We will implement a few more benchmarks and post the results once we have verified them. We also implemented multiple PRs which implement bugfixes, cleanups, and needed features like [support for scalars](https://github.com/rust-lang/rust/pull/150288). We also started working on LLVM optimizations which make sure that we can achieve even better performance.
  > 2. I noticed that our offload intrinsic allowed running Rust code on the GPU, but it wasn't of much help when calling gpu vendor libraries like cuBLAS. I [implemented](https://github.com/rust-lang/rust/pull/150683) a new helper intrinsic which allows calling those functions conveniently, without having to manually move data to or from the device. It will benefit from the same LLVM optimizations as our full offload intrinsic. It also a bit simpler to set up on the compiler and linker side, so it already works with `std` and mangled kernel names, something that we still have to improve for our main offload intrinsic.
  > 3. A lot of work happened on the LLVM offload side for SPIRV and Intel GPU support. At the moment, our Rust frontend is tested on NVIDIA and AMD server and consumer GPUs, as well as AMD HPC and Lapotop APUs. [Karol Zwolak] reached out since he wants to help with with also running Rust on Intel GPUs. Offload relies on LLVM which started gaining Intel support, so hopefully we won't need much work beyond a new intel-gpu target and a new stdarch module. There is also work on a new spirv target for rustc, which we could also support if it goes through LLVM. Due to some open questions around typed pointers it does not seem clear yet whether it will, so we will have to wait.
  > 4. Nikita started working on [updating](https://github.com/rust-lang/rust/pull/150722) our submodule to LLVM 22. This hopefully does not only brings some compile and runtime performance improvements, but also greatly simplifies how we can build and use offload. Once it landed I'll refactor our bootstrapping logic, and as part of that start building offload in CI.

  </details>

- **[Manuel Drehwald]** — [comment from 2026-04-01](https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-4166618042)

  <details>

  > `std::autodiff` is now partly in CI, and `std::offload` got tested on a lot more benchmarks.
  >
  > #### autodiff
  >
  > Work continued on enabling autodiff in nightly. Since the last update, we have enabled autodiff in some Mingw and Linux runners. Users can now download libEnzyme artifacts, place them locally in the right spot for their toolchain, and then use autodiff on their nightly compiler. Once macOS is added, we will enable a new rustup component that will handle the download for users. Before enabling autodiff on macOS, however, we want to change how we distribute LLVM on this target (from static to dynamic linking). There are a lot of workflows and users of this target, not all of which can be modelled in the Rust CI. Our last two attempts sadly broke such downstream users and local contributors, so both attempts had to be reverted. Since testing here is tricky, progress here might be on the slower side; we will see.
  >
  > #### offload
  >
  > Most of the work on the offload side lately has been invisible, since we were working on implementing more benchmarks and LLVM optimizations, as well as missing features, discovered by those benchmarks. We achieved excellent performance on those benchmarks; more details will soon be presented by [Marcelo Domínguez] at the EuroLLVM conference in two weeks!
  >
  > Beyond benchmarks, there was a lot of tinkering on smaller PRs, reviewing, and housekeeping. LLVM-22 landed, so we updated our bootrstrap code to make use of new APIs, and tried to move a few smaller PRs forward, mainly around a better user experience and for making more Rust features available. Since the focus is still on benchmarks, not many of those PRs landed. They are in a mostly ready state, so it's a good time to pick them up if you're considering contributing. Please ping me on Zulip or in any PR with the offload label if you are interested!

  </details>

### [Getting Rust for Linux into stable Rust: compiler features](https://github.com/rust-lang/rust-project-goals/issues/407)

- **People involved:** **[Tomas Sedovic]**, compiler contributors
- **Champions:** [compiler] ([Wesley Wiser])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/rust-for-linux-compiler-features.html)

4 detailed updates available.

- **[Tomas Sedovic]** — [comment from 2026-01-16](https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3760359980)

  <details>

  > Update from the 2026-01-14 meeting:
  >
  > #### `#![register_tool]` [rust#66079](https://github.com/rust-lang/rust/issues/66079)
  >
  > Tyler Mandry proposed FCP of the [RFC#3808](https://github.com/rust-lang/rfcs/pull/3808) and nominated it for a Lang discussion.
  >
  > #### `-Zdebuginfo-compression` [rust#120953](https://github.com/rust-lang/rust/issues/120953)
  >
  > Wesley Wiser proposed stabilization: [rust#150625](ttps://github.com/rust-lang/rust/pull/150625).
  >
  > Josh Triplett suggested trying to bring [zlib-rs](https://github.com/trifectatechfoundation/zlib-rs) in the kernel as a case study.
  >
  > #### `-Zdirect-access-external-data` [rust#127488](https://github.com/rust-lang/rust/issues/127488)
  >
  > [rust#150494](https://github.com/rust-lang/rust/pull/150494) was merged two days ago, what reminds is updating the documentation and stabilizing the feature.
  >
  > There's an [ongoing discussion about the feature on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/channel/425075-rust-for-linux/topic/New.20relocation.20model.20for.20relocatable.20code.20but.20static.20data/with/566044955) as well.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-02-17](https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-3914550342)

  <details>

  > Updates from the 2026-01-28 and 2026-02-11 meetings:
  >
  > #### -Zdirect-access-external-data [rust#127488](https://github.com/rust-lang/rust/issues/127488)
  >
  > [Gary Guo]'s [fix PR](https://github.com/rust-lang/rust/pull/150494) was merged.
  >
  > #### `--emit=noreturn`
  >
  > [Miguel Ojeda] reiterated that this is high on the list of compiler features the project needs. Right now, they're [doing these checks manually](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/tools/objtool/check.c#n186).
  >
  > Improving objtool's handling of noreturn is on [Gary Guo]'s radar but there wasn't time yet.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-03-16](https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-4068915161)

  <details>

  > Update from the 2026-03-11 meeting:
  >
  > #### `--emit=noreturn`
  >
  > It seems that figuring out which functions are `noreturn` is at a level too low for rustc. Function signatures are not sufficient and there are cases where [rustc doesn't know](https://godbolt.org/z/jexfjGanf) whether to emit `noreturn`. It is something we should ask the LLVM to give us that information.
  >
  > #### [`-Zsanitizer=kernel-hwaddress`](https://github.com/rust-lang/compiler-team/issues/975)
  >
  > [Alice Ryhl] opened a new issue to introduce the `-Zsanitizer=kernel-hwaddress` sanitizer for aarch64 targets: https://github.com/rust-lang/compiler-team/issues/975
  >
  > #### [`-Zharden-sls`](https://github.com/rust-lang/rust/issues/116851)
  >
  > [Wesley Wiser] is working on [allowing forbidden target features to be hard errors](https://github.com/rust-lang/rust/pull/152821), which the `-Zharden-sls` patch should be rebased on top of.
  >
  > #### [`#![register_tool]`](https://github.com/rust-lang/rust/issues/66079)
  >
  > The [corresponding RFC](https://github.com/rust-lang/rfcs/pull/3808) has been discussed by the Lang team on 2026-03-11. The overall vibe was positive and [TC] is going to read through it and hopefully check a box on the proposed FCP.
  >
  > #### [`-Zdebuginfo-compression`](https://github.com/rust-lang/rust/issues/120953)
  >
  > The [proposed stabilization](https://github.com/rust-lang/rust/pull/150625) received some feedback that needs to be addressed.
  >
  > #### [`-Zdirect-access-external-data`](https://github.com/rust-lang/rust/issues/127488)
  >
  > The discussion here has stalled.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-04-10](https://github.com/rust-lang/rust-project-goals/issues/407#issuecomment-4223621611)

  <details>

  > Update from the 2026-04-08 meeting:
  >
  > #### [`-Zsanitize=kernel-hwaddress`](https://github.com/rust-lang/rust/pull/153049)
  >
  > [rust#153049](https://github.com/rust-lang/rust/pull/153049) is merged. What remains of the [tracking issue rust#154171](https://github.com/rust-lang/rust/issues/154171) is a few docs checklists.
  >
  > [Alice Ryhl] added the unstable book doc changes in the PR itself and [Wesley Wiser] confirmed that's all the documentation needed for sanitizers.

  </details>

### [Getting Rust for Linux into stable Rust: language features](https://github.com/rust-lang/rust-project-goals/issues/116)

- **People involved:** **[Tomas Sedovic]**, [Ding Xiang Fei]
- **Champions:** [lang] ([Josh Triplett]), [lang-docs] ([TC])
- **Status:** Superseded by the [Rust for Linux](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html) 2026 roadmap

6 detailed updates available.

- **[Tomas Sedovic]** — [comment from 2026-01-19](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3768690708)

  <details>

  > Update from the 2026-01-14 meeting.
  >
  > #### `Deref` / `Receiver`
  >
  > Ding's arbitrary_self_types: Split the Autoderef chain [rust#146095](https://github.com/rust-lang/rust/pull/146095) is waiting on reviews. It updates the method resolution to essentially: `deref_chain(T).flat_map(|U| receiver_chain(U))`.
  >
  > The perf run was a wash and a carter has completed yesterday. Analysis pending.
  >
  > #### [RFC #3851: Supertrait Auto-impl](https://github.com/rust-lang/rfcs/pull/3851)
  >
  > Ding has submitted a [Rust Project goal for Supertrait Auto Impl](https://github.com/rust-lang/rust-project-goals/pull/436).
  >
  > #### Arbitrary Self Types [rust#44874](https://github.com/rust-lang/rust/issues/44874)
  >
  > We've discovered the `#[feature(arbitrary_self_types_pointer)]` feature gate. As the Lang consensus is to not support the `Receiver` trait on raw pointer types we're probably going to remove it (but this needs further discussion). This was a remnant from the original proposal, but the Lang has changed direction since.
  >
  > #### `derive(CoercePointee)` [rust#123430](https://github.com/rust-lang/rust/issues/123430)
  >
  > Ding is working on a fix to prevent accidental specialization of the trait implementation. [rust#149968](https://github.com/rust-lang/rust/pull/149968) is adding an interim fix.
  >
  > Alice opened a Reference PR for [rust#136776](https://github.com/rust-lang/rust/pull/136776). There are questions around the behaviour of the `as` cast vs. coercions.
  >
  > #### Pass pointers to const in assembly [rfc#3848](https://github.com/rust-lang/rfcs/pull/3848)
  >
  > Gary opened implementation for the RFC: [rust#138618](https://github.com/rust-lang/rust/pull/138618).
  >
  > #### Field Projections [goal#390](https://github.com/rust-lang/rust-project-goals/issues/390)
  >
  > Benno updated the Field Representing Types PR to the latest design. This makes the PR much simpler.
  >
  > Tyler opened a [Beyond References wiki](https://rust-lang.github.io/beyond-refs) to keep all the proposals, resources in one place.
  >
  > #### In-place initialization [goal#395](https://github.com/rust-lang/rust-project-goals/issues/395)
  >
  > Ding is writing a post to describe all the open proposals including Alice's new one that she brouhght up during the [LPC 2025](https://lpc.events/event/19/). He'll merge it in the [Beyond References wiki](https://rust-lang.github.io/beyond-refs).
  >
  > #### Macros, attributes, derives, etc.
  >
  > Josh brought up his work on adding more capable declarative macros for writing attributes and derives. He's asked the Rust for Linux team for what they need to stop using proc macros.
  >
  > Miguel noted they've just added dependency on [syn](https://crates.io/crates/syn), but they would like to remove it some day if their could.
  >
  > Benno provided a few cases of large macros that he thought were unlikely to be replaceable by declarative-style ones. Josh suggested there may be a way and suggested an asynchronous discussion.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-02-16](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3909582195)

  <details>

  > Updates from the 2026-01-28 and 2026-02-11 meetings:
  >
  > #### Removing the `likely`/`unlikely` hints in favour of `cold_path`
  >
  > The [stabilization of `core::hint::cold_path` lint](https://github.com/rust-lang/rust/pull/151576) is imminent and after it, the `likely` and `unlikely` hints are likely (pardon the pun) to be removed.
  >
  > The team discussed the impact of this. These hints are used in C but not yet in Rust. `cold_path` would be sufficient, but `likely`/`unlikely` would still be more convenient in cases where there isn't an `else` branch. [Tyler Mandry] mentioned that these can be implemented in terms of `cold_path`.
  >
  > #### Niche optimizations
  >
  > We discussed the feasibility of embedding data in lower bits of a pointer -- something the kernel is doing in C. This could also enable setting the top bit in the integers (which is otherwise never set) and make it represent an error in that case (and a regular pointer otherwise).
  >
  > Ideally, this would be done in safe Rust, as the idea is to improve the safety of the C code in question.
  >
  > Extending the niches is something Rust wants to see, but it's waiting on pattern types. There are short/medium-term options by using `unsafe` and wrapping it in a safe macro, but the long-term hope is to have this supported in the language.
  >
  > #### Vendoring zerocopy
  >
  > The project has interest in vendoring [zerocopy](https://github.com/google/zerocopy). We had its maintainers [Jack Wrenn] and [Joshua Liebow-Feeser] join us to discuss this and answer our questions. The main question was about whether to vendor at all, how often should we (or will have to) upgrade, and how much of it is expected to end up in the standard library.
  >
  > The project follows semver with the extended promise to not break minor versions even before 1.0.0. We could vendor the current 0.8 and we should be upgrade on our own terms (e.g. when we bring in new features) rather than being forced to.
  >
  > Right now, the project is able to experiment with various approaches and capabilities. Any stdlib integration a long way away, but there is interest in integrating these to the language and libraries where appropriate.
  >
  > #### New trait solver
  >
  > There's been a long-term effort to finish the new trait solver, which will unblock a lot of things. [Niko Matsakis] asked about things it's blocking for Rust for Linux.
  >
  > This is the list: unmovable types, guaranteed destructors, Type Alias Impl Trait (TAIT), Return Type Notation (RTN), const traits, const generics (over integer types), extern type.
  >
  > #### 2026 Project goals
  >
  > This year brings in the concept of roadmaps. We now have a [Rust for Linux](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html) and a few more granular Goals. We'll be adding more goals over time, but the one merged cover what we've been focusing on for now.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-03-11](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-4039494143)

  <details>

  > Update from the 2026-02-25 meeting:
  >
  > #### 2026 Project goals
  >
  > We spent most of the meeting going over the open Project goals, the [Rust for Linux roadmap](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html) and other things we'd like to see that aren't the right shape for a goal.
  >
  > [Miguel Ojeda] brought up the upcoming Debian 14 release (coming out probably somewhere around Q2 of 2027) and we went over each item and decided whether it's something we need to make sure is in that release or not.
  >
  > Debian stable is an important milestone and the Rust version in it serves as a baseline for Rust for Linux development.
  >
  > I'll add all this data into the [roadmap](https://rust-lang.github.io/rust-project-goals/2026/roadmap-rust-for-linux.html).

  </details>

- **[Tomas Sedovic]** — [comment from 2026-03-16](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-4068513211)

  <details>

  > Update from the 2026-03-11 meeting:
  >
  > #### Field projections
  >
  > We now have a [macro and machinery that uses the projection mechanism](https://lore.kernel.org/rust-for-linux/20260302164239.284084-1-gary@kernel.org/).
  >
  > The `dma_read!` / `dma_write!` macros switched over to it. This also fixes a soundness issue [1].
  >
  > Note: this is done entirely via macros and doesn't use any Field projections language features. The Field projection syntax and traits should make this more ergonomic and integrate the borrow checker so we can accept more code.
  >
  > [1]: https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/Soundness.20of.20.60dma_read!.28.29.60/with/573645610
  >
  > We're planning to have a design meeting with the Lang team in the last week of March.
  >
  > #### rustfmt imports formatting and trailing slashes
  >
  > We talked about the rustfmt formatting of the `use` statements again. While the trailing empty comment `//` workaround ([see this update](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3442107142)) is acceptable as a temporary measure, we need to find a long-term solution where you can configure rustfmt to accept this style.
  >
  > We don't have a issue for this specific formatting yet, though it was discussed in [#3361](https://github.com/rust-lang/rustfmt/issues/3361#issuecomment-3382614679).
  >
  > The next step are to create such an issue. We were hesitant to add burden to a team that's already at limit, but having the issue would let us track it from the Rust for Linux side.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-03-26](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-4136755115)

  <details>

  > Update from the 2026-03-26 meeting:
  >
  > #### Const generics
  >
  > [Boxy] asked the team for features that are most important under the const generics umbrella. This might help with prioritisation and just understanding of practical uses.
  >
  > 1. **Ability to do arithmetic on const generic types**: e.g. the kernel has a type [Bounded](https://rust.docs.kernel.org/next/kernel/num/bounded/struct.Bounded.html) which has a value and a maximum size (in bits). Both the bit width and value are const values. They want to be able to do arithmetics on these types (starting with bit shifts) that will guarantee the the result will fit within the specified size at compile time.
  > 2. **Argument-position const generics**: right now, the const generic types must be specified in the type bound section (within the angle brackets). So for example you have to write: `Bounded::<u8, 4>::new::<7>()` instead of the more natural `Bounded::<u8, 4>::new(7)`. This gets more complicated when there's const-time calculation happening rather than just a numerical constant -- in which case this also needs to be wrapped in curly brackets: `{ ... }`.
  > 3. Being generic over types other than numbers: pointers would be useful for [asm_const_ptr](https://rust-lang.github.io/rfcs/3848-asm-const-ptr.html). String literals too -- even if they're just passed through without being processed / operated on. And if going from a passthrough string makes it possible to pass through any type, that would help the team replace some typestate patterns they're using with an `enum`.
  >
  > #### `statx`
  >
  > [Alice Ryhl] [proposed](https://github.com/rust-lang/libs-team/issues/761) being able to create `std::fs::Metadata` from Linux `statx` syscall.
  >
  > This was discussed in the Libs-API meeting and they had questions about possible evolutions of the `statx` ABI -- if/how it can grow in the future and how they could handle that if they wanted some of the new data available. So we discussed it in the Rust for Linux meeting.
  >
  > In the end, it seems prudent to be reasonably defensive rather than relying on the syscall pre-filling default values.
  >
  > [Alice Ryhl] [proposed](https://github.com/rust-lang/libs-team/issues/761#issuecomment-4132354333) an opaque `statx` struct that would give the stdlib a way to decide on the struct's size, pre-filled contents and mask.
  >
  > [Miguel Ojeda] suggested contacting Christian Brauner and Alexander Viro (i.e. the VFS maintainers); [Josh Triplett] agreed that it would be good if we can get a thread with the right people in linux-fsdevel.

  </details>

- **[Tomas Sedovic]** — [comment from 2026-04-10](https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-4224585576)

  <details>

  > Update from the 2026-04-08 meeting:
  >
  > #### zerocopy features in Rust's std
  >
  > zerocopy uses two traits that are both polyfills for unstable traits : `KnownLayout` (for `ptr_metadata`) and `Immutable` (for `Freeze`). It would [help maintenance of zerocopy](https://github.com/rust-lang/rust/issues/81513#issuecomment-2414679019) (which Rust for Linux plans to start using) if these were stabilised.
  >
  > `ptr_metadata` is something the team wants in the kernel independently. It's possibly blocked on (or at least might have interactions with) the [Sized Hierarchy work](https://github.com/rust-lang/rust/issues/144404).
  >
  > `Freeze` (now `NoCell`) has an [RFC](https://github.com/rust-lang/rfcs/pull/3633).
  >
  > #### `Deref`/`Receiver`
  >
  > [Jack Huey] started reviewing [Ding Xiang Fei]'s [PR to split the autoderef chain](https://github.com/rust-lang/rust/pull/146095) and feels it's not ready to go in front of the full Lang team.
  >
  > We also discussed the dependence/independence of the `Deref` and `Receiver` implementations, in particular whether it ever makes sense to implement `Deref` but _not_ `Receiver`. [Josh Triplett] suggested gathering examples for cases like that (where you can't use the type as a `Self` type in the function declaration, but allow calling methods on it).
  >
  > The current plan for the experiment is to have these traits separate, but have the compiler enforce that if they implement the same type, their targets are identical. This will let us open the door for any future possibilities (a supertrait / subtrait relation, or having diverging targets in the future).
  >
  > We want to experiment to see where and how these traits and their possible evolution might be helpful.
  >
  > #### null-ptr-deref
  >
  > The team would like to have a (an optional) compiler guarantee, that the compiler never removes null checks on raw pointers. What can currently happen in C is that if you deref a null pointer, the compiler can do optimisations including removing any subsequent checks whether that pointer is null, because dereferencing a null pointer is undefined behaviour.
  >
  > But the null check can still help prevent further bugs and in C, the kernel now disables the optimisation that would remove it.
  >
  > [Miguel Ojeda] is going to open an MCP for this.
  >
  > #### In-Place Initialization
  >
  > [Benno Lossin] opened a [proposal for an in-person room](https://github.com/rust-lang/all-hands-2026/issues/17) at the 2026 All Hands for In-place initialization.
  >
  > Here's a [meta issue](https://github.com/rust-lang/rust/issues/153825) tracking all the proposals and discussions about the feature.
  >
  > The design space is complex and the team hopes that discussing it in person will help move it forward.

  </details>

### [Implement Open API Namespace Support](https://github.com/rust-lang/rust-project-goals/issues/256)

- **People involved:** **[Ed Page]**, [b-naber]
- **Champions:** [cargo] ([Ed Page]), [compiler] ([b-naber]), [crates-io] ([Carol Nichols])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/open-namespaces.html)

### [MIR move elimination](https://github.com/rust-lang/rust-project-goals/issues/396)

- **People involved:** **[Amanieu d'Antras]**
- **Champions:** [lang] ([Amanieu d'Antras])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/mir-move-elimination.html)

1 detailed update available.

- **[Amanieu d'Antras]** — [comment from 2026-04-03](https://github.com/rust-lang/rust-project-goals/issues/396#issuecomment-4183522763)

  <details>

  > The RFC has just been [published](https://github.com/rust-lang/rfcs/pull/3943). It has been significantly reworked since the last draft.
  >
  > Notable changes:
  >
  > - Removed the concept of activation/de-activation. Now the semantics don't need to deal with partially allocated locals. This is less powerful optimization-wise but should still cover most cases.
  > - Added byref/byval to call arguments to clarify how they are passed.
  > - Added a separate section for the surface language changes to separate it from the MIR changes.
  > - Added more details on the MIR optimization which eliminates moves.
  > - Changed the MIR operand evaluation order to be left-to-right, except for destination places which are always evaluated last.
  > - Added StorageLive back: we need it to mark the location where `llvm.lifetime.start` should be inserted, which is not the same as the location where a local is initialized. In the opsem, `StorageLive` doesn't actually allocate the local, that's still done when it is initialized by a write.

  </details>

### [Prototype a new set of Cargo "plumbing" commands](https://github.com/rust-lang/rust-project-goals/issues/264)

- **People involved:** **[Ed Page]**
- **Champions:** [cargo] ([Ed Page])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/cargo-plumbing.html)

### [Prototype Cargo build analysis](https://github.com/rust-lang/rust-project-goals/issues/398)

- **People involved:** **[Weihang Lo]**
- **Champions:** [cargo] ([Weihang Lo])
- **Status:** Completed

1 detailed update available.

- **[Weihang Lo]** — [comment from 2026-01-08](https://github.com/rust-lang/rust-project-goals/issues/398#issuecomment-3725163795)

  <details>

  > The prototype of this project goal is basically complete.
  >
  > ### Current state
  >
  > This project goal introduces **build analysis support in Cargo**, with the aim of making build behavior understandable across multiple invocations, not just a single run.
  >
  > At a high level, the prototype:
  >
  > - Records build metadata over time, including:
  >   - rebuild reasons
  >   - timing information
  >   - relevant invocation context
  > - Stores this data locally in a structured log format suitable for later analysis
  > - Exposes the data via unstable `cargo report` subcommands, such as:
  >   - `cargo report sessions` - list session IDs
  >   - `cargo report timings` - HTML timing report
  >   - `cargo report rebuilds` - Why things rebuilt
  >
  > See [the Reference](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-analysis) for a more thorough usage documentation
  >
  > ***
  >
  > ### Path towards stabilization
  >
  > Before this feature can be stabilized, the following unresolved questions must be answered. They might not block stabilization, but need to be evaluated if it is fine to leave for future.
  >
  > #### `cargo report` commands
  >
  > This is a stabilization blocker.
  >
  > - [ ] Currently all three report commands (`sessions`, `rebuilds`, `timings`)
  >       implicitly inspect global log files when if not in a workspace.
  >   - Should this be explicit with a flag?
  >   - Should this be an error if not in a workspace?
  > - [ ] Bikeshed on command names
  >   - Currently we have all nouns
  >     - For `sessions`
  >       - `runs` simple but ambiguous
  >       - Just `log` like `git log`
  >       - `history` user-friendly (`docker history`, shell `history`, though not alike)
  >     - For `timings`:
  >       - Not controversial, as we have `--timings` flag already
  >     - For `rebuilds`:
  >       - `rebuild-reasons` more explicit
  >   - Or move to action-oriented verbs:
  >   - `cargo report list-sessions`
  >   - `cargo report analyze-timings` ([`bazel analyze-profile`](https://bazel.build/docs/user-manual#analyze-profile))
  >   - `cargo report explain-rebuilds`
  >   - Or question-oriented verbs:
  >   - `cargo report what-ran` more general ([`buck2 log what-ran`](https://buck2.build/docs/developers/what-ran/))
  >   - `cargo report why-rebuilt/why-reran`
  >
  > ##### `cargo report sessions`
  >
  > - Currently it prints a human-readable output without a format for programmable use cases.
  >   - Should we provide a programmable output (for example behind `--message-format=json`)?
  >
  > ##### `cargo report rebuilds`
  >
  > - Extend the report from fingerprint to new hash (`-Cmetadata`/`-Cextra-filename`)
  >   - We currently can't distinguish whether a fresh build is a real new build or just rustflags changed
  >   - <https://github.com/rust-lang/cargo/pull/16456#discussion_r2662364819>
  > - Make each rebuilt reason more actionable and friendly for end-users.
  > - Should we log the fingerprint values being compared, or just the diff result?
  > - [#t-cargo > logging unit fingerprint @ 💬](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/logging.20unit.20fingerprint/near/565825913)
  >
  > #### Log message schema
  >
  > This is a stabilization blocker.
  >
  > - [ ] Providing types for reading log messages
  >   - We should export `LogMessage` enum and related types in `cargo-util-schemas`
  >   - Users may want to parse logs programmatically
  >   - <https://github.com/rust-lang/cargo/pull/16150#discussion_r2462065538>
  > - [ ] JSON schema evolution and versioning
  >   - Should we version the schema explicitly in each message?
  >   - Compatibility might be the same as <https://doc.rust-lang.org/nightly/cargo/commands/cargo-metadata.html?highlight=compa#compatibility>
  > - [ ] Message structure consistency
  >   - Current log messages deviate from cargo's normal JSON message structure
  >   - Should we align with existing cargo JSON output format, for example the `target` field?
  >   - <https://github.com/rust-lang/cargo/pull/16414#discussion_r2632724893>
  >   - <https://github.com/rust-lang/cargo/pull/16303#discussion_r2565526807>
  >   - <https://github.com/rust-lang/cargo/pull/16303#discussion_r2561862478>
  > - [ ] Should we expose the entire `DirtyReason` enum as-is?
  >   - Currently exposes internal implementation details
  >   - May want to create a separate public-facing enum
  >   - Need to decide which variants are user-facing vs internal
  > - [ ] Check usefulness of each variant
  >   - Some variants may be obsolete (e.g., `RustflagsChanged` may be rare after `-Cmetadata` changes)
  >   - Need audit of which variants actually occur in practice
  >   - Remove or consolidate rarely-used variants
  > - [ ] Make dirty reasons end-user friendly
  >   - Current reasons are technical (e.g., "local fingerprint type changed")
  >   - Users need actionable messages (e.g., "file modified: src/lib.rs")
  > - [ ] Expose `target` and `mode`
  >   - Are they universal for all kind of units?
  >     We might want to rename mode to action, as an action kind of a unit.
  >   - <https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build.20analysis.20log.20format/near/564781487>
  >
  > #### Log infrastructure
  >
  > These are mostly future possibilities, not a stabilization blocker,
  > as it is highly possible to do incremental improvements.
  >
  > - [ ] log compression <https://github.com/rust-lang/cargo/issues/16475>
  > - [ ] log rotation <https://github.com/rust-lang/cargo/issues/16471>
  > - [ ] Is losing data on crashes ok? https://github.com/rust-lang/cargo/pull//16150#discussion_r2462056940
  >
  > See also <https://github.com/rust-lang/cargo/issues/16471#issuecomment-3724915770>
  >
  > #### Nested Cargo calls
  >
  > See <https://github.com/rust-lang/cargo/issues/16477>.
  >
  > Basically, we need to have a way to associate log files of nested Cargo calls.
  > That helps other tools as well as `cargo fix` itself.
  >
  > This is a stabilization blocker.
  >
  > #### How contributors can help
  >
  > Future contributors can help by:
  >
  > - picking up any linked issues below or in <https://github.com/rust-lang/cargo/issues/15844>
  > - building external tools utilizing the log messages, and providing feedback
  > - providing real-world feedback from large or unusual builds
  >
  > A series of follow-up tasks has been cut to track remaining work:
  >
  > - https://github.com/rust-lang/cargo/issues/16470
  > - https://github.com/rust-lang/cargo/issues/16471
  > - https://github.com/rust-lang/cargo/issues/16472
  > - https://github.com/rust-lang/cargo/issues/16473
  > - https://github.com/rust-lang/cargo/issues/16474
  > - https://github.com/rust-lang/cargo/issues/16475
  > - https://github.com/rust-lang/cargo/issues/16477
  > - https://github.com/rust-lang/cargo/issues/16488

  </details>

### [reflection and comptime](https://github.com/rust-lang/rust-project-goals/issues/406)

- **People involved:** **[Oliver Scherer]**
- **Champions:** [compiler] ([Oliver Scherer]), [lang] ([Scott McMurray]), [libs] ([Josh Triplett])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/reflection-and-comptime.html)

5 detailed updates available.

- **[Oliver Scherer]** — [comment from 2026-01-14](https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-3749416638)

  <details>

  > - [The MVP](https://github.com/rust-lang/rust/pull/146923) has landed, and we even got the first contribs adding array support to reflection.
  >   - there are lots more types and type information that we could support, and it's rather easy to add more. Happy to review any work here.
  > - [`try_as_dyn` and `try_as_dyn_mut`](https://github.com/rust-lang/rust/pull/150033) have landed, and I'm working on [removing the `'static` requirement](https://github.com/rust-lang/rust/pull/150161).

  </details>

- **[Oliver Scherer]** — [comment from 2026-02-09](https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-3871224433)

  <details>

  > - @BD103 added [`Type::of` for unsized types](https://github.com/rust-lang/rust/pull/151019) and support for [slices](https://github.com/rust-lang/rust/pull/151118), [arrays](https://github.com/rust-lang/rust/pull/151031), and [raw pointer](https://github.com/rust-lang/rust/pull/151119)
  > - [Asuna] added [all of our primitives](https://github.com/rust-lang/rust/pull/151123)
  > - [Jamie Hill-Daniel] gave us [references](https://github.com/rust-lang/rust/pull/151222)
  > - @izagawd made it possible to [extract some info from `dyn Trait`](https://github.com/rust-lang/rust/pull/151239)
  >
  > There is ongoing work for Adts and function pointers, both of which will land as MVPs and will need some work to make them respect semver or generally become useful in practice
  >
  > [Removing the `'static` bound from `try_as_dyn`](https://github.com/rust-lang/rust/pull/150161) turned out to have many warts, so I'm limiting it to a much smaller subset and will have borrowck emit the `'static` requirement if the other rules do not apply (instead of having an unconditional `'static` requirement)

  </details>

- **[Oliver Scherer]** — [comment from 2026-03-19](https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-4093308372)

  <details>

  > - I added [support for getting reflection information of any type, not just `'static` ones](https://github.com/rust-lang/rust/pull/152381)
  > - [9SonSteroids] added a [function pointer MVP](https://github.com/rust-lang/rust/pull/152173) and [trait object support](https://github.com/rust-lang/rust/pull/152003)
  > - [Asuna] added [basic struct/enum/union support](https://github.com/rust-lang/rust/pull/151142)

  </details>

- **[Oliver Scherer]** — [comment from 2026-04-22](https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-4294713163)

  <details>

  > No changes since last time.
  >
  > I'm writing a document for the lang team meeting on reflection next week

  </details>

- **[Oliver Scherer]** — [comment from 2026-04-22](https://github.com/rust-lang/rust-project-goals/issues/406#issuecomment-4294725433)

  <details>

  > ### Help wanted
  >
  > - add more information to adts (e.g. doc comments, attributes, ...), whatever else is usually used by crates like bevy-reflect
  > - need to make struct field reflection respect privacy

  </details>

### [Rework Cargo Build Dir Layout](https://github.com/rust-lang/rust-project-goals/issues/401)

- **People involved:** **[Ross Sullivan]**
- **Champions:** [cargo] ([Weihang Lo])
- **Status:** Completed; the [Cargo cross workspace cache](https://rust-lang.github.io/rust-project-goals/2026/cargo-cross-workspace-cache.html) 2026 goal will build on this work

2 detailed updates available.

- **[Ross Sullivan]** — [comment from 2026-01-15](https://github.com/rust-lang/rust-project-goals/issues/401#issuecomment-3752502595)

  <details>

  > [Fine grain locking for `build-dir`](https://github.com/rust-lang/cargo/pull/16155) was merged and now available on nightly via `-Zfine-grain-locking` unstable flag. 🎉
  >
  > There are some known issues we'd like to address before doing a formal call for testing. Notably, improving blocking messages, fixing potential thread starvation in Cargo's job queue when locks block, and investigate increasing rlimits to reduce risk of hitting max file descriptors for large projects.
  >
  > I am hopeful that these issues will be resolved over the coming month and we can do a call for testing to start gathering feedback from the community on whether the new locking strategy improves workflows.

  </details>

- **[Ross Sullivan]** — [comment from 2026-03-09](https://github.com/rust-lang/rust-project-goals/issues/401#issuecomment-4024119674)

  <details>

  > After the initial PR from the last update was merged, we shifted our focus to resolving some of the known issues. Notably, locking blocks the Cargo job queue slowly causing thread starvation if many build units are held by another Cargo instance.
  >
  > We investigated adding the ability for Cargo to "suspend" a job internally while waiting for a lock, but we felt this change was a bit invasive and did not fit well with how the job queue was designed. Instead we plan to change our design to acquire all build unit locks prior to running the job queue (see [#16657](https://github.com/rust-lang/cargo/pull/16657)).
  >
  > At the same time, we have continued to refine the new `build-dir` to prepare it for a call for testing and eventual stabilization. ([#16542](https://github.com/rust-lang/cargo/pull/16542), [#16502](https://github.com/rust-lang/cargo/pull/16502), [#16515](https://github.com/rust-lang/cargo/pull/16515), [#16514](https://github.com/rust-lang/cargo/pull/16514))
  >
  > Finally we decided to split `.cargo-lock` into 2 locks to allow `cargo check` and `cargo build` to run in parallel when `artifact-dir == build-dir` (and `-Zfine-grain-locking` is enabled)
  >
  > I suspect this may be the last update on this goal, as the 2026 slate of goals is coming up. While I did not renew this goal for 2026, I do plan to continue work on this and eventually stabilize this within this year.

  </details>

### [Run more tests for GCC backend in the Rust's CI](https://github.com/rust-lang/rust-project-goals/issues/402)

- **People involved:** **[Guillaume Gomez]**
- **Champions:** [compiler] ([Wesley Wiser]), [infra] ([Marco Ieni])
- **Status:** Completed

### [Rust Stabilization of MemorySanitizer and ThreadSanitizer Support](https://github.com/rust-lang/rust-project-goals/issues/403)

- **People involved:** **[Jakob Koschel]**, [Bastian Kersting]
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/stabilization-of-sanitizer-support.html)

3 detailed updates available.

- **[Jakob Koschel]** — [comment from 2026-01-14](https://github.com/rust-lang/rust-project-goals/issues/403#issuecomment-3750257467)

  <details>

  > The [MCP](https://github.com/rust-lang/compiler-team/issues/951) has been seconded and is still waiting 3 days to be approved. Once that is done, we can proceed with merging the Tier 2 target.

  </details>

- **[Jakob Koschel]** — [comment from 2026-02-16](https://github.com/rust-lang/rust-project-goals/issues/403#issuecomment-3907370057)

  <details>

  > Both the [MCP](https://github.com/rust-lang/compiler-team/issues/951) and the [PR](https://github.com/rust-lang/rust/pull/149644) for the AddressSanitizer target have been merged. Next up I should prepare the MCP for the Memory- and ThreadSanitizer targets, hopefully sending out soon.

  </details>

- **[Jakob Koschel]** — [comment from 2026-03-31](https://github.com/rust-lang/rust-project-goals/issues/403#issuecomment-4161071545)

  <details>

  > The [targets for MSan and TSan](https://github.com/rust-lang/rust/pull/152757) are merged now.
  >
  > Next, I'll be working on stabilizing those two, now that we have a way to use it without other unstable features (`build-std`).

  </details>

### [Rust Vision Document](https://github.com/rust-lang/rust-project-goals/issues/269)

- **People involved:** **[Niko Matsakis]**, vision team
- **Status:** Partially completed; work continues outside of Project Goals

### [rustc-perf improvements](https://github.com/rust-lang/rust-project-goals/issues/275)

- **People involved:** **[James Barford]**, [Jakub Beránek], [David Wood]
- **Champions:** [compiler] ([David Wood]), [infra] ([Jakub Beránek])
- **Status:** Technical work completed; remaining policy and infrastructure work postponed

### [Stabilize public/private dependencies](https://github.com/rust-lang/rust-project-goals/issues/272)

- **People involved:** **[Ed Page]**
- **Champions:** [cargo] ([Ed Page])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/pub-priv.html)

### [Stabilize rustdoc `doc_cfg` feature](https://github.com/rust-lang/rust-project-goals/issues/404)

- **People involved:** **[Guillaume Gomez]**
- **Champions:** [rustdoc] ([Guillaume Gomez])
- **Status:** Not completed ([blocked](https://github.com/rust-lang/rust-project-goals/issues/404#issuecomment-3667410003))

### [SVE and SME on AArch64](https://github.com/rust-lang/rust-project-goals/issues/270)

- **People involved:** **[David Wood]**
- **Champions:** [compiler] ([David Wood]), [lang] ([Niko Matsakis]), [libs] ([Amanieu d'Antras])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/scalable-vectors.html)

5 detailed updates available.

- **[David Wood]** — [comment from 2026-01-15](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3754437263)

  <details>

  > [rust-lang/rust#143924](http://github.com/rust-lang/rust/pull/143924) has been merged, enabling scalable vector types to be defined on nightly, and I'm working on a patch to introduce unstable intrinsics/scalable vector types to `std::arch`

  </details>

- **[David Wood]** — [comment from 2026-02-17](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3914105881)

  <details>

  > Progress has been slow since the last update because I've been busy, but I've been working on a rebase of [rust-lang/stdarch#1509](https://github.com/rust-lang/stdarch/pull/1509), which has bitrot quite a bit. [Rémy Rakic] is joining me to work on the Sized Hierarchy parts of the goal.

  </details>

- **[David Wood]** — [comment from 2026-03-17](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-4073820988)

  <details>

  > On the scalable vector half of the goal, I've got a branch with [rust-lang/stdarch#1509](https://github.com/rust-lang/stdarch/pull/1509) rebased, though without the `intrinsic-test` tool having been updated - that ended up being tricky and we've agreed to do it as a follow-up. We've opened [rust-lang/rust#153286](http://github.com/rust-lang/rust/pull/153286) with the compiler fixes that the stdarch patch requires, which should land soon ([rust-lang/rust#153653](http://github.com/rust-lang/rust/pull/153653) was opened and landed in the interim).
  >
  > On the sized hierarchy half of the goal, [Rémy Rakic] has been updating our RFC such that we can discuss it in design meetings with the language team on the 18th and 25th - we'll update [rust-lang/rfcs#3729](https://github.com/rust-lang/rfcs/pull/3729) later today. We've split out the `const Sized` parts as a future possibility (though one we are committed to pursuing) as that has more open design questions, and we've discussed the proposed syntax and approach to migration - which are what we intend to focus on in the design meetings. He's also been working out how we can start implementing our migration strategy and help resolve blockers in other areas.

  </details>

- **[David Wood]** — [comment from 2026-03-17](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-4076740397)

  <details>

  > Per last comment, [rust-lang/rfcs#3729 has been updated](https://github.com/rust-lang/rfcs/pull/3729#issuecomment-4076702909)

  </details>

- **[David Wood]** — [comment from 2026-04-14](https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-4245300680)

  <details>

  > For the scalable vector half of the goal, we've landed a bunch of compiler fixes - [rust-lang/rust#153286](http://github.com/rust-lang/rust/pull/153286), [rust-lang/rust#153608](http://github.com/rust-lang/rust/pull/153608), [rust-lang/rust#154850](http://github.com/rust-lang/rust/pull/154850), [rust-lang/rust#154950](http://github.com/rust-lang/rust/pull/154950), [rust-lang/rust#155106](http://github.com/rust-lang/rust/pull/155106) and [rust-lang/rust#155243](http://github.com/rust-lang/rust/pull/155243) - and opened our stdarch patch with intrinsics - [rust-lang/stdarch#2071](https://github.com/rust-lang/stdarch/pull/2071). That patch should be passing CI tomorrow once nightly updates to fix an unrelated spurious CI failure. We've got a handful of follow-ups to do afterwards, listed on [rust-lang/rust#145052](http://github.com/rust-lang/rust/pull/145052).
  >
  > For the sized hierarchy half of the goal, [Rémy Rakic] and I had two design meetings with the language team ([2026/03/18](https://hackmd.io/@rust-lang-team/SJlubIO5bl) and [2026/03/25](https://hackmd.io/@rust-lang-team/r1TBpYZsWg)) discussing the syntax/naming and migration strategy respectively.
  >
  > On syntax, the language team preferred introducing an "only bounds" syntax to control opting-out of default bounds and opting-in to alternative bounds in a family of traits (described in [an alternative in the RFC](https://github.com/davidtwco/rfcs/blob/1bb22f00880cb6d24192af4041585683b56e9b3a/text/3729-sized-hierarchy.md#adding-only-bounds)), but there was an open question of whether that syntax should apply to an individual bound or all of the bounds - [Niko Matsakis] is investigating that.
  >
  > On naming, the language team also preferred the name `SizeOfVal` over `MetaSized`, and didn't like `Pointee` but had no better alternatives. [Rémy Rakic] prepared rust-lang/rust#154374 to do that renaming and started [a discussion with the library team](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/vibeck.20on.20Sized.20Hierarchy.20trait.20names/near/583074346) to confirm they were happy with the name, because changing it involves an amount of churn. The library team [wanted to know what other traits in the hierarchy might later be introduced](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/vibeck.20on.20Sized.20Hierarchy.20trait.20names/near/584067359), as that would help inform the naming of the currently proposed traits, so [Rémy Rakic] [wrote up a document](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/vibeck.20on.20Sized.20Hierarchy.20trait.20names/near/584637057) with that information. We're holding off on doing any name changes until we find some consensus between libs and lang - who is responsible for these traits' names is a bit unclear.
  >
  > On migration, the language team were largely happy with our proposed approach, and we realised that the approach proposed by lcnr for associated types might also work for our other migrations. [Rémy Rakic] has had meetings with lcnr to better understand that approach and to work out the next steps for implementing it.

  </details>

### [Type System Documentation](https://github.com/rust-lang/rust-project-goals/issues/405)

- **People involved:** **[Boxy]**, [lcnr]
- **Champions:** [types] ([Boxy])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/typesystem-docs.html)

### [Unsafe Fields](https://github.com/rust-lang/rust-project-goals/issues/273)

- **People involved:** **[Jack Wrenn]**, [Jacob Pratt], [Luca Versari]
- **Champions:** [compiler] ([Jack Wrenn]), [lang] ([Scott McMurray])
- **Status:** [Continued](https://rust-lang.github.io/rust-project-goals/2026/unsafe-fields.html)

2 detailed updates available.

- **[Jack Wrenn]** — [comment from 2026-02-18](https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-3922926899)

  <details>

  > RFC has been accepted. I'm preparing a 2026 continuing goal for stabilization.

  </details>

- **[Jack Wrenn]** — [comment from 2026-04-14](https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-4244724055)

  <details>

  > Opened PR ([#16767](https://github.com/rust-lang/rust/issues/16767)(https://github.com/rust-lang/rust-clippy/pull/16767)) extending Clippy support to unsafe fields.
  >
  > ### Blockers
  >
  > Waiting for t-clippy to review [#16767](https://github.com/rust-lang/rust/issues/16767)(https://github.com/rust-lang/rust-clippy/pull/16767).

  </details>

[James Barford]: https://github.com/Jamesbarford
[Carol Nichols]: https://github.com/carols10cents
[b-naber]: https://github.com/b-naber
[Ben Kimock]: https://github.com/Saethlin
[tiif]: https://github.com/tiif
[Santiago Pastorino]: https://github.com/spastorino
[Trifecta Tech Foundation]: https://trifectatech.org/
[Folkert de Vries]: https://github.com/folkertdev
[Sparrow Li]: https://github.com/SparrowLii
[bjorn3]: https://github.com/bjorn3
[9SonSteroids]: https://github.com/9SonSteroids
[Bastian Kersting]: https://github.com/1c3t3a
[Alex Eris Celeste née Gilding]: https://github.com/AlexCeleste
[Amanieu d'Antras]: https://github.com/Amanieu
[Benno Lossin]: https://github.com/BennoLossin
[Boxy]: https://github.com/BoxyUwU
[Alice Ryhl]: https://github.com/Darksonn
[Guillaume Gomez]: https://github.com/GuillaumeGomez
[Joel Marcey]: https://github.com/JoelMarcey
[Kivooeo]: https://github.com/Kivooeo
[Nadrieril]: https://github.com/Nadrieril
[Pete LeVasseur]: https://github.com/PLeVasseur
[Ralf Jung]: https://github.com/RalfJung
[Marcelo Domínguez]: https://github.com/Sa4dus
[Shoyu Vanilla]: https://github.com/ShoyuVanilla
[Asuna]: https://github.com/SpriteOvO
[Wesley Wiser]: https://github.com/WesleyWiser
[Manuel Drehwald]: https://github.com/ZuseZ4
[Aapo Alasuutari]: https://github.com/aapoalas
[Adam Gemmell]: https://github.com/adamgemmell
[Alona Enraght-Moony]: https://github.com/adotinthevoid
[adwin]: https://github.com/adwinwhite
[Amanda Stjerna]: https://github.com/amandasystems
[Boxy]: https://github.com/boxyuwu
[Noah Lev]: https://github.com/camelid
[Jamie Hill-Daniel]: https://github.com/clubby789
[Michael Goulet]: https://github.com/compiler-errors
[Taylor Cramer]: https://github.com/cramertj
[David Wood]: https://github.com/davidtwco
[Ding Xiang Fei]: https://github.com/dingxiangfei2009
[David Tolnay]: https://github.com/dtolnay
[Eric Huss]: https://github.com/ehuss
[Ed Page]: https://github.com/epage
[León Orell Valerian Liehr]: https://github.com/fmease
[Frank King]: https://github.com/frank-king
[Ian McCormack]: https://github.com/icmccorm
[Jack Huey]: https://github.com/jackh726
[Jakob Koschel]: https://github.com/jakos-sec
[Jacob Pratt]: https://github.com/jhpratt
[Joshua Liebow-Feeser]: https://github.com/joshlf
[Josh Triplett]: https://github.com/joshtriplett
[Jack Wrenn]: https://github.com/jswrenn
[Karol Zwolak]: https://github.com/karolzwolak
[Hristian Kirtchev]: https://github.com/kirtchev-adacore
[Jakub Beránek]: https://github.com/kobzol
[lcnr]: https://github.com/lcnr
[Rémy Rakic]: https://github.com/lqd
[Mara Bos]: https://github.com/m-ou-se
[mu001999]: https://github.com/mu001999
[Gary Guo]: https://github.com/nbdd0121
[Niko Matsakis]: https://github.com/nikomatsakis
[Nicholas Nethercote]: https://github.com/nnethercote
[Predrag Gruevski]: https://github.com/obi1kenobi
[Miguel Ojeda]: https://github.com/ojeda
[Oliver Scherer]: https://github.com/oli-obk
[Vadim Petrochenkov]: https://github.com/petrochenkov
[Ross Sullivan]: https://github.com/ranger-ross
[Redddy]: https://github.com/reddevilmidzy
[Marcelo Domínguez]: https://github.com/sa4dUs
[Scott McMurray]: https://github.com/scottmcm
[sgasho]: https://github.com/sgasho
[teor]: https://github.com/teor2345
[Tyler Mandry]: https://github.com/tmandry
[Tomas Sedovic]: https://github.com/tomassedovic
[TC]: https://github.com/traviscross
[Tshepang Mbambo]: https://github.com/tshepang
[Luca Versari]: https://github.com/veluca93
[Weihang Lo]: https://github.com/weihanglo
[Jane Lusby]: https://github.com/yaahc
[Yoshua Wuyts]: https://github.com/yoshuawuyts
[Marco Ieni]: https://github.com/marcoieni
[all]: https://www.rust-lang.org/governance/teams
[all-hands]: https://www.rust-lang.org/governance/teams
[alumni]: https://www.rust-lang.org/governance/teams
[android]: https://www.rust-lang.org/governance/teams
[apple]: https://www.rust-lang.org/governance/teams
[arewewebyet]: https://www.rust-lang.org/governance/teams
[arm]: https://www.rust-lang.org/governance/teams
[arm-maintainers]: https://www.rust-lang.org/governance/teams
[beyond-refs-editors]: https://www.rust-lang.org/governance/teams
[book]: https://github.com/rust-lang/book
[bootstrap]: https://github.com/rust-lang/rust
[cargo]: https://github.com/rust-lang/cargo
[clippy]: https://github.com/rust-lang/rust-clippy
[clippy-contributors]: https://github.com/rust-lang/rust-clippy
[cloud-compute]: https://www.rust-lang.org/governance/teams
[codegen-c-maintainers]: https://github.com/rust-lang/rustc_codegen_c
[community]: https://github.com/rust-community/team
[community-events]: https://github.com/rust-community/events-team
[community-localization]: https://github.com/rust-lang/community-localization
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
[fls]: http://github.com/rust-lang/fls-team
[fls-contributors]: https://www.rust-lang.org/governance/teams
[formality]: https://github.com/rust-lang/a-mir-formality
[foundation-board-project-directors]: https://www.rust-lang.org/governance/teams
[foundation-email-redirects]: https://www.rust-lang.org/governance/teams
[foundation-staff]: https://www.rust-lang.org/governance/teams
[fuchsia]: https://www.rust-lang.org/governance/teams
[goal-owners]: https://www.rust-lang.org/governance/teams
[goals]: https://github.com/rust-lang/rust-project-goals
[gpu-target]: https://www.rust-lang.org/governance/teams
[gsoc-contributors]: https://www.rust-lang.org/governance/teams
[hiring]: https://www.rust-lang.org/governance/teams
[infra]: https://github.com/rust-lang/infra-team
[infra-admins]: https://www.rust-lang.org/governance/teams
[infra-bors]: https://github.com/rust-lang/bors
[infra-bors-admins]: https://www.rust-lang.org/governance/teams
[inside-rust-reviewers]: https://www.rust-lang.org/governance/teams
[internal-sites]: https://www.rust-lang.org/governance/teams
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
[perspectives-on-llms-editors]: https://github.com/rust-lang/perspectives-on-llms/
[program]: https://www.rust-lang.org/governance/teams
[project-async-crashdump-debugging]: https://github.com/rust-lang/async-crashdump-debugging-initiative
[project-const-generics]: https://github.com/rust-lang/project-const-generics
[project-const-traits]: https://github.com/rust-lang/project-const-traits
[project-dyn-upcasting]: https://github.com/rust-lang/dyn-upcasting-coercion-initiative
[project-exploit-mitigations]: https://github.com/rust-lang/project-exploit-mitigations
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
[rfmf-design-committee]: https://www.rust-lang.org/governance/teams
[risc-v]: https://www.rust-lang.org/governance/teams
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[rust-analyzer-contributors]: https://github.com/rust-lang/rust-analyzer
[rust-by-example]: https://github.com/rust-lang/rust-by-example
[rust-for-linux]: https://www.rust-lang.org/governance/teams
[rust-timer]: https://www.rust-lang.org/governance/teams
[rustc-dev-guide]: https://forge.rust-lang.org/compiler/working-areas.html
[rustconf-emails]: https://www.rust-lang.org/governance/teams
[rustdoc]: https://github.com/rust-lang/rust
[rustdoc-frontend]: https://www.rust-lang.org/governance/teams
[rustdoc-internals]: https://www.rust-lang.org/governance/teams
[rustdoc-json-backend]: https://www.rust-lang.org/governance/teams
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustfmt-contributors]: https://github.com/rust-lang/rustfmt
[rustlings]: https://github.com/rust-lang/rustlings
[rustup]: https://github.com/rust-lang/rustup
[security-response]: https://github.com/rust-lang/wg-security-response
[social-media]: https://www.rust-lang.org/governance/teams
[spec]: https://github.com/rust-lang/spec
[spec-contributors]: https://github.com/rust-lang/spec
[style]: https://github.com/rust-lang/style-team
[survey]: https://github.com/rust-lang/surveys
[team-repo-admins]: https://www.rust-lang.org/governance/teams
[testing-devex]: https://www.rust-lang.org/governance/teams
[triage]: https://www.rust-lang.org/governance/teams
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
[wg-linker]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-llvm]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-macros]: https://github.com/rust-lang/wg-macros
[wg-mir-opt]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-parallel-rustc]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-polonius]: https://forge.rust-lang.org/compiler/working-areas.html
[wg-safe-transmute]: https://github.com/rust-lang/project-safe-transmute
[wg-secure-code]: https://github.com/rust-secure-code/wg
[windows]: https://www.rust-lang.org/governance/teams
[Complete]: https://img.shields.io/badge/Complete-green
[Help wanted]: https://img.shields.io/badge/Help%20wanted-yellow
[Team]: https://img.shields.io/badge/Team%20ask-red
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[TBD]: https://img.shields.io/badge/TBD-red
