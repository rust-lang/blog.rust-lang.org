---
layout: post
title: "Brainstorming Async Rust's Shiny Future"
author: Niko Matsakis
description: "Brainstorming Async Rust's Shiny Future"
team: the Async Foundations Working Group <https://rust-lang.github.io/wg-async-foundations/>
---

On March 18th, we [announced the start of the Async Vision Doc][announce] process. Since then, we've landed [17 "status quo" stories][sq] and we have [5 more stories in open PRs][prs]; [Ryan Levick] and [I] have also hosted more than ten [collaborative][] [writing][] [sessions][] over the course of the last few weeks.

[collaborative]: https://smallcultfollowing.com/babysteps/blog/2021/03/22/async-vision-doc-writing-sessions/
[writing]: https://smallcultfollowing.com/babysteps/blog/2021/03/25/async-vision-doc-writing-sessions-ii/
[sessions]: https://smallcultfollowing.com/babysteps/blog/2021/03/29/async-vision-doc-writing-sessions-iii/

Now that we have a good base of "status quo" stories, we are starting to imagine what the [✨ "shiny future" ✨][sf] might look like. **We want your help!** If you have a great idea for Async Rust[^youknow], then [take a look at the template and open a PR][template]! Alternatively, if you have an idea for a story but would like to discuss it before writing, you can [open a "shiny future" issue][open].

[^youknow]: Don't be modest. You know you do.

When writing "shiny future" stories, the goal is to focus on the **experience** of Rust's users first and foremost, and not so much on the specific technical details. In fact, you don't even have to know exactly how the experience will be achieved. We have a few years to figure that out, after all. 🚀

Every "shiny future" story is a "retelling" of one or more "status quo" stories. The idea is to replay the same scenario but hopefully with a happier ending, as a result of the improvements we've made. If you don't see a "status quo" story that is right for telling your "shiny future" story, though, that's no problem! Just write it now! We are still requesting "status quo" and "shiny future" stories, and we will do so right up until the end. 

### 🤔 Frequently asked questions

#### What is the async vision doc and how does it work?

Here is the idea in a nutshell:

> We are launching a collaborative effort to build a shared [vision document][vd] for Async Rust. **Our goal is to engage the entire community in a collective act of the imagination:** how can we make the end-to-end experience of using Async I/O not only a pragmatic choice, but a _joyful_ one?

Check out the [original announcement][announce] to get more details! The [How to Vision] page outlines the process we are using.

#### "Status quo" stories? "Shiny future" stories? What are you talking about?

The "status quo" and "shiny future" stories are the heart of the [Async Vision Doc][vd]. Each story describes the experiences of one or more of our four [characters] as they go about achieving their goals using Async Rust. 

The "status quo" stories describe the experiences that users have today. They are an amalgamation of the real experiences of people using Async Rust, as reported to us by interviews, blog posts, and tweets. The goal with these stories is to help us understand and gauge the cumulative impact that problems can have on our users.

The "shiny future" stories describe those some characters achieving those same goals, but looking forward a few years into the future. They are meant to illustrate the experience we are aiming towards, and to give the overall context for the RFCs and other kinds of changes we want to pursue.

#### What happens when there are multiple "shiny future" stories about the same thing?

During this brainstorming period, we want to focus on getting as many ideas as we can. Having multiple "shiny futures" that address the same problem is a feature, not a bug, as it will let us mix-and-match later to try and find the best overall plan.

#### Do we have to know exactly how we will achieve the "shiny future"?

No! Of course, we will eventually have to figure out the precise designs, but at this point we're more interested in talking about the experience we aim to create. That said, shiny future PRs can also include [design docs] that give technical details.

#### What if we write a "shiny future" story but it turns out to be impossible to implement?

Glad you asked! The vision document is a living document, and we intend to revisit it regularly. This is important because it turns out that predicting the future is hard. We fully expect that some aspects of the "shiny future" stories we write are going to be wrong, sometimes very wrong. We will be regularly returning to the vision document to check how things are going and adjust our trajectory appropriately.

#### What if I'm not sure how to get started?

[Ryan Levick] and [I] have been hosting "vision doc writing sessions" and they expect to continue doing this. These sessions are a fun way to work with other people from the Rust community. Keep an eye on the [babysteps blog] or twitter for announcements.

#### Can I still submit "status quo" stories?

Yes! We are accepting status quo stories right up until the end of the brainstorming period.

#### What is the "scope" of a shiny future story?

All the stories in the vision doc are meant to cover the full "end to end" experience of using async Rust. That means that sometimes they will take about things that are really part of projects that are outside of the Rust org. For example, we might write a shiny future that involves how the standard library has published standard traits for core concepts and those concepts have been adopted by libraries throughout the ecosystem.

#### How long does the brainstorming period last, and what happens next?

The brainstorming period lasts until the end of the month. After that, the [working group leads] are going to merge the remaining stories and get to work drafting a synthesized vision document that incorporates elements of the various stories that have been submitted (the ["harmonizing"] period).


[vd]: https://rust-lang.github.io/wg-async-foundations/vision.html#-the-vision
[sq]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo.html
[sf]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future.html
[prs]: https://github.com/rust-lang/wg-async-foundations/pulls
[announce]: https://blog.rust-lang.org/2021/03/18/async-vision-doc.html
[bp]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html#brainstorming
[template]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future/template.html
[open]: https://github.com/rust-lang/wg-async-foundations/issues/new/choose
[ws]: https://smallcultfollowing.com/babysteps/blog/2021/03/29/async-vision-doc-writing-sessions-iii/
[design docs]: https://rust-lang.github.io/wg-async-foundations/design_docs.html
[I]: https://twitter.com/nikomatsakis/
[Ryan Levick]: https://twitter.com/ryan_levick/
[How to Vision]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html
[babysteps blog]: https://smallcultfollowing.com/babysteps/
[characters]: https://rust-lang.github.io/wg-async-foundations/vision/characters.html
[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge