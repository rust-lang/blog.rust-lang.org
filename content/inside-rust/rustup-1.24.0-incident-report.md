+++
path = "inside-rust/2021/04/28/rustup-1.24.0-incident-report"
title = "Rustup 1.24.0 release incident report for 2021-04-27"
authors = ["Daniel Silverstone"]
aliases = ["inside-rust/2021/04/28/rustup-1.24.0-incident-report.html"]

[extra]
team = "the Rustup team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#wg-rustup"
+++

On 2021-04-27 at 15:09 UTC we released a new version of Rustup (1.24.0). At
15:23 UTC we received a report that we had introduced a regression in the part
of the code which is responsible for proxying the `rustfmt` and `cargo-fmt`
portions of Rust toolchains. At 15:27 UTC we had confirmed and identified the
cause of the problem, and while we worked on a fix, we reverted the released
Rustup to version 1.23.1 - an action completed by 15:56 UTC.

This means that for approximately 47 minutes, CI jobs which used the code
formatting features of Rust toolchains may have had spurious failures, and users
who upgraded will have had to revert to 1.23.1. The revert process is designed
to be as easy as upgrading was, meaning that users should not have had lingering
issues.

## Root cause of the issue

In an effort to
[reduce confusion when downloaded copies of `rustup-init.exe`are renamed][rcon]
we merged a change which causes Rustup to report an error if an unknown name is
used when invoking the binary.

[rcon]: https://github.com/rust-lang/rustup/issues/2286

Due to past complexities with `rustfmt` and `cargo-fmt` being binaries which
tended to be distributed by `cargo install` rather than via
`rustup component add` there is some intricate handling in Rustup's proxy
management for those specific binaries. The fix for the above issue failed to
include these corner case proxies in the check it undertook to ensure the caller
hadn't used an unexpected binary name.

The 1.24.0 release had been staged at this point, however there was a problem
with the low-memory installation pathways which required a fix, and at the time
we incorrectly assessed that it was low-impact to rebase the release onto the
new master branch which had not only the fix for the low-memory installation
pathway but also the "refuse bad names" change for the above issue.

Subsequent testing of the release focussed almost entirely on the installation
pathways, omitting to validate the proxy name verification code we had also
acquired into the release. As a result, this regression slipped in.

## Resolution

The author of the validation PR correctly identified it as the root-cause of
the regression, and the team discussed and decided that it was better to fix
the problem properly than to simply revert the change out of the release.

The release team member who was helping with the release process began the
revert to Rustup 1.23.1 while the fixes were developed. In addition an issue
was filed around adding some tests around all the proxies (we currently test a
subset which we believed to be representative). We subsequently staged a
proposed 1.24.1 release to Rust's development stage and we have issued a [call
for beta testers][beta] to confirm that we have not introduced any other
regressions.

[beta]: https://internals.rust-lang.org/t/seeking-beta-testers-for-rustup-1-24-1/14582

## Lessons learned

The big lesson here is that while we've taken similar notes away from past
releases of Rustup and other tooling, we've not yet managed to set up a proper
beta-testing process for Rustup. As such we will be making changes to the
Rustup release process to codify testing phases with the wider community.

## Long term changes to Rustup releases

In order to try and reduce the chance of this happening again, the [release
process][rp] will be updated to include a public beta-testing phase for any non-
purely-bugfix release and we intend to look into the possibility of a "nightly"
Rustup release for a _small_ subset of platforms.

[rp]: https://github.com/rust-lang/rustup/blob/master/CONTRIBUTING.md#making-a-release

Finally we are hoping to work with the [release team][rt] to do what we can to
unify the Rustup release process with the well oiled Rust release process
though, due to the historical differences in how Rustup has been released, this
will likely be a long term effort.

[rt]: https://www.rust-lang.org/governance/teams/release

## Action items

- [#2739]: Testing for proxying, including TOOLS and DUP_TOOLS
- [#2741]: Release process should include explicit beta test period

[#2739]: https://github.com/rust-lang/rustup/issues/2739
[#2741]: https://github.com/rust-lang/rustup/issues/2741

And as mentioned above, longer term we shall look to see what unification we can
do between releasing Rustup and how the Rust release train runs.
