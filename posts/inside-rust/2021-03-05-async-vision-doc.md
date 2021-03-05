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
[r]: https://rust-lang.github.io/wg-async-foundations/vision/roadmap.html
[dt]: https://rust-lang.github.io/wg-async-foundations/vision/tenets.html
[cc]: https://rust-lang.github.io/wg-async-foundations/vision/characters.html
[dd]: https://rust-lang.github.io/wg-async-foundations/design_docs.html
[gf]: https://forms.gle/YKNniGhaNXBhmjXNA
[gr]: https://is.gd/T6TadC
[z]: https://rust-lang.zulipchat.com/
[wgz]: https://rust-lang.zulipchat.com/#streams/187312/wg-async-foundations
[amb]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future.html#think-big----too-big-if-you-have-to
[cok]: https://en.wikipedia.org/wiki/Curse_of_knowledge
[workarounds]: https://rust-lang.github.io/async-book/07_workarounds/01_chapter.html

The [Async Foundations Working Group][wg] believes Rust can become one of the most popular choices for building network applications and services. People are already building all kinds of networked applications using Async Rust, ranging from embedded devices to clients and servers to foundational cloud services. We want those people to love using Async Rust. For that to happen, we need to move Async Rust beyond the "MVP" state it's in today and make it accessible to everyone.

We are launching a collaborative effort to build a shared [vision document][vd] for Async Rust. **Our goal is to engage the entire community in a collective act of the imagination: how can we make the end-to-end experience of using Async I/O not only a pragmatic choice, but a joyful one?**

If you'd like to be a part of that, good news: we need your help! As one of the first steps, we want to encourage people to write about their experiences using Async Rust. Read on for more details!

### Involving the whole community

Building a truly great experience for Async Rust takes more than just `async fn`. We need to think about the end-to-end experience -- everything from what happens when people first learn Async Rust, to what happens as they are building their application, to what happens after deployment.

The challenge for us is to drive that holistic product experience while retaining the loosely coupled, exploration-oriented ecosystem we have built. Async Rust was intentionally designed not to have a "one size fits all" mindset, and we don't want to change that. But we need a forum where we can coordinate on the overall experience we are all shooting for. This is where the [vision doc][vd] comes in.

### To know where you're going, you have to know where you are

Our first goal for the [vision document][vd] is to describe what it's like to use Async Rust today. We've created a [cast of characters][cc] that represent the different backgrounds and experiences that people bring to bear when learning and using Async Rust:

* **Alan**, the full-stack developer working across many companies;
* **Grace**, the principal engineer hacking on a data storage service;
* **Niklaus**, the developer building generic Rust libraries and frameworks;
* **Barbara**, the embedded developer doing networking.

Each character has a biography with more details. For example, let me tell you a bit more about Grace:

> Grace is a principal engineer who has been building high-performance networking systems in Java and C++ for a number of years. She currently works on a distributed data storage service that is used in a lot of the world's largest web properties. This service is implemented in Java, with certain key components written in C++. Grace is currently working on introducing Rust into the system.

For each character, we are writing a series of ["status quo" stories][sq] that describe the challenges they face as they try to achieve their goals (and typically fail in dramatic fashion). These stories are not fiction. They are an amalgamation of the real experiences of people using Async Rust, as reported to us by interviews, blog posts, and tweets.

For example, here is what happens to Grace as she is porting part of her service into Rust (still a rough draft, mind):

> When examining metrics for her service in the past, Grace has noticed tail latencies in the P99 that exceed their SLA, and identified GC in the routing layer as the problem. She decides to investigate rewriting the routing service in Rust. A few months of frantic hacking follow:
> 
> <img src="https://media.giphy.com/media/ule4vhcY1xEKQ/giphy.gif" alt="montage of cats typing" width=200></img>
> 
> Soon enough, she and her team have a proof of concept working. They run some local stress tests and notice that 5% of requests hang and fail to respond and the client eventually times out. She is unable to reproduce this problem when running one-off requests locally, it only shows up when sending above 200 requests-per-second. 
>
> She realizes that she doesn't have any tooling that can give her insight into what's going on. She starts to add lots of logging, attempting to tie log entries to specific connections. Using an operating system tool, she is able to identify the socket addresses for the hung connections, so she also includes the socket addresses in each log message. She can then grep the logs to find those that are associated with the hung connections. Of course, the logs only tell her what the connection managed to do successfully, they don't tell her why it stopped -- so she has to keep going back and adding more logs until she can narrow down the exact call that hangs.
>
> Eventually, she identifies that the last log message is right before calling the HMAC authentication library that is written C, and is synchronous, but only when refreshing the private authentication token from the source of truth. 

As a spoiler, Grace eventually figures out that the `Waker` for her hand-written `Future` is never being invoked (and yes, this is a true story), and so she is ultimately able to the fix the bug, but it took her several frustrating days to track it down. It sure would be nice if there were a better way!

### Why we write the "status quo" stories

The "status quo" stories help us compensate for the [curse of knowledge][cok] -- in particular, the folks working on Async Rust tend to be experts in Async Rust. We've gotten used to the [workarounds] required to be productive, and we know the little tips and tricks that can get you out of a jam. 

Writing the "status quo" stories gives us a concrete way to put ourselves in the shoes of an average Async Rust user. They help us to estimate the cumulative impact all the paper cuts can have on someone still learning their way around. This gives us the data we need to prioritize.

### Next stop, the future!

The ultimate goal of the vision doc, of course, is not just to tell us where we are now, but where we are going. Once we're making good progress on the status quo user stories, we're also going to start talking about the "shiny future" -- that is, what will these stories read like 2-3 years in the future? Maybe Grace has access to a debugging tool, for example, that is able to diagnose her stuck futures and tell her what kind of future they are blocked on, so she doesn't have to grep through the logs. Probably she has better ways to do her logging, too. [Let's be ambitious!][amb]

<a name="help"></a>

### Want to help? Tell us your story!

We want to make sure all Async Rust users and their experiences are reflected in the async vision doc, so please tell us your story! Share it in any form you want: blog posts, tweets, letters to the editor, whatever, as long as it has a URL. Be sure to include two things:

* Which of the [cast of characters][cc] fits you best? If none of them seem to fit you, then tell us what makes you different from them.
* What are some of the challenges that you've encountered? Bonus points if you write in narrative style.

After you write your post, let us know about it by submitting the URL using this [google form][gf]. Also, if you tweet about it, use the hashtag `#asyncvisiondoc`. We'll publish the links that people submit on the [Async Foundations working group page][wg].

**We are operating on a quick timeline.** If you want to be sure we are able to read and think about your story, please submit it by **Friday, March 12th**. If you can't make that deadline, submit your story anyway. We plan to revisit the vision doc regularly and all stories are useful.

### That's not enough! I want to do more!

If you'd like to do more, that's great! Pop onto the [rust-lang zulip][z], join [`#wg-async-foundations`][wgz], and say hi. We're always looking for help.
