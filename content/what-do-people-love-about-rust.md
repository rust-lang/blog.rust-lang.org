+++
path = "2025/12/19/what-do-people-love-about-rust"
title = "What do people love about Rust?"
authors = ["Niko Matsakis"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad#team-project-vision-doc-2025"
+++

Rust has been named Stack Overflow's Most Loved (now called Most Admired) language every year since our 1.0 release in 2015. That means people who use Rust want to keep using Rust[^gleam]--and not just for performance-heavy stuff or embedded development, but for shell scripts, web apps, and all kinds of things you wouldn't expect. One of our participants captured it well when they said, "At this point, I don't want to write code in any other language but Rust."

[^gleam]: In 2025, 72% of Rust users said they wanted to keep using it. In the past, Rust had a *way* higher score than any other language, but this year, [Gleam came awfully close](https://survey.stackoverflow.co/2025/technology/#admired-and-desired), with 70%! Good for them! Gleam looks awesome--and hey, good choice on the `fn` keyword. ;)

When we sat down to crunch the vision doc data, one of the things we really wanted to explain was: *What is it that inspires that strong loyalty to Rust?*[^messitup] Based on the interviews, the answer is at once simple and complicated. The short version is that **Rust empowers them to write reliable and efficient software**. If that sounds familiar, it should: [it's the slogan that we have right there on our web page](https://www.rust-lang.org). The more interesting question is **how** that empowerment comes about, and what it implies for how we evolve Rust.

[^messitup]: And, uh, how can we be sure not to mess it up?

## What do people appreciate about Rust?

The first thing we noticed is that, throughout every conversation, no matter whether someone is writing their first Rust program or has been using it for years, no matter whether they're building massive data clusters or embedded devices or just messing around, there are a consistent set of things that they say they like about Rust.

The first is **reliability**. People love that "if it compiles, it works" feeling:

> "What I really love about Rust is that if it compiles it usually runs. That is fantastic, and that is something that I'm not used to in Java." -- Senior software engineer working in automotive embedded systems

> "Rust is one of those languages that has just got your back. You will have a lot more sleep and you actually have to be less clever." -- Rust consultant and open source framework developer

Another, of course, is **efficiency**. This comes up in particular at the extremes, both very large scale (data centers) and very small scale (embedded):

> "I want to keep the machine resources there for the \[main\] computation. Not stealing resources for a watchdog." -- Software engineer working on data science platforms

>  "You also get a speed benefit from using Rust. For example, \[..\] just the fact that we changed from this Python component to a Rust component gave us a 100fold speed increase." -- Rust developer at a medical device startup

Efficiency comes up particularly often when talking to customers running **"at-scale" workloads**, where even small performance wins can translate into big cost savings:

> "We have a library -- effectively it's like an embedded database -- that we deploy on lots of machines. It was written in Java and we recently rewrote it from Java to Rust and we got close to I think 9x to 10x performance wins." -- Distinguished engineer working on cloud infrastructure services

> "I'm seeing 4x efficiency in the same module between Java code that loads a VM and Rust. That's a lot of money you save in data center cost." -- Backend engineering company founder specializing in financial services

At the other end of the spectrum, people doing embedded development or working at low-levels of abstraction highlight Rust's ability to give **low-level control and access to system details**:

> "Rust was that replacement for C I'd been looking for forever." -- Backend engineering company founder specializing in financial services

> "If you're going to write something new and you do kind of low-level systemsy stuff, I think Rust is honestly the only real choice." -- Distinguished engineer 

Many people cite the importance of Rust's **supportive tooling**, which helps them get up and going quickly, and in particular the compiler's error messages:

> "I think a big part of why I was able to succeed at learning Rust is the tooling. For me, getting started with Rust, the language was challenging, but the tooling was incredibly easy." -- Executive at a developer tools company

> "The tooling really works for me and works for us. The number one way that I think I engage with Rust is through its tooling ecosystem. I build my code through Cargo. I test it through Cargo. We rely on Clippy for everything." -- Embedded systems engineer working on safety-critical robotics

> "I think the error messages and suggestions from the Rust compiler are super helpful also." -- Professor specializing in formal verification

Finally, one of Rust's most important virtues is its **extensibility**. Both in the language itself and through the crates.io ecosystem, Rust is designed to let end-users create libraries and abstractions that meet their needs:

> "The crate ecosystem combined with the stability guarantees and the semantic versioning mean that it's the best grab and go ecosystem I've ever seen." -- Computer science professor and programming language designer

> "I think proc macros are a really big superpower for Rust." -- Creator and maintainer of Rust networking libraries

> "Rust is incredibly good at making it very very easy to get started, to reuse things, just to experiment quickly with new tools, new libraries, all the rest of it... so for me, as an experimentation platform, it's great." -- Rust expert and consultant focused on embedded and real-time systems

# But what they *love* is the sense of empowerment and versatility

Reliability, efficiency, tooling, ecosystem—these are all things that people *appreciate* about Rust. But what they *love* isn't any one of those things. It's the way the combination makes Rust a **trusted, versatile tool** that you can bring to **virtually any problem**:

> "When I got to know about it, I was like 'yeah this is the language I've been looking for'. This is the language that will just make me stop thinking about using C and Python. So I just have to use Rust because then I can go as low as possible as high as possible." -- Software engineer and community organizer in Africa

> "I wanted a language that works well from top to bottom in a stacking all the way from embedded to very fancy applications" -- Computer science professor and programming language designer

> "If \[Rust\] is going to try and sort of sell itself more in any particular way, I would probably be saying high performance, highly expressive, general purpose language, with the great aspect that you can write everything from the top to the bottom of your stack in it." -- Rust expert and consultant focused on embedded and real-time systems

## Each piece is necessary for the whole to work

Take away the reliability, and you don't trust it: you're second-guessing every deployment, afraid to refactor, hesitant to let junior developers touch the critical paths.

> "Rust just lowers that bar. It's a lot easier to write correct Rust code. As a leader on the team, I feel a lot safer when we have less experienced engineers contributing to these critical applications." -- Distinguished engineer working on cloud infrastructure services

> "My experience with writing Rust software tends to be **once you've got it working, it stays working**. That's a combination of a lot of care taken in terms of backwards compatibility with the language and a lot of care taken around the general ecosystem." -- Rust expert and consultant focused on embedded and real-time systems

Reliability also provides guardrails that help people enter new domains—whether you're a beginner learning the ropes or an expert venturing into unfamiliar territory:

> "Rust introduces you to all these things, like match and all these really nice functional programming methods." -- Software engineer with production Rust experience

> "I think Rust ownership discipline is useful both for regular Rust programmers and also for verification. I think it allows you to within the scope of your function to know very clearly what you're modifying, what's not being modified, what's aliased and what's not aliased." -- Professor specializing in formal verification

> "I discovered Rust... and was basically using it just to give myself a little bit more confidence being like a solo firmware developer" -- Software engineer working on automotive digital cockpit systems

Take away the efficiency and low-level control, and there are places you can't go: embedded systems, real-time applications, anywhere that cost-per-cycle matters.

> "The performance in Rust is nutty. It is so much better and it's safe. When we rewrote C++ and C libraries or C applications into Rust, they would end up being faster because Rust was better at laying out memory." -- Senior Principal Engineer leading consumer shopping experiences

> "9 times out of 10, I write microcontroller code and I only test it through unit testing. I put it on real hardware and it just works the first time." -- Embedded systems engineer working on safety-critical robotics

> "I can confidently build systems that scale." -- Engineering manager with 20 years experience in media and streaming platforms

Take away the tooling and ecosystem, and you can't get started: or you can, but it's a slog, and you never feel productive.

> "For me, getting started with Rust, the language was challenging, but the tooling was incredibly easy... I could just start writing code and it would build and run, and that to me made a huge difference." -- Founder and CEO of company creating developer tools

> "Cargo is an amazing package manager. It is probably the best one I've ever worked with. I don't think I ever run into issues with Cargo. It just works." -- Software engineer with production Rust experience

> "The Rust compiler is fantastic at kind of the errors it gives you. It's tremendously helpful in the type of errors it produces for it. But not just errors, but the fact it also catches the errors that other languages may not catch." -- Distinguished engineer working on cloud infrastructure services

## The result: Rust as a gateway into new domains

When all these pieces come together, something interesting happens: Rust becomes a **gateway** into domains that would otherwise be inaccessible. We heard story after story of people whose careers changed because Rust gave them confidence to tackle things they couldn't before:

> "I was civil engineering and I studied front-end development on my own, self taught. I had no computer background. I got interested in Rust and distributed systems and designs and systems around it. I changed my major, I studied CS and Rust at the same time." -- Software engineer transitioning to cryptography research

> "I've been working with arbitrary subsidiaries of \[a multinational engineering and technology company\] for the last 25 years. Always doing software development mostly in the Java space... two years ago I started peeking into the automotive sector. In that context it was a natural consequence to either start working with C++ (which I did not want to do) or take the opportunity to dive into the newly established Rust ecosystem." -- Senior software engineer working in automotive embedded systems

> "I started in blockchain. Currently I'm doing something else at my day job. Rust actually gave me the way to get into that domain." -- Rust developer and aerospace community leader

> "Before that, I had 10 years of programming on some dynamic programming languages, especially Ruby, to develop web applications. I wanted to choose some language which focuses on system programming, so I chose Rust as my new choice. It is a change of my career." -- Rust consultant and author working in automotive systems and blockchain infrastructure

## But the balance is crucial

Each of Rust's attributes are necessary for versatility across domains. But when taken too far, or when other attributes are missing, they can become an obstacle.

### Example: Complex APIs and type complexity

One of the most powerful aspects of Rust is the way that its type system allows modeling aspects of the application domain. This prevents bugs and also makes it easier for noobs to get started[^sleep]:

[^sleep]: ...for experienced devs operating on less sleep, who do tend to act a lot like noobs.

> "Instead of using just a raw bit field, somebody encoded it into the type system. So when you'd have a function like 'open door', you can't pass an 'open door' if the door's already open. The type system will just kick that out and reject it." -- Software engineer working on automotive digital cockpit systems

> "You can create contracts. For example, when you are allowed to use locks in which order." -- Senior embedded systems engineer working on automotive middleware development

The problem though is that sometimes the work to encode those invariants in types can create something that feels more complex than the problem itself:

> "When you got Rust that's both async and generic and has lifetimes, then those types become so complicated that you basically have to be some sort of Rust god in order to even understand this code or be able to do it." -- Software engineer with production Rust experience

> "Instead of spaghetti code, you have spaghetti typing" -- Platform architect at automotive semiconductor company

> "I find it more opaque, harder to get my head around it. The types describe not just the interface of the thing but also the lifetime and how you are accessing it, whether it's on the stack or the heap, there's a lot of stuff packed into them." -- Software engineer working on data science platforms

This leads some to advocate for not using some of Rust's more complex features unless they are truly needed:

> "My argument is that the hard parts of Rust -- traits, lifetimes, etc -- are not actually fundamental for being productive. There's a way to set up the learning curve and libraries to onboard people a lot faster." -- Creator and maintainer of Rust networking libraries

### Example: Async ecosystem is performant but doesn't meet the bar for supportiveness

Async Rust has fueled a huge jump in using Rust to build network systems. But many commenters talked about the sense that "async Rust" was something altogether more difficult than sync Rust:

> "I feel like there's a ramp in learning and then there's a jump and then there's async over here. And so the goal is to get enough excitement about Rust to where you can jump the chasm of sadness and land on the async Rust side." -- Software engineer working on automotive digital cockpit systems

> "My general impression is actually pretty negative. It feels unbaked... there is a lot of arcane knowledge that you need in order to use it effectively, like Pin---like I could not tell you how Pin works, right?" -- Research software engineer with Rust expertise

For Rust to provide that "trusted tool that will help you tackle new domains" experience, people need to be leverage their expectations and knowledge of Rust in that new domain. With async, not only are there missing language features (e.g., `async fn` in traits only became available last year, and still have gaps), but the supportive tooling and ecosystem that users count on to "bridge the gap" elsewhere works less well:

> "I was in favor of not using async, because the error messages were so hard to deal with." -- Desktop application developer

> "The fact that there are still plenty of situations where you go *that library looks useful, I want to use that library* and then that immediately locks you into one of tokio-rs or one of the other runtimes, and you're like *that's a bit disappointing because I was trying to write a library as well and now I'm locked into a runtime*." -- Safety systems engineer working on functional safety for Linux

> "We generally use Rust for services, and we use async a lot because a lot of libraries to interact with databases and other things are async. The times when we've had problems with this is like, um, unexplained high CPU usage, for example. The only really direct way to try to troubleshoot that or diagnose it is like, *OK, I'm going to attach GDB and I'm gonna try to see what all of the threads are doing*. GDB is -- I mean, this is not Rust's fault obviously -- but GDB is not a very easy to use tool, especially in a larger application. \[..\] And with async, it's, more difficult, because you don't see your code running, it's actually just sitting on the heap right now. Early on, I didn't actually realize that that was the case." -- Experienced Rust developer at a company using Rust and Python

Async is important enough that it merits a deep dive. Our research revealed a lot of frustration but we didn't go deep enough to give more specific insights. This would be a good task to be undertaken by the future User Research team (as proposed in [our first post](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/)).

### Example: The wealth of crates on crates.io are a key enabler but can be an obstacle

We mentioned earlier how Rust's extensibility is part of how it achieves versatility. Mechanisms like overloadable operators, traits, and macros let libraries create rich experiences for developers; a minimal standard library combined with easy package management encourage the creation of a rich ecosystem of crates covering needs both common and niche. However, particularly when people are first getting started, that *extensibility* can come at the cost of *supportiveness*, when the "tyranny of choice" becomes overwhelming:

> "The crates to use are sort of undiscoverable. There's a layer of tacit knowledge about what crates to use for specific things that you kind of gather through experience and through difficulty. Everyone's doing all of their research." -- Web developer and conference speaker working on developer frameworks

> "Crates.io gives you some of the metadata that you need to make those decisions, but it's not like a one stop shop, right? It's not like you go to crates.io and ask 'what I want to accomplish X, what library do I use'---it doesn't just answer that." -- Research software engineer

The Rust org has historically been reluctant to "bless" particular crates in the ecosystem. But the reality is that some crates are omnipresent. This is particular challenging for new users to navigate:

> "The tutorial uses `Result<Box<dyn Error>>` -- but nobody else does. Everybody uses anyhow-result... I started off using the result thing but all the information I found has example code using anyhow. It was a bit of a mismatch and I didn't know what I should do." -- Software engineer working on data science platforms

> "There is no clear recorded consensus on which 3P crates to use. \[..\] Sometimes it's really not clear---which CBOR crate do you use?\[..\] It's not easy to see which crates are still actively maintained. \[..\] The fact that there are so many crates on crates.io makes that a little bit of a risk." -- Rust team from a large technology company

## Recommendations

### Enumerate Rust's design goals and integrating them into our processes

We recommend creating an RFC that defines the goals we are shooting for as we work on Rust. The RFC should cover the experience of using Rust in total (language, tools, and libraries). This RFC could be authored by the proposed User Research team, though it's not clear who should accept it — perhaps the User Research team itself, or perhaps the leadership council.

This post identified how the real "empowering magic" of Rust arises from achieving a number of different attributes all at once -- reliability, efficiency, low-level control, supportiveness, and so forth. It would be valuable to have a canonical list of those values that we could collectively refer to as a community and that we could use when evaluating RFCs or other proposed designs.

There have been a number of prior approaches at this work that we could build on (e.g., [this post from Tyler Mandry](https://tmandry.gitlab.io/blog/posts/the-main-thing/), the [Rustacean Principles](https://smallcultfollowing.com/babysteps/blog/2021/09/08/rustacean-principles/), or the [Rust Design Axioms](https://smallcultfollowing.com/babysteps/blog/2023/12/07/rust-design-axioms/)). One insight from our research is that we don't need to define which values are "most important". We've seen that for Rust to truly work, it must achieve **all** the factors at once. Instead of ranking, it may help to describe how it feels when you:

- **Don't achieve it** (too little)
- **Get it right** (the sweet spot)
- **Go overboard** (too much)

This "goldilocks" framing helps people recognize where they are and course-correct, without creating false hierarchies.

### Double down on extensibility

We recommend **doubling down on extensibility** as a core strategy. Rust's extensibility — traits, macros, operator overloading — has been key to its versatility. But that extensibility is currently concentrated in certain areas: the type system and early-stage proc macros. We should expand it to cover **supportive interfaces** (better diagnostics and guidance from crates) and **compilation workflow** (letting crates integrate at more stages of the build process).

Rust's extensibility is a big part of how Rust achieves versatility, and that versatility is a big part of what people love about Rust. Leveraging mechanisms like proc macros, the trait system, and the borrow checker, Rust crates are able to expose high-level, elegant interfaces that compile down to efficient machine code. At its best, it can feel a bit like magic. 

Unfortunately, while Rust gives crates good tools for building safe, efficient abstractions, we don't provide tools to enable **supportive** ones. Within builtin Rust language concepts, we have worked hard to create effective error messages that help steer users to success; we ship the compiler with lints that catch common mistakes or enforce important conventions. But crates benefit from none of this. RFCs like [RFC #3368](https://rust-lang.github.io/rfcs/3368-diagnostic-attribute-namespace.html), which introduced the diagnostic namespace and `#[diagnostic::on_unimplemented]`, Rust has already begun moving in this direction. We should continue and look for opportunities to go further, particularly for proc-macros which often create DSL-like interfaces.

The other major challenge for extensibility is concerned with the build system and backend. Rust's current extensibility mechanisms (e.g., build.rs, proc-macros) are focused on the *early stages* of the compilation process. But many extensions to Rust, ranging from interop to theorem proving to GPU programming to distributed systems, would benefit from being able to integrate into other stages of the compilation process. The [Stable MIR](https://github.com/rust-lang/project-stable-mir) project and the [build-std project goal](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html?highlight=std#build-std) are two examples of this sort of work.

Doubling down on extensibility will not only make current Rust easier to use, it will enable and support Rust's use in new domains. Safety Critical applications in particular require a host of custom lints and tooling to support the associated standards. Compiler extensibility allows Rust to support those niche needs in a more general way.

### Help users get oriented in the Rust ecosystem

We recommend finding ways to help users navigate the crates.io ecosystem. Idiomatic Rust today relies on custom crates for everything from error-handling to async runtimes. Leaning on the ecosystem helps Rust to scale to more domains and allows for innovative new approaches to be discovered. But finding which crates to use presents a real obstacle when people are getting started. The Rust org maintains a carefully neutral stance, which is good, but also means that people don't have anywhere to go for advice on a good "starter set" crates.

The right solution here is not obvious. Expanding the standard library could cut off further experimentation; "blessing" crates carries risks of politics. But just because the right solution is difficult doesn't mean we should ignore the problem. Rust has a history of exploring creative solutions to old tradeoffs, and we should turn that energy to this problem as well.

Part of the solution is enabling better interop between libraries. This could come in the form of adding key interop traits (particularly for async) or by blessing standard building blocks (e.g., the `http` crate, which provides type definitions for HTTP libraries). Changes to coherence rules can also help, as the current rules do not permit a new interop trait to be introduced in the ecosystem and incrementally adopted.

## Conclusion

To sum up the main points in this post:

* What people love about Rust is the way it empowers them to tackle tough problems and new domains. This is not the result of any one attribute but rather a careful balancing act between many; if any of them are compromised, the language suffers significantly.
* We make three recommendations to help Rust continue to scale across domains and usage levels
    * Enumerate and describe Rust's design goals and integrate them into our processes, helping to ensure they are observed by future language designers and the broader ecosystem.
    * Double down on extensibility, introducing the ability for crates to influence the develop experience and the compilation pipeline.
    * Help users to navigate the crates.io ecosystem and enable smoother interop
