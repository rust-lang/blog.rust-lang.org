+++
path = "2018/11/27/Rust-survey-2018"
title = "Rust Survey 2018 Results"
authors = ["The Rust Survey Team"]
aliases = ["2018/11/27/Rust-survey-2018.html"]
+++

Another year means another Rust survey, and this year marks Rust's third annual survey. This year, the survey launched for the first time in multiple languages. In total **14** languages, in addition to English, were covered.  The results from non-English languages totalled *25% of all responses* and helped pushed the number of responses to a new record of **5991 responses**. Before we begin the analysis, we just want to give a big "thank you!" to all the people who took the time to respond and give us your thoughts. It’s because of your help that Rust will continue to improve year after year.

![Do you use Rust](1-Do_you_use_Rust.png)

Despite having an increase in the number of responses, this year also saw an increase in the percentage of Rust users. Up from last year’s 66.9% Rust users, this year nearly three-quarters of responses were from Rust users.

# Rust Users

## **Time with Rust**

![How long have you worked in Rust](4-How_long_have_you_worked_in_Rust.png)

We’re seeing a steady stream of new users into Rust. At the time of the survey, ~23% of Rust users had been using it for 3 months or less.  Likewise, nearly a quarter of the users have used Rust for at least 2 years.  

![How long did it take to be productive](5-How_long_did_it_take_to_be_productive.png)

Over 40% of Rust users felt productive in Rust in less than a month of use, and over 70% felt productive in their first year. Unfortunately, there is a noticeable struggle among users, as over 22% do not yet feel productive.

![How long have you been unproductive](5a-How_long_have_you_been_unproductive.png)

Looking closer at these users who feel unproductive in Rust, only about 25% are in their first month of use. The challenge here is to find ways to help bridge users to productivity so they don't get stuck.

## **How much do you use Rust?**

![Size of summed Rust projects](6-Size_of_summed_Rust_projects.png)

Rust projects are continuing to trend to larger sizes, with larger overall investments. Medium to large investments in Rust (those totally over 10k and 100k lines of code respectively) have grown from 8.9% in 2016, to 16% in 2017, to **23%** this year. 

![How often do you use Rust](7-How_often_use_Rust.png)

We’ve also seen a growth in Rust regular usage. Up from 17.5% last year, Rust daily usage is now nearly a quarter of users. In total, Rust weekly total usage has risen from 60.8% to 66.4%.

## **Rust expertise**

![How would you rate your Rust expertise](8-How_you_rate_your_Rust_expertise.png)

Rather than being a simple curve, Rust expertise has two peaks: one around a "3", and another at "7", showing that users tend to see themselves as just above beginner or experienced without necessarily being expert.

![How difficult are Rust concepts](9-How_difficult_are_Rust_concepts.png)

Rust users generally felt that Enums and Cargo were largely easy concepts; followed by Iterators, Modules, and Traits. More challenging concepts of Trait Bounds and Unsafe came next. Lastly, the most challenging concepts were Macros, Ownership & Borrowing, and Lifetimes. These challenges match closely to feedback we’ve heard in years past and continue to be a focus of continued productivity improvements like NLL and the continued macro system improvements. 

![What programming languages are you familiar with](22-Programming_language_familiarity.png)

Humorously, we see that Rust isn't actually the top programming language that users were familiar with. Instead, it pulled in a 2nd place behind Python.

## **Rust toolchain**

![Which Rust version do you use](10-Which_Rust_version.png)

We’re seeing similar numbers in users of the current stable release since last year. Perhaps surprisingly, we’re continuing to see a rise in the number of users who use the Nightly compiler in their workflow. For the second year in a row, Nightly usage has continued to rise, and is now over 56% (up from 51.6% of last year).

When asked why they used nightly, people responded with a broad range of reasons including: access to 2018 edition, asm, async/await, clippy, embedded development, rocket, NLL, proc macros, and wasm.

![Has upgrading the compiler broken your code](11-Has_upgrading_compiler_broken_you.png)

The percentage of people who see a breakage during a routine compiler update has stayed the same since last year, with 7.4% saying they’ve experienced breakage.

![If so how much work to fix it](12-If_so_how_much_work_to_fix.png)

Breakages generally leaned to requiring minor fixes, though some reported having moderate or major fixes to upgrade to the next stable compiler.

![Preferred install method](13-Preferred_install_method.png)

We again see a strong showing for `rustup`, which continues to hold at 90% of Rust installs. Linux distros follow as a distant second at 17%.

![Experience with Rust tools](14-Tool-experience.png)

Tools like `rustfmt` and `rustdoc` had a strong show, with lots of positive support.  Following these is the `clippy` tool -- despite having fewer users, its users enjoy the tool.  The IDE support tools `Rust Language Server` and `racer` had positive support but unfortunately, of the tools surveyed, generated a few more dislike votes and comments. The `bindgen` tool has relatively small userbase.

## **Rust workflow**

![Which platform are you developing on](15-Platform_developing_on.png)

Linux continues to be a powerhouse among Rust developers, holding on to roughly 80% of Rust developers.  Windows usage has grown slightly from 31% last year to 34% this year, its second year in a row of growth.

