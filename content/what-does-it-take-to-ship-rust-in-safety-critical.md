+++
path = "2026/01/12/what-does-it-take-to-ship-rust-in-safety-critical"
title = "What does it take to ship Rust in safety-critical?"
authors = ["Pete LeVasseur"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-project-vision-doc-2025"
+++

_This is another post in our series covering what we learned through the Vision Doc process. In [our first post](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/), we described the overall approach and what we learned about doing user research. In [our second post](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/), we explored what people love about Rust. This post goes deep on one domain: safety-critical software._

When we set out on the Vision Doc work, one area we wanted to explore in depth was safety-critical systems: software where malfunction can result in injury, loss of life, or environmental harm. Think vehicles, airplanes, medical devices, industrial automation. We spoke with engineers at OEMs, integrators, and suppliers across automotive (mostly), industrial, aerospace, and medical contexts.

What we found surprised us a bit. The conversations kept circling back to a single tension: Rust's compiler-enforced guarantees support much of what Functional Safety Engineers and Software Engineers in these spaces spend their time preventing, but once you move beyond prototyping into the higher-criticality parts of a system, the ecosystem support thins out fast. There is no MATLAB/Simulink Rust code generation. There is no OSEK or AUTOSAR Classic-compatible RTOS written in Rust or with first-class Rust support. The tooling for qualification and certification is still maturing.

## Quick context: what makes software "safety-critical"

If you've never worked in these spaces, here's the short version. Each safety-critical domain has standards that define a ladder of integrity levels: ISO 26262 in automotive, IEC 61508 in industrial, IEC 62304 in medical devices, DO-178C in aerospace. The details differ, but the shape is similar: as you climb the ladder toward higher criticality, the demands on your development process, verification, and evidence all increase, and so do the costs.[^costs]

This creates a strong incentive for *decomposition*: isolate the highest-criticality logic into the smallest surface area you can, and keep everything else at lower levels where costs are more manageable and you can move faster.

We'll use automotive terminology in this post (QM through ASIL D, in increasing safety-criticality) since that's where most of our interviews came from, but the patterns generalize. The story at low criticality looks very different from the story at high criticality, regardless of domain.

## Rust is already in production for safety-critical systems

Before diving into the challenges, it is worth noting that Rust is not just being evaluated in these domains. It is deployed and running in production.

We spoke with a principal firmware engineer working on mobile robotics systems certified to IEC 61508 SIL 2:

> "We had a new project coming up that involved a safety system. And in the past, we'd always done these projects in C using third party stack analysis and unit testing tools that were just generally never very good, but you had to do them as part of the safety rating standards. Rust presented an opportunity where 90% of what the stack analysis stuff had to check for is just done by the compiler. That combined with the fact that now we had a safety qualified compiler to point to was kind of a breakthrough." -- Principal Firmware Engineer (mobile robotics)

We also spoke with an engineer at a medical device company deploying IEC 62304 Class B software to intensive care units:

> "All of the product code that we deploy to end users and customers is currently in Rust. We do EEG analysis with our software and that's being deployed to ICUs, intensive care units, and patient monitors." -- Rust developer at a medical device company

> "We changed from this Python component to a Rust component and I think that gave us a 100-fold speed increase." -- Rust developer at a medical device company

These are not proofs of concept. They are shipping systems in regulated environments, going through audits and certification processes. The path is there. The question is how to make it easier for the next teams coming through.

## Rust adoption is easiest at QM, and the constraints sharpen fast

At low criticality, teams described a pragmatic approach: use Rust and the crates ecosystem to move quickly, then harden what you ship. One architect at an automotive OEM told us:

> "We can use any crate [from crates.io] [..] we have to take care to prepare the software components for production usage." -- Architect at Automotive OEM

But at higher levels, third-party dependencies become difficult to justify. Teams either rewrite, internalize, or strictly constrain what they use. An embedded systems engineer put it bluntly:

> "We tend not to use 3rd party dependencies or nursery crates [..] solutions become kludgier as you get lower in the stack." -- Firmware Engineer

Some teams described building escape hatches, abstraction layers designed for future replacement:

> "We create an interface that we'd eventually like to have to simplify replacement later on [..] sometimes rewrite, but even if re-using an existing crate we often change APIs, write more tests." -- Team Lead at Automotive Supplier (ASIL D target)

Even teams that do use crates from crates.io described treating that as a temporary accelerator, something to track carefully and remove from critical paths before shipping:

> "We use crates mainly for things in the beginning where we need to set up things fast, proof of concept, but we try to track those dependencies very explicitly and for the critical parts of the software try to get rid of them in the long run." -- Team lead at an automotive software company developing middleware in Rust

In aerospace, the "control the whole stack" instinct is even stronger:

> "In aerospace there's a notion of we must own all the code ourselves. We must have control of every single line of code." -- Engineering lead in aerospace

This is the first big takeaway: **a lot of "Rust in safety-critical" is not just about whether Rust compiles for a target. It is about whether teams can assemble an evidence-friendly software stack and keep it stable over long product lifetimes.**

## The compiler is doing work teams used to do elsewhere

Many interviewees framed Rust's value in terms of work shifted earlier and made more repeatable by the compiler. This is not just "nice," it changes how much manual review you can realistically afford. Much of what was historically process-based enforcement through coding standards like MISRA C and CERT C becomes a language-level concern in Rust, checked by the compiler rather than external static analysis or manual review.

> "Roughly 90% of what we used to check with external tools is built into Rust's compiler." -- Principal Firmware Engineer (mobile robotics)

We heard variations of this from teams dealing with large codebases and varied skill levels:

> "We cannot control the skill of developers from end to end. We have to check the code quality. Rust by checking at compile time, or Clippy tools, is very useful for our domain." -- Engineer at a major automaker

Even on smaller teams, the review load matters:

> "I usually tend to work on teams between five and eight. Even so, it's too much code. I feel confident moving faster, a certain class of flaws that you aren't worrying about." -- Embedded systems engineer (mobile robotics)

Closely related: people repeatedly highlighted Rust's consistency around error handling:

> "Having a single accepted way of handling errors used throughout the ecosystem is something that Rust did completely right." -- Automotive Technical Lead

For teams building products with 15-to-20-year lifetimes and "teams of teams," compiler-enforced invariants scale better than "we will just review harder."

## Teams want newer compilers, but also stability they can explain

A common pattern in safety-critical environments is conservative toolchain selection. But engineers pointed out a tension: older toolchains carry their own defect history.

> "[..] traditional wisdom is that after something's been around and gone through motions / testing then considered more stable and safer [..] older compilers used tend to have more bugs [and they become] hard to justify" -- Software Engineer at an Automotive supplier

Rust's edition system was described as a real advantage here, especially for incremental migration strategies that are common in automotive programs:

> "[The edition system is] golden for automotive, where incremental migration is essential." -- Software Engineer at major Automaker

In practice, "stability" is also about managing the mismatch between what the platform supports and what the ecosystem expects. Teams described pinning Rust versions, then fighting dependency drift:

> "We can pin the Rust toolchain, but because almost all crates are implemented for the latest versions, we have to downgrade. It's very time-consuming." -- Engineer at a major automaker

For safety-critical adoption, "stability" is operational. Teams need to answer questions like: What does a Rust upgrade change, and what does it not change? What are the bounds on migration work? How do we demonstrate we have managed upgrade risk?

## Target support matters in practical ways

Safety-critical software often runs on long-lived platforms and RTOSs. Even when "support exists," there can be caveats. Teams described friction around targets like QNX, where upstream Rust support exists but with limitations (for example, QNX 8.0 support is currently `no_std` only).[^qnx]

This connects to Rust's target tier policy: the policy itself is clear, but regulated teams still need to map "tier" to "what can I responsibly bet on for this platform and this product lifetime."

> "I had experiences where all of a sudden I was upgrading the compiler and my toolchain and dependencies didn't work anymore for the Tier 3 target we're using. That's simply not acceptable. If you want to invest in some technology, you want to have a certain reliability." -- Senior software engineer at a major automaker

## `core` is the spine, and it sets expectations

In `no_std` environments, `core` becomes the spine of Rust. Teams described it as both rich enough to build real products and small enough to audit.

A lot of Rust's safety leverage lives there: `Option` and `Result`, slices, iterators, `Cell` and `RefCell`, atomics, `MaybeUninit`, `Pin`. But we also heard a consistent shape of gaps: many embedded and safety-critical projects want `no_std`-friendly building blocks (fixed-size collections, queues) and predictable math primitives, but do not want to rely on "just any" third-party crate at higher integrity levels.

> "Most of the math library stuff is not in core, it's in std. Sin, cosine... the workaround for now has been the libm crate. It'd be nice if it was in core." -- Principal Firmware Engineer (mobile robotics)

## Async is appealing, but the long-run story is not settled

Some safety-critical-adjacent systems are already heavily asynchronous: daemons, middleware frameworks, event-driven architectures. That makes Rust's async story interesting.

But people also expressed uncertainty about ecosystem lock-in and what it would take to use async in higher-criticality components. One team lead developing middleware told us:

> "We're not sure how async will work out in the long-run [in Rust for safety-critical]. [..] A lot of our software is highly asynchronous and a lot of our daemons in the AUTOSAR Adaptive Platform world are basically following a reactor pattern. [..] [C++14] doesn't really support these concepts, so some of this is lack of familiarity." -- Team lead at an automotive software company developing middleware in Rust

And when teams look at async through an ISO 26262 lens, the runtime question shows up immediately:

> "If we want to make use of async Rust, of course you need some runtime which is providing this with all the quality artifacts and process artifacts for ISO 26262." -- Team lead at an automotive software company developing middleware in Rust

Async is not "just a language feature" in safety-critical contexts. It pulls in runtime choices, scheduling assumptions, and, at higher integrity levels, the question of what it would mean to certify or qualify the relevant parts of the stack.

## Recommendations

Rather than a generic wish list, here are concrete pressure points that came directly out of these conversations. Each is something we heard teams doing already, but in an ad hoc way.

**Turn "target tier policy" into a safety-critical onramp.** The friction we heard is not about the policy being unclear, it is about translating "tier" into practical decisions. A short, target-focused readiness checklist would help: Which targets exist? Which ones are `no_std` only? What is the last known tested OS version? What are the top blockers? The raw ingredients exist in rustc docs, release notes, and issue trackers, but pulling them together in one place would lower the barrier. This could be a collaboration between the compiler team, platform maintainers, and the Safety-Critical Rust Consortium.

**Document "dependency lifecycle" patterns teams are already using.** The QM story is often: use crates early, track carefully, shrink dependencies for higher-criticality parts. The ASIL B+ story is often: avoid third-party crates entirely, or use abstraction layers and plan to replace later. Turning those patterns into a reusable playbook would help new teams make the same moves with less trial and error. This seems like a natural fit for the Safety-Critical Rust Consortium's liaison work.

**For async, identify the artifact boundaries that matter.** Teams are looking for quality and process artifacts when they adopt an async runtime. A productive next step is writing down, at a technical level, what artifacts teams actually need and what subset of runtime behavior they want to constrain. Work is already happening in this space.[^score] The async working group and libs team could help work with safety-critical community folks to define what a "safety-case friendly" runtime would look like.

**Treat interop as part of the safety story.** Many teams are not going to rewrite their world in Rust. They are going to integrate Rust into existing C and C++ systems and carry that boundary for years. Guidance and tooling to keep interfaces correct, auditable, and in sync would help. The compiler team and lang team could consider how FFI boundaries are surfaced and checked.

> "We rely very heavily on FFI compatibility between C, C++, and Rust. In a safety-critical space, that's where the difficulty ends up being, generating bindings, finding out what the problem was." -- Embedded systems engineer (mobile robotics)

## Conclusion

To sum up the main points in this post:

- Rust is already deployed in production for safety-critical systems, including mobile robotics (IEC 61508 SIL 2) and medical devices (IEC 62304 Class B). The path exists.
- Rust's defaults (memory safety, thread safety, strong typing) map directly to much of what Functional Safety Engineers spend their time preventing. But ecosystem support thins out as you move toward higher-criticality software.
- At low criticality (QM), teams use crates freely and harden later. At higher levels (ASIL B+), third-party dependencies become difficult to justify, and teams rewrite, internalize, or build abstraction layers for future replacement.
- The compiler is doing work that used to require external tools and manual review. Much of what was historically process-based enforcement through standards like MISRA C and CERT C becomes a language-level concern, checked by the compiler. That can scale better than "review harder" for long-lived products with large teams and supports engineers in these domains feeling more secure in the systems they ship.
- Stability is operational: teams need to explain what upgrades change, manage dependency drift, and map target tier policies to their platform reality.
- Async is appealing for middleware and event-driven systems, but the runtime and qualification story is not settled for higher-criticality use.

We make four recommendations: create target-focused readiness checklists, document dependency lifecycle patterns, identify artifact boundaries for async runtimes, and treat C/C++ interop as part of the safety story.

## Get involved

If you're working in safety-critical Rust, or you want to help make it easier, check out the [Rust Foundation's Safety-Critical Rust Consortium](https://github.com/rustfoundation/safety-critical-rust-consortium) and the in-progress [Safety-Critical Rust coding guidelines](https://github.com/rustfoundation/safety-critical-rust-coding-guidelines).

Hearing concrete constraints, examples of assessor feedback, and what "evidence" actually looks like in practice is incredibly helpful. The goal is to make Rust's strengths more accessible in environments where correctness and safety are not optional.

[^costs]: If you're curious about how rigor scales with cost in ISO 26262, [this Feabhas guide](https://www.feabhas.com/sites/default/files/2016-06/A%20quick%20guide%20to%20ISO%2026262%5B1%5D_0_0.pdf) gives a good high-level overview.

[^qnx]: See the [QNX target documentation](https://doc.rust-lang.org/beta/rustc/platform-support/nto-qnx.html) for current status.

[^score]: Eclipse SDV's [Eclipse S-CORE](https://projects.eclipse.org/projects/automotive.score) project includes an Orchestrator written in Rust for their async runtime, aimed at safety-critical automotive software.
