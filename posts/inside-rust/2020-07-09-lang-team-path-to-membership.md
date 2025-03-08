+++
layout = "post"
title = "Lang team design meeting: path to membership"
author = "Niko Matsakis"
description = "Lang team design meeting: path to membership"
team = "the lang team <https://www.rust-lang.org/governance/teams/lang>"
+++

This week the [lang team] design meeting was on the topic of the "path to
membership". This blog post gives a brief summary; you can also read
the [minutes] or view the [recording].

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-07-08-lang-team-path-to-membership.md
[lang team]: https://www.rust-lang.org/governance/teams/lang
[recording]: https://youtu.be/SeH-hZgDG1Y

The premise of the meeting was that the lang team has never had a
particularly clear *path to membership* -- i.e., it's been hard to say
exactly what are the kinds of steps that one should be taking if you
would like to become a member of the lang team. However, with the
shift to [major change proposals] and in particular [project groups],
we're starting to see what such a path looks like. 

[major change proposals]: https://github.com/rust-lang/rfcs/pull/2936/
[project groups]: https://github.com/rust-lang/rfcs/pull/2856

### Expectations for a team member

As part of our discussion, we came up with a relatively complete list of
what we see as the "expectations for a lang-team member". To be clear,
this is the full set of possible expectations: many members only have the
time to do some subset of these things at any given time.

- Lead project groups, where appropriate
- Liaison for projects, where appropriate
- Participate in triage meetings
- Participate in design meetings
- Respond to rfcbot fcp requests in a timely fashion
- Participate constructively in, and help facilitate, RFC discussion, issues, PRs, and other GitHub-based discussions
    - Provide important technical points
    - Help to drive discussions towards common understanding
    - Understanding and documenting the positions and points being raised
- Monitor and respond to communication in Zulip

### A sketch for a path to membership

The core idea for a path to membership is that we want some set of
steps that let us see people doing the things and demonstrating the
qualities we expect from lang-team members, so that we can tell how it
is working (and so that people can experience what it's like).

This suggests that a "path to membership" might look something like this:

* Lead or be heavily involved in one or more project groups
* Serve as a liaison for one or more project groups
* Participate in meetings, where possible
* Participate in discussions and in particular help to create summaries or otherwise resolve technical disputes in a productive way

We realize that we can identify people who are doing some of those
things already and approach to see if they are interested in lang-team
membership.  If so, we can look for opportunities to complete some of
the other bullets.

## A bit of background: project groups

We've not been blogging a lot about the idea of project groups and the
like so let me give just a bit of background. In short, the idea is
that we are trying to "intercept" the RFC process earlier by
introducing a "pre-step" called a Major Change Proposal
(MCP). (Terminology still subject to change as we experiment here.)

The idea is that if you have an idea you'd like to pursue, you can
file an MCP issue and describe the high-level details. If the idea
catches the eye of somebody within the team, we will create a
**project group** to pursue the idea, with that member serving as the
**lang team liaison** and you (or others) serving as the **group
lead**.

A **project group** doesn't have to be a huge group of people. It
might even just be one or two people and a dedicated Zulip stream.
The idea is that the group will work out the design space and author
RFCs; once the RFCs are accepted, the group can also pursue the
implementation (though the set of people involved may shift at that
point), and hopefully see the idea all the way through to
stabilization.

### Growing the set of folks who can serve as liaison

One of the things we talked about was the proper role for a project
group liaison. As described in the previous paragraph, a liaison was
basically a member of the team who would follow along with the design
and help to keep the rest of the team up to date. 

But we realized that if we limit liaisons to team members, then this
is incompatible with this idea of a "path to membership" where people
can "trial run" lang-team activities.  It's also somewhat incompatible
with a core goal, which is that the experience of someone who is *not*
on a team and someone who *is* on a team ought to be awfully close,
and that we should be careful when creating distinctions.

Therefore, we discussed the idea of saying that liaisons don't have to
be team members, they just have to be people who are heavily involved
in the project and who can be trusted to create regular updates for
the lang-team and keep the rest of the team in the loop. 

In particular, this can also be a useful stepping stone towards full
lang-team membership -- although it doesn't have to be. It's useful to
have people serve as liaisons even if they don't really have time or
interest in being on the lang team.

### Conclusion

We concluded that we'll start experimenting with "non-team-member
liaisons", and that people who are maybe interested in that role can
reach out privately to Josh Triplett or myself
(nikomatsakis). Further, we'll work to write up the "path to
membership" as well as the expectations for team membership.
