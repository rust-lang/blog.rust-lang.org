+++
layout = "post"
title = "Call for proposals: Rust 2025h1 project goals"
author = "Niko Matsakis"
team = "Leadership Council <https://www.rust-lang.org/governance/teams/leadership-council>"
+++

**As of today, we are officially accepting proposals for Rust Project Goals targeting 2025H1 (the first half of 2025).** If you'd like to participate in the process, or just to follow along, please check out the [2025h1 goal page][2025h1]. It includes listings of the goals currently under consideration, more details about the goals program, and instructions for how to submit a goal.

[2025h1]: https://rust-lang.github.io/rust-project-goals/2025h1/index.html

## What is the project goals program and how does it work?

Every six months, the Rust project commits to a set of goals for the upcoming half-year. The process involves:

* the owner of the goal program (currently me) posts a call for proposals (this post);
* would-be goal owners [open PRs][] against the [rust-project-goals] repository;
* the goal-program owner gathers feedback on these goals and chooses some of them to be included in the RFC proposing the final slate of goals.

To get an idea what the final slate of goals looks like, check out [RFC 3672][]. The RFC describes a set of goals, designates a few of them as flagship goals, and summarizes the work expected from each team. The RFC is approved by (at least) the leads of each team, effectively committing their team to prove the support that is described.

[rust-project-goals]: https://rust-lang.github.io/rust-project-goals/
[open PRs]: https://rust-lang.github.io/rust-project-goals/how_to/propose_a_goal.html
[May of 2024]: https://blog.rust-lang.org/inside-rust/2024/05/07/announcing-project-goals.html
[RFC 3614]: https://github.com/rust-lang/rfcs/pull/3614
[RFC 3672]: https://github.com/rust-lang/rfcs/pull/3672#issuecomment-2254599176
[PAIA]: https://blog.rust-lang.org/2024/08/12/Project-goals.html

## Should I submit a goal?

Opening a goal is an indication that you (or your company, etc) is willing to put up the resources needed to make it happen, at least if you get the indicated support from the teams. These resources are typically development time and effort, but they could be funding (in that case, we'd want to identify someone to take up the goal). If you pass that bar, then by all means, yes, open a goal. 

Note though that controversial goals are likely to not be accepted. If you have an idea that you think people won't like, then you should find ways to lower the ask of the teams. For example, maybe the goal should be to perform experiments to help make the case for the idea, rather than jumping straight to implementation.

## Can I still do X, even if I don't submit a goal for it?

Yes. Goals are not mandatory for work to proceed. They are a tracking mechanism to help stay on course.

## What have the experiences with the goals program so far?

The current round has shown positive results so far. Of the 26 total goals, the vast majority have made progress, with only 1 (an orphaned goal) receiving no updates at all. Most goals are still ongoing with 1 completed and 1 not yet complete.

Here are some quotes from goal owners and Rust maintainers talking about the program:

* "Thank you for leading this goals initiative, I feel like this is blowing a bit of life back into rust as a project."
* "The project goals provided two benefits for me. First, I could show people not involved in Rust that my work had some support from the Rust Project (e.g., helpful for applications/grants). Second, I got some new contributors from people following my updates and the requests for help I added there."

Many in the Rust community have commented that they appreciate the visibility into what the project is doing. Some quotes extracted from Reddit threads:

* "I really appreciate these kinds of little progress updates on the big changes coming to Rust. I personally don't like Zulip and can find the GitHub hard to wade through (so much good work being done!)."
* "\[Referring to cargo script\] I have vague awareness of having seen the above, but it's not until just now (an confluence of having just seen it again and then wrestling with convenience command scripting) that I'm realizing **how amazing this is**! "
* "Lovely to see this in place, and I'm really thrilled to see that all of the teams have their own smaller goals."

Areas for improvement include:

* We have not been effective at finding contributors for the orphaned goal.
* In some cases established owners have not really contributed to the goal or the work to drive the goal and post updates has been done by others. We don't have a clear mechanism for how to address this.
* Goals with multiple owners feels like an anti-pattern; it makes it hard to know who to contact.
* The lang team found that several of the goals to which we promised design meetings have not wound up using them or making much progress. This is not necessarily bad but may also indicate the team overestimated their capacity or that more focus would be useful.

## Is the goals program permanent?

Not yet. It's still considered experimental. Presuming that the process we derived the first time around continues to work well, I plan to propose an RFC documenting it and making it a permanent fixture.

## Conclusion

The Rust Project Goals program is driving progress, increasing transparency, and energizing the community. As we enter the second round, we invite you to contribute your ideas and help shape Rust's future. Whether you're proposing a goal or following along, your engagement is vital to Rust's continued growth and success. Join us in making Rust even better in 2025!
