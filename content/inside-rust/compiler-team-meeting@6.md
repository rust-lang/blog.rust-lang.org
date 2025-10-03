+++
path = "inside-rust/2020/02/07/compiler-team-meeting"
title = "2020-02-06 Compiler Team Triage Meeting"
authors = ["Wesley Wiser"]
description = "2019-02-06 Compiler Team Triage Meeting"
aliases = ["inside-rust/2020/02/07/compiler-team-meeting.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

The compiler team had our weekly triage meeting on 2020-02-06.
You can find the [minutes] on the [compiler-team repository].
Each week, we have general announcements from the team followed by check-ins from two of the compiler team working groups.

## Announcements

- There is a design meeting this Friday (2020-02-07) on [parser-librarification].

- [@davidtwco] has made progress on the polymorphization effort and has posted a [status update][polymorph_update].

- [@mw] has posted a major change proposal to make [incremental compilation respect the `-Ccodegen-units` setting][mw_change].

- Cargo report future-incompat [rfc#2834] is on track to get an FCP merge request from the cargo team in the near future.

## Working group sync

### [wg-rls-2.0]

- The Chalk dependency has been updated which dramatically reduced crashes.

- `cargo check` handling has been moved ino the server so all editors not just VS Code benefit.

- [@matklad] and [@nikomatsakis] have been working on an RFC to merge RLS and Rust Analyzer officially.

- Work is proceeding on "production readiness".
  - Binary releases are available on the project's GitHub page.
  - Planning to publish to VS Code extension marketplace "soon".

### [wg-self-profile]

- The ["Minimum Viable Product"][sp_mvp] has been completed!
  - Self-profiling is enabled for all perf.rust-lang.org runs and we automatically publish the data. ([Example][sp_example])

- [@mw] implemented query-key recording so queries can now be attributed to individual query invocations.

- The [`crox`] utility, which generates Chromium profiler compatible trace data, can now generate profiles for entire Cargo invocations via the `--dir` flag.
  - This allows inspecting various rustc processes and their individual threads on a common timeline with full query data.

- Quite a few people have tried `-Zself-profile` and have said they found it very useful!

[#67667]: https://github.com/rust-lang/rust/pull/67667
[#68530]: https://github.com/rust-lang/rust/pull/68530
[#68611]: https://github.com/rust-lang/rust/pull/68611
[@davidtwco]: https://github.com/davidtwco
[@matklad]: https://github.com/matklad
[@mw]: https://github.com/michaelwoerister
[@nikomatsakis]: https://github.com/nikomatsakis
[compiler-team repository]: https://github.com/rust-lang/compiler-team
[`crox`]: https://github.com/rust-lang/measureme/tree/master/crox
[minutes]: https://rust-lang.github.io/compiler-team/minutes/triage-meeting/2020-02-06/
[mw_change]: https://github.com/rust-lang/compiler-team/issues/245
[parser-librarification]: https://github.com/rust-lang/compiler-team/issues/237
[polymorph_update]: https://rust-lang.zulipchat.com/#narrow/stream/216091-t-compiler.2Fwg-polymorphization/topic/progress.20updates/near/187461928
[rfc#2834]: https://github.com/rust-lang/rfcs/issues/2834
[sp_example]: https://perf.rust-lang.org/detailed-query.html?commit=3761dcd3467441f78939ccb3b341b03b6a7558d7&base_commit=ac2f3fa41ac5ae8425b959f955bb7433b7c57aea&benchmark=await-call-tree-debug&run_name=clean
[sp_mvp]: https://github.com/rust-lang/rust/issues/58967
[wg-rls-2.0]: https://rust-lang.github.io/compiler-team/working-groups/rls-2.0/
[wg-self-profile]: https://rust-lang.github.io/compiler-team/working-groups/self-profile/
