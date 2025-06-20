+++
path = "2025/06/20/may-project-goals-update"
title = "May Project Goals Update"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

The Rust project is currently working towards a [slate of 40 project goals](https://rust-lang.github.io/rust-project-goals/2025H1/goals.html), with 3 of them designated as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025H1/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="7" max="34"></progress>
</div>
</div>
<!-- markdown separator -->

**Why this goal?** This work continues our drive to improve support for async programming in Rust. In 2024H2 we stabilized async closures; explored the generator design space; and began work on the `dynosaur` crate, an experimental proc-macro to provide dynamic dispatch for async functions in traits. In 2025H1 [our plan](https://rust-lang.github.io/rust-project-goals/2025h1/async.html) is to deliver (1) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (2) progress towards sync and async generators, simplifying the creation of iterators and async data streams; (3) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.

**What has happened?**

<!-- markdown separator -->

**Generators.** Experimental support for an `iter!` macro has landed in nightly. This is intended for nightly-only experimentation and will still need an RFC before it can stabilize. Tracking issue is [rust-lang/rust#142269](https://github.com/rust-lang/rust/issues/142269).

**Async book.** @nrc has been hard at work filling out the official Async Rust book, recently adding chapters on [concurrency primitives](https://rust-lang.github.io/async-book/part-guide/concurrency-primitives.html), structured concurrency, and pinning.

**dynosaur.** A [dynosaur RFC](https://github.com/spastorino/dynosaur/issues/78) was opened describing what blanket impls we think the proc macro should generate for a trait, to make the trait usable as `impl Trait` in argument position in other traits. This is the last remaining open design question before we release dynosaur 0.3 as a candidate for 1.0. Please chime in on the RFC if you have thoughts.


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->

<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-2957063073">Comment by @tmandry posted on 2025-06-09:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Generators.** Experimental support for an `iter!` macro has landed in nightly. This is intended for nightly-only experimentation and will still need an RFC before it can stabilize. Tracking issue is [rust-lang/rust#142269](https://github.com/rust-lang/rust/issues/142269).

**Async book.** @nrc has been hard at work filling out the official Async Rust book, recently adding chapters on [concurrency primitives](https://rust-lang.github.io/async-book/part-guide/concurrency-primitives.html), structured concurrency, and pinning.

**dynosaur.** A [dynosaur RFC](https://github.com/spastorino/dynosaur/issues/78) was opened describing what blanket impls we think the proc macro should generate for a trait, to make the trait usable as `impl Trait` in argument position in other traits. This is the last remaining open design question before we release dynosaur 0.3 as a candidate for 1.0. Please chime in on the RFC if you have thoughts.

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

The **All-Hands** did!

![picture](https://github.com/user-attachments/assets/1da0da5c-3cb0-48b5-91ae-6f043550969c)

More than 150 project members and invited guests attended, making this the largest in-person collaborative event in the history of the Rust project.

We celebrated the 10 year birthday of Rust 1.0. With over 300 people, we celebrated, listened to speeches from various former and current team members and contributors, and watched the live [release of Rust 1.87.0](https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/) on stage.

![Image](https://github.com/user-attachments/assets/c78c208c-a87a-46a3-8ee5-7086bca38b5a)

The feedback from the participants was overwhelmingly positive with an average score of 9.5/10. 🎉 The vast majority would like this to be a yearly event -- which Mara started working on.

<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2987791454">Comment by @m-ou-se posted on 2025-06-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update!

The all-hands has happened!

More than 150 project members and invited guests attended, making this the largest in-person collaborative event in the history of the Rust project.

![picture](https://github.com/user-attachments/assets/1da0da5c-3cb0-48b5-91ae-6f043550969c)

On Wednesday, several Rust project members gave talks to other project members and (potential) contributors, as part of the "Rust Project Track" at the RustWeek conference. The recordings are available [on YouTube](https://www.youtube.com/playlist?list=PL8Q1w7Ff68DBJsobwUVTr_vbb2MbxisAF). 📹

![Image](https://github.com/user-attachments/assets/b4287c7f-5813-4ec0-bf7f-bde4fd17202d)

On Thursday, we celebrated the 10 year birthday of Rust 1.0. With over 300 people, we celebrated, listened to speeches from various former and current team members and contributors, and watched the live [release of Rust 1.87.0](https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/) on stage.

![Image](https://github.com/user-attachments/assets/c78c208c-a87a-46a3-8ee5-7086bca38b5a)

On Friday and Saturday, the actual Rust All-Hands 2025 took place. For two full days spread over 10 different meeting rooms, both [pre-planned](https://docs.google.com/spreadsheets/d/1G07-f2pwAzEztZMpuxcCW3EWFS1pEX4ShNbsg91Qqjw/edit?gid=0#gid=0) and ad-hoc discussions took place on a very wide range of topics. Meeting notes have been collected in this Zulip topic: [#all-hands-2025 > Meeting notes!](https://rust-lang.zulipchat.com/#narrow/channel/486433-all-hands-2025/topic/Meeting.20notes!/with/518928628)

Many many long standing issues have been unblocked. Many new ideas were discussed, both small and big. Conflicts were resolved. Plans were made. And many personal connections were formed and improved. ❤

![Image](https://github.com/user-attachments/assets/7e5c42bc-2cf0-4e15-b69c-450264ef6e1d)

I've collected feedback from the participants (67 of you replied so far), and the replies where overwhelmingly positive with an average score of 9.5/10. 🎉 The vast majority would like this to be a yearly event. I've started working on making that happen!

<p>
<img src="https://github.com/user-attachments/assets/5d5f19e3-0ca6-4ffb-bcd7-3fb6a5f5162c" width="200" />
<img src="https://github.com/user-attachments/assets/147a14be-3f01-43e5-970b-bce779b0d361" width="200" />
<img src="https://github.com/user-attachments/assets/520ac176-5a25-4862-b7b8-acd59eef5210" width="112" />
<img src="https://github.com/user-attachments/assets/a1d21840-ec89-48d4-8a28-88180d0a90b6" width="200" />
<img src="https://github.com/user-attachments/assets/eb6c93fc-6d30-4330-96f1-a4c2a94ec3e3" width="200" />
<img src="https://github.com/user-attachments/assets/c3d65e4d-7861-44d6-9c7b-d7de95d1d954" width="112" />
<img src="https://github.com/user-attachments/assets/01ca577a-57a7-4abf-abcf-63607f410ecf" width="200" />
<img src="https://github.com/user-attachments/assets/25ac4a6e-560e-4876-9d00-48f81717dd65" width="200" />

</p>

Thank you all for attending! See you all next year! 🎊

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

**What has happened?** May saw significant progress on compiler flags, with MCPs for `-Zharden-sls` and `-Zretpoline*` being accepted. Several PRs were in progress ([#135927](https://github.com/rust-lang/rust/pull/135927), [#140733](https://github.com/rust-lang/rust/pull/140733), [#140740](https://github.com/rust-lang/rust/pull/140740)) that could potentially be combined, with the implementation approach matching clang's flag naming conventions for consistency. The RFC for configuring no-std externally [#3791](https://github.com/rust-lang/rfcs/pull/3791) entered T-compiler FCP with positive signals, and build-std discussions at the All Hands produced some consensus between libs and compiler teams, though more Cargo team involvement was needed.

The Rust for Linux team had strong participation at Rust Week, with many team members attending (Alice, Benno, Björn, Boqun, Gary, Miguel, Trevor). During the All Hands, attendees participated in a fun exercise predicting what percentage of the kernel will be written in Rust by 2035 - currently only about 0.1% of the kernel's 40M total lines are in Rust.

On language features, during May we continued work on arbitrary self types v2, where Ding focused on resolving the dichotomy between `Deref::Target` vs `Receiver::Target`. One potential approach discussed was splitting the feature gate to allow arbitrary self types only for types implementing `Deref`, which would cover the kernel use case. For `derive(CoercePointee)`, we continued waiting on PRs [#136764](https://github.com/rust-lang/rust/pull/136764) and [#136776](https://github.com/rust-lang/rust/pull/136776), with the latter needing diagnostic work.

The All Hands meeting also produced interesting developments on field projections, with Benno working on an approach that reuses borrow checker logic to extend what we do for `&` and `&mut` to custom types using the `->` syntax. Alice also presented a new proposal for AFIDT/RPITIDT and placement ([discussed here](https://rust-lang.zulipchat.com/#narrow/channel/486433-all-hands-2025/topic/Placement.20new.20in.20Rust/near/518803523)).

<!-- markdown separator -->


<details>
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2894792055">Comment by @nikomatsakis posted on 2025-05-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

TL;DR:

The primary focus for this year is compiled flags, and we are continuing to push on the various compiler flags and things that are needed to support building RFL on stable (e.g., RFC #3791 proposed adding `--crate-attr`, which permits injecting attributes into crates externally to allow the Kernel's build process to add things like `#![no_std]` so they don't have to be inserted manually into every file; MCPs for ABI flags like [`retpoline`](https://github.com/rust-lang/compiler-team/issues/868) and [`harden-sls`](https://github.com/rust-lang/compiler-team/issues/869) and [implementation of `-Zindirect-branch-cs-prefix`](https://github.com/rust-lang/rust/pull/140740)). A number of issues had minor design questions (how to manage clippy configuration; best approach for rustdoc tests) and we plan to use the RustWeek time to hash those out.

We are also finishing up some of the work on language items. We have had two stabilizations of lang features needed by Rust for Linux ([naked functions](https://github.com/rust-lang/rust/pull/134213), [`asm_goto` syntax](https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/#asm-jumps-to-rust-code)). The trickiest bit here is arbitrary self types, where we encountered a concern relating to pin and are still [discussing the best resolution](https://rust-lang.zulipchat.com/#narrow/channel/425075-rust-for-linux/topic/2025-05-07.20meeting/near/516734641).



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

The main bottleneck is the customization of the dependent `rustc-rayon` library. @oli-obk and @Zoxc are helping to move this forward.

<!-- markdown separator -->
*Help wanted:* Help test the deadlock code in the [issue list](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3AA-parallel-compiler) and try to reproduce the issues. If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Promoting.20Parallel.20Front.20End.20.28goals.23121.29/with/506292058).
<!-- markdown separator -->


<!-- markdown separator -->




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

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/109'><strong>Expose experimental LLVM features for GPU offloading</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="4"></progress>
</div>
</div>
<!-- markdown separator -->

*Help wanted:* [@ZuseZ4](https://github.com/ZuseZ4): there is only really one issue left which I'd like to see fixed before enabling autodiff on nightly, and that is MacOS support.

Most of the MacOS CI already works, we can now build Enzyme, LLVM, and rustc, but later fail when we build Cranelift due to linker flag issues. The person who was looking into it got busy with other things, so I would really appreciate it if someone could pick it up! Otherwise I can also just start by shipping autodiff on Linux only, but given how close we are to MacOS support, I feel like it would be a shame.

Since it's only an issue in CI, you don't need an own Mac to help with this. If anyone has time, I'm happy to chat here [here](https://github.com/rust-lang/rust/issues/140137) or on Zulip/Discord.


<!-- markdown separator -->


<details>
<summary>3 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-2907603375">Comment by @ZuseZ4 posted on 2025-05-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

And another round of updates. First of all, Google approved two GSoC projects for the summer, where @Sa4dUs will work on the autodiff frontend and @KMJ-007 will work on the backend. The frontend project is about improving our ABI handling to remove corner-cases around specific types that we currently can not differentiate. If time permits he might also get to re-model our frontend to lower our autodiff macro to a proper rustc intrinsic, which should allow us to simplify our logic a lot.
The backend project will look at how Enzyme uses TypeTrees, and create those during the lowering to LLVM-IR. This should allow autodiff to become more reliable, work on debug builds, and generally compile a lot faster.



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-2907625166">Comment by @ZuseZ4 posted on 2025-05-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The last weeks were focused on enabling autodiff in a lot more locations, as well as doing a lot of CI and Cmake work to be able to ship it on nightly. At the same time, autodiff is also gaining increasingly more contributors. That should help a lot with the uptick in issues, which I expect once we enable autodiff in nightly builds.

**Key developments:**
1. @Shourya742 added support for applying autodiff inside of `inherent impl blocks`. https://github.com/rust-lang/rust/pull/140104
2. @haenoe added support for applying autodiff to generic functions. https://github.com/rust-lang/rust/pull/140049
3. @Shourya742 added an optimization to inline the generated function, removing one layer of indirection. That should improve performance when differentiating tiny functions. https://github.com/rust-lang/rust/pull/139308
4. @haenoe added support for applying autodiff to inner (nested) functions. https://github.com/rust-lang/rust/pull/138314
5. I have found a bugfix for building rustc with both debug and autodiff enabled. This previously failed during bootstrap. This bugfix also solved the last remaining (compile time) performance regression of the autodiff feature. That means that if we now enable autodiff on nightly, it won't affect compile times for people not using it. https://github.com/rust-lang/rust/pull/140030
6. After a hint from Onur I also fixed autodiff check builds:https://github.com/rust-lang/rust/pull/140000, which makes contributing to autodiff easier.
7. I ran countless experiments on improving and fixing Enzyme's CMake and merged a few PRs into Enzyme. We don't fully support the macos dist runners yet and some of my CMake improvements only live in our Enzyme fork and aren't accepted by upstream yet, but the CI is now able to run longer before failing with the next bug, which should hopefully be easy to fix. At least I already received a hint on how to solve it.
8. @Shourya742 also helped with an experiment on how to bundle Enzyme with the Rust compiler. We ended up selecting a different distribution path, but the PR was helpful to discuss solutions with Infra contributors. https://github.com/rust-lang/rust/pull/140244
9. @Sa4dUs implemented a PR to split our `#[autodiff]` macro into `autodiff_forward` and `autodiff_reverse`. They behave quite differently in some ways that might surprise users, so I decided it's best for now to have them separated, which also will make teaching and documenting easier. https://github.com/rust-lang/rust/pull/140697

**Help Wanted:**
There are two or three smaller issues remaining to distribute Enzyme/autodiff. If anyone is open to help, either with bootstrap, CI, or CMake issues, I'd appreciate any support. Please feel free to ping me on Discord, Zulip, or in https://github.com/rust-lang/rust/pull/140064 to discuss what's left to do.


In general, we solved most of the distribution issues over the last weeks, and autodiff can now be applied to almost all functions. That's a pretty good base, so I will now start to look again more into the GPU support for rustc.


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-2973570029">Comment by @ZuseZ4 posted on 2025-06-15:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The last three weeks I had success in shifting away from autodiff, towards my other projects.

**Key developments:**
1) I forgot to mention it in a previous update, but I have added support for sret (struct return) handling to std::autodiff, so we now can differentiate a lot more functions reliably. https://github.com/rust-lang/rust/pull/139465

2) I added more support for batched autodiff in: https://github.com/rust-lang/rust/pull/139351

3) I have started working on a std::batching PR, which just allows fusing multiple function calls into one. https://github.com/rust-lang/rust/pull/141637. I am still not fully sure on how to design the frontend, but in general it will allow Array-of-Struct and Struct-of-Array vectorization. Based on a popular feedback I received it's now also generating SIMD types. So you can write your function in a scalar way, and just use the macro to generate a vectorized version which accepts and generates SIMD types.

4) My first PR to handle automatic data movement to and from a GPU is up! https://github.com/rust-lang/rust/pull/142097 It can handle data movements for almost arbitrary functions, as long as your function is named `kernel_{num}`, and each of your arguments is a pointer to exactly 256 f32 values. As the next step, I will probably work on the backend to generate the actual kernel launches, so people can run their Rust code on the GPU. Once I have that tested and working I will go back to develop a frontend, to remove the input type limitations and give users a way to manually schedule data transfers. The gpu/offload frontend will likely be very simple compared to my autodiff frontend, so I don't expect many complications and therefore leave it to the end.


**Help Wanted:**

There is only really one issue left which I'd like to see fixed before enabling autodiff on nightly, and that is MacOS support.
Most of the MacOS CI already works, we can now build Enzyme, LLVM, and rustc, but later fail when we build Cranelift due to linker flag issues. The person who was looking into it got busy with other things, so I would really appreciate it if someone could pick it up! Otherwise I can also just start by shipping autodiff on Linux only, but given how close we are to MacOS support, I feel like it would be a shame. Since it's only an issue in CI, you don't need an own Mac to help with this. If anyeone has time, I'm happy to chat here [here](https://github.com/rust-lang/rust/issues/140137) or on Zulip/Discord.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/262'><strong>Null and enum-discriminant runtime checks in debug builds</strong></a></div>
    <div style="flex: initial;"><progress value="1" max="3"></progress>
</div>
</div>
<!-- markdown separator -->

*Help wanted*: [1c3t3a](https://github.com/1c3t3a): happy to join forces on general checks and for advice what other UB would be great to check!! :).

<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/262#issuecomment-2900165635">Comment by @1c3t3a posted on 2025-05-22:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Upps, giving another status update here:

**Key developments**: Landed an extension of the alignment check to include (mutable) borrows in [rust#137940](https://github.com/rust-lang/rust/pull/137940). Working on the enums check (no draft PR yet). Hope to open a PR by mid next week.

**Blockers**: None so far.

**Help wanted**: Happy to join forces on general checks and for advice what other UB would be great to check!! :)

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/114'><strong>Optimizing Clippy &amp; linting</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="2"></progress>
</div>
</div>
<!-- markdown separator -->

*Help wanted:* Help is appreciated in anything with the [`performance-project` label](https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue%20state%3Aopen%20label%3Aperformance-project) in the Clippy repository.


<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-2907744144">Comment by @blyxyas posted on 2025-05-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Monthly update!

**Key developments:**

- Documentation lints have been optimized greatly, giving us up to a 13.5% decrease in documentation-heavy crates. See https://github.com/rust-lang/rust-clippy/pull/14693 and https://github.com/rust-lang/rust-clippy/pull/14870

- The efforts on getting Clippy benchmarked on the official \@rust-timer bot account are getting started by the infra team. This allows us to do per-PR benchmarking instead of fixing performance problems ad-hoc.

- We need to do further testing on the **early parallel lints effort**. While I have a working patch, no performance improvement has yet been proven.

- Work on making an interface for a single-lint Clippy, for denoising benchmarks is getting in the works.

**Blockers**
The query system not being parallelized. Currently working on a work-around but a parallel query system would make things a lot easier.

**Help wanted:**
Help is appreciated in anything with the [`performance-project` label](https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue%20state%3Aopen%20label%3Aperformance-project) in the Clippy repository.



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

<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-2904624198">Comment by @BoxyUwU posted on 2025-05-23:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We should now be correctly deferring evaluation of type system constants making use of generic parameters or inference variables. There's also been some work to make our normalization infrastructure more term agnostic (i.e. work on both types and consts). Camelid's PR mentioned in the previous update has also made great progress.

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
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="5"></progress>
</div>
</div>
<!-- markdown separator -->

<details>
<summary>No detailed updates available.</summary>
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
    <div style="flex: initial;"><progress value="4" max="6"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-2901991797">Comment by @tmandry posted on 2025-05-22:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Last week was the Rust All Hands. There were three days of discussions about interop at the all hands, led by @baumanj and including members from the Rust Project and C++ standards bodies as well as the developers of foundational Rust/C++ interop tools. The topics included

* Comparing differing needs of interop across the industry
* Sharing the design philosophy and approach of different interop tools
* Brainstorming how to tackle common interop problems between the languages, like differences in integer types, memory/object models, and move semantics
* Discussing ways the Rust and C++ languages and toolchains can develop to make interop easier in the future

Speaking for myself from the Rust Project side, it was a real pleasure to meet some of the faces from the C++ side! I look forward to working with them more in the future.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-2913058281">Comment by @Eh2406 posted on 2025-05-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The talk went smoothly and was well received. I had several useful and interesting conversations at Rust Week about  effort. That is all I have to report.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-2898702581">Comment by @epage posted on 2025-05-21:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- Key developments:
  - Moved crates to https://github.com/crate-ci/libtest2
- Blockers
- Help wanted

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-2958372526">Comment by @b-naber posted on 2025-06-10:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We have reached an [agreement](https://rust-lang.zulipchat.com/#narrow/channel/508023-project-packages-as-namespaces/topic/Consensus.20on.20compiler.20implementation/with/522594925) on the compiler implementation, and will implement it in the next 2-3 weeks hopefully.

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/257#issuecomment-2921300181">Comment by @jhpratt posted on 2025-05-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

https://github.com/rust-lang/rust/pull/141754 has been opened to parse `impl` restrictions and lower them to `rustc_middle`. A separate pull request will be opened to enforce the restriction soon after that is merged.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/259'><strong>Making compiletest more maintainable: reworking directive handling</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


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
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2910529914">Comment by @yaahc posted on 2025-05-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Quick update, Data is currently being gathered (and has been for almost 2 weeks now) on docs.rs and I should have it uploaded and accessible on the PoC dashboard within the next week or two (depending on how long I want to let the data gather).

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2936950325">Comment by @yaahc posted on 2025-06-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Bigger Update,

I've done the initial integration with the data gathered so far since rustweek. I have the data uploaded to the influxdb cloud instance managed by the infra team, I connected the infra team's grafana instance to said influxdb server and I imported my dashboards so we now have fancy graphs with real data on infra managed servers :tada:

![Image](https://github.com/user-attachments/assets/f8c9fdb0-b25a-4560-a4a9-70cd158c4e27)

![Image](https://github.com/user-attachments/assets/1998d719-5ba6-4721-8120-6ce00e4f20b2)

I'm now working with the infra team to see how we can open up access of the graphana dashboard so that anyone can go and poke around and look at the data.

Another issue that came up is that the influxdb cloud serverless free instance that we're currently using has a mandatory max 30 day retention policy, so either I have to figure out a way to get that disabled on our instance or our data will get steadily deleted and will only be useful as a PoC demo dashboard for a short window of time.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-2919293712">Comment by @lcnr posted on 2025-05-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We have triaged all major regressions discovered by the full crater run. While there are still some untriaged root regressions impacting a single crate, we've either fixed all major regressions or opened fixes to the affected crates in cases where the breakage is intended. We've started to track intended breakage in https://github.com/rust-lang/trait-system-refactor-initiative/issues/211.

We've fixed quite a few additional issues encountered via crater: https://github.com/rust-lang/rust/pull/140672 https://github.com/rust-lang/rust/pull/140678 https://github.com/rust-lang/rust/pull/140707 https://github.com/rust-lang/rust/pull/140711 https://github.com/rust-lang/rust/pull/140712 https://github.com/rust-lang/rust/pull/140713 https://github.com/rust-lang/rust/pull/141125 https://github.com/rust-lang/rust/pull/141332 https://github.com/rust-lang/rust/pull/141333 https://github.com/rust-lang/rust/pull/141334 https://github.com/rust-lang/rust/pull/141347 https://github.com/rust-lang/rust/pull/141359.

We are now tracking performance of some benchmarks with the new solver in our test suite and have started to optimize the new solver. Thank you @Kobzol for this! There are a lot of long-hanging fruit so we've made some large improvements already: https://github.com/rust-lang/rust/pull/141442 https://github.com/rust-lang/rust/pull/141500. There are also a bunch of additional improvements in-flight right now, e.g. https://github.com/rust-lang/rust/pull/141451. We still have a few crates which are *significantly* slower with the new solver, most notably `nalgebra` and `diesel`. I am confident we'll get the new solver a lot more competitive here over the next few months.

Going forward, we will continue to improve the performance of the new solver. We will also finally work through our backlog of in-process changes and land the new opaque type handling.



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-2919300609">Comment by @lcnr posted on 2025-05-29:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Ah, also @jackh726 continued to work on integrating the new solver in RustAnalyzer and it looks like we will be able to replace chalk in the near future.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/261#issuecomment-2907950212">Comment by @veluca93 posted on 2025-05-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: https://github.com/rust-lang/rust/issues/139368 was opened, which poses some possibly-relevant questions on the interaction between the `target_feature` attribute and traits. Otherwise, still trying to get a better understanding of the interaction between target feature and effects.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/106#issuecomment-2898578186">Comment by @oli-obk posted on 2025-05-21:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

No updates on my side, but we may be going back to the original proposal (modulo syntax) with a syntax that is extensible to more opt-out marker effects without lots of repetition of the `const` keyword

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
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->



<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2910844619">Comment by @epage posted on 2025-05-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

This has been [approved as a GSoC project](https://blog.rust-lang.org/2025/05/08/gsoc-2025-selected-projects/#selected-projects).

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-2927492513">Comment by @JoelMarcey posted on 2025-06-01:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key Developments: A [PR](https://github.com/rust-lang/fls/pull/563) is ready for review and merging to update the FLS to be self-sufficient, not relying on external Ferrocene packages for building. This will give us more control of changes we would like to make to the document, including theming, logos, naming, etc.

Next step: Make some modifications to the FLS content and have it published at https://rust-lang.github.io/fls

Blockers: Potential blocker around the [(re)naming / rebranding of the FLS](https://rust-lang.zulipchat.com/#narrow/channel/399173-t-spec/topic/On.20renaming.20the.20FLS/with/521565495).

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
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
<summary>2 detailed updates available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-2929535834">Comment by @davidtwco posted on 2025-06-02:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- @Jamesbarford has added the ability to write tests against the database to `rustc-perf` (rust-lang/rustc-perf#2119)
- @Jamesbarford has started to submit parts of rust-lang/rustc-perf#2081 in smaller chunks, with review feedback addressed, starting with rust-lang/rustc-perf#2134 (originally rust-lang/rustc-perf#2096)
- @Jamesbarford has prepared a [HackMD](https://hackmd.io/wq30YNEIQMSFLWWcWDSI9A) describing the design considerations involved in making rustc-perf support multiple collectors.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-2991265902">Comment by @Jamesbarford posted on 2025-06-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- @Kobzol & @Jamesbarford collaborated on finishing a workable draft for the new
  architecture of the `rustc-perf` benchmarking; https://hackmd.io/wq30YNEIQMSFLWWcWDSI9A
- @Kobzol PR enabling backfilling of data, required for the new system design
  https://github.com/rust-lang/rustc-perf/pull/2161
- @Jamesbarford PR for creating a cron job and doing a first stage queue of
  master commits; https://github.com/rust-lang/rustc-perf/pull/2163
- @Jamesbarford PR for the collectors configuration, holding off merging for the
  time being as we learn more about the system through building.
  https://github.com/rust-lang/rustc-perf/pull/2157
- @Kobzol PR allowing running the database tests on SQLite too;
  https://github.com/rust-lang/rustc-perf/pull/2152


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

<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-2913857226">Comment by @lqd posted on 2025-05-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Here are the key developments for May, though there was a bit less time this month due to the All Hands.

@amandasystems: A few more rounds of reviews were done on https://github.com/rust-lang/rust/pull/140466 (thanks to lcnr!), and most, if not all, of the feedback has been addressed already. Another PR was opened as a successor, containing another big chunk of work from the initial [PR #130227](https://github.com/rust-lang/rust/pull/130227): https://github.com/rust-lang/rust/pull/140737.

@tage64: The work discussed in the previous updates has been extracted into a few PRs, mostly to do perf runs to be able to gauge the overhead in the in-progress implementation. First, an alternative implementation to [rustc's dense bitset](https://github.com/rust-lang/rust/pull/141325), which is used extensively in dataflow analyses such as the ones in the borrow checker, for example. Then, [a prototype of the algorithm](https://github.com/rust-lang/rust/pull/141326) discussed in prior updates, trying to make the location-sensitive constraints built lazily, as well as the loans in scope themselves. (And the union of these two in [#141583](https://github.com/rust-lang/rust/pull/))

@lqd: As discussed in the previous update, I've tried to see if we can limit scope here by evaluating the current algorithm a bit more: the expressiveness it allows, and where it fails. I've also evaluated all the open issues about NLL expressiveness that we hoped to fix, and see the ones we support now or could defer to future improvements. It seems _possible_. I've also started to have some idea of the work needed to make it more production-ready. That includes the experiments made with Tage above, but also trying to lower the total overhead by finding wins in NLLs, and here I e.g. [have some improvements in-flight](https://github.com/rust-lang/rust/pull/141667) for the dataflow analysis used in liveness.

All Hands: we discussed with t-types the plan and in-progress PRs about opaque types, how they impact member constraints and in turn the constraint graph and SCCs. Some more work is needed here to ensure member constraints are correctly handled, even though they should only impact the SCCs and not the borrow checking algorithm per se (but there still are possible ambiguity issues if we don't take flow sensitivity into account here).

(Fun and interesting aside: there's [an RFC](https://discourse.llvm.org/t/rfc-intra-procedural-lifetime-analysis-in-clang/86291) to add a polonius-like lifetime analysis to clang)

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
    <div style="flex: initial;"><progress value="31" max="37"></progress>
</div>
</div>
<!-- markdown separator -->



<!-- markdown separator -->


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/119#issuecomment-2898705513">Comment by @epage posted on 2025-05-21:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments:
- rust-lang/rust#140035 has been merged

Blockers:

Help wanted:


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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-2929487910">Comment by @davidtwco posted on 2025-06-02:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

- rust-lang/rust#137944 is ready! It's in a t-types FCP to merge as there's a small unavoidable breakage (unless we wanted to wait for the new trait solver).
  - Once this is merged, I'll work on a `#[rustc_no_implicit_bounds]` attribute for tests, testing whether `Deref::Target` can be relaxed, and Part II.
- I've still not made any changes to the Sized Hierarchy RFC, there's a small amount of discussion which will be responded to once the implementation has landed.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-2901139665">Comment by @jswrenn posted on 2025-05-22:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments:** No significant developments since [previous updates](https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-2815572055).

**Blockers:** [Waiting](https://github.com/rust-lang/rfcs/pull/3458#issuecomment-2825837922) on lang team review.

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
