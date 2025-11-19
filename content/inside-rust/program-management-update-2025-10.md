+++
path = "inside-rust/9999/12/31/program-management-update--october-2025"
title = "Program management update — October 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

# Program management update — October 2025

Lots of things happened in October! I've added a new section containing things that happened throughout the broader Project. I think it's useful to highlight these, but I don't want to conflate them with my own work. These posts also serve as accountability to the Project, so it's important to make that distinction.

This post also contains a long list of things I *haven't done*.

## Leadership Council meeting summaries

James Munns and Kobzol brought up that while having the Council minutes open is valuable, they are difficult to skim, and it takes time and effort to figure out what was discussed.

To help with this, I've started posting short summaries of the Council meetings to [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.20minutes.20.26.20summaries/with/548003969).

I'd also like to remind everyone that the meetings are generally open to anyone who's interested ([they happen every two weeks on Fridays from 11:30 a.m. to 12:30 p.m. in the `America/New_York` time zone](https://github.com/rust-lang/calendar/blob/main/council.toml)).

The topics discussed are tracked publicly as [issues in the `leadership-council` repository](https://github.com/rust-lang/leadership-council/issues), and that's where the [minutes live, too](https://github.com/rust-lang/leadership-council/tree/main/minutes/sync-meeting/).


## Project Directors selection follow-up

A month ago, the Council selected three new Project Directors to join the board:

* Niko Matsakis
* David Wood
* Jack Huey

They replaced Scott McMurray, Jakob Degen, and Santiago Pastorino. You can read more [in the Foundation's update post](https://rustfoundation.org/media/introducing-the-rust-foundations-newest-project-directors-october-2025/) and the [announcement on the Rust Blog](https://blog.rust-lang.org/2025/10/15/announcing-the-new-rust-project-directors-2025/).

Niko, David, and Jack, congratulations!

As the facilitator of the process, I let all the candidates know right away and relayed feedback to those who weren't selected.

I've started a retrospective on the selection process and collected feedback on it. I'll put together a summary and propose updates and clarifications in the form of PRs [against the `leadership-council` repo](https://github.com/rust-lang/leadership-council/).


## Project Goals

Niko wrote the [blog post announcing the 2025 H2 goals](https://blog.rust-lang.org/2025/10/28/project-goals-2025h2/), but then got busy, so I addressed the outstanding feedback and got it published. We've set it up so I'm a collaborator on his blog fork. Next time, I'll be able to make edits to his goal PRs without having to wait for him. That should let us move more quickly.

Niko also added a [Reports section](https://rust-lang.github.io/rust-project-goals/2025h2/reports.html) to the [Rust Project Goals website](https://rust-lang.github.io/rust-project-goals/). The reports are automatically populated from the comments in [goal tracking issues](https://github.com/rust-lang/rust-project-goals/issues?q=is%3Aissue%20state%3Aopen%20label%3AC-tracking-issue).

We're meeting regularly and have mapped out the next steps: I'll keep publishing the monthly updates (the September one slipped, but [we have the PR open now](https://github.com/rust-lang/blog.rust-lang.org/pull/1731)), and we came up with a few ideas for running the program next year that [Niko will write up. We'll then discuss it with people who are interested](https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/2026.20plans/with/553239242).

We're thinking about either switching to an annual cadence or shifting the goal periods to start in April and October, with planning happening six weeks prior and the RFC being opened at the beginning of April or October. We'll bring this up to the Project and listen to its suggestions and any other feedback.

As mentioned before, I will be more directly involved in this.

### `build-std` RFCs posted!

`build-std` is a feature that has existed on nightly for a really long time and has been desired for even longer.

It is the ability to build the Rust standard library (std, core, etc.) on stable Rust.

This has taken a very long time, in part because many people need this but have very specific requirements, and so there have been a plethora of proposals and experiments.

Starting this year, we've had regular meetings and design sessions with representatives from various teams. David Wood has now posted three (out of five) RFCs outlining the first steps. If this is something you're interested in, go take a look!

* [build-std: context (rfc#3873)](https://github.com/rust-lang/rfcs/issues/3873) lays down the history and context of the various endeavors in this space
* [build-std: always (rfc#3874)](https://github.com/rust-lang/rfcs/issues/3874) adds a Cargo configuration option to trigger the rebuild of the standard libraries your code depends on
* [build-std: explicit dependencies (rfc#3875)](https://github.com/rust-lang/rfcs/issues/3875) allows setting standard library crates as dependencies in `Cargo.toml` explicitly

You can also take a look at the [build-std Zulip channel](https://rust-lang.zulipchat.com/#narrow/channel/516120-project-goals.2F2025h1.2Fbuild-std).

## Inaugural FLS team meeting

In March 2025, the Project [took over the development and maintenance of the FLS](https://blog.rust-lang.org/2025/03/26/adopting-the-fls/). It is a document (originally called the Ferrocene Language Specification) that describes Rust for the purposes of safety-critical work (e.g., the automotive industry). It is used to qualify toolchains based on Rust. The FLS was developed by Ferrous Systems and AdaCore and eventually donated to the Project.

This month, a new team was [chartered with maintaining and improving the document for the benefit of the Project and the safety-critical community](https://github.com/rust-lang/fls-team/blob/main/CHARTER.md). Before this, maintenance happened as part of the Spec team. There is a desire to define a specification covering the Rust language as a whole, but the FLS is a document that's in use today, describing Rust as it is *implemented* today (in contrast to the *intended* behavior, which is what a spec would typically spell out) with an eye towards safety-critical qualification.

On Friday, 2025-10-31, the team had its inaugural meeting, which I joined and took notes. We expect that at some point, a lot of the discussion will be geared towards writing and reviewing pull requests and improving the structure of the document, where my presence won't be as valuable.

Indeed, this is how the team maintaining the Rust reference operates, and there hasn't been a need for my involvement there yet.

But for now, I'm attending the Spec and FLS meetings, taking notes for both, and making sure things don't get dropped on the floor while the new structures are being formed.


## T-content

We've started editing and publishing the RustConf interviews. We have a few out:

* [Jan David Nose on the Rust Infrastructure Team](https://www.youtube.com/watch?v=r7i-2wHtNjw)
* [Yuri Astrakhan on Rivian VW Tech and MapLibre](https://www.youtube.com/watch?v=MgGsZmBNLO0)
* [Jack Huey and Niko Matsakis on the Rust Vision Doc team](https://www.youtube.com/watch?v=8iQN49ktdBo)
* [Bart Massey on Rust Embedded Working Group](https://www.youtube.com/watch?v=-o8W6wcJlBY)

And more will be coming soon. You can check out the [full playlist for the Rust Content videos on YouTube](https://www.youtube.com/playlist?list=PL85XCvVPmGQjYASSxbFI7tf7lts-6bDCi).

I haven't done any editing myself (yet?), but I've listened to them and given feedback. The recordings work really well as audio-only, too, and we've been thinking about releasing them as podcasts as well as recording interviews remotely (the Content team encompasses far more than just conference interviews).


## `fmt::Write` infrastructure

There have been multiple proposals (e.g., [ACP: Add API to write formatted data directly into a `Vec<u8>` libs-team##651](https://github.com/rust-lang/libs-team/issues/651) and [Add fmt::Write to io::Write adapter libs-team#133](https://github.com/rust-lang/libs-team/issues/133)) to extend the write and formatting machinery.

Rust has two `Write` traits: [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) for writing out binary data into things like files and network devices; and [std::fmt::Write](https://doc.rust-lang.org/std/fmt/trait.Write.html), which [also exists in `core`](https://doc.rust-lang.org/core/fmt/trait.Write.html), for writing UTF-8–encoded text. There's clearly *some* commonality there, but the traits accept different things and have different behaviors (e.g., `fmt::Write` is not flushable) and error types. They're broadly not compatible with one another.

This can get annoying if you want to write formatted UTF-8 text to a bytes interface (e.g., `io::Stdout`, a file, or a `Vec<u8>`).

The `fmt::Write` trait allows one to write individual characters, strings, and formatted strings into types that implement it. However, no such implementation exists for byte sinks. But Rust's strings have a well-defined byte-based representation (`String` is literally just a wrapper over `Vec<u8>`, after all). That means you should be able to write string data into anything that accepts bytes without unnecessary ceremony.

Last month, there was an interesting discussion in the Libs-API Triage meeting on how we could substantially improve the situation, but because that got deep into the weeds, I scheduled a separate meeting where Josh Triplett and Amanieu d'Antras could talk it through.

Here's what they came up with:

Define a new trait (tentatively named `WriteFmt`) with a method that writes a `&str` into it. It would have an *associated* error type (i.e., the error is part of the trait definition, and when you `impl` it, you specify the error type, too). And define a corresponding macro modeled after `write!`.

It could look something like this:

```rust
pub trait WriteFmt {
    type Error;
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;
}
```

This would let both `io` and `fmt` use the same underlying trait. The associated error would let us specify that, e.g., writing to `String` or `Vec<u8>` would return a `Result<(), !>`. Since such a call by definition can't return an error, you wouldn't have to handle the `Result` in that case, and you wouldn't need to handle the "unused `std::result::Result` that must be used" warning.

This would require a different macro equivalent to the current `write!`, but the hope is that we could switch from the old one to the new one over an edition.

The next step is to write an RFC.

I'm excited about this. It's a clean proposal that unifies everything rather than adding workaround methods or adapter traits. It's a great example of "stability without stagnation."


## Hiring a second program manager

The Leadership Council is [asking the Foundation to fund the Project Priorities budget in 2026](https://github.com/rust-lang/leadership-council/issues/234). This is a pool of money that the Council would have at its discretion to pay for things it deems worthwhile for the Project. It is from this budget that the travel grants, program manager, and compiler ops roles are being paid.

In addition to being able to continue these initiatives, the Council is considering funding a second Program Manager.

Be aware that *this is a proposal*; the funding has not been approved, and no role has been posted. It makes a lot of sense to me, and it might be a good idea to explain why.

Right now, this role has a bus factor of one. If I were to disappear (leave, be let go, or have a severe health issue), it would put the Project into a bind. Hiring a new person takes time, and training can be very time-consuming. For as long as the Project sees value in this role, losing its program manager would be a significant disruption.

Beyond that, even a short-ish absence can be (and has been) felt already. I will take breaks, go on holiday, attend a conference, and get sick. During those times, a lot of the work I'm doing would either not be done at all or would be done with difficulty.

Having two people helps because one of them can temporarily handle everything or at least make sure the most important things don't get dropped.

It also means that I would be able to train the new PM without putting a burden on the teams.

If we find someone in a different time zone, even better! I'm based in Central Europe, and having someone in the Americas or East Asia to handle the American afternoons would be quite useful.

Finally, I am at capacity with my work. Between the meetings, follow-up, spending more time on Project Goals, and all the other things on my plate, I can keep going, but there are things the Project would like to get done that are becoming difficult to slot in.

Here are some of the things I'm struggling to find time for:
* Regular reporting of Rust Foundation teams' work
    * The Foundation has several people working full-time on various useful initiatives: crates.io, security (code signing), infra, and C++ interop.
    * The Infra team posts regular updates, but the work of the other teams is less visible.
    * I'd like to reach out to each team and make sure regular updates are published.
    * Ideally, these would be regular conversations, possibly with the PM penning the updates.
* Rust beginner resources
    * This is something that came from our Bevy discussions.
    * A lot of newcomers don't have any programming background, much less any experience with Rust.
    * Conversely, a lot of even beginner-targeted resources expect the reader to be familiar with programming in general.
    * There may be broader interest between gamedev, Rust Edu, etc., to pool existing resources together and build more beginner-friendly education resources through the lens of building games.
* GPU support for scientific computing and machine learning
    * People are working on this, we should reach out and see if there's any support from the Project they would want.
* Rust Editions
    * Support future editions, and help with tooling, updates, calls for testing, etc.
    * This is a brand-new endeavor I will pick up, but I don't have a lot of time to work on it right now.
    * When it comes, I'll have to reprioritize my work to support this, but splitting the goals and editions burdens between two PMs would be really helpful.
* Publish a blog post about the Style team: its history, what it does, and a call for contributions
    * The Style team has been resource-starved for a long time, and some of the rustfmt work is being blocked by it.
    * This needs Caleb and me to find some time to talk (scheduling this has been difficult).
    * After our discussions, I'll write a blog post explaining what's going on and hopefully drive interested people to contribute.
* Deeper engagement with the Content team (interviewing, editing, and writing)
* Write a post about how the Foundation, Project Directors, and Leadership Council all fit together, what they do, etc.
* Write a summary of what the various Rust teams do, how they operate, and how to get involved
    * I've met people who think they want to contribute to the compiler, but their actual interest lies in, e.g., the libs or lang teams.
    * There's been interest in clarifying the situation, giving an overview, and linking to each team's onboarding docs, issues, etc.
* Write a Rust Glossary and Pronunciation Guide
    * Rust has a lot of terminology that Project members are broadly familiar with but outsiders might not be.
    * This would be an official document that would explain that, e.g., `fn foo() -> Vec<&'static str>` is "a function foo that returns a vec of static string slices," that `&[(Point, Tile)]` is a "slice of tuples of Point and Tile," or that `&LinkedList<T>` is a "shared reference to a linked list of T."
    * It would also cover how these things are commonly pronounced (`VecDeque` = "vec deck", `char` either as in **char**_acter_ or **char**_coal_), etc.

Others I am actively working on, but they're going slowly:
* Share the retrospective on the Project Directors election and propose clarifications in the docs.
* Publish the C++ interop report from RustConf.
* Improve the Rust Project Goal tooling (automation, CI, rendering/stylistic fixes, and new views into the data).

So yes, as far as I'm concerned, having another PM would be really useful.

## Worth a look

There have been a few announcements that I had no direct involvement in but that may be good to be aware of.

### All Hands 2026 travel survey reminder

There's going to be another All Hands in 2026! \o/

It will take place in Utrecht, the Netherlands, as part of RustWeek, hosted by RustNL (just like in 2025).

If you're thinking of going, please fill out the [Rust All Hands 2026 Interest Form](https://forms.gle/GhkvDSfdBaYHUrRJ7). If you need financial support to get there, you can put that in, too. The organizers can't make any promises now, but they'll do their best to make sure everyone can join.

You can read more in [Mara's announcement post](https://blog.rust-lang.org/inside-rust/2025/09/30/all-hands-2026/).

### Funding Rust maintainers

The question of supporting Rust maintainers in a sustainable fashion has been brought up again.

Features typically garner more attention, and they're often what sponsors are looking to fund, but without code reviews, fixes, and general maintenance, features will become slow to impossible to land. There are several highly desirable capabilities that are waiting for the new trait solver or borrow checker capabilities, for example. There are also teams that rely on one or two people, which makes it difficult to even stay afloat, much less add things Rust engineers and companies care about.

To support people doing this work, the Rust Foundation [announced it will set up a maintainers fund](https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/).

Similarly, [RustNL is seeking sponsors to support maintainers as well](https://rustnl.org/fund/).

These efforts sprung up independently, but their timing is not a coincidence. There have recently been a few instances of long-term Rust maintainers losing their jobs (putting their ability to continue their fantastic contributions at the same level in question), and this is an ongoing topic in free and open source software in general.

I'm really excited to see this and hope that this might be something we can get sponsors to provide funds for as well (donating money can be easier than employing a person dedicated to software maintenance).

### Seven new Compiler team members!

The Compiler team keeps growing:

<https://blog.rust-lang.org/inside-rust/2025/10/28/compiler-team-new-members/>


### Infrastructure Team 2025 Q3 Recap and Q4 Plan

The Infra team posted its quarterly update and plan:

<https://blog.rust-lang.org/inside-rust/2025/10/16/infrastructure-team-q3-recap-and-q4-plan/>

There are lots of interesting bits there: the infrastructure got more robust (critical data backups on two cloud providers, alerts set up for CDN issues), developer experience improved, and <https://rust-lang.org/> is now a static website.

Give it a look!


## Stats

Total words of meeting minutes written: 224.2k (June–October)

Meetings attended: 32

Total words of meeting minutes written (October): 54.3k

Average (mean) word count per team meeting:

* Cargo: 2.3k
* Lang triage: 5.6k
* Libs-API: 4.7k
* Leadership council: 3.5k
