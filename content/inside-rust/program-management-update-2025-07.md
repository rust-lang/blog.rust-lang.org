+++
path = "inside-rust/9999/12/31/program-management-update-2025-07"
title = "Program management update — July 2025"
authors = ["Tomas Sedovic"]

[extra]
team = "Edition & Goals teams"
team_url = "https://www.rust-lang.org/governance/teams/"
+++

Welcome to the second Rust PM update!

Things were a bit busy in the personal life. We had to temporarily move to a smaller apartment which was quite time-consuming and energy-draining. And after that, I took a week off and we spent it in the mountains.

So things were a bit slower in July. In addition to meetings, what clearly dominated was the Project Goals work.

## Wrapping up 2025 H1 goals

As we entered the second half of the year, it was time to wrap up the current Project Goals and get the new ones going. In the spirit of helping people focus on other things, I took this over.

First, I've sent a couple requests for goal updates. This lead me to noticing and fixing an issue with the triagebot. See [the `ping-goals` section](#ping-goals-retry-issue).

After people provided their final messages, I've published the final 2025H1 Project Goals update:

https://example.com/july-project-goals-update/

(**TODO** write and publish the July final goal update)

With that done, I've closed most of the tracking issues. Some goals are being renewed for the H2 period so we're keeping those open.

(**TODO**: again, make sure we've actually closed the issues before publishing the blog.)

## Starting 2025 H2 goals

With all of that out of the way, Niko published the [call for 2025 H2 goal proposals][2025H2CFP].

[2025H2CFP]: /inside-rust/2025/06/23/project-goals-2025h2-call-for-submissions/

Based on the feedback we've received, we've made some changes:

* We're getting the proposals in front of the Teams and their leads earlier (well before the final RFC is written) so they are aware and have input into what's being proposed.
* We're asking them to suggest what they'd like to see as a flagship goal before the final selection is made.
* For each proposed goal, we've asked each team with asks mentioned in the goal to select a "champion" who commits to making sure the goal is not blocked by that team.

Once all the champions are selected, we'll meet with the team leads, go over the goals and figure out which ones we want to accept.

Similarly, we'll take their input regarding flagship goals.


As the goals were being proposed, Niko, Rémy and I looked at the PRs, fixed any CI issues and merged them.

For each team, I've opened a call for champions Zulip thread listing goals that requested their support. [Here's an example for the Lang team](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/2025H2.20Goal.20Review).

As with any new process, people had questions. I was there to help clarify we're asking for, the overall plan and next steps. When there were specific questions for a goal, I made sure to connect the right people.

### Rust for Linux

Continuing the Rust for Linux effort, I'll be the point of contact for the two goals they're proposed ([compiler][rfl-compiler] and [language][rfl-lang] features for getting Rust for Linux into stable Rust).

[rfl-compiler]: https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-compiler.html
[rfl-lang]: https://rust-lang.github.io/rust-project-goals/2025h2/Rust-for-Linux-language.html

I will set up the agenda, run the meetings and keep the tracking issues up-to-date.

## Misc

### `ping-goals` retry issue

The request for updates is done by the `ping-goals` command of the @triagebot. This returned an error ("Failed to await at this time: connection closed before message completed"), but did actually ping everyone.

Multiple time, in fact:

https://rust-lang.zulipchat.com/#narrow/channel/435869-project-goals/topic/Prepare.20const.20traits.20for.20stabilization.20.28goals.23106.29

As you can see in the thread, this wasn't a new behavior either. Precisely the sort of thing I can and should look for improving:

https://github.com/rust-lang/triagebot/issues/2108

Urgau https://github.com/Urgau looked into it and found out this is because rustbot was sending the pings one-by-one and taking some time to finish.

Zulip interpreted this as a timeout and retried the requests a few times.

Here's the fix he opened: https://github.com/rust-lang/triagebot/pull/2109.

Thank you Urgau!

### Leadership Council Minutes

Having taken over the minutes for the Leadership Council meetings in June, I've now closed the loop by cleaning them up and publishing them in [the leadership-council repository][leadership-council-minutes].

[leadership-council-minutes]: https://github.com/rust-lang/leadership-council/tree/main/minutes/sync-meeting

### Meeting Minutes Upload

uploaded all the other project meeting notes I took (where relevant) TODO


## Stats

meetings attended:

word count for minutes

note the dubiousness of these stats and that they're not a target metric.

NOTE: count total minuted words. (note: includes people names but I did type those too):

	$ grep --no-filename '^\w\+: ' -r . | wc --words -
77269

WHOA: that's almost the length of a novel! (what am I doing with my life‽)


## NOTE: July stats

* cargo
  * 2025-07-29: 1531
  * 2025-07-22: 1335
  * 2025-07-15: 600 (I was absent then)
  * 2025-07-08: 1919
  * 2025-07-01: 1616
* lang triage:
  * 2025-07-02: 2023
  * 2025-07-09: 2815
* libs (api and mcp)
  * 2025-07-01: 5774
  * 2025-07-08: 5783
  * 2025-07-15: 705
  * 2025-07-22: 4981
* spec
* leadership council
  * 2025-07-04: 1792
  * 2025-07-18: 2228
* total across the board
$ grep --no-filename '^\w\+: ' -r 2025-07-* | wc --words -
42680


### NOTE: June stats



* cargo
  * 2025-06-24: 1316
  * 2025-06-17: 1362
  * 2025-06-10: 1470
  * 2025-06-03: 925
* lang
  * 2025-06-04: 1717
  * 2025-06-11: 2251
  * 2025-06-18: 2028
  * 2025-06-25: 1597
* libs (api and mcp)
  * 2025-06-10: 5098
  * 2025-06-03: 4412
* spec
* leadership council
  * 2025-06-20: 1787
  * 2025-06-06: 2073
* total across the board
$ grep --no-filename '^\w\+: ' -r 2025-06-* | wc --words -
34589

### For comparison, some May stats

* cargo
  * 2025-05-27: 786
  * 2025-05-20: 891
  * 2025-05-13: 0 (skipped for RustWeek)
  * 2025-05-06: 936
  * 2025-04-29: 816
* lang triage
  * 2025-05-21: 990
  * 2025-05-07: 3145
  * 2025-04-30: 1000
  * 2025-04-23: 507
* libs (api and mcp)
  * 2025-05-27: 1341
* spec
* leadership council
  * 2025-05-23: 1263
  * 2025-05-09: 1344
  * 2025-04-25: 2023
  * 2025-04-11: 1256


TODO: pick up some small wins? (publish meeting notes, reach out to folks)
