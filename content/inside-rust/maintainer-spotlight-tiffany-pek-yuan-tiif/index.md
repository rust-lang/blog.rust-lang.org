+++
path = "inside-rust/2026/06/03/maintainer-spotlight-tiffany-pek-yuan-tiif"
title = "Maintainer spotlight: Tiffany Pek Yuan (@tiif)"
authors = ["Jakub Beránek, Lori Lorusso"]

[extra]
team = "the Content team"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-content"
+++

There are [hundreds][all-members-page] of people who [maintain][maintenance-post] the Rust toolchain, often on a volunteer basis on top of another job. The Rust [Content team][content-team] is working on a series of blog posts highlighting some of these prolific contributors to recognize the awesome work that they are doing in order to make Rust better for everyone. This post is the first one in the series.

<hr style="height: 1px; background-color: black;" />

<aside style="float: right; clear: both; margin-left: 10px;">
  <img alt="A photo of Tiffany Pek Yuan" src="tiif.jpg" width="200" style="width: 12em; border: 1px solid white;" />
</aside>

In this post, we would like to introduce **Tiffany Pek Yuan** ([@tiif](https://github.com/tiif/)). Tiffany started contributing to Rust only two years ago, as a Google Summer of Code [(GSoC) contributor][gsoc-project]. Since then, she already became a member of the [Compiler][compiler-team] and [Formality][formality-team] teams, and made substantial contributions to several projects, including the compiler, [Miri][miri] and [a-mir-formality][a-mir-formality]. She also transitioned from being a mentee to a mentor, as she now mentors another contributor in the [Outreachy][outreachy-a-mir-formality] program.

Currently, she is finishing her college degree while also working on the Rust compiler as a Rust maintainer intern in the [RustNL Maintainers Team][rustnl-maintainers].

> I got to know Tiffany as a GSoC student adding miri support for checking unsafe code in tokio. It was a joy to mentor her, and subsequently seeing her sticking around and learning about formality and rustc on her own.
>
> Oli Scherer, Rust compiler team member and Tiffany's GSoC mentor

We interviewed Tiffany to find out about her experiences with joining the Rust Project and contributing to Rust. Read more below!

**Can you briefly introduce yourself?**

Hi, my name is Tiffany Pek. I'm currently studying a Bachelor's degree in computer science at the National University of Singapore. I should hopefully graduate in a few months! I contributed to various parts of Rust, right now I am focusing on a new trait solver bug fix that is related to implied bounds and modeling the borrow checker's semantics in the [a-mir-formality][a-mir-formality] project, which attempts to define a model of the type system.

**How did you start with Rust? Did you learn it in one of your classes, or was it a hobby?**

I did eventually attend a subject on Rust, but I already learnt it before that by myself. I heard about Rust around 2022, because I was searching for a systems programming language. I tried it for a research project and I really liked it. I tried a lot of programming languages before that, such as Java, Python or C++. They weren't bad, but Rust seemed much more enjoyable. What fascinated me about it were its really nice error messages.

**How did you get involved with the Rust Project?**

I remember telling my friend in college that it would be *so cool* to contribute to the Rust Project. Then, just a month later, I learned about Rust's [involvement][rust-gsoc-2024] in Google Summer of Code (GSoC), and I immediately thought "I must join this program"! We all knew about GSoC at our university, but not many people tried to apply to it for some reason. I applied to GSoC and got accepted.

I think I would eventually contribute to Rust even without GSoC, but this internship gave me an opportunity to work on Rust almost full-time, without having to care about getting another job. That was very helpful.

**How was your experience with GSoC?**

I think it went really well! The experience was really enjoyable, because there were several other interns working on other projects at the same time. And all of us were kind of "newbies", so I wasn't the only person struggling with something. For example, when I messed up rebasing one of my pull requests, I saw that the same thing happened to another GSoC contributor, so I was like "oh yeah, we are the same!" and I didn't feel so bad about it. Also, Oli is a very nice mentor too, I had a lot of fun talking with them.

Jakub is also a really good organizer, which makes the program a huge success. I was really impressed by [how we already discussed burnout](https://rust-lang.zulipchat.com/#narrow/channel/421156-gsoc/topic/GSoC.202024.20participants/near/439416607) even before the internship started.

Overall, it was a great onboarding experience, and it motivated me to stay as I really love the community.

**What was your GSoC project about?**

I worked on [Miri][miri], which is the undefined behavior detection tool for Rust. At that time, it wasn't really possible to run Miri on non-trivial asynchronous Rust programs that used the [tokio](https://tokio.rs/) crate. I [added][gsoc-project] support for various non-blocking I/O syscalls (such as `epoll`) to it, in order to expand the set of programs that Miri can handle. We then worked with Tokio maintainers to run Miri on most Tokio tests in their CI, which was quite exciting.

**What was your first contribution?**

I added two [tests](https://github.com/rust-lang/miri/pull/3398) to Miri. While doing so, I found out that the contribution documentation contained some unclear instructions, so I also improved it in the same PR.

**Would you recommend that approach for new contributors?**

Yes, I think that improving documentation is a good way to start contributing to Rust. It is also important to first locate areas that you are interested in, because the Rust Project is huge, there is Miri, the compiler, rustdoc, rust-analyzer, Cargo, and many other areas in need of contributions. Before making larger changes, it is always good to discuss them first with the corresponding maintainers before you open a PR.

If you are just starting with Rust, always read the error messages, because they are really nice. And if you find the error message difficult to understand, it might be a bug, so go open an issue!

**What made you stay with Rust even after GSoC?**

I stick around because of the community. During GSoC, I started talking to other Rust GSoC contributors. I ended up meeting two of them and other Rust Project members at a conference in Beijing. Subsequently, I made many amazing friends at the [Rust All Hands][all-hands-2025], which makes Rust no longer a purely technical project, but also a hobby project me and a lot of my friends love and care about.

**How did you join the Compiler team?**

I started contributing to Rust in March 2024. I worked mostly on Miri during my GSoC project, but later I transitioned to working on the compiler. I was invited to join the Compiler team in November 2025. I didn't expect the invitation at all, so it surprised me.

At that point, I already interacted with the Rust community a lot, so it didn't feel like a large change to join the team, but getting officially recognized does feel really nice. I also think that being a Rust team member might help me with searching for a job.

**Are you happy that you joined the Rust Project?**

I am very happy about it! The people in the Project are really nice, and we frequently talk with one another, which is great.

I was able to attend the [RustWeek](https://rustweek.org) conference and the Rust All Hands in 2025 and 2026, which was super nice. I'm grateful for the travel grant that I got from the Rust Foundation, which allowed me to travel to these events. I made so many friends there, which is part of the reason why I am still staying with the Rust Project. All my Rust friends were there!

**What excites you about working on the Rust compiler?**

I find the compiler really exciting! It is like a mysterious black-box. If you do not know about how it works, it can seem scary, but once you start learning about it, it is actually quite neat to work with.

I find the new trait solver and Polonius work (the new borrow checking implementation) very exciting because they are the very core part of the language. Once the new trait solver is completed, we will be able to unblock a lot of features and fix many unsoundness bugs. And Polonius will allow the compiler to accept many correct programs that were previously rejected, which can make the language even more expressive.

**What is it like to work in the compiler team, which is large and very visible?**

The compiler team is so big[^compiler-team-size] simply because the codebase is very large, so everyone specializes in different parts of the compiler. I mostly only review parts of the compiler that I am familiar with. I never know what is going on in another part of the compiler, as there are just so many things happening every day.

[^compiler-team-size]: Currently, it has over 70 [members](https://rust-lang.org/governance/teams/compiler/#team-compiler).

**Who is your biggest mentor in the Project?**

It is multiple people, not just a single one. Initially it was definitely [oli][oli], because he was my GSoC mentor, and we talked (and still talk) quite a lot. Later I started working on type system stuff, where I help [lcnr][lcnr] with the new trait solver. I also talk a lot with [Boxy][boxy], who is now a co-lead of the compiler team.

**Did you also try mentoring someone?**

I try to leave easy improvements to other people, by opening issues for them that contain contribution instructions, and then try to answer questions from potential contributors on those issues. Recently I also started mentoring a contributor in the [Outreachy][outreachy-a-mir-formality] program, which is exciting.

**Would you like to change something in the Rust Project?**

I know a lot of people who could produce awesome stuff if they had the opportunity to work on Rust full-time. But without funding, they have to work on and off, which is not very productive for them, and also bad for knowledge transfer. I wish more Rust contributors were funded.

**Were you able to find funding for your own Rust contributions?**

When I started contributing to Rust, I was funded by the GSoC program, which was nice. Later I joined an internship program at ETH Zurich, which also provided a source of funding. During those times, I was quite productive. But similar funding sources are usually only temporary. I am still a student, so I have to deal with my academic life, and I also need an extra job to support myself, which can be a bit difficult.

Finding a remote job can be tricky, because I live in Singapore, and the time zone difference is not that great. If I'm going to schedule a meeting with someone in the US, I usually have to do it at night. Luckily I don't go to sleep that early :-) But it can be an issue when interacting with people and companies from Europe or the US.

I hope to finish my studies soon, and after that I will have to get a full-time job. I think that I would still want to contribute to Rust even in that scenario, but I'm not sure yet how to do that. Maybe I would need to sacrifice my weekends a little bit, or something like that. It can be a bit difficult, but it is not impossible.

Right now, I have an internship in the RustNL Maintainers Team. It will be awesome if I can find a full-time job which allows me to work on Rust as I really love what I am doing now.

I think the upcoming funding efforts, such as the RustNL Maintainers Team and the Rust Foundation Maintainer Fund, will be able to financially support more Rust Project contributors. Though I am slightly worried about the incentives of companies funding maintenance work. Company interests are not always necessarily the same as the community's interests, which can create a bit of a conflict of interest. But I'm sure that there are ways to mitigate this.

<hr style="height: 1px; background-color: black;" />

We thank Tiffany for sharing her thoughts with us, and for all her work on improving Rust!

[all-members-page]: https://rust-lang.org/governance/people
[content-team]: https://rust-lang.org/governance/teams/launching-pad/#team-content
[compiler-team]: https://rust-lang.org/governance/teams/compiler/#team-compiler
[formality-team]: https://rust-lang.org/governance/teams/compiler/#team-formality
[maintenance-post]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/
[gsoc-project]: https://blog.rust-lang.org/2024/11/07/gsoc-2024-results/#tokio-async-support-in-miri
[rust-gsoc-2024]: https://blog.rust-lang.org/2024/02/21/Rust-participates-in-GSoC-2024/
[rustnl-maintainers]: https://rustnl.org/maintainers
[a-mir-formality]: https://github.com/rust-lang/a-mir-formality
[miri]: https://github.com/rust-lang/miri
[outreachy-a-mir-formality]: https://blog.rust-lang.org/2026/05/04/outreachy-2026-may/#fuzzing-the-a-mir-formality-type-system-implementation
[lcnr]: https://github.com/lcnr
[oli]: https://github.com/oli-obk
[boxy]: https://github.com/boxyuwu
[all-hands-2025]: https://blog.rust-lang.org/inside-rust/2024/09/02/all-hands
