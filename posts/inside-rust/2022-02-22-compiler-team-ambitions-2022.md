---
layout: post
title: Rust Compiler Ambitions for 2022
author: Felix Klock, Wesley Wiser
description: "The compiler team's concrete initiatives and hopeful aspirations for this year."
team: The Compiler Team <https://www.rust-lang.org/governance/teams/compiler>
---

# Rust Compiler Ambitions for 2022

Some people have been wondering about what the Rust Compiler Team has planned for 2022. This note is to let you all know what activities the team plans to focus on this year.

This document is structured into three parts: our [Overall Themes][] for this year, the [Concrete Initiatives][] we have resources to drive, and [Aspirations][] for what we could do if given more help.

[Overall Themes]: #Overall-Themes
[Concrete Initiatives]: #Concrete-Initiatives
[Aspirations]: #Aspirations

## Introduction

Part of the motivation for this note is to encourage new contributors to get involved. We have a lot of newcomers, from individuals to large organizations, who are very excited about Rust's potential, and we want to show all of them what they can do to help.

This is a list of items, divided into a [Concrete Initiatives][] section and an [Aspirations][] section. We accumulated these items during discussions with the Compiler Team and the Compiler Contributors.
The [Concrete Initiatives][] have owners assigned; each has  allocated time this year to attack the problem. The [Aspirations][], on the other hand, are items that the team agrees would be great areas for investment, but where we currently lack sufficient resources or experienced developers to make progress this year. 

This is *not* a list of everything we will do this year; at least, not without help.

You can think of the [Aspirations][] part of the doc as an explicit call to arms: If you see something there that interests you, please reach out to the owners listed in that section to find out how you might be able to help.

As you read the document, it is useful to keep in mind that [Rust is not a company][mara-post]: The teams, and the leaders of the teams, do not establish goals in a top-down manner, nor do they hand out tasks in a round-robin fashion. Instead, we collectively (and iteratively) refine our a shared vision for the future, and take steps that hopefully move towards that future. Each contributor decides for themself how much time they can afford to contribute, and that can vary wildly between contributors. The goals that we set for the project must be aligned with the goals of our current and future contributors; otherwise, they just won't get done. We have processes (e.g. [RFCs](https://github.com/rust-lang/rfcs#readme), [MCPs](https://forge.rust-lang.org/compiler/mcp.html)) that try to ensure alignment; in some ways, a document like this one is just another tool for recalibrating alignment.

<!-- 
But the flip side of this is: if something really is important, then there almost certainly exists a contributor willing to work on it. The real hurdle then is *enabling* that contributor to succeed.
(Note: this is hard! Its not just about mentorship/education; its just as much about achieving *alignment* amongst the whole group!)
-->

[mara-post]: https://blog.m-ou.se/rust-is-not-a-company/

[antoyo]: https://github.com/antoyo
<!-- antoyo sponsorship: https://github.com/sponsors/antoyo -->
[Aaron Hill]: https://github.com/Aaron1011
<!-- Aaron1011: no affiliation -->
[bjorn3]: https://github.com/bjorn3
<!-- bjorn3 donation page: https://liberapay.com/bjorn3 -->
[cjgillot]: https://github.com/cjgillot
<!-- no response from cjgillot re affiliation yet -->
[davidtwco]: https://github.com/davidtwco
<!-- davidtwco affiliation: "Huawei R&D UK"-->
[estebank]: https://github.com/estebank
<!-- estebank affiliation: AWS --> 
[lcnr]: https://github.com/lcnr
<!-- lcnr sponsorship: https://lcnr.de/funding/ -->
[mw]: https://github.com/michaelwoerister
<!-- mw affiliation: MS -->
[nikomatsakis]: https://github.com/nikomatsakis
<!-- nikomatsakis affiliation: AWS -->
[oli-obk]: https://github.com/oli-obk
<!-- oli affiliation: AWS --> 
[jackh726]: https://github.com/jackh726
<!-- jackh726: no affiliation -->
[lqd]: https://github.com/lqd
<!-- lqd affiliation: ISRG -->
[nnethercote]: https://github.com/nnethercote
<!-- nnethercote affiliation: Futurewei -->
[tmandry]: https://github.com/tmandry
<!-- tmandry affiliation: Google (TBD) -->
[scottmcm]: https://github.com/scottmcm
<!-- scottmcm: no affiliation -->
[pnkfelix]: https://github.com/pnkfelix
<!-- pnkfelix affiliation: AWS -->
[wesleywiser]: https://github.com/wesleywiser
<!-- wesleywiser affiliation: MS -->
[jswrenn]: https://github.com/jswrenn
<!-- jswrenn affiliation: AWS -->
[apiraino]: https://github.com/apiraino
<!-- apiraino: no affiliation -->
[simulacrum]: https://github.com/Mark-simulacrum
<!-- simulacrum sponsorship: https://github.com/sponsors/Mark-Simulacrum -->
[rylev]: https://github.com/rylev
<!-- rylev affiliation: MS -->
[xldenis]: https://github.com/xldenis

