+++
path = "2018/04/06/all-hands"
title = "The Rust Team All Hands in Berlin: a Recap"
authors = ["Aaron Turon"]
aliases = ["2018/04/06/all-hands.html"]
+++

Last week we held an "All Hands" event in Berlin, which drew more than 50 people
involved in 15 different Rust Teams or Working Groups, with a majority being
volunteer contributors. This was the first such event, and its location reflects
the current concentration of team members in Europe. The week was a smashing
success which we plan to repeat on at least an annual basis.

The impetus for this get-together was, in part, our [ambitious plans] to ship
*Rust, 2018 edition* later this year. A week of work-focused facetime was a
great way to kick off these efforts!

We've also asked attendees to blog and tweet about their experiences at the
#RustAllHands hashtag; the Content Team will be gathering up and summarizing
this content as well.

[ambitious plans]: https://blog.rust-lang.org/2018/03/12/roadmap.html

## Highlights by team

Below we'll go through the biggest items addressed last week. **Note
that, as always, reaching consensus in a meeting does *not* mean any final
decision has been made**. All major decisions will go through the usual [RFC
process].

[RFC process]: https://github.com/rust-lang/rfcs

### Language design

- **Macros**: put together a proposal for the 2018 edition.
  - Stabilize a forward-compatible subset of procedural macros that explicitly
    opts out of hygiene (by asking all names to be interpreted at the call
    site).
  - Work out a revised API surface for procedural macros based on what we've learned so far.
  - Stabilize importing macros through normal `use` statements.
  - **Alex Crichton will drive the stabilization process**.
