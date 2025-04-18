+++
path = "2020/04/17/Rust-survey-2019"
title = "Rust Survey 2019 Results"
authors = ["The Rust Survey Team"]
aliases = ["2020/04/17/Rust-survey-2019.html"]
+++

> Translation available for [Chinese | 中文](https://web.archive.org/web/20200611004214/http://www.secondstate.info/blog/rust-2019)

Greetings Rustaceans!

We are happy to present the results of our fourth annual survey of our Rust community. Before we dig into the analysis, we want to give a big "thank you!" to all of the people who took the time to respond. You are vital to Rust continuing to improve year after year!

Let's start by looking at the survey audience.

## Survey Audience

The survey was available in **14** different languages and we received **3997** responses. 

Here is the language distribution of the responses we received.

* English: 69.6%
* Chinese: 10.8%
* German: 4.3%
* French: 3.3%
* Japanese: 3.0%
* Polish: 1.2%
* Portuguese: 1.2%
* Spanish: .9%
* Korean: .8%
* Italian: .6%
* Swedish: .5%
* Vietnamese: .2%

In the 2019 survey, 82.8% of responders indicated they used Rust, 7.1% indicated they did not currently use Rust but had used it in the past, and 10.1% indicated that they had never used Rust.

If we compare this to the 2018 survey – where 75% of responders indicated they used Rust, 8% indicated they did not currently use Rust but had used it in the past, and 8% indicated they had never used Rust – more responders were using Rust in 2019.

## Looking Back on Rust 2018

In December 2018 we released the Rust 2018 edition - Rust 1.31.0. In the 2019 survey, 92% of Rust users indicated they were using the new edition. 85% said that upgrading to the Rust 2018 edition was easy. 

Next, we asked users to rate the improvement of key aspects of the Rust language.

![How has adoption level improved](35-Adoption-Level-improvement.svg)

![How has async io improved](36-Async-IO-improvement.svg)

![How has compile time improved](37-compile-time-improvement.svg)

![How has GUI development improved](38-GUI-Development-improvement.svg)

![How has IDE support improved](39-IDE-improvement.svg)

![How has library support improved](40-Library-Support-improvement.svg)

![How have stable language features and crates improved](41-Stable-Language-Features-and-Crates-improvement.svg)

![How has Rust documentation improved](42-Rust-documentation-improvement.svg)

![How has the learning curve improved](43-learning-curve-improvement.svg)

![How has tools and support improved](44-tools-and-support-improvement.svg)

Overall, many aspects of the Rust language were perceived as "somewhat better" in the 2018 edition.

## Conferences and Community

We noticed some differences between English language and other language results. Within the non-English language survey subset, the majority of the issues and concerns identified are the same as those within the English language. However, one concern/trend stands out among the non-English speaking subset - a desire for Rust documentation in their native language, or the language they took the survey in. This was particularly notable within the Chinese-language group, though that is likely due to the higher representation.

We are tracking the [ongoing translation efforts](https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations) with the "Translation" GitHub issue label.

We received a lot of feedback on how we can improve Rust and make it feel more welcoming to more people. We can't include all of it here, so here is a summary of some of the feedback that stood out to us.

People are in general asking for more learning material about Rust. In terms of expertise it's mainly beginner and intermediate level material being requested. A lot of these requests also asked for video content specifically.

The common blockers that people mention to participating is that they have social anxiety, and accessibility. One of the common reasons mentioned was that some resources are hard to read for people with dyslexia.

Here are some specific responses to the question "What action could we take to make you feel more welcome?"
* I feel too inexperienced and under skilled to participate in the Rust community
* Advertise more ways for newcomers to contribute/participate
* More organized mentorship, online classes
* Do video tutorials on how to contribute to the compiler. I'd love to contribute but I feel intimidated
* It's not easy to find resources for newcomers to see how Rust is being used in open source projects, so that they see the action as they're learning the language.
* More tutorials/blogs that explain simple rust & coding concepts like the reader is a complete beginner
* More intermediate level tutorials. We already have a million "Introductions to Rust".
* Smaller groups of helping people - social anxiety is making it hard to talk in the Discord, for example
* Don't have synchronous meetings at late EU hours. Have fewer synchronous meetings and/or more consistently publish and aggregate meeting notes for team meetings.

These issues are definitely ones we want to address in 2020 and beyond.

## Who is using Rust and what for?

![How often do you use Rust](11-How-Often-Use-Rust.svg)

Rust daily usage has trended slightly upward at 27.63% (it was just under 25% last year and 17.5% on 2017). Daily or weekly usage has also continued to trend slightly upward. This year it was 68.52%, last year it was 66.4%, and in 2017 it was 60.8%.

![How would you rate your Rust expertise](12-How-Rate-Rust-Expertise.svg)

We also asked users how they would rate their Rust expertise - there is a clear peak around "7".

![How would you rate your Rust expertise](29-Rust-expertise-how-long-using-Rust.svg)

To dig deeper into this, we correlated users' self-rated Rust expertise with how long they had been using Rust.

![What title best matches your role for Rust users](24-use-rust-role-title.svg)

For some larger context, we examined what titles users working with Rust full time tend to have in their organization (survey respondents could select more than one).

By far the most common title for a Rust user is, unsurprisingly, Programmer/Software Engineer.

![What industry do you work in for Rust users](25-use-rust-industry.svg)

To get even more context, we asked Rust survey respondents to identify what industry they work in.

For users who use Rust full time, the most common industry by far is backend web applications.

![Size of summed Rust projects](10-Size-Of-Summed-projects.svg)

The majority of Rust projects (43%) are 1,000-10,000 lines of code. Rust projects of medium to large size (those totaling over 10k lines of code) continue to trend higher. They have grown from 8.9% in 2016, to 16% in 2017, to 23% in 2018, to **34%** in 2019.

## Why not use Rust?

A big part of a welcoming Rust community is reaching out to non-users as well.

![Why did you stop using Rust?](4-Why-Stopped-Rust.svg)

When we asked why someone had stopped using Rust, the most common response was "My company doesn't use Rust" - which indicates Rust adoption is still the biggest reason. After that, learning curve, lack of needed libraries, being slowed down by switching to Rust, and lack of IDE support were the most common reasons a user stopped using Rust.

![Why have you never used Rust?](5-Why-Never-Used-Rust.svg)

For users who indicated they had never used Rust before, most indicated either "I haven't learned Rust yet, but I want to" or "My company doesn't use Rust" - again pointing to adoption as the main hurdle.

For more context, we also examined what title non-Rust users feel best matches their role. 

![What title best matches your role for non Rust users](26-not-using-rust-title.svg)

Like with Rust users, by far the most common title is Programmer/Software Engineer.

![What industry do you work in for non Rust users](27-not-using-rust-industry.svg)

Also like with Rust users, the most common industry by far is backend web applications.

![I would use Rust more often if...](20-Would-use-rust-more-often-if.svg)

We also asked users what would lead them to use Rust more often. Most indicated they would use Rust more if their company adopted it, if Rust had more libraries that they need, and if IDE support was better. The most common reasons after those pointed to a need to improve the learning curve and interoperability. 

As adoption seemed to be the biggest problem preventing some respondents from using Rust, let's dive deeper into it. 

## Rust Adoption - a Closer Look

First, we asked what would we could do to improve adoption of Rust.

![How can we improve Rust for better adoption](45-improve-adoption.svg)

Several users gave specific examples:
* "Smoothest learning curve as possible, as a small business even 4-6 weeks to become productive is a lot to ask"
* "Higher market penetration"
* "More stable libraries"
* "A full-stack web framework like Rails, Django and Phoenix"
* "Better documentation, more examples, recommendation on what crates to use"
* "More emphasis on how it is a safer alternative to C or C++ (and really should be the default usually).”
* "Improve compile times. Compiling development builds at least as fast as Go would be table stakes for us to consider Rust. (Release builds can be slow."
* "Better platform support"
* "Security and performance, cost efficient and "green" (low carbon footprint) language"
* "Embedded development targeting ARM"
* "Better GUI framework, similar to Qt or directly using Qt via bindings."

Most indicated that Rust maturity - such as more libraries and complete learning resources and more mature production capabilities - would make Rust more appealing.

Let's take a closer look at each of these, starting with the need for more mature libraries.

## Libraries - a Closer Look

When we asked users what libraries they consider critical to the Rust ecosystem, these were the top ten responses:
* serde
* rand
* tokio
* async
* clap
* regex
* log
* futures
* hyper
* lazy_static

![What dependencies are 1.0 or above](28-dependencies-1-0-or-above.svg)

We also asked how many dependencies users were using were 1.0 or above.
* 0.8% indicated "All"
* 6.7% indicated "Most"
* 65.9% indicated "Some"
* 5.2% indicated "None"
* 21.4% indicated "I don't know"

## IDEs and Tooling - a Closer Look

IDE support for Rust was also cited as a barrier to adoption.

![What editor are you using](31-editor-using.svg)

When we asked users what editors they use, Vim and VSCode were the most popular by far, followed by Intellij.

We also asked what IDE setups users used:
* 43.3% indicated RLS
* 21.7% indicated Intellij
* 15.2% indicated Rust-analyzer
* 12.4% indicated No (or CTAGS)
* 4.2% indicated Only Racer

![What platform are you developing on](32-what-platform-developing-on.svg)

As for platforms that users develop on - Linux and Windows continue to dominate. 
* 55% of Rust users develop on Linux
* 24% develop on Windows
* 23% develop on macOS

We found that the vast majority of all users use the current stable version of Rust (63%). It should be noted that the survey allowed respondents to select more than one option for what Rust version they use.

* 30.5% use the nightly version
* 2.5% use the Beta release
* 63% use the current stable version
* 3.1% use a previous stable release
* 0.6% use a custom fork
* 0.3% don't know 

Surprisingly, the number of users using the Nightly compiler in their workflow is down at 20%. Last year it was at over 56%.

## Learning Curve - a Closer Look

Rust is well known for its significant learning curve. 

![How long did it take to be productive](8-How-Long-To-Be-Productive.svg)

About 37% of Rust users felt productive in Rust in less than a month of use - this is not too different from the percentage last year (40%). Over 70% felt productive in their first year. Unfortunately, like last year, there is still a struggle among users - 21% indicated they did not yet feel productive.

![Expertise level of respondents who don't feel productive yet](22-unproductive-expertise.svg)

As a point of interest, we took the subset of users who don't feel productive yet and plotted their ratings of their Rust expertise. This indicates that people who don't feel productive had low to intermediate levels of expertise - which are the groups that need the most support from our learning materials, documentation, and more.

## Interoperability - a Closer Look

Over the years some users have expressed a desire for Rust to be more interoperable with other languages.

![What languages would you want to use with Rust](23-interoperability-languages.svg)

When we asked users what languages they would want to be interoperable with Rust, there was a wide spread of answers, but C dominates, followed (somewhat surprisingly) by R, which is followed very closely behind by C++. It should be noted that respondents were able to select more than one language in response to this question - these percentages are based on total responses.

![What platforms are you targeting](30-platforms-targeting.svg)

When it comes to what platforms using are targeting for their applications Linux remains the first choice with 36.9%, with Windows as second at 16.3%. Following close behind Windows are macOS and Web Assembly at 14% each. We are also seeing more users targeting Android and Apple iOS.

## Conclusions

Overall our users indicated that productivity is still an important goal for their work (with or without using Rust). The results show the overriding problem hindering use of Rust is adoption. The learning curve continues to be a challenge - we appear to most need to improve our follow through for **intermediate** users - but so are libraries and tooling.

Thank you to all who participated in this survey - these results are immensely informative to us - especially how we can  improve both Rust the language and the entire Rust ecosystem. We look forward to continuing working for and with you for 2020 and beyond!
