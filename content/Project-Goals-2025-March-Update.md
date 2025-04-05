+++
layout = "post"
date = 2025-04-04
title = "March Goals Update"
author = "Rémy Rakic"
team = "Goals Team <https://www.rust-lang.org/governance/teams/goals>"
+++

The Rust project is currently working towards a [slate of 40 project goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html), with 3 of them designed as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="34"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** This work continues our drive to improve support for async programming in Rust. In 2024H2 we stabilized async closures; explored the generator design space; and began work on the `dynosaur` crate, an experimental proc-macro to provide dynamic dispatch for async functions in traits. In 2025H1 [our plan](https://rust-lang.github.io/rust-project-goals/2025h1/async.html) is to deliver (1) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (2) progress towards sync and async generators, simplifying the creation of iterators and async data streams; (3) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.

**What has happened?** **Generators.** Initial implementation work has started on an `iter!` macro experiment in <https://github.com/rust-lang/rust/pull/137725>. Discussions have centered around whether the macro should accept blocks in addition to closures, whether thunk closures with an empty arguments list should implement `IntoIterator`, and whether blocks should evaluate to a type that is `Iterator` as well as `IntoIterator`. See the [design meeting notes](https://hackmd.io/iQDQ_J3MTzaKBhq1FTbToQ?view) for more.

**dynosaur.** We released [dynosaur v0.2.0](https://github.com/spastorino/dynosaur/releases/tag/0.2.0) with some critical bug fixes and one breaking change. We have several more breaking changes queued up for an 0.3 release line that we also use plan to use as a 1.0 candidate.

**Pin ergonomics.** <https://github.com/rust-lang/rust/pull/135733> landed to implement `&pin const self` and `&pin mut self` sugars as part of the ongoing pin ergonomics experiment. Another PR is open with an early implementation of applying this syntax to borrowing expressions. There has been some discussion within parts of the lang team on whether to prefer this `&pin mut T` syntax or `&mut pin T`, the latter of which applies equally well to `Box<pin T>` but requires an edition.

<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/263'><strong>Organize Rust All-Hands 2025</strong></a></div>
    <div style="flex: initial;"><progress value="9" max="28"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** May 15, 2025 marks the 10-year anniversary of Rust's 1.0 release; it also marks 10 years since the [creation of the Rust subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). At the time [there were 6 Rust teams with 24 people in total](http://web.archive.org/web/20150517235608/http://www.rust-lang.org/team.html). There are now 57 teams with 166 people. In-person All Hands meetings are an effective way to help these maintainers get to know one another with high-bandwidth discussions. This year, the Rust project will be coming together for [RustWeek 2025](https://2025.rustweek.org), a joint event organized with [RustNL](https://2025.rustweek.org/about/). Participating project teams will use the time to share knowledge, make plans, or just get to know one another better. One particular goal for the All Hands is reviewing a draft of the [Rust Vision Doc](./rust-vision-doc.md), a document that aims to take stock of where Rust is and lay out high-level goals for the next few years.

**What has happened?**
> - Invite more guests, after deciding on who else to invite. (To be discussed today in the council meeting.)
> - Figure out if we can fund the travel+hotel costs for guests too. (To be discussed today in the council meeting.)

[Mara] has asked all attendees for suggestions for guests to invite. Based on that, [Mara] has invited roughly 20 guests so far. Only two of them needed funding for their travel, which we can cover from the same travel budget.

> - Open the call for proposals for talks for the Project Track (on wednesday) as part of the RustWeek conference.

The Rust Project Track at RustWeek has been published: <https://rustweek.org/schedule/wednesday/>

This track is filled with talks that are relevant to folks attending the all-hands afterwards.

[Mara]: https://github.com/m-ou-se

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2769198839">Comment by @m-ou-se posted on 2025-04-01:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

> - Invite more guests, after deciding on who else to invite. (To be discussed today in the council meeting.)
> - Figure out if we can fund the travel+hotel costs for guests too. (To be discussed today in the council meeting.)

I've asked all attendees for suggestions for guests to invite. Based on that, I've invited roughly 20 guests so far. Only two of them needed funding for their travel, which we can cover from the same travel budget.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Stabilize tooling needed by Rust for Linux</strong></a></div>
    <div style="flex: initial;"><progress value="11" max="26"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** This goal continues our work from 2024H2 in supporting the [experimental support for Rust development in the Linux kernel][RFL.com]. Whereas in 2024H2 we were focused on stabilizing required language features, our focus in 2025H1 is stabilizing compiler flags and tooling options. We will (1) implement [RFC #3716] which lays out a design for ABI-modifying flags; (2) take the first step towards stabilizing [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html); (3) extending rustdoc, clippy, and the compiler with features that extract metadata for integration into other build systems (in this case, the kernel's build system).

[RFC #3716]: https://github.com/rust-lang/rfcs/pull/3716
[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

**What has happened?** Most of the major items are in an iteration phase. The rustdoc changes for exporting doctests are the furthest along, with a working prototype; the RFL project has been integrating that prototype and providing feedback. Clippy stabilization now has a pre-RFC and there is active iteration towards support for build-std.

Other areas of progress:

* We have an [open PR](https://github.com/rust-lang/rust/pull/136926) to stabilize `-Zdwarf-version`.
* The lang and types team have been discussing the best path forward to resolve [#136702](https://github.com/rust-lang/rust/issues/136702). This is a soundness concern that was raised around certain casts, specifically, casts from a type like `*mut dyn Foo + '_` (with some lifetime) to `*mut dyn Foo + 'static` (with a static lifetime). Rust's defaulting rules mean that the latter is more commonly written with a defaulted lifetime, i.e., just `*mut dyn Foo`, which makes this an easy footgun. This kind of cast has always been dubious, as it disregards the lifetime in a rather subtle way, but when combined with arbitrary self types it permits users to disregard safety invariants making it hard to enforce soundness (see [#136702](https://github.com/rust-lang/rust/issues/136702) for details). The current proposal under discussion in [#136776](https://github.com/rust-lang/rust/issues/136776) is to make this sort of cast a hard error at least outside of an unsafe block; we evaluated the feasibility of doing a future-compatibility-warning and found it was infeasible. Crater runs suggest very limited fallout from this soundness fix but discussion continues about the best set of rules to adopt so as to balance minimizing fallout with overall language simplicity.

<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2721136744">Comment by @nikomatsakis posted on 2025-03-13:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update from our 2025-03-12 meeting ([full minutes](https://hackmd.io/@rust-lang-team/S181TSknyl)):

* RFL team requests someone to look at [#138368](https://github.com/rust-lang/rust/pull/138368) which is needed by kernel, [@davidtwco] to do so.
* `-Zbinary-dep-info` may not be needed; RFL may be able to emulate it.
* `rustdoc` changes for exporting doctests are being incorporated. [@GuillaumeGomez] is working on the kernel side of the feature too. [@ojeda] thinks it would be a good idea to do it in a way that does not tie both projects too much, so that `rustdoc` has more flexibility to change the output later on.
* [Pre-RFC](https://hackmd.io/@flip1995/By87NXIc1g) authored for clippy stabilization.
* Active iteration on the build-std design; feedback being provided by cargo team.
* [@wesleywiser] sent a [PR to stabilize `-Zdwarf-version`](https://github.com/rust-lang/rust/pull/136926).
* RfL doesn't use `cfg(no_global_oom_handling)` anymore. Soon, stable/LTS kernels that support several Rust versions will not use it either. Thus upstream Rust could potentially remove the `cfg` without breaking Linux, though other users like Windows may be still using it ([#**t-libs>no_global_oom_handling removal**](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/no_global_oom_handling.20removal/with/498600545)).
* Some discussion about best way forward for disabling orphan rule to allow experimentation with no firm conclusion.

[@davidtwco]: https://github.com/davidtwco
[@GuillaumeGomez]: https://github.com/GuillaumeGomez
[@ojeda]: https://github.com/ojeda
[@wesleywiser]: https://github.com/wesleywiser

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2755502098">Comment by @nikomatsakis posted on 2025-03-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Updates from [today's meeting](https://hackmd.io/@rust-lang-team/H1hZmpW6ke):

### Finalizing 2024h2 goals

* asm-goto is now stabilized! will be released in 1.87.
* asm-const has a [preliminary impl](https://github.com/rust-lang/rust/pull/138618), gcc support is needed.
* While not used in RFL, `naked_asm` is not on the list but it will be moving forward for stabilization. It suffers from the same LLVM bug as `global_asm` forgetting target feature flags.

### ABI-modifying compiler flags
* Andrew Zhogin has opened a draft PR (<https://github.com/rust-lang/rust/pull/138736>) following Alice's issue about which santisers should be modifiers (<https://github.com/rust-lang/rust/issues/138453>)

### Extract dependency information, configure no-std externally (-Zcrate-attr)

* We decided we don't need to be able to extract dependency information
* `-Zcrate-attr` has an RFC from jyn: <https://github.com/rust-lang/rfcs/pull/3791>

### Rustdoc features to extract doc tests

* No update.

### Clippy configuration

* [Pre-RFC](https://hackmd.io/@flip1995/By87NXIc1g) was published but hasn't (to our knowledge) made progress. Would be good to sync up on next steps with [@flip1995](https://github.com/flip1995).

### [Build-std](https://github.com/rust-lang/rust-project-goals/issues/274)

* No update. Progress will resume next week when the contributor working on this returns from holiday.

### `-Zsanitize-kcfi-arity`

* Added this as a new deliverable. These kind of "emerging codegen flag" requests can be expected from time to time. Notes available [here](https://clang.llvm.org/docs/ControlFlowIntegrity.html#fsanitize-kcfi-arity) and [here](https://lore.kernel.org/lkml/20250224123703.843199044@infradead.org/).
* The PR has been reviewed and is unblocked to land. 


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>

<br>

## Goals looking for help

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Promoting Parallel Front End</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div>
<!-- markdown separator --> 

<!-- markdown separator --> 


<!-- markdown separator --> 
*Help wanted:* Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/labels/WG-compiler-parallel) and try to reproduce the issues. If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Promoting.20Parallel.20Front.20End.20.28goals.23121.29/with/506292058).
<!-- markdown separator --> 


<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/121#issuecomment-2731267314">Comment by @SparrowLii posted on 2025-03-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

* **Key developments:** Several deadlock issue that remain for more than a year were resolved by #137731
The new test suit for parallel front end is being improved
* **Blockers:** null
* **Help wanted:** Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/labels/WG-compiler-parallel) and try to reproduce the issue

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/272'><strong>Stabilize public/private dependencies</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 

<!-- markdown separator --> 


<!-- markdown separator --> 
*Help wanted:* T-compiler people to work on the blocking issues [#119428](https://github.com/rust-lang/rust/issues/119428) and [#71043](https://github.com/rust-lang/rust/issues/71043). If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Stabilize.20public.2Fprivate.20dependencies.20.28goals.23272.29).
<!-- markdown separator --> 


<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/272#issuecomment-2730114361">Comment by @epage posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Key developments: @tgross35 got rust-lang/rust#135501 merged which improved which made progress on rust-lang/rust#119428, one of the two main blockers.  In rust-lang/rust#119428, we've further discussed further designs and trade offs.
- Blockers: Further work on rust-lang/rust#119428 and rust-lang/rust#71043
- Help wanted: T-compiler people to work on those above issues.



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<br>

## Other goal updates

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/100'><strong>&quot;Stabilizable&quot; prototype for expanded const generics</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-2729302537">Comment by @BoxyUwU posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

camelids PR has been merged, we now correctly (to the best of my knowledge) lower const paths under mgca. I have a PR open to ensure that we handle evaluation of paths to consts with generics or inference variables correctly, and that we do not attempt to evaluate constants before they have been checked to be well formed. I'm also currently mentoring someone to implement proper handling of normalization of inherent associated constants under mgca.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/274'><strong>build-std</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-2694682050">Comment by @davidtwco posted on 2025-03-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

A small update, @adamgemmell shared [revisions to the aforementioned document](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build-std.20goal/near/502644552), further feedback to which is being addressed.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 

Earlier this month, we completed one checkbox of the goal: `#[doc(hidden)]` in sealed trait analysis, live in `cargo-semver-checks` v0.40. We also made significant progress on type system modeling, which is part of two more checkboxes.
- We shipped method receiver types in our schema, enabling more than a dozen new lints.
- We have a draft schema for `?Sized` bounds, and are putting the finishing touches on `'static` and "outlives" bounds. More lints will follow here.
- We also have a draft schema for the new `use<>` precise capturing syntax.

Additionally, `cargo-semver-checks` is participating in Google Summer of Code, so this month we had the privilege of merging many contributions from new contributors who are considering applying for GSoC with us! We're looking forward to this summer, and would like to wish the candidates good luck in the application process!

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-2708411596">Comment by @obi1kenobi posted on 2025-03-08:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments:**
- Sealed trait analysis correctly handles `#[doc(hidden)]` items. This completes one checkbox of this goal!
- We shipped a series of lints detecting breakage in generic types, lifetimes, and const generics. One of them has already caught accidental breakage in the real world!

[`cargo-semver-checks` v0.40](https://github.com/obi1kenobi/cargo-semver-checks/releases/tag/v0.40.0), released today, includes a variety of improvements to sealed trait analysis. They can be summarized as "smarter, faster, more correct," and will have an immediate positive impact on popular crates such as `diesel` and `zerocopy`.

While we [already shipped a series of lints](https://github.com/obi1kenobi/cargo-semver-checks/releases/tag/v0.39.0) detecting generics-related breakage, more work is needed to complete that checkbox. This, and the "special cases like `'static` and `?Sized`", will be the focus of upcoming work.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/252'><strong>Declarative (&#x60;macro_rules!&#x60;) macro improvements</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="29"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/253'><strong>Evaluate approaches for seamless interop between C++ and Rust</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-2751789282">Comment by @tmandry posted on 2025-03-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Since our last update, there has been talk of dedicating some time at the Rust All Hands for interop discussion; @baumanj and @tmandry are going to work on fleshing out an agenda. @cramertj and @tmandry brainstormed with @oli-obk (who was very helpful) about ways of supporting a more ambitious "template instantiation from Rust" goal, and this may get turned into a prototype at some point.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Experiment with ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="8"></progress>
</div>
</div>
<!-- markdown separator --> 

There is now an early prototype available that allows you to write `x.use`; if the type of `X` implements `UseCloned`, then this is equivalent to `x.clone()`, else it is equivalent to a move. This is not the desired end semantics in a few ways, just a step along the road. Nothing to see here (yet).

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-2730880430">Comment by @nikomatsakis posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update: rust-lang/rust#134797 has landed.

Semantics as implemented in the PR:

* [x] Introduced a trait `UseCloned` implemented for `Rc` and `Arc` types.
* [x] `x.use` checks whether `x`'s type `X` implements the `UseCloned` trait; if so, then `x.use` is equivalent to `x.clone()`, otherwise it is a copy/move of `x`;
* [x] `use || ...x...` closures act like `move` closures but respect the `UseCloned` trait, so they will either `clone`, copy, or move `x` as appropriate.

Next steps:

* [ ] Modify codegen so that we guarantee that `x.use` will do a copy if `X: Copy` is true after monomorphization. Right now the desugaring to `clone` occurs before monomorphization and hence it will call the `clone` method even for those instances where `X` is a `Copy` type.
* [ ] Convert `x.use` to a move rather than a clone if this is a last-use.
* [ ] Make `x` equivalent to `x.use` but with an (allow-by-default) lint to signal that something special is happened.

Notable decisions made and discussions: 

* Opted to name the trait that controls whether `x.use` does a clone or a move `UseCloned` rather than `Use`. This is because the trait does not control whether or not you can use something but rather controls what happens when you do.
* [Question was raised on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/.60ergonomic_clones.60.20does.20not.20deref/near/505889669) as to whether `x.use` should auto-deref. After thinking it over, reached the conclusion that [it should not](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/.60ergonomic_clones.60.20does.20not.20deref/near/506157506), because `x` and `x.use` should eventually behave the same modulo lints, but that (as ever) a `&T -> T` coercion would be useful for ergonomic reasons.


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-2749926179">Comment by @ZuseZ4 posted on 2025-03-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I just noticed that I missed my February update, so I'll keep this update a bit more high-level, to not make it too long.

**Key developments:**
1) All key autodiff PRs got merged. So after building `rust-lang/rust` with the autodiff feature enabled, users can now use it, without the need for any custom fork.
2) std::autodiff received the first PRs from new contributors, which have not been previously involved in rustc development! My plan is to grow a team to maintain this feature, so that's a great start. The PRs are [here](https://github.com/rust-lang/rust/pull/137713), [here](https://github.com/rust-lang/rust/pull/138231) and [here](https://github.com/rust-lang/rust/pull/138314). Over time I hope to hand over increasingly larger issues.
3) I received an offer to join the Rust compiler team, so now I can also officially review and approve PRs! For now I'll focus on reviewing PRs in the fields I'm most comfortable with, so autodiff, batching, and soon GPU offload.
4) I implemented a standalone batching feature. It was a bit larger (~2k LoC) and needed some (back then unmerged) autodiff PRs, since they both use the same underlying Enzyme infrastructure. I therefore did not push for merging it.
5) I recently implemented batching as part of the autodiff macro, for people who want to use both together. I subsequently split out a first set of code improvements and refactorings, which already [got merged](https://github.com/rust-lang/rust/pull/138627). The remaining autodiff feature [PR](https://github.com/rust-lang/rust/pull/137880) is only 600 loc, so I'm currently cleaning it up for review.
6) I spend time preparing an MCP to enable autodiff in CI (and therefore nightly). I also spend a lot of time discussing a potential MLIR backend for rustc. Please reach out if you want to be involved!

**Help wanted: **
We want to support autodiff in lib builds, instead of only binaries. oli-obk and I recently figured out the underlying bug, and I started with a PR in https://github.com/rust-lang/rust/pull/137570. The problem is that autodiff assumes fat-lto builds, but lib builds compile some of the library code using thin-lto, even if users specify `lto=fat` in their Cargo.toml. We'd want to move every thing to fat-lto if we enable Autodiff as a temporary solution, and later move towards embed-bc as a longer-term solution. If you have some time to help please reach out! Some of us have already looked into it a little but got side-tracked, so it's better to talk first about which code to re-use, rather than starting from scratch.


I also booked my RustWeek ticket, so I'm happy to talk about all types of Scientific Computing, HPC, ML, or cursed Rust(c) and LLVM internals! Please feel free to dm me if you're also going and want to meet.




<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/110'><strong>Extend pubgrub to match cargo&#x27;s dependency resolution</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-2725379795">Comment by @Eh2406 posted on 2025-03-14:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Progress continues to be stalled by high priority tasks for $DAY_JOB. It continues to be unclear when the demands of work will allow me to return focus to this project.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/254'><strong>Externally Implementable Items</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="9"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/255'><strong>Finish the libtest json output experiment</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-2730118576">Comment by @epage posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Key developments: 
  - Between tasks on #92, I've started to refresh myself on the libtest-next code base
- Blockers: 
- Help wanted: 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/256'><strong>Implement Open API Namespace Support</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/257'><strong>Implement restrictions, prepare for stabilization</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="8"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/258'><strong>Improve state machine codegen</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 

We've started work on implementing `#[loop_match]` on [this branch](https://github.com/trifectatechfoundation/rust/tree/loop_match_attr). For the time being integer and enum patterns are supported. The [benchmarks](https://github.com/rust-lang/rust-project-goals/issues/258#issuecomment-2732965199), are extremely encouraging, showing large improvements over the status quo, and significant improvements versus `-Cllvm-args=-enable-dfa-jump-thread`. 

Our next steps can be found in the [todo file](https://github.com/trifectatechfoundation/rust/blob/loop_match_attr/loop_match_todo.md), and focus mostly on improving the code quality and robustness.

<!-- markdown separator --> 


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/258#issuecomment-2732962674">Comment by @folkertdev posted on 2025-03-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@traviscross how would we make progress on that? So far we've mostly been talking to @joshtriplett, under the assumption that a `#[loop_match]` attribute on loops combined with a `#[const_continue]` attribute on "jumps to the next iteration" will be acceptable as a language experiment.

Our current implementation handles the following

```rust
#![feature(loop_match)]

enum State {
    A,
    B,
}

fn main() {
    let mut state = State::A;
    #[loop_match]
    'outer: loop {
        state = 'blk: {
            match state {
                State::A =>
                {
                    #[const_continue]
                    break 'blk State::B
                }
                State::B => break 'outer,
            }
        }
    }
}
```

Crucially, this does not add syntax, only the attributes and internal logic in MIR lowering for statically performing the pattern match to pick the right branch to jump to.

The main challenge is then to implement this in the compiler itself, which we've been working on (I'll post our tl;dr update shortly)

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/258#issuecomment-2732965199">Comment by @folkertdev posted on 2025-03-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Some benchmarks (as of march 18th)

A benchmark of https://github.com/bjorn3/comrak/blob/loop_match_attr/autolink_email.rs, basically a big state machine that is a perfect fit for loop match

```
Benchmark 1: ./autolink_email
  Time (mean ± σ):      1.126 s ±  0.012 s    [User: 1.126 s, System: 0.000 s]
  Range (min … max):    1.105 s …  1.141 s    10 runs
 
Benchmark 2: ./autolink_email_llvm_dfa
  Time (mean ± σ):     583.9 ms ±   6.9 ms    [User: 581.8 ms, System: 2.0 ms]
  Range (min … max):   575.4 ms … 591.3 ms    10 runs
 
Benchmark 3: ./autolink_email_loop_match
  Time (mean ± σ):     411.4 ms ±   8.8 ms    [User: 410.1 ms, System: 1.3 ms]
  Range (min … max):   403.2 ms … 430.4 ms    10 runs
 
Summary
  ./autolink_email_loop_match ran
    1.42 ± 0.03 times faster than ./autolink_email_llvm_dfa
    2.74 ± 0.07 times faster than ./autolink_email
```

`#[loop_match]` beats the status quo, but also beats the llvm flag by a large margin.

---

A benchmark of zlib decompression with chunks of 16 bytes (this makes the impact of `loop_match` more visible)

```
Benchmark 1 (65 runs): target/release/examples/uncompress-baseline rs-chunked 4
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          77.7ms ± 3.04ms    74.6ms … 88.9ms          9 (14%)        0%
  peak_rss           24.1MB ± 64.6KB    24.0MB … 24.2MB          0 ( 0%)        0%
  cpu_cycles          303M  ± 11.8M      293M  …  348M           9 (14%)        0%
  instructions        833M  ±  266       833M  …  833M           0 ( 0%)        0%
  cache_references   3.62M  ±  310K     3.19M  … 4.93M           1 ( 2%)        0%
  cache_misses        209K  ± 34.2K      143K  …  325K           1 ( 2%)        0%
  branch_misses      4.09M  ± 10.0K     4.08M  … 4.13M           5 ( 8%)        0%
Benchmark 2 (68 runs): target/release/examples/uncompress-llvm-dfa rs-chunked 4
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          74.0ms ± 3.24ms    70.6ms … 85.0ms          4 ( 6%)        🚀-  4.8% ±  1.4%
  peak_rss           24.1MB ± 27.1KB    24.0MB … 24.1MB          3 ( 4%)          -  0.1% ±  0.1%
  cpu_cycles          287M  ± 12.7M      277M  …  330M           4 ( 6%)        🚀-  5.4% ±  1.4%
  instructions        797M  ±  235       797M  …  797M           0 ( 0%)        🚀-  4.3% ±  0.0%
  cache_references   3.56M  ±  439K     3.08M  … 5.93M           2 ( 3%)          -  1.8% ±  3.6%
  cache_misses        144K  ± 32.5K     83.7K  …  249K           2 ( 3%)        🚀- 31.2% ±  5.4%
  branch_misses      4.09M  ± 9.62K     4.07M  … 4.12M           1 ( 1%)          -  0.1% ±  0.1%
Benchmark 3 (70 runs): target/release/examples/uncompress-loop-match rs-chunked 4
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          71.6ms ± 2.43ms    69.3ms … 78.8ms          6 ( 9%)        🚀-  7.8% ±  1.2%
  peak_rss           24.1MB ± 72.8KB    23.9MB … 24.2MB         20 (29%)          -  0.0% ±  0.1%
  cpu_cycles          278M  ± 9.59M      270M  …  305M           7 (10%)        🚀-  8.5% ±  1.2%
  instructions        779M  ±  277       779M  …  779M           0 ( 0%)        🚀-  6.6% ±  0.0%
  cache_references   3.49M  ±  270K     3.15M  … 4.17M           4 ( 6%)        🚀-  3.8% ±  2.7%
  cache_misses        142K  ± 25.6K     86.0K  …  197K           0 ( 0%)        🚀- 32.0% ±  4.8%
  branch_misses      4.09M  ± 7.83K     4.08M  … 4.12M           1 ( 1%)          +  0.0% ±  0.1%
Benchmark 4 (69 runs): target/release/examples/uncompress-llvm-dfa-loop-match rs-chunked 4
  measurement          mean ± σ            min … max           outliers         delta
  wall_time          72.8ms ± 2.57ms    69.7ms … 80.0ms          7 (10%)        🚀-  6.3% ±  1.2%
  peak_rss           24.1MB ± 35.1KB    23.9MB … 24.1MB          2 ( 3%)          -  0.1% ±  0.1%
  cpu_cycles          281M  ± 10.1M      269M  …  312M           5 ( 7%)        🚀-  7.5% ±  1.2%
  instructions        778M  ±  243       778M  …  778M           0 ( 0%)        🚀-  6.7% ±  0.0%
  cache_references   3.45M  ±  277K     2.95M  … 4.14M           0 ( 0%)        🚀-  4.7% ±  2.7%
  cache_misses        176K  ± 43.4K      106K  …  301K           0 ( 0%)        🚀- 15.8% ±  6.3%
  branch_misses      4.16M  ± 96.0K     4.08M  … 4.37M           0 ( 0%)        💩+  1.7% ±  0.6%
```

The important points: `loop-match` is faster than `llfm-dfa`, and when combined performance is worse than when using `loop-match` on its own. 


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/258#issuecomment-2733772028">Comment by @traviscross posted on 2025-03-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Thanks for that update.  Have reached out separately.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/126'><strong>Instrument the Rust standard library with safety contracts</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="8"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/126#issuecomment-2731150866">Comment by @celinval posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We have been able to merge the initial support for contracts in the Rust compiler under the `contracts` unstable feature. @tautschnig has created the first PR to incorporate contracts in the standard library and uncovered a few limitations that we've been working on.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/259'><strong>Making compiletest more maintainable: reworking directive handling</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/259#issuecomment-2726093440">Comment by @jieyouxu posted on 2025-03-15:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update (2025-03-15):

- Doing a survey pass on compiletest to make sure I have the full picture.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/260'><strong>Metrics Initiative</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2695460081">Comment by @yaahc posted on 2025-03-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

After further review I've decided to limit scope initially and not get ahead of myself so I can make sure the schemas I'm working with can support the kind of queries and charts we're going to eventually want in the final version of the unstable feature usage metric. I'm hoping that by limiting scope I can have most of the items currently outlined in this project goal done ahead of schedule so I can move onto building the proper foundations based on the proof of concept and start to design more permanent components. As such I've opted for the following:

* minimal change to the current JSON format I need, which is including the timestamp
* Gain clarity on exactly what questions I should be answering with the unstable feature usage metrics, the desired graphs and tables, and how this influences what information I need to gather and how to construct the appropriate queries within graphana
* gathering a sample dataset from docs.rs rather than viewing it as the long term integration, since there are definitely some sampleset bias issues in that dataset from initial conversations with docs.rs
  * Figure out proper hash/id to use in the metrics file names to avoid collisions with different conditional compilation variants of the same crate with different feature enabled.

For the second item above I need to have more detailed conversations with both @rust-lang/libs-api and @rust-lang/lang 


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/122'><strong>Model coherence in a-mir-formality</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-2730965112">Comment by @nikomatsakis posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update:

@tiif has been working on integrating const-generic effects into a-mir-formality and making good progress.

I have begun exploring integration of the [MiniRust](https://github.com/minirust/minirust/blob/9ae11cc202d040f08bc13ec5254d3d41d5f3cc25/spec/lang/syntax.md#statements-terminators) definition of MIR. This doesn't directly work towards the goal of modeling coherence but it will be needed for const generic work to be effective. 

I am considering some simplification and cleanup work as well. 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/113'><strong>Next-generation trait solver</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-2729033787">Comment by @lcnr posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The two cycle handling PRs mentioned in the previous update have been merged, allowing `nalgebra`  to compile with the new solver enabled. I have now started to work on opaque types in borrowck again. This is a quite involved issue and will likely take a few more weeks until it's fully implemented.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/261'><strong>Nightly support for ergonomic SIMD multiversioning</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/261#issuecomment-2730590041">Comment by @veluca93 posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: Started investigating how the proposed SIMD multiversioning options might fit in the context of the efforts for formalizing a Rust effect system


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/262'><strong>Null and enum-discriminant runtime checks in debug builds</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-2730435572">Comment by @blyxyas posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Monthly update!

- https://github.com/rust-lang/rust-clippy/issues/13821 has been merged. This has successfully optimized the MSRV extraction from the source code.

On the old MSRV extraction,`Symbol::intern` use was sky high being about 3.5 times higher than the rest of the compilation combined. Now, it's at normal levels. Note that `Symbol::intern` is a very expensive and locking function, so this is very notable. Thanks to @Alexendoo for this incredible work!

As a general note on the month, I'd say that we've experimented a lot.

- Starting efforts on parallelizing the lint system.
- https://github.com/rust-lang/rust-clippy/issues/14423 Started taking a deeper look into our dependence on `libLLVM.so` and heavy relocation problems.
- I took a look into heap allocation optimization, seems that we are fine. For the moment, rust-clippy#14423 is the priority.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/106'><strong>Prepare const traits for stabilization</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="14"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/106#issuecomment-2740985917">Comment by @oli-obk posted on 2025-03-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I opened an RFC (https://github.com/rust-lang/rfcs/pull/3762) and we had a lang team meeting about it. Some design exploration and bikeshedding later we have settled on using (const)instead of ~const along with some more annotations for explicitness and some fewer annotations in other places. The RFC has been updated accordingly. There is still ongoing discussions about reintroducing the "fewer annotations" for redundancy and easier processing by humans.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/264'><strong>Prototype a new set of Cargo &quot;plumbing&quot; commands</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/265'><strong>Publish first rust-lang-owned release of &quot;FLS&quot;</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-2725594168">Comment by @JoelMarcey posted on 2025-03-14:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key Developments: Working on a public announcement of Ferrous' contribution of the FLS. Goal is to have that released soon. Also working out the technical details of the contribution, particularly around how to initially integrate the FLS into the Project itself.

Blockers: None yet.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-2770004340">Comment by @JoelMarcey posted on 2025-04-01:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key Developments: Public [announcement](https://rustfoundation.org/media/ferrous-systems-donates-ferrocene-language-specification-to-rust-project/) of the FLS donation to the Rust [Project](https://blog.rust-lang.org/2025/03/26/adopting-the-fls.html).

Blockers: None

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/266'><strong>Publish first version of StableMIR on crates.io</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-2741227092">Comment by @celinval posted on 2025-03-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We have proposed a project idea to Google Summer of Code to implement the refactoring and infrastructure improvements needed for this project. I'm working on breaking down the work into smaller tasks so they can be implemented incrementally.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-2741239158">Comment by @celinval posted on 2025-03-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I am also happy to share that @makai410 is joining us in this effort! 🥳 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/267'><strong>Research: How to achieve safety when linking separately compiled code</strong></a></div>
    <div style="flex: initial;"><progress value="3" max="11"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/268'><strong>Run the 2025H1 project goal program</strong></a></div>
    <div style="flex: initial;"><progress value="7" max="13"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/268#issuecomment-2694770347">Comment by @nikomatsakis posted on 2025-03-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update: February goal update [has been posted](https://blog.rust-lang.org/2025/03/03/Project-Goals-Feb-Update.html). We made significant revisions to the way that goal updates are prepared. If you are a goal owner, it's worth reading the directions for [how to report your status](https://rust-lang.github.io/rust-project-goals/how_to/report_status.html), especially the part about [help wanted](https://rust-lang.github.io/rust-project-goals/how_to/report_status.html#help-wanted-comments) and [summary](https://rust-lang.github.io/rust-project-goals/how_to/report_status.html#summary-comments) comments. 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/268#issuecomment-2730386220">Comment by @nikomatsakis posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update: We sent out the first round of pings for the March update. The plan is to create the document on **March 25th**, so @rust-lang/goal-owners please get your updates in by then. Note that you can create a [TL;DR comment](https://rust-lang.github.io/rust-project-goals/how_to/report_status.html#summary-comments) if you want to add 2-3 bullet points that will be embedded directly into the final blog post.

In terms of goal planning:

* @nandsh is planning to do a detailed retrospective on the goals program in conjunction with her research at CMU. Please reach out to her on Zulip (**Nandini**) if you are interested in participating.
* We are planning to overhaul the ping process as [described in this hackmd](https://hackmd.io/@spastorino/BJjZ0gf2Je). In short, pings will come on the 2nd/3rd Monday of the month. No pings will be sent if you've posted a comment that month. The blog post will be prepared on the 3rd Friday.
* We've been discussing how to structure 2025H2 goals and are thinking of making a few changes. We'll break out three categories of goals (Flagship / Core / Stretch), with "Core" goals being those deemed most important. We'll also have a 'pre-read' before the RFC opens with team leads to look for cross-team collaborative opportunities. At least that's the *current* plan.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/269'><strong>Rust Vision Document</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 

* We drafted a [Rust Vision Doc Action Plan](https://hackmd.io/5hKhzllDQYmOiw5uogybZg?both).
* We expect to publish our announcement blog post by end of Month including a survey requesting volunteers to speak with us. We are also creating plans for interviews with company contacts, global community groups, and Rust maintainers.

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/269#issuecomment-2730322014">Comment by @nikomatsakis posted on 2025-03-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update:

I've asked @jackh726 to co-lead the team with me. Together we pulled together a [Rust Vision Doc action plan](https://hackmd.io/5hKhzllDQYmOiw5uogybZg?both).

The plan begins by posting a blog post ([draft available here](https://hackmd.io/@rust-vision-doc/S1p_UNIoye)) announcing the effort. We are coordinating with the Foundation to create a survey which will be linked from the blog post. The [survey questions](https://hackmd.io/@rust-vision-doc/r1cqDGMn1x) ask about user's experience but also look for volunteers we can speak with.

We are pulling together the team that will perform the interviewing. We've been in touch with UX reseearchers who will brief us on some of the basics of UX research. We're finalizing team membership now plus the set of focus areas, we expect to cover at least users/companies, Rust project maintainers, and Rust global communities. See the [Rust Vision Doc action plan](https://hackmd.io/5hKhzllDQYmOiw5uogybZg?both) for more details.


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/275'><strong>rustc-perf improvements</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="7"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-2694687450">Comment by @davidtwco posted on 2025-03-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

A small update, @Jamesbarford aligned with @kobzol on a high-level architecture and will begin fleshing out the details and making some small patches to rustc-perf to gain familiarity with the codebase.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/118'><strong>Scalable Polonius support on nightly</strong></a></div>
    <div style="flex: initial;"><progress value="10" max="18"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-2747703673">Comment by @lqd posted on 2025-03-24:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Here are the key developments for this update.

Amanda has continued on the placeholder removal task. In particular on the remaining issues with rewritten type tests. The in-progress work caused incorrect errors to be emitted under the rewrite scheme, and a new strategy to handle these was discussed. This has been implemented in the PR, and seems to work as hoped. So the PR should now be in a state that is ready for more in-depth review pass, and should hopefully land soon.

Tage has started his master's thesis with a focus on the earliest parts of the borrow checking process, in order to experiment with graded borrow-checking, incrementalism, avoiding work that's not needed for loans that are not invalidated, and so on. A lot of great progress has been made on these parts already, and more are being discussed even in the later areas (live and active loans).

I have focused on taking care of the remaining diagnostics and test failures of the location-sensitive analysis. For diagnostics in particular, the PRs mentioned in the previous updates have landed, and I've fixed a handful of NLL spans, all the remaining differences under the compare-mode, and blessed differences that were improvements. For the test failures, handling liveness differently in traversal fixed most of the remaining failures, while a couple are due to the friction with mid-points avoidance scheme. For these, we have a few different paths forward, but with different trade-offs and we'll be discussing and evaluation these in the very near future. Another two are still left to analyze in-depth to see what's going on.

Our near future focus will be to continue down the path to correctness while also expanding test coverage that feels lacking in certain very niche areas, and that we want to improve. At the same time, we'll also work on a figuring out a better architecture to streamline the entire end-to-end process, to allow early outs, avoid work that is not needed, etc.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/271'><strong>Secure quorum-based cryptographic verification and mirroring for crates.io</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="12"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/119'><strong>Stabilize cargo-script</strong></a></div>
    <div style="flex: initial;"><progress value="31" max="36"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-2754835536">Comment by @lqd posted on 2025-03-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

This project goal was actually carried over from 2024h2, in https://github.com/rust-lang/rust-project-goals/pull/294

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/270'><strong>SVE and SME on AArch64</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="16"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-2694677621">Comment by @davidtwco posted on 2025-03-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

A small update, we've opened a draft PR for the initial implementation of this - rust-lang/rust#137944. Otherwise, just continued to address feedback on the RFCs.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-2733240803">Comment by @davidtwco posted on 2025-03-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- We've been resolving review feedback on the implementation of the Sized Hierarchy RFC on rust-lang/rust#137944. We're also working on reducing the performance regression in the PR, by avoiding unnecessary elaboration of sizedness supertraits and extending the existing `Sized` case in `type_op_prove_predicate` query's fast path.
- There's not been any changes to the RFC, there's minor feedback that has yet to be responded to, but it's otherwise just waiting on t-lang.
- We've been experimenting with rebasing rust-lang/rust#118917 on top of rust-lang/rust#137944 to confirm that const sizedness allows us to remove the type system exceptions that the SVE implementation previously relied on. We're happy to confirm that it does.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/273'><strong>Unsafe Fields</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="7"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/123'><strong>Use annotate-snippets for rustc diagnostic output</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="13"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/123#issuecomment-2766902313">Comment by @Muscraft posted on 2025-03-31:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

While my time was limited these past few months, lots of progress was made! I was able to align `annotate-snippets` internals with `rustc`'s [`HumanEmitter`](https://github.com/rust-lang/rust/blob/5cc60728e7ee10eb2ae5f61f7d412d9805b22f0c/compiler/rustc_errors/src/emitter.rs#L629) and get the new API implemented. These changes have not been merged yet, but [they can be found here](https://github.com/Muscraft/annotate-snippets-rs/tree/feedback). As part of this work, I got `rustc` using `annotate-snippets` as its only renderer. During all of this, I started working on making `rustc` use `annotate-snippets` as its only renderer, which turned out to be a huge benefit. I was able to get a feel for the new API while addressing rendering divergences. As of the time of writing, all but ~30 tests of the roughly 18,000 UI tests are passing.

```
test result: FAILED. 18432 passed; 29 failed; 193 ignored; 0 measured; 0 filtered out; finished in 102.32s
```

Most of the failing tests are caused by a few things:
- `annotate-snippets` right aligns numbers, whereas `rustc` left aligns
- `annotate-snippets` doesn't handle multiple suggestions for the same span very well
- Problems with handling `FailureNote`
- `annotate-snippets` doesn't currently support colored labels and titles, i.e., the magenta highlight `rustc` uses
- `rustc` wants to pass titles similar to `error: internal compiler error[E0080]`, but `annotate-snippets` doesn't support that well
- differences in how `rustc` and `annotate-snippets` handle term width during tests
  - When testing, `rustc` uses `DEFAULT_COLUMN_WIDTH` and does not subtract the code offset, while `annotate-snippets` does
- Slight differences in how "newline"/end of line highlighting is handled
- JSON output rendering contains color escapes


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>
