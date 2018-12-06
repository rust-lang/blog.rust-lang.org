---
layout: post
title: "Rust 2017 Survey Results"
author: Jonathan Turner
---

It's that time of the year, where we take a good look at how things are going by asking the community at large -- both Rust users and non-users.  And wow, did you respond!

This year we had **5,368 responses**.  *That's over 2,000 more responses than we had last year*!

The scale of the feedback was both inspiring and humbling, and we've worked hard to read through all of your comments and suggestions.  There were so many helpful ideas and experiences that people shared, and we truly do appreciate it.  Without further ado, let's take a look.

![Chart: 66.9% Rust users, 9.9% stopped using, 23.2% never used](/images/2017-09-05-Rust-2017-Survey-Results/do_you_use_rust.png)

Just as we saw last year, 2/3rds of responses are from Rust users and the remainder from non-users. This year, we separated out the "don't use Rust" to capture those who used Rust and stopped from those who never used Rust. It's inspiring to see so many developers wanting to help us make Rust better (even if they don't use it!) so that it can reach even more people.

We'll talk more about Rust non-users later in this report, but first let's look at the responses from Rust users.

# Using Rust

![Chart: 0.5% less than a day, 4.2% less than a week, 13.1% less than a month, 39.7% less than a year, 42.5% over a year](/images/2017-09-05-Rust-2017-Survey-Results/how_long_using_rust.png "0.5% less than a day, 4.2% less than a week, 13.1% less than a month, 39.7% less than a year, 42.5% over a year")
*(hover for more info)*

This year, we're seeing a growing amount of experienced users sticking with Rust, with the "more than a year" users growing to over 42% (up from 30% from last year).  The beginners are also an impressively large set, with the "less than a month" crowd at just about 18%, meaning we're currently attracting nearly a 1/5th of our user base size, even as it grows larger, every month.

