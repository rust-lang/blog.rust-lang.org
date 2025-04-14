+++
path = "inside-rust/2020/03/11/lang-team-design-meetings"
title = "March Lang Team Design Meetings"
authors = ["Niko Matsakis"]
description = "Lang Team Design Meetings scheduled for March"
aliases = ["inside-rust/2020/03/11/lang-team-design-meetings.html"]

[extra]
team = "the language team"
team_url = "https://www.rust-lang.org/governance/teams/lang"
+++

We've scheduled our **language team design meetings** for March. We have plans for two meetings:

* March 16th -- we will summarize discussions from [the ffi-unwind project group] about whether we should permit foreign exceptions to propagate across the "C" ABI boundary, or establish a separate ABI (e.g., "C unwind") for such cases.
* March 23rd -- we will talk with James Munns about Sealed Rust (see the blog posts [The Pitch] and [The Plan])

[the ffi-unwind project group]: https://github.com/rust-lang/project-ffi-unwind
[The Pitch]: https://ferrous-systems.com/blog/sealed-rust-the-pitch/
[The Plan]: https://ferrous-systems.com/blog/sealed-rust-the-plan/

## About the language team design meetings

The idea of the design meeting is that it's a time for us to have
in-depth discussions on some particular topic. This might be a burning
problem that we've discovered, an update on some existing design work,
or a forward looking proposal.

The meetings are open for anyone to listen in and attend. They are
typically also recorded and posted online, along with minutes, after
the fact. They generally take place on Mondays at "noon Boston time"
-- but for the precise scheduling you should check the [lang team
calendar]. Scheduled meetings are subject to change and
cancelation. In that case, the calendar events will be updated.

[lang team calendar]: https://github.com/rust-lang/lang-team/#meeting-calendar
