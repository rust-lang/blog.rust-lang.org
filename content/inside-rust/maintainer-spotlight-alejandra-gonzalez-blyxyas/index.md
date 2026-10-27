+++
path = "inside-rust/2026/09/21/maintainer-spotlight-alejandra-gonzalez-blyxyas"
title = "Maintainer spotlight: Alejandra González (@blyxyas)"
authors = ["Jakub Beránek", "Lori Lorusso"]

[extra]
team = "the Content team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-content"
+++

There are [hundreds][all-members-page] of people who [maintain][maintenance-post] the Rust toolchain, often on a volunteer basis on top of another job. The Rust [Content team][content-team] is working on a series of blog posts highlighting some of these prolific contributors to recognize the awesome work that they are doing in order to make Rust better for everyone. You can find the previous post [here][previous-post].
**Consider [donating][rfmf] to the Rust Foundation Maintainers Fund if you'd like to support Rust Project maintainers.**

<hr style="height: 1px; background-color: black;" />

<aside style="float: right; clear: both; margin-left: 10px;">
  <img alt="A photo of Alejandra González" src="alejandra-gonzalez.jpg" width="200" style="width: 12em; border: 1px solid white;" />
</aside>

In this post, we would like to introduce **Alejandra González** ([@blyxyas](https://github.com/blyxyas)). Alejandra started contributing to Rust around four years ago. She primarily works on the Clippy linter as a part of the Clippy team, which she joined in 2023. She is helping other people starting their Rust and Clippy contribution journeys, and recently, she became one of our first [Maintainers in Residence][mir-announcement]!

We interviewed Alejandra to find out how she started contributing to Rust, and why she considers performance to be so crucial. Read more below!

**Can you briefly introduce yourself and your background?**

Hi, I am Alejandra González, also known as `@blyxyas`. I was born breathing the mediterranean sea brew, or, in other words, I am from Spain. When I was around thirteen, I thought that I should learn something that will pay off in the long run, and what better than programming?

I started with JavaScript and web development, but it didn't feel like my thing, because I like to make fast things, and this technology didn't allow me to do that. I also tried C++ for a while, but didn't like it very much. Then I discovered Rust through a YouTube channel, on the archaic version 1.45, and I was immediately hooked. The borrow checker, traits, macros, it was mind-bending at first, and it felt great.

**How did you learn Rust?**

Primarily through [Rustlings][rustlings], which I really like. I think that many programming language learning materials are not that great. They are either extremely beginner, or extremely advanced.

Everyone knows what's a loop, they look pretty much identical in every single language, on the other hand, I had no idea what an enum is, and I had to find out! Rustlings really hits that sweet spot, where things are a bit challenging, but you can still solve them within half an hour.

The Rust book is of course also an incredible learning material. It is probably the only book that I would actually recommend for learning tech-related things. It is unparalleled.

**How did you start contributing to Rust?**

While using Rust, I was constantly thinking that it is neat, but there are little things missing, which could make it so much better. I had this itch to improve it, as it didn't *completely* live up to my expectations, it was a 9 out of 10.

So I opened the Rust [contribution guide][rustc-dev-guide], and started learning Git so that I could open my first pull request (PR). The guide suggested contributing to Clippy as a good onboarding project, so my [first contribution][first-pr] was adding a new lint to Clippy. The lint warns users if they misuse the caret (`^`) operator, because unlike in some other programming languages, it represents the Exclusive OR (XOR) operation and not exponentiation.

While working on my first pull request, I created a lot of weird and broken Git commits and pushes. It was extremely embarrassing, and I felt that I was wasting everyone's time. I was thrilled once the lint was in a ready state to be merged, until my reviewer uttered the words "squash those commits"... I didn't know much about Git back then, as I was learning it on the spot. So I tried to watch a tutorial on squashing commits; it wasn't enough. My attempts didn't end well, and I managed to pull unrelated changes from another branch and made a mess. I suppose that this has happened to everyone at least once :)

It is a bit weird to look back at it today, now that I am helping other contributors make their own Clippy PRs.

Overall, I think that contributing to Clippy is much easier than to add a new feature to the Rust compiler, for example. It is thanks to Clippy's architecture that even though we have over 800 lints, each one of them is standalone, and you can understand most of them just by reading a single Rust file. When you think of a new lint, it's almost guaranteed that someone in the codebase has already had a similar idea to yours, so you can mix and match parts of other lints to make a new creation.

**How did you join the Clippy team? Did you have a mentor?**

A lot of people were mentoring me, but one Clippy maintainer, [xFrednet][xfrednet], stood out. He had a wonderful vibe, not only because of his technical knowledge, but also due to the way he interacted with other contributors. Thanks to him, I felt completely welcome in the Rust Project, and it molded the way that I talk to new contributors. It's much more productive to have a travel guide as a maintainer, ready to take you by their side; than to have only a code guardian, only focused on the code quality.

After I had mastered the arts of the Clip, xFrednet invited me to the team, and here we are now.

The thing that keeps me going since then are the people in the Rust Project. It is astonishing how many wonderful people consistently work on the same project. I think that many people go to Rust for the promise of good technology, and actually stay for the people. This was also my case.

**You are now also mentoring other contributors. What does it mean to you to be a mentor?**

There are people reaching out to me, wanting to get started contributing to Clippy. They bring expertise from different sectors, with different personalities, and all are coming together to make a code linter a bit better. I think that it is really great when beginners call out to you to help them solve their problem.

Even without going into mentoring, meeting so many cool people feels great. In the past two years, I've been in contact with the [Rust for Linux][rfl] team. And I really like that there is a mutual relationship with the Rust for Linux project, having a portion of Linux developers reach to teams at Rust is amazing. I love Linux, I use it every day, and I think it is one of the most important pieces of technology ever created. So seeing that they are adopting Rust fills me with pride. It kind of validates that what we're doing is having a real, positive impact in the world.

**What are you working on in Rust at the moment?**

My primary goal is to improve the performance of the Rust compiler and Clippy. We've talked a lot about Clippy, so apart from it I'm currently working on a [Project Goal][project-goal] which is focused on improving the incremental machinery inside in the compiler, as to reduce useless recompilations, and make it overall faster. It's a big task and an exciting plan, but it takes a lot of work and cross-team discussions to make progress in this area.

It's both much easier and much harder than you think.

**Why are you interested in improving performance?**

When I started programming, I had a really slow computer, with the (in)famous Pentium 4 CPU, so my hardware was extremely underpowered for the things I wanted to do. Simply because of the money I had available, I was limited to the things I could achieve.

Today, I find that horrible, because we are essentially gate-keeping a language that has turned into a very important technology in recent years, to people who can afford powerful hardware. I want people to not have to spend their whole fortune to get a computer powerful enough for programming in Rust.  The day that a Raspberry Pi could compile a whole Rust workspace in less than a minute, that day I can retire with a clean conscience (and go improve performance at Zig). But that is certainly not currently possible.

And also there is the whole aspect of not killing our planet as fast by producing more performant software. I fundamentally think that we are not entitled to kill the planet just to further our technology, without thinking about the impact.

**You recently became one of the first Maintainers in Residence. How does that feel?**

It is amazing! When I first started on Clippy, I was really excited, I reviewed a lot of pull requests, and did a lot of the (sometimes a bit boring) [maintenance work][maintenance-post] in my spare time. As with pretty much everyone, at one point the burn out creeps in. Open source is an infinite amount of work; there is always another pull request to review, another issue to solve, another feature to add, because anyone can open a PR or an issue or request a feature. It is very tiring over a long time period.

Being a [Maintainer in Residence][mir-announcement] means having financial support to do what needs to be done. I am now reviewing several pull requests per day, and taking the necessary time for each one. Without taking shortcuts, without doing less quality work. That is something I would have never dreamt of, and it really takes a huge burden off my back.

Before, whenever I was doing maintenance, I always had this doubt: should I do something else with my life? Should I get a boring office job and work in a corporate environment, or maybe go back to studying? Now I don't have to worry about that, which really helped reshape my life. I live in a medium-sized town in Spain, so the help that the MiR program provides is huge in comparison to my expenses. It completely takes off that anxiety and those doubts. For now, I'm able to do what I love, and I have no words to describe how great that feels.

That being said, I also have to remind me that it is okay not to work *all the time*. Sometimes, I'm insecure about the value I bring to Rust and the people and companies funding me. To combat that, I really like this quote: "You are able to take a break, and Rust will not break without you."

**Would you like to share anything else with the Rust community?**

Now more than ever, we've been getting bad news in the technology sector. Job offerings have been fewer and layoffs are taking over many people's careers. It seems that every couple of weeks we find ourselves in a new all-time crisis. I want to reassure people that technology exists as proof of humankind's resilience. In the age where it seems like the machine is engineered to work against us, the Rust team is dedicated to have a more humane language. Please stick around to see it bloom.

Also, incremental compile-times are going to SHATTER in the next very few years. That's it, that's the announcement. Please stick around for it.

<hr style="height: 1px; background-color: black;" />

We thank Alejandra for sharing her thoughts with us, and for all her work on improving Clippy and Rust!

[all-members-page]: https://rust-lang.org/governance/people
[content-team]: https://rust-lang.org/governance/teams/launching-pad/#team-content
[rfmf]: https://github.com/sponsors/rustfoundation
[maintenance-post]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/
[mir-announcement]: https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/
[previous-post]: https://blog.rust-lang.org/inside-rust/2026/07/07/maintainer-spotlight-gen-li-rami3l/
[first-pr]: https://github.com/rust-lang/rust-clippy/pull/9506
[rustlings]: https://github.com/rust-lang/rustlings
[how-to-start-contributing]: https://forge.rust-lang.org/how-to-start-contributing.html#how-to-start-contributing-1
[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org
[xfrednet]: https://github.com/xFrednet
[rfl]: https://rust-for-linux.com
[project-goal]: https://goals.rust-lang.org/2026/incremental-system-rethought.html
