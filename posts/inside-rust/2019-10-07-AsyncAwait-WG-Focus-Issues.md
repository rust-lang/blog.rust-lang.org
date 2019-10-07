---
layout: post
title: "Async Foundations Update: Time for polish!"
author: Niko Matsakis
description: "A new blog where the Rust team can post updates on the latest developments"
team: the Async Foundations WG <https://rust-lang.github.io/compiler-team/working-groups/async-await/>
---

As you've perhaps heard, recently the async-await feature [landed on
the Rust beta branch][blog]. This marks a big turning point in the
usability story for Async Rust. But there's still a lot of work to do.
As we mentioned in the main post, the focus for the [Async Foundations
WG][wg] in the immediate term is going to be **polish**, **polish**
and (ahem) **more polish**.

In particular, we want to take aim at a backlog of strange
diagnostics, suboptimal performance, and the occasional inexplicable
type-check failure. This is a shift: whereas before, we could have
laser focus on things that truly blocked stabilization, we've now got
a large set of bugs, often without a clear prioritization between
them. This requires us to mix up how the [Async Foundations WG][wg] is
operating.

[wg]: https://rust-lang.github.io/compiler-team/working-groups/async-await/
[blog]: /2019/09/30/Async-await-hits-beta.html

### Announcing: focus issues

So how do you deal with a large pile of issues, all of which are
important but none of which are vital? One at a time, of course.

The way we've chosen to organize this is something we call **[focus
issues][fi]**. We're trying to keep a small number of issues tagged as
focus issues at any given time. As we close them, we'll pick new ones
to replace them. The number of these issues depends on mentoring
bandwidth and on how many people are hacking -- as a rule of thumb,
they should mostly all be assigned and actively progressing at any
given time.

[fi]: https://rust-lang.github.io/compiler-team/working-groups/async-await/#how-to-get-involved

We also have a secondary set of issues called **on deck issues**.
These are the candidates to become focus issues as focus issues are
completed. If you'd like us to consider fixing something sooner rather
than later, you can [add the "on deck" label yourself][nom], along
with a bit of context explaining why you think this issue is more
important than the rest.

### How you can help

You can help in two ways:

* **Fix bugs!** If you'd like to take a shot at fixing a bug, try to
  come to the [triage meeting] or just show up in
  [`#wg-async-foundations` on Zulip][z]. Maybe we can find something
  for you.
* **Nominate bugs!** If you've got a bug that is really
  annoying you, feel free to "nominate it" by [following the
  instructions here][nom]. This will help us to fix the things that
  are bothering people the most.

[triage meeting]: https://rust-lang.github.io/compiler-team/working-groups/async-await/#triage-meeting
[nom]: https://rust-lang.github.io/compiler-team/working-groups/async-await/#nominating-issues
[z]: https://rust-lang.zulipchat.com/#narrow/stream/187312-wg-async-foundations
