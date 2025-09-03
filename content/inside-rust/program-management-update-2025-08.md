+++
path = "inside-rust/9999/12/31/program-management-update-2025-08"
title = "Program management update — August 2025"
authors = ["Tomas Sedovic"]


[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/"
+++

# Program management update — August 2025

Quite a lot has happened in August. Let's dive in!

## Leadership Council

The Rust Leadership Council is now looking for new representatives and this is as good a time as any to remind everyone what it does.

The Council is composed of Project members, one for each [top-level team](https://www.rust-lang.org/governance) and its subteams. They [represent the interests of their teams and the long-term success of the Project as a whole](https://github.com/rust-lang/leadership-council/blob/main/roles/council-representative.md).

They also coordinate with the Foundation and elect the Project Directors on the Foundation's board (more on this later).

Their work largely happens publicly in [the Council's repository](https://github.com/rust-lang/leadership-council/issues).

The representatives meet every other Friday and link these issues in their agenda. When they make a decision, they summarize the discussion and propose an FCP (final comment period) on the relevant issue. As with all FCPs in the Project, they're interested in any feedback people have until the comment period is closed. They review it all.

If you want to see what the Council is up to, these issues are a great complement to the [Council meeting minutes](https://github.com/rust-lang/leadership-council/tree/main/minutes/sync-meeting) I'm taking and publishing on their behalf.

[kangrejos]: https://kangrejos.com/

To see this in practice, here is a proposal to send me to the annual [Rust for Linux workshop (Kangrejos)][kangrejos]:

https://github.com/rust-lang/leadership-council/issues/217

### Representative Selections

Every six month, half of the Council's term ends.

Infra, Lang, Libs and Mods teams are looking for a new representative and the nominations are now open.

If you want to learn more or you're interested in representing your team, please read [Eric Huss's post announcing the selection](https://blog.rust-lang.org/inside-rust/2025/08/15/leadership-council-repr-selection/).


## Rust Foundation Project Directors 2025

This fall, [we're also looking for new Project Directors][pd-blog].

[pd-blog]: https://blog.rust-lang.org/inside-rust/2025/08/20/electing-new-project-directors-2025/

The Directors have staggered terms as well and half is up for election every year.

This time it's Santiago Pastorino, Scott McMurray, and Jakob Degen's. None are seeking reelection.

These are seats directly on the Rust Foundation board. The Project directors serve the interest of the Rust Project as a whole and sit alongside the Member Directors who represent companies funding the Foundation.

Each resolution the Foundation passes must be approved by *both* Member and Project Directors separately. That means regardless of the size of the board, the Project has an equal voice in Foundation matters.

You can nominate yourself or another person until 2025-09-18. Please [read the blog post for more information][pd-blog]. The Project is always looking for fresh faces and diverse voices.

I am the [facilitator of the selection process](https://github.com/rust-lang/leadership-council/blob/main/policies/project-directorship/election-process.md#setup) this time around. That means I've authored the blog post above, proposed the timeline, and I'll seek out consent and statements from the nominees. I've also announced the election on zulip as well as an email that should reach all Project members and I'll see it all through, including facilitating the actual election process.

## Bevy/gamedev followup

A month ago, the Lang team invited the [Bevy game engine folks](https://bevy.org/) to talk about issues faced by their new users as well as any pain points the project is facing.

There were two big topics they mentioned: reflection and variadic generics.

Both have been requested for a long time and the interest is much broader than just Bevy or even just the game development space.

### Reflection

Reflection is a mechanism that lets your program look at any type and understand it: getting its name, fields and *their names and types* while your program is running. This is in contrast to the `derive` macro or trait bounds that are processed at compile time.

Projects like Bevy currently rely on the `derive` macros. For example, pretty much all its types have `derive(Reflect)` to provide dynamic field access and type inspection, serialization/deserialization and scripting. While the usage is simple, these macros are difficult to write and debug. And the language has limitations on where they can be applied.

You can only implement a trait for a type (which is what `derive` does, under the hood) if either the trait or type is *defined* in the crate you're implementing it in (this is the [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)).

So if want to implement `Reflect` (defined in [bevy_reflect](https://crates.io/crates/bevy_reflect), not your crate) you could derive it for your custom type, but not e.g. for [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) or `[f32; 2]` because they're defined in the standard library.

You have to create a new `enum`/`struct` that wraps that type and implement the trait yourself. This all gets very complex very quickly and no good solution exists right now.

In practice, projects like Serde and Bevy often provide implementations for common standard library types (including tuples up to a limited size). But when a new crate comes up, it either has to implement all the useful traits in the ecosystem, convince to the ecosystem to provide the implementations for its types or be immediately less useful than the existing crates. This can lead to ecosystem stagnation.

With reflection, a lot of this machinery would just be available on every type everywhere and everyone could use it.

[oli-obk](https://github.com/oli-obk) opened the [reflection and comptime goal](https://rust-lang.github.io/rust-project-goals/2025h2/reflection-and-comptime.html) for the 2025H2 period that will build the initial functionality and extend it later on.

This happened with little intervention on my part, but I made sure that the Bevy folks were aware (they were!) and I'll be keeping an eye on this to help move it forward and be on the lookout for other projects that may find this useful.

### Variadic Generics

Remember how I said crates implement traits for tuples up to a certain size? That's a limitation of Rust that is -- again -- felt in many different areas.

The basic idea is: suppose you have a tuple of types that all implement a given trait. You want the tuple to be able to implement that trait too.

For example, if all the elements of a tuple implement the [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait, you should be able to `dbg!()` or `println!("{:?}", ...)` such a tuple.

And you can!

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("{tuple:?}");
}

// (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
```
([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=88ec1f7ff90a07da8a0c9852c81594ce))

...sort of:

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("{tuple:?}");
}

// error[E0277]: `({integer}, [...], {integer})` doesn't implement `Debug`
// --> src/main.rs:3:16
// [...]
 
```
([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=245a91abb5ac3162bb537428e348fd02))

Even in the Rust standard library, traits like this are [only implemented for tuples up to 12 elements](https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations-1).

This is, again, keenly felt by anyone writing an Entity Component System (ECS) or Object-relational mapping (ORM) and in particular their query systems.

Some time ago, [Olivier Faune](https://github.com/PoignardAzur) took up the mantle and drove the discussions at the last two RustWeek conferences (read their [2024](https://poignardazur.github.io/2024/05/25/report-on-rustnl-variadics/) and [2025](https://poignardazur.github.io/2025/06/07/report-on-variadics-rustweek/) reports).

Olivier also wrote [Variadic Generics ideas that won't work for Rust](https://poignardazur.github.io/2025/07/09/variadic-generics-dead-ends/) which highlights the many pitfals even the simplest "why don't we just..." ideas inevitably run into. This is a complex feature that touches a lot of Rust's machinery and it can't be added in easily.

But we still want it!

Some of the efforts blocking this in the past have either been resolved or are going to be soon (e.g. the [new trait type solver](https://rustc-dev-guide.rust-lang.org/solve/trait-solving.html)). And the Lang team now feels like they have the capacity to review a proposal.

I've done a lot of background reading (which made me appreciate the complexity), talked to Olivier and [Alice Cecile](https://github.com/alice-i-cecile) and [opened a design meeting on the Lang side](https://github.com/rust-lang/lang-team/issues/348) as there is a way forward now.

The next steps are getting an RFC written and scheduling the design meeting. I'm again on the lookout for other people interested in the space (either with proposals of their own or usecases we want to make sure are heard) so I can point them to this space.


## Lori Lorusso: Foundation Director of Outreach

Earlier this month, [Lori](https://rustfoundation.org/about/#lori-lorusso-director-of-outreach) joined the Rust Foundation.

She'll be overseeing the grants program as well as the external and internal outreach and communication. She'll also look at bringing in communities and people from areas that we haven't reached yet.

As our roles overlap a bit (and can definitely benefit from our collaboration -- e.g. on the communication between the Project and Foundation), we've set up a regular check-in set up.

One of the near-term things I'll do is get her onboarded on [the Rust blog](https://blog.rust-lang.org/) system so she can publish posts there.


## Content Team

This month also saw a formation of a [new team](https://www.rust-lang.org/governance/teams/launching-pad#team-content) focused on producing audio/video/text content about Rust and people working on it. These can be interviews, podcasts etc.

[TC](https://github.com/traviscross) and [Pete LeVasseur](https://github.com/PLeVasseur) are the leads, and we also have [Cameron Dershem](https://github.com/cldershem), [Xander Cesari](https://github.com/MerrimanInd), [Tyler Mandry](https://github.com/tmandry), [Lori](https://github.com/LoriLorusso), and yours truly.

[rustconf]: https://rustconf.com/

We already have a few interviews planned for [RustConf 2025][rustconf].

Here's the [Content Team's charter](https://github.com/rust-lang/leadership-council/issues/206). Forming a new team is something you propose to the Leadership Council in their repo's issues. It then gets discussed at their meeting and decided on using the FCP process.


## build-std

[build-std](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html) is an ongoing initiative to provide a blessed, stable process for building the Rust standard library you can use instead of the one we provide.

There are many different motivations for this, for example supporting platforms where Rust doesn't ship a precompiled library, optimizing it to known hardware or reduce its size (by e.g. removing features that are not necessary). This is of big interest to the [Rust for Linux project](https://rust-for-linux.com/).

[David Wood](https://github.com/davidtwco) and [Adam Gemmell](https://github.com/adamgemmell) wrote a comprehensive document describing its history, motivations, past experiments, and proposals for an minimal scope that they can start building.

This has been regularly reviewed by a handful of people across the teams that will be affected by this. After many rounds of feedback, David feels the proposal is solid enough to open to a broader group.

He's shared it with more representatives from the Libs, crates.io, embedded, bootstrap and infra and compiler and Cargo teams. He's also shared it with non-Cargo users e.g. the Rust for Linux folks (who are interested in building std without Cargo).

Once this next round settles down, he will open the RFC (it will likely be several documents each focusing on a different stage of the effort).

## Rust for Linux

[Rust for Linux](https://rust-for-linux.com/) is an ongoing initiative to be able to write Linux kernel code in Rust. The motivation is memory safety, a modern language and bring in more contributors due to the combination of the two.

The focus in this second half of 2025 is to bring unstable features the project is using into stable Rust. Linux takes stability and backwards compatibility very seriously and will not rely on features that can change from release to release or even be removed entirely.

### RFC: pass pointers to `const`

Rust has support for [inline assembly](https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html), via the `asm!` macro for example. Assembly is as close as one can get to writing machine code directly. This lets you do things the language doesn't have direct support such as accessing CPU registers or reading/writing values to device-specific segments in memory that allow control their behavior (this is often used when writing drivers).

The macro lets you interpolate constant values into the assembly code you've written, similar to how you can interpolate values to a format string or macro:

```rust
println!("The meaning of life, universe and everything: {}", 42);
```

But constant values only get you so far. A common thing when writing assembly is to be able to pass *pointer* values (e.g. pointer to a specific field of a struct you want to manipulate) around. But currently, [only integer constant expressions are allowed](https://doc.rust-lang.org/1.89.0/reference/inline-assembly.html#r-asm.operand-type.supported-operands.sym).

[Alice Ryhl](https://github.com/Darksonn) opened an RFC that [allows specifying pointers in the `const` operand](https://github.com/rust-lang/rfcs/pull/3848) too. Conceptually (and in assembly specifically) pointers could be thought of as just numbers that are interpreted as addresses to memory (although in Rust, the story is far more complex and [pointers are not the same thing as integers](https://doc.rust-lang.org/std/ptr/index.html#provenance)).

This is now ready for feedback from the Lang team so I've opened a [design issue](https://github.com/rust-lang/lang-team/issues/347)  and got it scheduled for review and discussion.

### Field projections

[field-projections]: https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html

We also had a design meeting on [Field Projections][field-projections]. When you have a type behind a `&` or `&mut` reference, you can access its field "directly" as if the pointer type wasn't there:

```rust
struct Position {
    x: f32,
    y: f32,
}

impl Position {
	fn get_x(&self) -> &f32 {
	    &self.x
	}
}
```

The language understands `Position` is behind a pointer, calculates an offset to the field `x` and gives you that pointer back.

But there's a long list of wrapper types where field access makes sense, but it's not implemented because the semantics or limitations are different from the regular `Deref/DerefMut` traits. For example: `MaybeUninit<T>`, `Pin<P>`, `Cell<T>`, or the raw pointers `*const T`/`*mut T`. And of course custom types.

Linux uses pinned values (`Pin<P>`, values that can't move around in memory), raw pointers and `MaybeUninit` all over the place in addition to many custom fields that would greatly benefit from field projections.

[Benno Lossin](https://github.com/BennoLossin) who owns the [Field Projection goal][field-projections] prepared a [design](https://hackmd.io/@rust-lang-team/S1I1aEc_lx) to move this forward as a lang experiment. This was approved, we now have a [Field Projection tracking issue](https://github.com/rust-lang/rust/issues/145383) as well as an [initial implementation](https://github.com/BennoLossin/rust/tree/field-projections).

### Reducing codegen size

The last Rust for Linux meeting got into a fascinating discussion about an ongoing need to reduce the size of binary generated by rustc. There is functionality in the Rust standard library that is not used in the kernel, but takes up space. For example support for 128-bit integer types, the entire `alloc` crate or the floating point formatting code. Some are just never going to be relevant, others (like alloc) are re-implemented by Rust for Linux.

There's an interest in being able to compile certain features out (using the `cfg!` macros) and/or having a minimal core that projects can build on top. Add the functionality they need, without being burdened by the rest.

This is something I plan to gather more information about and follow-up on.

### Kangrejos 2025

Finally, I'll be joining the Rust for Linux team at their [Kangrejos workshop][kangrejos] in Spain in September. I hope to get to know the Rust for Linux people better in a less formal environment, get a more hand-on experience with what they're doing and the challenges they're facing, and be the conduit for a smoother collaboration between them and the Rust Project.


## Conferences

September is going to be an _eventful_ (if you pardon the pun) month!

First up, [RustConf 2025][rustconf] takes place in Seatle, Washington, USA (from 2025-09-02 to 2025-09-05). RustConf has virtual tickets so you can attend online as well.

Second is the [RustGlobal China and RustChinaConf 2025](https://rustcc.cn/2025conf/) in Hangzhou, China (from 2025-09-13 to 2025-09-14)

And finally the aforementioned [Rust for Linux workshop, Kangrejos][kangrejos] in Oviedo, Spain (from from 2025-09-17 to 2025-09-18).

If nothing else, look forward to a good batch of talks being posted online in the coming weeks and months!

## Stats

Total words of meeting minutes written: 138.6k (June - August).

Meetings attended: 31

Total words written: 46k

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 2.5k
* Libs: 5.9k
* Leadership council: 2.9k

You can see the [June and July stats in the previous update](https://blog.rust-lang.org/inside-rust/2025/08/05/program-management-update-2025-07/#fun-stats).
