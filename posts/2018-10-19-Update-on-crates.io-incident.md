---
layout: post
title: "Update on the October 15, 2018 incident on crates.io"
author: The Crates.io Team
---

On Monday, Oct 15, starting at approximately 20:00 UTC, crates.io sustained
an operational incident. You can find the status page report [here][status] and our
tweets about it [here][tweets].

[status]: http://status.crates.io/incidents/cymjwvkrtjn3
[tweets]: https://twitter.com/cratesiostatus/status/1051953125030940674

## Root Cause

A user called `cratesio` was created on crates.io and proceeded to upload
packages using common, short names. These packages contained nothing beyond a
`Cargo.toml` file and a `README.md` instructing users that if they wanted to use
the name, they should open an issue on the crates.io issue tracker.

The rate at which this user uploaded packages eventually resulted in our servers
being throttled by GitHub, causing a slowdown in all package uploads or yanks.
Endpoints which did not involve updating the index were unaffected.

We decided to take action on this behavior because:
 - The contents of the uploaded packages appeared to be an attempt to impersonate
the crates.io team (both through the username `cratesio`, as well as directing people
to the crates-io issue tracker in the crates' `Readme` files)
 - the rate of uploading impacted the stability of the service

## Action Taken

The user's IP address was banned immediately. We then backdated the users' packages to remove
their packages from the homepage. We also redirected the `cratesio` user's page to a 404.

Finally, the `cratesio` user and all crates they uploaded were deleted.
The user was reported to GitHub, and has since been banned by them.

## Timeline of events

- 20:09 UTC: The GitHub user `cratesio` registers an account
- 20:13 UTC: This user begins uploading packages at a rate of roughly one package
  every 2 seconds
- 20:17 UTC: All requests updating the index begin to take 10+ seconds
- 20:41 UTC: An email is sent to the Rust moderation team reporting this user
- 20:46 UTC: The report is forwarded to the crates.io team
- 20:50 UTC: The user is reported in the crates.io team Discord.
- 21:00 UTC: The user's IP address is blocked from accessing the site
- 21:20 UTC: The user's packages were removed from the crates.io homepage
- 21:20 UTC: The incident is announced on status.crates.io
- 22:49 UTC: The user's account page on crates.io is removed.
- 23:58 UTC: The packages, all associated data, and the user's account are deleted
  from crates.io
- 00:40 UTC: The packages are removed from the index.

## Future measures

It should not have been possible for a single user or IP address to upload that
many packages in a short period of time. We will be introducing rate limiting on
this endpoint to limit the number of packages a script is able to upload in the
future.

We are also looking into disallowing usernames that could be impersonating
official Rust teams. We will be updating our policies to clearly state that this
form of impersonation is not allowed. We will be deciding the exact wording of
this policy in the coming weeks.

While it is impossible to tell a user's intent, many, including the team, have
speculated that this action was either associated with or directly related to the
recent escalation in community frustration around crates.io policies, in particular,
the squatting policy.

Regardless of whether this incident had this intent, the cratesio team would like
to reiterate that taking actions such as the one we experienced on Tuesday is not
an appropriate nor effective way to contribute to dialogue about crates.io policy.
We will be adding a policy making it clear that attempting to disrupt crates.io in order
to make or further a point is not appropriate and will be considered a malicious attack.
We will be deciding on the exact wording of this policy in the coming weeks.

If you feel that a policy is problematic, the correct place to propose a change is by
creating an RFC or messaging the team at help@crates.io.

We also have seen a lot of frustration that the crates.io team is not listening to the concerns
that are being raised on both official and unofficial Rust forums. We agree that we should
improve our communication with the community and intend to develop more processes
for folks to communicate with us, as well as for the team to communicate to the general
community.

## Background

There has been a growing amount of discussion in the community around our
squatting policy and our decision not to have namespacing.

[The original squatting policy](https://internals.rust-lang.org/t/crates-io-package-policies/1041),
published in 2014, contains a lot more information about the rationale behind
the policy than what is currently on our website. The full text of the original
policy is:

> Nobody likes a “squatter”, but finding good rules that define squatting that
> can be applied mechanically is notoriously difficult. If we require that the
> package has at least some content in it, squatters will insert random content.
> If we require regular updates, squatters will make sure to update regularly,
> and that rule might apply over-zealously to packages that are relatively
> stable.

> A more case-by-case policy would be very hard to get right, and would almost
> certainly result in bad mistakes and and regular controversies.

> Instead, we are going to stick to a first-come, first-served system. If someone
> wants to take over a package, and the previous owner agrees, the existing
> maintainer can add them as an owner, and the new maintainer can remove them. If
> necessary, the team may reach out to inactive maintainers and help mediate the
> process of ownership transfer. We know that this means, in practice, that
> certain desirable names will be taken early on, and that those early users may
> not be using them in the most optimal way (whether they are claimed by squatters
> or just low-quality packages). Other ecosystems have addressed this problem
> through the use of more colorful names, and we think that this is actually a
> feature, not a bug, of this system. We talk about this more below.

We will be discussing whether including some of this information in the policy
published on our website would help more people to understand the rationale
behind our policy, without requiring members of the team to reply to every forum
thread wanting to re-litigate what has already been discussed at length.

## Conclusion

We wanted to share the details of what happened and why the crates.io team chose to take action
as quickly as possible. The policy changes we've described will be discussed
during the next several team meetings. Nothing is set in stone until the team
has a chance to discuss them further, but we wanted to share the possible
changes we're discussing to limit speculation on what future actions we're
planning on taking.

As a reminder, if you would like to report an incident regarding cratesio, you
can message the team at help@crates.io. You can view the status of the service
at https://crates-io.statuspage.io/ and/or by following @cratesiostatus on Twitter.
