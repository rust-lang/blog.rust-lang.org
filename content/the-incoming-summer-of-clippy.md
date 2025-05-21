+++
path = "9999/12/31/the-incoming-summer-of-clippy"
title = "The incoming Summer of Clippy"
authors = ["blyxyas"]

[extra]
team = "the Clippy team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-clippy"
+++

We are currently working on a feature freeze for the Clippy project. This
period would be 12 weeks long, starting on the introduction of 1.89 to beta
on the 26 of June, and ending on the 18th of September.

This *summer of Clippy* comes from a lack of the necessary capacity needed
to maintain all the current lints (about 750 of them 😱) and still add new ones.
We need to care for the Clippy project the same way that Clippy cares about our
code, and note that every single one of them needs maintaining and accounting
for current and future changes in the compiler as a whole.

As a team we’ll focus on making contributing the best it can be, and mark the
issues that could be great as a starting point to get familiar with the project.

## What can I expect as a user?

As a user you can expect to have more accurate lints, with less false negatives
and more edge cases covered. A bump into Clippy’s toolchain would not involve
the same risk for false positives as it has been until now. We hope that by
upping the standard for lints all around the project we can allow users to
expect even more from the already great linter.

If you have a lint which you want to be implemented, you can still open an
issue for it, but no one will implement it (also, lint-adding pull requests
opened after the starting date will be blocked for the duration of the period).

Note that pull requests opened before the starting date can still be reviewed
and merged, but unless they are already in a very advanced state they will not
be prioritized over bugfixes.

## Can I help?

As always, any help on the project is very appreciated. Apart from opening bug
reports, you can follow [our contributing guidelines] and check [our suggested
good first issues] for opening pull requests . Everything from fixing diagnostic
issues and improving documentation, to fixing application-wide crashes
really helps Clippy get into the excellent state that our users deserve.

If you need direct help when fixing a bug, don't hesitate to open a thread on
Zulip, or to open the PR with the questions you have in order to start the
feedback stream.

## Conclusion

Thanks a lot for all these years of supporting Clippy, we hope that via this
little feature hiatus we can achieve the excellence that the Rust user base
deserves (if that's even possible).

[our contributing guidelines]: https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md
[our suggested good first issues]: <!-- ADD HERE OUR TRACKING ISSUE -->


