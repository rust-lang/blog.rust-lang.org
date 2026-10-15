+++
path = "inside-rust/2026/04/08/gpu-target-notification-group"
title = "Announcing the GPU notification group and Linker Working Area"
authors = ["The compiler team"]
description = "Announcing the GPU notification group and Linker Working Area"
aliases = ["inside-rust/2026/04/08/gpu-target-notification-group.html"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++

We are announcing two new groups in the compiler team:

* The GPU target notification group, for helping us to diagnose and resolve GPU-related issues
* The Linker working area, with expertise or interest in linkers

### About the GPU-target notification group

The GPU target notification group is made of experts from various vendors (AMD, Nvidia, Intel, etc.) and its goal is to better respond to issues and in general work in this area of expertise, for example [this one][rust-150452].

This group has an associated Zulip stream ([`#t-compiler/gpgpu-backend`]).

[rust-lang/team]: https://github.com/rust-lang/team
[`t-compiler/gpgpu-backend`]: https://rust-lang.zulipchat.com/#narrow/channel/422870-t-compiler.2Fgpgpu-backend
[rust-150452]: https://github.com/rust-lang/rust/pull/150452

### About the Linker working area

Creating of a linker group was discussed in [MCP#964][mcp#964] to kickstart work about integrating alternative linkers in the compiler, like [Wild][Wild]. As the second goal, this group can be a point of contact for handling questions and issues regarding linkers.

This group has an associated Zulip stream ([`#t-compiler/linker`]).

[`t-compiler/linker`]: https://rust-lang.zulipchat.com/#narrow/channel/585172-t-compiler.2Flinker
[mcp#964]: https://github.com/rust-lang/compiler-team/issues/964
[Wild]: https://github.com/wild-linker/wild/

### What is a "notification group" and a "working area group"?

Notification groups are made of people volunteering to keep an eye and respond to issues in a specific area they have expertise, they mostly help with maintenaince.

Working areas explore and do work focused on specific areas with the goal of adding features or improving the compiler.

Each group may have an associated Zulip stream where people can ask questions and discuss topics specific to that target.

Anyone can add their own name to these groups and be notified when an issue in that area is raised. If you are interested in participating, feel free to sign up for these groups. To do so, just open a PR against the [rust-lang/team] repository (here an [example][team-2341]).

For more info, see the documentation [about notification groups][forge-docs-notif-group] and compiler [working areas][forge-docs-work-area].

[forge-docs-notif-group]: https://forge.rust-lang.org/compiler/notification-groups.html
[forge-docs-work-area]: https://forge.rust-lang.org/compiler/notification-groups.html
[team-2341]: https://github.com/rust-lang/team/pull/2341
