---
layout: post
title: "rust-analyzer joins the Rust organization!"
author: The rust-analyzer Team on behalf of the entire Rust Team
---

We have an exciting announcement to make!
The [rust-analyzer](https://rust-analyzer.github.io) project, a new implementation of the Language Server Protocol (LSP) for Rust, is now officially a part of the wider Rust organization! 🎉

We want to start by thanking everyone who has gotten us this far, from contributors, to sponsors, to all the users of rust-analyzer in the Rust community. We could not have done this without you.

The immediate impact of this organizational change is limited -- nothing changes for rust-analyzer users or contributors.
However, this change unblocks technical work to make rust-analyzer the officially recommended language server for Rust in the near future.

If you were hesitant to try rust-analyzer before, today is a good opportunity to do so.
Not only is it a very capable language server for Rust, but according to [VS Code statistics](https://marketplace.visualstudio.com/search?target=VSCode&category=Programming%20Languages&sortBy=Rating), it is one of the best rated LSP implementations *across* programming languages.
We highly recommend giving rust-analyzer a spin today, even if it will take some more time for us to complete the due process and switch from the existing officially recommended LSP implementation (RLS) properly.

rust-analyzer enjoys excellent support in many editors:

* For VS Code, install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension from the marketplace.
* For Neovim, follow the setup instructions [from this post](https://sharksforarms.dev/posts/neovim-rust/).
  For Vim, see [coc-rust-analyzer](https://github.com/fannheyward/coc-rust-analyzer).
* For Emacs, follow the setup instructions [from this post](https://robert.kra.hn/posts/2021-02-07_rust-with-emacs/).

For other editors, check [the manual](https://rust-analyzer.github.io/manual.html).

Finally, if you are using IntelliJ-Platform based IDEs like CLion, IDEA or PyCharm, you don't need rust-analyzer.
You should use the awesome [IntelliJ Rust](https://intellij-rust.github.io) plugin by JetBrains.

## History and Future

The rust-analyzer project was started at the very end of 2017 ([first commit](https://github.com/rust-analyzer/rust-analyzer/commit/a63222cd240d9b5405826783603f3b391c90885d)).
At that time, the existing LSP implementation, RLS, had been providing IDE support for Rust for several years.
While it handled well the most important features, it was clearly far from the experience offered by state-of-the-art IDEs for some other languages.

Originally, the plan was to just experiment with error-resilient parsing for Rust; but when you have a good parser, it is so tempting [to add a simple LSP server](https://github.com/rust-analyzer/rust-analyzer/commit/d7c5a6f3081c2e7266620779d3c32067f947b959) on top of it.
Long story short, it took surprisingly little effort to get to a prototype which was already useful as an IDE, which happened in Autumn 2018.
At that critical point, the company [Ferrous Systems](https://ferrous-systems.com) (which was newborn itself) stepped in to fund further development of the prototype.

During 2019, the then nascent rust-analyzer community worked hard to build out the foundation of an IDE.
By 2020, we realized that what we had built was no longer a prototype, but an already tremendously useful tool for day-to-day Rust programming.
This culminated in [RFC2912](https://github.com/rust-lang/rfcs/pull/2912): "Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation".
The RFC was accepted with overwhelming support from the community: it is still the most upvoted Rust RFC ever.
However, there was a wrinkle in the plan -- rust-analyzer was not an official Rust project!
That's what we are fixing today!

Next, we will proceed with the plan outlined in the RFC: advertising rust-analyzer as the very likely future of Rust IDE support, gathering feedback, and, conditioned on the positive outcome of that, sunsetting RLS, the currently recommended language server. So, once again -- do try out rust-analyzer and leave feedback on the [tracking issues](https://github.com/rust-analyzer/rust-analyzer/issues/4224).

After the transition, we will double down on the technical side of things.

As exciting as rust-analyzer is today, it only scratches the surface of what's possible when you bring the compiler's intricate understanding of the code right into the text editor.
The end-game we are aiming for is creating an API to analyze and transform Rust code with full access to semantics.


## Funding

One of the hardest nuts to crack for the present transition was the question of funding.
Today, Rust is organized as a set of somewhat independent projects (rustc, cargo, rustup, rustfmt), and there's deliberately no way to fund a specific project directly. The new [Rust Foundation](https://foundation.rust-lang.org) is the official place to sponsor Rust in general, with the Foundation Board overseeing funds allocation. Yet, it has always been encouraged for individuals to seek individual funding. While the Rust project may advertise funding opportunities for individual contributors, it does not officially _endorse_ these efforts nor does it facilitate the funding of entire teams.

rust-analyzer has received a significant share of funds from its OpenCollective and later GitHub Sponsors, managed by Ferrous Systems. This OpenCollective funded efforts by both individual contributors and Ferrous Systems employees. Details of this can be found in their [transparency reports](https://rust-analyzer.github.io/blog/2021/08/03/financial-report-3.html). 

Luckily, the OpenCollective has always been managed in a way that would make it possible to transfer it to a different account holder.
With this transition, the OpenCollective will be renamed from "rust-analyzer OpenCollective" to "Ferrous Systems OpenCollective (rust-analyzer)". This allows current sponsors to continue to sponsor and also make it clear that their chosen project will continue to be funded.

In a sense, the OpenCollective is handed to Ferrous Systems. All Sponsor credits will move to <https://ferrous-systems.com/open-source/#sponsors>.

We would like to thank Ferrous Systems for their openness and flexibility in the process, for their thoughtfulness in making sure the funding situation around rust-analyzer was clear, and for taking on the effort of fundraising.

Eventually the [rust-analyzer GitHub Sponsors](https://github.com/sponsors/rust-analyzer) will also move away from the rust-analyzer GitHub organisation.

And of course, another great way for companies to support rust-analyzer development is to hire [the people working on rust-analyzer](https://github.com/rust-analyzer/rust-analyzer/graphs/contributors) to continue to do so.


## Thanks

We'd like to once again thank everyone who help get rust-analyzer to this point. From experiment to being well on its way to the officially recommended LSP implementation for Rust, we couldn't have done it without the help of our contributors, sponsors, and users.

## Conclusion

So that's where we are at right now!
Thanks to the awesome contributors to rustc, clippy, cargo, LSP, IntelliJ Rust, RLS and rust-analyzer, Rust today already enjoys great IDE support, even if it still has a bit of experimental flair to it.
