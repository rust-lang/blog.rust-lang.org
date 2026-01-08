+++
path = "inside-rust/2026/01/12/what-is-maintenance-anyway"
title = "What is maintenance, anyway?"
authors = ["Jakub Beránek"]

[extra]
team = "the Rust Foundation Maintainer Fund Design Committee"
team_url = "https://github.com/rust-lang/leadership-council/blob/main/committees/maintainer-fund-design-group.md"
+++

Recently, the Rust Foundation has announced the [Rust Foundation Maintainer Fund][rfmf], whose goal is to support Rust Project maintainers. We are currently cooperating with the Foundation to define the [guidelines][rfmf-doc] of this Fund, such as what kind of work to fund, how to select maintainers to fund and others. One of the questions that keeps coming up is this: *Who exactly is a maintainer, and what work can be considered as being maintenance?* This post provides some observations that might help answer these questions.

Wikipedia defines [software maintenance](https://en.wikipedia.org/wiki/Software_maintenance) as "modification of software after delivery", but that does not correspond very well to how maintenance works in open-source or in Rust. There is no single delivery of Rust after which it would switch to "maintenance mode"; we deliver a new nightly version of Rust every day, and a new stable version every six weeks. And if someone contributes a pull request to the compiler, they clearly modify it, but as much as we treasure contributions to Rust, that does not immediately make them a maintainer. Instead, let us try to describe how we understand maintenance in the Rust Project.

## Keeping the lights on

We could interpret the word "maintain" literally, in the sense of keeping something in a specific (hopefully working) state over a long time period. And that is indeed a large part of what maintainers do: they ensure that things that work today will still continue working tomorrow. There are many ways in which a software project might stop working properly, so this work is naturally very diverse. It includes activities such as issue triaging, investigating and fixing bugs, dealing with CI failures, responding to security incidents, resolving performance regressions, updating dependencies, making sure that documentation is up to date, and many others.

This kind of "janitorial" work is quite challenging for several reasons:

- It can be difficult to demonstrate the positive effects of this work on a given software project. Users sometimes assume that if something works now, it will keep working indefinitely. Furthermore, Rust has a very high bar for being reliable (in many aspects of that word), so people are used to the fact that the compiler works correctly, that critical bugs are resolved as quickly as possible, that new releases are published every six weeks, etc. But all that takes considerable effort, even without considering shipping any new features. Therefore, a successful report of maintenance work can be simply "things still work as they used to", and consistent maintenance over time manifests as an *absence* of noteworthy incidents.
- It can be very unpredictable, because it is hard to estimate in advance what things will break next. This also makes it difficult to plan this work ahead and make promises about what kind of work will get done in a given time period.
- Finding motivation for performing a lot of maintenance work can be a struggle. It is often not work that someone *wants* to do, but work that *has to be done*. That can be draining, especially if that work is done by volunteers.
- Maintenance is often not on a roadmap, and often not viewed as high-status work that gets someone recognized or promoted.

## Enabling evolution

The maintenance activities described above are required to keep a software project working. However, usually that's not *all* we need. Most software projects need to *evolve* over time, and Rust is no exception; this usually means adding new features and making improvements. Improving the language, compiler, standard library, tooling, etc. might unblock some Rust users, which is great. But what might not be so obvious is that we often need to unblock the contributors who implement these new features in the first place!

Before someone even starts to implement a new feature, they often want to get a vibe check from someone else knowledgeable with the corresponding part of the codebase, or advice on how best to approach the implementation. Furthermore, certain features require complex refactorings to be performed before they can be implemented. Then, once a pull request with a new feature is submitted, someone has to review it. And even once a pull request lands, it may introduce new [technical debt](https://en.wikipedia.org/wiki/Technical_debt), which will eventually need cleaning up.

However, it's not only refactoring and code reviews that help make progress on various improvements. Even implementing a new feature can unblock many other contributors, who might want to add other features that depend on it.

And all of that is another part of what maintainers do. They continuously improve the codebase so that it is easier to land new changes, they review pull requests, and they communicate with and mentor other contributors. This work is crucial to ensure the long-term health of constantly evolving codebases. And it also has a strong second-order effect. By unblocking other Rust contributors, new features and improvements can be landed quicker and more frequently, which in turns unblocks and improves the lives of Rust users, which results in a positive feedback cycle. This is a multiplicative effect that strongly benefits everyone!

This enablement work of course requires having a deep knowledge of the codebase, but also a long-term focus to perform complex refactorings and grit to continuously review tons of pull requests.

## Conclusion

Based on the observations presented above, we could say that a maintainer is someone who continuously ensures that a software project keeps working, but who also does a lot of hard (and often invisible) work to enable other contributors to evolve and improve the project.

You probably already guessed that maintenance work is not easy, and maybe that is one of the reasons why there seems to be a perpetual imbalance. There are often many people who want to contribute to open source, by making improvements and implementing new features, as it is often seen as being fun, and implementing a new feature is also a very visible achievement that may help meet someone's needs or give them something obvious to be proud of. But there are far fewer people who want to continuously maintain a codebase (especially if they are not paid for it!), as the results of good maintenance are much more difficult to demonstrate, and it often more closely resembles work rather than pure fun.

While being a maintainer can be challenging, *becoming* one is even more difficult. It is not really possible to become a maintainer overnight, as it takes time to not only gain deep expertise in parts of a codebase, but also to gain trust of other maintainers and contributors. All while regularly doing work that many others aren't enthusiastic about doing.

We are incredibly happy that we have so many awesome maintainers in the Rust Project, who pour their souls into ensuring that Rust becomes better every day, and who invest enormous amounts of time into becoming experts in certain Rust projects so that they can effectively maintain them. However, many of those maintainers are volunteers, and when so much maintenance burden falls on someone who still has to work another job to pay the bills, it can lead to burnout. This is something that we want to prevent.

People who do open-source maintenance deserve to be [supported](https://rust-lang.org/funding/) and rewarded for it, which is where maintainer funding comes in. We are currently seeking mechanisms for supporting people who maintain various Rust Project codebases, in a way that provides stability and enables them to focus on what they do best: improving Rust. We also want to publicize their great work—which is otherwise often near-invisible—both to give them something to be proud of, and to attract more support and funding for this critical work. With the Rust Foundation Maintainer Fund and other similar initiatives, we hope to improve the long-term sustainability of Rust maintainership. Stay tuned for more updates.

[rfmf]: https://rustfoundation.org/media/announcing-the-rust-foundation-maintainers-fund/
[rfmf-doc]: https://github.com/rust-lang/funding/blob/main/design-docs/exploration.md
