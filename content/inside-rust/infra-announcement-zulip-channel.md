+++
path = "inside-rust/2025/10/02/infra-announcement-zulip-channel"
title = "Infrastructure announcement zulip channel"
authors = ["Jieyou Xu"]

[extra]
team = "the Infrastructure Team"
team_url = "https://rust-lang.org/governance/teams/infra/#team-infra"
+++

The [Infrastructure Team][infra-team] is working on improving communication for
project infrastructure changes that may impact project team members and [Dev
Desktop][dev-desktop] users.

We intend to create a new **low traffic** Zulip channel,
[`#infra/announcements`], dedicated to infrastructure change announcements.
Examples of announcement topics may include, but are not limited to:

- Commissioning/decommissioning of [Dev Desktops][dev-desktop]
- Significant CI changes
- Significant permission changes
- Significant prolonged outages of infrastructure services
- ... and other significant events which the Infrastructure Team anticipates may
  impact project members.

Transient outages will _not_ be included in this announcement channel to
maintain a high signal-to-noise ratio of this announcement channel.

We will subscribe:

- All project team members
- `gsoc-contributors`, `ospp-contributors` and `cloud-compute` marker team
  members

with registered Zulip IDs[^team-repo] to a dedicated [`#infra/announcements`
zulip channel][`#infra/announcements`].[^mute]

For project team members without registered Zulip IDs, you are encouraged to do
so by updating your `people/<handle>.toml` entry in the [rust-lang/team]
repository so that the Infrastructure Team can consistently announce project
infrastructure changes that may impact you. 


[^team-repo]: In the [rust-lang/team] repo. If your `people/<handle>.toml` entry
    does not already have a `zulip-id` field, you can send a PR against the
    [rust-lang/team] repo to update your `people/` entry with your Zulip ID.
[^mute]: Note that you can mute specific tops that do not concern you
    specifically. We adopted this "blanket" announce approach because in our
    experience, not reaching impacted project team members and [Dev
    Desktop][dev-desktop] user is significantly more problematic.


[infra-team]: https://rust-lang.org/governance/teams/infra/#team-infra
[dev-desktop]: https://forge.rust-lang.org/infra/docs/dev-desktop.html
[rust-lang/team]: https://github.com/rust-lang/team
[`#infra/announcements`]:
    https://rust-lang.zulipchat.com/#narrow/channel/533458-t-infra.2Fannouncements
