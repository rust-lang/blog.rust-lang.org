+++
path = "2026/06/25/vision-doc-journeys-to-learning-rust"
title = "What the Rust Vision Doc tells us about the journey(s!) of learning Rust"
authors = ["Pete LeVasseur"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-project-vision-doc-2025"
+++

_This is another post in our series covering what we learned through the Vision Doc process. We previously [described the overall approach and what we learned about doing user research](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/), we [explored what people love about Rust](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/), [dug into what it takes to ship safety-crticial Rust](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/), and [described some of the major challenges that people face when using Rust](https://blog.rust-lang.org/2026/03/20/rust-challenges/)._

In this post we walk through what folks have found on their journey to learn the Rust programming language with ups and downs covered.

As a disclaimer, LLMs (Large Language Models) come up in this post because our interviewees brought them up. We're scoping discussion to their use as a learning tool, covering research and example generation, not broader questions about AI (Artificial Intelligence) in software development.

# Many paths to needing Rust

The interviews surfaced several different paths into Rust: curiosity, embedded work, job-market pressure, organizational adoption, and reassignment after a team or company chose Rust. That last path matters because many learners are not evaluating Rust from a blank slate; they are trying to become productive after Rust has already arrived in their work.

> "Funny enough, I've advocated for more niche languages than Rust in the past. Rust has pretty much stopped being as much of a niche language as it was, but it's not Java." -- Fractional CTO

# Rust learning resources

Likely as expected, the folks that we talked to reach for a range of resources to learn Rust. Some reach for official documentation, such as [The Rust Programming Language Book](https://doc.rust-lang.org/book/) and find that sufficient to build on what the compiler was already showing them.

> "I started with the official Rust documentation because there are a lot of great examples of how features like the borrow checker work." -- Software engineer at an Automotive supplier

Others needed more passes and more formats, sometimes reaching for resources the community maintains, such as [Rustlings](https://rustlings.rust-lang.org/), [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html), and [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).

> "The first time I went through the chapter in [The Rust Programming Language] on borrow checking, I was like, what is this? I read it again, then I watched a YouTube video of someone explaining the chapter." -- Rust freelance consultant

> "Rust book, Rustlings, Zero to Production in Rust, Jon Gjengset tutorials. A bunch of books. It's not a one-pass reading. Can't say how many times I've gone through it." -- Software engineer working on video streaming and storage

These resources have brought up an entire generation of Rust programmers. But, to some, there is a perception that these resources have trouble keeping pace with the langauge.

> "We'd like to use [The Rust Programming Language/'the book'], but we've found that it's out of date, unfortunately. We've looked at the GitHub repo and found it's got a lot of unresolved issues and unmerged PRs" -- Principal Software Engineering work on Rust adoption in a regulated industry

Whether or not this is factually true, Rust's growth has nonetheless put more scrutiny on these materials. Companies evaluating adoption and engineers getting reassigned to Rust teams are looking at them with fresh eyes and finding the gaps that affect their own evaluation.

# Beginner stumblings and unlearning habits

It's pretty typical for Rust to be the 2nd, 3rd or Nth programming language that someone picks up. They'd end up writing their most familiar language in Rust, whether C++ patterns, Java patterns, or whatever they knew, for months or even years. Eventually they got comfortable enough to start writing idiomatic Rust.

> "There's a bit of a drop in productivity compared to C if you're already familiar with it just because you're learning new rules, new syntax."  -- Principal Firmware Engineer (mobile robotics)

> "In the beginning it was more poking around the code and adding and removing some ampersands and asterisks to try to make sense of `mut` and not `mut` and whatever." -- Senior engineer with 20 years of Java experience in cloud and IoT

We also spoke with someone who found that not having much of a programming background seemed to benefit people picking up Rust. Not having worn-in grooves from other languages may play a role here, and it's worth investigating further.

> "I had someone who had never programmed much before start working on the internals of [our Rust project]. She was just fine with getting into Rust. It's more of the senior people that struggle as they need to unlearn practices which may work in other languages, but it's not the 'Rust' way." -- Researcher, Automotive OEM R&D Lab

# Learning to work with the borrow checker

We heard a lot about learning to work with the borrow checker instead of against it. People get there through different paths, but a few patterns came up repeatedly.

## The compiler as teacher

Rust's diagnostics did the teaching on their own, especially around lifetimes.

> "If you mess up the lifetimes in a piece of code that you've written by hand, I usually find that Rust's diagnostics are very helpful" -- Researcher working on static analysis of Rust programs

> "Whatever's missing, the compiler usually fills in: it tells me 'you need to declare the lifetime of this reference', so I know and can figure it out. That all generally works pretty well." -- Senior Software Engineer

## Learning by doing

Others felt like they only really internalized the borrow checker after writing a lot of Rust. It took projects, coding challenges, prototyping and so on until at some point it clicked.

> "I actually did not understand the borrow checker until I spent a lot of time writing Rust" -- Founder of a startup built on Rust

> "Besides the prototyping work, I also did coding-challenge-type stuff to get familiar with Rust for Advent of Code. [..] It eventually clicked to the point where I wasn't fighting with Rust, it was working for me. I had that experience other people describe: when I managed to get my program to fit with Rust, it worked. I didn't spend time debugging." -- Principal Software Engineer, large SaaS provider

## Letting go of "clone guilt"

Some learners arrive with the assumption that good Rust means zero clones, zero copies, lifetimes threaded through everything. They set the bar at optimal before they've learned how to write idiomatic Rust, and it makes the borrow checker feel harder than it needs to be at the outset.

> "On one of my first projects, I was like, 'I don't ever want to copy or clone anything,' so I carefully wove through all the lifetimes and got myself into a bit of a bind. Then I saw someone else just cloning the struct I was working with, and it was super cheap. Sometimes you can just clone and it's going to be okay." -- Researcher at a university

The experienced Rust developers we spoke with consistently said the same thing: clone freely while you're learning, then optimize when you understand the problem. Rust's reputation for performance and correctness feeds this. Newcomers assume anything less than optimal is wrong before they've written a first working program, and clone guilt is how that shows up.

# Multi-paradigm, but not the OOP some are used to

The Rust programming language is multi-paradigm, and how that lands depends on what you're coming from. We heard some that came from a functional background were delighted with digging into learning how much Rust inherits from that lineage. Some others noted that they and others on their teams struggled to unlearn the object-oriented style they'd come to use heavily in other languages like C++ and Java.

> "Developers coming from C++ tend to think object-oriented. I think that's a difference between C++ and Rust." -- Architect at Automotive OEM

> "I had exactly that thing, where I would apply all my years of Java and JS thinking, where I could just create some object, not care about it, return it, have it sloshing around between various functions. Found myself reaching for these patterns and then being told 'no, you cannot do that'." -- Principal Engineer at a SaaS company

Developers coming from functional programming had less to unlearn: strong typing, pattern matching, and an expression-oriented style were already familiar.

> "My background has been more functional programming, strong typing. That originated for me as a Lisper: once a Lisper, always a Lisper." -- Principal Software Engineer working on Rust tooling for safety-regulated industries

> "The languages I primarily used before Rust were things like OCaml. Way back, I came from C and C++, the classic languages, and then I spent quite a long time doing primarily pure functional stuff. These days I've ended up back in what I like to think of as a pragmatic center ground [with Rust]." -- Fractional CTO

# Teaching Rust in academia

We spoke with a university professor that's been teaching Rust generally and used it in an embedded development class. In the academic environment, they were able to use proxies for some things such as "traits are like interfaces in Java" because the students had already gone through a set of courses in their first and second years that taught them Java. They introduced concepts slowly throughout the course, choosing to deal with some more complex topics like generics later. The outcome generally was that students had no problem picking up Rust in this setting.

> "I couldn't see any big difference on the embedded side. We also teach an embedded class, and we did an experiment. Half of the students' feedback was worse on the Rust class, mostly because they needed to build the project themselves. The C students just got one from [an LLM], absolutely no problem." -- University Professor, on teaching Rust

The C cohort leaned on LLMs for the project in ways the Rust cohort couldn't. We don't yet have a clear answer for why.

What did come through clearly was the Rust cohort's experience with the community. Some students needed to figure out which drivers to use for the embedded project and how to use them. Their professor encouraged them to open issues and ask questions directly on GitHub, and the maintainers responded. Students who had never contributed to open source before were getting answers from the people who wrote the code.

# Learning using LLMs

Some experienced folks shared that they saw LLMs as a tool that can help someone come up to speed quickly, either as a research tool or for generating example Rust code to understand concepts.

> "I'm optimistic that there's a way to work [LLMs] in that will cut down that learning curve. One of the big things these tools bring is reducing the learning curve in general; these are very good tools to help you navigate a space that you don't know yet." -- Maintainer of large open source Rust crate

> "I try [LLMs] out once a month, usually for generating an example or something like this. Just like with Stack Overflow: when you read an example, you should read it carefully and try to understand it. Not copy and paste it, but type it in your own words in code and then check it, because that's where the teeny tiny little mistakes are." -- Founder of startup built on Rust

For some learners, an LLM is just another way to find answers, no different than a search engine.

> "So for the most part, picking up Rust - how do I learn? I'll [use web search for] things, I'll ask [an LLM], I'll just poke around and read the code." -- Senior Software Engineer working in a regulated space

One founder went further and claimed that LLMs change who can become a Rust developer. One consulting company founder described hiring high school graduates with no systems programming background and training them as Rust developers, with LLMs filling in the learning gaps that would previously have required years of experience.

> "At the beginning, I was worried, but now that we have [LLMs] supporting development, the difficulty of the language doesn't matter. I'm seeing a huge opportunity behind strong runtime languages like Rust. [..] In [Developing Country] we hire 20-25 high school graduates, train them to be Rust programmers, then they enhance our workforce worldwide." -- Founder of a consulting company

We heard this from one organization. This is a claim that the combination of Rust's compiler and LLM tooling can dramatically shorten the path from beginner to working developer. Whether it generalizes depends on questions we can't answer from a single interview: how long these developers stay, what kind of code they can maintain independently, and whether this training/learning model works outside this company's particular structure. If it holds up, the pool of people who can become Rust developers is much larger than the usual hiring profile suggests.

# Organizational considerations for Rust learners

We spoke with a number of folks on teams that are using Rust in larger organizations. Teams wanted to know that everyone would end up at roughly the same level of competence, which led a good number to invest in training courses to get there. Some leaders found that staff was able to ramp well enough by reading The Rust Programming Language, going through Rustlings, and then picking up lower risk and priority tickets to work on. Having a sense of community was also important within companies; it helps people know they are not alone when they are asked to work on Rust after, say, a reorganization happens.

> "[..] the idea with the class as opposed to 'just read the Rust book on your own' was that this gives everyone kind of the same baseline going in."  -- Principal Firmware Engineer (mobile robotics)

> "So typically we're going to have people work through Rustlings, work through The Rust Programming Language. We have them then start to pick up lower risk tickets to work on." -- Principal Engineer at a large SaaS provider

> "We've got an internal Slack channel for Rust learning where people can drop questions and others will come in and answer them. That helps build up understanding and community." -- Software Engineer at a large corporation

Some organizations found that while the person they'd hire would need to learn Rust, it was still preferable to the alternative of hiring someone for a critical piece of software written in another language.

> "They needed to grow and maintain this C++ codebase. They had a C++ wizard, and they tried for about two years to find someone with the same level of expertise. They ended up hiring people that didn't know Rust and ramping them up, creating FFI bindings from the C++ side so they could work in Rust. And you can feel it: the borrow checker is teaching these people the right way to handle their systems." -- Principal Engineer at an Automotive OEM

The community and helping each other aspect seems to grow bonds as organizations mature.

> "Our team is [all about] mentorship. I've mentored people coming up to speed on Rust, and people help each other hugely." -- Principal Software Engineer at a large SaaS company

# Silent attrition

We identified some cases where people have approached Rust and bounced off of it, for one reason or another. In the below case, someone with a background in a language with fewer guardrails found themselves frustrated enough with Rust to walk away.

> "All of that means that that embedded ecosystem is very frustrating to somebody who comes from C and is like, why can't I just get a pointer to this peripheral and then write into the registers. What are you doing to me? [..] My friend never got over that. He looked at it and said, I'm not going to deal with this and walked away." -– A second University Professor

There may be language features that for a particular domain are not seen as comfortable or usable yet, such as async Rust usage in a safety domain. We'd like to map which language features feel off-limits in which domains; async in safety-critical work probably isn't the only case.

> "We're not fully sure how async [Rust] will work out in the long run in our domain. [..] People don't feel comfortable yet since C++14 doesn't provide such concepts. [..] It's the chicken-and-egg problem again: we probably need to gain some experience to see whether we can actually benefit from these new concepts in the automotive and safety domains." -- Team Lead at Automotive Supplier (ASIL D target)

We heard in at least one case, that while the language was challenging and there was a near bounce, the tooling helped keep them coming back and trying.

> "Well, I think my early impressions of Rust - one is I find C++ so intimidating, and I think a big part of why I was able to succeed at [..] learning Rust is the tooling. I mean, all this makes sense [..] but it's like, for me, getting started with Rust, the language was challenging, but the tooling was incredibly easy." -- Founder of another startup built on Rust

While it might be considered more of a community concern, if there are interactions online and in spaces that point to learners having 
so-called "skill issues" this feeds into the narrative that Rust must be hard to learn. We may be unintentially turning away Rust Project contributors and maintainers due to the vibes being put out when new learners show up in certain spaces.

> "People are very helpful, but generally the attitude is: if your program is very complicated, it's mostly a skill issue. There's not that much empathy when people get stuck learning, and a lot of people are just pushed away by it. There's probably a huge number of people who silently stop wanting to write Rust, because at some point it gets complicated and the feedback they get is 'you just need to be a better programmer, obviously'." -- Software Engineer at a SaaS Provider

## Feedback on near-bounces from survey

We found a few interesting perspectives collected in the Rust Vision doc survey which we administered with examples of bouncing and coming back:

> "I started before 1.0, got stuck very soon when trying to translate patterns from C++ to Rust (due to borrow checking). I tried again after 1.0 and it stuck. [..]" -- Survey Respondent A

Survey Respondent A went on to share in a more detailed response about a perceived weakness in Rust learning materials related to lifetimes and the borrow checker are explained. There was an observation that it's fairly easy to run into more complex situations with lifetimes and the borrow checker. They felt that the current state of this sort of material and tutorials is fairly superficial and can leave learners stuck when they run into those more complex situations.

One respondent that bounced once and came back shared challenges around usage of async. In concert with Rust's memory-safety and the borrow checker, they found some of the nitty-gritty details of async were difficult to learn. While we're aware of the Rust Project's continuous efforts to improve Rust's async story, this is another data point of a user that faced challenges.

Another survey respondent shared how they had multiple times bounced in trying to learn Rust. They returned after a year or so and found Rustlings to be highly motivating. We note that having multiple pathways for folks to learn Rust opens up more possibilities for those that nearly bounced, just like this person.

## Need more focused work on silent attritrion

The thing that stood out most to us was the lack of real, first-hand knowledge of having bounced when learning Rust. While this is an obvious effect of soliciting answers to our survey and opportunities to interview through Rust channels and our networks, this cohort is good future candidate where interviews could start.

# Conclusions

Across these conversations, the experience of learning Rust depended heavily on context. Why someone was learning and what support they had mattered as much as the borrow checker. The same kinds of examples kept coming up: a training course that got a team to a shared baseline, a maintainer answering a student's first GitHub issue, and a colleague whose code showed that cloning was okay.
 
That context is largely something the community has a hand in. With that in mind, here is what we take away from what we heard, and what we still don't know.
 
## What seems worth trying
 
**Learning materials aimed at unlearning.** Syntax barely came up when people described their struggles. People struggled with unlearning habits from previous languages, whether OOP structuring from C++ and Java or the instinct to grab a raw pointer to a peripheral. Most of our learning materials teach Rust from first principles, and that works. What we didn't come across is much written for, say, the engineer with ten years of Java who lands on a Rust team after a reorg: material that names the patterns they'll reach for that won't transfer, and shows what to do instead. The professor we spoke with did a version of this in the classroom, leaning on "traits are like interfaces in Java" and saving generics for later in the course, and the students did fine. Something similar could work outside the classroom too.
 
**Put the "clone freely while you're learning" advice somewhere official.** Every experienced developer we spoke with gave the same advice, but learners seem to mostly pick it up by accident, like the researcher who happened to see someone else cloning the struct they had been carefully threading lifetimes through. Saying it early in official materials would take some of the steepness out of the curve. The broader version belongs there too: idiomatic Rust doesn't have to mean optimal Rust, especially on a first project.
 
**Diagnostics are already a primary learning resource: several people told us the compiler taught them lifetimes before any documentation did.** Diagnostics reach learners right at the moment they're stuck. When writing new ones, it seems worth keeping the confused newcomer in mind alongside the expert, because for a lot of people this is where the learning happens.
 
**Is "the book" actually out of date?** Whether or not The Rust Programming Language or other materials are actually behind, a team evaluating Rust looked at its repository, saw unresolved issues and unmerged PRs, and moved on. As more companies evaluate adoption, more people will look at these materials with the same fresh eyes. Visible issue triage and some communication about what's current and what's planned would address the perception, separately from whatever content work may or may not be needed.
 
**How stuck learners get treated is shaping who stays.** We heard about students getting answers on GitHub from the maintainers who wrote the code, and we heard about learners being told their struggles were a skill issue. The first group came away with a lasting good impression of Rust. Some of the second group walked away entirely, and because they leave quietly, it's easy to underestimate how many of them there are. The welcoming side of the community came up unprompted as a reason people stayed, so we know it makes a difference when we get this right.
 
**Every organization we spoke with described essentially the same ramp-up for bringing a team to Rust.** Teams that brought groups of developers to Rust described roughly the same approach: get everyone to a shared baseline with a training course or with The Rust Programming Language and Rustlings, start people on lower-risk tickets, and give them somewhere internal to ask questions. Several organizations also found that hiring developers without Rust experience and ramping them up worked out better than continuing to search for rare expertise in another language. None of this is complicated, and teams weighing adoption don't need to invent a training program from scratch.
 
## What we still don't know
 
The biggest gap is the people we didn't reach. Nearly everyone we spoke with stuck with Rust long enough to be reachable through Rust channels, so the stories of bouncing off came to us second-hand: a friend who walked away from embedded Rust, colleagues who quietly stopped after the responses they got. As we wrote in [our first post](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/), finding people who decided against Rust takes targeted outreach. If the proposed User Research team comes together, talking with learners who bounced would make a good early project, and learning is probably the area where that research would teach us the most.
 
We also don't know what to make of LLMs as a learning tool yet. They came up as a search engine, as an example generator, and in one organization's case as something that makes training high school graduates into working Rust developers possible. We saw a classroom where the C cohort leaned on LLMs in ways the Rust cohort couldn't, and we don't have an explanation for it. All of this comes from a handful of conversations, so we treat it as a set of leads to follow up on. Given how quickly the tools are changing, it seems better to study this deliberately than to wait and see what folklore develops.
 
The folks we spoke with showed that people do get there: with enough passes through the materials and enough code written, it eventually clicks. The opportunities above are mostly about making it work for the people who didn't pick Rust on purpose, and for the ones who would have stuck around if their early experience had gone a little differently.
