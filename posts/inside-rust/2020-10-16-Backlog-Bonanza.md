+++
layout = "post"
date = 2020-10-16
title = "Lang team Backlog Bonanza and Project Proposals"
author = "Nicholas Matsakis"
team = "the lang team <https://www.rust-lang.org/governance/teams/lang>"
+++

A month or two back, the lang team embarked on a new initiative that
we call the "Backlog Bonanza". The idea is simple: we are holding a
series of meetings in which we go through every pending RFC, one by
one, and try to reach some sort of determination about what to do with
it.  Once we've finished that, we can start in on categorizing other
forms of backlog, such as tracking issues.

### Possible outcomes for each RFC

When we look at an RFC, we're typically deciding between one of the following outcomes:

* **Close** the RFC, if the problem doesn't seem like high priority at the moment, or the solution seems quite far from what we would want.
* Close, but **suggest a [project proposal]**, if we think that the we might like to see the problem solved, but we aren't sure if the RFC has the design right, or we're not sure who would be a good liaison.
* **Merge** the RFC, if we think the RFC basically nailed it and we have a lang team liaison in mind.

[project proposal]: https://lang-team.rust-lang.org/proposing_a_project.html

### Wait, what is a project proposal?

I'm so glad you asked! The lang team is experimenting with a new
process for extending the language. Instead of starting out by writing
an RFC, the idea is to start with a **[project proposal]**. This is a
lightweight description that you do by [opening an issue] on the
[lang-team repository] using the "Project proposal" template. That
will create an issue and a corresponding stream on Zulip.

[lang-team repository]: https://github.com/rust-lang/lang-team/
[opening an issue]: https://github.com/rust-lang/lang-team/issues/new/choose

In our [weekly triage meetings], we go over each new project proposal
and try to provide feedback. Project proposals generally result in one
of a few possible outcomes:

* **Close**, if we feel like the idea isn't a good fit right now.
* **Suggest implementing**, if we feel like the idea is simple or obvious enough that an RFC isn't really needed. In that case, folks can just write a PR and we can use the fcp process to approve the PR.
* **Charter a project group**, if we feel like the idea is good, but we'd like to see the design spelled out. To do this, there has to be some lang-team liaison who wants to help see it through (though that liaison doesn't have to be a member of the team; [serving as a liaison is a good way to get more involved in the lang team][path]).

[weekly triage meetings]: https://lang-team.rust-lang.org/meetings.html
[path]: https://blog.rust-lang.org/inside-rust/2020/07/09/lang-team-path-to-membership.html

### Chartering a project group

A "project group" is basically just a group of people working together
completing some idea. Project groups are often pretty small, just 1 or 2
people, though they can get significantly larger.

Creating a smaller project group is meant to be lightweight. We
basically convert the project proposal into a charter that states the
general goals and create an associated zulip stream where folks can
chat. Each project also gets a tracking issue that shows up on our
[lang team project board]. (For larger project groups, we can make a
dedicated repo and an entry in the [Rust team repo].)

[recent example]: https://github.com/rust-lang/lang-team/tree/master/projects/declarative-macro-repetition-counts
[Rust team repo]: https://github.com/rust-lang/team
[lang team project board]: https://github.com/rust-lang/lang-team/projects/2

In the early stages of an idea, project groups work to draft an
RFC. This RFC is then taken to the lang-team for feedback. Once the
lang-team is basically happy on the direction things are going, we'll
encourage the group to open the RFC on the main RFC repository, where
it'll get feedback from a broader audience.

Once the RFC is **accepted**, the hope is that project groups stick
around. If desired, the same folks can try to implement the feature
(in collaboration with the compiler team) or we can find new people.
But this way, as those people try to implement, they'll become a part
of the same group that was designing the feature so that we can
iterate more readily. The same logic applies to the other aspects of
shipping a feature, most notably writing documentation.

### Tracking projects: the lang team project board

I mentioned the [lang team project board] off-hand in the previous
paragraph. This is our attempt to track the ongoing efforts. It breaks
down the various projects into stages, with the things that are closest
to shipping coming first:

* **Stabilization** -- projects that we are ready to stabilize, or in
  the process of stabilizing.
* **Evaluation** -- projects that are fully implemented but where we are
  seeking feedback on how well the design works.
* **Implementation** -- projects that are currently working on implementation
  (and sometimes concurrent design iteration).
* **Pending RFC** -- projects with an RFC that is pending public comment
* **Design** -- projects actively iterating towards an RFC
* **Shortlisted** -- project ideas that we might want to take up once we
  find a suitable liaison or people have enough bandwidth
  
### Ways to get involved

If you like, you are welcome to attend backlog bonanza meetings. They
are open for anyone and take place during our [design meeting]
most weeks. We haven't setup a very good process for announcing our
design meeting schedule, though, that's something that we need to get
better at.

[design meeting]: https://lang-team.rust-lang.org/meetings.html

Alternatively, if you have ideas you'd like to float, please feel free
to open a [project proposal].
