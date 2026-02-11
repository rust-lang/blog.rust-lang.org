+++
path = "inside-rust/9999/12/31/program-management-update-2026-01"
title = "Program management update — January 2026"
authors = ["Tomas Sedovic"]

[extra]
team = "the Goals team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-goals"
+++

Happy 2026! We got off to a great start with Project goals and hiring. I had a couple of weeks off in December, and that was really lovely.

## Project goals

The goals initiative is in full swing. Following the [call for proposals](https://blog.rust-lang.org/inside-rust/2025/12/16/please-submit-2026-project-goal-proposals/), people started opening [pull requests][rpg] for new goals. In addition, many of the current goals are planned to continue into 2026.

[rpg]: https://github.com/rust-lang/rust-project-goals/pulls

As of right now, we have around 60 goals proposed, with a few more to come. That's a lot! But we're moving to the second phase.

In February, we're inviting feedback on the proposed goals -- particularly from the teams being asked for something. You can look at the [Goals by team][goals-by-team] page, find your team, and see all goals that call it out. It's a good time to check whether these goals make sense to you, start thinking about your capacity, and consider whether you have champions who can sign up.

[goals-by-team]: https://rust-lang.github.io/rust-project-goals/2026/reference.html#goals-by-team

In March, we'll open the RFC for 2026 goals, which all the team leads will review. They'll look at the proposed goals in aggregate and sign off on the champions and team capacity.

And then from April to December, it's implementation season!

### 2026 changes

In addition to running the program throughout the whole year, we've made other changes.

Flagship themes gave way to [Roadmaps][roadmaps]. Many of the larger initiatives we want to achieve don't really match the six-month or even yearly cycle. We want goals to be achievable in a single year, but e.g., [completing the `async` features and ergonomics][async-roadmap] will likely take multiple years to complete.

[async-roadmap]: https://rust-lang.github.io/rust-project-goals/2026/roadmap-just-add-async.html

A goal can belong to more than one roadmap. 

Above roadmaps are [Application areas][application-areas] such as "Cross-language interop" or "Safety-critical & regulated" -- large initiatives that often align with a specific industry. We are hoping that the roadmaps and application areas can help focus industry funding. For example, an automotive company may be very interested in funding the "Safety-critical" area, as the code and libraries they use need to go through expensive attestation. On the other hand, companies with large C++ codebases will be more interested in "Cross-language interop".

[application-areas]: https://rust-lang.github.io/rust-project-goals/2026/reference.html#application-areas
[roadmaps]: https://rust-lang.github.io/rust-project-goals/2026/reference.html#roadmaps
[highlights]: https://rust-lang.github.io/rust-project-goals/2026/highlights.html

There are also [individual goals that we want to highlight][highlights] because we think people will be excited about them and we expect them to stabilize this year (note that the current list is likely to change while we look at all the proposed goals).

Getting all this in was a significant effort by Niko Matsakis, Rémy (lqd), and myself. I'd like to thank everyone for their submissions and for participating in the discussion!


## Hiring a second Program Manager

Originally, the Leadership Council planned to hire two Program Managers so there would be continuity, a backup, and enough capacity to cover the Project's needs. Unfortunately, in 2025, it hadn't been clear whether there would be enough budget to support two people, so the Council had opted to hire just one.

After getting the 2026 funding from the Foundation clarified, the Council [allocated extra funds to hire a second Program Manager][hiring]. This let us reach out to Nurzhan Saken, whom we'd hoped to bring on board along with yours truly back in June.

[hiring]: https://github.com/rust-lang/leadership-council/issues/255

I spoke to Nurzhan, saw why TC and Joel Marcey wanted to hire him in the first place, and I'm delighted to announce that Nurzhan joined the Project at the beginning of February!

I'll help him get on board, and I'm really excited about this.


## Rust for Linux

We've continued our regular meetings, and you can see the updates on the [Rust for Linux project goal tracking issue][rfl-tracking].

[rfl-tracking]: https://github.com/rust-lang/rust-project-goals/issues/116

Some highlights:

* [We now have a wiki][beyond-refs] that covers Field Projection, In-place initialization, and the other language features that we're planning to build.
* Ding Xiang Fei opened a 2026 Project goal for [Supertrait `auto impl`][supertrait-auto-impl].
* Gary Guo opened a [pull request](https://github.com/rust-lang/rust/pull/138618) implementing the [Pass pointers to `const` in assembly RFC#3848](https://github.com/rust-lang/rfcs/pull/3848).

[beyond-refs]: https://rust-lang.github.io/beyond-refs/
[supertrait-auto-impl]: https://rust-lang.github.io/rust-project-goals/2026/supertrait-auto-impl.html
[field-projections]: https://rust-lang.github.io/rust-project-goals/2026/field-projections.html
[in-place-init]: https://rust-lang.github.io/rust-project-goals/2026/in-place-init.html

I've opened a [pull request][rfl-roadmap] that proposes a new [Rust for Linux roadmap][rfl-roadmap] which ties in the existing standalone goals ([Field projections][field-projections], [In-place initialization][in-place-init], and [Supertrait `auto impl`][supertrait-auto-impl]) and breaks down the original language features into individual goals as well.

This way, we can track everything for Rust for Linux under a single roadmap item and also have smaller, more focused, and easier-to-review goals.

[rfl-roadmap]: https://github.com/rust-lang/rust-project-goals/pull/531


## CPython collaboration

We continued our weekly meetings with the Python folks interested in bringing Rust into CPython (the canonical Python interpreter, written in C). We have representatives from the Compiler, Cargo, Libs-API, and Language teams there, and we bring in additional experts where it makes sense.

Here are some of the topics we went through:

* Bootstrapping Rust currently relies on a Python script (`bootstrap.py`). If Python starts depending on Rust at some point, we'll have a cycle we'll need to sort out.
* Safely modeling Python objects that are attached/detached from Python threads within Rust. This is something PyO3 deals with, but the API is clunky, and there are soundness edge cases there. We discussed language features that could help here.
* Building/linking the Rust standard library to Python extension modules in a way that doesn't expose symbols that could conflict.
* Setting different linker arguments for `bin` and `lib` targets when both are present in a single crate.
* Potential interoperability between `async` Rust and Python's `asyncio`.
* Introducing Rust into large C codebases -- the technical as well as social aspects. For this, I reached out to the Rust for Linux folks, and Miguel Ojeda shared his perspective on introducing Rust into the Linux kernel.

Please note, this is all a preliminary discussion. Even the idea of adding Rust to CPython has not yet been proposed *formally*. One of the outcomes of the work here is a [Python Enhancement Proposal (PEP)][pep] (analogue to Rust RFCs) that does so.

[pep]: https://peps.python.org/


## cargo-script

This is one of my most anticipated features. [Cargo script][cargo-script] lets you take a single Rust file, specify any dependencies, and run it. You can think of it as a much more streamlined `cargo run` where you don't have to set up a whole directory structure with `Cargo.toml` and so on.

[cargo-script]: https://github.com/rust-lang/rfcs/pull/3502

In its simplest form, you can do:

```rust
#!/usr/bin/env cargo

fn main() {
    println!("Hello, world!");
}
```

(until it's stabilized, the actual shebang invocation is something like: `#!/usr/bin/env -S cargo +nightly -Zscript`)

And run it either with `cargo hello-world.rs` or (if you mark the file as executable): `./hello-world.rs`:

```
$ ./hello-world.rs
warning: `package.edition` is unspecified, defaulting to `2024`
   Compiling hello-world v0.0.0 (/home/example/tmp/hello-world.rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `/home/example/.cargo/build/23/c81d708245acde/target/debug/hello-world`
Hello, world!
```

Cargo will build the source and run the compiled binary.

This gets really powerful when you start including dependencies -- cargo script is able to parse a "front matter" section which is a subset of `Cargo.toml`:

```rust
#!/usr/bin/env cargo
---cargo
package.edition = "2024"
[dependencies]
chrono = "0.4"
---

fn main() {
    use chrono::Datelike;

    let now = chrono::Local::now();
    let days_in_month = now.num_days_in_month();
    let last_day = now.with_day(days_in_month.into()).unwrap().date();

    println!("This month has {days_in_month} days and the last day is: {last_day}");
}
```

And have it download and compile the dependencies and run the resulting script:

```
$ ./hello-world.rs
    Updating crates.io index
     Locking 31 packages to latest Rust 1.93.0-nightly compatible versions
      Adding android_system_properties v0.1.5
      Adding autocfg v1.5.0
   [...]
   Compiling num-traits v0.2.19
   Compiling chrono v0.4.43
   Compiling hello-world v0.0.0 (/home/example/tmp/hello-world.rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `/home/example/.cargo/build/23/c81d708245acde/target/debug/hello-world`
This month has 28 days and the last day is: 2026-02-28+01:00
```

In addition to writing "scripts", cargo script is *fantastic* for coming up with minimal bug reproducers and quick prototypes. The overhead of a few directories and an extra file doesn't *seem like that much*, but in practice, having a single file you can share or just include in a Markdown code block makes a huge difference.

This feature, in various iterations, has existed for years, but now (thanks to Ed Page's determination) it is tantalizingly close to shipping on stable.

Getting the front matter stabilized required working with the Style team to settle on all the formatting specifics and then implementing them. Once the Style team thought it was all set, I checked with Ed and learned that while everything was good there, the [front matter stabilization](https://github.com/rust-lang/rust/pull/148051) seemed stuck on the language side.

Early in December, the Lang team had a [concern about stray carriage-return](https://github.com/rust-lang/rust/pull/148051#issuecomment-3608436373) (CR, `/r`) characters causing confusion: they could be rendered as new lines, but the Rust tooling would not interpret them as such. Ed [opened a PR a few days later](https://github.com/rust-lang/rust/pull/149823), which sat there for weeks without comment.

I brought this up with Josh Triplett, and we invited Ed to a Lang meeting where the team discussed this, clarified the request and the reasoning behind it, and agreed on the next steps. Ed made the requested changes, and the front matter stabilization has passed the final comment period (FCP) a few days ago.

The [cargo script stabilization issue](https://github.com/rust-lang/cargo/pull/16569) is now in FCP as well, with no concerns listed.

I'm extremely excited about this.


## Crates.io mirroring and verification

While there is support for alternative registries, [crates.io](https://crates.io/) is what most users (and tooling, e.g., CI) interact with when downloading crates. This can put too much pressure on crates.io, and some people can't rely on it (for example, because they don't have enough bandwidth, a reliable internet connection, or firewalls preventing access).

Linux package registries have, of course, dealt with this as well, and most popular distros have package mirrors, so you rarely even hit the original source. These can be closer to you and may often back internal CI in companies.

One concern with mirrors -- especially with the security of the supply chain finally being taken more seriously -- is making sure they serve the same content as crates.io would: that there's no tampering either on the mirror's side or during transit.

Walter Pearce and Josh Triplett are [driving an effort to build such a verification system](https://github.com/rust-lang/rfcs/pull/3724) building on [The Update Framework (TUF)](https://theupdateframework.io/), and last year, we had a [Project goal tracking this work](https://rust-lang.github.io/rust-project-goals/2025h1/verification-and-mirroring.html).

This was moving forward, but aside from a few meetings, there was not a lot of external communication. This was frustrating for people who were following the work and wanted to build on top of it. This effort is also sponsored by the Foundation, and while they produce regular reports, they're not frequent or detailed enough for the needs of the Rust ecosystem.

I set up a meeting with a group of people interested in this work, and they clarified their expectations and came to an agreement. Here's what came out of it:

* We'll open a 2026 goal for this effort.
* Walter will be the point of contact, and he'll be sending regular updates.
* I've set up a fortnightly sync meeting with the interested people.
* The Program Managers (Nurzhan and/or I) will attend, take minutes, and make sure the updates are posted.

There's great work being done here, but we all need to work towards making it visible.


## Worth a look

### Rust Foundation posts

* [Announcing the RustConf 2026 Program Committee](https://rustfoundation.org/media/announcing-the-rustconf-2026-program-committee/)
* [How We Invested in Rust in 2025 — and What Comes Next](https://rustfoundation.org/media/annual-report-strategy-2025/)
* [Rust Foundation Member Announcement: Meilisearch & Doulos](https://rustfoundation.org/media/rust-foundation-member-announcement-meilisearch-doulos/)
* [Rust at Scale: What WhatsApp’s Security Journey Tells Us About the Future of Safer Software](https://rustfoundation.org/media/rust-at-scale-what-whatsapps-security-journey-tells-us-about-the-future-of-safer-software/)

### Rust blog posts

* [This Development-cycle in Cargo: 1.93](https://blog.rust-lang.org/inside-rust/2026/01/07/this-development-cycle-in-cargo-1.93/)
* [What is maintenance, anyway?](https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/)
* [Infrastructure Team 2025 Q4 Recap and Q1 2026 Plan](https://blog.rust-lang.org/inside-rust/2026/01/13/infrastructure-team-q4-2025-recap-and-q1-2026-plan/)
* [What does it take to ship Rust in safety-critical?](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/)
* [crates.io: development update](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/)
* [Announcing Rust 1.93.0](https://blog.rust-lang.org/2026/01/22/Rust-1.93.0/)
* [2025 Rust Foundation Annual Report Project Director Update](https://blog.rust-lang.org/inside-rust/2026/01/27/2025-rust-foundation-annual-report/)
* [First look at 2026 Project goals](https://blog.rust-lang.org/inside-rust/2026/02/03/first-look-at-2026-project-goals/)
* [January 2026 Project Director Update](https://blog.rust-lang.org/inside-rust/2026/02/09/project-director-update/)


## Stats

Total words of meeting minutes written: 323k (June–January)

Meetings attended: 40

Total words of meeting minutes written (January): 57.8k

Average (mean) word count per team meeting:

* Cargo: 1.9k
* Lang triage: 6.7k
* Libs-API: 6.9k
* Leadership Council: 3.2k
