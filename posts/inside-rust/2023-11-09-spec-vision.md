---
layout: post
title: "Our Vision for the Rust Specification"
author: Eric, Felix, Joel and Mara
team: the specification team <https://www.rust-lang.org/governance/teams/lang#Specification%20team>
---

A few months ago, by accepting [RFC 3355](https://rust-lang.github.io/rfcs/3355-rust-spec.html), the decision was made to start working on an official specification for the Rust language. Eric (maintainer of the Rust Reference), Felix (Rust language team), Joel (Rust Foundation) and Mara (author of the RFC) have been working together to get this effort started.

With this blog post, we present the steps we have taken so far and our plans for approaching the rest of this big task.

## Editor

One of the first steps was to fill the role of "editor" [as laid out in the RFC](https://rust-lang.github.io/rfcs/3355-rust-spec.html#role-of-the-editor). The responsibility of coordination and editing of the specification is purposely delegated to the Rust Foundation, to ensure continuity of the work.

As part of the hiring effort by the Foundation, we interviewed a few interested candidates with relevant experience. After a process in which an offer was eventually rejected by a candidate, the Foundation changed its approach and looked internally instead. The Foundation has given its Director of Technology, Joel, the opportunity to make the role of editor a part of his existing job. Because of his vast experience with industry standards and technical editing together with his already close proximity to the Rust project, Eric, Felix and Mara were quickly on board with having Joel as the editor, who has accepted this role.

## Specification Team

Since the editor will not be doing the work alone, we are forming a team around the specification work, the Specification Team, as a subteam of the language team.

Initially, its members are:

- Felix Klock (team lead)
- Mara Bos (team lead)
- Joel Marcey (team member, editor)
- Eric Huss (team member)

## Stakeholders

We will select and maintain a list of *stakeholders*, a selection of both experts and consumers of the specification, who will serve as advisors and reviewers.

Initially, the stakeholders will consist of:

- All members of the Rust language team
- One or more representatives from the types team
- One or more representatives from the operational semantics team
- One or more representatives from Ferrocene (High Assurance/Availability, e.g. Automotive Industry.)
- One or more representatives from Formal Methods Research and Development
- One or more representatives from Operating System Development (Rust for Linux; Microsoft)

## Authority and Approval

While the specification team is in charge of writing and editing the specification, the authority over definition of the Rust language remains with the relevant teams, such as the language team and the library API team. Those teams are expected to engage other teams/subteams to provide their approval when necessary.

To allow the specification team to produce content and iterate on it without being blocked by approval processes, we will be working on a draft specification in our work repository.
With the help of some tooling, we will publicly keep track of which items still require team approval, and which items have open concerns from stakeholders.

We will categorize all changes as either a minor or a major change.
Minor changes are items that appear uncontroversial or trivial to the specification team.
For example, changes that were already approved by the language team team via FCP, typographic and grammar fixes, clarifications where the original intent is clear, and similar unexciting changes.
Major changes are those that are potentially questionable, important, or controversial.
Any member of the specification team and the relevant authoritative team(s) and any specification stakeholder can flag a change as major.
Major changes to the specification will have to go through the usual approval process (e.g. language FCP) before they may appear in a published (non-draft) version of the specification.

The language and specification teams should strive to have at least one shared member (e.g. Felix), acting as a liaison to help ensure that the understanding of what we consider a minor versus a major change remains in sync.

## Goal

The goal of the specification team is to create and maintain the Rust specification.

The purpose of the Rust specification is to provide an authoritative resource for determining what source texts are valid Rust programs, and how such programs behave.

An ideal specification both (1.)&nbsp;defines *prescriptive bounds* on the semantics of a given Rust program for current and future Rust versions, and (2.)&nbsp;provides *descriptive details* of the semantics that are specific to the Rust version coupled with that instance of the specification.

The provision of the version-specific details can be provided directly in the specification, or can be indirectly provided via delegation to other documents owned by the relevant Rust teams.

<details><summary>Explanation of the terms</summary>
<blockquote>

The words above have been chosen carefully; it is worth elaborating on those words and the overall phrasing:

**"defines"**: The utility of a specification comes from (1.)&nbsp;forcing authors to define things and (2.)&nbsp;its value of those definitions to the readers of the specification.

**"semantics"**: Rust has a static and dynamic semantics.
The static semantics of Rust dictates which programs are accepted in the language, while the dynamic semantics determines which of those accepted programs are well-defined, as well as their respective meanings.
The word "semantics" in the purpose statement refers to both the static and dynamic semantics of Rust collectively.

**"current", "future"**: The Rust language has been evolving since its inception, and we expect it to continue to evolve going forward.
These evolutionary steps represent a traversal over the language design landscape.
For every Rust release, we expect the current implementation to be standing at one point in that landscape, while the Rust community's idealized goal awaits at some higher point up the mountain.

**"prescriptive", "descriptive"**: A descriptive dictionary is one that attempts to describe how a word *is* used, while a prescriptive dictionary is one that prescribes how a word *should be* used.
We take inspiration from that distinction to tease apart two important audience types.

Rust provides a stability promise: "You should never have to fear upgrading to a new version of stable Rust."
That raises a natural question: Why does the purpose statement distinguish version-crossing prescriptive definitions from version-specific descriptive definitions?

Our answer:
That stability promise left a bit of wiggle-room for itself, in terms of what the project considers "fear" vs "reasonable labor associated with a Rust upgrade."
When defining semantics, one must be more explicit about any such wiggle-room.
Some Rust users *need* a description of the expected semantics as it stands for the Rust release that sits in their hands; they are the audience for version-specific details.
But other Rust users, such as some library developers, have a more forward-looking perspective.
The forward-looking developers may require an assurance that one specific code snippet A will always be accepted, and will also always have a particular meaning.
They may require an assurance that a different snippet B will *never* be accepted.
Or they may require an assurance that a third snippet C leveraging Unsafe Rust will always have undefined behavior (e.g. to justify a local transformation by arguing that no *new* undefined behavior is injected by that transformation.)
These are all cases that call for a prescriptive definition of the semantics; it does not matter in these cases what the compiler currently does -- what matters is what it will do in the future, which is inherently prescriptive.

It would be premature to fix firm definitions in those areas, e.g. categorizing for each input program whether it is accepted or rejected by the type inference system, and then forcing all futures versions of Rust to follow that same categorization.
Another similar example: If we chose a fixed grammar, and then said all future versions of Rust must strictly categorize all source inputs as accepted or rejected according to that one grammar, then that would restrict our ability to add future backward-compatible language extensions to the grammar.
Therefore, these kinds of guarantees (especially with respect to details of the type inference rules, or details of what unsafe code is well-defined) are where the prescriptive *bounds* arise.
Such bounds allow for a middle ground of programs, where we do not commit all future versions of Rust to always make the same decision that the current version makes.
For example, one can then say, prescriptively, that a given grammar provides a lower bound on the set of programs that must be accepted by all future versions of Rust, while still allowing the language to evolve in a backward compatible fashion.
One can also say, descriptively, that the current version of Rust rejects source inputs that do not conform to the grammar.

The descriptive definitions tell the reader how a construct will behave with respect to that Rust version; the prescriptive bounds tell the reader what they can and cannot expect to hold true in the future of Rust.

Thus, we conclude that an ideal specification will need to address both the static and dynamic semantics, for both the prescriptive bounds and descriptive details.

**"delegates"**: There are broad areas where the questions of what semantics we want, and how they should be specified, are open research topics.
Examples of such areas include: macros 2.0, the type inference rules, the trait matching rules, and the operational semantics of unsafe code.
It is not reasonable for the specification team to claim authority on such topics.
Instead, other teams will be invited to contribute their own detailed descriptions, which can be published as their own documents that the specification can reference.
Each such document is, like the specification itself, coupled to a specific Rust version.
Furthermore, each such document is analogous to the detailed descriptions: the scope of each document produced via delegation is intended to be restricted to a specific Rust version.

If a contributing team has input about broader prescriptive rules that should hold beyond the current Rust version, then that should be be part of the Rust specification document itself.
Such prescriptive rules should always be the responsibility of the specification team to incorporate into the document.
All such prescriptive rules are then subject to the specification approval process.

</blockquote>
</details>

<p></p>

## Incremental Development

It is ambitious to provide both prescriptive bounds for current and future Rust versions and descriptive details of the current Rust version.
We will maximize the value of our efforts by working iteratively and incrementally.
The specification can have gaps where the prescribed bounds are broader than necessary.
Subsequent releases of the specification can tighten those prescribed bounds.

We expect early versions of the spec to focus heavily on delivering the detailed description of the current Rust version.
Such a specification could be derived heavily from an existing work product, such as the Ferrocene specification, since that explicitly focuses on providing a detailed description of a specific Rust version. 
Feedback on those version-specific descriptions will help us learn how best to craft the prescriptive bounds in the specification.

The prescriptive bounds can start with useful high-level guarantees (e.g. "safe Rust cannot cause undefined behavior"), and then future versions of the specification can incrementally add more details to the prescriptive bounds (e.g. "unsafe Rust cannot cause undefined behaviour under the following conditions: …"), incrementally gaining more details over time.

## Scope

The specification should cover at least the following areas of Rust's syntax and semantics. Some parts may be inherently coupled to specific backends or target implementation techniques (e.g. inline asm).

* The grammar of Rust, specified via Backus-Naur Form (BMF) or some reasonable extension of BNF.
* Macro expansion
    * Macro-by-example (`macro_rules`) transcription; Hygiene
    * `cfg` attributes
    * Procedural macros; attributes and derive
* Path and identifier resolution
    * Modules
* Static semantics
    * Type definitions; type expressions; layout
    * Type inference and type-checking; subtyping
    * Lifetimes and borrow-checking
* Generics; Associated item resolution and Trait solving
* Operational semantics of safe Rust
    * binding forms; match expressions; drop glue
    * moving and copying of values; borrows
    * field projection; method dispatch
    * operator overloading
* Operational semantics of unsafe Rust
    * memory model
    * inline assembly
* Const evaluation
* Crates and crate linkage

This list can be expanded over time.

## Presentation

The Rust Specification will be a publicly accessible document, similar to all other Rust documentation (and with the same [licensing](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT) [terms](https://github.com/rust-lang/rust/blob/master/LICENSE-APACHE)).
The text will be written in English and will only use technical terms that are defined within the specification itself or that have clear definitions in a freely available online dictionary.

Individual items in the specification can each be referenced and named: not just in hyperlinks, but also in human text (e.g. "[syntax.patterns.arm.5]"). When possible, these names/references to items should persist across versions of the specification.

Iterations of the specification should include renderings that highlight the differences between versions. (See e.g. Ada Reference Manual.)

The Rust specification will be maintained in a format that that encourages volunteer contribution, even if the specification team expects to have to reauthor each contribution in order to maintain a consistent voice for the specification.

While completeness and correctness are the primary priority, we will try our best to make the specification as accessible as possible. Ideally, any Rust programmer should be able to dive in and find the answer to any language question they might have, without the need to ask a "language lawyer" who is already deeply familiar with the document.

### Release Cadence

Rust releases continue to proceed independently of the specification approval process.

If a specification has not been approved for a given release, then the release goes out without an associated specification.
(We might still decide to deliver a specification associated with a given release at a later time, however.)

This is by design. The specification effort must not add new hurdles for the project to overcome in order to meet its existing obligations, such as the 6-week release cadence.

Our aspiration is that we will *eventually* reach a point where delivering an updated specification is automatic and can be done according to the project's 6-week release cadence.
But, for the short and medium term, we want to have the freedom to lag behind that 6-week release cadence.
The ability to lag behind the Rust release schedule may be especially useful when the specification team is incrementally adding new content for previously unaddressed areas, or significantly narrowing the prescriptive bounds in the current version of the specification.

While the specification development process will not block releases, changes to *language features* should be coupled with relevant updates to the specification.
Once we have begun publishing a specification coupled to specific releases, then changes to the language features that are documented in the current specification cannot be stabilized without a corresponding pull request approved by the specification team to the current draft specification.
Changes to language features that are not documented in the specification can be stabilized without an update to the specification, but require a specification team member's acknowledgement that the corresponding feature is undocumented.

By enforcing this rule that new features must be part of the specification before they are stabilized, we will hopefully eliminate the main source of potential lag between the specification and the Rust release.

## Next Steps

Now that we have taken the initial steps of selecting an editor, forming the initial team, and documenting our vision in this blog post, the next steps are:

- Setting up a regular meeting schedule for the team.
- Establishing the list of stakeholders.
- Making a first "demo product", to be reviewed by the stakeholders. That is, setting up our tooling and picking a small slice of Rust to fully document (including cross references, etc.), to give an accurate first impression of what the full specification will look like.
