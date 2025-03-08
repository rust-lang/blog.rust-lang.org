+++
layout = "post"
title = "February Lang Team Design Meetings"
author = "Niko Matsakis"
description = "Lang Team Design Meetings scheduled for February"
team = "the language team <https://www.rust-lang.org/governance/teams/lang>"
+++

We've scheduled our **language team design meetings** for February. The current plans are as follows:

* February 3rd -- we will do an overview of **specialization**. We'll cover details from the RFC but in particular talk about the [as-yet-unimplemented plans to make it sound](http://aturon.github.io/tech/2018/04/05/sound-specialization/).
* February 10th -- we will cover interactions between the `&T` type and LLVM's dereferenceable attribute, such as [rust-lang/rust#55005](https://github.com/rust-lang/rust/issues/55005) and [problems modeling MMIO](https://github.com/japaric/volatile-register/issues/10).
* February 17th -- no meeting, it is President's Day in the US and many of us are absent
* February 24th -- we will summarize discussions from [the ffi-unwind project group] about whether we should permit foreign exceptions to propagate across the "C" ABI boundary, or establish a separate ABI (e.g., "C unwind") for such cases.

[the ffi-unwind project group]: https://github.com/rust-lang/project-ffi-unwind

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
