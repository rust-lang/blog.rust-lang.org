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

Over the course of this year, the [Vision Doc group](https://rust-lang.org/governance/teams/launching-pad/#team-project-vision-doc-2025) has gathered up a lot of data. We began with a [broad-based survey that got about 4200 responses][vds]. After that, we conducted over 70 interviews, each one about 45 minutes, with as broad a set of Rust users as we could find[^notaswide].

[vds]: https://blog.rust-lang.org/2025/04/04/vision-doc-survey/

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

## Step 1: Broad-based survey

Before embarking on individual interviews, we wanted to get a broad snapshot of Rust usage. We also wanted to find a base of people that we could talk to. We created a survey that asked a few short "demographic" questions -- e.g., where does the respondent live, what domains do they work on, how would they rate their experience -- and some open-ended questions about their journey to Rust, what kind of projects they feel are a good fit for Rust, what they found challenging when learning, etc. It also asked for (optional) contact information.

We got a LOT of responses -- over 4200! Analyzing this much data is not easy, and we were very grateful to [Kapiche](https://www.kapiche.com/), who offered us free use of their tool to work through the data. ❤

The survey is useful in two ways. First, it's an interesting data-set in its own right, although you have to be aware of selection bias. Second, the survey also gave us something that we can use to cross-validate some of what we heard in 1:1 interviews and to look for themes we might otherwise have missed. And of course it gave us additional names of people we can talk to (though most respondents didn't leave contact information).

## Step 2: Interviewing individuals

The next step after the survey was to get out there and talk to people. We sourced people from a lot of places: the survey and personal contacts, of course, but we also sat down with people at conferences and went to meetups. We even went to a Python meetup in an effort to find people who were a bit outside the usual "Rust circle".

When interviewing people, the basic insight of User Experience research is that you don't necessarily ask people the exact questions you want to answer. That is likely to get them speculating and giving you the answer that they think they "ought" to say. Instead, you come at it sideways. You ask them factual, non-leading questions. In other words, you certainly don't say, "Do you agree the borrow checker is really hard?" And you probably don't even say, "What is the biggest pain point you had with Rust?" Instead, you might say, "What was the last time you felt confused by an error message?" And then go from there, "Is this a typical example? If not, what's another case where you felt confused?"

To be honest, these sorts of "extremely non-leading questions" are kind of difficult to do. But they can uncover some surprising results.

## We got answers -- but not all the answers we wanted

4200 survey responses and 70 interviews later, we got a lot of information -- but we still don't feel like we have the answers to some of the biggest questions. Given the kinds of questions we asked, we got a pretty good view on the kinds of things people love about Rust and what it offers relative to other languages. We got a sense for the broad areas that people find challenging. We also learned a few things about how the Rust project interacts with others and how things vary across the globe.

What we really *don't* have is enough data to say "if you do X, Y, and Z, that will really unblock Rust adoption in this domain". We just didn't get into enough technical detail, for example, to give guidance on which features ought to be prioritized, or to help answer specific design questions that the lang or libs team may consider.

## One big lesson: there are only 24 hours in a day

One of the things we learned was that you need to stay focused. There were so many questions we wanted to ask, but only so much time in which to do so. Ultimately, we wound up narrowing our scope in several ways:

* we focused primarily on the individual developer experience, and only had minimal discussion with companies as a whole;
* we dove fairly deep into one area (the Safety Critical domain) but didn't go as deep into the details of other domains;
* we focused primarily on Rust adoption, and in particular did not even attempt to answer the questions about "Rust the open-source project".

## Another big lesson: haters gonna... stay quiet?

One thing we found surprisingly difficult was finding people to interview who *didn't* like Rust. 49% of survey respondents, for example, rated their Rust comfort as 4 or 5 out of 5, and only 18.5% said 1 or 2. And of those, only a handful gave contact information.

It turns out that people who think Rust isn't worth using mostly don't read the Rust blog or want to talk about that with a bunch of Rust fanatics.[^shocking] This is a shame, of course, as likely those folks have a lot to teach us about the boundaries of where Rust adds value. We are currently doing some targeted outreach in an attempt to grow our scope here, so stay tuned, we may get more data.

[^shocking]: Shocking, I know. But, actually, it is a *little* -- most programmers love telling you how much they hate everything you do, in my experience?

## One fun fact: enums are Rust's underappreciated superpower

We will do a deeper dive into the things people say that they like about Rust later (hint: performance and reliability both make the cut). One interesting thing we found was the number of people that talked *specifically* about Rust enums, which allow you to package up the state of your program along with the data it has available in that state. Enums are a concept that Rust adapted from functional languages like OCaml and Haskell and fit into the system programming setting.

>  "The usage of Enum is a new concept for me. And I like this concept. It's not a class and it's not just a boolean, limited to false or true. It has different states." -- New Rust developer
>
> "Tagged unions. I don't think I've seriously used another production language which has that. Whenever I go back to a different language I really miss that as a way of accurately modeling the domain." -- Embedded developer

## Where do we go from here? *Create a user research team*

When we set out to write the vision doc, we imagined that it would take the form of an RFC. We imagined that RFC identifying key focus areas for Rust and making other kinds of recommendations. Now that we've been through it, we don't think we have the data we need to write that kind of RFC (and we're also not sure if that's the right kind of RFC to write). But we did learn a lot and we *are* convinced of the importance of this kind of work.

Therefore, our plan is to do the following. First, we're going to write-up a series of blog posts diving into what we learned about our research questions along with other kinds of questions that we encountered as we went.

Second, we plan to author an RFC proposing a **dedicated user research team for the Rust org**. The role of this team would be to gather data of all forms (interviews, surveys, etc) and make it available to the Rust project. And whenever they can, they would help to connect Rust customers directly with people extending and improving Rust. 

The vision doc process was in many ways our first foray into this kind of research, and it taught us a few things:

* **First, we have to go broad *and* deep**. For this first round, we focused on high-level questions about people's experiences with Rust, and we didn't get deep into technical blockers. This gives us a good overview but limits the depth of recommendations we can make.
* **Second, to answer specific questions we need to do specific research**. One of our hypotheses was that we could use UX interviews to help decide thorny questions that come up in RFCs -- e.g., the notorious debate between `await x` and `x.await` from yesteryear. What we learned is "sort of". The broad interviews we did *did* give us information about what kinds of things are important to people (e.g., convenience vs reliability, and so forth), and we'll cover some of that in upcoming write-ups. But to shed light on specific questions (e.g., "will `x.await` be confused for a field access") will really require more specific research. This may be interviews but it could also be other kinds of tests. These are all things though that a user research team could help with.
* **Third, we should find ways to "open the data" and publish results incrementally**. We conducted all of our interviews with a strong guarantee of privacy and we expect to delete the information we've gathered once this project wraps up. Our goal was to ensure people could talk in an unfiltered way. This should always be an option we offer people -- but that level of privacy has a cost, which is that we are not able to share the raw data, even widely across the Rust teams, and (worse) people have to wait for us to do analysis before they can learn anything. This won't work for a long-running team. At the same time, even for seemingly innocuous conversations, posting full transcripts of conversations openly on the internet may not be the best option, so we need to find a sensible compromise.


