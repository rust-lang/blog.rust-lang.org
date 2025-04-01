+++
path = "inside-rust/2023/11/15/spec-vision"
title = "Our Vision for the Rust Specification"
authors = ["Eric, Felix, Joel and Mara"]
aliases = ["inside-rust/2023/11/15/spec-vision.html"]

[extra]
team = "the specification team"
team_url = "https://www.rust-lang.org/governance/teams/lang#Specification%20team"
+++

A few months ago, by accepting [RFC 3355](https://rust-lang.github.io/rfcs/3355-rust-spec.html), the decision was made to start working on an official specification for the Rust language. Eric (maintainer of the Rust Reference), Felix (Rust language team), Joel (Rust Foundation) and Mara (author of the RFC) have been working together to get this effort started.

With this blog post, we present the steps we have taken so far and our plans for approaching the rest of this big task.

## Editor

One of the first steps was to fill the role of "editor" [as laid out in the RFC](https://rust-lang.github.io/rfcs/3355-rust-spec.html#role-of-the-editor). The responsibility of coordination and editing of the specification is purposely delegated to the Rust Foundation, to ensure continuity of the work.

As part of the hiring effort for this role by the Foundation, we interviewed a few interested candidates with relevant experience. Because an offer to a candidate was eventually rejected, the Foundation opted to consider internal options as an alternative. The Foundation's Director of Technology, Joel, came forward as a willing candidate for the position as part of his existing job. Eric, Felix, and Mara were quickly on board with the idea of having Joel as the Specification Editor due to his vast experience with industry standards and technical editing and his proximity to the Rust project.

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

While the specification team is in charge of writing and editing the specification, the authority over definition of the Rust language remains with the relevant teams, such as the language team and the library API team. Those teams are expected to involve other teams/subteams when necessary, such as by filing issues, nominating issues for discussion, and requesting FCP approval on critical decisions.

To allow the specification team to produce content and iterate on it without being blocked by approval processes, we will be working on a draft specification in our work repository.
With the help of some tooling, we will publicly keep track of which items still require team approval, and which items have open concerns from stakeholders.

We will categorize all changes as either a minor or a major change.
Minor changes are items that appear uncontroversial or trivial to the specification team.
For example, changes that were already approved by the language team via FCP, typographic and grammar fixes, clarifications where the original intent is clear, and similar unexciting changes.
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

**"current and future versions"**: The Rust language has been evolving since its inception, and we expect it to continue to evolve going forward.
These evolutionary steps represent a traversal over the language design landscape.
For every Rust release, we expect the current implementation to be standing at one point in that landscape, while the Rust community's idealized goal awaits at some higher point up the mountain.

Rust's evolution follows two axes: release versions (such as Rust 1.73) and editions (such as the 2021 edition).
Each version of Rust supports its most recent edition and all prior editions.
The specification will evolve in parallel, with specification releases corresponding to Rust releases.
Each release of the specification defines the semantics of all editions supported by that Rust release.

**"prescriptive", "descriptive"**: A descriptive dictionary is one that attempts to describe how a word *is* used, while a prescriptive dictionary is one that prescribes how a word *should be* used.
We take inspiration from that distinction to tease apart two important audience types.

Rust provides a stability promise: "You should never have to fear upgrading to a new version of stable Rust."
That raises a natural question: Why does the purpose statement distinguish version-crossing prescriptive definitions from version-specific descriptive definitions?

Our answer:

The descriptive definitions tell the reader how a construct will behave with respect to a specific Rust version (e.g. Rust 1.76).
The prescriptive bounds tell the reader what they can and cannot expect to hold true in future Rust versions.

We treat these as distinct concepts because they serve two distinct audiences.

The statement of Rust's stability promise left a bit of wiggle-room for itself, in terms of what the project considers "fear" vs "reasonable labor associated with a Rust upgrade."
When defining semantics, one must be more explicit about any such wiggle-room.
Some Rust users *need* a description of the expected semantics as it stands for the Rust release that sits in their hands; they are the audience for version-specific details.
But other Rust users, such as some library developers, have a more forward-looking perspective.
The forward-looking developers may require an assurance that one specific code snippet A will always be accepted, and will also always have a particular meaning.
They may require an assurance that a different snippet B will *never* be accepted.
Or they may require an assurance that a third snippet C leveraging Unsafe Rust will always have undefined behavior (e.g. to justify a local transformation by arguing that no *new* undefined behavior is injected by that transformation.)
These are all cases that call for a prescriptive definition of the semantics.
Stating what the Rust compiler currently does is not sufficient for these developers; they need to know what future versions of the compiler may do, which is inherently prescriptive.

**"bounds"**: From the perspective of a Rust user, an ideal specification would provide definitions that are both precise and prescriptive.
However, it is premature for the project to provide prescriptive definitions that are 100% precise in all areas of Rust's semantics.

Example 1: Rust's type inference rules are not ready to be set in stone for all future versions.
The rules are still undergoing development; a sound program that is rejected by the type system today may be deemed acceptable tomorrow.

Example 2: If we chose a fixed grammar, and then said all future versions of Rust must strictly categorize all source inputs as accepted or rejected according to that one grammar, then that would restrict our ability to add future backward-compatible language extensions to the grammar.

In order to allow prescriptive definitions in the face of such challenges as these, we sacrifice some precision in order to regain flexibility,
by planning for our prescriptive definitions to be framed as *bounds* on the semantics.

Example 3: The Rust memory model is still an open research area.
We are not yet prepared to establish a binary sound/unsound categorization for arbitrary unsafe code and set it in stone for all future versions of Rust.

But, there are some unsafe code patterns that are definitely sound; these can be used as the basis for defining a *lower bound* on what unsafe code is well-defined.
There are likewise unsafe code anti-patterns that are certain to be unsound; these can be used as the basis for defining an *upper bound* on what unsafe code *might* be well-defined in Rust's dynamic semantics (or, as an alternative perspective: these provide a lower bound on what unsafe code will always be considered undefined behavior in Rust).

Prescriptive bounds allow for the specification to include a middle ground of programs, where we do not commit all future versions of Rust to always make the same decision that the current version makes.
For example, one can then say, *prescriptively*, that a given grammar provides a lower bound on the set of programs that must be accepted by all future versions of Rust, while still allowing the language to evolve in a backward compatible fashion.
One can also say, *descriptively*, that the current version of Rust rejects source inputs that do not conform to the grammar.

Over time, the gap between the upper and lower bounds will shrink as the specification evolves and becomes more precise, resolving ambiguities in Rust's semantics.
In the limit, when/if the upper and lower bounds meet, this idealized process yields a completely precise prescriptive definition.

In the interim, before we reach that limit, the specification will provide both prescriptive bounds and descriptive details, for both the static and dynamic semantics.

**"delegation"**: There are broad areas where the questions of what semantics we want, and how they should be specified, are open research topics.
Examples of such areas include: macros 2.0, the type inference rules, the trait matching rules, and the operational semantics of unsafe code.
It is not reasonable for the specification team to claim authority on such topics.
Instead, other teams will be invited to contribute their own detailed descriptions, which can be published as their own documents that the specification can reference.
Each such document is, like the specification itself, coupled to a specific Rust version.
Furthermore, each such document is analogous to the detailed descriptions: the scope of each document produced via delegation is intended to be restricted to a specific Rust version.

If a contributing team has input about broader prescriptive rules that should hold beyond the current Rust version, then that should be part of the Rust specification document itself.
Such prescriptive rules should always be the responsibility of the specification team to incorporate into the document.
All such prescriptive rules are then subject to the specification approval process.

</blockquote>
</details>

<p></p>

## Incremental Development

It is ambitious to provide both prescriptive bounds for current and future Rust versions and descriptive details of the current Rust version.
We will maximize the value of our efforts by working iteratively and incrementally.

We expect early versions of the spec to focus heavily on delivering the detailed description of the current Rust version.
Such a specification could be derived heavily from an existing work product, such as the Ferrocene specification, since that explicitly focuses on providing a detailed description of a specific Rust version. 
Feedback on those version-specific descriptions will help us learn how best to craft the prescriptive bounds in the specification.

Due to our aforementioned focus on the current Rust version, early versions of the specification may have gaps where the prescriptive bounds are more imprecise than necessary.
For example, prescribing "unsafe Rust code might cause undefined behavior" provides no guidance on how to write well-defined unsafe code.
Even with such imprecision, the prescriptive bounds can still provide useful high-level guarantees (e.g. "safe Rust cannot cause undefined behavior").
Future versions of the specification then add more prescriptive details (e.g. "unsafe Rust code cannot cause undefined behaviour under the following conditions: …") until we reach our desired level of precision.

## Scope

The specification should cover at least the following areas of Rust's syntax and semantics. Some parts may be inherently coupled to specific backends or target implementation techniques (e.g. inline asm).

* The grammar of Rust, specified via Backus-Naur Form (BNF) or some reasonable extension of BNF.
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

The Rust specification will be maintained in a format that encourages volunteer contribution, even if the specification team expects to have to reauthor each contribution in order to maintain a consistent voice for the specification.

While completeness and correctness are the primary priority, we will try our best to make the specification as accessible as possible. Ideally, any Rust programmer should be able to dive in and find the answer to any language question they might have, without the need to ask a "language lawyer" who is already deeply familiar with the document.

### Release Cadence

Rust releases will proceed independently of the specification approval process.

If a specification has not been approved for a given release, then the release will go out without an associated specification.
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
