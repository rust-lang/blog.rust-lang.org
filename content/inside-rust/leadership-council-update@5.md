+++
path = "inside-rust/2024/12/09/leadership-council-update"
title = "December 2024 Leadership Council Update"
authors = ["Eric Huss"]
aliases = ["inside-rust/2024/12/09/leadership-council-update.html"]

[extra]
team = "Leadership Council"
team_url = "https://www.rust-lang.org/governance/teams/leadership-council"
+++

Hello again from the Rust Leadership Council!
We wanted to share an update on what the Council has been working on since [our last update][update].

[update]: https://blog.rust-lang.org/inside-rust/2024/09/06/leadership-council-update.html

## Accomplishments so far

### Welcome Oli Scherer and TC to the Council

As [announced in September](https://blog.rust-lang.org/inside-rust/2024/09/27/leadership-council-repr-selection.html), Oli Scherer and TC joined the Council representing the moderation team and lang team respectively.

### Foundation updates

In November we wrapped up the election process for the Project Directors of the Rust Foundation. Ryan Levick is continuing for another term with Carol Nichols joining the board ([see announcement][pd-election]). We'd like to thank TC for facilitating the election process.

We tried to further clarify the definition of employment which is used for determining Director eligibility ([rust-lang/leadership-council#111]).

We reviewed our processes for how the Council interacts with the Directors and the Foundation. We identified some concerns about maintaining good communication with the Directors since Mark Rousskov, the only Council member who was also a Director, is stepping down as Director. We also looked at our communication processes with the Foundation, and didn't have specific changes to recommend other than documenting the current status ([rust-lang/leadership-council#41]).

In November we also coordinated with the directors on a project-wide request for the annual feedback on the Foundation executive director.

[pd-election]: https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-newest-project-director-carol-nichols/
[rust-lang/leadership-council#111]: https://github.com/rust-lang/leadership-council/pull/111
[rust-lang/leadership-council#41]: https://github.com/rust-lang/leadership-council/issues/41

## Travel grant policy

In April we started a Travel Grant program which provides funds for project members to travel to Rust-related events. We have followed up on that program to publicly [document the policy][travel-grant-process]. We are also looking to [remove the experimental status][travel-grant-experiment] of the policy, as we feel like we have been happy with how the program has been working, and expect to keep it funded in the near term.

We'd like to remind all project members that they are welcome to take advantage of this program, and to reach out to your representative if you have any questions.

[travel-grant-process]: https://github.com/rust-lang/leadership-council/blob/main/policies/spending/travel.md
[travel-grant-experiment]: https://github.com/rust-lang/leadership-council/pull/122

## Compiler operations

We approved funding for a compiler operations position ([rust-lang/leadership-council#114]). This role is intended to help ensure the effectiveness of the compiler team. The specifics are being coordinated between the compiler team leads and the Foundation.

[rust-lang/leadership-council#114]: https://github.com/rust-lang/leadership-council/issues/114

## Project Grants

We approved the use of a portion of the Council's budget to extend one-year grants to several of the candidates of the [Fellowship Program] ([rust-lang/leadership-council#112]). We had more excellent applications to the program than the existing funding would cover, and we had the opportunity to divert some of the Council's budget toward this program.

[Fellowship Program]: https://foundation.rust-lang.org/grants/fellowships/
[rust-lang/leadership-council#112]: https://github.com/rust-lang/leadership-council/issues/112

### Additional items

And a few other items:

- Approved distribution of GSoC award funds ([rust-lang/leadership-council#116]).
- Set up a document to help us track the [Council's budget][budget-tracker].
- Approved a Foundation board meeting substitute for the December board meeting ([rust-lang/leadership-council#117]).
- Various updates to the Council's own meeting process:
    - Enacted some improvements for how meeting minutes are approved to help streamline the process while still allowing members time to review ([notes][minutes-review]).
    - Switched to an issue-oriented process for managing the Council's meeting agenda.
    - Decided to stop video recordings of Council meetings ([rust-lang/leadership-council#102]).
- Eric Holk gave a [presentation at the Ubuntu Summit][ubuntu] about the Rust Project.
- Approved the [gccrs blog post].

[rust-lang/leadership-council#116]: https://github.com/rust-lang/leadership-council/issues/116
[budget-tracker]: https://hackmd.io/@rust-leadership-council/ryBmBnFCC
[rust-lang/leadership-council#117]: https://github.com/rust-lang/leadership-council/issues/117
[minutes-review]: https://github.com/rust-lang/leadership-council/blob/687946a596e65b8f6fd524bcc0afa4ab497581c5/minutes/sync-meeting/2024-11-22.md#review-of-minutes-process
[ubuntu]: https://www.youtube.com/live/ZNK4aSv-krI?t=528s
[gccrs blog post]: https://blog.rust-lang.org/2024/11/07/gccrs-an-alternative-compiler-for-rust.html
[rust-lang/leadership-council#102]: https://github.com/rust-lang/leadership-council/pull/102
[rust-lang/leadership-council#103]: https://github.com/rust-lang/leadership-council/issues/103

## Ongoing work

There are various efforts underway on projects that have had significant discussions since the last update, but have not concluded with any decisions, yet.
They are:

### Shape of Rust

Work continues to define the shape of the structure of the Rust project ([rust-lang/leadership-council#33]) and to determine how to organize the teams in the [Launching Pad]. James Munns has been working to better understand the current status of the Launching Pad teams, and to work on plans for what to do with those teams ([rust-lang/leadership-council#118]).

[rust-lang/leadership-council#33]: https://github.com/rust-lang/leadership-council/issues/33
[Launching Pad]: https://forge.rust-lang.org/governance/council.html#the-launching-pad-top-level-team
[rust-lang/leadership-council#118]: https://github.com/rust-lang/leadership-council/issues/118

### Foundation budget

We are continuously looking for ways to effectively use the Council's budget ([rust-lang/leadership-council#103]). Although we have approved several programs, we still have a significant portion of the budget to spend. James Munns [suggested an RFQ process][rfq] for soliciting quotes on potential projects.

### License and copyright

We continue to look at how we can tackle the backlog of issues and concerns regarding the licensing and copyright handling throughout the Rust project ([rust-lang/leadership-council#24]). We [opened a topic][license-topic] to try to get more input on different approaches we can take.

Jonathan Pallant has been working on improving the copyright notices shipped with Rust ([rust-lang/leadership-council#120]), and has asked the Council to help approve this change.

[license-topic]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/license.2Fcopyright.20support
[rust-lang/leadership-council#24]: https://github.com/rust-lang/leadership-council/issues/24
[rust-lang/leadership-council#120]: https://github.com/rust-lang/leadership-council/issues/120

### Additional items

- Work continues to prepare for the all-hands event in May 2025 at [RustWeek 2025] which corresponds with Rust's 10-year anniversary.
- We have been looking to change the policy for meeting observers ([rust-lang/leadership-council#110]).
- We have been asked to review the obligation for the Council and the Project as a whole for approval of [The Update Framework RFC], but no conclusion has been reached.
- We have been reviewing the [open tasks] for the Council and trying to identify what is needed to move them forward.
- Niko Matsakis has reached out to the Council for [establishing a 3-5 year plan for the project][plan], which is in the initial stages of discussion.
- We have been working on a coordinated analysis of the [inference breakage] that happened in the Rust 1.80 release, since the issue involved many teams throughout the project.

[RustWeek 2025]: https://rustweek.org/
[rust-lang/leadership-council#110]: https://github.com/rust-lang/leadership-council/pull/110
[The Update Framework RFC]: https://github.com/rust-lang/rfcs/pull/3724
[open tasks]: https://github.com/rust-lang/leadership-council/issues
[plan]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/A.20Rust.203-5.20YP/near/480932967
[inference breakage]: https://github.com/rust-lang/rust/issues/127343
[rfq]: https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/Discussion.20regarding.20a.20Council.20RFQ.20process/near/476426031

## Meeting minutes

We publish minutes from all Council meetings to the [Leadership Council repo][minutes].
Links to the minutes since our last update are:

* [September 27, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-09-27.md)
* [October 11, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-10-11.md)
* [October 25, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-10-25.md)
* [November 8, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-11-08.md)
* [November 22, 2024](https://github.com/rust-lang/leadership-council/blob/main/minutes/sync-meeting/2024-11-22.md)

[minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes
