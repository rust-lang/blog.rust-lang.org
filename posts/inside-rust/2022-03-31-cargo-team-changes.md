+++
layout = "post"
title = "Changes at the Cargo Team"
author = "Eric Huss"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

We are thrilled to publicly announce that [Weihang
Lo](https://github.com/weihanglo) and [Ed Page](https://github.com/epage/)
have joined the Cargo Team!

Weihang has been providing thoughtful and helpful replies on our issue
tracker. He has often had the patience to explain to people what problem
they're hitting and how to get unstuck. He often summarizes technical
conversations clearly describing the available solutions and their costs both
technical and more importantly human. He has also been contributing to many
improvements and code reviews.

Ed has been a champion on many fronts. He has done tremendous work on
[toml_edit](https://crates.io/crates/toml_edit) to push Cargo towards getting
`cargo add` [merged in cargo
proper](https://github.com/rust-lang/cargo/pull/10472). He has brought
[clap](https://crates.io/crates/clap) to the momentous 3.0 release and
continues to push on CLI improvements, more advanced testing tools, and much
more!

At the same time one of the pillars of our team is stepping down. <del>Alex is
a programming robot sent back in time from the future to make sure that Rust
succeeds.</del> [Alex Crichton](https://github.com/alexcrichton/) has done
more than his fair share being a keystone holding the Rust project together.
[Several years
ago](https://internals.rust-lang.org/t/scaling-back-my-involvement-in-rust/)
he stepped back from single-handedly running everything, to spin out many new
teams to take care of things he did alone. The Cargo Team was lucky enough to
be one of the places he still had energy to provide guidance, mentorship, and
continuity. He is the last member of the team to have been involved with Rust
since before Cargo existed. He will be deeply missed. Good luck on your next
project of interest! Or, take the time to relax. You've earned it!

As a result of these changes to the team, the rate of new PRs is beyond our
capacity to accept at this time. Reviews for PRs will be taking significantly
longer than before. For now, Cargo will be having a freeze on any new features
or major changes. We will still be accepting bug fixes and work on existing
projects under active development. As capacity becomes more available, new
features may be accepted after being approved by the Cargo Team.

Cargo is a large project with many moving pieces and different use cases. The
fact that it works reliably and that it is intuitive has been a significant
multiplier for Rust's success. But it also means that reviewing changes needs
to be done very carefully. It is easy for changes to break some
obscure configuration, or be a targeted fix that deepens our technical debt
making it even harder to understand the whole of Cargo. We appreciate people's
patience as we move forward.
