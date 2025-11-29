+++
path = "9999/12/31/lessons-learned-from-the-rust-vision-doc-process"
title = "Lessons learned from the Rust Vision Doc process"
authors = ["Niko Matsakis"]

[extra]
team = "Vision Doc group"
team_url = "https://www.rust-lang.org/governance/teams/launching-pad/#team-project-vision-doc-2025"
+++


Starting earlier this year, a group of us set on a crazy quest: to author a "Rust vision doc". As we described it in the original [project goal proposal](https://rust-lang.github.io/rust-project-goals/2025h1/rust-vision-doc.html):

> The Rust Vision Doc will summarize the state of Rust adoption -- where is Rust adding value? what works well? what doesn't? -- based on conversations with individual Rust users from different communities, major Rust projects, and companies large and small that are adopting Rust.

Over the course of this year, the [Vision Doc group](https://rust-lang.org/governance/teams/launching-pad/#team-project-vision-doc-2025) has gathered up a lot of data. We began with a [broad-based survey that got about 4200 responses](https://blog.rust-lang.org/2025/04/04/vision-doc-survey/). After that, we conducted over 70 interviews, each one about 45 minutes, with as broad a set of Rust users as we could find[^notaswide].

This is the first of a series of blog posts covering what we learned throughout that process and what recommendations we have to offer as a result. This first post is going to go broad. We'll discuss the process we used and where we think it could be improved going forward. We'll talk about some of the big themes we heard -- some that were surprising and others that were, well, not surprising at all. Finally, we'll close with some recommendations for how the project might do more work like this in the future.

[^notaswide]: "As wide a variety of Rust users **as we could find**" -- the last part is important. One of the weaknesses of this work is that we wanted to hear from more Rust skeptics than we did.

## The questions we were trying to answer

One of the first things we did in starting out with the vision doc was to meet with a User Research expert, [Holly Ellis](https://www.linkedin.com/in/holly-ellis-0a336445/), who gave us a quick tutorial on how User Research works[^tyh]. Working with her, we laid out a set of **research questions** that we wanted to answer. Our first cut was very broad, covering three themes:

[^tyh]: Thanks Holly! We are ever in your debt.

* Rust the technology:
    * "How does Rust fit into the overall language landscape? What is Rust's mission?"
    * "What brings people to Rust and why do they choose to use it for a particular problem...?"
    * "What would help Rust to succeed in these domains...?" (e.g., network systems, embedded)
    * "How can we scale Rust to industry-wide adoption? And how can we ensure that, as we do so, we continue to have a happy, joyful open-source community?"
* Rust the global project:
    * "How can we improve the experience of using Rust for people across the globe?"
    * "How can we improve the experience of contributing to and maintaining Rust for people across the globe?"
* Rust the open-source project:
    * "How can we tap into the knowledge, experience, and enthusiasm of a growing Rust userbase to improve Rust?"
    * "How can we ensure that individual or volunteer Rust maintainers are well-supported?"
    * "What is the right model for Foundation-project interaction?"

## How to do a UX interview

The basic insight of UX is that you don't necessarily ask people the exact questions you want to answer. That is likely to get them speculating and giving you the answer that they think they "ought" to say.

What you want to know, of course, is what's *true* -- so you come at it sideways. You ask them factual, non-leading questions. In other words, you certainly don't say "Do you agree the borrow checker is really hard?" And you probably don't even say, "What is the biggest pain point you had with Rust?" Instead, you might say, "What was the last time you felt confused by an error message?" And then go from there, "Is this a typical example? If not, what's another case where you felt confused?"

To be honest, these kind of "extremely non-leading questions" are kind of difficult to do. But they can uncover some surprising results.

## We got answers -- but not all the answers we wanted

70 interviews later, we got a lot of information -- but we still don't feel like we have the answers to some of the biggest questions. Given the kinds of questions we asked, we got a pretty good view on the kinds of things people love about Rust and what it offers relative to other languages. We got a sense for the broad areas that people find challenging. We also learned a few things about how the Rust project interacts with others and how things vary across the globe.

What we really *don't* have is enough data to say "if you do X, Y, and Z, that will really unblock Rust adoption in this domain". We just didn't get into enough technical detail, for example, to give guidance on which features ought to be prioritized, or to help answer specific design questions that the lang or libs team may consider.

## One big lesson: there are only 24 hours in a day

One of the things we learned was that you need to stay focused. There were so many questions we wanted to ask, but only so much time in which to do so. Ultimately, we wound up narrowing our scope in several ways:

* we focused primarily on the individual developer experience, and only had minimal discussion with companies as a whole;
* we dove fairly deep into one area (the Safety Critical domain) but didn't go as deep into the details of other domains;
* we focused primarily on Rust adoption, and in particular did not even attempt to answer the questions about "Rust the open-source project".

## Another big lesson: haters gonna... stay quiet?

One thing we found surprisingly difficult was finding people to interview who *didn't* like Rust. It turns out that people who think Rust isn't worth using mostly don't read the Rust blog or want to talk about that with a bunch of Rust fanatics.[^shocking] This is a shame, of course, as likely those folks have a lot to teach us about the boundaries of where Rust adds value. We are currently doing some targeted outreach in an attempt to grow our scope here, so stay tuned, we may get more data.

[^shocking]: Shocking, I know. But, actually, it is a *little* -- most programmers love telling you how much they hate everything you do, in my experience?

## What do people *love about Rust?*

You've all heard the headlines: Rust has been named Stack Overflow's most admired (or most loved, depending on the year) language since 2016. This question ranks the number of current Rust users that want to keep using Rust and compares that to other programming languages. And Rust does really, really well. So what is it that Rust users find valuable about Rust? This was one of the things we probed in our questions. What did we hear?

### Reliability

First, unsurprisingly, people love Rust's **reliability**:

> "What I really love about Rust is that if it compiles it usually runs. That is fantastic, and that is something that I'm not used to in Java." -- New Rust developer who previously Java
>
> "Rust is is one of those languages that have just got to your back. You will have a lot more sleep and you actually have to be less clever." -- Long-time Rust developer (>10 years)
>
> "9 times out of 10, I write microcontroller code and I only test it through unit testing. I put it on real hardware and it just works the first time." -- Embedded developer
>
> "My experience with writing Rust software tends to be once you've got it working, it stays working. That's a combination of a lot of care taken in terms of backwards compatibility with the language and a lot of care taken around the general ecosystem." -- Long-time Rust developer

### Performance

And of course, they love Rust's **high performance**:

> "The performance in Rust is nutty. It is so much better and it's safe. When we rewrote C++ and C libraries or C applications into Rust, they would end up being faster because Rust was better at laying out memory." -- Senior developer at a large company
> 
> "I'm seeing 4x efficiency gains in the same module comparing Rust and Java. That's a lot of money you save in data center costs." -- Software developer working with banks and enterprise systems

### Supportive, polished tooling

But it's not just the language, it's also the **high quality tools**. A common theme was that while learning the language may have challenges, the care and attention put into the tooling ensure that people are able to get up and going:

> "For me, getting started with Rust, the language was challenging, but the tooling was incredibly easy... I could just start writing code and it would build and run, and that to me made a huge difference." -- New Rust developer
>
> "Cargo is an amazing package manager. It is probably the best one I've ever worked with. I don't think I ever run into issues with cargo. It just works." -- New Rust developer
>
> "The Rust compiler is fantastic at kind of the errors it gives you. It's tremendously helpful in the type of errors it produces for it. But not just errors, but the fact it also catches the errors that other languages may not catch." -- Senior developer at a large company

### Versatility

Many Rust users talked about how they value Rust's ability to bridge low- and high-level programming in one language:

> "Most of the things I do, Rust fits. So no need to actually change that. I try to exclusively use Rust." -- Experienced Rust developer who came from web development
>
> "Rust is more of a one-stop-tool that you can actually learn it and then use it for other purposes, like embedded, android, web services, and other stuff." -- Rust developer and meetup organizer
>
> "High performance, highly expressive, general purpose language, with the great aspect that you can write everything from the top to the bottom of your stack in it." -- Experienced Rust developer
> 
> "So, in the future, I think Rust was developed first as a system-level programming language, but currently, Rust is becoming more and more a multi-domain programming language that can be used in low-level programming as embedded systems, and it's also suitable for some other kind of tasks like the GUI and the web service." -- Rust developer in China

### Rust as a way to "level up"

Another familiar theme, but one that showed up loud and clear in our conversations, is that Rust is a way to "level up" as a developer, learning new concepts and new domains that were previously inaccessible:

> "Rust introduces you to all these things, like match and all these really nice functional programming methods." -- New Rust Developer
>
> "I think Rust ownership discipline is useful both for regular Rust programmers and also for verification. I think it allows you to within the scope of your function to know very clearly what you're modifying, what's not being modified, what's aliased and what's not aliased."

## Enums as Rust's underappreciated superpower?

One interesting thing was the number of people that talked *specifically* about Rust enums, which allow you to package up the state of your program along with the data it has available in that state. Enums are a concept that Rust adapted from functional languages like Ocaml and Haskell and fit into the system programming setting.

>  "The usage of Enum is a new concept for me. And I like this concept. It's not a class and it's not just a boolean, limited to false or true. It has different states." -- New Rust developer
>
> "Tagged unions. I don't think I've seriously used another production language which has that. Whenever I go back to a different language I really miss that as a way of accurately modeling the domain." -- Embedded developer

## Our biggest surprise: Rust users blocked on slow stabilizations

One of the biggest surprises for me was how many people talked about the slow pace of stabilization. It's not that I thought we were super fast, but I thought that stabilization would be ["inside baseball"](https://en.wikipedia.org/wiki/Inside_baseball_(metaphor)), something that Rust developers think about but not Rust users. And for many Rust users, that is true, Rust gives them everything they need. But for those who *are* blocked on unstable features, the pace of stabilization can be a *very sharp* pain point:

> "The stabilization of features is excruciatingly slow. Really, really slow. \[..\] There are some features that are sitting there for six to seven years. We would really like to see them on stable because the policy of some clients doesn't allow using nightly software."

Async Rust has been a focus area for some time, but there are still a number of language features needed to bring it up to parity with sync Rust, and this showed up loud and clear in our conversations (we'll go deeper into lessons around async Rust in a future blog post):

> "My general impression is actually pretty negative. It feels unbaked."

## Contributing to Rust as a company is hard

One of the magical things about open-source, of course, is that if there are features you need, you are empowered to go and unblock yourself, right? Well, sort of. 

What we found from talking to people about their experiences contributing to Rust is that, if you come as an individual with a narrow ask, the experience can often be pretty good:

> "If I could make sense of the code, I would file a contribution. Even if you're not a Rust contributor, you can kind of tell if something makes sense. When I struggled with something, I went to a couple of office hours with cargo team, which we found very helpful." -- Rust developer at a startup

But when you try to take on larger tasks, and particularly if you are trying to take on a set of larger tasks as a *company*, it can be very confusing to figure out what you should do next:[^notjustyou]

> "\[My other project\] is in a bit of a limbo. We have an MCP. It's been accepted in January, but I myself didn't really spend that much time on it since then. It's a bit unclear to me at times how I can proceed." -- that same Rust developer from the same startup
>
> "It'd be interesting to have closer cooperation in general. We are very interested in contributing more upstream. We do not want to come across as dumping a stack of requirements on the table and then not doing anything." -- Developer at a large company

[^notjustyou]: The challenge of driving large projects through Rust is, uh, not exactly a surprise. To those who have tried and failed to get traction on an idea for Rust, let me just say, "it's not just you". =)

This tracks with things that I've heard in the context of the Rust [Project Goals program](https://github.com/rust-lang/rust-project-goals/). For example, even the Rust for Linux group had challenge getting traction with the Rust org at first, but now that we have dedicated outreach and official project goals [like this one](https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-language.html), we've been able to make steady progress.

## Where do we go from here? *Create a user research team*

This brings us to our first recommendation, which is that **the Rust org needs a dedicated user research team**. The role of this team would be to gather data of all forms (interviews, surveys, etc) and make it available to the Rust project. And whenever they can, they would help to connect Rust customers directly with people extending and improving Rust. 

The vision doc process was in many ways our first foray into this kind of research, and it taught us a few things:

* **First, we have to go broad *and* deep**. For this first round, we focused on high-level questions about people's experiences with Rust, and we didn't get deep into technical blockers. This gives us a good overview but limits the depth of recommendations we can make.
* **Second, we should find ways to "open the data" and publish results incrementally**. We conducted all of our interviews with a strong guarantee of privacy and we expect to delete the information we've gathered once this project wraps up. Our goal was to ensure people could talk in an unfiltered way. This should always be an option we offer people -- but that level of privacy has a cost, which is that we are not able to share the raw data, even widely across the Rust teams, and (worse) people have to wait for us to do analysis before they can learn anything. This won't work for a long-running team. At the same time, even for seemingly innocuous conversations, posting full transcripts of conversations openly on the internet may not be the best option, so we need to find a sensible compromise.

In the meantime, we will continue writing posts like this, carving out some of the lessons we've learned this year.