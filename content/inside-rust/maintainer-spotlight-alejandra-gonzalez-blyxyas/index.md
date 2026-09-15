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

Hi, I am Alejandra González, also known as `@blyxyas`. I was born breathing the mediterranean sea brew, or, in other words, I am from Spain. When I was around thirteen, I thought that I should learn something that will enable me to make a living in the long run, so I started teaching myself programming.

I started with JavaScript and web development, but it didn't feel like my thing, because I like to make fast things, and this technology didn't allow me to do that. I also tried C++ for a while, but I didn't like it very much. Then I discovered Rust through the Fireship YouTube channel, at around version 1.45, and I was immediately hooked. The borrow checker, traits, it was mind-bending at first, but it felt great.

**How did you learn Rust?**

Primarily through [Rustlings][rustlings], which I really like. I think that many programming language learning materials are not that great. They are either extremely beginner, or extremely advanced. I already knew what's a loop and a function, but I needed something a little bit involved, but not too much, so that my brain would not explode. Rustlings really hit that middle point, where things are a bit challenging, but you can still solve them within half an hour.

The Rust book is of course also an incredible learning material. It is probably the only book that I would actually recommend for learning tech-related things. It is unparalleled.

**How did you start contributing to Rust?**

While using Rust, I was constantly thinking that it is neat, but there are little things missing, which could make it so much better. I had this itch to improve it, as it didn't *completely* live up to my expectations, it was a 9 out of 10.

So I opened the Rust [contribution guide][rustc-dev-guide], and started learning git, so that I could open my first pull request (PR). The guide suggested contributing to Clippy as a good onboarding project, so my [first contribution][first-pr] was adding a new lint to the Clippy linter. The lint warns users if they misuse the caret (`^`) operator, because unlike in some other programming languages, it represents the Exclusive OR (XOR) operation, and not exponentiation.

While working on my first pull request, I created a lot of weird and broken git commits and pushes. Because of that, I was extremely embarrassed, as I felt like I was wasting everyone's time. Once the PR was ready to be merged, the reviewer told me to "squash" my commits. I didn't know anything about git back then, as I was learning it on the spot. I tried to watch a tutorial on squashing commits, but it wasn't enough, and my attempts didn't end well. I managed to pull unrelated changes from another branch and made a mess. I suppose that this happened to everyone :)

It is a bit weird to look back at it today, now that I am helping other contributors make their own Clippy PRs.

Overall, I think that contributing to Clippy is much easier to contribute to it than to add a new unstable feature to the Rust compiler, for example. It is thanks to Clippy's architecture; even though we have over 800 lints, each one of them is standalone, and you can understand most of them just by reading a single Rust file. And when you create a new lint, you can simply copy-paste code from other lints and adjust it.

**How did you join the Clippy team? Did you have a mentor?**

A lot of people were mentoring me, but one Clippy maintainer, [xFrednet][xfrednet], stood out. He had a wonderful vibe, not only because of his technical knowledge, but also due to the way he interacted with other contributors. Thanks to him, I felt completely welcome in the Rust Project, it was really awesome. After I spent some time contributing to Clippy, he also invited me to join the Clippy team.

The thing that keeps me going since then are the people in the Rust Project. It is astonishing how many wonderful people consistently work on the same project. I think that many people went to Rust for the technology, and stayed for the people, and that was also my case.

**You are now also mentoring other contributors. What does it mean to you to be a mentor?**

There are actually quite a lot of people reaching out to me, wanting to help getting started with contributing to Clippy. They bring expertise from different places, and they come from all kinds of countries. I think that it is really great when beginners call out to you to help them solve their problem.

I am also mentoring some contributors from the [Rust for Linux][rfl] project, who want to create lints specific to Rust code in the Linux kernel. I really like that there is a mutual relationship with the Rust for Linux project, having a portion of Linux developers reach out to me and to the Clippy and other Rust teams is amazing. I love Linux, I use it every day, and I think it is one of the most important pieces of technology ever created. So seeing that they are adopting Rust fills me with pride.

**What are you working on in Rust at the moment?**

My primary goal is to improve the performance of the Rust compiler. I am currently working on a [Project Goal][project-goal] which is focused on prototyping a change to the Rust compiler's incremental machinery, so that it producess less unnecessary recompilations, and is overall faster. It is a big task, and an exciting plan, but it takes a lot of work and cross-team discussions to make progress in this area.

**Why are you interested in improving performance?**

When I started programming, I had a really slow computer, with a Pentium 4 CPU, so my hardware was extremely underpowered for the things I wanted to do. Later I saved up for a more powerful computer, but it was still quite slow when working with Rust. It would be wonderful if even a Raspberry Pi could compile a whole Rust workspace in less than a minute. But that is certainly not currently possible.

I find that horrible, because we are essentially gate-keeping a language which has turned into a very important technology in recent years, to people who can afford powerful hardware. I want people to not have to spend their whole fortune to get a computer powerful enough for programming in Rust.

And also there is the whole aspect of not killing our planet as fast by producing more performant software.

**You recently became one of the first Maintainers in Residence. How does that feel?**

It is amazing! When I first started on Clippy, I was really excited, I reviewed a lot of pull requests, and did a lot of the (sometimes a bit boring) [maintenance work][maintenance-post] in my spare time. As with pretty much everyone, at one point I felt really burnt out from doing that. Open source is pretty much an infinite amount of work. There is always another pull request to review, another issue to solve, another feature to add, because anyone can open a PR or an issue or request a feature. It is tiring over a long time period.

Being a [Maintainer in Residence][mir-announcement] means having financial support to do what needs to be done. I am now reviewing several pull requests per day, and taking the necessary time for each one. Without taking shortcuts, without doing less quality work. That is something I would have never dreamt of, and it really takes a huge burden off my back.

Before, whenever I was doing maintenance, I always had this doubt: should I do something else with my life? Should I get a boring office job and work in a corporate environment, or maybe go back to studying? Now I don't have to worry about that, which really helped reshape my life. I live in a medium-sized town in Spain, so the help that the MiR program provides is huge. It completely takes off that anxiety and those doubts. For now, I'm able to do what I love, and I have no words to describe how great that feels.

That being said, I also have to remind me that it is okay not to work *all the time*. Sometimes, I have anxiety and insecurity about the value I bring to Rust and the people and companies funding me. To combat that, I really like this quote: "You are able to take a break, and Rust will not break without you."

**Would you like to share anything else with the Rust community?**

Now more than ever, we've been getting bad news in the technology sector. Job offerings have been fewer and layoffs are taking over many people's careers. It seems that every couple of weeks we find ourselves in a new all-time crisis. I want to reassure people that technology exists as proof of humankind's resilience. In the age where it seems like the machine is engineered to work against us, the Rust team is dedicated to have a more humane language. Please stick around to see it bloom.

Also, incremental compile-times are going to SHATTER soon. That's it, that's the announcement. Please stick around for it.

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
