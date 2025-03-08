+++
layout = "post"
date = 2019-12-20
title = "2019-12-17 Infrastructure Team Meeting"
author = "Pietro Albini"
team = "the infrastructure team <https://www.rust-lang.org/governance/teams/operations#infra>"
+++

Meeting run by pietroalbini. Minutes written by pietroalbini.  
Attending: aidanhs, alexcrichton, kennytm, Mark-Simulacrum, pietroalbini,
shepmaster

## Spurious Chocolatey failures

Since a few days ago, Chocolatey started returning 503 error codes at random,
preventing some Windows CI runners from downloading MSYS2, and the tree was
closed to prevent the failures from affecting a bunch of PRs. There were a few
proposals on how to work around them, spanning from a proper mirroring
implementation to just a quick fix to get CI working again.

After the discussion the team agreed to prioritize getting CI working
consistently rather than implement the correct solution from the start. aidanhs
is going to look into that during the weekend, if nobody else finishes the work
before.

## Progress on moving the infrastructure to Terraform

In the past few months pietroalbini started exploring moving the configuration
of Rust’s AWS infrastructure to Terraform, to allow for better collaboration
and auditability. So far a few services have been moved over to Terraform, and
pietroalbini proposed in the meeting to evaluate moving dynamic applications
over to ECS and configure those with Terraform.

alexcrichton brought up the point that pietroalbini is at the moment the only
person in the team familiar with our Terraform setup, and while it’s probably a
better solution than the status quo we need more people able to work with it
before we decide to fully commit to using it. He acknowledged the current
solution (services manually configured through the console) is not scalable nor
long term, but we can’t jump to Terraform if most of the team doesn’t know how
to work with it.

shepmaster said that this is not much different than the situation in the past,
where alexcrichton was the only one knowing how our infrastructure worked
(since then other people in the team got up to speed). aidanhs pointed out that
we had a case in the past like that, when only alexcrichton knew how a system
worked and it broke while he was on holiday.

alexcrichton proposed to pause investing time into Terraform until at least
another team member gets familiar with it, and Mark-Simulacrum said he’s
willing to pair with pietroalbini for a few hours to migrate one of the
services to Terraform, learning how it works in the process.

The feelings about ECS turned up to be mostly the same: everyone agreed it’s
surely a better solution than the setup we have right now, but only
pietroalbini is familiar with it. We agreed that pietroalbini and
Mark-Simulacrum are going to deploy one of the small applications to ECS using
Terraform while pairing, testing both the new things in one go.

In the end, pietroalbini reminded the team that not every application we
currently host is going to fit into ECS + Fargate without changes, especially
because you can’t really persist data on the filesystem with it, but he
mentioned he’s willing to do the implementation work to adapt those
applications not to use the filesystem anymore.
