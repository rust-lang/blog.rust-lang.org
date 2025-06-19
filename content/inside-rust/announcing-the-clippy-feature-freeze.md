+++
path = "inside-rust/2025/06/19/announcing-the-clippy-feature-freeze"
title = "Announcing the Clippy feature freeze"
authors = ["blyxyas"]

[extra]
team = "the Clippy team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#team-clippy"
+++

The Clippy project will be on feature-freeze for 12 weeks, starting from Rust
1.89.0 beta (June 26th 2025) to September 18th 2025 (Rust 1.89.0 stable
release). During this time no new features will be accepted, only bug fixes.

This *feature freeze* comes from a lack of the necessary capacity needed
to maintain all the current lints (over 750 of them 😱) and still add new ones.
We need to care for the Clippy project the same way that Clippy cares about our
code, and note that every single one of them needs maintaining and accounting
for current and future changes in the compiler as a whole.

As a team we’ll focus on making contributing the best it can be, and mark the
issues that could be great as a starting point to get familiar with the project.

## What can I expect as a user?

As a user you can expect to have more accurate lints, with less false positives
and more edge cases covered. A bump into Clippy’s toolchain would not involve
the same risk for false positives as it has been until now. We hope that by
upping the standard for lints all around the project we can allow users to
expect even more from the already great linter.

If you have a lint which you want to be implemented, you can still open an
issue for it, but PRs adding new lints won't be reviewed during the feature
freeze. Instead, they will be put into a queue and reviewed (and merged) after
the feature freeze is over.

> Open pull requests that add new lints and are in a **very** advanced state
> might still get reviewed and/or merged during the feature freeze.

## Can I help?

As always, any help on the project is very appreciated. Apart from opening bug
reports, you can follow [our contributing guidelines] and check [our suggested
good first issues] for opening pull requests. Everything from fixing diagnostic
issues and improving documentation, to fixing application-wide crashes
really helps Clippy get into the excellent state that our users deserve.

If you need direct help when fixing a bug, don't hesitate to open a thread on
[Zulip] and/or open a draft PR with the questions you have in order to start
the feedback stream.

## Conclusion

Thanks a lot for all these years of supporting Clippy, we hope that via this
little feature hiatus we can achieve the excellence that the Rust user base
deserves. 

[our contributing guidelines]: https://github.com/rust-lang/rust-clippy/blob/master/CONTRIBUTING.md
[our suggested good first issues]: <!-- ADD HERE OUR TRACKING ISSUE -->
[Zulip]: https://rust-lang.zulipchat.com/#narrow/channel/257328-clippy

