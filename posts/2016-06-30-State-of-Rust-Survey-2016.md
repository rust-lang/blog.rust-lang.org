---
layout: post
title: "State of Rust Survey 2016"
author: Jonathan Turner
---

We recently wrapped up with a survey for the Rust community.  Little did we know that it would grow to be one of the largest language community surveys.  A *huge* thank you to the **3,086** people who responded!  We're humbled by the response, and we're thankful for all the great feedback.

The goal of the survey was simple: we wanted to know how Rust was doing in its first year.  We asked a variety of questions to better understand how Rust was being used, how well the Rust tools worked, and what the challenges are for people wanting to adopt Rust.

We plan to run a similar survey each year to track how we're progressing and spot places we can improve.  With that, let's get started.

## Do You Use Rust?

We wanted to make sure the survey was open to both users of Rust and to people who didn't use Rust.  Rust users give us a sense of how the current language and tools are working and where we need to improve.  Rust non-users shed light on missing use-cases and obstacles for Rust’s adoption.

![Do you use Rust][do-you-use-rust]

We're happy to report that more than a third of the responses were from people not using Rust.  These respondents gave a lot of great feedback on adoption roadblocks, which we'll talk about later in this blog post.

## Growing numbers trying Rust

![Time using Rust][time-using-rust]

Almost 2,000 people responded saying they were Rust users.  Of these, almost 24% were new users.  This is encouraging to see.  The community is growing, with a healthy portion of newcomers playing with Rust now that could become long-term users.

Equally encouraging is seeing that once someone has become a Rust user, they tend to stick around and continue using it.  One might expect a sharp drop-off if users became quickly disenchanted and moved onto other technologies.  Instead, we see the opposite.  Users that come in and stay past their initial experiences tend to stay long-term, with a fairly even spread between 3 months to 12 months (when we first went 1.0).  We've seen similar patterns looking at [crates.io](https://crates.io/) usage, as well as in the [StackOverflow developer survey](https://stackoverflow.com/research/developer-survey-2016).

## Using Rust

We asked a number of questions trying to get a clear picture of what it's like to use Rust today.  The first questions focused on the Rust compiler.

![Versions of Rust you use][versions-of-rust]

In the above chart, you see the top five rustc version combinations for users writing Rust.  At the time of the survey, version 1.8 was the latest stable release.  This factors strongly in the results as the most popular version of Rust to use.  Perhaps surprisingly is how much the nightly also plays a key role in for many developers, with over 400 using it as their only Rust compiler version. Stabilizing features and APIs, and thereby encouraging transition to the stable channel, continues to be a priority for the team.

![Has an upgrade broken code][after_1_0_broke_code]

In the pre-1.0 days, Rust releases would regularly break user's code.  In reaching 1.0, we began releasing versions that maintained backwards compatibility with 1.0.  For stable Rust, 83.6% of users did not experience any breakage in their project as they upgraded to the next stable version.  Previous research based on automated testing against the ecosystem put this number [closer to 96%](https://internals.rust-lang.org/t/rust-regressions-2015-year-end-report/2993), which is more in line with expectations.

Why the discrepancy? Looking at the data more closely, it seems people used this question as a catch-all for any kind of breakage, including packages in cargo, compiler plugins needing updates, and the changes to libc.  We'll be sure to word this question more clearly in the future. But we also plan to launch a forum discussion digging further into the details, to make sure that there’s not something missing from the test automation that runs against crates.io.

![Fixing broken code][easy_to_fix]

Luckily, regardless of what bucket the breakage fell into, they were largely easy to solve as people upgraded.

![Do you like Cargo][like_cargo]

Another big piece of the Rust development experience is using the Cargo tool.  Here we saw overwhelming support for Cargo, with 94.1% of people saying they would rate it a 4 or 5.  This helps to emphasize that Cargo continues to be a core part of what it means to write Rust (and that people enjoy using it!)

## Rust at Work

An important part of a programming language's success is that it's used for "real" work.  We asked a few questions to understand how Rust was doing in the workplace.  Were people using it in their day jobs?  How much was it being used?

![Using Rust at work][rust_at_work]

