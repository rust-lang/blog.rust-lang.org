+++
path = "9999/12/31/july-project-goals-update"
title = "July Project Goals Update"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

The Rust project is currently working towards a [slate of 40 project goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html), with 3 of them designated as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

This is the final update for the first half of 2025. We're in the process of selecting goals for the second half of the year.

[Here are the goals that are currently proposed for 2025H2](https://rust-lang.github.io/rust-project-goals/2025h2/goals.html).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="14" max="34"></progress>
</div>
</div>
<!-- markdown separator -->

**Why this goal?** This work continues our drive to improve support for async programming in Rust. In 2024H2 we stabilized async closures; explored the generator design space; and began work on the `dynosaur` crate, an experimental proc-macro to provide dynamic dispatch for async functions in traits. In 2025H1 [our plan](https://rust-lang.github.io/rust-project-goals/2025h1/async.html) is to deliver (1) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (2) progress towards sync and async generators, simplifying the creation of iterators and async data streams; (3) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.


**[H1 Recap from @tmandry:](https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-3081983979)**

**What went well**: This cycle we saw significant progress in a few areas:

- We had productive conversations with the language team on generators, and landed an experimental implementation for a builtin `iter!` macro that implements unpinned generators.
- We shipped async closures and the new lifetime capture rules as part of Rust 2024.
- We developed a proc macro, [dynosaur](https://crates.io/crates/dynosaur), that can be used to support `async fn` together with `dyn Trait`.
- We landed an early-stage experiment to support `async Drop` in the compiler.
- We landed an experimental implementation of autoreborrowing for pinned references, along with a number of other improvements for pin ergonomics.

**What didn't:** In some areas, we didn't make as much progress as we hoped. In retrospect, the scope of this goal was too large for one person to manage. With flagship project goals, there this a desire to paint a grand vision that I think would be better served by another mechanism without a time bound on it. I've been calling this a "north star".

In some cases, like RTN, progress has been by technical debt in the Rust compiler's type system. For that there is an ongoing project goal to replace the trait solver with a next-generation version. Finally, on the design front, progress is sometimes slowed by uncertainty and disagreement around the future of pinning in the Rust language.

**Looking forward:** My takeaway from this is that in the next project goals cycle, we should focus on answering more fundamental questions of Rust's evolution. These should reduce uncertainty and pave the way for us to unblock major features for async in future cycles. For example, how far we can push [pin ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html)? What approach should we take for [in-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html), and can it support `async fn` in `dyn Trait`? How will we support [evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/evolving-traits.html) in a general way that allows us to support the Tower "middleware" pattern with `async fn`?

I'm excited by the lineup of goals we have for this next cycle. See you on the other side!

<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-3081983979">Comment by @tmandry posted on 2025-07-17:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

[dynosaur v0.3](https://github.com/spastorino/dynosaur/releases/tag/0.3.0) has been released. This release contains some breaking changes in preparation for an upcoming 1.0 release. See the linked release notes for more details.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-3134566343">Comment by @tmandry posted on 2025-07-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**H1 Recap**

**What went well**: This cycle we saw significant progress in a few areas:

- We had productive conversations with the language team on generators, and landed an experimental implementation for a builtin `iter!` macro that implements unpinned generators.
- We shipped async closures and the new lifetime capture rules as part of Rust 2024.
- We developed a proc macro, [dynosaur](https://crates.io/crates/dynosaur), that can be used to support `async fn` together with `dyn Trait`.
- We landed an early-stage experiment to support `async Drop` in the compiler.
- We landed an experimental implementation of autoreborrowing for pinned references, along with a number of other improvements for pin ergonomics.

**What didn't:** In some areas, we didn't make as much progress as we hoped. In retrospect, the scope of this goal was too large for one person to manage. With flagship project goals, there this a desire to paint a grand vision that I think would be better served by another mechanism without a time bound on it. I've been calling this a "north star".

In some cases, like RTN, progress has been by technical debt in the Rust compiler's type system. For that there is an ongoing project goal to replace the trait solver with a next-generation version. Finally, on the design front, progress is sometimes slowed by uncertainty and disagreement around the future of pinning in the Rust language.

**Looking forward:** My takeaway from this is that in the next project goals cycle, we should focus on answering more fundamental questions of Rust's evolution. These should reduce uncertainty and pave the way for us to unblock major features for async in future cycles. For example, how far we can push [pin ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html)? What approach should we take for [in-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html), and can it support `async fn` in `dyn Trait`? How will we support [evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/evolving-traits.html) in a general way that allows us to support the Tower "middleware" pattern with `async fn`?

I'm excited by the lineup of goals we have for this next cycle. See you on the other side!

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/263'><strong>Organize Rust All-Hands 2025</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Status: Completed!"></img>
</div>
</div>
<!-- markdown separator -->

**Why this goal?** May 15, 2025 marks the 10-year anniversary of Rust's 1.0 release; it also marks 10 years since the [creation of the Rust subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). At the time [there were 6 Rust teams with 24 people in total](http://web.archive.org/web/20150517235608/http://www.rust-lang.org/team.html). There are now 57 teams with 166 people. In-person All Hands meetings are an effective way to help these maintainers get to know one another with high-bandwidth discussions. This year, the Rust project will be coming together for [RustWeek 2025](https://2025.rustweek.org), a joint event organized with [RustNL](https://2025.rustweek.org/about/). Participating project teams will use the time to share knowledge, make plans, or just get to know one another better. One particular goal for the All Hands is reviewing a draft of the [Rust Vision Doc](./rust-vision-doc.md), a document that aims to take stock of where Rust is and lay out high-level goals for the next few years.

**What has happened?**

<!-- markdown separator -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Stabilize tooling needed by Rust for Linux</strong></a></div>
    <div style="flex: initial;"><progress value="12" max="26"></progress>
</div>
</div>
<!-- markdown separator -->

**Why this goal?** This goal continues our work from 2024H2 in supporting the [experimental support for Rust development in the Linux kernel][RFL.com]. Whereas in 2024H2 we were focused on stabilizing required language features, our focus in 2025H1 is stabilizing compiler flags and tooling options. We will (1) implement [RFC #3716] which lays out a design for ABI-modifying flags; (2) take the first step towards stabilizing [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html); (3) extending rustdoc, clippy, and the compiler with features that extract metadata for integration into other build systems (in this case, the kernel's build system).

[RFC #3716]: https://github.com/rust-lang/rfcs/pull/3716
[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

**What has happened?**

* [@Ding](https://github.com/dingxiangfei2009) opened a [PR#142518](https://github.com/rust-lang/rust/pull/142518) that implements the [in-place initialization experiment](https://github.com/rust-lang/lang-team/issues/336).
* @Ding is working on an experimental implementation ([PR#143527](https://github.com/rust-lang/rust/pull/143527)) for `arbitrary_self_types`.
* Ding opened a PR to Clang (a C frontend for LLVM): [Queries on GCC-style inline assembly statements](https://github.com/llvm/llvm-project/pull/143424) and got it merged.
* [@ojeda](https://github.com/ojeda) opened two Rust for Linux goals for the next period:
    * <https://github.com/rust-lang/rust-project-goals/pull/347>
	* <https://github.com/rust-lang/rust-project-goals/pull/346>

<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3045728071">Comment by @tomassedovic posted on 2025-07-07:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

## In-place initialization ##

Ding opened a [PR#142518](https://github.com/rust-lang/rust/pull/142518) that implements the [in-place initialization experiment](https://github.com/rust-lang/lang-team/issues/336).

## `arbitrary_self_types`

Ding is working on an experimental implementation ([PR#143527](https://github.com/rust-lang/rust/pull/143527)).

## Queries on GCC-style inline assembly statements:

Ding opened a PR to Clang (a C frontend for LLVM): https://github.com/llvm/llvm-project/pull/143424 and got it merged.

This is part of the LLVM/Clang issues the Rust for Linux project needs: https://github.com/Rust-for-Linux/linux/issues/1132.

## `-Zindirect-branch-cs-prefix`:

We've discussed whether this needs to be a separate target feature vs. a modifier on the existing `retpoline` one. Josh argued that since having this enabled without retpoline doesn't make sense, it should be a modifier. On the other hand, Miguel mentioned that it would be clearer on the user's side (easier to map the names from GCC and Clang to `rustc` when they're the same and see that we're enabling the same thing in Rust and Linux kernel's `Makefiles`).

Ultimately, this is a compiler question and should be resolved here: https://github.com/rust-lang/rust/pull/140740

Note: `-Cmin-function-alignment` will be another similar case.

## Stabilizing `AddressSanitizer` and `LeakSanitizer`:

* https://github.com/rust-lang/rust/pull/123617
* https://github.com/rust-lang/rust/pull/142681

In light of the newly-proposed `#[sanitize(xyz = "on|off")]` syntax, we've discussed whether it makes sense to add a shorthand to enable/disable all of them at once (e.g. `#[sanitize(all = "on|off")]`). The experience from the field suggests that this is rarely something people do.

We've also discussed what values should the options have (e.g. `"yes"`/`"no"` vs. `"on"`/`"off"` or `true`/`false`). No strong preferences, but in case of an error, the compiler should suggest the correct value to use.

P.S.: There will be a Lang design meeting regarding in-place initialization on Wednesday 2025-07-30: https://github.com/rust-lang/lang-team/issues/332.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-3089525491">Comment by @tomassedovic posted on 2025-07-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

## 2025H2 Goals

@ojeda proposed two goals to move the effort forward: one for the language and the other for the compiler.

* https://github.com/rust-lang/rust-project-goals/pull/347
* https://github.com/rust-lang/rust-project-goals/pull/346

## Ongoing work updates

@dingxiangfei2009 drafted a [Pre-RFC](https://hackmd.io/@rust-for-linux-/SkucBLsWxl) for the supertrait-item-in-subtrait-impl work. Need to add two modifications to the RFC to incorporate t-lang requests.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>




## Goals looking for help

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Promoting Parallel Front End</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div>
<!-- markdown separator -->

<!-- markdown separator -->


<!-- markdown separator -->
*Help wanted:* Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3AA-parallel-compiler) and try to reproduce the issue
<!-- markdown separator -->


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/121#issuecomment-3059845260">Comment by @SparrowLii posted on 2025-07-11:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

* **Key developments:** We bring rustc-rayon in rustc's working tree, the [PR](https://github.com/rust-lang/rust/pull/143035) that fixes several deadlock issues has been merged.
* **Blockers:** null
* **Help wanted:** Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3AA-parallel-compiler) and try to reproduce the issue

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
*Help wanted:* this project goal needs a compiler developer to move forward.
<!-- markdown separator -->


<!-- markdown separator -->


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/272#issuecomment-3057667265">Comment by @epage posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Help wanted: this project goal needs a compiler developer to move forward.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/272#issuecomment-3061279359">Comment by @sladyn98 posted on 2025-07-11:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@epage  hey i would like to help contribute with this, if you could probably mentor me in the right direction, i could learn and ramp up and move this forward, i could start with some tasks, scope them out into small bite sized chunks and contribute

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/272#issuecomment-3062406737">Comment by @epage posted on 2025-07-11:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

This is mostly in the compiler atm and I'm not in a position to mentor or review compiler changes; my first compiler PR is being merged right now.  I'm mostly on this from the Cargo side and overall coordination.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/119'><strong>Stabilize cargo-script</strong></a></div>
    <div style="flex: initial;"><progress value="31" max="37"></progress>
</div>
</div>
<!-- markdown separator -->


*Help wanted*: I'll be working towards verifying rustfmt, rust-analyzer, and other tooling support and will be needing at least reviews from people, if not some mentorship.


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-3057687450">Comment by @epage posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments:
* @epage is shifting attention back to this now that toml v0.9 is out
* `-Zunpretty` support is being added in rust-lang/rust#143708

Blockers

Help wanted
* I'll be working towards verifying rustfmt, rust-analyzer, and other tooling support and will be needing at least reviews from people, if not some mentorship.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-3119584346">Comment by @BoxyUwU posted on 2025-07-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Not much to say since the last update- I have been focused on other areas of const generics and I believe camelid has been relatively busy with other things too. I intend for the next const generics project goal to be more broadly scoped than just `min_generic_const_args` so that other const generics work can be given a summary here :)

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/274'><strong>build-std</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator -->

- Discussed the latest round of feedback on the pre-RFC, the most significant of which is that the scope of the RFC is almost certainly too large for an MVP.
- @davidtwco presented a reformulation of the plan which focuses on the core components of build-std and leaves more features for future extensions after a minimal MVP:
  - Stage 1a: Introduce manual controls for enabling the build-std behavior in Cargo.
  - Stage 1b: Introduce Cargo syntax to declare explicit dependencies on core, alloc and std crates.
    - This stage enables the use of Tier 3 targets on stable Rust and allows the ecosystem to start transitioning to explicit dependencies on the standard library.
    - This stage would be considered the minimal MVP.
  - Stage 2: Teach Cargo to build std with different codegen/target modifier options.
    - This stage allows the standard library to be compiled with custom codegen options.
  - Stage 3: Enable automatic standard library rebuilds.
    - This stage focuses on making build-std behave ergonomically and naturally without users having to manually ask for the standard library to be built.
- General consensus was reached that this plan feels viable. @davidtwco will write the Stage 1a/b RFC.
- Submitted a [2025H2 goal proposal](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html)


<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3103580370">Comment by @wesleywiser posted on 2025-07-22:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Updates from our biweekly sync call:
  - Discussed the latest round of feedback on the pre-RFC, the most significant of which is that the scope of the RFC is almost certainly too large for an MVP.
  - @davidtwco presented a reformulation of the plan which focuses on the core components of build-std and leaves more features for future extensions after a minimal MVP:
    - Stage 1a: Introduce manual controls for enabling the build-std behavior in Cargo.
    - Stage 1b: Introduce Cargo syntax to declare explicit dependencies on core, alloc and std crates.
      - This stage enables the use of Tier 3 targets on stable Rust and allows the ecosystem to start transitioning to explicit dependencies on the standard library.
      - This stage would be considered the minimal MVP.
    - Stage 2: Teach Cargo to build std with different codegen/target modifier options.
      - This stage allows the standard library to be compiled with custom codegen options.
    - Stage 3: Enable automatic standard library rebuilds.
      - This stage focuses on making build-std behave ergonomically and naturally without users having to manually ask for the standard library to be built.
  - General consensus was reached that this plan feels viable. @davidtwco will write the Stage 1a/b RFC.
  - Some discussion on various threads from the previous RFC draft.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-3130048243">Comment by @wesleywiser posted on 2025-07-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Continuing the build-std work has been submitted as a Project Goal for 2025H2: https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="5"></progress>
</div>
</div>
<!-- markdown separator -->

Belated update for May and June: RustWeek was _extremely_ productive! It was great to sit down in a room with all the stakeholders and talk about what it would take to get cross-crate linting working reliably at scale.

As a result of this work we identified a lot of previously-unknown blockers, as well as some paths forward. More work remains, but it's nice that we now have a much better idea of what that work should look like.

TL;DR:
- `?Sized` linting is blocked since it [requires additional data in rustdoc JSON](https://github.com/rust-lang/rust/issues/143197).
  - Currently we get information on the _syntactic_ presence of `?Sized`. But another bound might be implying `Sized`, which makes `?Sized` not true overall.
  - Failing to account for this would mean we get both false negatives and false positives. This is effectively a dual of the the "implied bounds" issue in the previous post.
- Cross-crate linting has had some positive movement, and some additional blockers identified.
  - docs.rs has begun hosting rustdoc JSON, allowing us to use it as a cache to avoid rebuilding rustdoc JSON in cross-crate linting scenarios where those builds could get expensive.
  - We need a way to determine which features in dependencies are active (recursively) given a set of features active in the the top crate, so we know how to generate accurate rustdoc JSON. That information is not currently available via the lockfile or any cargo interface.
  - We need to work with the rustdoc and cargo teams to make it possible to use rmeta files to correctly combine data across crates. This has many moving parts and will take time to get right, but based on in-person conversations at RustWeek we all agreed was the best and most reliable path forward.
- Other improvements to `cargo-semver-checks` are ongoing: a full set of `#[target_feature]` lints ships in the next release, and two folks participating in Google Summer of Code have begun contributing to `cargo-semver-checks` already!

While the targets for the 2025H1 goals proved a bit too ambitious to hit in this timeline, I'm looking forward to continuing my work on the goal in the 2025H2 period!


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-3036860610">Comment by @obi1kenobi posted on 2025-07-04:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Belated update for May and June: RustWeek was _extremely_ productive! It was great to sit down in a room with all the stakeholders and talk about what it would take to get cross-crate linting working reliably at scale.

As a result of this work we identified a lot of previously-unknown blockers, as well as some paths forward. More work remains, but it's nice that we now have a much better idea of what that work should look like.

TL;DR:
- `?Sized` linting is blocked since it [requires additional data in rustdoc JSON](https://github.com/rust-lang/rust/issues/143197).
  - Currently we get information on the _syntactic_ presence of `?Sized`. But another bound might be implying `Sized`, which makes `?Sized` not true overall.
  - Failing to account for this would mean we get both false negatives and false positives. This is effectively a dual of the the "implied bounds" issue in the previous post.
- Cross-crate linting has had some positive movement, and some additional blockers identified.
  - docs.rs has begun hosting rustdoc JSON, allowing us to use it as a cache to avoid rebuilding rustdoc JSON in cross-crate linting scenarios where those builds could get expensive.
  - We need a way to determine which features in dependencies are active (recursively) given a set of features active in the the top crate, so we know how to generate accurate rustdoc JSON. That information is not currently available via the lockfile or any cargo interface.
  - We need to work with the rustdoc and cargo teams to make it possible to use rmeta files to correctly combine data across crates. This has many moving parts and will take time to get right, but based on in-person conversations at RustWeek we all agreed was the best and most reliable path forward.
- Other improvements to `cargo-semver-checks` are ongoing: a full set of `#[target_feature]` lints ships in the next release, and two folks participating in Google Summer of Code have begun contributing to `cargo-semver-checks` already!

While the targets for the 2025H1 goals proved a bit too ambitious to hit in this timeline, I'm looking forward to continuing my work on the goal in the 2025H2 period!

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/252'><strong>Declarative (&#x60;macro_rules!&#x60;) macro improvements</strong></a></div>
    <div style="flex: initial;"><progress value="15" max="29"></progress>
</div>
</div>
<!-- markdown separator -->


Current status:
- @joshtriplett authored RFCs for both [attribute macros](https://github.com/rust-lang/rfcs/pull/3697) and [derive macros](https://github.com/rust-lang/rfcs/pull/3698).
- After some further iteration with the lang team, both RFCs were accepted and merged.
- @joshtriplett, @eholk, and @vincenzopalazzo did some successful group-spelunking into the implementation of macros in rustc.
- @joshtriplett [rewrote the `macro_rules!` parser](https://github.com/rust-lang/rust/pull/143070/), which enabled future extensibility *and* resulted in better error messages. This then enabled several follow-up refactors and simplifications.
- @joshtriplett wrote a PR implementing attribute macros.



<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/252#issuecomment-3095959868">Comment by @joshtriplett posted on 2025-07-21:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Current status:
- @joshtriplett authored RFCs for both [attribute macros](https://github.com/rust-lang/rfcs/pull/3697) and [derive macros](https://github.com/rust-lang/rfcs/pull/3698).
- After some further iteration with the lang team, both RFCs were accepted and merged.
- @joshtriplett, @eholk, and @vincenzopalazzo did some successful group-spelunking into the implementation of macros in rustc.
- @joshtriplett [rewrote the `macro_rules!` parser](https://github.com/rust-lang/rust/pull/143070/), which enabled future extensibility *and* resulted in better error messages. This then enabled several follow-up refactors and simplifications.
- @joshtriplett wrote a PR implementing attribute macros (review in progress).

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/252#issuecomment-3132736622">Comment by @joshtriplett posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update: Implementation PR for attribute macros is up.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/253'><strong>Evaluate approaches for seamless interop between C++ and Rust</strong></a></div>
    <div style="flex: initial;"><progress value="4" max="6"></progress>
</div>
</div>
<!-- markdown separator -->

**Recap** by @tmandry:

This project goals cycle was important for C++ interop. With the language team we [established that we should evolve Rust to enable a first-class C++ interop story](https://hackmd.io/2Ar_7CNoRkeXk1AARyOL7A?view), making rich and automatic bindings possible between the two languages. At the Rust All Hands, [people from across the industry met](https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-2901991797) to describe their needs to each other, what is working for them, and what isn't. This process of discovery has led to a lot of insight into where we can make progress now and ideas for what it will take to really "solve" interop.

One thing I think we can say with certainty is that interop is a vast problem space, and that any two groups who want interop are very likely to have different specific needs. I'm excited about the project goal proposal by @baumanj to begin [mapping this problem space](https://rust-lang.github.io/rust-project-goals/2025h2/interop-problem-map.html) out in the open, so that as we refer to problems we can better understand where our needs overlap and diverge.

Despite the diversity of needs, we've noticed that there is quite a bit of overlap when it comes to language evolution. This includes many features requested by Rust for Linux, a flagship customer of the Rust project. In retrospect, this is not surprising: Rust for Linux needs fine-grained interop with C APIs, which is roughly a subset of the needs for interop with C++ APIs. Often the need runs deeper than interop, and is more about supporting patterns in Rust that existing systems languages already support as a first-class feature.

I'm looking forward to tackling areas where we can "extend the fundamentals" of Rust in a way that makes these, and other use cases, possible. This includes H2 project goal proposals like [pin ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html), [reborrowing](https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html), [field projections](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html), and [in-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html).

Thanks to everyone who contributed to the discussions this past cycle. Looking forward to seeing you in the next one!




<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-3134117709">Comment by @tmandry posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Ahead of the all hands, @cramertj and @tmandry collaborated on a prototype called [ecdysis](https://github.com/cramertj/rust/tree/ecdysis) that explored the viability of instantiating types "on-demand" in the Rust compiler. These types are intended to look like C++ template instantiations. The prototype was a success in that it made the direction look viable and also surfaced some foundational work that needs to happen in the compiler first. That said, continuing to pursue it is not the highest priority for either of us at the moment.

Many thanks to @oli-obk for their advice and pointers.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-3134165233">Comment by @tmandry posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Recap**

This project goals cycle was important for C++ interop. With the language team we [established that we should evolve Rust to enable a first-class C++ interop story](https://hackmd.io/2Ar_7CNoRkeXk1AARyOL7A?view), making rich and automatic bindings possible between the two languages. At the Rust All Hands, [people from across the industry met](https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-2901991797) to describe their needs to each other, what is working for them, and what isn't. This process of discovery has led to a lot of insight into where we can make progress now and ideas for what it will take to really "solve" interop.

One thing I think we can say with certainty is that interop is a vast problem space, and that any two groups who want interop are very likely to have different specific needs. I'm excited about the project goal proposal by @baumanj to begin [mapping this problem space](https://rust-lang.github.io/rust-project-goals/2025h2/interop-problem-map.html) out in the open, so that as we refer to problems we can better understand where our needs overlap and diverge.

Despite the diversity of needs, we've noticed that there is quite a bit of overlap when it comes to language evolution. This includes many features requested by Rust for Linux, a flagship customer of the Rust project. In retrospect, this is not surprising: Rust for Linux needs fine-grained interop with C APIs, which is roughly a subset of the needs for interop with C++ APIs. Often the need runs deeper than interop, and is more about supporting patterns in Rust that existing systems languages already support as a first-class feature.

I'm looking forward to tackling areas where we can "extend the fundamentals" of Rust in a way that makes these, and other use cases, possible. This includes H2 project goal proposals like [pin ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html), [reborrowing](https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html), [field projections](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html), and [in-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html).

Thanks to everyone who contributed to the discussions this past cycle. Looking forward to seeing you in the next one!

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Experiment with ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="6" max="8"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-3020689056">Comment by @spastorino posted on 2025-06-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We're currently working on the last-use optimization. We've the liveness analysis needed implemented and we need to extensively test it.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="4"></progress>
</div>
</div>
<!-- markdown separator -->

@ZuseZ4:

The last update for this project-goal period! I have continued to work on the gpu support, while our two Rust/LLVM autodiff gsoc students made great progress with their corresponding projects.

**Key developments:**

1) My memory-movement PR got reviewed and after a few iterations landed in nightly. That means you can now don't even have to build your own rustc to move data to and from a GPU (with the limitations mentioned in my previous post). As part of my PR, I also updated the rustc-dev-guide: https://rustc-dev-guide.rust-lang.org/offload/installation.html

2) Now that the host (CPU) code landed, I looked into compiling rust kernels to GPUs. When experimenting with the amdgcn target for rustc I noticed a regression, due to which all examples for that target failed. I submitted a small patch to fix it. It landed a few days ago, and prevents rustc from generating f128 types on AMD GPUs: https://github.com/rust-lang/rust/pull/144383

3) I looked into HIP and OpenMP (managed/kernel-mode) examples to see what's needed to launch the kernels. I should already have most of the code upstream, since it landed as part of my host PR, so I think I should soon be able to add the remaining glue code to start running Rust code on GPUs. https://github.com/rust-lang/rust/pull/142696.

4) The main PR of @KMJ-007 is up, to start generating typetrees for Enzyme, the backend of our std::autodiff module. Enzyme sometimes wants more information about a type than it can get from LLVM, so it either needs to deduce it (slow), or it will fail to compile (bad). In the future we hope to lower MIR information to Enzyme, and this is the first step for it. I just submitted the first round of reviews: https://github.com/rust-lang/rust/pull/142640

5) The main PR of @Sa4dUs is up, it replaces my historically grown middle-end with a proper rustc-autodiff-intrinsic. This allows us to remove a few hacks and thus makes it easier to maintain. It will also handle more corner-cases, and reduces the amount of autodiff related code in rustc by ~400 lines. I also gave it a first review pass.


I also submitted an updated project-goal to finish the `std::offload` module, to the point where we can write an interesting amount of kernels in pure (nightly) Rust and launch them to GPUs. All new project goals are supposed to have "champions" from the teams they are related to, which in the case of my autodiff/batching/offload work would be t-compiler and t-lang (see Niko's blog post for more details). Since I joined the compiler team a while ago I can now champion for it myself on the compiler side, and @traviscross volunteered to continue the support on the language side, thank you!


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-3134501597">Comment by @ZuseZ4 posted on 2025-07-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The last update for this project-goal period! I have continued to work on the gpu support, while our two Rust/LLVM autodiff gsoc students made great progress with their corresponding projects.

**Key developments:**

1) My memory-movement PR got reviewed and after a few iterations landed in nightly. That means you can now don't even have to build your own rustc to move data to and from a GPU (with the limitations mentioned in my previous post). As part of my PR, I also updated the rustc-dev-guide: https://rustc-dev-guide.rust-lang.org/offload/installation.html

2) Now that the host (CPU) code landed, I looked into compiling rust kernels to GPUs. When experimenting with the amdgcn target for rustc I noticed a regression, due to which all examples for that target failed. I submitted a small patch to fix it. It landed a few days ago, and prevents rustc from generating f128 types on AMD GPUs: https://github.com/rust-lang/rust/pull/144383

3) I looked into HIP and OpenMP (managed/kernel-mode) examples to see what's needed to launch the kernels. I should already have most of the code upstream, since it landed as part of my host PR, so I think I should soon be able to add the remaining glue code to start running Rust code on GPUs. https://github.com/rust-lang/rust/pull/142696.

4) The main PR of @KMJ-007 is up, to start generating typetrees for Enzyme, the backend of our std::autodiff module. Enzyme sometimes wants more information about a type than it can get from LLVM, so it either needs to deduce it (slow), or it will fail to compile (bad). In the future we hope to lower MIR information to Enzyme, and this is the first step for it. I just submitted the first round of reviews: https://github.com/rust-lang/rust/pull/142640

5) The main PR of @Sa4dUs is up, it replaces my historically grown middle-end with a proper rustc-autodiff-intrinsic. This allows us to remove a few hacks and thus makes it easier to maintain. It will also handle more corner-cases, and reduces the amount of autodiff related code in rustc by ~400 lines. I also gave it a first review pass.


I also submitted an updated project-goal to finish the `std::offload` module, to the point where we can write an interesting amount of kernels in pure (nightly) Rust and launch them to GPUs. All new project goals are supposed to have "champions" from the teams they are related to, which in the case of my autodiff/batching/offload work would be t-compiler and t-lang (see Niko's blog post for more details). Since I joined the compiler team a while ago I can now champion for it myself on the compiler side, and @traviscross volunteered to continue the support on the language side, thank you!

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
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@Eh2406:

My time at Amazon is coming to an end. They supported the very successful effort with the 2024h2 goal, and encouraged me to propose the 2025h1 goal that is now wrapping up. Unfortunately other work efforts led to the very limited progress on the 2025h1 goal. I do not know what comes next, but it definitely involves taking time to relax and recover. Recovering involves rediscovering the joy in the work that I love. And, I have a deep passion for this problem. I hope to make some time to work on this. But, relaxing requires reducing the commitments I have made to others and the associated stress. So I will not promise progress, nor will I renew the goal for 2025h2.


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-3028998355">Comment by @Eh2406 posted on 2025-07-02:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

My time at Amazon is coming to an end. They supported the very successful effort with the 2024h2 goal, and encouraged me to propose the 2025h1 goal that is now wrapping up. Unfortunately other work efforts led to the very limited progress on the 2025h1 goal. I do not know what comes next, but it definitely involves taking time to relax and recover. Recovering involves rediscovering the joy in the work that I love. And, I have a deep passion for this problem. I hope to make some time to work on this. But, relaxing requires reducing the commitments I have made to others and the associated stress. So I will not promise progress, nor will I renew the goal for 2025h2.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-3118612290">Comment by @tomassedovic posted on 2025-07-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Thank you for everything Jacob and good luck!

As the 2025 H1 period is coming to an end and we're focusing on the goals for the second half of the year, we will close this issue by the end of this month (July 2025).

If you or someone else out there is working on this and has updates to share, please add them as a comment here by 2025-07-29 so they can be included in the final blog post.

Even after the issue is closed, the work here *can* be picked up -- we'll just no longer track it as part of the 2025H1 goals effort.

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
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-3057692058">Comment by @epage posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments:

Blockers
* Staffing wise, attention was taken by toml v0.9 and now cargo-script

Help wanted
* Help in writing out the end-user API on top of the raw harness

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-3129973078">Comment by @epage posted on 2025-07-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments:
- https://github.com/assert-rs/libtest2/pull/94
- https://github.com/assert-rs/libtest2/pull/99
- https://github.com/assert-rs/libtest2/pull/100

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-3129899632">Comment by @b-naber posted on 2025-07-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Chiming in for @epage here since further progress is still blocked on the compiler implementation. Unfortunately things have been moving more slowly than I had initially hoped. We have been doing some refactoring (https://github.com/rust-lang/rust/pull/142547 and https://github.com/rust-lang/rust/pull/144131) that allow us to introduce a new `Scope` for namespaced crates inside name resolution. There's a draft PR (https://github.com/rust-lang/rust/pull/140271) that should be straightforward to adapt to the refactoring.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

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



<!-- markdown separator -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/126'><strong>Instrument the Rust standard library with safety contracts</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="8"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/126#issuecomment-3033658101">Comment by @celinval posted on 2025-07-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Unfortunately, we didn't make much progress since April except for a very useful discussion during Rust all hands. A few notes can be found here: https://hackmd.io/@qnR1-HVLRx-dekU5dvtvkw/SyUuR6SZgx. We're still waiting for the design discussion meeting with the compiler team.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/126#issuecomment-3120266419">Comment by @celinval posted on 2025-07-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@dawidl022 is working as part of GSoC to improve contracts implementation under @tautschnig mentorship. Additionally, @tautschnig and @carolynzech are working on porting contracts from https://github.com/model-checking/verify-rust-std to the Rust repo.

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/260'><strong>Metrics Initiative</strong></a></div>
    <div style="flex: initial;"><progress value="6" max="7"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-3063856749">Comment by @yaahc posted on 2025-07-11:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

No update for this month beyond the previous finalish update. I still intend to publish the json->influxdb conversion code

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/113'><strong>Next-generation trait solver</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3068877231">Comment by @lcnr posted on 2025-07-14:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We - or well, overwhelmingly @compiler-errors - continued to make performance improvements to the new solver over the last month: https://github.com/rust-lang/rust/pull/142802 https://github.com/rust-lang/rust/pull/142732 https://github.com/rust-lang/rust/pull/142317 https://github.com/rust-lang/rust/pull/142316 https://github.com/rust-lang/rust/pull/142223 https://github.com/rust-lang/rust/pull/142090 https://github.com/rust-lang/rust/pull/142088 https://github.com/rust-lang/rust/pull/142085 https://github.com/rust-lang/rust/pull/141927 https://github.com/rust-lang/rust/pull/141581 https://github.com/rust-lang/rust/pull/141451. `nalgebra` is currently 70% slower than with the old solver implementation and we seem to be about 30-50% slower in most *normal* crates.

I've been working on strengthening the search graph to avoid the hang in rayon and https://github.com/rust-lang/trait-system-refactor-initiative/issues/210 in a principled way. This has been more challenging than expected and will take at least another week to get done.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-3131816067">Comment by @lcnr posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Since the last update @compiler-errors landed two additional perf optimizations: https://github.com/rust-lang/rust/pull/143500 https://github.com/rust-lang/rust/pull/143309.

I am still working on the hang in rayon and https://github.com/rust-lang/trait-system-refactor-initiative/issues/210. I've ended up having to change the invariants of the type system to support a fast paths based on structural identity, e.g. quickly proving `T: Trait<'a>` via a `T: Trait<'a>` where-bound, in https://github.com/rust-lang/rust/pull/144405. Changing this invariant requires some additional work in HIR typeck, so I am currently reducing the perf impact of that change.

With this I can then land the actual fast paths which fix both rayon and similar hangs due to a large number of where-bounds. This should also be done soon. I will then go back to implement the new opaque type handling approach as that's the only remaining issue before we can call for testing.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/261#issuecomment-3057901302">Comment by @veluca93 posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: https://github.com/rust-lang/rust/issues/143352 proposes an experimental feature to investigate an effect-based approach to integrate generics and target features, effectively giving ways to have different monomorphizations of a function have different target features.

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/262#issuecomment-3118996993">Comment by @1c3t3a posted on 2025-07-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments**: [Landed](https://github.com/rust-lang/rust/pull/141759) the enum discriminant check and enabled it for transmutes to enums for now (this is not so powerful), currently [extending](https://github.com/rust-lang/rust/pull/144353) it to union reads and pointer reads.

**Blockers:** question of how to insert a check if we already observe UB (e.g. the enum is only represented by an i1 in LLVM IR). This is to be addressed by the next project goal: https://rust-lang.github.io/rust-project-goals/2025h2/comprehensive-niche-checks.html.



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>
<!-- markdown separator -->

@blyxyas:

**Final monthly update!**

- Even more optimizations have been achieved on the documentation lints front. https://github.com/rust-lang/rust-clippy/pull/15030. (-6.7% on `bumpalo`).

- The 3rd heaviest function was optimized away by 99.75%, along with the [`strlen_on_c_strings`](https://rust-lang.github.io/rust-clippy/master/index.html#strlen_on_c_strings) lint. This gives us about a 15% optimization on `tokio`. https://github.com/rust-lang/rust-clippy/pull/15043

- As a minor improvement, we now instantiate a lot less types on `unit_return_expecting_ord` (89% less calls in some benchmarks). This saves us a lot of locks on the type interner.

As a final update to the project goal, I'd like to say a little bit more:

I'm very happy with how this project goal has turned out. We've seen improvements in the 35-60% range for your real world projects and while I couldn't deliver the two objectives the project goal promised because of an excess in ambition, I still don't think that these are too far-fetched by any means.

As some specific examples, you can now witness a **38%** performance improvements in analyzing Cargo, and a **61%** in analyzing Tokio!

Much more to come, and thanks for sticking by while we make Clippy a better project, with better developer experience.
Have a great week, and I hope that you can enjoy all the performance improvements that we've delivered across this project goal.


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-3013307725">Comment by @blyxyas posted on 2025-06-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Final monthly update!**

- Even more optimizations have been achieved on the documentation lints front. https://github.com/rust-lang/rust-clippy/pull/15030. (-6.7% on `bumpalo`).

- The 3rd heaviest function was optimized away by 99.75%, along with the [`strlen_on_c_strings`](https://rust-lang.github.io/rust-clippy/master/index.html#strlen_on_c_strings) lint. This gives us about a 15% optimization on `tokio`. https://github.com/rust-lang/rust-clippy/pull/15043

- As a minor improvement, we now instantiate a lot less types on `unit_return_expecting_ord` (89% less calls in some benchmarks). This saves us a lot of locks on the type interner.

As a final update to the project goal, I'd like to say a little bit more:

I'm very happy with how this project goal has turned out. We've seen improvements in the 35-60% range for your real world projects and while I couldn't deliver the two objectives the project goal promised because of an excess in ambition, I still don't think that these are too far-fetched by any means.

As some specific examples, you can now witness a **38%** performance improvements in analyzing Cargo, and a **61%** in analyzing Tokio!

Much more to come, and thanks for sticking by while we make Clippy a better project, with better developer experience.
Have a great week, and I hope that you can enjoy all the performance improvements that we've delivered across this project goal.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/106'><strong>Prepare const traits for stabilization</strong></a></div>
    <div style="flex: initial;"><progress value="6" max="14"></progress>
</div>
</div>
<!-- markdown separator -->

@oli-obk:

The following contributors have made many libcore traits `const`:

* @Daniel-Aaron-Bloom
* @estebank
* @Randl
* @SciMind2460

@fee1-dead has also updated the syntax to allow for `const trait Trait {}` declarations instead of `#[const_trait] trait Trait {}`.

Thanks y'all for moving this feature along!

We have encountered few issues, but there is one major one:

without `dyn [const] Trait` support we cannot turn any of the `core::fmt` traits const in a usable way. This in turn makes things like `Result::unwrap` not usable in const contexts without using `const_eval_select` to not actually perform any formatting within const contexts.

It is my belief that now would be a good time to call for testing to get community input on the current syntax and behaviour.


<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/106#issuecomment-3057687748">Comment by @oli-obk posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The current proposal is `[const] Trait` syntax for bounds, `impl const Trait for Type` syntax for impls and `const Trait` for trait declarations. No annotations on methods in traits or impls required, but all implied from the trait or impl.

Re-constification of libstd has commenced

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/106#issuecomment-3126049238">Comment by @oli-obk posted on 2025-07-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The following contributors have made many libcore traits `const`:

* @Daniel-Aaron-Bloom
* @estebank
* @Randl
* @SciMind2460

@fee1-dead has also updated the syntax to allow for `const trait Trait {}` declarations instead of `#[const_trait] trait Trait {}`.

Thanks y'all for moving this feature along!

We have encountered few issues, but there is one major one:

without `dyn [const] Trait` support we cannot turn any of the `core::fmt` traits const in a usable way. This in turn makes things like `Result::unwrap` not usable in const contexts without using `const_eval_select` to not actually perform any formatting within const contexts.

It is my belief that now would be a good time to call for testing to get community input on the current syntax and behaviour.

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


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-3057671794">Comment by @epage posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

* Key developments:
  * GSoC work has started on https://github.com/crate-ci/cargo-plumbing
  * `cargo locate-manifest` is merged
  * `cargo read-manifest` is merged
  * Investigation is on-going for dependency resolution
* Blockers
* Help wanted

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-3129904179">Comment by @epage posted on 2025-07-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments:
* https://github.com/crate-ci/cargo-plumbing/pull/50 has been posted

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/265'><strong>Publish first rust-lang-owned release of &quot;FLS&quot;</strong></a></div>
    <div style="flex: initial;"><img src="https://img.shields.io/badge/Status-Completed%20%3D%29-green" alt="Status: Completed!"></img>
</div>
</div>
<!-- markdown separator --> 


Key Developments: **Goal Complete.** 

The FLS is now an independent repository within the Rust Project, not relying on imported Ferrocene packages for building (we have brought them in locally). A version of the FLS has been published at https://rust-lang.github.io/fls using the new build process. The content changes were mostly non-normative at this point, but we have officially published the first rust-lang owned release of the FLS.

Next steps: Continue adding/modifying appropriate content for the FLS moving forward. Determine any potential H2 2025 spec-related project goals.


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-3019529070">Comment by @JoelMarcey posted on 2025-06-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key Developments: **Goal Complete.**

The FLS is now an independent repository within the Rust Project, not relying on imported Ferrocene packages for building (we have brought them in locally). A version of the FLS has been published at https://rust-lang.github.io/fls using the new build process. The content changes were mostly non-normative at this point, but we have officially published the first rust-lang owned release of the FLS.

Next steps: Continue adding/modifying appropriate content for the FLS moving forward. Determine any potential H2 2025 spec-related project goals.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/266'><strong>Publish first version of StableMIR on crates.io</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="6"></progress>
</div>
</div>
<!-- markdown separator -->


We're almost done with the refactoring thanks again to @makai410 who is part of the GSoC. We are now considering renaming the crate before publishing, if you have any suggestion, please post it in https://rust-lang.zulipchat.com/#narrow/channel/320896-project-stable-mir/topic/Renaming.20StableMIR/with/520505712.

Finally, we're designing the test and release automation.


<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-3033560981">Comment by @celinval posted on 2025-07-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We're almost done with the refactoring thanks again to @makai410 who is part of the GSoC.

The `stable_mir` crate is now `rustc_public`. We are now finalizing the infrastructure and working on a compiler MCP. We should be ready to publish version 0.1 in the second half of the year. Thanks to everyone who helped, especially @makai410, who did most of the work.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-3120251130">Comment by @celinval posted on 2025-07-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The `stable_mir` crate is now `rustc_public`. We are now finalizing the infrastructure and working on a compiler MCP. We should be ready to publish version 0.1 in the second half of the year. Thanks to everyone who helped, especially @makai410, who did most of the work.

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/269'><strong>Rust Vision Document</strong></a></div>
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
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/275'><strong>rustc-perf improvements</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="7"></progress>
</div>
</div>
<!-- markdown separator -->

We made further progress on the new benchmarking scheme. The side of the website is nearing MVP status, currently we are switching focus on the side of the collector tha truns the benchmarks.

Some notable PRs:
- Benchmark request queue for try builds and release artifacts (https://github.com/rust-lang/rustc-perf/pull/2166, https://github.com/rust-lang/rustc-perf/pull/2192, https://github.com/rust-lang/rustc-perf/pull/2197, https://github.com/rust-lang/rustc-perf/pull/2201).
- Splitting of benchmark requests into benchmark jobs, including backfilling (https://github.com/rust-lang/rustc-perf/pull/2207).
- Benchmark sets (https://github.com/rust-lang/rustc-perf/pull/2206).


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-3132709062">Comment by @Kobzol posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We made further progress on the new benchmarking scheme. The side of the website is nearing MVP status, currently we are switching focus on the side of the collector tha truns the benchmarks.

Some notable PRs:
- Benchmark request queue for try builds and release artifacts (https://github.com/rust-lang/rustc-perf/pull/2166, https://github.com/rust-lang/rustc-perf/pull/2192, https://github.com/rust-lang/rustc-perf/pull/2197, https://github.com/rust-lang/rustc-perf/pull/2201).
- Splitting of benchmark requests into benchmark jobs, including backfilling (https://github.com/rust-lang/rustc-perf/pull/2207).
- Benchmark sets (https://github.com/rust-lang/rustc-perf/pull/2206).

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/118'><strong>Scalable Polonius support on nightly</strong></a></div>
    <div style="flex: initial;"><progress value="10" max="18"></progress>
</div>
</div>
<!-- markdown separator -->

@lqd:


Here are the key developments for the month of June, the last of this H1 project goal period.

Amanda has been preparing a **couple of papers** on polonius 🔥!

As for me, I've continued on the previous threads of work:
- the drop-liveness dataflow [optimization](https://github.com/rust-lang/rust/pull/141667) landed, and [I've also changed](https://github.com/rust-lang/rust/pull/142471) the bitset used in the loans-in-scope computation to better support the sparser cases with a lot of loans that we see in a handful of benchmarks (and we could tune that cutoff if we wanted to, it's currently around 2K by default in the `MixedBitSet` implementation IIRC).
- the rustc-perf benchmarks we have mostly exercise the move/init dataflow parts of borrow-checking, so I've created a stress test that puts emphasis on the loans-in-scope computation in particular, and have started gathering stats on crates.io code to have realistic examples. There are juicy functions in there, where one of the dataflow passes can take 40 seconds.
- I reworked the in-tree analysis to what should be close to a "polonius alpha" version of the analysis -- modulo a few loose ends that still need to be fixed -- and did some perf runs and a few crater runs with it enabled by default: nothing exploded. We know that this version based on reachability fixes fewer issues than a full version handling 100% of the flow-sensitivity problem -- like the datalog implementation did, albeit too slowly -- but is _actionable_ and meaningful progress: it fixes many cases of NLL problem 3. We're also reasonably confident that we can make a production-ready version of this alpha algorithm, and in this project goal period we have identified the areas where improvements can be made to gradually improve expressiveness, and that we wish to explore later.
- I also discovered a couple of failing examples with the new edition edition 2024 capture rules, and generally need to take care of member constraints, so it's not unexpected. Another small signal to improve test coverage, but not specific to borrowck: it's for all tests and editions in general, as seen in [MCP #861](https://github.com/rust-lang/compiler-team/issues/861).
- I've opened [PR #143093](https://github.com/rust-lang/rust/pull/143093) to land this polonius alpha analysis, and after looking into fixing member constraints, it should be the behavioral basis of what we hope to stabilize in the future, once it's more suited to production (e.g. better perf, better test coverage, more edge cases analyses, formalism) be it by incremental improvements, or via a different rewritten version of this algorithm -- with modifications to NLLs to make the interactions lazier/on-demand, so that we don't run a more expensive analysis if we don't need to.

In the future, hopefully for a h2 project goal, I plan to do that work towards stabilizing this alpha version of the analysis.


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-3019421051">Comment by @lqd posted on 2025-06-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Here are the key developments for the month of June, the last of this H1 project goal period.

Amanda has been preparing a **couple of papers** on polonius 🔥!

As for me, I've continued on the previous threads of work:
- the drop-liveness dataflow [optimization](https://github.com/rust-lang/rust/pull/141667) landed, and [I've also changed](https://github.com/rust-lang/rust/pull/142471) the bitset used in the loans-in-scope computation to better support the sparser cases with a lot of loans that we see in a handful of benchmarks (and we could tune that cutoff if we wanted to, it's currently around 2K by default in the `MixedBitSet` implementation IIRC).
- the rustc-perf benchmarks we have mostly exercise the move/init dataflow parts of borrow-checking, so I've created a stress test that puts emphasis on the loans-in-scope computation in particular, and have started gathering stats on crates.io code to have realistic examples. There are juicy functions in there, where one of the dataflow passes can take 40 seconds.
- I reworked the in-tree analysis to what should be close to a "polonius alpha" version of the analysis -- modulo a few loose ends that still need to be fixed -- and did some perf runs and a few crater runs with it enabled by default: nothing exploded. We know that this version based on reachability fixes fewer issues than a full version handling 100% of the flow-sensitivity problem -- like the datalog implementation did, albeit too slowly -- but is _actionable_ and meaningful progress: it fixes many cases of NLL problem 3. We're also reasonably confident that we can make a production-ready version of this alpha algorithm, and in this project goal period we have identified the areas where improvements can be made to gradually improve expressiveness, and that we wish to explore later.
- I also discovered a couple of failing examples with the new edition edition 2024 capture rules, and generally need to take care of member constraints, so it's not unexpected. Another small signal to improve test coverage, but not specific to borrowck: it's for all tests and editions in general, as seen in [MCP #861](https://github.com/rust-lang/compiler-team/issues/861).
- I've opened [PR #143093](https://github.com/rust-lang/rust/pull/143093) to land this polonius alpha analysis, and after looking into fixing member constraints, it should be the behavioral basis of what we hope to stabilize in the future, once it's more suited to production (e.g. better perf, better test coverage, more edge cases analyses, formalism) be it by incremental improvements, or via a different rewritten version of this algorithm -- with modifications to NLLs to make the interactions lazier/on-demand, so that we don't run a more expensive analysis if we don't need to.

In the future, hopefully for a h2 project goal, I plan to do that work towards stabilizing this alpha version of the analysis.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/271'><strong>Secure quorum-based cryptographic verification and mirroring for crates.io</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="12"></progress>
</div>
</div>
<!-- markdown separator -->

@walterhpearce:

Hello All -

Following is a status update and breakdown on where things currently stand for the MVP implementation of TUF and the choices we’ve landed at so far with the discussion via this goal. At the end of this update is a briefer list-form of this update.

In summary, we have landed at moving forward with a TAP-16 Merkle Tree implementation of TUF for crates.io, with technical choices pending on the best balance and optimization for our specific performance needs. We are still currently on track to have a MVP public implementation by the end of July of this implementation, which optimizations will be tested against. This includes:

- Test repositories and tooling for rustup, releases and crates.io
- Temporary repository tooling for updates (We are currently outside these services, and so updates occur via periodic checks)
- An out-of-band index copy for crates.io for in-line signing testing
- cargo-signing subcommand tooling for end-user functionality (TUF updates, validation and downloading)

We still have open questions for the specific approach of the Merkle tree, which is continuing into H2. We have also reached an acceptable consensus with the infrastructure team for deployment planning.

TUF Implementation

During H1, we experimented with 4 implementations of TUF: To-spec, Hashed Bins, Succinct Hashed Bins, and TUF TAP-16 Merkle Trees. Hashed Bins & Succinct Hashed Bins are the current approaches being experimented with in the Python community, and we wanted to see how that would align with our growth and bandwidth requirements. After experimenting, we found the linear growth models to still be unacceptable, thus landing at the Merkle Tree implementation. This still comes at a round-trip increase cost, however, and for H2 we are now experimenting with how to implement the Merkle tree to reduce round-trips - via balancing, implementation details and tree slicing - or a combination of the three..

Quorum & Roles

On the higher level grounds of quorums and infrastructure, through discussions, we have come to a consensus on maintaining a top-level quorum, but removing intermediate levels for simplicity. The root quorum shall be the Infrastructure team for initial deployment; roles under this quorum will be nightly, releases, rustup and crates.io; each one of these keys will be a single live key which resides in KMS. We will leverage KMS API’s to perform live signing for all actions of those roles (new releases and crates). The hierarchy initially proposed in the RFC will be removed in favor of this approach.

The root quorum will manage the roles via tuf-on-ci on a github repository, while actual signing actions using the live keys will all occur via local tooling in their CI.

Choices Made

Listed here the choices made as a part of this goal:
- Initial root quorum will be the infrastructure team with a 3-member threshold. This can be rotated or grown at any time by that team in the future.
- Role keys will live in KMS and be used in the appropriate CI/infrastructure of those teams (Infra for nightly, releases and rustup; the crates.io team for crates). This will be managed via IAM access to the KMS.
- TAP-16 Merkle Tree implementation of TUF was chosen. Other methods linear+ growth models were unacceptable. We still have open questions to resolve around bandwidth vs. round-trips
- tuf-on-ci will only be used for the root quorum and role changes, to leverage PR-workflows for easy management.
- The source-of-truth TUF repository will live in an S3 bucket.
- We will rely on cloudtrail for audit logging of KMS and work to make those logs available for transparency

Next Steps

- A public MVP will go live at the end of July / August, and live changes/tests will be made of the Merkle tree implementation there.
- We still need to determine the appropriate trade off for round trips vs. bandwidth for the Merkle Tree. We are collecting more granular logs from the sparse index and crates.io index as a whole to accomplish this. Crate downloads vs. updates are very unbalanced, and we expect to get significant reductions of both by appropriately balancing the tree.
- Work needs to start on beginning to stand up infrastructure in the project to house this in the simpleinfra repository. Besides the raw infrastructure, this needs to be tooling for the initial creation ceremony.
- We’ve begun thinking about what different mirroring strategies look like when utilizing TUF, to make sure we consider those when deploying this. The MVP provides basic validation of any mirror, but how can mirroring and fallbacks possibly be integrated?


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/271#issuecomment-3133590786">Comment by @walterhpearce posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Hello All -

Following is a status update and breakdown on where things currently stand for the MVP implementation of TUF and the choices we’ve landed at so far with the discussion via this goal. At the end of this update is a briefer list-form of this update.

In summary, we have landed at moving forward with a TAP-16 Merkle Tree implementation of TUF for crates.io, with technical choices pending on the best balance and optimization for our specific performance needs. We are still currently on track to have a MVP public implementation by the end of July of this implementation, which optimizations will be tested against. This includes:

- Test repositories and tooling for rustup, releases and crates.io
- Temporary repository tooling for updates (We are currently outside these services, and so updates occur via periodic checks)
- An out-of-band index copy for crates.io for in-line signing testing
- cargo-signing subcommand tooling for end-user functionality (TUF updates, validation and downloading)

We still have open questions for the specific approach of the Merkle tree, which is continuing into H2. We have also reached an acceptable consensus with the infrastructure team for deployment planning.

TUF Implementation

During H1, we experimented with 4 implementations of TUF: To-spec, Hashed Bins, Succinct Hashed Bins, and TUF TAP-16 Merkle Trees. Hashed Bins & Succinct Hashed Bins are the current approaches being experimented with in the Python community, and we wanted to see how that would align with our growth and bandwidth requirements. After experimenting, we found the linear growth models to still be unacceptable, thus landing at the Merkle Tree implementation. This still comes at a round-trip increase cost, however, and for H2 we are now experimenting with how to implement the Merkle tree to reduce round-trips - via balancing, implementation details and tree slicing - or a combination of the three..

Quorum & Roles

On the higher level grounds of quorums and infrastructure, through discussions, we have come to a consensus on maintaining a top-level quorum, but removing intermediate levels for simplicity. The root quorum shall be the Infrastructure team for initial deployment; roles under this quorum will be nightly, releases, rustup and crates.io; each one of these keys will be a single live key which resides in KMS. We will leverage KMS API’s to perform live signing for all actions of those roles (new releases and crates). The hierarchy initially proposed in the RFC will be removed in favor of this approach.

The root quorum will manage the roles via tuf-on-ci on a github repository, while actual signing actions using the live keys will all occur via local tooling in their CI.

Choices Made

Listed here the choices made as a part of this goal:
- Initial root quorum will be the infrastructure team with a 3-member threshold. This can be rotated or grown at any time by that team in the future.
- Role keys will live in KMS and be used in the appropriate CI/infrastructure of those teams (Infra for nightly, releases and rustup; the crates.io team for crates). This will be managed via IAM access to the KMS.
- TAP-16 Merkle Tree implementation of TUF was chosen. Other methods linear+ growth models were unacceptable. We still have open questions to resolve around bandwidth vs. round-trips
- tuf-on-ci will only be used for the root quorum and role changes, to leverage PR-workflows for easy management.
- The source-of-truth TUF repository will live in an S3 bucket.
- We will rely on cloudtrail for audit logging of KMS and work to make those logs available for transparency

Next Steps

- A public MVP will go live at the end of July / August, and live changes/tests will be made of the Merkle tree implementation there.
- We still need to determine the appropriate trade off for round trips vs. bandwidth for the Merkle Tree. We are collecting more granular logs from the sparse index and crates.io index as a whole to accomplish this. Crate downloads vs. updates are very unbalanced, and we expect to get significant reductions of both by appropriately balancing the tree.
- Work needs to start on beginning to stand up infrastructure in the project to house this in the simpleinfra repository. Besides the raw infrastructure, this needs to be tooling for the initial creation ceremony.
- We’ve begun thinking about what different mirroring strategies look like when utilizing TUF, to make sure we consider those when deploying this. The MVP provides basic validation of any mirror, but how can mirroring and fallbacks possibly be integrated?



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/270'><strong>SVE and SME on AArch64</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="16"></progress>
</div>
</div>
<!-- markdown separator -->

@davidtwco:

- rust-lang/rust#137944 got merged with Part I of the Sized Hierarchy work
  - A bug was discovered through fuzzing when the feature was enabled, users could write `dyn PointeeSized` which would trigger the builtin impl for `PointeeSized`, which doesn't exist. rust-lang/rust#143104 was merged to fix that.
  - In attempt to experiment with relaxing `Deref::Target`, we discovered that sizedness supertraits weren't being elaborated from where bounds on projections.
    - Adding those bounds meant that there could be two candidates for some obligations - from a where bound and from an item bound - where previously there would only be the item bound. Where bounds take priority and this could result in regions being equated that did not previously.
    - By fixing that, we ran into issues with normalisation that was happening which restricted what code using GATs was accepted. Fixing this got everything passing but more code is accepted.
    - rust-lang/rust#142712 has this fixed, but isn't yet merged as it's quite involved.
- I've still not made any changes to the Sized Hierarchy RFC, there's a small amount of discussion which will be responded to once the implementation has landed.
- While implementing Part II of the Sized Hierarchy work, we ran into limitations of the old solver w/r/t host effect predicates around coinductive cycles. We've put that aside until there's nothing else to do or the new solver is ready.
- We've been reviving the RFC and implementation of the SVE infrastructure, relying on some exceptions because of not having const sizedness yet, but knowing that we've got a solution for that coming, we're hoping to see this merged as an experiment once it is ready.
- We've opened rust-lang/rust#144404 that documents the current status of the Sized Hierarchy feature and our plans for it.
    - As before, implementing const sizedness is on hold until the next solver is ready or there's nothing else to do.
    - We've opened rust-lang/rust#144064 with the interesting parts of rust-lang/rust#142712 from a t-types perspective, that's currently waiting on FCP checkboxes.
        - This will enable experimentation with relaxing `Deref::Target` to `PointeeSized`.
- We've opened rust-lang/rfcs#3838 and rust-lang/rust#143924 updating rust-lang/rfcs#3268 and rust-lang/rust#118917 respectively.
    - There's been lots of useful feedback on this that we're working on addressing and will have an update soon


<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3062329839">Comment by @davidtwco posted on 2025-07-11:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- rust-lang/rust#137944 got merged with Part I of the Sized Hierarchy work
  - A bug was discovered through fuzzing when the feature was enabled, users could write `dyn PointeeSized` which would trigger the builtin impl for `PointeeSized`, which doesn't exist. rust-lang/rust#143104 was merged to fix that.
  - In attempt to experiment with relaxing `Deref::Target`, we discovered that sizedness supertraits weren't being elaborated from where bounds on projections.
    - Adding those bounds meant that there could be two candidates for some obligations - from a where bound and from an item bound - where previously there would only be the item bound. Where bounds take priority and this could result in regions being equated that did not previously.
    - By fixing that, we ran into issues with normalisation that was happening which restricted what code using GATs was accepted. Fixing this got everything passing but more code is accepted.
    - rust-lang/rust#142712 has this fixed, but isn't yet merged as it's quite involved.
- I've still not made any changes to the Sized Hierarchy RFC, there's a small amount of discussion which will be responded to once the implementation has landed.
- While implementing Part II of the Sized Hierarchy work, we ran into limitations of the old solver w/r/t host effect predicates around coinductive cycles. We've put that aside until there's nothing else to do or the new solver is ready.
- We've been reviving the RFC and implementation of the SVE infrastructure, relying on some exceptions because of not having const sizedness yet, but knowing that we've got a solution for that coming, we're hoping to see this merged as an experiment once it is ready.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-3132566564">Comment by @davidtwco posted on 2025-07-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- We've opened rust-lang/rust#144404 that documents the current status of the Sized Hierarchy feature and our plans for it.
    - As before, implementing const sizedness is on hold until the next solver is ready or there's nothing else to do.
    - We've opened rust-lang/rust#144064 with the interesting parts of rust-lang/rust#142712 from a t-types perspective, that's currently waiting on FCP checkboxes.
        - This will enable experimentation with relaxing `Deref::Target` to `PointeeSized`.
- We've opened rust-lang/rfcs#3838 and rust-lang/rust#143924 updating rust-lang/rfcs#3268 and rust-lang/rust#118917 respectively.
    - There's been lots of useful feedback on this that we're working on addressing and will have an update soon

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
    <div style="flex: initial;"><progress value="2" max="13"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/123#issuecomment-3058802310">Comment by @Muscraft posted on 2025-07-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments
- The new API for `annotate-snippets` got merged (and tweaked)
- [`annotate-snippets` passed all of `rustc`'s UI tests for the first time](https://asciinema.org/a/MlUN66AxlyLbaJ9VP8zDWlutt)
- I started getting `annotate-snippets` ready for release
- [I started opening PRs](https://github.com/rust-lang/rust/pulls?q=is%3Apr+author%3AMuscraft+created%3A%3E2025-03-31+) to get `rustc` to match `annotate-snippets` planned output changes

Blockers

Help wanted


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>
