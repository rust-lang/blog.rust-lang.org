+++
path = "9999/12/31/april-project-goals-update"
title = "April Project Goals Update"
authors = ["Rémy Rakic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

The Rust project is currently working towards a [slate of 40 project goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html), with 3 of them designated as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="34"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** This work continues our drive to improve support for async programming in Rust. In 2024H2 we stabilized async closures; explored the generator design space; and began work on the `dynosaur` crate, an experimental proc-macro to provide dynamic dispatch for async functions in traits. In 2025H1 [our plan](https://rust-lang.github.io/rust-project-goals/2025h1/async.html) is to deliver (1) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (2) progress towards sync and async generators, simplifying the creation of iterators and async data streams; (3) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.

**What has happened?** 

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-2896086972">Comment by @tmandry posted on 2025-05-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Async fn in traits.** An FCP proposal to stabilize return type notation was started in https://github.com/rust-lang/rust/pull/138424. However, it is currently blocked on concerns that stabilizing it now will make it more difficult to ship Rust's next-generation trait solver.

**Async fn in dyn trait.** There have been discussions around next steps to support this in the language. More experimentation is needed, along with an initial RFC.

**dynosaur.** More breaking changes have landed and we expect to release v0.3 soon.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/263'><strong>Organize Rust All-Hands 2025</strong></a></div>
    <div style="flex: initial;"><progress value="18" max="28"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** May 15, 2025 marks the 10-year anniversary of Rust's 1.0 release; it also marks 10 years since the [creation of the Rust subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). At the time [there were 6 Rust teams with 24 people in total](http://web.archive.org/web/20150517235608/http://www.rust-lang.org/team.html). There are now 57 teams with 166 people. In-person All Hands meetings are an effective way to help these maintainers get to know one another with high-bandwidth discussions. This year, the Rust project will be coming together for [RustWeek 2025](https://2025.rustweek.org), a joint event organized with [RustNL](https://2025.rustweek.org/about/). Participating project teams will use the time to share knowledge, make plans, or just get to know one another better. One particular goal for the All Hands is reviewing a draft of the [Rust Vision Doc](./rust-vision-doc.md), a document that aims to take stock of where Rust is and lay out high-level goals for the next few years.

**What has happened?** 

<!-- markdown separator --> 


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2824692607">Comment by @m-ou-se posted on 2025-04-23:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update:

Below is the preliminary schedule for the "pre all hands" and all hands:

![Image](https://github.com/user-attachments/assets/9dd9aa14-1b37-478b-9071-3865ebac3994)

![Image](https://github.com/user-attachments/assets/4849caa8-c7af-4a9e-b1ee-75c1fafc7a6c)

![Image](https://github.com/user-attachments/assets/5888677e-3319-453c-9758-f8014954ebe6)

The last day has a lot of empty slots for now. I'm still working on filling those, but I'll leave a few empty slots to allow for flexibility during the event itself.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2824770439">Comment by @m-ou-se posted on 2025-04-23:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I published our covid policy: https://rustweek.org/covid-policy-all-hands-and-unconf/

And got us a ton of covid self-tests and Aranet4 CO₂ sensors:

![Image](https://github.com/user-attachments/assets/09f53af5-785b-4871-b7be-06f58b1b786d)
![Image](https://github.com/user-attachments/assets/fdd79c8f-94bb-40b2-9cf9-ab949ebeb4e8)

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2824783540">Comment by @m-ou-se posted on 2025-04-23:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

For remote attendance, I got us a bunch of Jabra Speak2 75 conferencing speaker-microphones. They are battery powered and work out-of-the-box both over USB and Bluetooth on any platform.

I'll put them near the entrance for anyone to borrow for any of the meeting rooms.

![Image](https://github.com/user-attachments/assets/1669b50d-a60c-4ba0-8e82-9f403820d0a7)

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

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

**What has happened?** The primary focus for this year is compiled flags, and we are continuing to push on the various compiler flags and things that are needed to support building RFL on stable (e.g., RFC #3791 proposed adding `--crate-attr`, which permits injecting attributes into crates externally to allow the Kernel's build process to add things like `#![no_std]` so they don't have to be inserted manually into every file; MCPs for ABI flags like [`retpoline`](https://github.com/rust-lang/compiler-team/issues/868) and [`harden-sls`](https://github.com/rust-lang/compiler-team/issues/869) and [implementation of `-Zindirect-branch-cs-prefix`](https://github.com/rust-lang/rust/pull/140740)). A number of issues had minor design questions (how to manage clippy configuration; best approach for rustdoc tests) and we plan to use the RustWeek time to hash those out.

We are also finishing up some of the work on language items. We have had two stabilizations of lang features needed by Rust for Linux ([naked functions](https://github.com/rust-lang/rust/pull/134213), [`asm_goto` syntax](https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/#asm-jumps-to-rust-code)). The trickiest bit here is arbitrary self types, where we encountered a concern relating to pin and are still [discussing the best resolution](https://rust-lang.zulipchat.com/#narrow/channel/425075-rust-for-linux/topic/2025-05-07.20meeting/near/516734641).

<!-- markdown separator --> 


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2894303032">Comment by @ojeda posted on 2025-05-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update from our 2025-04-09 meeting ([full minutes](https://hackmd.io/@rust-lang-team/rkqdLEER1l)):

  - Some progress on `arbitrary_self_types`. In particular, decided to do with respect to pin and other related cases.
  
  - `asm_goto` is solved, apart from output operands. For `asm_const`, https://github.com/rust-lang/rust/pull/138618 is nominated.

  - ABI-modifying compiler flags: some PRs waiting review, e.g. https://github.com/rust-lang/rust/pull/138736.

  - `--crate-attr` RFC is up: https://github.com/rust-lang/rfcs/pull/3791.

  - `-Zsanitize-kcfi-arity`'s implementation PR got merged: https://github.com/rust-lang/rust/pull/138368. If all is good from the Linux side, a stabilization PR will be sent.

  - CFI `core::fmt` issue: https://github.com/rust-lang/rust/issues/115199.

  - Discussion around `bindgen`, `repr(align)` and packed types. RFC nominated for lang discussion: https://github.com/rust-lang/rfcs/pull/3718#issuecomment-2790654254.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2894308715">Comment by @ojeda posted on 2025-05-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update from our 2025-04-23 meeting ([full minutes](https://hackmd.io/@rust-lang-team/BJZ69jLkgx)):

  - Naked functions were stabilized, which could see some use in the kernel.

  - Lang discussed `#[repr(align)]` (the kernel is interested in, at least, the global one, i.e. `-Zmin-function-alignment=N`).

  - `asm_const`: @nbdd0121 will reply on the latest review comments in the implementation PR: https://github.com/rust-lang/rust/issues/128464.

  - `--crate-attr`: the author of the RFC (https://github.com/rust-lang/rfcs/pull/3791) is looking for a new owner. The RFC is in proposed FCP. Small updates to the text may be needed. Otherwise compiler probably wants to merge it. @Mark-Simulacrum to be pinged.

  - Clippy configuration etc.: @flip1995 will be at RustWeek, the plan is to discuss it there.

  - `rustdoc` extract doctests: @GuillaumeGomez and @ojeda plan to discuss it at RustWeek.

  - `-Zsanitize-kcfi-arity`: waiting on the kernel side (`tc-build` support sent).

  - CFI `core::fmt` issue: PR submitted: https://github.com/rust-lang/rust/pull/139632.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2894319024">Comment by @ojeda posted on 2025-05-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update from our 2025-05-07 meeting ([full minutes](https://hackmd.io/@rust-lang-team/S1Y3l7Kxel)):

  - Enthusiasm and plans for RustWeek.

  - `arbitrary_self_types`: update from @dingxiangfei2009 at https://rust-lang.zulipchat.com/#narrow/channel/425075-rust-for-linux/topic/2025-05-07.20meeting/near/516734641 -- he plans to talk to types in order to find a solution. @davidtwco will ping @petrochenkov about `rustc_resolve`.

  - Sanitizer support and `#[sanitize(off)]`: discussed by lang at https://github.com/rust-lang/rust/pull/123617#issuecomment-2859621119. Discussion about allowing to disable particular sanitizers. Older concern from compiler at https://github.com/rust-lang/rust/pull/123617#issuecomment-2192330122.

  - `asm_const` with pointers support: lang talked about it -- lang will want an RFC: https://github.com/rust-lang/rust/issues/128464#issuecomment-2861515372.

  - ABI-modifying compiler flags: two MCPs filled: https://github.com/rust-lang/compiler-team/issues/868 (`-Zretpoline` and `-Zretpoline-external-thunk`) and https://github.com/rust-lang/compiler-team/issues/869 (`-Zharden-sls`).

    Implementation PR for `-Zindirect-branch-cs-prefix` at https://github.com/rust-lang/rust/pull/140740 that goes on top of https://github.com/rust-lang/rust/pull/135927.

    @davidtwco agreed there is likely no need for a separate MCP for this last one, i.e. it could go into the `-Zretpoline*` one. @azhogin pinged about this at https://github.com/rust-lang/rust/pull/135927#issuecomment-2859906060.
  
  - `--crate-attr`: @Mark-Simulacrum was pinged and he is OK to adopt the RFC (https://github.com/rust-lang/rfcs/pull/3791).

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>




## Goals looking for help


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

<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-2842025884">Comment by @BoxyUwU posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I've a PR open to make resolving inherent assoc terms in item signatures not result in query cycles, this will be necessary for uses of inherent assoc consts in the type system under mgca. camelid is currently working on representing const items as aliases to type system consts rather than bodies like they currently are, this is necessary to implement normalization of const const aliases under mgca, it will also allow us to implement the core mgca check of const aliases in the type system being equal to valid const arguments, and also we'll be able to split out a full gca feature gate *without* that restriction.

The PR mentioned in the previous update to handle const aliases with inference variables correctly has turned into a bit of a rabbit hole. It turned out that there were *stable* issues around const evaluation and illformed constants resulting in ICEs so I've wound up trying to get those fixed and have been writing up a document explaining justification for a breaking change there. 

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-2857981933">Comment by @davidtwco posted on 2025-05-07:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- We've started a regular biweekly sync call with upstream stakeholders in build-std from the lang, compiler and cargo teams where we discuss aspects of our tentative design or clarify constraints.
- @adamgemmell has continued to draft our proposal for build-std, which we're discussing in our regular sync calls.
- We're hosting a session at the All Hands next week to discuss build-std.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-2848816055">Comment by @obi1kenobi posted on 2025-05-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We encountered some speedbumps this month.

TL;DR:
- While working on `'static` and "outlives" bounds, we discovered Rust's ability to _imply_ bounds that are not stated explicitly at the definition site.
- Implied bounds are load-bearing for SemVer; failure to take them into account will produce _both_ false-positives _and_ false-negatives.
- While technical limitations make it infeasible for `cargo-semver-checks` to correctly deduce implied bounds, `rustc` has this capability internally.
- We have [asked the rustdoc team](https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/Show.20implied.20bounds.20in.20rustdoc.20JSON/near/515146429) to expose implied bounds in rustdoc JSON by using those `rustc` internal APIs.

There are some good news as well, though! While looking at the `#[target_feature]` attribute:
- We discovered previously-undocumented SemVer hazards.
- We [discovered a case of unsoundness](https://github.com/rust-lang/rust/issues/139368) when that attribute is used on trait methods.
- With the help of contributors and the rustdoc team, rustdoc JSON began including additional information that will help future `cargo-semver-checks` versions catch those SemVer hazards.

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Experiment with ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="8"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-2786926399">Comment by @nikomatsakis posted on 2025-04-08:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

In reviewing https://github.com/rust-lang/rust/pull/138628, we realized that the tests were not behaving as expected because they were running in Rust 2015 which had distinct capture rules. My [suggestion](https://github.com/rust-lang/rust/pull/138628#pullrequestreview-2750344682) was to limit the `use` keyword (or at least use closures...) to Rust 2021 so as to avoid having to think about how it interacts with earlier capture rules (as well as potential migrations). I believe this follows from the Edition axiom that [Editions are meant to be adopted](https://rust-lang.github.io/rfcs/3085-edition-2021.html#editions-are-meant-to-be-adopted).

There is an interesting tension with [Rust should feel like one language](https://rust-lang.github.io/rfcs/3085-edition-2021.html#rust-should-feel-like-one-language). My feeling is that there is a missing tenet: the reason we do editions and not fine-grained features is because we wish to avoid combianotoric explosion, where odd combinations of features can lead to untested scenarios. But that is exactly what would be happening here if we allow `use` on older editions. So I think the rule should be that you make new features available on older editions *up until the point where they interact with something that changed* -- in this case, `use` closures interact with the closure capture rules which changed in Rust 2021, so we should limit this feature to Rust 2021 and newer.

Put another way, you should never have to go back and modify an edition migration to work differently. That suggestions you are attempting to push the feature too far back.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-2843027977">Comment by @spastorino posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We've modified codegen so that we guarantee that `x.use` will do a copy if `X: Copy` is true after monomorphization. Before this change the desugaring to `clone` occured only before monomorphization and hence it would call the `clone` method even for those instances where `X` is a `Copy` type. So with this modification we avoid such situation.

We are not working on convert `x.use` to a move rather than a clone if this is a last-use.

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
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

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-2848617679">Comment by @Eh2406 posted on 2025-05-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I will be giving [a talk at Rust-Week](https://rustweek.org/talks/jacob/) about the history that brought us to this project/goal. Aside from preparing for that talk I have not had time for this effort.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-2841981003">Comment by @epage posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Key developments:
  - Continued efforts to clean up the existing code
  - t-testing-devex has been working out where the crates should live in preparation for publishing them ([#t-testing-devex > crate ownership @ 💬](https://rust-lang.zulipchat.com/#narrow/channel/404371-t-testing-devex/topic/crate.20ownership/near/513724212))
  - Looking to build on the work done for `test-r` and `rustest` crates as they align with the long term vision
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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-2843397925">Comment by @eholk posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@b-naber and I have been working on the rustc side of the implementation for this feature.

I merged https://github.com/rust-lang/rust/pull/139647, which adds the unstables `-Z namespaced-crates` option to the compiler and enables parsing of externs like `--extern foo::bar=libbar.rlib`. @b-naber has led the resolver changes and has a draft PR up at https://github.com/rust-lang/rust/pull/140271.

The implementation work has raised some [new concerns](https://github.com/rust-lang/rust/issues/122349#issuecomment-2832241624) about the overall direction, so work is ongoing to resolve those while continuing to make progress in the meantime.

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
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/126#issuecomment-2843622028">Comment by @celinval posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We fixed issue https://github.com/rust-lang/rust/issues/136925 that was blocking contract annotations on constant functions, which unblocks the initial PR to add some contract annotations in the standard library (https://github.com/rust-lang/rust/pull/136578). The PR currently triggers a CI failure which we are investigating.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/259#issuecomment-2843934346">Comment by @jieyouxu posted on 2025-05-01:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update (2025-05-01):

- Not much updates, recent compiletest changes were surrounding error annotation strictness/canonicalization and landing a new executor that doesn't depend on libtest, and I've mostly been involved in reviewing those.
- Next planned changes are first to introduce some discipline into compiletest's error handling and contributor-facing diagnostics, because configuration and directive handling currently still has a ton of random panics all over the place.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/260'><strong>Metrics Initiative</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="6"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2810680086">Comment by @yaahc posted on 2025-04-16:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Small progress update:

following the plan mentioned above plus some extra bits, I've implemented the following changes

* changed the json output to include the timestamp
* changed the file naming to ensure uniqueness and not overwrite metrics for the same crate when built with different configurations
  * previously I was piggybacking on the hash used to name artifacts in the `.cargo` or `build` directories, which in the compiler is known as `extra_filename` and is configured by cargo, but it turns out this doesn't guarantee uniqueness
  * I've replaced this with the ["Strict Version Hash"](https://rustc-dev-guide.rust-lang.org/backend/libs-and-metadata.html?highlight=stable%20version%20hash#strict-version-hash) (SVH)
    * Doing so introduced an ICE when compiling some crates with incremental compilation enabled. I've since resolved this in https://github.com/rust-lang/rust/pull/139502 and tested this version against the top 100 crates in the ecosystem and their dependencies to verify its working
* I've been working with the infra team and they've setup a cloud instance of influxdb 3.0 and grafana, influxdb is setup, grafana in progress
* I met with both libs and lang to discuss their needs related to the unstable feature usage metrics and metrics in general

Next Steps:
* I've got everything setup for the docs.rs team to start gathering a sample dataset for which I will then upload to the server the infra team setup
* update locally hosted PoC impl to work with recent changes to metrics files and naming and validate that it's working as expected
* work on the queries for the grafana instance to setup a graph per feature showing usage over time
  * probably going to create fake usage data to with for this
* on the side I'm also looking into how much work it would be to track relative usage of various library APIs under a single feature flag (e.g. https://github.com/rust-lang/rust/issues/139911 tracking the specific functions used)
* develop a better understanding of the expected cost of running an influxdb server

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2822723527">Comment by @yaahc posted on 2025-04-22:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

posting this here so I can link to it in other places, I've setup the basic usage over time chart using some synthesized data that just emulates quadraticly (is that a word?) increasing feature usage for my given feature over the course of a week (the generated data starts at 0 usages per day and ends at 1000 usages per day). This  chart counts the usage over each day long period and charts those counts over a week. The dip at the end is the difference between when I generated the data, after which there is zero usage data, and when I queried it.

With this I should be ready to just upload the data once we've gathered it from docs.rs, all I need to do is polish and export the dashboards I've made from grafana to the rust-lang grafana instance, connect that instance to the rust-lang influxdb instance, and upload the data to influxdb once we've gathered it.

![Image](https://github.com/user-attachments/assets/4d3db7cf-04bc-400c-8791-ac10c402ccdf)

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
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-2846022991">Comment by @lcnr posted on 2025-05-01:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We've made a lot of progress over the last 1.5 months. My change to opaque types in borrowck is pretty much done now: https://github.com/rust-lang/rust/pull/139587. It still needs some cleanup and an FCP to actually merge. We've already merged multiple cleanups on the way here.

We then started to test crater with the `-Znext-solver=globally`. @compiler-errors and me encountered and merged the fixes for 13 issues since then: https://github.com/rust-lang/rust/pull/139791 https://github.com/rust-lang/rust/pull/139798 https://github.com/rust-lang/rust/pull/140236 https://github.com/rust-lang/rust/pull/139900 https://github.com/rust-lang/rust/pull/139828 https://github.com/rust-lang/rust/pull/139774 https://github.com/rust-lang/rust/pull/139762 https://github.com/rust-lang/rust/pull/139789 https://github.com/rust-lang/rust/pull/138845 https://github.com/rust-lang/rust/pull/140306 https://github.com/rust-lang/rust/pull/140305 https://github.com/rust-lang/rust/pull/140276 https://github.com/rust-lang/rust/pull/140302. @Nadrieril was also helpful by minimizing an encountered issue.

With these improvements and multiple in-flight changes we're now at significantly less than 100 remaining regressions in the top 10000 crates and have started the first complete crater run today. We are using a single PR for all crater runs. Check out https://github.com/rust-lang/rust/pull/133502 for the current status and the stack of in-flight changes. 

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/261#issuecomment-2841895996">Comment by @veluca93 posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: nothing substantial.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-2837910940">Comment by @blyxyas posted on 2025-04-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Monthly update!

- In [the last monthly update](https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-2730435572) we saw the impact that [interning symbols](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_span/symbol/struct.Symbol.html#method.intern) had on the program's performance. An effort to minimize this via a pre-interning symbol mechanism has been implemented in https://github.com/rust-lang/rust-clippy/pull/14650 and https://github.com/rust-lang/rust/pull/138682

- We're phasing out the old ["str path"](https://doc.rust-lang.org/nightly/nightly-rustc/clippy_utils/fn.match_def_path.html) infrastructure into a new lazy alternative. https://github.com/rust-lang/rust-clippy/pull/14705

- We're currently in the effort to optimize some documentation lints that took up to 15% of the Clippy runtime (depending on how much documentation for each line of code you had.) See https://github.com/rust-lang/rust-clippy/pull/14693

- We've also been experimenting with lots of new possibilities, mainly on parallel lints. Althought they currently are not performance improvements, there are some great hope put into them.

- Memory consumption and branch mispredictions are being monitored, they do not seem out of the ordinary.

- Monitoring cache misses and references, turns out that about 31% of cache references (792m found) are cache misses (253m found) in some benchmarks. We will check what's behind those numbers and if they can be improved.

This has been a great month for performance!

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/106#issuecomment-2841950116">Comment by @oli-obk posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

No updates

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/121'><strong>Promoting Parallel Front End</strong></a></div>
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
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/264'><strong>Prototype a new set of Cargo &quot;plumbing&quot; commands</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2841965738">Comment by @epage posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Key developments:
  - The [cargo-plumbing repo](https://github.com/crate-ci/cargo-plumbing)  was created to serve as the central place for collaboration on this effort
  - @ashiskumarnaik posted crate-ci/cargo-plumbing#5 for the first plumbing command
- Blockers: 
- Help wanted: 



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2851032075">Comment by @ojuschugh1 posted on 2025-05-05:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Hi @epage  , I am interested in working on this project. If you are still looking for someone.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2852224902">Comment by @epage posted on 2025-05-05:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

@ojuschugh1 iirc there is a GSoC proposal for this and we are waiting to hear whether it was accepted.  If it was, it would likely involve coordinating with them on tasks.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/265'><strong>Publish first rust-lang-owned release of &quot;FLS&quot;</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-2847874500">Comment by @JoelMarcey posted on 2025-05-02:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key Developments: The FLS repo has officially been transferred from Ferrous to the Rust Project. https://github.com/rust-lang/fls is now live. 

Next step: Integrate the FLS with the Rust build system in order to support publishing within project processes.

Blockers: None yet. The build system integration could create some support requests, however.

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
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-2842836236">Comment by @celinval posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We have started the refactoring. @makai410 has moved all the existing code into a single crate and they have started moving things around.

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



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-2857992478">Comment by @davidtwco posted on 2025-05-07:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- @Jamesbarford has been working with @Kobzol to implement a database-backed job queueing mechanism, which will better scale to support multiple collectors and ends up being the key part of rustc-perf needing adapted to support multiple collectors.
- @Jamesbarford has also upstreamed tests for the existing queue ordering (rust-lang/rustc-perf#2072).

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-2850719303">Comment by @lqd posted on 2025-05-05:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Here are the key developments for the month of April

- @amandasystems
   - extracted a handful of PRs out of the gigantic placeholder rewrite PR, to make it easier to review and land
   - https://github.com/rust-lang/rust/pull/139960, https://github.com/rust-lang/rust/pull/139965, and https://github.com/rust-lang/rust/pull/140466
- Tage
   - continued experimenting and making progress with the early phase of the process, and making building constraints, and traversing them per loan, lazy
   - started extracting some of that work for discussion, review, PRs, as well as writing reports for his masters thesis
- @lqd
   - continued on improving the algorithm. We're now at a point where we have an approximation of the datalog algorithm, which handles our UI tests -- except one where control flow in a loop connects to regions that are live before and after the loop: this causes a false positive that our datalog implementation used to accept (via a more comprehensive but slower approach).
   - we're currently discussing whether we can cut scope *here*, as this formulation accepts NLL problem case 3. We'll need to evaluate what limits this formulation imposes on expressiveness outside NLL problem case 3 and streaming iterators -- and whether it indeed has an easier path to becoming production ready. We'll also still try to see if it's possible to still improve the algorithm and avoid emitting errors on [issue 46589](https://github.com/rust-lang/rust/issues/46589), since we initially hoped to fix this one as well.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-2841974754">Comment by @epage posted on 2025-04-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: 
- @fee1-dead posted rust-lang/rust#140035 for compiler support for frontmatters (which supersedes rust-lang/rust#137193)

Blockers: 

Help wanted: 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/272'><strong>Stabilize public/private dependencies</strong></a></div>
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
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/270'><strong>SVE and SME on AArch64</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="16"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-2857967334">Comment by @davidtwco posted on 2025-05-07:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- We've resolved a handful of rounds of feedback on rust-lang/rust#137944 from @oli-obk, @lcnr  and @fee1-dead; resolved issues from a crater run (bar one); and worked to decrease the performance regression.
  - We've removed the constness parts of the patch to make it smaller and easier to review. Constness will come in a Part II. 
  - There's currently a -1% mean regression (min 0.1%, max 5.3%) that we're working to improve, but starting to run out of ideas. Regressions are just a consequence of the compiler having to prove more things with the addition of `MetaSized` bounds, rather than hot spots in newly introduced code.
  - Given the large impact of the change, we ran a crater run and found three distinct issues, two have been fixed. The remaining issue is a overflow in a single niche crate which we're working out how we can resolve.
  - We're largely just waiting on hearing from our reviewers what would be needed to see this change land. 
- We've not made any changes to the Sized Hierarchy RFC, there's a small amount of discussion which will be responded to once the implementation has landed.
- We're working on changes to the SVE RFC which further clarifies that the language changes are decided by the Sized RFC and that the SVE RFC is only proposing the forever-unstable `repr(scalable)` attribute which are non-`const Sized` and lower to `vscale` in LLVM.

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-2815572055">Comment by @jswrenn posted on 2025-04-18:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments:** After the last lang team meeting, Ralf observed that the additive/subtractive dichotomy (and its attendant design concerns w.r.t. `Drop`) could be sidestepped, since a field type *already* cannot be put into an unsound-to-drop state without unsafe code. With this observation, we can reduce field safety tooling to two rules:

1. a field should be marked unsafe if it carries a safety invariant (of any kind)
2. a field marked `unsafe` is unsafe to use

The RFC now reflects this design and has more or less reached a fixed point. Ongoing discussion on the RFC is now mostly limited to weighing this design against a proposed alternative that mixes syntactically knobs and wrapper types. The RFC would benefit from formal review by @rust-lang/lang.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/123'><strong>Use annotate-snippets for rustc diagnostic output</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="13"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


