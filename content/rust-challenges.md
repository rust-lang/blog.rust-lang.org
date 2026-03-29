+++
path = "2026/03/20/rust-challenges"
title = "What we heard about Rust's challenges"
authors = ["Jack Huey"]
aliases = ["2026/03/20/rust-challenges.md"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-project-vision-doc-2025"
+++

*Author's note*

The [original version](https://github.com/rust-lang/blog.rust-lang.org/blob/786db4b4d202d85ef79741de5be4cee5ea330999/content/rust-challenges.md) of this article has been retracted. I used an LLM to write the first draft, though this had come after many hours of *planning* and going through the data and analyses to identify the points to be made, as well as me going through the post line by line, editing into my voice and verifying the wording and scope of the text was accurate. However, many people still felt like the LLM-speak bled through in ways that felt uncomfortable. Given this, I and other members of the Rust Project have decided to retract the post in its entirety.

I stand by the content of the post. As I said, the LLM did not decide the points to be made - those were done well in advance of even beginning to write the blog post. And, admittedly, I *did* need to make edits to dampen the scope of them (in large part because I couldn't find specific quotes to substantiate them, even though I often "felt" that they were true given what I know as a Rust Project member), but in general I (and the Vision Doc team) defined the content, not an LLM.

Many people thought that the blog post felt "empty", with no "real substance". While I see the point here, this is unfortunately just how the data played out and goal of this effort. The Vision Doc team conducted ~70 interviews (mostly 1:1), which were the basis for the conclusions in this blog post. This is *a lot* of data, it's hard to fully capture the essence of them in a single blog post. And yet, it is also *not enough* data to fully capture the nuance of differences across groups of different types. On top of this, it shouldn't be that unexpected the problems we heard about in these interviews are the same problems that we (and many others) mostly already knew existed. The insight these interviews give us is that they allow us to begin to capture *for whom which issues are most prominent*.

The insights we identify and the conclusions we make are supported by the data we have gathered. When making these posts, the Vision Doc team has tried to stay as neutral as possible, doing our best to not exert bias by making any claims that cannot be supported *as stated* by the data itself. With drastically more time, I would have loved to pull in data from the ~5500 survey responses we got, which ultimately could help us make *stronger* claims or conclusions, but unfortunately that is time that I haven't had. That shouldn't diminish the substance of the insights and conclusions we *have* been able to make though.

Wording matters though. And it's clear that to many people, the blog post as-written didn't meet the mark that they want. LLMs are a tool that many people use (including me, obviously) to varying degrees to help do things that they couldn't do before (either for lack of skill, lack of time, or lack of motivation). In this case, I used an LLM to compensate for the lack of time for me to dedicate to sifting through transcripts for the ~70 interviews we did, and the many analyses that followed, to find specific quotes and write an early draft. It certainly did not help that writing and editing of this post happened over the span of about 3 months - meaning that things that "worked" in early edits did not necessarily work in later edits.

This all being said, I think that we as a Vision Doc team owe it to the Rust Project and the community to share (at least to some extent) what we have learned here. So, I have taken the original challenges identified by the team (without the recommendations or conclusions) and will provide a brief personal commentary on them. I've chosen to exclude any specific quotes - rather, just focus on the "high level" ideas. So, as a disclaimer, this will mean that the statements here will be much more biased than what we typically want to publish as part of the Vision Doc work.

----

Across the ~70 interviews the Vision Doc team conducted, we heard *a lot* of complaints. Of course, we tried to keep these interviews pretty high-level, not focusing on any particular technical details. Rather, we wanted to get a general sense of what the difficulties were that people encountered, among the other topics discussed during these interviews. Here, we've identified a few common challenges to most people, and then a few challenges that are more domain-specific. 

## Challenges that are universal

We heard a number of things that basically everyone said was an issue for them, in some capacity. Doing things to address these issues could have a universal impact, but that is not to say that these issues necessarily *block* people from using Rust.

The universal challenges, you've definitely heard before. If you write Rust, you've probably encountered them. That's what makes them universal. However, the point is that we share the data that we gather, and the fact that we have learned that these challenges *do* affect everyone is data in itself: we have sampled different domains, different experience levels, and different backgrounds; and we have found that these challenges exist for everyone.

### Compilation performance

Everybody knows that "compile times" are a thing that Rust is known for. This is an ever-moving target: the Rust Project tracks performance of the compiler on every merged change to track regressions, many people have attempted many times to make substantial progress here, and yet there is always more that we want to or could do.

The good news, is that among our interviews, nobody really told us that compilation time *currently* blocks them. We did hear things to the effect of "if we keep writing more and more Rust code, we may eventually get to a point that compile times are an issue", so that's not to say that we're "in the clear" but it is important to think about how this matters *on balance* with other challenges that people face.

### Borrow checking and ownership

Again, another thing that Rust is known for. Borrow checking and ownership is a hard topic that basically every beginner struggles with. However, we found that "Rust experts" don't really complain about the borrow checker anymore: it *is* a challenge that goes away with experience. That's not to say we can't do better for beginners, but it's not clear exactly what that means.

Certainly learning materials and compiler error messages help, and these are areas that we've tried in the past and today to sincerely provide the best experience. Despite that, the borrow checker remains a difficult part of the Rust language. We have [ongoing efforts](https://rust-lang.github.io/rust-project-goals/2026/polonius.html) to improve the borrow checker, but it's likely that there are (for example) language features that may make this better. (Or worse!)

We [found previously](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/#each-piece-is-necessary-for-the-whole-to-work) that what makes Rust *great* is the balance that we put on reliability, efficiency, and versatility. And, we need to be careful when adjusting something as core as the borrow checker to maintain this balance.

### Async

When conducting our interviews, async was consistently something that many people had issues with. Beginners often said that they basically completely ignore async while learning. People who *do* use async often said that the choice wasn't always clear, and that even though using async feels like the right choice *now*, they still encounter issues.

Fortunately, unlike performance and the borrow checker, we have a number of clear "next steps" for async (e.g. async fns in traits, async drop, async version of std traits) that will *begin* to solve these issues and close the gap. Of course, for other things (like the coloring problem), we don't have good "solutions" just quite yet.

### Ecosystem crates

We [previously](https://blog.rust-lang.org/2025/12/19/what-do-people-love-about-rust/#example-the-wealth-of-crates-on-crates-io-are-a-key-enabler-but-can-be-an-obstacle) talked about how crates.io creates a wealth of resources for people to turn to, but people still run into issues. For one, when there *are* crates that do the thing people want, they need to know: which crates do the things they need, which crates can they trust, and which crates are just overall the "best" for them. Further, in some domains and industries, there *aren't* crates that do what people need; Rust support for some industries are still too immature.

## Challenges that are domain-specific

Though more challenging given the limited diversity in the interviews we conducted, we still were able to find *some* domain-specific challenges: at least, we were able to hear about some challenges that seem to disproportionately effect some domains over others.

### Embedded

For developers programming for embedded systems, we heard most often about the difficulties that fundamentally boil down to constrained resource management. For example, embedded developers are often unable to use the vast majority of the crate ecosystem, they often have trouble using the standard library, and the debugging experience is generally harder. Things that are "normal" for most Rust developers are oftentimes "special" for embedded developers.

### Safety-critical

We made an [entire post](https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/) about shipping Rust in safety critical systems. The biggest issue for safety-critical developers with Rust is the lack of availability or maturity for tools to *certify* their Rust code.

### GUI development

The biggest issue heard from GUI developers is compilation time but is slightly different from the general case, because GUI development is so heavily dependent on the *visual* changes - and so this is a slightly different workflow than just "check if the code compiles and passes tests".