## Overall Themes

### Fulfill Rust's Promise (🦀)

Fulfilling Rust's Promise is a cross-cutting theme; it means identifying the gaps between expectation and reality for each of our three pillars: [Performance, Reliability, and Productivity][rust-lang], and then addressing those gaps.


[rust-lang]: https://www.rust-lang.org

### Developer Delight (👩‍💻)

We have opportunities to improve the experience of writing, of compiling, and of running Rust code. We want answers to the question, "what would delight Rust developers?" This is not about meeting their expectations: It's about *surpassing* them.


[Debugging concrete]: #Debugging-Initiatives
[Debugging aspirational]: #Debugging-Aspirations
[Debugging]: #Debugging-Object-Code
[Faster concrete]: #Compilation-Time-Initiatives
[Faster aspirational]: #Faster-Compilation-Aspirations
[Expressiveness concrete]: #Language-Expressiveness-Initiatives
[Expressiveness aspirational]: #Expressiveness-Aspirations

### Contributor Workflow (🛠️)

Finally, improving the Compiler Contributor Workflow means technology enhancements that benefit people maintaining and extending the Rust compiler itself.

(We also make non-technical enhancements, such as changes to our social processes, but this document focuses on technology.)


[Librarification Initiatives]: #Librarification-Initiatives
[Librarification Aspirations]: #Librarification-Aspirations

[Compiler Backend]: #Compiler-Backend
[Review Queue Crew]: #Review-Queue-Crew

Category  | [Concrete Initiatives] |  [Aspirations]
----------|---------------------|-----------
I-unsound | [Initiatives][I-unsound Issues]      |
Async Rust| [Initiatives][Async Initiatives]     |
Debugging | [Initiatives][Debugging Initiatives] | [Aspirations][Debugging Aspirations]
Faster Builds | [Initiatives][Faster Builds Initiatives] | [Aspirations][Faster Builds Aspirations]
Expressiveness     | [Initiatives][Expressiveness Initiatives] | [Aspirations][Expressiveness Aspirations]
Librarification    | [Initiatives][Librarification Initiatives]                 | [Aspirations][Librarification Aspirations]
Team Operations    |                             | [Aspirations][Team Operations]
Backend            |                             | [Aspirations][Backend Aspirations]
Diagnostics        |                             | [Aspirations][Diagnostics Aspirations]

[Concrete Initiatives]: #concrete-initiatives
[I-unsound Issues]: #i-unsound-issues-
[Async Initiatives]: #async-rust-initiatives--
[Debugging Initiatives]: #debugging-initiatives-
[Faster Builds Initiatives]: #faster-builds-initiatives--
[Expressiveness Initiatives]: #expressiveness-initatives--
[Librarification Initiatives]: #librarification-initiatives-
[Aspirations]: #aspirations
[P-high Aspirations]: #p-high-aspirations-
[Debugging Aspirations]: #debugging-aspirations-
[Faster Builds Aspirations]: #faster-builds-aspirations--
[Expressiveness Aspirations]: #expressiveness-aspirations--
[Librarification Aspirations]: #librarification-aspirations-%EF%B8%8F
[Team Operations]: #compiler-team-operations-aspirations-%EF%B8%8F
[Backend Aspirations]: #compiler-backend-aspirations-%EF%B8%8F-
[Diagnostics Aspirations]: #diagnostics-aspirations-