![Which platforms are you developing for](16-Platforms_targeting.png)

Linux and Windows continued to show strongly as targets for Rust applications. Other platforms held largely the same as last year, with one exception: WebAssembly. The new technology has showed impressive growth, nearly doubling from last year's 13% to this year's 24%.

![What editors do you use](17-Editors.png)

Vim, the front-runner in editors for two years has now finally been bested by VSCode, which grew from 33.8% of Rust developers to 44.4% this year.

## **Rust at work**

![Do you use Rust at work](18-Rust_at_work.png)

Rust continues is slow-and-steady growth in the workplace. We're now seeing year-over-year growth of full-time and part-time Rust, growing from last year's 4.4% full-time and 16.6% part-time to this year's **8.9% full-time** and **21.2% part-time**, a doubling of full-time Rust commercial use.  In total, Rust commercial use grew from 21% to just over 30% of Rust users.

![Is your company evaluating Rust](19-Company_evaluate_Rust.png)

There is more room for Rust to grow into more companies, over a third of which users report aren't currently looking into evaluating Rust in the coming year.  When paired with the survey data that said that nearly half of non-users needed the company support, this shows the need for further company outreach or more company-focused information about Rust.

## **Feeling welcome**

![Do you feel welcome in the Rust community](23-Do_you_feel_welcome.png)

An important part of the Rust community efforts are ensuring that the Rust project is a welcoming place for its users. New users should feel encouraged to explore, share ideas, and generally be themselves.

When asked, both current Rust users and non-users largely felt welcome, though over a quarter of responses weren't sure.  There was also some regional variation in these responses.  For example, responses on the Russian version of the survey showed double the percent of unwelcome feelings at 4%. Mainland China showed even more at 8%.

There's a challenge here to help Rust communities worldwide feel like they are part of what makes Rust unique, as Rust continues to grow a strong presence in more areas of the world.

![Are you underrepresented in tech](24-Underrepresented.png)

The number of people in Rust who self-identify as being part of a group underrepresented in technology is growing slowly year-over-year. The survey also highlights some challenges, as the number of women is still lower than the industry average of women in programming fields.

# Rust Non-Users

A big part of a welcoming Rust community is reaching out to non-users as well. As we have in years past, we again asked the reasons why people weren't using Rust. 

![How long before you stopped](2-How_long_before_you_stopped.png)

For those who stopped using Rust, just over 50% stopped using Rust in less than a month. Likewise, roughly 50% of people who left Rust managed to use it for more than a month before stopping.

![Why are you not using Rust](3-Why_not_using_Rust.png)

Many non-users responded that they did want to learn Rust, but there were factors that slowed them down. First among these is that the companies the responders work for do not themselves use Rust. Nearly one half of the non-users were blocked by the lack of company support. 

Additionally, 1 in 4 non-users were slowed by the feeling of Rust being too intimidating or complicated. The work towards improving Rust IDE support has helped (down from 25% to 16%), though we still see a strong push towards even better IDE support among non-users.

# Challenges

As we've done in past years, we asked for your comments in where Rust can improve. This year, we see some familiar themes as well as some new ones in this feedback. The top ten themes this year are:

1. the need for better library support
2. a more improved IDE experience
3. the need for broader adoption of Rust generally
4. a richer ecosystem of tools and support
5. an improved learning curve
6. the need for important language features and crates to be stable and supported
7. support for async programming
8. support for GUI development
9. better documentation
10. improved compile times

New this year is the rising need to **support GUI development**, showing that Rust continues to grow not only on the server, but that people are feeling the need to stretch into application development.

> "Improve Rust marketing. Many people don't know about it"

Comments remind us that while Rust may be well-known in some circles, it still has room to grow and in many tech circles Rust may not yet be well-known. 

> "Keeping a focus on adoption/tutorials/books/novice experience will pay dividends in the years to come."

In addition to outreach, a broader set of documentation would in turn help reach a broader audience. 

> "Stability and maturity of developer tools, make it easier to get a working setup and to debug applications"

Many people commented on the IDE support, pointing out not only instability or inaccuracy in the RLS, but also the need for a much stronger IDE story that covered more areas, like easier debugging.

> "The maturity of the ecosystem and libraries. Have a good ecosystem of "standard" libraries is key for the future of the language"

A common theme continues to be the need to push libraries to completion and grow the set of "standard" libraries that users can use. Some comments point out this isn't the fault of maintainers, who are already working hard to write and publish the crates, but that generally more companies need to get involved and offer commercial support.

> "Ergonomics and discoverability of "putting it together" documentation"

Some people pointed out that ergonomics goes hand in hand with richer documentation, seeing that these aren't separate concepts but rather challenges that should be tackled together in a unified approach. 

## Looking forward

This year saw the strongest survey yet. Not only was it the largest community survey, it was the first to cover languages outside of English. Rust continues to grow steadily, and with it, both its strengths and challenges are introduced to a broader audience. 

We look forward to using your feedback in planning for 2019, and we're excited to see where we can take Rust next.
