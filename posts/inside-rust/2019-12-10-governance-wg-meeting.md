---
layout: post
title: "Governance Working Group Update"
author: Niko Matsakis
team: the Governance WG <https://github.com/rust-lang/wg-governance>
---

Hello everyone! The governance working group met last week to discuss
writing out a policy for access privileges on our Github
repositories. This blog post summarizes that meeting and also
announces the topic of our next meeting, which takes place on Tuesday,
December 17, 2019 ([calendar event]).

[calendar event]: https://calendar.google.com/event?action=TEMPLATE&tmeid=MnNuZDc0NzkzYTBkcDNzY2FkbWgzNjk3a2hfMjAxOTEyMTdUMjIwMDAwWiBtb3ppbGxhLmNvbV85YzZmYzNsNmJoZzhiY3A4Y2FmcnZrM29mNEBn&tmsrc=mozilla.com_9c6fc3l6bhg8bcp8cafrvk3of4%40group.calendar.google.com&scp=ALL

Also, this week we have a [video recording
available](https://youtu.be/CyYwretwM8E). (We're going to generally
try and record meetings when possible.)

## Next meeting

The next meeting will be discussing project groups and their integration into
the lang team. This is building on a few different posts and ideas:

* XAMPPRocky's draft RFC [clarifying our terminology around working groups](https://github.com/rust-lang/wg-governance/blob/master/draft-rfcs/working-group-terminology.md)
* My [Shepherds 3.0](http://smallcultfollowing.com/babysteps/blog/2019/09/11/aic-shepherds-3-0/) blog post
* The embedded working group's [shepherded projects RFC](https://github.com/rust-embedded/wg/pull/378)
* My recent blog post about [an improved pre-RFC
  process](http://smallcultfollowing.com/babysteps/blog/2019/12/03/aic-improving-the-pre-rfc-process/)

## Access rights policy

I'll summarize our conclusions here. Consult the [wg-governance]
repository to find more [detailed minutes] from our conversation.
The key conclusions were:

* Where possible, we should stick to a single org ([`rust-lang`]).
    * In particular, team-specific organizations like
      [`rust-dev-tools`] and [`rust-community`] ought to be merged
      into [`rust-lang`].
    * Using a single organization makes it much easier to administrate.
    * Note that we've already deprecated the [`rust-lang-nursery`] org
* As an exception, we will for now continue having each domain working group
  operate outside of its own org (e.g., [`rust-embedded`]). Those orgs
  are quite active and have a diverse membership and we don't want to
  disturb that for now.
    * However, it would be good if each such org added the `rust-lang-owner` bot
      as an owner, so that the rust infra team has access.
* For repositories, we will avoid giving access to individuals, and instead try to
  give access only to entities (teams, working groups, etc) that are created and
  managed by the Rust [team] repository.
    * In general, it is not recommended to give owner or admin access; write access suffices.
    * (Unfortunately, read and triage access is often not sufficient for us.)

We also enumerated a number of [action items] to putting this policy
in to practice. We'll be revisiting the topic periodically to check on
progress.

[wg-governance]: https://github.com/rust-lang/wg-governance/
[detailed minutes]: https://github.com/rust-lang/wg-governance/blob/master/minutes/2019.12.03.md
[wg-governance]: https://github.com/rust-lang/wg-governance/
[`rust-dev-tools`]: https://github.com/rust-dev-tools/
[`rust-community`]: https://github.com/rust-community/
[`rust-lang`]: https://github.com/rust-community/
[`rust-lang-nursery`]: https://github.com/rust-lang-nursery/
[`rust-embedded`]: https://github.com/rust-embedded/wg
[team]: https://github.com/rust-lang/team
[action items]: https://github.com/rust-lang/wg-governance/blob/master/minutes/2019.12.03.md#action-items
