+++
layout = "post"
date = 2023-07-01
title = "Rustfmt support for let-else statements"
author = "Caleb Cartwright"
team = "the style team <https://www.rust-lang.org/governance/teams/lang#Style%20team> and the rustfmt team <https://www.rust-lang.org/governance/teams/dev-tools#Rustfmt%20team>"
+++

Rustfmt will add support for formatting [let-else statements] starting with the nightly 2023-07-02 toolchain, and then let-else formatting support should come to stable Rust as part of the 1.72 release.

## Overview

let-else statements were [stabilized][let-else stabilization] back in 2022 as part of the 1.65.0 release. However, the current and previous versions of Rustfmt did not have formatting support for let-else statements. When Rustfmt encountered a let-else statement it would leave it alone and maintain the manual styling originally authored by the developer.

After updating to one of the toolchains with let-else formatting support, you may notice that `cargo fmt`/`rustfmt` invocations want to "change" the formatting of your let-else statements. However, this isn't actually a "change" in formatting, but instead is simply Rustfmt applying the [let-else formatting rules] for the very first time.

Rustfmt support for let-else statements has been a long standing request, and the Project has taken a number of steps to prevent a recurrence of the delay between feature stabilization and formatting support, as well as putting additional procedures in place which should enable more expeditious formatting support for nightly-only syntax.

## Background and Context

Rust has an official [Style Guide] that articulates the default formatting style for Rust code. The Style Guide functions as a specification that defines the default formatting behavior for Rustfmt, and Rustfmt's primary mission is to provide automated formatting capabilities based around that Style Guide specification. Rustfmt is a direct consumer of the Style Guide, but Rustfmt does not unilaterally dictate what the default formatting style of language constructs should be.

The initial Style Guide was developed many years ago (beginning in 2016), and was driven by a Style Team in collaboration with the community through an RFC process. The Style Guide was then made official in 2018 via [RFC 2436].

That initial Style Team was more akin to a Project Working Group in today's terms, as they had a fixed scope with a main goal to simply pull together the initial Style Guide. Accordingly that initial Style Team was disbanded once the Guide was made official.

There was subsequently no designated group within the Rust Project that was 
explicitly responsible for the Style Guide, and no group explicitly focused on determining the official Style for new language constructs.

The absence of a team/group with ownership of the Style Guide didn't really cause problems at first, as the new syntax that came along during the first few years was comparatively non-controversial when it came to default style and formatting. However, over time challenges started to develop when there was increasingly less community consensus and no governing team within the Project to make the final decision about how new language syntax should be styled.

This was certainly the case with let-else statements, with lots of varying perspectives on how they should be styled. Without any team/group to make the decision and update the Style Guide with the official rules for let-else statements, Rustfmt was blocked and was unable to proceed.

These circumstances around let-else statements resulted in a greater understanding across the Project of the need to establish a team to own and maintain the Style Guide. However, it was also well understood that spinning up a new team and respective processes would take some time, and the decision was made to not block the stabilization of features that were otherwise fully ready to be stabilized, like let-else statements, in the nascency of such a new team and new processes.

Accordingly, let-else statements were stabilized and released without formatting support and with an understanding that the new Style Team and then subsequently the Rustfmt Team would later complete the requisite work required to incorporate formatting support.

## Steps Taken

A number of steps have been taken to improve matters in this space. This includes steps to address the aforementioned issues and deal with some of the "style debt" that accrued over the years in the absence of a Style Team, and also to establish new processes and mechanisms to bring about other formatting/styling improvements.

* [Launched a new, permanent Style Team][style-team-rfc] that's responsible for the Style Guide.
* Established a mechanism to evolve the default style while still maintaining stability guarantees ([RFC 3338][style-edition-rfc]).
* Developed a [nightly-syntax-policy] that provides clarity around style rules for unstable/nightly-only syntax, and enables Rustfmt to provide earlier support for such syntax.

Furthermore, the Style Team is also continuing to diligently work through the backlog of those "style debt" items, and the Rustfmt team is in turn actively working on respective formatting implementation. The Rustfmt team is also focused on growing the team in order to improve contributor and review capacity. 

## Conclusion

We know that many have wanted let-else formatting support for a while, and we're sorry it's taken this long. We also recognize that Rustfmt now starting to format let-else statements may cause some formatting churn, and that's a highly undesirable scenario we strive to avoid.

However, we believe the benefits of delivering let-else formatting support outweigh those drawbacks. While it's possible there may be another future case or two where we have to do something similar as we work through the style backlog, we're hopeful that over time this new team and these new processes will reduce (or eliminate) the possibility of a recurrence by addressing the historical problems that played such an outsize role in the let-else delay, and also bring about various other improvements.
 

Both the Style and Rustfmt teams hang out on Zulip so if you'd like to get more involved or have any questions please drop by on [T-Style][style-zulip] and/or [T-Rustfmt][rustfmt-zulip].

[let-else statements]: https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html
[let-else stabilization]: https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#let-else-statements
[let-else formatting rules]: https://doc.rust-lang.org/nightly/style-guide/statements.html#else-blocks-let-else-statements
[style guide]: https://doc.rust-lang.org/nightly/style-guide/
[RFC 2436]: https://rust-lang.github.io/rfcs/2436-style-guide.html
[style-team-rfc]: https://rust-lang.github.io/rfcs/3309-style-team.html
[style-edition-rfc]: https://rust-lang.github.io/rfcs/3338-style-evolution.html
[nightly-syntax-policy]: https://github.com/rust-lang/style-team/blob/468570a02856a6bbe3994164e1a16a13b56b5cf4/nightly-style-procedure.md
[style-zulip]: https://rust-lang.zulipchat.com/#narrow/stream/346005-t-style
[rustfmt-zulip]: https://rust-lang.zulipchat.com/#narrow/stream/357797-t-rustfmt
