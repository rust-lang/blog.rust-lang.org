---
layout: post
title: "The Rustc Learning Working Group - An Introduction"
author: Amanjeev Sethi
description: "introduction learning working group useful links"
team: the rustc learning working group <https://www.rust-lang.org/governance/teams/compiler#wg-learning>
---

The [Learning Working Group], formed in April 2019, is focused on making the
compiler easier to learn by ensuring that [rustc-guide] and API docs are
"complete". It is one of the many efforts by the Rust Compiler team to
decrease the barrier of contributing to the compiler. As noted on the WG’s
homepage —

*This working group aims to accomplish the following:*

- *Ensure that major components of rustc are covered in rustc-guide*
- *Ensure that API doc coverage is at least 90%*

The Learning Group is making entry to contribute easier by improving the
compiler documentation for the new and potential contributors. We all know
this bias — the more time we’ve spent in a system, the less likely we are to
see the issues that the newcomers might face. Given that, this group
organically became an attractive place for beginners, who would benefit
from documenting the compiler internals, while learning those parts of it
at the same time. This benefits the entire compiler team by giving the
documentation a perspective from the new contributors.

The Learning group, in general, is starting to document the
“Compiler lecture series”, which are a list of Youtube video lectures
given earlier by the more knowledgeable members of the compiler team.
There is also the task of improving the documentation structure of
[rustc-guide]. At first, each member used to pick a video lecture by
themselves and contribute via a Github pull request to the
[rustc-guide Github repository]. This proved to be a bit difficult
for the following reasons —

1. Not all members would get to watch and work on the lectures of
their choice.
2. The knowledge would still be fragmented depending on who watched
which lecture.
3. Some lectures are more interesting than others and this means that
some contributors would miss out on the interesting ones.
4. Certain lectures are more difficult and require input from multiple
people.

Hence, at the moment, the group decided to work on one video at a time.
This has proven to be beneficial for the entire group, especially with
the lecture [Representing types in rustc]. At the time of this writing,
it is still a work-in-progress but the input from everyone allows people
to work together while benefiting from the questions that everyone has
about the lecture.

The group is still learning about the best possible ways to organize and
manage and some compiler team veterans are always there to help out!
We are always in need of help from both existing compiler contributors
and new folks who want to contribute and learn.

## Call for participation

There is no bar to entry in the group. To join the group, you can
drop a message in [`#t-compiler/wg-learning` on Zulip] introducing
yourself. We would love to hear from you and hope we all together
can make the documentation better.

## Important  resources

- **[Learning Working Group]**
- **[Rustc Guide Book]**
- **[Rustc Guide Repository]**
- **[Github Project (Kanban)]**
- **[Learning WG Meeting Minutes]**
- **[Rust Youtube Videos]** 
- **[Zulip Stream: `#t-compiler/wg-learning` on Zulip]**

[Learning Working Group]: https://github.com/rust-lang/compiler-team/tree/master/content/working-groups/learning
[rustc-guide]: https://rust-lang.github.io/rustc-guide/
[Rustc Guide Book]: https://rust-lang.github.io/rustc-guide/
[rustc-guide Github repository]: https://github.com/rust-lang/rustc-guide
[Rustc Guide Repository]: https://github.com/rust-lang/rustc-guide
[Representing types in rustc]: https://www.youtube.com/watch?v=c01TsOsr3-c
[Github Project (Kanban)]: https://github.com/rust-lang/rustc-guide/projects/2
[Learning WG Meeting Minutes]: https://github.com/rust-lang/compiler-team/tree/master/content/working-groups/learning/minutes
[Rust Youtube Videos]: https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA/playlists
[rust-lang/rust]: https://github.com/rust-lang/rust
[Zulip Stream: `#t-compiler/wg-learning` on Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-learning
[`#t-compiler/wg-learning` on Zulip]: https://rust-lang.zulipchat.com/#narrow/stream/196385-t-compiler.2Fwg-learning