- **Extern types**: worked through [remaining issues for stabilization](https://github.com/rust-lang/rust/issues/43467#issuecomment-377521693).
- **Improvements to `derive`**: a [proposal](https://github.com/rust-lang/rfcs/pull/2385) to make `derive` more ergonomic in Rust 2018.
- **Best practices**: set up a cross-cutting guidelines effort, encompassing the
  API guidelines but also including style, lints, and Lang Team recommendations.

### Libraries

- **SIMD**: talked through final steps of stabilization; we hope to [stabilize in 1.27](https://github.com/rust-lang/rust/issues/48556#issuecomment-378184312).
- **Allocators**: developed a conservative proposal for stabilizing global allocators; **Simon Sapin has set up a [new tracking issue](https://github.com/rust-lang/rust/issues/49668)**.

### Compiler

- **Tool integration**: extensive discussion and planning about the needs of
  compiler clients, composable plugins, and the compiler's new query
  architecture.
- **Query compilation**: a plan for end-to-end query compilation, i.e. fully-incremental compilation.
- **libsyntax**: a long-run plan for a revamped libsyntax, shared amongst a variety of tools.
- **Contribution**: improved contribution experience for the compiler.

### Community

- **Mozilla Activate**: shipped a revamp of the page
- **RustBridge "in a box"**: shipped a ton of resources.
- **Events**: developed event standards, including:
  - Diversity Outreach
  - [Speaker support and quality](https://github.com/rust-community/events-team/pull/18)
  - [Program guidelines](https://github.com/rust-community/events-team/pull/19)
  - [Review committee guidelines](https://github.com/rust-community/events-team/pull/20)
  - Communication and transparency (e.g. [timelines](https://github.com/rust-community/events-team/pull/22))
  - Attendee experience, accessibility and support

### Documentation

- **Edition planning**: determined resources needed for the 2018 edition, what
  that means for the Rust Bookshelf, and who will be responsible for each.
- **Team blog**: “This week in Rust docs” is going to move to a new [docs team blog](https://rust-docs.github.io/blog/)!
- **Doxidize** (aka rustdoc2): made [initial public release](https://github.com/steveklabnik/doxidize); it's like https://docusaurus.io/ but for Rust.
- **Intermediate-level docs**: contributed to [idea generation](https://github.com/rust-docs/team/issues/8).

### Tools

- **Edition planning**, especially for the rustfix tool.
- **Clippy lint audit**: developed plan for reaching 1.0 on Clippy this year, based on categorizing lints into Correctness, Performance, Style, Complexity, Pedantry, and Nursery categories.
- **Custom test frameworks**: reached consensus on most of the details for [the RFC](https://github.com/rust-lang/rfcs/pull/2318)
- **IDEs**: discussed improvements to code completion, stability
  improvements, and new features like refactoring and auto-import support.

### Cargo

- **Xargo integration**: making a few more platforms tier 1 addresses the
  immediate need for embedded; otherwise, plan to go
  the
  ["std-aware Cargo" route](https://github.com/rust-lang/rfcs/pull/1133#issuecomment-362355002) late
  in 2018. Key insight: will entirely obsolete the concept of "targets" in rustup.
- **Rustup integration**: with Xargo integration we can simplify rustup; plan to
  expose new interface via Cargo late in 2018.
- **Custom registries**: ready to stabilize.
- **Profiles**: the v2 design is clear, and **Aleksey Kladov plans to implement**.
- **Public dependencies**: significantly revised plan to have more conservative ecosystem impact. **Aaron Turon will write a blog post**.
- **Build system integration**: determined two pieces of functionality to implement to decouple the RLS from Cargo.
- **Project templates**: developed a minimal design; **withoutboats will write an RFC**.
- **Custom workflows**: designed workflow customization, which is useful for frameworks; **Aaron Turon has written a [blog post](http://aturon.github.io/2018/04/05/workflows/)**.

### Infrastructure

- **bors queue**: hatched and resourced lots of ideas to reduce the PR queue for Rust 2018.
- **crater**: pietroalbini is testing a bot for running crater!
- **Travis log bot**: TimNN has [written a bot] to [extract errors] from Travis logs

[written a bot]: https://github.com/rust-ops/rust-log-analyzer
[extract errors]: https://github.com/rust-lang/rust/pull/49513#issuecomment-377538323

### WG: CLI apps

- [WG overview slides](https://git.io/rust-all-hands-cli).
- **Survey and strategy**: dove into survey data and developed strategy from it; posts forthcoming.
- **Distribution**: "distribution-friendly" badge on crates.io
- **Extensible Cargo install**: wrote [an RFC](https://github.com/rust-lang/rfcs/pull/2376) on-site!

### WG: network services

- [WG overview slides](https://gist.github.com/withoutboats/6d4c4639b286d3da19d89d8af82d82d7).
- **Launching the WG**: determined goals for the WG, including async/await, documentation, mid-level HTTP libraries, and the [Tower](https://github.com/tower-rs/tower) ecosystem.
  Kickoff announcement coming soon!
- **Async/await**: finalized design and stabilization approach for RFCs (blog post and links to RFCs [here](https://boats.gitlab.io/blog/post/2018-04-06-async-await-final/)).

### WG: embedded devices

- [WG overview slides](https://github.com/japaric/all-hands-2018-embedded)
- **Embedded Rust on stable**: addressed all known blockers and several mid-priority issues as well.
- **The Embedded Rust book**: defined audience and basic outline.

### WG: WebAssembly

- [WG overview slides](https://gist.github.com/fitzgen/700e134cffe9a8438524f0a39810a4d0).
- **2018 edition planning**, including scoping the toolchain and book efforts for the release.
- **JS integration**: dove into integrating JS callbacks vs Rust functions in wasm-bindgen.
- **Ecosystem integration**: wasm-bindgen now works with CommonJS!
- **Code bloat**, [reducing the footprint] of panicking from 44k to 350 bytes.

[reducing the footprint]: https://github.com/rust-lang/rust/pull/49488

### Unsafe code guidelines

- **Restarted the WG**: dug back into two competing approaches ("validity" and
  "access"-based), substantially iterated on both. **Niko Matsakis and Ralf Jung
  will be writing blog posts about these ideas**.
- **Pressing issues**: tackled a few near-term decisions that need to be made,
  including `union` semantics, `Pin` semantics, `thread::abort` and more.

### Web site

- **Domain WG sketching**: over a couple of sessions, the four domain-centered
  working groups (listed above) developed some initial sketches of landing pages
  for each domain.

### Rust reach

- **Prepared for launch**, which happened [earlier this week!](https://blog.rust-lang.org/2018/04/02/Increasing-Rusts-Reach-2018.html)

### New working groups

In addition to the work by existing teams, we had critical mass to form two new working groups:

- **Verification**: bringing together folks interested in testing, static analysis, and formal verification.
- **Codegen**: work to improve the quality of code rustc generates, both in terms of size and runtime performance.

The Verification WG has been [formally announced](https://internals.rust-lang.org/t/announcing-the-formal-verification-working-group/7240/), and the Codegen WG should be announced soon!

## General reflections and notes of appreciation

The combination of having a clear goal -- Rust 2018 -- and a solid majority of
team member present made the All Hands extremely fun and productive. We strive
to keep the Rust community open and inclusive through asynchronous online
communication, but it's important to occasionally come together in person. The
mix of ambition and kindness at play last week really captured the spirit of the
Rust Community.

Of course, an event like this is only possible with a lot of help, and I'd like
to thank the co-organizers and Mozilla Berlin office staff:

- Johann Hofmann
- Jan-Erik Rediger
- Florian Gilcher
- Ashley Williams
- Martyna Sobczak

as well as all the team leads who organized and led sessions!
