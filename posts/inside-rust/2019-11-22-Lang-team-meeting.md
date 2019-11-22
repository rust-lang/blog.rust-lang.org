---
layout: post
title: "2019-11-14 and 2019-11-21 Lang Team Triage Meetings"
author: Niko Matsakis
description: "2019-11-14 and 2019-11-21 Lang Team Triage Meetings"
team: the lang team <https://www.rust-lang.org/governance/teams/lang>
---

Since I apparently forgot to post a blog post last week, this blog
post covers two lang-team triage meetings: [2019-11-14] and
[2019-11-21]. Recordings from [both] [meetings] are also available.

[lang-team]: https://github.com/rust-lang/lang-team/
[both]: https://youtu.be/0exyVhBmDW0
[meetings]: https://youtu.be/X2z3CoV0OUM

## Updates on shepherded items

Here is a list of the ["shepherded items"] that the lang team is
tracking, along with weekly updates on the latest developments.

["shepherded items"]: http://smallcultfollowing.com/babysteps/blog/2019/09/11/aic-shepherds-3-0/

* [const-eval](https://github.com/rust-lang/const-eval)
    - there is progress towards extending the set of expressions
      permitted in constants to include `&mut` borrows, if/match, and
      loops.
* [project-ffi-unwind](https://github.com/rust-lang/project-ffi-unwind)
    - current status: currently evaluating whether "C" functions should 
      permit unwinding by default
    - trying to get measurements of the impact on code size
    - prototyped the plan in a rustc branch, but needs a few updates and to be executed
      on a representative code base (likely Fuschia)
* [Coherence can be bypassed by an indirect impl for a trait object](https://github.com/rust-lang/rust/issues/57893)
    - did a [crater run of the proposal](https://github.com/rust-lang/rust/pull/66037#issuecomment-549575983) but have only partially analyzed the impact
* grammar working group — qmx
    - no updates this week
* `!` type and fallback — centril
    * [`!` is stable on nightly!](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=a16420f41bbd2496ed07c75cf048189e)

## About the lang-team meetings

The lang-team triage meetings are held weekly on Zoom (see our
[meeting calendar]). Anyone is welcome to come and observe. We make a
"best effort" to record the meetings though technical issues sometimes
intervene.

[meeting calendar]: https://github.com/rust-lang/lang-team/#meeting-calendar
