+++
path = "inside-rust/2020/05/26/website-retrospective"
title = "A retrospective on the 2018 rust-lang.org redesign"
authors = ["Nick Cameron"]
aliases = ["inside-rust/2020/05/26/website-retrospective.html"]

[extra]
team = "the core team"
team_url = "https://www.rust-lang.org/governance/teams/core"
+++

We released our second 'edition' of Rust at the end of 2018. Part of that release was a revamp of the [Rust website](https://www.rust-lang.org). That work was completed on time, but there was some controversy when it was released, and the project itself was difficult and draining for those involved. This retrospective is an attempt to record the lessons learned from the project, and to put the project into context for those interested but not directly involved.

This retrospective aims to be [blameless](https://blog.newrelic.com/technology/blameless-retrospectives/) and forward-looking. There is no benefit in re-litigating what happened, and we are interested here in the planning, project management, and community aspects, rather than critiquing the design or implementation.


## Lessons learned

What have we learned for the next time we take on a similar project?


- People first: people are more valuable than schedules; keeping people healthy, happy, and productive is the most important aspect of managing a project.
- Open communication: we should be as open as possible as early as possible with communication about projects, even when the nature of the project means we can't be open with all development.
- Get feedback early and set expectations about the kind of feedback which is useful. We have ongoing problems in the Rust community where (mostly) well-intentioned feedback from the community becomes overwhelming to the point of harassment; we do not have a solution to this.
- Be prepared to manage feedback, in particular by having enough people available to respond.
- Recognize the complexity of projects and ensure appropriate project management.
- Projects should have clear ownership.
- Large projects should not be scheduled to finish at the same time.
- Work iteratively, rather than going for 'big bang' releases.
- Consider ongoing maintenance: how much is there to do? Who will do it? Failing to consider this means there is more pressure on the initial release.

A lot of these points seem obvious with hindsight. However, every decision is a trade-off, and despite best intentions, it is easy to mis-weight factors in these trade-offs. The above are factors that, with hindsight, should have had more weight.

In the next sections, I'll expand on some lessons from the summary and then give some context by describing the project.

### Communication

Projects which have a primary focus on design have different dynamics to most other software projects. For example, there is the risk of 'design by committee'. When trying to do open development, this risk is magnified since the 'committee' is effectively the entire world. However, in retrospect we overshot and were not open enough with the website project.

We could have better communicated the motivation and constraints of the project. By the time of the beta release, the community did not share the project team's conceptualization of the website's requirements. In the future, we might create a pre-RFC to discuss and communicate requirements without starting design work. Once a high-level design is made, it should be actively evangelized to the community.

As well as asking for feedback (see below), we should communicate project progress and share opportunities for contribution. When looking back at a repository on GitHub, it is not obvious how much iteration has taken place, or what issues have been discussed. But, if the repository is followed from the start, these things are clear.

In general, communication should be a conversation. Unfortunately, due to other ongoing projects, we did not have enough people with enough time to have that conversation. We think an important lesson here is not to schedule large projects concurrently.

### Project management

We underestimated the scale of the project, both in terms of the work to be done and the number of people who would need to be coordinated. As a result, several good people were burnt-out by the project. Errors in software estimation are common; we should have reacted by putting people first - no project is worth losing good people for. One reason that did not happen is that nobody felt empowered to step back and re-evaluate the project. In general, ownership of the project was unclear and this led to poor leadership. Furthermore, the ownership which did exist was not communicated well to the wider community.

The project as a whole highlighted not just our relative inexperience (with this kind of project), but also our process debt. We had not (and to a great extent still have not) created processes and structures to support projects and people when things start to go wrong. This lets small issues snowball into large ones. For the website project this was compounded by not having enough people involved - they became over-worked and stressed, and that meant they did not have the bandwidth to implement good project management, even when we knew the right thing to do.

### Feedback

As mentioned earlier, we think that community feedback would have been easier to manage if it had been collected during the whole project, rather than being compressed into the final two weeks. Beyond that, we needed much better staffing for the feedback period. Handling feedback was an extremely stressful and difficult experience. In the future, we should ensure there are more people and that we structure feedback as much as possible to ensure that it is useful rather than overwhelming.

A minority of the community went beyond what was acceptable as feedback. Coupled with the 'pile on' effect of discussions on the internet, this became harassment of the website developers. This is unacceptable behavior, and we expect better from the community. Some of the effect was unintentional, and this is a problem that affects controversial RFC discussions too. It is not clear how to solve this, but is something we should investigate.


## Context

The website revamp was part of the 2018 edition. The edition was an awesome achievement, but an incredible amount of work. The new website was planned from near the beginning of the year, in the early stages of edition planning. We considered it important for the new website to be ready in time for the edition for maximum impact and because the previous website was unfit for our goals (more below). Because of the timing, there were fewer people available to work on the website than might have been the case, there was less time and energy for leadership and oversight, and there were many competing projects for those involved.

The previous website had been incrementally added to, but there had never been major work on either content or design (other than the initial release). Essentially, the website rewrite was a completely new project in a domain where we had no organizational experience (there were individuals who had experience in web development, but there had not been opportunity for that experience to become institutional knowledge).

The initial website was well-suited to its purpose and audience: presenting a small research project to interested hackers. However, as the project and website have grown, the website became less and less appropriate.

There was consensus among the core team that the old website needed replacing. Although many in the community have fond memories of it (after all, it was most people's very first contact with Rust), there were several ways in which the old website was objectively inadequate: it was difficult to find information, much of the content was out of date, pages were crowded and poorly organized, it was hard to update and to localize (which resulted in missing and out of date information), and it was missing many parts of the community and ecosystem (e.g., any mention of using Rust in embedded systems).

Design-wise, the previous website was simple and tidy, but it had problems - it was hard to emphasize text, there was little contrast between sections (making it hard to read), and it lacked the vibrancy of Rust's ecosystem and community. It was designed for the audience which built it, and our ambitions for Rust, and the audience for the website, had since grown larger.

One of the goals of the 2018 edition was to appeal to a wider audience. The website was a key tool for achieving that goal. However, it was clear the design and most of the content needed a complete overhaul.

This sounded like a relatively standard website project to produce a relatively small website. However, in retrospect, the constraints were difficult - there is a lot of information that needed to be made accessible, without making the website overwhelming; we needed to serve newcomers with different backgrounds, as well as existing Rust users looking to find information; the previous 'small' website had grown large, and there was a lot of content to update or replace.

Work was slow to start and progress was slow, in part due to staffing issues. Content was sought from the teams in mid-2018. We vastly underestimated the complexity of producing and collecting content. Content was slow to produce and slow to review; there were many unrecognized dependencies. We needed lots of iteration. Essentially, the website became a project with 50-ish people, but was managed as if it were a project with one or two people. We were building a website before most content was ready, which is a well-known web development anti-pattern.

Despite this, and largely due to heroic efforts, the website was finished on time. All planned content was present and polished. We had a striking and vibrant new design, and an implementation that made the website much easier to keep up to date and to translate. Essential information was easy to find, and the website was accessible to a wider audience, in particular developers who knew nothing about Rust, engineering management, and a wider section of potential contributors.

Unfortunately, it was only just in time. As well as meaning that the last phase of work was stressful and rushed, it meant we didn't have as much time as we should have had for testing and feedback: only two weeks to gather and address feedback on the beta release. Because of this and an earlier lack of communication, there was a flood of commentary, some of which was vocally negative and some which was trolling or harassment. The team did not have the resources or time to respond well.

Of course, being a software project, there were some bugs (most of which were quickly resolved), and some missing features (notably, localization, which made the website a worse experience for many visitors who did not speak English natively).

Post-release, content and design was polished, bugs were addressed, and we attempted to create a team to maintain the website. Unfortunately, some of the poor behavior from the community continued. Several people involved with the edition and specifically the website were left burnt out and left Rust or cut back work significantly.


## Conclusion

In summary, we regard the website as a successful (but imperfect) product, but delivered in a sub-optimal manner. A lot of the things that went wrong were fairly common project management issues. We believe the highest-level lesson to take away is that the Rust organization should improve its project and product management. (To be clear, we think this is an organizational issue, not a comment on any individuals' skills in the domain). Our usual development style is iterative and incremental; when working on larger, less incremental projects, we need to put in more resources, management, and coordination to ensure success. The project was under-staffed and, beyond the obvious problems, that meant that even when we knew the right thing to do, we did not have the people, time, or energy to do it.

Finally, thank you to everyone who built the website and who helped with this retrospective.
