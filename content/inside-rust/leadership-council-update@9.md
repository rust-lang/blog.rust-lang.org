+++
path = "inside-rust/2025/12/09/leadership-council-update"
title = "Leadership Council update — December 2025"
authors = ["Eric Huss"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council! We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2025/09/10/leadership-council-update/

## Accomplishments so far

### Welcome Jakub Beránek to the Council

As [announced in September](https://blog.rust-lang.org/inside-rust/2025/09/23/leadership-council-repr-selection/), [Jakub Beránek] joined the Council representing the Infrastructure team.

[Jakub Beránek]: https://github.com/Kobzol

### Foundation updates

In October we wrapped up the election process for the Project Directors of the Rust Foundation. [Jack Huey], [Niko Matsakis], and [David Wood] are our new Directors ([see announcement][pd-election]). We'd like to thank [Tomas Sedovic] for facilitating the election process.

[pd-election]: https://rustfoundation.org/media/introducing-the-rust-foundations-newest-project-directors-october-2025/
[Jack Huey]: https://github.com/jackh726/
[Niko Matsakis]: https://github.com/nikomatsakis/
[David Wood]: https://github.com/davidtwco/
[Tomas Sedovic]: https://github.com/tomassedovic

We've been discussing the 2026 "Project Priorities" budget with the Foundation, which would continue the program funded by Microsoft in 2024 and 2025. We're happy to report that we will be able to continue this program, which funds initiatives such as our travel budget, program management, compiler operations, and other maintenance support for the Rust project. The final budget will be submitted for board approval at the next meeting in December. We'll have more to share about this in the future. [#183](https://github.com/rust-lang/leadership-council/issues/183) [#234](https://github.com/rust-lang/leadership-council/issues/234)

We have engaged in extensive discussions regarding the Foundation's 3-year strategy, which is also scheduled for approval at the December board meeting. [#197](https://github.com/rust-lang/leadership-council/issues/197)

After monitoring travel budget spending and addressing concerns regarding 2025 expenditure levels, we have decided to maintain our current policy for the time being. [#182](https://github.com/rust-lang/leadership-council/issues/182)

We coordinated with the Foundation on the [announcement of the Rust Innovation Lab](https://blog.rust-lang.org/2025/09/03/welcoming-the-rust-innovation-lab/), and are excited to see this initiative move forward.

### Shape of Rust

We have implemented changes to the team structure to further clarify the status of teams in the Launching Pad. The following teams have been moved or archived:

| Team                          | Tracking Issue                                                | Outcome?                                  |
| :---                          | :---                                                          | :---                                      |
| Community Content Team        | [#129](https://github.com/rust-lang/leadership-council/issues/129)    | Archived                                  |
| Community Rustbridge Team     | [#132](https://github.com/rust-lang/leadership-council/issues/132)    | Archived                                  |
| Book Team                     | [#123](https://github.com/rust-lang/leadership-council/issues/123)    | Moved to lang-docs                        |
| Docker Team                   | [#124](https://github.com/rust-lang/leadership-council/issues/124)    | Moved to T-Infra                          |
| Async WG                      | [#137](https://github.com/rust-lang/leadership-council/issues/137)    | Moved to T-Lang                           |
| Rust By Example WG            | [#139](https://github.com/rust-lang/leadership-council/issues/139)    | Moved to lang-docs                        |

The following teams have confirmed that they will stay in the Launching Pad, which will likely be rechartered in the future to cover cross-cutting teams:

| Team                          | Tracking Issue                                                | Outcome?                                  |
| :---                          | :---                                                          | :---                                      |
| This Week In Rust             | [#125](https://github.com/rust-lang/leadership-council/issues/125)    | Stay in Launching Pad                     |
| Community Survey Team         | [#133](https://github.com/rust-lang/leadership-council/issues/133)    | Renamed, Stay in Launching Pad            |
| Security Response WG          | [#141](https://github.com/rust-lang/leadership-council/issues/141)    | Renamed, Stay in Launching Pad            |
| Triage WG                     | [#142](https://github.com/rust-lang/leadership-council/issues/142)    | Renamed, Stay in Launching Pad            |


Progress has been made with other Launching Pad teams, though these efforts have not yet concluded. The following teams are still under discussion:

| Team                          | Tracking Issue                                                |
| :---                          | :---                                                          |
| Secure Code Working Group     | [#140](https://github.com/rust-lang/leadership-council/issues/140) |
| Embedded Working Group        | [#127](https://github.com/rust-lang/leadership-council/issues/127) |
| CLI Working Group             | [#138](https://github.com/rust-lang/leadership-council/issues/138) |
| Gamedev Working Group         | [#126](https://github.com/rust-lang/leadership-council/issues/126) |

[James Munns] submitted [a proposal](https://github.com/rust-lang/leadership-council/issues/159#issuecomment-3237633417) detailing the vision and operational structure of the Rust Society.

[James Munns]: https://github.com/jamesmunns

### Additional items

Other updates include:

- We concluded a contributor survey primarily focused on understanding the balance between funded and volunteer-based contributions within the Rust Project. The results will help inform how we can better support maintainers. [#222](https://github.com/rust-lang/leadership-council/issues/222)
- We approved a requirement for all Project members to have a Zulip account (in addition to a GitHub account) to ensure more reliable communication. [#228](https://github.com/rust-lang/leadership-council/issues/228)
- We agreed to extend an invitation to the Rust for Linux contributors for the 2026 All-Hands. [#236](https://github.com/rust-lang/leadership-council/issues/236)
- In response to a query regarding responsibility for stable versus unstable support in stable releases, we clarified that this generally falls under the Lang team's purview, with the Compiler team primarily responsible for implementation. [#180](https://github.com/rust-lang/leadership-council/issues/180)
- We agreed to delegate decisions regarding GSoC mentor rewards to the Mentorship team. [#232](https://github.com/rust-lang/leadership-council/issues/232)
- We considered a request to establish a Project Representative position but decided against it for the time being. [#226](https://github.com/rust-lang/leadership-council/issues/226)

## Ongoing work

Several projects have seen significant discussion since our last update but have not yet reached a conclusion.

### Foundation

We are excited about the development of the [Rust Foundation Maintainers Fund](https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/). We are working closely with the Foundation to guide how this program will fund Rust Project maintainers. A working group is forming to define the usage of these funds ([#248](https://github.com/rust-lang/leadership-council/pull/248)). We expect to have an approved plan and policy in place in January 2026. [#242](https://github.com/rust-lang/leadership-council/issues/242)

David Wood has proposed a clarification of Project Director responsibilities, which we hope will improve understanding and communication between the Council, the Directors, and the Foundation. [#250](https://github.com/rust-lang/leadership-council/pull/250)

### Additional items

- We are following up on a request to add Graydon Hoare to the alumni section of the website.
[#231](https://github.com/rust-lang/leadership-council/issues/231)
- Regarding a request to link the community Discord from our website, we have deferred the decision to the website management team. [#219](https://github.com/rust-lang/leadership-council/issues/219)
- We received invitations to apply for funding from the Rust Ecosystem Fund. [#240](https://github.com/rust-lang/leadership-council/issues/240)

## Following our work

As you can see from the many links above, the work of the Council happens mostly in public on GitHub issues posted in [our repository](https://github.com/rust-lang/leadership-council/issues). The items on our meeting agendas are drawn from these. After discussing an item, we summarize that discussion and any shared [rationales](https://aturon.github.io/tech/2018/05/25/listening-part-1/) on the issue. When we make decisions, we propose on the issue a "final comment period" (FCP), and as with all FCPs in the Project, we're interested in and review any feedback that people have before or during this final comment period.

To follow our work in real time, watch our repository. You can also see meeting summaries posted on Zulip in [`#council > Meeting minutes & summaries`][summaries].

[summaries]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Meeting.20minutes.20.26.20summaries/with/561198432

## Meeting minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes]. Links to the minutes since our last update are:

* [August 29, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-08-29.md)
* [September 12, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-09-12.md)
* [September 26, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-09-26.md)
* [October 10, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-10-10.md)
* [October 24, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-10-24.md)
* [November 7, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-11-07.md)
* [November 14, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-11-14.md)
* [November 21, 2025](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2025-11-21.md)

[minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes
