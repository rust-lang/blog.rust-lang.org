---
layout: post
title: "Brainstorming Async Rust's Shiny Future"
author: Niko Matsakis
description: "Brainstorming Async Rust's Shiny Future"
team: the Async Foundations Working Group <https://rust-lang.github.io/wg-async-foundations/>
---

On March 18th, we [announced the start of the Async Vision Doc][announce] process. Since then, we've landed [24 "status quo" stories][sq] and we have [4 more stories in open PRs][prs]; [Ryan Levick] and [I] have also hosted more than ten collaborative writing sessions over the course of the last few weeks, and we have [more scheduled for this week][cws].

[cws]: https://smallcultfollowing.com/babysteps/blog/2021/04/12/async-vision-doc-writing-sessions-v/

Now that we have a good base of "status quo" stories, we are starting to imagine what the [✨ "shiny future" ✨][sf] might look like. **We want your help!** If you have a great idea for Async Rust[^youknow], then [take a look at the template and open a PR][template]! Alternatively, if you have an idea for a story but would like to discuss it before writing, you can [open a "shiny future" issue][open]. Also, we would still love to get more ["status quo" stories][sq], so please continue to share those.

[^youknow]: Don't be modest. You know you do.

When writing "shiny future" stories, the goal is to focus on the **experience** of Rust's users first and foremost, and not so much on the specific technical details. In fact, you don't even have to know exactly how the experience will be achieved. We have a few years to figure that out, after all. 🚀

Every "shiny future" story is a "retelling" of one or more "status quo" stories. The idea is to replay the same scenario but hopefully with a happier ending, as a result of the improvements we've made. If you don't see a "status quo" story that is right for telling your "shiny future" story, though, that's no problem! Write up your story and we'll figure out the "status quo" story it addresses. There is always the option of writing a new "status quo" story, too; we are still requesting "status quo" and "shiny future" stories, and we will do so right up until the end.

If you'd like to see what a "shiny future" story looks like, we have merged one example, [Barbara Makes a Wish](https://rust-lang.github.io/wg-async-foundations/vision/shiny_future/barbara_makes_a_wish.html). This story describes Barbara's experiences using a nifty new tool that gives her lots of information about the state of her async executor. It is a "retelling" of the "status quo" story [Barbara Wants Async Insights](https://rust-lang.github.io/wg-async-foundations/vision/status_quo/barbara_wants_async_insights.html).

#### What is the async vision doc and how does it work?

Here is the idea in a nutshell:

> We are launching a collaborative effort to build a shared [vision document][vd] for Async Rust. **Our goal is to engage the entire community in a collective act of the imagination:** how can we make the end-to-end experience of using Async I/O not only a pragmatic choice, but a _joyful_ one?

As described in the [original announcement][announce], the [vision document][vd] is structured as a series of "status quo" and "shiny future" stories. Each story describes the experiences of one or more of our four [characters] as they go about achieving their goals using Async Rust.

The "status quo" stories describe the experiences that users have today. They are an amalgamation of the real experiences of people using Async Rust, as reported to us by interviews, blog posts, and tweets. The goal with these stories is to help us understand and gauge the cumulative impact that problems can have on our users.

The "shiny future" stories describe those some characters achieving those same goals, but looking forward a few years into the future. They are meant to illustrate the experience we are aiming towards, and to give the overall context for the RFCs and other kinds of changes we want to pursue.

### The brainstorming period and what comes next

We are currently in the midst of the [brainstorming period][bp]. This means that we are seeking to collect as many stories -- both about the "status quo" and the "shiny future" -- as we can. The brainstorming period lasts until the end of April. After that, the [working group leads] are going to merge the remaining stories and get to work drafting a synthesized vision document that incorporates elements of the various stories that have been submitted.

Going forward, we plan to revisit the vision document regularly. We fully expect that some aspects of the "shiny future" stories we write are going to be wrong, sometimes very wrong. We will be regularly returning to the vision document to check how things are going and adjust our trajectory appropriately.

### This sounds cool, how can I get involved?

If you'd like to help, we'd love to have you! If you've got an idea for a story, then feel free to create a PR to the wg-async-foundations repository based on one of the following templates:

* [Template for "status quo" stories][sqtemplate]
* [Template for "shiny future" stories][template]


If you'd like a bit more inspiration, then you can join [Ryan Levick] and [I] at one of our vision doc writing sessions. We have [more sessions scheduled this week][cws] and you can look for announcements from us on twitter or check the `#wg-async-foundations` stream on [the rust-lang Zulip][z].

[vd]: https://rust-lang.github.io/wg-async-foundations/vision.html#-the-vision
[sq]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo.html
[sf]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future.html
[sq]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo.html
[prs]: https://github.com/rust-lang/wg-async-foundations/pulls
[announce]: https://blog.rust-lang.org/2021/03/18/async-vision-doc.html
[bp]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html#brainstorming
[template]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future/template.html
[sqtemplate]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/template.html
[open]: https://github.com/rust-lang/wg-async-foundations/issues/new/choose
[ws]: https://smallcultfollowing.com/babysteps/blog/2021/03/29/async-vision-doc-writing-sessions-iii/
[design docs]: https://rust-lang.github.io/wg-async-foundations/design_docs.html
[I]: https://twitter.com/nikomatsakis/
[Ryan Levick]: https://twitter.com/ryan_levick/
[How to Vision]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html
[babysteps blog]: https://smallcultfollowing.com/babysteps/
[characters]: https://rust-lang.github.io/wg-async-foundations/vision/characters.html
[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[z]: https://rust-lang.zulipchat.com/
[working group leads]: https://rust-lang.github.io/wg-async-foundations/#leads
