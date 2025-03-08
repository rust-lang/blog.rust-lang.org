+++
layout = "post"
title = "A tale of broken badges and 23,000 features"
author = "Tobias Bieniek"
team = "the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>"
+++

Around mid-October of 2023 the crates.io team was [notified](https://github.com/rust-lang/crates.io/issues/7269) by one of our users that a [shields.io](https://shields.io) badge for their crate stopped working. The issue reporter was kind enough to already debug the problem and figured out that the API request that shields.io sends to crates.io was most likely the problem. Here is a quote from the original issue:

> This crate makes heavy use of feature flags which bloat the response payload of the API.

Apparently the API response for this specific crate had broken the 20 MB mark and shields.io wasn't particularly happy with this. Interestingly, this crate only had 9 versions published at this point in time. But how do you get to 20 MB with only 9 published versions?

As the quote above already mentions, this crate is using features… a lot of features… almost 23,000! 😱

What crate needs that many features? Well, this crate provides SVG icons for Rust-based web applications… and it uses one feature per icon so that the payload size of the final WebAssembly bundle stays small.

At first glance there should be nothing wrong with this. This seems like a reasonable thing to do from a crate author perspective and neither cargo, nor crates.io, were showing any warnings about this. Unfortunately, some of the internals are not too happy about such a high number of features…

The first problem that was already identified by the crate author: the API responses from crates.io are getting veeeery large. Adding to the problem is the fact that the crates.io API currently does not paginate the list of published versions. Changing this is obviously a breaking change, so our team had been a bit reluctant to change the behavior of the API in that regard, though this situation has shown that we will likely have to tackle this problem in the near future.

The next problem is that the [index file](https://index.crates.io/ic/on/icondata) for this crate is also getting large. With 9 published versions it already contains 11 MB of data. And just like the crates.io API, there is currently no pagination built into the package index file format.

Now you may ask, why do the package index and cargo need to know about features? Well, the easy answer is: for dependency resolution. Features can enable optional dependencies, so when a dependency feature is used it might influence the dependency resolution. Our initial thought was that we could at least drop all empty feature declarations from the index file (e.g. `foo = []`), but the cargo team informed us that cargo relies on them being available there too, and so for backwards-compatibility reasons this is not an option.

On the bright side, most Rust users are on cargo versions these days that use the sparse package index by default, which only downloads index files for packages actually being used. In other words: only users of this icon crate need to pay the price for downloading all the metadata. On the flipside, this means users who are still using the git-based index are all paying for this one crate using 23,000 features.

So, where do we go from here? 🤔

While we believe that supporting such a high number of features is conceptually a valid request, with the current implementation details in crates.io and cargo we cannot support this. After analyzing all of these downstream effects from a single crate having that many features, we realized we need some form of restriction on crates.io to keep the system from falling apart.

Now comes the important part: **on 2023-10-16 the crates.io team deployed a change limiting the number of features a crate can have to 300 for any new crates/versions being published.**

… for now, or at least until we have found solutions for the above problems.

We are aware of a couple of crates that also have legitimate reasons for having more than 300 features, and we have granted them appropriate exceptions to this rule, but we would like to ask everyone to be mindful of these limitations of our current systems.

We also invite everyone to participate in finding solutions to the above problems. The best place to discuss ideas is the [crates.io Zulip stream](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io/), and once an idea is a bit more fleshed out it will then be transformed into an [RFC](https://github.com/rust-lang/rfcs/).

Finally, we would like to thank [Charles Edward Gagnon](https://github.com/Carlosted) for making us aware of this problem. We also want to reiterate that the author and their crate are not to blame for this. It is hard to know of these crates.io implementation details when developing crates, so if anything, the blame would be on us, the crates.io team, for not having limits on this earlier. Anyway, we have them now, and now you all know why! 👋
