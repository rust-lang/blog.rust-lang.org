+++
path = "2026/03/20/rust-challenges.md"
title = "What we heard about Rust's challenges, and how we can address them"
authors = ["Jack Huey"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-project-vision-doc-2025"
+++

When we set out to understand Rust's challenges, we expected to hear about the borrow checker learning curve and maybe some ecosystem gaps. Of course, we did. A lot. But, of course, it's more nuanced.

The conventional wisdom is that Rust has a steep learning curve, but once you "get it," smooth sailing awaits. We found that while some challenges disappear with experience, they are replaced with others. Beginners struggle with ownership concepts, experts face domain-specific challenges: async complexity for network developers, certification gaps for safety-critical teams, ecosystem maturity issues for embedded developers.

This isn't all doom and gloom though: we ultimately found that despite Rust's challenges, it remains necessary and desired:

> If all the things laid out [to make Rust better] were done, I'd be a happy Rust programmer. If not, I'd still be a Rust programmer. -- Engineering manager adopting Rust for performance

## The universal challenges that affect everyone

Across every interview, regardless of experience level or domain, we heard about the same core set of challenges. These aren't beginner problems that go away—they're fundamental friction points that manifest differently as developers grow.

### Compilation performance: the universal productivity tax

**Every single cohort** we analyzed—from novices to experts, from embedded developers to web developers—cited compilation times as a significant barrier to productivity:

> "Java takes about 100 milliseconds, Rust anywhere from 5 seconds to a minute depending on what you changed" -- Distinguished engineer working on backend systems at a large company

> "8 to 10s iteration cycle... when you want to tweak the padding on a box" -- GUI development team

The impact varies by domain, but the pattern is consistent. CLI tool and GUI developers, who need rapid iteration cycles, are hit hardest. Safety-critical developers with 25-30 minute build times face workflow disruption. Size-constrained embedded developers are forced into optimized builds that take longer to compile and complicate debugging.

What's particularly important to note, is that this isn't just about absolute build times; it's about the **development velocity tax** that compounds over time. Long compile times can have strong negative impact on code iteration time. Anything that can reduce this code iteration time - hot reloading, fast debug builds, faster linking - will have an outsized impact on development velocity.

Moreover, the compilation performance tax compounds at scale. Individual developers might tolerate 5-10 second builds, but teams with CI/CD pipelines, large codebases, and frequent iterations face exponentially worse impacts. One participant noted 25-30 minute builds that create "wait for 30 minutes before the tool finds out I made a mistake" cycles.

### The borrow checker: first it's sour, then it's sweet

The borrow checker is often touted as a "beginner problem", and we found that this is largely true: Novices are most strongly impacted by the borrow checker, but this often extends even into the stage where a developer is *comfortable* writing Rust where they *still* get tripped by the borrow checker sometimes.

However, highly-experienced Rust developers basically never cite the borrow checker itself as a frustration for them.

> Ownership: The first time I went through the chapter, I was really like, what is this? - Developer learning Rust as a first language

> I actually did not understand the borrow checker until I spent a lot of time writing Rust - Executive at a developer tools company

### Async complexity: the "Three Horsemen" problem

Multiple participants identified `async` as a pain point. Many people, not just beginners, often choose to completely avoid it, instead focusing on solely on sync Rust. This is because, for many, `async` Rust feels completely different.

> My biggest complaint with Rust is async. If we want to use [a tool], we're forced into that model...not just a different language, but a different programming model...I have zero [experience], I've been avoiding it. - Developer working on a security agent at a large company

Of course, those who *do* use it often share how complex it is, how it can feel incomplete in ways, or how it is difficult to learn.

> "When you got Rust that's both async and generic and has lifetimes, then those types become so complicated that you basically have to be some sort of Rust god" -- Software engineer with production Rust experience

> "My general impression is actually pretty negative. It feels unbaked... there is a lot of arcane knowledge that you need" -- Research software engineer

> "There's a significant learning gap between basic Rust and async programming... creating a 'chasm of sadness' that requires substantial investment to cross." -- Professional developer

The async complexity isn't just about individual developer experience: it is exacerbated by **ecosystem fragmentation and architectural lock-in**:

> "the fact that there is still plenty of situations where you go that library looks useful I want to use that library and then that immediately locks you into one of tokio or one of the other runtimes" -- Community-focused developer

This fragmentation forces architectural decisions early and limits library compatibility, creating a unique challenge among programming languages.

Of course, it would remiss to clarify that plenty of people *do* express positive sentiments of async, often despite the mentioned challenges.

### Ecosystem navigation: choice paralysis and tacit knowledge

The Rust ecosystem shows **uneven maturity across domains**: excellent for CLI tools and web backends, but significantly lacking in other domains such as embedded and safety-critical applications. This creates a fragmented adoption landscape where Rust's reputation varies dramatically depending on your domain.

> "Biggest reason people don't use Rust is that the ecosystem they'd be entering into is not what they expect. It doesn't have the tooling that C++ has nor the libraries." -- Developer for a large tech company

> "I think the amount of choice you can have often makes it difficult to make the right choice" -- Developer transitioning from high-level languages

> "the crates to use are sort of undiscoverable... There's a layer of tacit knowledge about what crates to use for specific things that you kind of gather through experience" -- Web developer

The problem isn't lack of libraries—it's that choosing the right ones requires expertise that newcomers don't have.

The Rust Project has made this choice mostly intentionally though: it has chosen not to bless certain crates in order to not unduly stifle innovation. The expectation is that if a newer crate ends up being "better" than some well-established crate, then that newer crate should be become more popular; but, if the Project recommends using the more established crate, then that is less likely to happen. This is a tradeoff that might be worth reevaluating, or finding clever solutions to.

## How challenges amplify differently across domains

While the core challenges are universal, different domains have unique challenges that ultimately must be either adoption blockers or acceptable trade-offs.

### Embedded systems: where every constraint matters

Embedded developers face the most constrained environment for resources, which can amplify other challenges like learning.

> "if you pull in a crate, you pull in a lot of things and you have no control" -- Embedded systems researcher

> "can't use standard collections like hashmaps" -- Embedded software engineer

Debug builds become too large for small controllers, forcing developers into optimized builds that complicate debugging. Cross-compilation adds another layer of complexity. The "no-std" ecosystem, while growing, still has significant gaps.

### Safety-critical systems: stability vs. innovation tension

Safety-critical developers need Rust's memory safety guarantees, but face unique challenges around certification and tooling:

> "we don't have the same tools we have to measure its safety criticality as we do in C++ and I think it's a worry point" -- Safety systems engineer

> "not a lot of people know Rust not a lot of managers actually trust that this is a technology that's here to stay" -- Safety-critical developer on organizational barriers

The tension between Rust's rapid evolution and safety-critical requirements for stability creates adoption barriers even when the technical benefits are clear.

To note, we [previously](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/) wrote a blog post all about safety-critical Rust. Check it out!

### GUI development: compile times inhibit iteration speed

GUI developers need rapid visual feedback, making compilation times particularly painful:

> We've got a UI framework that's just Rust code so when you want to tweak the padding on a box ... it's a pain that we just kind of accept a 10 seconds or more iteration cycle. -- Developer working on a GUI app

## Background-dependent learning paths

One important insight gained from this work, and it seems obvious if you think about it, is that learning Rust isn't a universal experience: it depends heavily on your background:

**High-level language developers** must learn systems concepts alongside Rust:

> The challenge for me was I needed to grasp the idea of a lower-level computer science ideas and Rust at the same time. -- Developer with Typescript background

**Low-level developers** often struggle to unlearn patterns and concepts:

> I'm coming from C++ world so I had the big class that does everything. Taken a while for me to internalize that "dude you gotta go down a level". -- Developer with C++ background

> Rust tried to hide away notion of pointers - Just tell me it's a pointer -- System-level developer

Interestingly though, learning Rust alongside C++ can help students understand both better:

> Students learn smart pointers in C++ and then 'we're just now learning smart pointers with Rust as well' — learning both at the same time makes it easier. -- Community organizer

## Recommendations

### Invest in compilation performance as a first-class concern

Given that compilation performance affects every single user group, we recommend treating it as a first-class language concern, not just an implementation detail. This could include:

- **Incremental compilation improvements** that better match developer workflows
- **Build system innovations** that reduce the iteration cycle tax
- **Tooling integration** that makes build times less disruptive

We do want to quickly shout a couple of neat community projects that have this goal in mind:
- The [subsecond crate](https://crates.io/crates/subsecond) by the Dioxus team allows hot-reloading, which can make workflows like those found in GUI development more seamless
- The [Wild linker](https://github.com/davidlattimore/wild) aims to be a fast linker for Linux, with plans for incremental linking

### Invest in ecosystem guidance and compatibility

We [previously](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/#help-users-get-oriented-in-the-rust-ecosystem) made some suggestions in this area, and they still hold true. Finding ways to not only help users find crates that are useful to them, but also enable better compatibility between crates will surely have a net-positive benefit to the Rust community.

### Address learning diversity

When someone is learning Rust, their programming language background, level of experience, and domain in which they are trying to work in, all influence the challenges they face. We recommend that the Rust Project and the community find ways to *tailor* learning paths to individuals' needs. For example, for someone with a C or C++ background, it might be useful to be able to directly compare references to pointers.

Similarly, having domain-specific learning materials can help newcomers focus on the problems they are facing more specifically than a general "Rust tutorial" might. The [Embedded Rust Book](docs.rust-embedded.org/book/) does this, for example.

### Close the gap between sync and async Rust

This is a tall order -- there are a lot of moving parts here, but it's clear that many people struggle. On one hand, async Rust feels often "incomplete" in some language features compared to sync Rust. On the other, documentation is often focused on sync Rust (for example, much of [The Rust Programming Language Book](https://doc.rust-lang.org/book/title-page.html) is focused on sync code patterns).

Within the Rust Project, we can work towards stabilizing long-awaited features such as async functions in dyn traits, or improving compiler errors for issues with, for example, lifetimes and async code. We can include fundamental async library traits and functions within `std`, enabling a more cohesive async ecosystem.

Of course, as much as can be done *within* the Rust Project, even getting more community involvement in producing tutorials, example code, and otherwise just sharing knowledge, can go a long way to closing the gap.

## Conclusion

Rust's challenges are more nuanced than the conventional "steep learning curve" narrative suggests. They are domain-specific and evolve with experience.

Understanding these patterns is crucial for Rust's continued growth. As we work to expand Rust's reach, we need to address not just the initial learning curve, but the ongoing friction that affects productivity across all experience levels.

The good news is that recognizing these patterns gives us recommendations for improvement. By acknowledging the expertise gradient, prioritizing compilation performance, creating better ecosystem navigation, and addressing background-dependent challenges, we can help Rust fulfill its promise of empowering everyone to build reliable, efficient software.
