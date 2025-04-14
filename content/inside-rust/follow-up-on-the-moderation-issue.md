+++
path = "inside-rust/2021/12/17/follow-up-on-the-moderation-issue"
title = "Follow-up on the moderation issue"
authors = ["Ryan Levick and Mara Bos"]
aliases = ["inside-rust/2021/12/17/follow-up-on-the-moderation-issue.html"]

[extra]
team = "the Rust Project"
team_url = "https://www.rust-lang.org/"
+++

Last week, the following e-mail was sent to all members of the Rust project
(including all working groups) to follow up on the moderation issue.
The footnotes have been added to provide additional context to the wider Rust community,
and were not part of the original e-mail.

---

**From:** Mara Bos \
**To:** All members of the Rust project \
**Date:** Fri, 10 Dec 2021 18:12:06 UTC \
**Subject:** Follow-up on the moderation issue

Hey all,

On behalf of the top-level team leads, the new mods, the project directors to
the Foundation[^project-directors], and the core team[^core], I'd like to
apologize for the delay in getting back to all of you after the resignation of
the moderation team[^resignation].

Over the past few weeks, it has been nearly the full time job of many involved
to collect all the context, understand the perspectives of those involved, find
common ground, and rebuild understanding and trust. This work is subtle, highly
context dependent, and sometimes extremely personal. Still, we owe it to all
project members to be as transparent as we can be. In this email, I'll attempt
to update you on the shape of the issues and restate our collective commitment
to solving them together with the entire Rust project and eventually the wider
Rust community.

**What's going on?**

The most immediate cause of the current issue was a disagreement between the
former members of the moderation team and the core team about how to handle a
moderation issue in which the core team itself were interested parties.
I cannot share more context on that issue without violating the privacy of
those involved, including of those who reported the issue.
However, as frustrating as it might be for those without any context,
I am convinced it's not in any way necessary to get more people involved in
that specific moderation issue itself.

Historically, moderation actions involving Rust team members have been reviewed
in collaboration between the moderation team and the core team.
However, due to the involvement of the core team, there was no clear process to follow.
Both teams put in substantial effort to try to deal with this lack of process,
but over an eight month period involving miscommunication and disagreements,
this escalated into a trust issue between the moderation team and the core team.
Both the moderators and the core team ended up in an unworkable situation
where no one could have full context, making a path forward impossible.

**Why are certain groups or people directly involved in solving this and others not?**

Due to the resignation being unexpected and the complexity of the situation being high,
we both needed to act quickly and involve the most obvious stakeholders.
Since the concept of leadership of the Rust project is fluid and not well defined,
it is very hard to pick the right set of people to involve,
while making sure different perspectives are represented and heard.
Any set would likely leave out some person or group who needed to be involved,
but to get this unblocked, we started with all top-level team leads, project directors
to the Foundation, all core team members, and the new moderation team members,
to discuss next steps. This was chosen since it was easy to determine
objectively who fit this description and who did not.

Over time, we will expand this group to include others in the project.
However, due to the highly context-dependent and sensitive nature of what's
being discussed, opening all discussion to everyone in the project (which is
effectively making it open to the public) would be counterproductive and make
fact finding and context building impossible.

Ryan Levick and I have stepped forward to coordinate the work here.
If you would like to be involved or provide input,
please contact either of us by e-mail or on Zulip.

**What are we going to do?**

With all this in mind, we are committed to the following high level goals:

1\. The recent events are one of several indicators that the Rust project has
underspecified policies for handling complex moderation issues. This must be
fixed with publicly documented procedures around moderation that ensure
privacy, fairness, accountability, and justice to all parties involved. We are
gathering input and experiences to try to find an outcome that will satisfy
everyone's needs and takes into account past experiences. Decisions will not be
made without wider community involvement.

2\. More generally, the issue was another indicator to a fact that was already
clear to all involved in Rust project leadership, including all members of the
core team: the Rust project needs to adapt its structures for how it does
governance. What the future of Rust governance should look like is a big open
question, but from the input we've collected so far, there does seem to be
enough common ground to build on. We want to solve this problem with feedback
from all Rust project members.

3\. Most specifically, we need to resolve the specific moderation issue that
was at the center of the disagreement between core and the former moderation
team. This resolution needs to respect all the values listed in point 1:
privacy, fairness, accountability, and justice to all parties involved. This
will be handled with input from all involved parties.

These issues are highly complex, require a large amount of context to
understand, and involve private and personal information which should not be
discussed publicly. However, at the center of the Rust project is a belief in
transparency and openness. We are committed to solving these issues with all
members of the Rust project and the wider Rust community, but we ask for your
patience while thoughts are organized and drafts are constructed. Details and
plans will be discussed publicly when we can ensure that doing so will not
cause more confusion.

Needless to say, it is difficult to govern an open source project which has
reached a size larger than most companies[^project-size] and yet is composed of
volunteers. We have a lot of work ahead of us, but we are confident that the
Rust project will come out stronger from this. While these issues are serious
and require care to reach positive conclusions, we are confident that this will
not negatively impact our ability to continue to ship improvements to the Rust
language and its core tooling, documentation, and support.[^continue]

If you have any concerns or thoughts, please reach out to me or Ryan with
questions, concerns, or comments.

Thanks, \
Mara Bos (Library team lead), \
on behalf of top-level team leads, project directors to the Foundation, core
team members, and the new moderators.

---

[^project-directors]: These are the members of the Rust project who represent the project on the board of the Rust Foundation.

[^core]: Unlike in many other projects, the Rust project is made up of many teams. The core team is one of them, and does not make decisions that fall under the scope of one of the other teams.

[^resignation]: <https://github.com/rust-lang/team/pull/671>

[^project-size]: For reference, the original email went out to approximately 300 people.

[^continue]: Rust 1.57 was released two weeks ago, and we will continue to ship new releases of Rust on schedule.
