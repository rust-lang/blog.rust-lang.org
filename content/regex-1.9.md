+++
path = "2023/07/05/regex-1.9"
title = "Announcing regex 1.9"
authors = ["Andrew Gallant"]
aliases = ["2023/07/05/regex-1.9.html"]

[extra]
team = "The regex crate team"
team_url = "https://www.rust-lang.org/governance/teams/library#Regex%20crate%20team"
+++

The regex sub-team is announcing the release of `regex 1.9`. The `regex` crate
is maintained by the Rust project and is the recommended way to use regular
expressions in Rust. Its defining characteristic is its guarantee of worst case
linear time searches with respect to the size of the string being searched.

Releases of the `regex` crate aren't normally announced on this blog, but
since the majority of its internals have been rewritten in version 1.9, this
announcement serves to encourage extra scrutiny. If you run into any problems
or performance regressions, please report them on the [issue tracker] or [ask
questions on the Discussion forum][discussions].

Few API additions have been made, but one worth calling out is the
[`Captures::extract`] method that should make getting capture groups in some
cases more convenient. Otherwise, the main change folks should see is hopefully
faster search times.

You can read more in the [CHANGELOG] and in a more in depth blog post on
[regex crate internals as a library][regex-internals].

[issue tracker]: https://github.com/rust-lang/regex/issues
[discussions]: https://github.com/rust-lang/regex/discussions
[`Captures::extract`]: https://docs.rs/regex/1.*/regex/struct.Captures.html#method.extract
[CHANGELOG]: https://github.com/rust-lang/regex/blob/master/CHANGELOG.md#190-2023-07-05
[regex-internals]: https://blog.burntsushi.net/regex-internals/