We were pleasantly surprised to see that already, in Rust's first year, 16.1% of Rust users are using Rust at work part-time and 3.7% are using at work full-time.  Combined, **nearly 1/5th of Rust users are using Rust commercially**.  We're seeing this reflected in the growing number of [companies using Rust](https://www.rust-lang.org/friends.html).

We also asked about the size of the codebases that Rust developers were building.

![Size of part-time codebases][part_time]

![Size of full-time codebases][full_time]

We see strong numbers in project size as developers put more time into Rust at work.  Over half of the Rust users using Rust full-time at work have codebases that are tens or hundreds of thousands of lines of code.

Equally encouraging is the growth we expect to see in Rust in the workplace, as we see in the next chart.

![Using Rust at work in future][rust_at_work_future]

Of those not currently using Rust at work, more than 40% plan on being able to use Rust at work.  This will help carry Rust to more places and in more areas.  Speaking of carrying to more areas, we saw a wide variety of job domains represented in the survey:

![Demographics of work areas][demo_areas]

It's encouraging to see people from so many different backgrounds interested in Rust.  It underscores Rust’s potential across a broad spectrum of programming tasks and the need for libraries to support these areas.

## Challenges for Rust

An important part of the survey was understanding what's getting in the way of people using Rust.  This data can help guide our energies in the coming year.  Over **1,900** people responded here, giving us a detailed picture of the challenges with using and promoting Rust.  While we'll be exploring these responses in upcoming blog posts, here we'll look at three strong themes in the feedback: learning curve, immaturity of the language and libraries, and immaturity of the tools.

# Learning Curve

Rust is a unique language, introducing new concepts, like ownership, that are not usually explicit in languages.  While these concepts are what make Rust so powerful, they can also be an obstacle when first getting to know the language.

In total, **1 in 4** people commented on the learning curve when talking about Rust's challenges.  Here are some of the comments:

> "Borrow checker is hard to grasp for a beginner."

> "The borrow system, albeit powerful, can be difficult to learn."

> "Steep learning curve at the beginning"

The proverbial gauntlet has been thrown.  For Rust to do well, it will need to retain the power it has while also improving the experience of learning the language and mastering its ownership system.  There are a few early initiatives here, including a [new Rust book](http://rust-lang.github.io/book/), an [upcoming O'Reilly book](http://shop.oreilly.com/product/0636920040385.do), improvements to [Rust error messages](https://github.com/jonathandturner/rfcs/blob/master/text/0000-default-and-expanded-rustc-errors.md), as well as improvements to the borrow checker to [give fewer false warnings](http://smallcultfollowing.com/babysteps/blog/2016/05/04/non-lexical-lifetimes-based-on-liveness/). We expect learning curve and developer productivity to be an area of sustained focus.

# Immaturity of the Language and Libraries

Of those commenting on Rust's challenges, **1 in 9** mentioned the relative immaturity of the Rust language was a factor.  While some people pointed out their favorite missing feature, the consensus formed around the need to move the ecosystem onto the stable language and away from requiring the nightly builds of the compiler.

> "a major blocker is how many crates still only put their best foot forward if you're using a nightly compiler"

> "I don't like having to use a nightly plus a build.rs for parsing json with serde.  It needs to be simpler."

> "I also found myself unable to use a lot of nice looking crates because many were locked on nightly because of feature usage."

While there will always be a subset of users that want to live on the bleeding edge and use every new feature, it's become clear that as Rust matures it will need to build more infrastructure around the stable language and compiler.

Closely related to the language are the libraries.  People often mentioned both in the same sentence, seeing the experience of programming Rust as one built on the combination of language and library.  In total, **1 in 7** commenters mentioned the lack of libraries.  The kinds of libraries people mentioned ran the gamut in terms of topic, covering areas like GUIs, scientific/numeric computing, serialization support, web/networking, async I/O, parallel/concurrent patterns, and richer data structures (including more containers and broader coverage of general algorithms).

Of course, immaturity is to be expected one year in, and to some degree is a problem that only time can solve. But there was also a sense that people coming to Rust wanted more of a "batteries included" experience, gathering together the best of the ecosystem in a simple package.  There are some proposals in the works for how best to build this experience, and we’re looking forward to discussing these ideas in the coming weeks.

# Immaturity of the Tooling

Another strong theme for improvement was the relative immaturity of the tooling for Rust.  While tools like Cargo have been invaluable to a number of Rust users, other tools need attention.

Of non-Rust users, **1 in 4** responded that they aren't currently using Rust because of the lack of strong IDE support.  As one user puts it *"[f]or a complex language like Rust, good editor tooling makes the learning process interactive."*  Modern IDEs have become a powerful way to explore unfamiliar APIs, unfamiliar language features, and unfamiliar error messages.

Investing in IDE support not only helps new users but also helps enable teams moving to Rust and the growth of larger codebases, as we see in some of the quotes about Rust's challenges:

> "I won't use the language until there's IDE support for it, and I know other devs that feel the same way. As productive as your language's syntax is, I'm more productive in a worse language with an editor that has code-completion."

> "Users/projects considering switching languages often are not willing to sacrifice tooling quality even for a better language."

> "Proper IDE support (hard to get it accepted at work for that reason)"

Other languages have had years to build up their tooling muscle, and for Rust to stand on even footing, we'll also have to build up our own muscle.  There are some early experiments here, namely [Racer](https://github.com/phildawes/racer) and [rustw](https://github.com/nrc/rustw), as well as a [number of IDE plugins](https://areweideyet.com/).

We've also been investing in other tooling muscles, including a [new installer with cross-compilation support](http://blog.rust-lang.org/2016/05/13/rustup.html).  These are just the first steps, and we'll be exploring more ideas in further blog posts.

## Survey Demographics

![Popular meetup locations][meetup_locations]

Today, Rust has a worldwide audience.  Rather than being lumped in one place, we see Rust users in Europe, Japan, Australia, with new meetups popping up everyday.  We also asked where people who responded lived, and over 1000 of the 3000 survey responses mentioned living in Europe (with USA following it up at 835).

![Demographics on programming language background][what_languages]

The parents of most modern languages, C and C++, show strongly in terms of the programming languages that people are most comfortable with.  Close by are Java and JavaScript.  Perhaps one surprising point here is the large number of Python users attracted to Rust.

For those who already have existing projects in other languages but want to use Rust, it's worth mentioning here the on-going efforts to aide in using Rust with other languages, including work to [integrate with Ruby](https://github.com/rustbridge/helix) and [integrate with JavaScript/Node.js](https://github.com/rustbridge/neon).

![Members of underrepresented groups][underrepresented]

Rust strives to be a [warm, welcoming and inclusive community](https://www.rust-lang.org/conduct.html). The survey shows that, despite that spirit, we have a ways to go in terms of diversity. We have nascent efforts, like [RustBridge](https://github.com/rust-community/rustbridge), to more proactively reach out to underrepresented groups and make Rust more accessible, but there is a lot more work to be done.  We'll be watching the results of this part of the survey closely and continue to invest in outreach, mentoring, and leadership to foster inclusivity.

## Warm Feelings

At the end of the survey, we threw in a catch-all question: "Anything else you'd like to tell us?"  Rather than being a large batch of additional things to look at, we received an outpouring of support from the community.

I'll let some of the quotes speak for themselves:

> "Rust has been an incredible productivity boon for me. *Thank you* so much, and keep up the good work!"

> "Thank you for making Rust awesome!"

> "Working in the Rust community has been an amazing experience."

And we couldn't agree more.  One of the best things about working in Rust is that you're part of a community of people working together to build something awesome.  A big thank you(!!) to all of you who have contributed to Rust.  Rust is what it is because of you.

We'd love to hear your comments and invite you to jump in and participate in the upcoming discussions on ways we can tackle the challenges brought up in this survey.


[do-you-use-rust]: /images/2016-06-Survey/do_you_use_rust.png
[time-using-rust]: /images/2016-06-Survey/time_using_rust.png
[versions-of-rust]: /images/2016-06-Survey/versions_of_rust.png
[after_1_0_broke_code]: /images/2016-06-Survey/after_1_0_broke_code.png
[easy_to_fix]: /images/2016-06-Survey/easy_to_fix.png
[like_cargo]: /images/2016-06-Survey/like_cargo.png
[rust_at_work]: /images/2016-06-Survey/rust_at_work.png
[part_time]: /images/2016-06-Survey/part_time.png
[full_time]: /images/2016-06-Survey/full_time.png
[rust_at_work_future]: /images/2016-06-Survey/rust_at_work_future.png
[demo_areas]: /images/2016-06-Survey/demo_areas.png
[meetup_locations]: /images/2016-06-Survey/meetup_locations.png
[what_languages]: /images/2016-06-Survey/what_language.png
[underrepresented]: /images/2016-06-Survey/underrepresented.png
