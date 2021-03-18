---
layout: post
title: "Building a shared vision for Async Rust"
author: Niko Matsakis
description: "Building a shared vision for Async Rust"
team: the Async Foundations Working Group <https://rust-lang.github.io/wg-async-foundations/>
---

[wg]: https://rust-lang.github.io/wg-async-foundations/
[vd]: https://rust-lang.github.io/wg-async-foundations/vision.html#-the-vision
[sq]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo.html
[sf]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future.html
[cc]: https://rust-lang.github.io/wg-async-foundations/vision/characters.html
[htv]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision.html
[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[workarounds]: https://github.com/rust-lang/async-book/tree/a927107bfe501a44dde1560a5942b1471c11c71d/src/07_workarounds
[Grace]: https://rust-lang.github.io/wg-async-foundations/vision/characters/grace.html
[Alan]: https://rust-lang.github.io/wg-async-foundations/vision/characters/alan.html
[soflow]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/alan_runs_into_stack_trouble.html
[awards]: https://rust-lang.github.io/wg-async-foundations/vision/how_to_vision/awards.html
[gsq]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/grace_deploys_her_service.html
[template]: https://rust-lang.github.io/wg-async-foundations/vision/status_quo/template.html

The [Async Foundations Working Group][wg] believes Rust can become one of the most popular choices for building distributed systems, ranging from embedded devices to foundational cloud services. Whatever they're using it for, we want all developers to love using Async Rust. For that to happen, we need to move Async Rust beyond the "MVP" state it's in today and make it accessible to everyone.

We are launching a collaborative effort to build a shared [vision document][vd] for Async Rust. **Our goal is to engage the entire community in a collective act of the imagination:** how can we make the end-to-end experience of using Async I/O not only a pragmatic choice, but a _joyful_ one?

### The vision document starts with the status quo...

The "vision document" starts with a [cast of characters][cc]. Each character is tied to a particular Rust value (e.g., performance, productivity, etc) determined by their background; this background also informs the expectations they bring when using Rust. 

Let me introduce you to one character, [Grace]. As an experienced C developer, Grace is used to high performance and control, but she likes the idea of using Rust to get memory safety. Here is her biography:

> Grace has been writing C and C++ for a number of years. She's accustomed to hacking lots of low-level details to coax the most performance she can from her code. She's also experienced her share of epic debugging sessions resulting from memory errors in C. She's intrigued by Rust: she likes the idea of getting the same control and performance she gets from C but with the productivity benefits she gets from memory safety. She's currently experimenting with introducing Rust into some of the systems she works on, and she's considering Rust for a few greenfield projects as well.

For each character, we will write a series of ["status quo" stories][sq] that describe the challenges they face as they try to achieve their goals (and typically fail in dramatic fashion!) **These stories are not fiction.** They are an amalgamation of the real experiences of people using Async Rust, as reported to us by interviews, blog posts, and tweets. To give you the idea, we currently have two examples: one where [Grace has to debug a custom future that she wrote][gsq], and another where [Alan] -- a programmer coming from a GC'd language -- [encounters a stack overflow and has to debug the cause][soflow].

Writing the "status quo" stories helps us to compensate for the [curse of knowledge][cok]: the folks working on Async Rust tend to be experts in Async Rust. We've gotten used to the [workarounds] required to be productive, and we know the little tips and tricks that can get you out of a jam. The stories help us gauge the cumulative impact all the paper cuts can have on someone still learning their way around. This gives us the data we need to prioritize.

### ...and then tells how we will change it

The ultimate goal of the vision doc, of course, is not just to tell us where we are now, but where we are going and how  we will get there. Once we've made good progress on the status quo stories, the next step will be start brainstorming stories about the ["shiny future"][sf].

Shiny future stories talk about what the world of async could look like 2 or 3 years in the future. Typically, they will replay the same scenario as a "status quo" story, but with a happier ending. For example, maybe Grace has access to a debugging tool that is able to diagnose her stuck tasks and tell her what kind of future they are blocked on, so she doesn't have to grep through the logs. Maybe the compiler could warn Alan about a likely stack overflow, or (better yet) we can tweak the design of `select` to avoid the problem in the first place. The idea is to be ambitious and focus first and foremost on the user experience we want to create; we'll figure out the steps along the way (and maybe adjust the goal, if we have to).

### Involving the whole community

The async vision document provides a forum where the Async Rust community can plan a great overall experience for Async Rust users. Async Rust was intentionally designed not to have a "one size fits all" mindset, and we don't want to change that. Our goal is to build a shared vision for the end-to-end experience while retaining the loosely coupled, exploration-oriented ecosystem we have built.

The process we are using to write the vision doc encourages active collaboration and "positive sum" thinking. It starts with a brainstorming period, during which we aim to collect as many "status quo" and "shiny future" stories as we can. This brainstorming period runs for six weeks, until the end of April. For the first two weeks (until 2021-04-02), we are collecting "status quo" stories only. After that, we will accept both "status quo" and "shiny future" stories until the end of the brainstorming period. Finally, to cap off the brainstorming period, we will select winners for [awards] like "Most Humorous Story" or "Must Supportive Contributor". 

Once the brainstorming period is complete, the working group leads will begin work on assembling the various stories and shiny futures into a coherent draft. This draft will be reviewed by the community and the Rust teams and adjusted based on feedback.

### Want to help?

If you'd like to help us to write the vision document, we'd love for you to contribute your experiences and vision! Right now, we are focused on creating status quo stories. We are looking for people to author PRs or to talk about their experiences on issues or elsewhere. If you'd like to get started, check out the [template for status quo stories][template] -- it has all the information you need to open a PR. Alternatively, you can view the [How To Vision][htv] page, which covers the whole vision document process in detail.
