+++
layout = "post"
title = "Core Team Update: May 2021"
author = "Steve Klabnik"
team = "The Core Team <https://www.rust-lang.org/governance/teams/core>"
+++

Hey everyone!  Back in August of last year, the core team wrote a blog post
titled "[Laying the foundation for Rust's Future][future]." Ever since then,
the Core Team has been doing a tremendous amount of work to help get the
foundation going, and prepare the project for the changes that have now
occurred because of these events.

But that also means we've been very quiet!  This sort of work has largely
been focused inward, and not really something that's visible from the
outside, even if you are on a Rust team.  However, thanks to these efforts,
the Foundation now exists, and is starting to work on its mission.  That also
means it's time for the core team to shift the focus of its work.

Beyond the Foundation efforts, we've also had several initiatives underway
that you may or may not know about:

## The 2021 Roadmap

The [2021 Roadmap] RFC was merged back in January.  This set the goals for
the core team for this year.  This is a bit different from previous years,
where we laid out goals for the entire project.  We decided this year to
focus on Core, and give the teams the leeway to set their own goals, while we
focused on overall organizational health.

Later in the year, we'll be starting the process for next year as well. We're
not actively thinking about this yet, but ideally, a yearly roadmap would be
merged in December, rather than in January, so we want to make sure and begin
early enough to get this shipped on time for 2022.

## Team Charters

As part of that work, we've begun the first steps of a process to give each
team a formal charter.  Way back in [RFC 1068], the scope of the initial
teams was laid out.  While that has served us well over the years, as teams
were spun up, shut down, and changed, we haven't always done a good job of
making it clear where the boundaries of responsibility lie in each team.
Part of the magic in Rust's governance structure is that individual teams are
given significant authority to do as they see fit, but that also means that
we have to be concious about scope. We'll have more to report on this process
as it continues to unfold, but the end goal is stated in the roadmap:

> The Rust teams, in concert with the core team, will work to establish a
> charter for each of the Rust teams over the course of the year, with an aim
> for defining, particularly, the purpose and membership requirements.  Our
> goal is that going into 2022, all active groups within the Rust project will
> have well-defined charters and membership.

Now is the time to redouble efforts here, and we are excited to work with all
of the teams on nailing these charters down. As a start, we've been working with
the Libs, Release, and Cargo teams for an initial pass at their charters. We've
structured this to be largely team-driven; we're setting expectations and framing
what charters should look like, but the teams work on what goes in them. As this
progresses, we hope to learn more about the challenges involved so that we can
improve our process and bring in more teams.

It's worth pointing out explicitly that this work also includes the Core Team; we'll
be creating a charter for ourselves as well. 

## Audit of packages owned by the project

As another example of something the core team has been working on, we've been
doing work to clarify the status of a number of packages owned by the Rust team
on crates.io. We are conducting a full audit of these packages, making sure that
they're things that should be owned by the project, making sure that they have
appropriate permissions, making sure that they have people taking care of them,
all of that kind of thing. Historically, we've been fairly ad-hoc about this sort
of thing, but as we grow, it is very imporant to be deliberate. An
[RFC][crate-ownership-rfc] was just opened to create a policy here.

## Thanks!

So that's a quick summary of what we've been up to, and some of what we'll be
doing in the immediate future. We plan on trying to communicate what Core is working 
on more often in the future; 2020 was an extremely complex year for a variety of
reasons, but we're feeling really positive about the future of Core and the
Rust project generally. Thank you to the teams for all you've done for Rust.

[future]: https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html
[2021 Roadmap]: https://github.com/rust-lang/rfcs/pull/3037
[RFC 1068]: https://github.com/rust-lang/rfcs/blob/master/text/1068-rust-governance.md
[crate-ownership-rfc]: https://github.com/rust-lang/rfcs/pull/3119
