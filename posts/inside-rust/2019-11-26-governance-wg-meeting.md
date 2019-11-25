---
layout: post
title: "2019-11-19 Governance Working Group Meeting"
author: Nell Shamrell-Harrington
team: The Governance WG <https://github.com/rust-lang/wg-governance>
---

Hello everyone! Last week the governance working group met. Here are the large issues we discussed, information on our next meeting, followed by minutes from the last meeting.

## Large Issues Discussed

 We reviewed the [current governance RFC](https://rust-lang.github.io/rfcs/1068-rust-governance.html) and noted governance items that have been added since the RFC was written. We also noted things that have changed or have just not happened, as well as things that could be improved. Please see the full notes below for details.

## Next Meeting

Next meeting will be at **22:00 UTC on Tuesday, December 3** and will be focused on the need for a [GitHub Access Policy](https://github.com/rust-lang/wg-governance/issues/4).

We'd like to encourage anyone who's interested, regardless of their
previous experience to come to the `#wg-governance`
channel on Discord to attend the meeting. (Our meetings are done over a video
call with Zoom, but we use the Discord channel to organise ourselves).

If there are other issues you would like to see us discuss or discuss with us, please mention them in a comment on [this GitHub issue](https://github.com/rust-lang/wg-governance/issues/29).

Thank you and happy Thanksgiving!

## Full Meeting Notes

Here are the full notes for our meeting on November 19, 2019.

### Attending

nikomatsakis, nellshamrell, xampprocky, batmanaod

### Goal

https://rust-lang.github.io/rfcs/1068-rust-governance.html)
* Decide topic for next meeting

### Notes

* Draft terminology RFC https://hackmd.io/s/rJn0cDFsB

### Xampp Rocky

Things that don't seem to be happening:

* Scalability: talking about the RFC process, this mentions RFCs being closed or assigned a shepherd, but this is not accurate
* Subteams publishing status of RFCs regularly
    * In practice, many of them can stall in a variety of phases
    * Sometimes in "almost FCP"
    * Not a clear distinction of which RFCs are being worked on and which are not, or where the focus is
    * We haven't talked so much about how to surface this information
    * Ideally I think github PRs would be trimmed down
        * right now, people open up RFCs for anything they want to have added, some of them get outdated
* In some cases, the team policies have changed
    * e.g. libs team prefers to have direct PRs for smaller additions
* Initial list of teams is out of date
    * Doesn't include release team and some other newer teams
    * teams are supposed to have an RFC policy but that is not up to date
* Feature gating
    * core team is listed as deciding to ungate but in practice this is not true
* Core team
    * not mentioned how the core team is formed apart from the requirement that leads of teams are part of core team
    * "observer role" has not been formalized, is there a path to membership?
        * generally true for other teams as well

### Niko

* Consensus
    * Subteam leaders:
        * Making final decisions in cases of contentious RFCs that are unable to reach consensus otherwise (should be rare).
            * this isn't what we've done in practice, see below

### Kyle

* Question that has arisen:
    * "recourse" if core team gets out of sync?
        * based on commentary

* Role of the core team
    * In practice, the core team hasn't really gotten involved in technical decisions
        * it's never happened that the core team tries to overrule team decisions, or even been close to happening
        * core team members sometimes get involved in discussions, and are treated like any other respected member of the community, but don't generally overrule (e.g.) on the naming of a function or something like that
    * Core team focused on governance itself, functioning of the project, mediation between people
        * edition mechanism was a core team decision
        * has technical aspects but it is ultimately a project policy decision
    * Interesting examples that are "almost core"
        * future compatibility warning policy
            * is it core? feels a bit smaller
        * target tier policy
    * Tagging of teams
        * nobody wants jurisdiction of a problem
            * deprecation policy around github projects --
                * how do we set the "procedure" around deprecation?
                * is that release team? or is it govenance wg? or who?
                * release team might execute it, but not necessarily set policy
                * does that default to core?
        * dispute about "who has jurisdiction"
            * example dispute around the `!` stabilization
* How to improve communication?
    * How can we provide stucture to improve communication?
    * Seems like something that would require deeper analysis from looking at teams

### Things that have been "added" since the RFC was written

* New teams and roles within teams
    * core team observer, lang team shepherds, [compiler team contributors](https://rust-lang.github.io/rfcs/2689-compiler-team-contributors.html)
* Domain working groups
* Project groups
* RFC policy changes for certain teams
    * new teams have no policy, old team policies are out of date
* Inside Rust blog

### Things that have changed or just not happened

* Subteam lead resolving contentious issues
* Feature gating decided by teams, not by core team
* In general, core team has not been involved in technical decision making, and has been more focused on policy

### Things that could be improved

* Regularly looking over the policy documents to see if they still reflect reality

### Output and goals

* This meeting: come up with a plan for updating the RFC to be more inline with how community functions
    * Define working groups / project groups ([draft](https://hackmd.io/s/rJn0cDFsB))
* Convert the text of RFC 1068 to a forge structure
    * Governance
        * RFC Policies
            * Language changes
            * Library additions

* Create `draft RFC` folder in `wg-governance` repo.

### Next meeting sketches

* Follow-up on this meeting
* Access policy thing -- get pietro or some other folks, maybe, but can they make this time?
* RFC proposal https://github.com/nikomatsakis/project-staged-rfcs/blob/master/rfcs/0001-shepherded-rfcs.md
* Try to contact non-Rust people

### Next meeting run by Nell

* Follow-up from this meeting:
    * Record minutes
    * Write a blog post summarizing some of this discussion

* Access policy
    * Homework:
        * read [issue #4] which has the discussion
        * Nell to talk to pietro
    * Goal:
        * get some first draft text

[issue #4]: https://github.com/rust-lang/wg-governance/issues/4