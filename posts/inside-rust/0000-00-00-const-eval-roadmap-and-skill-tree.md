# Const eval roadmap

Y'all probably noticed that const eval finally got loops, mutable local variables and branches. With those features existing, we were able to make a lot of basic arithmetic functions `const fn `. Things like `checked_div` needs to be able to check that the divisor is not zero and return `None` in that case. These features have been asked for a lot, and while it may not appear so, these were the low hanging fruit.

So you may now be thinking "If the low hanging fruit took you 5 years" *(yes oli-obk has been working on const eval for that long, and in the end ecstaticmorse implemented const control flow)*, "will we see any of the more fancy features in this new decade?".

TLDR: Yes, and we now have a system for making things go fast (brrr?) while still making sure we don't accidentally bypass the decision processes.

Instead of creating more and more [RFCs](https://github.com/rust-lang/rfcs/) or [MCPs](https://github.com/rust-lang/lang-team/) and DDOSing the language team, we made one roadmap that doesn't actually have the details on each feature. The plan is a tree-graph that lists the dependencies between features. The language team then signs off on the roadmap, giving the const-eval team the permission to experiment with unstable features without checking back with the language team. Stabilization of any of the features will still work by having a language team sign off, but that can then be done with the full knowledge about the final product instead of hypotheticals. If new features that are not yet on the plan are desired to get implemented, the roadmap is adjusted and the language team signs off on the roadmap changes.

This new scheme makes it much easier to keep an overview over how features interact with other features. So without further adue (and hoping that the previous sentence's statement is true), let's introduce the

[Const. Eval. Skill. Tree.](https://rust-lang.github.io/const-eval/)

All the nodes are hyperlinks, pointing to the tracking issue for a specific feature. The edges turn bold and pink when you hover over them, making it easy to track which nodes it connects. If you want to see the source that generates this graph, you can find it at [in the const-eval team repository](https://github.com/rust-lang/const-eval/blob/master/src/skill_tree.md).

Nodes further on the right are higher level features depending on features whose nodes are further left. Arrows point from low level features to high level features. So an Arrow is basically pointing from a feature to the features that are enabled by it.

Looking at the current state of the roadmap, you can see that there are three major low level features:

* trait bounds on `const fn`
    * is being worked on, and a preliminary subset of it has been implemented. 
* mutable references
    * is being worked on, basically works, we just need to hammer out some kinks
* unsafe code
    * RalfJung is in the process of writing up a document explaining how we can reliably detect most undefined behaviour in const eval code (and the kind of UB we cannot detect).

So, as you can see, things are moving along nicely and we'll likely be able to give you new const eval features to play on nightly with soon.