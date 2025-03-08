+++
layout = "post"
title = "Lang Team Design Meetings"
author = "Niko Matsakis"
description = "Lang Team Design Meetings"
team = "the language team <https://www.rust-lang.org/governance/teams/lang>"
+++

Hi all! I wanted to give a quick update about the lang team. We're
starting something new this year: a regular **design meeting**.  The
idea of the design meeting is that it's a time for us to have in-depth
discussions on some particular topic. This might be a burning problem
that we've discovered, an update on some existing design work, or a
forward looking proposal.

The meetings will be scheduled in advance. Ideally, each meeting
should also have a blog post before-hand giving background material,
although that may be difficult to achieve in practice. Similarly,
after each meeting, we'll typically post minutes and a recording, and
ideally try to write up a summary blog post with major
points. (Sometimes recordings are hard to achieve, either for
technical reasons or because we wanted the ability to discuss more
sensitive topics.)

The meetings are open for anyone to listen in and attend. They
generally take place on Mondays at "noon Boston time" -- but for the
precise scheduling you should check the [lang team calendar]. We'll
try to keep the calendar up to date with the topic to be discussed in
each meeting, as well.  Meetings might not happen every week, if we
don't have a topic in mind.

[lang team calendar]: https://github.com/rust-lang/lang-team/#meeting-calendar

## First design meeting

Our first design meeting was actually last Monday! We discussed the
soundness hole that was found some time ago in `Pin`.  You can read
the [minutes] from our discussion and a [recording] is also
available. You might also find it easier to read the [comments in the
internals thread][comments]. The good news is that we seem to be
centralizing on a [solution based on negative impls][neg].

[minutes]: https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2020-01-06-Pin-unsoundness.md
[recording]: https://youtu.be/MX_GRNLhlY8
[comments]: https://internals.rust-lang.org/t/unsoundness-in-pin/11311/112?u=nikomatsakis
[neg]: https://internals.rust-lang.org/t/explicit-negative-impls-to-fix-pin-soundness-hole/11587

## Next design meeting

The next design meeting will take place this coming Monday. The topic
will be how to fix [rust-lang/rust#57893], which is a soundness bug
related to `dyn` types. There is a [prepared writeup describing the
problem and a possible solution][gist] to guide the meeting. It is
maybe worth noting that one of the proposed solutions involved a Rust
2021 edition as well (although there are smaller, more targeted fixes
that do not require a new edition).

[rust-lang/rust#57893]: https://github.com/rust-lang/rust/issues/57893
[gist]: https://gist.github.com/0cf84ac05ce7751b5759cbf335c4d327

## Design meetings after that

The next two weeks are likely to not have design meetings:

* January 20th is Martin Luther King, Jr. day in the US.
* January 27th is the Mozilla All Hands.

As a result, in both cases, many of the lang team members will be
unable to attend.