<!-- end of manually made [toc] -->

## Concrete Initiatives

This section is the closest thing to a "roadmap" we have for 2022. It is a list of important items with dedicated owners that have time allocated to make significant progress on the problem this year. 

### I-unsound issues (🦀)

As of this writing, we have 69 [open issues tagged I-unsound](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-unsound), and 44 of those are [also tagged T-compiler](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AI-unsound++label%3AT-compiler).


In theory, any unsoundness issue potentially undermines Rust's promise of reliability. We want, by the end of this year, to have a clear understanding of how each of those I-unsound issues came to be. We are looking into systematically detecting such issues and whether we can deploy mitigations or fixes for entire classes of issues, instead of addressing them on a case by case basis.

[oli-obk], from Amazon Web Services, will be the primary owner of  work in this space. Please reach out to [oli-obk] and [pnkfelix] (also from Amazon Web Services) if you are interested in helping resolve these issues!


### Async Rust Initiatives (🦀, 👩‍💻)

There is significant overlap between async rust and other areas of this document, such as debugging and language expressiveness.

#### async traits

Rust today does not allow `async fn` in a trait, so Async Rust code usually ends up with components that are too tightly coupled; one cannot write reusable, general-purpose libraries without using workarounds like `#[async_trait]` that impose hidden costs. [nikomatsakis], from Amazon Web Services, and [tmandry], from Google, are driving the [async fn in traits initiative](https://github.com/rust-lang/async-fundamentals-initiative/issues/5), which will unlock the ability to write `async` methods in traits, natively.

#### async crashdump dissection

[mw], from Microsoft, is driving the [async crashdump initiative](https://rust-lang.github.io/async-crashdump-debugging-initiative/), which will enable developers to understand the control-flow stacks encoded in crashdumps for their async Rust programs.

There is a ton of other work being done in the Async Rust space. Check out the [Async Vision web site](https://rust-lang.github.io/wg-async/welcome.html) for more information.


### Debugging Initiatives (🦀)

[wesleywiser], from Microsoft, and [pnkfelix] are spinning up a wg-debugging working group. It will cover at least the following sub-items: improving Rust's debuginfo quality ([mw], [wesleywiser]), supporting split debuginfo ([davidtwco], from Huawei R&D UK), and better integration with trace-based debuggers like `rr` ([pnkfelix]).

The immediate goals for this initiative: establish the working group, determine priorities for the backlog of debugging issues, and find out what active users of debuggers miss most when they operate on Rust code.

### Faster Builds Initiatives (👩‍💻, 🛠️)
 
The Rust compiler's end-to-end latency is known to be a problem.
 
[lqd], sponsored by the Internet Security Research Group, is dedicating the majority of 2022 to working on this, partnering with Rust's compiler-performance working group as well as performance experts like [nnethercote] (from Futurewei Technologies). [lqd] has their own [living document](https://hackmd.io/3Dp68rTDSpWvRDfWF6lbMw?view) that lists areas under investigation, and [nnethercote] has a [roadmap under development](https://hackmd.io/YJQSj_nLSZWl2sbI84R1qA).

[ISRG]: https://www.abetterinternet.org/

### Expressiveness Initiatives (👩‍💻, 🦀)

A common refrain we hear is: "I need feature X, but it's not implemented in rustc or stable." 
In Rust, we use an open Request-for-Comment (RFC) process for designing new features. Currently, we have [this set of RFCs approved][RFC tracking issue list]; here are some imporant features with dedicated owners that we expect forward movement on.

[RFC tracking issue list]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AC-tracking-issue++label%3AB-RFC-approved+

Generic Associated Types, or [GATs](https://github.com/rust-lang/generic-associated-types-initiative/issues/4), are an ongoing effort owned by [jackh726]. GATs have many applications, such as traits whose associated types have lifetimes tied to the local borrowing of the receiver type ([e.g. `StreamingIterator`][GAT-motivation]).

[GAT-motivation]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md#motivation

[`async fn` in traits](https://github.com/rust-lang/async-fundamentals-initiative/issues/5) is an ongoing effort (already mentioned above) owned by [tmandry]. This is one of the most frequently requested features for async rust: supplying first class support for traits like `trait Foo { async fn bar(&self); }`

The [safe transmute](https://github.com/rust-lang/lang-team/issues/21) project, led by [jswrenn] from Amazon Web Services, is expected to be feature-complete in summer 2022. It will enable a large class of types to be transmuted (i.e. zero-cost type conversion) without any risk of injecting undefined behavior.

### Librarification Initiatives (🛠️)

These are initiatives dedicated to the "librarification" of the compiler: breaking the monolithic code base of `rustc` into a set of decoupled parts that can be independently developed, and, ideally, repurposed for other kinds of tools besides `rustc` such as `rust-analyzer`.

#### Chalk

[Chalk] is a reimplementation of Rust's trait system using declarative logic rules, a la Prolog.

[Chalk]: https://github.com/rust-lang/chalk

Chalk has been years in development, and has been experimentally integrated into rustc in the past. This year, [jackh726] and [nikomatsakis] own the task of improving the chalk integration, to drive it to the point where the team can consider migrating to chalk as the implementation of the trait system. This will unlock many features that up until now have been too difficult to implement in the old trait system implementation, and its declarative structure will provide a proper foundation for people to reason about the *correctness* of the trait system.

If you want to help out with this, reach out to [jackh726] and [nikomatsakis].

## Aspirations

We would love help with any of the areas listed on this document, but this section specifically lists areas where we know we lack resources today.

If you are interested in helping with any items here, please do reach out to the owner listed; they will be thrilled to talk to you.

### P-high Aspirations (🦀)

[pnkfelix] and [wesleywiser], as team leads, are deploying processes to help us get a handle on the "high priority, but *not critical*" issues that the compiler has accumulated. We will be gradually identifying owners for each who will move progress forward, and in general working to keep better track of the set overall.

If you would like to help with the task of reviewing or resolving such issues, reach out to [wesleywiser] and [apiraino], who are co-leads of WG-prioritization.

### Debugging Aspirations (👩‍💻)

We want better integration, at least with the popular debuggers. The command sequence to set up an idealized debugging experience is too obscure and therefore goes unused.

We want to improve expression evaluation support: Today, most forms of method invocation do not work, because the debuggers do not know about Rust's method resolution rules.

We want to revisit our debugger extension architecture for rendering Rust data structures, which is currently mostly independent sets of Python scripts.

If you want to help out here, please reach out to [pnkfelix] and [wesleywiser].

### Faster Builds Aspirations (👩‍💻, 🛠️)

#### Parallel Compilation

Parallel Compilation is one avenue for improving compiler performance. It is also a very complex area, especially when it comes to the tradeoff of how much of a hit one is willing to take on single core builds in order to enable more parallel computation. This is an area we think needs long-term collaborative effort with the compiler team. We do not expect to deliver a solution here this year.

If you want to discuss more with us about past attempts and ideas for the future, please reach out to [pnkfelix] and [wesleywiser]. 

#### Incremental Compilation Aspirations

Incremental compilation performance and stability are both ongoing concerns to the team. We *know* there is significant room to improve the effectiveness of incremental compilation, in terms of reducing the amount of redundant work done by successive `rustc` invocations. 

In addition, there is a significant amount of work that could be done to improve our testing infrastructure for incremental compiliation which does not require deep knowledge of the compiler. 

If you want to learn more, reach out to [cjgillot] and [Aaron Hill].

#### Inter-crate Sharing Aspirations

nnethercote has noted that there may be opportunities
to improve end-to-end compilation time for multi-crate builds by identifying redundant activity that can be shared between builds of distinct crates. (For example, the metadata from libstd is read and decoded on every single crate compile.)

If you are interested in exploring this idea further, reach out to [nnethercote] and [lqd].

### Expressiveness Aspirations (🦀, 👩‍💻)

const generics and const eval are making steady progress. There are a *lot* of feature flags, which implies there's a lot of knobs that could be turned on and off.

What we can probably use the most help with is in identifying what subset of the features we should be striving to stabilize in order to unlock specific use cases for Rust developers.

So, if you or your team is enthuastically awaiting const generics or const eval, reach out to [lcnr] (supported via [sponsorship][sponsor-lcnr]) and [oli-obk].

[sponsor-lcnr]: https://lcnr.de/funding/

### Librarification Aspirations (🛠️)

#### MIR tooling

Various stakeholders, especially in the formal methods space, are making extensions to Rust that are based on analyzing MIR, the intermediate representation used by the compiler. Should we be trying to stabilize that as an interop format of some kind?

For example, [Kani] is a bit-precise model-checker for Rust under development at Amazon Web Services. It is implemented as another backend on `rustc`; but it would be cleaner if rustc could just generate MIR and their compiler could consume MIR. [Prusti] and [Creusot] could likewise benefit from a stable MIR interop.

[Kani]: https://github.com/model-checking/kani
[Prusti]: https://github.com/viperproject/prusti-dev#prusti
[Creusot]: https://github.com/xldenis/creusot#about

Reach out to [xldenis], from the LMF at the University of Paris-Saclay (and co-lead of the Rust Formal Methods working group), and [pnkfelix] if you are interested in helping us here.

### Compiler Team Operations Aspirations (🛠️)

#### MCVE reduction tooling

One common task for compiler developers is to create a [minimal complete verifiable example][E-needs-mcve]. This task is largely mechanical; pnkfelix has a [blog post][mcve blog post] about Rust source-to-source tranformations that accomplish this. But despite its mechanical nature, the current state of the art in automating this task is in tools like [creduce](https://github.com/csmith-project/creduce), which have some big limitations (such as only working on a single file at a time).

This is an area where you do not need any knowledge of the `rustc` source code at all. Anyone with an interest in programming language technology can get involved; e.g. one might consider adding IDE commands for certain code reducing transformations.

If you are interested in helping in this area, please reach out to [pnkfelix].

[E-needs-mcve]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AE-needs-mcve+
[mcve blog post]: https://blog.pnkfx.org/blog/2019/11/18/rust-bug-minimization-patterns/

#### Performance Dashboard

[perf.rust-lang.org][perf] is a dashboard that measures the performance of `rustc`, in terms of resources (time and memory) consumed during compilation. [@rust-timer] is a bot that summarizes whether a given Pull Request regressed or improved performance. 

The performance working group has many ideas for things to improve in these tools, but limited resources. This is an area where you don't need any compiler expertise to make a huge impact; for example, our Web Front-end could use work. And Data Scientists might have useful insights into our problems. Beyond just measuring the compiler's own performance, we're also interested in measuring the runtime performance of produced binaries.

Reach out to [rylev], from Microsoft, and [simulacrum] (supported via [sponsorship](https://github.com/sponsors/Mark-Simulacrum)), performance working group lead, if you want to help.

[@rust-timer]: https://github.com/rust-timer
[perf]: https://perf.rust-lang.org/

### Compiler Backend Aspirations (🛠️, 👩‍💻)

#### Ease writing new backends

One source of tedium when defining a new Rust compiler backend is implementing the intrinsics that each backend must provide. But a small change to the intrinsic system: namely, allowing intrinsics to define a [fallback MIR implementation][], could ease that burden. Reach out to [scottmcm] if you are interested in helping out here.

[fallback MIR implementation]: https://github.com/rust-lang/rust/issues/93145

#### Cranelift

The [Cranelift Code Generator][Cranelift] is getting a lot of attention from various parties. rustc has a [Cranelift backend][]. If you are interested in helping out with it, reach out to [bjorn3] (supported via [sponsorship][sponsor-bjorn3]).

[sponsor-bjorn3]: https://liberapay.com/bjorn3

[Cranelift]: https://github.com/bytecodealliance/wasmtime/tree/main/cranelift
[Cranelift backend]: https://github.com/bjorn3/rustc_codegen_cranelift

#### GCC backend

In addition to the LLVM and Cranelift backends, there is also a new backend under development that uses `libgccjit` from GCC (which, as many have clarified, is usable for ahead-of-time as well as just-in-time compilation). This backend enables Rust to target more platforms that are not supported by LLVM.

If you are interested in helping out with this project, reach out to [antoyo] (supported via [sponsorship](https://github.com/sponsors/antoyo)) and [bjorn3].


### Diagnostics Aspirations (👩‍💻)

The Rust compiler has pretty good diagnotics. But the good news is, there's a [full employment theorem](https://en.wikipedia.org/wiki/Full_employment_theorem) for diagnostics engineers which is supported by the 1,500+ [open diagnostics issues](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-diagnostics) we have.

Diagnostics improvements are an *excellent* first step for learning about how to contribute to the Rust compiler. If you're interested in helping out but don't have any idea where to start, fixing diagnostic bugs is a great jumping off point, and you can reach out to [estebank], at Amazon Web Services, to find out more about how to help.


## Conclusion

Reading over this list, the number of items on it seems quite daunting.

Do we really think we can get all this stuff done in one year?

No, we don't! :laughing:

The introduction explicitly said the latter half are things that *don't* have resources attached to them. And the word "aspiration" was chosen to reinforce that.

This is where you all, the Rust community (including *future members* of that community) come into the picture. Each item has one or two people listed with it; if you're feeling inspired, please do contact us!

## FAQ

#### How can I learn about progress on all this stuff? Will we see another post like this soon?

The Rust project constantly  experiments with different ways to track progress on its on-going initiatives. We do not yet have a single place that summarizes the status of everything, though there is some effort towards making better use of Github Projects for this; see e.g. what the lang team is doing with its [initiatives](https://github.com/orgs/rust-lang/projects/16).

The compiler team leadership plans to put out a post in June summarizing the progress so far on the items listed here, and another post in November with a retrospective on how the year went.

#### I did not see any mention of monadic burritos (or other non-Rust language feature); why is that not part of your plan?

The scope of this doc is largely restricted to Compiler Team issues. Language design work is done by the Language Design team. You can reach out to them about their initiatives for this year and beyond.

#### What do I do if I'm interested in learning more about a specific item on this list?

Each item in this list has one or more owners listed with it. The Rust Compiler team largely communicates via the [Zulip] chat platform.

So: set up a Zulip account, sign into it, and then send a private message to the relevant owners, telling them that you want to help with their rustc initiatives. They'll help you get started from there.

[Rustc Dev Guide]: https://rustc-dev-guide.rust-lang.org/
[Zulip]: https://rust-lang.zulipchat.com/ 

#### What do I do if I'm interested in compiler development but have no experience in compilers?

This is not a problem! Many members of our community learned about compilers by working on rustc, and we encourage others to do so as well. You can start by reading the [Rustc Dev Guide] and by joining us on [Zulip]. You may also benefit from watching the RustConf 2021 presentation on [Contributing to the Compiler] by [estebank].

[Contributing to the Compiler]: https://www.youtube.com/watch?v=vCODCbUSA_w

In addition, there are areas in this project where people without compiler expertise can have impact. For example, as mentioned in the [Performance Dashboard](#Performance-Dashboard) section, some of our internal tools could use some web front-end work.






