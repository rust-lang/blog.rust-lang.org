---
layout: post
title: "Rustc Trait System Refactor Initiative Update: Stabilizing `-Znext-solver=coherence`"
author: lcnr
team: The Rustc Trait System Refactor Initiative <https://github.com/rust-lang/trait-system-refactor-initiative/>
---

It's been half a year since we last summarized our progress in the [Types Team update blog post](https://blog.rust-lang.org/2024/06/26/types-team-update.html). With the next-generation trait solver now getting used by default in coherence checking on beta[^2], it's time for another update. The next-generation trait solver is intended to fully replace the existing type system components responsible for proving trait bounds, normalizing associated types, and much more. This should fix many long-standing (soundness) bugs, enable future type system improvements, and improve compile-times. See [this previous blog post](https://blog.rust-lang.org/inside-rust/2023/07/17/trait-system-refactor-initiative.html) for more details.

[^2]: Which will get stabilized as version 1.84.

The FCP to stabilize the use of the new solver in coherence already passed in April. Checking auto-trait bounds of deeply recursive types unfortunately caused exponential blowup with the new implementation, resulting in a hang caught by CI. Fixing this hang required us to add a [complex caching optimization](https://github.com/rust-lang/rust/pull/128828) to efficiently handle cycles while still being correct[^1]. We  then merged the stabilization PR only to revert it 5 days later as it resulted in [a separate hang in `nalgebra`](https://github.com/rust-lang/rust/issues/130056). Luckily the fix this time was a lot simpler, which allowed us to [stabilize `-Znext-solver=coherence` again](https://github.com/rust-lang/rust/pull/130654) soon after.

[^1]: The old implementation avoids these hangs by [incorrectly reusing cache-entries even if they should not be applicable](https://github.com/rust-lang/trait-system-refactor-initiative/issues/119). Due to subtle reasons™ we believe this to not be an exploitable soundness issue.

While the new solver will only be used in coherence checking for now, this already removes some inconsistencies and theoretical soundness issues, see [the stabilization report](https://github.com/rust-lang/rust/pull/130654) for a complete summary. In case you encounter any weird hangs, unexpected overlap errors, or unhelpful diagnostics, please open a [GitHub issue](https://github.com/rust-lang/rust/issues).

We then moved our focus to [bootstrap using exclusively the new solver](https://github.com/rust-lang/rust/pull/133502) again. We've made significant progress here and are able to fully compile `rustc` and `cargo`, [allowing `try`-builds to succeed](https://github.com/rust-lang/rust/pull/133502#issuecomment-2505183120). We are currently looking into the [remaining compile-time benchmark failures](https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor/topic/help.20getting.20some.20rough.20perf.20numbers/near/485121929). We intend to start to optimize the new implementation and to test more and more crates until a full [`crater` run](https://github.com/rust-lang/crater) will be useful.

### Looking ahead

To unblock the stabilization of the next-generation trait solver we're required to fix [multiple](https://github.com/rust-lang/rust/pull/133501) [issues](https://rust.godbolt.org/z/aoseYzMx9) which also affect the current implementation. This is due to subtle differences between the implementations causing the new solver to trigger bugs which are avoided by the old solver. The cases where the new implementation works correctly while the old one does not are unfortunately far less relevant as nobody has written code while relies on the behavior of the next-generation trait solver yet.

We will also have to spend a significant effort towards improving error messages when using the new trait solver. There are still many cases where they are significantly worse than the status quo.

We are still working towards our first crater run and are aware of multiple significant issues. We expect to resolve most of them during the next few months. So while we're not quite at the point where bug reports are useful, we intend to explicitly ask for testing of `-Znext-solver=globally` soon. To follow our progress, check out the [`-Znext-solver` tracking issue](https://github.com/rust-lang/rust/issues/107374) or join us [on zulip](https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor).
