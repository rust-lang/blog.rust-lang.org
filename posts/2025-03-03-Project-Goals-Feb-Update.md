---
layout: post
title: "February Project Goals Update"
author: Rémy Rakic, Niko Matsakis, Santiago Pastorino
team: Goals Team <https://www.rust-lang.org/governance/teams/goals>
---

This is the first Project Goals update for the new 2025h1 period. For the first 6 months of 2025, the Rust project will work towards a [slate of 39 project goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html), with 3 of them designed as [Flagship Goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html#flagship-goals). This post provides selected updates on our progress towards these goals (or, in some cases, lack thereof). The full details for any particular goal are available in its associated [tracking issue on the rust-project-goals repository](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

## Flagship goals

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/105'><strong>Bring the Async Rust experience closer to parity with sync Rust</strong></a></div>
    <div style="flex: initial;"><progress value="5" max="34"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** This work continues our drive to improve support for async programming in Rust. In 2024H2 we stabilized async closures; explored the generator design space; and began work on the `dynosaur` crate, an experimental proc-macro to provide dynamic dispatch for async functions in traits. In 2025H1 [our plan](https://rust-lang.github.io/rust-project-goals/2025h1/async.html) is to deliver (1) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (2) progress towards sync and async generators, simplifying the creation of iterators and async data streams; (3) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.

**What has happened?** The biggest news is that Rust 1.85 is stable and includes two major features that impact Async Rust. The first is [async closures](https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html), which has been on many people&#x27;s wish lists for a long time and was expertly moved forward by @compiler-errors over the last year.

The second feature included in 1.85 is the new [lifetime capture rules](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html) as part of Rust 2024 edition. This should substantially improve the experience of using async Rust anytime a user writes `-> impl Future`, as it removes the need for `+ '_` or similar bounds in most cases. It will also lead to an easier to understand language, since those bounds only worked by exploiting the more subtle rules of `impl Trait` in a way that runs contrary to their actual semantic role in the language. In the 2024 Edition, the subtle rule is gone and we capture all input lifetimes by default, with the ability to use `+ use<>` syntax to opt out. See [this blog post](https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html) for more.

**Generators.** The lang team also held a design meeting to review the design for generators, with the outcome of the last one being that we will implement a `std::iter::iter!` macro (exact path TBD) in the compiler, as a lang team experiment that allows the use of the `yield` syntax. We decided to go in this direction because we want to reserve `gen` for self-borrowing and perhaps lending generators, and aren&#x27;t yet decided on which subset of features to expose under that syntax. This decision interacts with ongoing compiler development that isn&#x27;t ready yet to enable experimentation with lending.

Our hope is that in the meantime, by shipping `iter!` we will give people the chance to start using generators in their own code and better understand which limitations people hit in practice.

As you may have noticed, I&#x27;m not talking about async generators here. Those are the ultimate goal for the async initiative, but we felt the first step should be clarifying the state of synchronous generators so we can build on that when talking about async ones.

**Dynosaur.** [dynosaur v0.1.3](https://github.com/spastorino/dynosaur/releases/tag/0.1.3) was released, with another release in the works. We think we are approaching a 1.0 release real soon now (tm). At this point you should be able to try it on your crate to enable dyn dispatch for traits with `async fn` and other `-> impl Trait` methods. If you need to use it together with `#[trait_variant]`, you may need to wait until the next release when [#55](https://github.com/spastorino/dynosaur/issues/55) is fixed.

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/105#issuecomment-2686259726">Comment by @tmandry posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

## Rust 1.85

The first update is the release of Rust 1.85, which had at least two major features that impact Async Rust. The first is [async closures](https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html), which has been on many people's wish lists for a long time and was expertly moved forward by @compiler-errors over the last year.

The second is the new [lifetime capture rules](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html) as part of Rust 2024 edition. This should substantially improve the experience of using async Rust anytime a user writes `-> impl Future`, as it removes the need for `+ '_` or similar bounds in most cases. It will also lead to an easier to understand language, since those bounds only worked by exploiting the more subtle rules of `impl Trait` in a way that runs contrary to their actual semantic role in the language. In the 2024 Edition, the subtle rule is gone and we capture all input lifetimes by default, with the ability to use `+ use<>` syntax to opt out. See [this blog post](https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html) for more.

## Generators

The lang team held two design meetings on generators, with the outcome of the last one being that we will implement a `std::iter::iter!` macro (exact path TBD) in the compiler, as a lang team experiment that allows the use of the `yield` syntax. We decided to go in this direction because we want to reserve `gen` for self-borrowing and perhaps lending generators, and aren't yet decided on which subset of features to expose under that syntax. This decision interacts with ongoing compiler development that isn't ready yet to enable experimentation with lending.

Our hope is that in the meantime, by shipping `iter!` we will give people the chance to start using generators in their own code and better understand which limitations people hit in practice.

As you may have noticed, I'm not talking about async generators here. Those are the ultimate goal for the async initiative, but we felt the first step should be clarifying the state of synchronous generators so we can build on that when talking about async ones.

## dynosaur

[dynosaur v0.1.3](https://github.com/spastorino/dynosaur/releases/tag/0.1.3) was released, with another release in the works. We think we are approaching a 1.0 release real soon now (tm). At this point you should be able to try it on your crate to enable dyn dispatch for traits with `async fn` and other `-> impl Trait` methods. If you need to use it together with `#[trait_variant]`, you may need to wait until the next release when [#55](https://github.com/spastorino/dynosaur/issues/55) is fixed.

## Other

The async project goal was accepted for 2025H1!

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/263'><strong>Organize Rust All-Hands 2025</strong></a></div>
    <div style="flex: initial;"><progress value="9" max="28"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** May 15, 2025 marks the 10-year anniversary of Rust&#x27;s 1.0 release; it also marks 10 years since the [creation of the Rust subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). At the time [there were 6 Rust teams with 24 people in total](http://web.archive.org/web/20150517235608/http://www.rust-lang.org/team.html). There are now 57 teams with 166 people. In-person All Hands meetings are an effective way to help these maintainers get to know one another with high-bandwidth discussions. This year, the Rust project will be coming together for [RustWeek 2025](https://2025.rustweek.org), a joint event organized with [RustNL](https://2025.rustweek.org/about/). Participating project teams will use the time to share knowledge, make plans, or just get to know one another better. One particular goal for the All Hands is reviewing a draft of the [Rust Vision Doc](./rust-vision-doc.md), a document that aims to take stock of where Rust is and lay out high-level goals for the next few years.

**What has happened?** Planning is proceeding well. In addition to Rust maintainers, we are inviting all project goal owners to attend the All Hands (note that the accompanying [RustWeek conference](https://rustweek.org/) is open to the public, it's just the [All Hands](https://rustweek.org/all-hands/) portion that is invitation only). There are currently over 100 project members signed up to attend.

For those invited to the All Hands:

* Travel funds may be available if you are unable to finance travel through your employer. Get in touch for details.
* Please participate in the brainstorm for how best to use the All Hands time in the all-hands-2025 Zulip stream.
* If you do plan to attend,  please purchase your travel + [hotels](https://rustweek.org/hotels-all-hands/) in a timely fashion as the discount rates will expire.

<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/263#issuecomment-2690795898">Comment by @m-ou-se posted on 2025-02-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

What happened so far:

- Allocated a budget for the event.
- Funds have been transferred from the Rust Foundation to RustNL.
- Booked the venue, including lunch and snacks.
- Remaining budget from last year's travel grant programme added to this year's, to help cover the travel costs.
- Announcement published: https://blog.rust-lang.org/inside-rust/2024/09/02/all-hands.html
- After sending many reminders to teams and individuals, about 110 project members signed up. (And a few cancelled.)
- Formal invitations sent out to all those who registered.
- Information on the all-hands: https://rustweek.org/all-hands/
- Hotel reservations available: https://rustweek.org/hotels-all-hands/
- Created a public and a private zulip channel.
- About 65 people confirmed they booked their hotel.
- Opened up discussion on what to discuss at the all-hands.
- Invited guests: project goal (task) owners who aren't a project member (12 people in total). 4 of those signed up so far.
- Acquired 150 secret gifts for the pre-all-hands day.

Next up:
- Remind folks to get a ticket for the RustWeek conference (tuesday+wednesday) if they want to join that as well.
- Invite more guests, after deciding on who else to invite. (To be discussed today in the council meeting.)
- Figure out if we can fund the travel+hotel costs for guests too. (To be discussed today in the council meeting.)
- Organise all the ideas for topics at the all-hands, so we can turn them into a consistent schedule later.
- Draft an allocation of the rooms depending on the teams and topics.
- Open the call for proposals for talks for the Project Track (on wednesday) as part of the RustWeek conference.



<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>



<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/116'><strong>Stabilize tooling needed by Rust for Linux</strong></a></div>
    <div style="flex: initial;"><progress value="8" max="22"></progress>
</div>
</div>
<!-- markdown separator --> 

**Why this goal?** This goal continues our work from 2024H2 in supporting the [experimental support for Rust development in the Linux kernel][RFL.com]. Whereas in 2024H2 we were focused on stabilizing required language features, our focus in 2025H1 is stabilizing compiler flags and tooling options. We will (1) implement [RFC #3716] which lays out a design for ABI-modifying flags; (2) take the first step towards stabilizing [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](./build-std.md); (3) extending rustdoc, clippy, and the compiler with features that extract metadata for integration into other build systems (in this case, the kernel&#x27;s build system).

[RFC #3716]: https://github.com/rust-lang/rfcs/pull/3716
[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

**What has happened?** We established the precise set of 2025H1 deliverables and we have been tracking them and have begun making progress towards them. Rustdoc has been updated to support extracting doc tests so that the Kernel can execute them in a special environment (this was previously done with a big hack) and RFL is in the process of trying to us that new support. The first PR towards the implementation of [RFC #3716](https://github.com/rust-lang/rfcs/pull/3716) has landed and the ARM team has begun reading early drafts of the design doc for `-Zbuild-core` with the cargo team.

We are also working to finalize the stabilization of the language features that were developed in 2024H2, as two late-breaking complications arose. The first (an interaction between casting of raw pointers and arbitrary self types) is expected to be resolved by limiting the casts of raw pointers, which previously accepted some surprising code. We identified that only a very small set of crates relied on this bug/misfeature; we expect nonetheless to issue a forwards compatibility warning. We are also resolving an issue where `derive(CoercePointee)` was found to reveal the existence of some unstable impls in the stdlib.

<!-- markdown separator --> 


<details>
<summary>3 detailed updates availabled.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2593828706">Comment by @nikomatsakis posted on 2025-01-15:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

In our meeting today we reviewed the plans for the [2025H1 project goal](https://rust-lang.github.io/rust-project-goals/2025h1/rfl.html)...
 
**"Almost done" stuff from before**

* re-stabilize CoercePointee -- Alice is looking at this, it's a good opportunity to try out the new template that is being discussed
* stabilize arbitrary self types v2 -- @adetaylor [left a comment 3 weeks ago](https://github.com/rust-lang/rust/issues/44874) indicating that everything was more-or-less landed. RFL is [already using it](https://github.com/Rust-for-Linux/linux/commit/c95bbb59a9b2), providing evidence that it works reasonably well. Next steps are then sorting out documentation and moving to stabilize. 
* asm-goto -- ready for stabilization, not been merged yet, still doing some work on the Rust reference (PRs https://github.com/rust-lang/rust/pull/133870, https://github.com/rust-lang/reference/pull/1693)

**ABI-modifying flags**

The rust-lang/rfcs#3716 is now in Final Comment Period (FCP). There is a preliminary implementation in #133138 that @petrochenkov is going to be reviewing. Some more work will be needed to test, cleanup, etc after that lands.

**Other flags from [RFL#2](https://github.com/Rust-for-Linux/linux/issues/2)**

We went through a series of flags from that RFL uses and looking into what might be blocking each of them. The process to stabilize one of these is basically to prepare the stabilization PR (minimal, but we need to rename the flag from `-Z` to `-C`) with a stabilization report and proper docs and then cc @davidtwco or @wesleywiser to prepare stabilization. In most cases we need to document how the flags can be misused, most commonly by linking against std or other crates not built with the same flags. If there are major correctness concerns as a result we likely want to consider the flag as "ABI-modifying".

* ability to extract dependency info, currently using `-Zbinary_dep_depinfo=y` -- basically ready to stabilize
        * tmandry: Do you want toolchain runtimes (libstd, compiler-rt) in your dep info? In my experience this features does 90% of what I want it to do, but removing the inclusion of runtimes is the only question I have before stabilizing.
* -Zcrate-attr, used to configure no-std without requiring it in the source file -- no real concerns
* `-Zunpretty=expanded` -- have to clearly document that the output is not stable, much like we did for emitting MIR. This is already "de facto" stable because in practice `cargo expand` uses `RUSTC_BOOTSTRAP=1` and everybody uses it. 
* `-Zno-jump-tables` -- this should be considered an ABI-modifying flag because we believe it is needed for CFI and therefore there is a risk if incompatible modules are linked.
* `-Zdwarf-version`, `-Zdebuginfo-compression` -- this should be ready to stabilize, so long as we document that you should expect a mix of debuginfo etc when mixing things compiled with different versions (notably libstd, which uses DWARF4). Debuggers are already prepared for this scenario. zstd compression is supported as of Rust 1.82.0.

**stable rustdoc features allowing the RFL project to extract and customize rustdoc tests (`--extract-doctests`)**

@imperio authored https://github.com/rust-lang/rust/pull/134531, which is now up for review. Once PR lands, RFL will validate the design, and it can proceed to stabilization.

**clippy configuration (possibly `.clippy.toml` and `CLIPPY_CONF_DIR`)**

We discussed with clippy team, seems like this is not a big deal, mostly a doc change, one concern was whether clippy should accept options it doesn't recognize (because they may come from some future version of clippy). Not a big deal as of now, RFL only uses (msrv, check-private-items=true, disallowed-macros).

**rebuild libcore**

ARM team is working on this as part of [this project goal](https://rust-lang.github.io/rust-project-goals/2025h1/build-std.html), expect updates. 🎉 

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2623263974">Comment by @nikomatsakis posted on 2025-01-30:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Updates:

## 2024H2 cleanup

* Arbitrary self types v2: Stabilization PR is open (rust-lang/rust#135881).
* Stabilize CoercePointee: RFL is using it now; we are making progress towards completing the stabilization.

## 2025H1

* ABI-modifying flags RFC (rust-lang/rfcs#3716) has [completed FCP](https://github.com/rust-lang/rfcs/pull/3716#issuecomment-2614078103) and needs to be merged. Implementation PR [#133138](https://github.com/rust-lang/rust/pull/133138) authored by @azhogin remains under review.
* Other compiler flags, we made a prioritized list. One easy next step would be to rename all `-Z` flags in this list to something stabilizable (e.g., `-C`) that requires `-Zunstable-features`.
  * [ ] `-Zdwarf-version` -- wesley wiser
  * [ ] `-Zdebuginfo-compression`, unblocked
  * [ ] `-Zcrate-attr`, used to configure no-std without requiring it in the source file, no real concerns
  * [ ] `-Zunpretty=expanded`, unblocked, maybe needs a PR that says "don't rely on this", Linux only uses it for debugging macros (i.e. not in the normal build, so it is less critical). Needs a stable name, e.g., `--emit macro-expanded`, and we should make sure output cannot be piped to rustc. `rustfmt` told us (Rust for Linux) that they will try their best to keep `rustfmt` able to format the output of the expansion.
  * [ ] `-Zno-jump-tables`, considered an ABI-modifying flag
  * [ ] `-Zbinary_dep_depinfo=y` -- basically ready to stabilize (@tmandry asked "Do you want toolchain runtimes (libstd, compiler-rt) in your dep info? In my experience this features does 90% of what I want it to do, but removing the inclusion of runtimes is the only question I have before stabilizing", but we don't understand this point yet, as they were not present in the meeting).
* stable rustdoc: PR rust-lang/rust#134531 is under review, should land soon, then next step will be for RFL folks to try it out.
* Clippy configuration: no progress, discussed some new options and posted in thread, very minimal work required here.
* rebuild libcore: @davidtwco wasn't present so no update, but we know progress is being made

### Publicizing this work

We discussed briefly how to better get the word out about this collaboration. Some points:

* @Darksonn will be speaking at Rust Nation
* We could author a Rust-lang blog post, perhaps once the language items are done done done?
* LWN article might be an option

### general-regs-only

We discussed the possibility of a flag to avoid use of floating point registers, no firm conclusion yet reached.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/116#issuecomment-2672338598">Comment by @nikomatsakis posted on 2025-02-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Updates from our [2025-02-12 meeting](https://hackmd.io/AkNBW942SoacLayPXHthCg):

Given the recent controversy about Rust usage in the Kernel, the RFL group wrote up a [policy document explainer](https://rust-for-linux.com/rust-kernel-policy) to explain the policy, and there was a [write-up on LWN](https://lwn.net/SubscriberLink/1007921/9020dbb12585d48f/).

Regarding arbitary self types and coerce pointee, we are waiting on rust-lang/rust#136764 and rust-lang/rust#136776. The former is on lang team FCP. The latter has received approval from lang team and is awaiting further impl work by @BoxyUwU.

@ojeda is looking into how to manage dependency information and configure no-std externally.

@GuillaumeGomez's impl of rustdoc features has landed and we are waiting on RFL to experiment with it.

@davidtwco's team at ARM has authored a document regarding a blessed way to build-std and are collecting feedback.

@wesleywiser is preparing a PR to add `-Zdwarf-version` to help advance compiler flags.

There is an annoying issue related to `cfg(no_fp_fmt_parse)`, which is no longer used by RFL but which remains in an older branch of the kernel (6.12, LTS). 

As a set of "leaf crates" that evolve together in a mono-repo-like fashion,  RFL would like to have a solution for disabling the orphan rule.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>

<br>

## Goals looking for help

<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/256'><strong>Implement Open API Namespace Support</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="3"></progress>
</div>
</div>

*Help wanted:* this project goal needs a compiler developer to move forward. If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Implement.20Open.20API.20Namespace.20Support.20.28goals.23256.29).

<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/256#issuecomment-2672709987">Comment by @epage posted on 2025-02-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Help wanted: this project goal needs a compiler developer to move forward.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/264'><strong>Prototype a new set of Cargo &quot;plumbing&quot; commands</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>

*Help wanted:* this project goal needs someone to work on the implementation. If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Prototype.20a.20new.20set.20of.20Cargo.20.22plumbing.22.20.28goals.23264.29).

<details>
<summary>2 detailed updates availabled.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2672710683">Comment by @epage posted on 2025-02-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Help wanted: this project goal needs someone to work on the implementation

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/264#issuecomment-2676759836">Comment by @ashiskumarnaik posted on 2025-02-23:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Hi @epage I am very interested to collaborate in this implementation work. Let's talk on Zulip, Check DM.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/266'><strong>Publish first version of StableMIR on crates.io</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="6"></progress>
</div>
</div>

*Help wanted:* looking for people that want to help do a bit of refactoring. Please reach out through the project-stable-mir [zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/320896-project-stable-mir) or [repository](https://github.com/rust-lang/project-stable-mir).

<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/266#issuecomment-2683064894">Comment by @celinval posted on 2025-02-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

No progress yet.

**Help Wanted:** Looking for people that want to help do a bit of refactoring. Please reach out through the project-stable-mir [zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/320896-project-stable-mir) or [repository](https://github.com/rust-lang/project-stable-mir).

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<br>
<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/272'><strong>Stabilize public/private dependencies</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>

*Help wanted:* this project goal needs a compiler developer to move forward. If you'd like to help, please post in [this goal's dedicated zulip topic](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Stabilize.20public.2Fprivate.20dependencies.20.28goals.23272.29).

<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/272#issuecomment-2672709551">Comment by @epage posted on 2025-02-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Help wanted: this project goal needs a compiler developer to move forward.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/100#issuecomment-2688795182">Comment by @BoxyUwU posted on 2025-02-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

camelid has a PR up which is ~fully finished + reviewed which enables resolving and lowering all paths under `min_generic_const_args`. It's taken a while to get this bit finished as we had to take care not to make parts of the compiler unmaintainable by duplicating all the logic for type and const path lowering.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/274#issuecomment-2668390281">Comment by @davidtwco posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

An initial update on what we've been up to and some background:

- This goal is submitted on behalf of the Rust team at Arm, but primarily worked on by @AdamGemmell. Anyone interested can always contact me for updates and I'll keep this issue up-to-date.
- Our team has been trying to make progress on build-std by completing the issues in [rust-lang/wg-cargo-std-aware](https://github.com/rust-lang/wg-cargo-std-aware) but found this wasn't especially effective as there wasn't a clearly defined scope or desired outcome for most issues and the relevant teams were lacking in the necessary context to evaluate any proposals.
- We've since had discussions with the Cargo team and agreed to draft a document describing the use cases, motivations and prior art for build-std such that the Cargo team can feel confident in reviewing any further proposals.
  - @AdamGemmell shared an initial draft of this [in #t-cargo on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/build-std.20goal/near/496782452) and **it is undergoing further revisions following feedback**.
  - Following reaching a shared understanding of the context of the feature, @AdamGemmell will then draft a complete proposal for build-std that could feasibly be stabilised. It will describe the use cases which are and are not considered in-scope for build-std, and which will feature in an initial implementation.
- @davidtwco is ensuring that whatever mechanism that is eventually proposed to enable build-std to work on stable toolchains will also be suitable for the Rust for Linux project to use when building core themselves.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/104'><strong>Continue resolving &#x60;cargo-semver-checks&#x60; blockers for merging into cargo</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="5"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/104#issuecomment-2667282957">Comment by @obi1kenobi posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Thanks, Niko! Copying in the new tasks from the 2025h1 period:
* [ ] Prototype cross-crate linting using workarounds (@obi1kenobi)
* [ ] Allow linting generic types, lifetimes, bounds (@obi1kenobi)
* [ ] Handle "special cases" like `'static` and `?Sized` (@obi1kenobi)
* [ ] Handle `#[doc(hidden)]` in sealed trait analysis (@obi1kenobi)
* [x] Discussion and moral support ([cargo](https://github.com/rust-lang/cargo), [rustdoc]

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/253#issuecomment-2686189141">Comment by @tmandry posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We had a lang team design meeting about C++ interop today. The outcome was very positive, with enthusiasm for supporting an ambitious vision of C++ interop: One where a large majority of real-world C++ code can have automated bindings to Rust and vice-versa.

At the same time, the team expressed a desire to do so in a way that remains in line with Rust's values. In particular, we would not make Rust a superset of Rust+C++, but instead would define extension points that can be used to express language interop boundaries that go beyond what Rust itself allows. As an example, we could allow template instantiation via a Rust "plugin" without adding templates to Rust itself. Similarly, we could allow calling overloaded C++ methods without adding function overloading to Rust itself. Other interop needs are more natural to enable with features in the Rust language, like custom reference types.

In either case, anything we do to support interop will need to be considered on its merits. Interop is a reason to support a feature, but it is never a "blank check" to add anything we might consider useful.

The discussion so far has been at a high level. Next steps will be:

- Discuss what significant-but-realistic milestones we might pursue as part of upcoming project goals, and what it would take to make them happen. Whether this happens as part of another lang team meeting or a more dedicated kickoff meeting for interested parties, I'll be sure to keep the lang team in the loop and will continue posting updates here.
- Dive into more specific proposals for use cases we would like to enable in meetings with the language, library, and compiler teams.

Notes: https://hackmd.io/2Ar_7CNoRkeXk1AARyOL7A?view

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/107'><strong>Experiment with ergonomic ref-counting</strong></a></div>
    <div style="flex: initial;"><progress value="2" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/107#issuecomment-2685478012">Comment by @spastorino posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

There's a PR up https://github.com/rust-lang/rust/pull/134797 which implements the proposed RFC without the optimizations.
The PR is not yet merged and we need to continue now working on addressing comments to the PR until is merged and then start implementing optimizations.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/109#issuecomment-2569888810">Comment by @ZuseZ4 posted on 2025-01-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Happy New Year everyone! After a few more rounds of feedback, the next autodiff PR recently got merged: https://github.com/rust-lang/rust/pull/130060
With that, I only have one last PR open to have a fully working autodiff MVP upstream. A few features had to be removed during upstreaming to simplify the reviewing process, but they should be easier to bring back as single PRs. 

Beginning next week, I will also work on an MVP for the `batching` feature of LLVM/Enzyme, which enables some AoS and SoA vectorization. It mostly re-uses the existing autodiff infrastructure, so I expect the PRs for it to be much smaller.

On the GPU side, there has been a recent push by another developer to add a new AMD GPU target to the Rust compiler. This is something that I would have needed for the llvm offload project anyway, so I'm very happy to see movement here: https://github.com/rust-lang/compiler-team/issues/823

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/110#issuecomment-2685771231">Comment by @Eh2406 posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

The major update so far is the release of [PubGrub 0.3](https://github.com/pubgrub-rs/pubgrub/releases/tag/v0.3.0). This makes available the critical improvements made to allow the functionality and performance demanded by Cargo and UV. The other production users can now take advantage of the past few years of improvements. Big thanks to @konstin for making the release happen.

Other progress is been stymied by being sick with Covid for a week in January and the resulting brain fog, followed by several high-priority projects for $DAY_JOB. It is unclear when the demands of  work will allow me to  return focus to this project.

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/254#issuecomment-2690848867">Comment by @m-ou-se posted on 2025-02-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Now that https://github.com/rust-lang/rust/pull/135726 is merged, @jdonszelmann and I will be working on implementing EII.

We have the design for the implementation worked out on our whiteboard. We don't expect any significant blockers at this point. We'll know more once we start writing the code next week.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/255#issuecomment-2672713599">Comment by @epage posted on 2025-02-20:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

This is on pause until the implementation for #92 is finished.  The rustc side of #92 is under review and then some additional r-a and cargo work before implementation is done and its waiting on testing and stabilization.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/257#issuecomment-2683781126">Comment by @jhpratt posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

First status update:

No progress. I will be reviewing the existing PR this weekend to see the feasibility of rebasing it versus reapplying patches by hand. My suspicion is that the latter will be preferable.

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


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/258#issuecomment-2683622470">Comment by @traviscross posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

This is, I believe, mostly waiting on us on the lang team to have a look, probably in a design meeting, to feel out what's in the realm of possibility for us to accept.


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

<a href="https://github.com/rust-lang/rust-project-goals/issues/126#issuecomment-2568584011">Comment by @celinval posted on 2025-01-03:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments:** We have written and verified around 220 safety contracts in the [verify-rust-std fork](https://github.com/model-checking/verify-rust-std). 3 out of 14 challenges have been solved. We have successfully integrated Kani in the repository CI, and we are working on the integration of 2 other verification tools: VeriFast and Goto-transcoder (ESBMC)




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
<summary>2 detailed updates availabled.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/259#issuecomment-2668468138">Comment by @jieyouxu posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update (2025-02-19):

- To make it easier to follow bootstrap's test flow going into running compiletest-managed test suites, I've added more tracing to bootstrap in https://github.com/rust-lang/rust/pull/137080.
- There are some prerequisite cleanups/improvements that I'm working on first to make it easier to read bootstrap + compiletest's codebase for reference: https://github.com/rust-lang/rust/pull/137224, https://github.com/rust-lang/rust/pull/136474, https://github.com/rust-lang/rust/pull/136542
- I'm thinking for the prototype I'm going to experiment with a branch off of rust-lang/rust instead of completely separately, so I can experiment under the context of bootstrap and tests that we actually have, instead of trying to experiment with it in a complete vacuum (esp. with respect to staging and dependency licenses).
    - Onur is working on [stabilize stage management for rustc tools #137215](https://github.com/rust-lang/rust/pull/137215) which I will wait on to match `ToolStd` behavior as used by compiletest.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/259#issuecomment-2688071048">Comment by @jieyouxu posted on 2025-02-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update (2025-02-27):

- Cleanups still waiting to be merged (some PRs are blocked on changes from others making this slow).

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
<summary>2 detailed updates availabled.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2683346208">Comment by @yaahc posted on 2025-02-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I'm very excited to see that this got accepted as a project goal 🥰 🎉 

Let me go ahead and start by giving an initial status update of where I'm at right now.

* We've already landed the initial implementation for configuring the directory where metrics should be stored which also acts as the enable flag for a default set of metrics, right now that includes ICE reports and unstable feature usage metrics
* Implemented basic unstable feature usage metrics, which currently dumps a json file per crate that is compiled showing which metrics are enabled. (example below)
* Implemented `rust-lang/rust/src/tools/features-status-dump` which dumps the status information for all unstable, stable, and removed features as a json file
* setup an extremely minimal initial Proof of Concept implementation locally on my laptop using InfluxDB 3.0 Alpha and Graphana (image below)
  * I have a small program I wrote that converts the json files into [influxdb's line protocol](https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/) for both the usage info and the status info (snippets shown below)
    * The timestamps are made up, since they all need to be distinct or else influxdb will treat them all as updates to the same row, I'd like to preserve this information from when the metrics were originally dumped, either in the json or by changing rustc to dump via influxdb's line format directly, or some equivalent protocol. (note this is probably only necessary for the usage metrics, but for the status metrics I'd have to change the line format schema from the example below to avoid the same problem, this has to do with how influxdb treats tags vs fields)
  * I gathered a minimal dataset by compiling rustc with `RUSTFLAGS_NOT_BOOTSTRAP="-Zmetrics-dir=$HOME/tmp/metrics" ./x build --stage 2` and `./x run src/tools/features-status-dump/`, save the output to the filesystem, and convert the output to the line protocol with the aforementioned program
  * Write the two resulting files to influxdb
  * I then setup the table two different ways, once by directly querying the database using influxdb's cli (query shown below), then again by trying to setup an equivalent query in graphana (there's definitely some kinks to work out here, I'm not an expert on graphana by any means.)

from `unstable_feature_usage_metrics-rustc_hir-3bc1eef297abaa83.json`

```json
{"lib_features":[{"symbol":"variant_count"}],"lang_features":[{"symbol":"associated_type_defaults","since":null},{"symbol":"closure_track_caller","since":null},{"symbol":"let_chains","since":null},{"symbol":"never_type","since":null},{"symbol":"rustc_attrs","since":null}]}
```

![Image](https://github.com/user-attachments/assets/e6e560f8-b06d-4843-a46f-a83e7a3e49c0)

Snippet of unstable feature usage metrics post conversion to line protocol

```
featureUsage,crateID="bc8fb5c22ba7eba3" feature="let_chains" 1739997597429030911
featureUsage,crateID="439ccecea0122a52" feature="assert_matches" 1739997597429867374
featureUsage,crateID="439ccecea0122a52" feature="extract_if" 1739997597429870052
featureUsage,crateID="439ccecea0122a52" feature="iter_intersperse" 1739997597429870855
featureUsage,crateID="439ccecea0122a52" feature="box_patterns" 1739997597429871639
```

Snippet of feature status metrics post conversion to line protocol

```
featureStatus,kind=lang status="unstable",since="1.5.0",has_gate_test=false,file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/unstable.rs",line=228,name="omit_gdb_pretty_printer_section" 1739478396884006508
featureStatus,kind=lang status="accepted",since="1.83.0",has_gate_test=false,tracking_issue="123742",file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/accepted.rs",line=197,name="expr_fragment_specifier_2024" 1739478396884040564
featureStatus,kind=lang status="accepted",since="1.0.0",has_gate_test=false,file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/accepted.rs",line=72,name="associated_types" 1739478396884042777
featureStatus,kind=lang status="unstable",since="1.79.0",has_gate_test=false,tracking_issue="123646",file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/unstable.rs",line=232,name="pattern_types" 1739478396884043914
featureStatus,kind=lang status="accepted",since="1.27.0",has_gate_test=false,tracking_issue="48848",file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/accepted.rs",line=223,name="generic_param_attrs" 1739478396884045054
featureStatus,kind=lang status="removed",since="1.81.0",has_gate_test=false,tracking_issue="83788",file="/home/jlusby/git/rust-lang/rust/compiler/rustc_feature/src/removed.rs",line=245,name="wasm_abi" 1739478396884046179
```

Run with `influxdb3 query --database=unstable-feature-metrics --file query.sql`

```SQL
SELECT
  COUNT(*) TotalCount, "featureStatus".name
FROM
  "featureStatus"
INNER JOIN "featureUsage" ON
  "featureUsage".feature = "featureStatus".name
GROUP BY
  "featureStatus".name
ORDER BY
    TotalCount DESC
```

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/260#issuecomment-2683396064">Comment by @yaahc posted on 2025-02-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

My next step is to revisit the output format, currently a direct json serialization of the data as it is represented internally within the compiler. This is already proven to be inadequate by personal experience, given the need for additional ad-hoc conversion into another format with faked timestamp data that wasn't present in the original dump, and by conversation with @badboy (Jan-Erik), where he recommended we explicitly avoid ad-hoc definitions of telemetry schemas which can lead to difficult to manage chaos.

I'm currently evaluating what options are available to me, such as a custom system built around influxdb's line format, or opentelemetry's metrics API.

Either way I want to use firefox's telemetry system as inspiration / a basis for requirements when evaluating the output format options

Relevant notes from my conversation w/ Jan-Erik

- firefox telemetry starts w/ the question, what is it we want to answer by collecting this data? has to be explicitly noted by whoever added new telemetry, there's a whole process around adding new telemetry
	- defining metric
	- description
	- owner
	- term limit (expires automatically, needs manual extension)
	- data stewards
		- do data review
		- checks that telemetry makes sense
		- check that everything adheres to standards
	- can have downside of increasing overhead to add metrics
	- helps w/ tooling, we can show all this info as documentation
	- schema is generated from definitions

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/122#issuecomment-2686403694">Comment by @nikomatsakis posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

I have established some regular "office hour" time slots where I am available on jitsi. We landed a few minor PRs and improvements to the parser and oli-obk and tif have been working on modeling of effects for the const generics work. I'm planning to start digging more into the modeling of coherence now that the basic merging is done.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/113#issuecomment-2688118450">Comment by @lcnr posted on 2025-02-27:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We've stabilized `-Znext-solver=coherence` [in version 1.84](https://blog.rust-lang.org/2025/01/09/Rust-1.84.0.html#migration-to-the-new-trait-solver-begins) and started to track the remaining issues in [a project board](https://github.com/orgs/rust-lang/projects/61).

Fixing the opaque types issue breaking `wg-grammar` is difficult and requires a more in-depth change for which there is now an [accepted Types MCP](https://github.com/rust-lang/types-team/issues/129). This likely also unblocks the TAIT stabilization using the old solver.

While waiting on the MCP I've started to look into the errors when compiling `nalgebra`. @lqd minimized the failures. They have been caused by insufficiencies in our cycle handling. With https://github.com/rust-lang/rust/pull/136824 and https://github.com/rust-lang/rust/pull/137314 cycles should now be fully supported.

We also fully triaged all UI tests with the new solver enabled with @compiler-errors, @BoxyUwU, and myself fixing multiple less involved issues.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/261#issuecomment-2681824358">Comment by @veluca93 posted on 2025-02-25:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments: Further discussions on implementation details of the three major proposed ways forward. Requested a design meeting in https://github.com/rust-lang/lang-team/issues/309.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/262#issuecomment-2667943769">Comment by @1c3t3a posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Status update: The null-checks landed in [rust#134424](https://github.com/rust-lang/rust/pull/134424). Next up are the enum discriminant checks.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/114#issuecomment-2684969001">Comment by @blyxyas posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

As a monthly update (previously posted on Zulip): We have some big progress!

- [rust-clippy#13821](https://github.com/rust-lang/rust-clippy/issues/13821) has been open and is currently being reviewed. It moves the MSRV logic out of lint-individualistic attribute extraction and into Clippy-wide MSRV (with a very good optimization, taking into account that only a small percentage of crates use.

- A Clippy-exclusive benchmarker has arrived, powered by the existing lintcheck infrastructure and perf. So it's compatible with flamegraph and other such tools. [rust-clippy#14194](https://github.com/rust-lang/rust-clippy/issues/14194) We can later expand this into CI or a dedicated bot.
- As you probably know, [rust#125116](https://github.com/rust-lang/rust/issues/125116) has been merged, just as a reminder of how that big goal was slayed like a dragon :dragon:.

We now know what functions to optimize (or, at least have a basic knowledge of where Clippy spends its time).
As some future functionality, I'd love to have a functionality to build cargo and rust with debug symbols and hook it up to Clippy, but that may be harder. It's not perfect, but it's a good start!

![clippy benchmark perf.data report](https://rust-lang.zulipchat.com/user_uploads/4715/1Ph9uNvqBfGDLGC55UXUhAw1/2025-02-17-000451_1918x582_scrot.png)

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


<!-- this comment helps to convince the markdown parser to do the right thing -->

<details>
<summary>No detailed updates available.</summary>
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

<a href="https://github.com/rust-lang/rust-project-goals/issues/265#issuecomment-2679552595">Comment by @JoelMarcey posted on 2025-02-24:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Last week at the Safety Critical Rust Consortium meeting in London, Ferrous systems publicly announced to consortium members that they have committed to contributing the FLS to the Rust Project. We are finalizing the details of that process, but we can start FLS integration testing in parallel, in anticipation.

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


<details>
<summary>2 detailed updates availabled.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/267#issuecomment-2690860706">Comment by @m-ou-se posted on 2025-02-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

We started the collaboration with the Delft University of Technology. We assembled a small research team with a professor and a MSc student who will be working on this as part of his MSc thesis. We meet weekly in person.

The project kicked off two weeks ago and is now in the literature research phase.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/267#issuecomment-2690864244">Comment by @m-ou-se posted on 2025-02-28:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

And related to this, someone else is working on an implementation of my `#[export]` rfc: https://github.com/rust-lang/rust/pull/134767 This will hopefully provide meaningful input for the research project.

<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/268'><strong>Run the 2025H1 project goal program</strong></a></div>
    <div style="flex: initial;"><progress value="6" max="13"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/268#issuecomment-2668673602">Comment by @nikomatsakis posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update: We are running behind schedule, but we are up and running nonetheless! Bear with us. The [goals RFC](https://github.com/rust-lang/rfcs/pull/3764) finished [FCP](https://github.com/rust-lang/rfcs/pull/3764#issuecomment-2645799451) on Feb 18. The new project goals team has been approved and we've updated the tracking issues for the new milestone.

Project goal owners are encouraged to update the issue body to reflect the actual tasks as they go with github checkboxes or other notation [as described here](https://rust-lang.github.io/rust-project-goals/how_to/report_status.html#updating-the-progress-bar). We're going to start pinging for our first status update soon!


<!-- this comment helps to convince the markdown parser to do the right thing -->

</blockquote>

</details>


<div style="display: flex;" class="mt2 mb3">
    <div style="flex: auto;"><a href='https://github.com/rust-lang/rust-project-goals/issues/269'><strong>Rust Vision Document</strong></a></div>
    <div style="flex: initial;"><progress value="0" max="4"></progress>
</div>
</div>
<!-- markdown separator --> 



<!-- markdown separator --> 


<details>
<summary>1 detailed update available.</summary>

<!-- this comment helps to convince the markdown parser to do the right thing -->


<!-- this comment helps to convince the markdown parser to do the right thing -->

<a href="https://github.com/rust-lang/rust-project-goals/issues/269#issuecomment-2669078146">Comment by @nikomatsakis posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Update so far: I put out a call for volunteers on Zulip ([hackmd](https://hackmd.io/@nikomatsakis/HJQOpqRwke)) and a number of folks responded. We had an initial meeting on Jan 30 ([notes here](https://hackmd.io/@nikomatsakis/H11FxVFOyx)). We have created a Zulip stream for this project goal (`vision-doc-2025`) and I also did some experimenting in the https://github.com/nikomatsakis/farsight repository for what the table of contents might look like. 

Next milestone is to layout a plan. We are somewhat behind schedule but not impossibly so! 

![Believe!](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExMzIxOWQ3bW5lb25obW04Y2lnMDkxa2Z0ZmZwcWw0N2h6cHU5ZWY2ZSZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/IazdAV1zjaGbBaGPgF/giphy.gif)

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/275#issuecomment-2668399213">Comment by @davidtwco posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

An initial update on what we've been up to and some background:

- This goal is submitted on behalf of the Rust team at Arm, but primarily worked on by @Jamesbarford. Anyone interested can always contact me for updates and I'll keep this issue up-to-date.
- We've scheduled a regular call with @Kobzol to discuss the constraints and requirements of any changes to rust-perf (see [the t-infra calendar](https://github.com/rust-lang/calendar)) and have drafted a document describing a proposed high-level architecture for the service following our changes.
  - This has been shared in the [#project-goals/2025h1/rustc-perf-improvements Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/478771-project-goals.2F2025h1.2Frustc-perf-improvements/topic/architecture.20proposal/near/499201831) to collect feedback.
  - Once we've reached an agreement on the high-level architecture, we'll prepare a more detailed plan with details like proposed changes to the database schema, before proceeding with the implementation.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/118#issuecomment-2627779557">Comment by @lqd posted on 2025-01-31:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

Key developments from this month:
- @amandasystems has continued working on the Sisyphean https://github.com/rust-lang/rust/pull/130227 and has made progress on rewriting type tests, diagnostics issues, fixing bugs, kept up with changes on master, and more. 
- big thanks to @jackh726 and @matthewjasper on reviews: with their help, all the PRs from the previous update have landed on nightly.
- I've opened a couple of PRs on the analysis itself (https://github.com/rust-lang/rust/pull/135290, https://github.com/rust-lang/rust/pull/136299) as well as a few cleanups. With these, there are only around 4 failing tests that still need investigation, and 8-10 diagnostics differences to iron out. This is my current focus, but we'll also need to expand test coverage.
- I've also opened a handful of PRs gradually expanding the polonius MIR dump with visualizations. I'll next add the interactive "tool" I've been using to help debug the test failures.
- on organization and collaboration:
    - we've met with one of Amanda's students for a possible Master's thesis on the more engineer-y side of polonius (perf <3)
    - and have also discussed, with @ralfjung's team, the related topic of modeling the borrowck in [`a-mir-formality`](https://github.com/rust-lang/a-mir-formality/)

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/270#issuecomment-2668362595">Comment by @davidtwco posted on 2025-02-19:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

An initial update on what we've been up to and some background:

- This goal is submitted on behalf of the Rust team at Arm, but primarily worked on by myself (@davidtwco) and @JamieCunliffe. Anyone interested can always contact me for updates and I'll keep this issue up-to-date.
- @JamieCunliffe been working on supporting Arm's scalable vector extension (SVE) for a couple years - primarily in rust-lang/rfcs#3268 and its implementation rust-lang/rust#118917.
  - Through this work, we've discovered other changes to the language necessary to be able to support these types without special cases in the type system, which we're also working on (see below).
  - Jamie is still resolving feedback on this RFC and its implementation, and keeping it rebased. We hope that it can be landed experimentally now that there's a feasible path to remove the special cases in the type system (see below).
  - **The next steps for this RFC and implementation are..**
    - ..to continue to respond to feedback on the RFC and implementation.
- I've (@davidtwco) been working on rust-lang/rfcs#3729 which improves Rust's support for exotically sized types, and would allow scalable vectors to be represented in the type system without special cases.
  - We've had two design meetings with the language team about the RFC and had a broadly positive reception.
    - There is a non-strict dependency on const traits (rust-lang/rfcs#3762) which has created uncertainty as to whether this RFC could be accepted without the specifics of const traits being nailed down.
  - I've been working on [implementing the RFC](https://github.com/davidtwco/rust/activity?ref=sized-hierarchy): an initial implementation of the non-const traits has been completed and adding the const traits is in-progress. 
    - The language team have indicated interest in seeing this land experimentally, but this will depend on whether the implementors of const traits are okay with this, as it would add to the work they need to do to make any syntactic changes requested by the language team in rust-lang/rfcs#3762.
  - **I'm continuing to respond to feedback on the RFC, but as this has largely trailed off, the next steps for this RFC are**..
    -  ..for the language team to decide to accept, reject, or request further changes to the RFC.
    -  ..for progress on the implementation to continue.

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

<a href="https://github.com/rust-lang/rust-project-goals/issues/273#issuecomment-2684902356">Comment by @jswrenn posted on 2025-02-26:</a><br>

<blockquote>

<!-- this comment helps to convince the markdown parser to do the right thing -->

**Key developments:** In a [Feb 19 Lang Team Design Meeting](https://hackmd.io/@jswrenn/Bkp8l4BtJg), we reached consensus that the MVP for unsafe fields should be limited to additive invariants. 

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