![Chart: 36.5% less 1000 lines, 46.3% 1000 to 10000 lines, 14.2% 10000 to 100000 lines, 2.1% over 100000, 0.9% don't know](/images/2017-09-05-Rust-2017-Survey-Results/size_of_rust_projects.png "36.5% less 1000 lines, 46.3% 1000 to 10000 lines, 14.2% 10000 to 100000 lines, 2.1% over 100000, 0.9% don't know")
*(hover for more info)*

People are working with ever-larger amounts of Rust, with medium- and large-scale lines of code totals both nearly doubling since last year as a percentage of the whole, now making up 16% of respondents (up from last year's 8.9%).  This shows a growing interest in using Rust in ever-larger projects, and a growing need for tools to support this growth.

![Chart: 17.5% daily, 43.3% weekly, 24.4% monthly, 14.9% rarely](/images/2017-09-05-Rust-2017-Survey-Results/how_often_use_rust.png "17.5% daily, 43.3% weekly, 24.4% monthly, 14.9% rarely")

Despite the rising amount of code developers are working with, we're seeing a small downtick in both daily and weekly users.  Daily users have fallen from 19% to 17.5%, and weekly users have fallen from 48.8% to 43.3%. This could be a natural transition in this stage of our growth, as a broader range of developers begin using Rust.

# Path to Stability

![Chart: 92.5% no, 7.5% yes](/images/2017-09-05-Rust-2017-Survey-Results/upgrade_stable.png "92.5% no, 7.5% yes")

In the last year, we made big strides in breakages caused by releases of the compiler.  Last year, 16.2% of respondents said that upgrading to a new stable Rust compiler broke their code.  This year, that number has **fallen to 7.5% of respondents**.  This is a huge improvement, and one we're proud of, though we'll continue working to push this down even further.

![Chart show strong support for nightly and current stable releases](/images/2017-09-05-Rust-2017-Survey-Results/which_version.png)

Developers have largely opted to move to nightly or a recent stable (with some on beta), showing that developers are eager to upgrade and do so quickly.  This simplifies the support structure a bit from last year, where developers were on a wider range of versions.

Stable users now make up 77.9% of Rust users. Unfortunately, despite our efforts with procedural macros and helping move crates like Serde to stable, we still have work to do to promote people moving away from the nightly Rust compiler.  This year shows an increase in nightly users, now at 1,852 votes it represents 51.6% of respondents using nightly, up from 48.8% of last year.

# How we use Rust

![Chart: 90.2% rustup, 18.9% linux distros, 5% homebrew, 4.7% official .msi, 3.1% official tarball, 1.4% official mac pkg](/images/2017-09-05-Rust-2017-Survey-Results/rustup.png "90.2% rustup, 18.9% linux distros, 5% homebrew, 4.7% official .msi, 3.1% official tarball, 1.4% official mac pkg")
*(hover for more info)*

One of the big success stories with Rust tooling was rustup, the Rust toolchain installer.  Last year, we saw a wide diversity in ways people were installing Rust.  This year, many of these have moved to using rustup as their main way of installing Rust, totalling now 3,205 of the responses, which moves it from last year's 52.8% to **90.2%**.

![Chart: 80.9% Linux, 35.5% macOS, 31.5% Windows, 3.2% BSD-variant](/images/2017-09-05-Rust-2017-Survey-Results/platforms_on.png)

Linux still features prominently as one of the main platforms Rust developers choose.  Of note, we also saw a rise in the use of Windows as a developer platform at 1,130 of the 3,588 total respondents, putting it at **31.5% of respondents**, up from 27.6% of last year.

![Chart: 91.5% Linux, 46.7% Windows, 38.2% macOS, 16.8% embedded, 13.2% WebAssembly and asm.js, 9.9% Android, 8.9% BSD-variant, 5.3% Apple iOS](/images/2017-09-05-Rust-2017-Survey-Results/platforms_targetted.png)

Next, we asked what platforms people were targeting with their Rust projects.  While we see a similar representation of desktop OSes here, we also see a growing range of other targets.  Android and iOS are at healthy 9.9% and 5.3% respectively, both almost **10x larger** than last year's percentages.  Embedded also has had substantial growth since last year's single-digit percentage.  As a whole, cross-compilation has grown considerably since this time last year.

![Chart: 45.8% vim, 33.8% vscode, 16.1% intellij, 15.7% atom, 15.4% emacs, 12.2% sublime, 1.5% eclipse, 1.5% visual studio](/images/2017-09-05-Rust-2017-Survey-Results/editors.png)

Among editors, vim remains king, though we see healthy growth in VSCode adoption at 34.1% (up from last year's 3.8%).  This growth no doubt has been helped by VSCode being one of the first platforms to get support for the [Rust Language Server](https://github.com/rust-lang-nursery/rls).

![Chart: 4.4% full-time, 16.6% part-time, 2.9% no but company uses Rust, 57.6% no, 2.4% not sure, 16.1% not applicable](/images/2017-09-05-Rust-2017-Survey-Results/use_rust_at_work.png "4.4% full-time, 16.6% part-time, 2.9% no but company uses Rust, 57.6% no, 2.4% not sure, 16.1% not applicable")
*(hover for more info)*

Rust in the workplace has also continued to grow.  This year's **4.4% full-time** and **16.6% part-time** Rust workers show a tick up from last year's 3.7% full-time and 16.1% part-time.

![Two charts part-time: 2016: 29.6% less than 1000 lines, 55.3% 1000 to 10000 lines, 14.5% 10000 to 100000 lines, 0.6% 100000 lines. 2017: 18.9% less than 1000 lines, 56% 1000 to 10000 lines, 23.1% 10000 to 100000 lines, 2% more than 100000 lines](/images/2017-09-05-Rust-2017-Survey-Results/part_time.png)

Users who use Rust **part-time** in their companies showed a growth in larger projects since last year, with the medium- and large-scale projects taking up more percentage of total projects this time around.

![Two charts full-time: 2016: 4.4% less than 1000 lines, 42.6% 1000 to 10000 lines, 39.7% 10000 to 100000 lines, 13.2% more than 100000 lines. 2017: 1.9% less than 1000 lines, 27.9% 1000 to 10000 lines, 52.6% 10000 to 100000 lines, 17.5% more than 100000 lines](/images/2017-09-05-Rust-2017-Survey-Results/full_time.png)

Likewise, **full-time** Rust commercial users saw medium- and large-scale projects grow to taking a larger part of the pie, with projects over 100,000 lines of code making up almost 18% of the all full-time commercial respondents, and a large shift in the 10,000-100,000 lines range from 39.7% up to **52.6%**.

# Feeling Welcome

![chart: 75.1% feel welcome, 1.3% don't feel welcome, 23.6% don't know](/images/2017-09-05-Rust-2017-Survey-Results/feel_welcome.png "75.1% feel welcome, 1.3% don't feel welcome, 23.6% don't know")
*(hover for more info)*

An important piece of the Rust community is to be one that is welcoming to new users, whether they are current users or potential users in the future.  We're pleased to see that over 3/4th of all respondents said they feel welcome in the Rust community, with 23.6% not sure.

![chart showing 81.4% not underrepresented, and a variety of underrepresented, with no category above 5%](/images/2017-09-05-Rust-2017-Survey-Results/diversity.png)

The demographics of respondents stayed about the same year over year.  Diversity and inclusiveness continue to be vital goals for the Rust project at all levels. The [Rust Bridge](https://github.com/rust-community/rustbridge/) initiative aims for diversity at the entry level.  The [Rust Reach](https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html) project, launched this year, brings in a wide range of expertise from people underrepresented in the Rust world, and pairs them with Rust team members to make Rust more accessible to a wider audience.

# Stopped using Rust

New this year, we separated out the people who had stopped using Rust from those who had never used Rust to better understand why they stopped.  Let's take a look first at when they stopped.

![chart: 3.2% less than a day, 18.5% less than a week, 43.1% less than a month, 30.2% less than a year, 4.9% more than a year](/images/2017-09-05-Rust-2017-Survey-Results/stopped_using_rust.png "3.2% less than a day, 18.5% less than a week, 43.1% less than a month, 30.2% less than a year, 4.9% more than a year")
*(hover for more info)*

The first surprise we had here was how long people gave Rust a try before they stopped.  Our initial hunch was that people would give up using Rust in the first day, or possibly the first week, if it didn't suit them or their project.  Instead, what we see is that people tried Rust for a much longer time on average than that.

Themes from people who stopped using Rust:

* 23% responded that Rust is **too difficult to use**.
* 20% responded that they **didn't have enough time** to learn and use Rust effectively.
* 10% responded that **tools aren't use mature enough**.
* 5% responded they needed **better IDE support**.
* The rest of users mentioned the need for **support for Rust in their jobs**, they'd **finished the project** they need to use Rust in, were turned away by **Rust's syntax**, **couldn't think of a project to build**, or had a **bad interaction with the Rust community**.

# Not using Rust

![chart: 666 company doesn't use Rust, 425 Rust is too intimidating hard to learn or too complicated, 295 Rust doesn't solve a problem for me, 255 Rust doesn't have good IDE support, 209 Rust doesn't have libraries I need, 161 Rust seems too risky for production, 89 Rust doesn't support platforms I need, 73 Rust doesn't have tools I need](/images/2017-09-05-Rust-2017-Survey-Results/dont_use_rust.png)

While the learning curve and language complexity still played a role in preventing people from picking up Rust, one aspect that resonated with many people is that there just simply aren't enough active commercial projects in Rust for people to be a part of.  For some, they could surmount the learning curve if there was strong incentive to do so.

# Areas for Improvement

Finally, at the end of the survey we we provided a free-form area to talk about where Rust could improve.  Before we get to the themes we saw, we wanted to give a big "thank you!" to everyone who posted thoughtful comments.  There are many, many good ideas, which we will be making available to the respective sub-teams for future planning.  With that, let's look at the themes that were important this year:

* 17% of responses underscored the need for **better ergonomics** in the language. People had many suggestions about how to improve Rust for day-to-day use, to allow for easier prototyping, to work with async programming more easily, and to be more flexible with more data structure types.  Just as before, the need for a much easier and smoother experience with the borrow checker and how to work with lifetimes was a popular request.
* 16% of responses talk about the importance of creating **better documentation**.  These covered a topics from helping users transition from other languages, creating more examples and sample projects, helping people get started with various tasks or crates, and creating video resources to facilitate learning.
* 15% of responses point out that **library support** needs to improve.  People mention the need for a strong support set of core libraries, of the difficulty finding high quality crates, the general maturity of the crates and the crate ecosystem, the need for libraries to cover a wide range of areas (eg web, GUIs, networking, databases, etc).  Additionally, people mentioned that libraries can be hard to get started with depending on their API design and amount of documentation.
* 9% of the responses encouraged us to continue to build our **IDE support**. Again, this year underscored that there are a sizeable section of developers that need support for Rust in their IDEs and tools.  The Rust Language Server, the on-going effort to support IDEs broadly, was mentioned as one of the top items people are looking forward to this year, and comments pointed to these efforts needing to reach stable and grow support into other IDEs as well as continuing to grow the number of available features.
* 8% of responses mention **learning curve** specifically.  As more developers try to pick up Rust or teach it to coworkers and friends, they're finding that there aren't sufficient resources to do so effectively and that Rust itself resists a smooth learning experience.
* Other strong themes included the need for: **faster compile times**, **more corporate support of Rust** (including jobs), **better language interop**, **improved tooling**, **better error messages**, ***more* marketing**, ***less* marketing**, and **improved support for web assembly**.

# Conclusion

We're blown away by the response this year.  Not only is this a much larger number of responses than we had last year, but we're also seeing a growing diversity in what people are using Rust for.  Thank you so much for your thoughtful replies.  We look forward to using your feedback, your suggestions, and your experience to help us plan for next year.

