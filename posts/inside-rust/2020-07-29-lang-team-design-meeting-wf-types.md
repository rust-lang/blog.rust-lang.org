+++
layout = "post"
title = "Lang team design meeting: well-formedness and type aliases"
author = "Niko Matsakis"
description = "Well-formedness and type aliases meeting report"
team = "the lang team <https://www.rust-lang.org/governance/teams/lang>"
+++

Hello! Did you know that the [lang team] now has regular design
meetings? We use these meetings to dig deeper into the output of
active project groups. After the meeting, we typically post a
recording to [YouTube] as well as some [minutes into the lang-team
repository][min]. I wanted to write a quick update listing out some of
the meetings we've had recently as well as some of our upcoming
meetings.

[YouTube]: https://www.youtube.com/playlist?list=PL85XCvVPmGQg-gYy7R6a_Y91oQLdsbSpa
[lang team]: https://www.rust-lang.org/governance/teams/lang
[min]: https://github.com/rust-lang/lang-team/tree/master/design-meeting-minutes

This blog post is about the meeting we held on 2020-07-29. We
discussed the idea of trying to enforce the "well-formedness" rules
for type aliases, as has been floated on and off over the years.

The context is that the compiler's current rules expand type aliases
as if they were a kind of macro, which means that we don't wind up
enforcing many sorts of rules about them.

For example, the following type alias definition is legal even though
it would be an error to ever use it:

```rust
struct MyType<T: Display> { t: T }

// This alias, perhaps, should err, as `Vec<u32>: Display`
// does not hold:
type MyAlias = MyType<Vec<u32>>;
```

For more information, check out the [minutes from the meeting] or
[watch the recording]. We covered a number of examples of what goes
wrong, as well as various possible "endstates" that we might want to
reach (for example, there is an argument that the above example should
be accepted after all, perhaps with a warning).

The conclusion during the meeting was that we would not put a lot of
energy into type aliases at this time, and in particular we wouldn't
aim for any Edition-related migrations and hard-errors, but we would
accept PRs that introduce warnings for type alias definitions that are
always an error to use. (Like any conclusion that happens in a
meeting, it may be revised if we encounter new evidence that changes
our minds.)

[minutes from the meeting]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-07-29-wf-checks-and-ty-aliases.md
[watch the recording]: https://youtu.be/tIBZYQSA_eM
