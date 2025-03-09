+++
layout = "post"
date = 2023-05-01
title = "Postmortem Analysis in Cargo"
author = "Jon Gjengset and Weihang Lo"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

At 01:52 UTC, 2022-10-28, [rust-lang/cargo#11183] was merged into the Cargo master branch. It introduced a bug that caused Cargo to fail to build packages that use a particular, but very common, dependency setup. The change nearly made its way into the next nightly release. If it had, it would have rendered any of the 30k crates with `serde_derive` as a dependency (one of the most popular crate on crates.io) unbuildable for anyone using the resulting nightly release.

Following this incident, the Cargo team went through a postmortem analysis, as is appropriate for incidents with (a potential for) broad blast-radius or otherwise significant impact. This time around, we followed a particular structured postmortem template in the hopes that it might make the resulting write-up more thorough, insightful, and actionable, and we ultimately found that it gave us a better understanding of the underlying root causes and failing/missing safe-guards. So, we wanted to share our experience with other Rust teams in case they may find it similarly useful, either in part or as a whole. 

The postmortem template consists of four parts:

* **What happened:** a summary that provides context for the event, including metrics or graphs that illustrate the impact of the event if available. This should include a summary of any user-facing impacts or experience during the event.
* **How we responded:** a timeline that describes all the events that occurred during the incident including specific date/time to the extent that they are known, as well as answers to the following four questions:
    * How was the event detected?
    * How could time to detection be improved?
    * How did you reach the point where you knew how to mitigate the impact?
    * How could time to mitigation be improved?
* **Why the event happened:** this is the juicy part. Here, we use the [Five Whys] approach to dig deeply down until the incident’s root causes are identified. Each answer is meant to spawn one or more why questions, until you’re confident that the left answers are fundamentally root causes. It’s also worth pointing out explicitly that “operator error” is **never** a root cause, and that this is not a process for assigning blame. Instead, any operator error is a symptom of a missing or broken mechanism, and the answers should focus on identifying those inadequate mechanisms.
* **How to fix it:** The outcome of the Five Whys exercise is a list of root causes that should be addressed to reduce the risk of a similar incident in the future. From these root causes, we produce short- and medium-term “action items” along with specific owners wherever possible. Long-term solutions can be discussed too, although the focus of action items should be on more immediate mitigation steps that will be taken relatively soon. Each action item is assigned a priority, and is then generally turned into a GitHub issue where applicable. Any items identified as urgent we start working on immediately, while other action items usually fall into the categories “soon” or “once feasible”.

*Note: to ensure the focus stays on mechanisms and processes, not individuals, individuals should not be named unless absolutely necessary. Use terms like "a contributor", "the maintainer", "a libs team member", etc.*

So, without further ado, here is [the postmortem for the aforementioned Cargo incident][postmortem].

[rust-lang/cargo#11183]: https://github.com/rust-lang/cargo/pull/11183
[Five Whys]: https://en.wikipedia.org/wiki/Five_whys
[postmortem]: https://github.com/rust-lang/cargo/issues/12064
