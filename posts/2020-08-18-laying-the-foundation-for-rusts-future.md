---
layout: post
title: "Laying the foundation for Rust's future"
author: The Rust Core Team
release: false
---

The Rust project was originally [conceived in 2010][2010] (depending on how you count, you might even say [2006][2006]!) as a [Mozilla Research] project, but the long term goal has always been to establish Rust as a self-sustaining project. In 2015, [with the launch of Rust 1.0][onepointoh], Rust established its project direction and governance independent of the Mozilla organization. Since then, Rust has been operating as an autonomous organization, with Mozilla being a prominent and consistent financial and legal sponsor.

Mozilla was, and continues to be, excited by the opportunity for the Rust language to be widely used, *and supported*, by many companies throughout the industry. Today, many companies, both large and small, are using Rust in more diverse and more significant ways, from [Amazon’s Firecracker][firecracker], to [Fastly’s Lucet][lucet], to critical services that power [Discord], [Cloudflare], [Figma], [1Password], and many, many more.

On Tuesday, August 11th 2020, Mozilla [announced][layoffs] their decision to restructure the company and to lay off around 250 people, including folks who are active members of the Rust project and the Rust community. Understandably, these layoffs have generated a lot of uncertainty and confusion about the impact on the Rust project itself. Our goal in this post is to address those concerns. We’ve also got a big announcement to make, so read on!

## Community impact

There’s no denying the impact these layoffs have had on all members of the Rust community, particularly the folks who have lost their jobs in the middle of a global pandemic. Sudden, unexpected layoffs can be a difficult experience, and they are made no less difficult when it feels like the world is watching. Impacted employees who are looking for job assistance can be found on [Mozilla’s talent directory][talent-directory].

Notwithstanding the deep personal impact, the Rust project as a whole is very resilient to such events. We have leaders and contributors from a diverse set of different backgrounds and employers, and that diversity is a critical strength. Further, it is a common misconception that all of the Mozilla employees who participated in Rust leadership did so as a part of their employment. In fact, many Mozilla employees in Rust leadership [contributed to Rust in their personal time][manish-tweet], not as a part of their job. 

Finally, we would like to emphasize that membership in Rust teams is given to individuals and is not connected to one’s employer. Mozilla employees who are also members of the Rust teams continue to be members today, even if they were affected by the layoffs. Of course, some may choose to scale back their involvement. We understand not everyone might be able to continue contributing, and we would fully support their decision. We're grateful for everything they have done for the project so far.

## Starting a foundation

As the project has grown in size, adoption, and maturity, we’ve begun to feel the pains of our success. We’ve developed legal and financial needs that our current organization lacks the capacity to fulfill. While we were able to be successful with Mozilla’s assistance for quite a while, we’ve reached a point where it’s difficult to operate without a legal name, address, and bank account. “How does the Rust project sign a contract?” has become a question we can no longer put off.

Last year, we began [investigating the idea of creating an independent Rust foundation][niko-post]. Members of the Rust Team with prior experience in open source foundations got together to look at the current landscape, identifying the things we’d need from a foundation, evaluating our options, and interviewing key members and directors from other foundations.

Building on that work, **Mozilla and the Rust Core Team are happy to announce plans to create a Rust foundation. Our goal is to have the first iteration of the foundation up and running by the end of the year.**

This foundation’s first task will be something Rust is already great at: [taking ownership]. This time, the resource is legal, rather than something in a program. The various trademarks and domain names associated with Rust, Cargo, and crates.io will move into the foundation, which will also take financial responsibility for the costs they incur. We see this first iteration of the foundation as just the beginning. There’s a lot of possibilities for growing the role of the foundation, and we’re excited to explore those in the future. 

For now though, we remain laser-focused on these initial narrow goals for the foundation. As an immediate step the Core Team has [selected members to form a project group][project-group] driving the efforts to form the foundation. Expect to see follow-up blog posts from the group with more details about the process and opportunities to give feedback. In the meantime, you can email the group at [foundation@rust-lang.org][mail].

## Leading with infrastructure

While we have only begun the process of setting up the foundation, over the past two years the Infrastructure Team has been leading the charge to reduce the reliance on any single company sponsoring the project, as well as growing the number of companies that support Rust.

These efforts have been quite successful, and — as you can see on our [sponsorship page][sponsors] — Rust’s infrastructure is already supported by a number of different companies throughout the ecosystem. As we legally transition into a fully independent entity, the Infrastructure Team plans to continue their efforts to ensure that we are not overly reliant on any single sponsor.

## Thank you

We’re excited to start the next chapter of the project by forming a foundation. We would like to thank everyone we shared this journey with so far: Mozilla for incubating the project and for their support in creating a foundation, our team of leaders and contributors for constantly improving the community and the language, and everyone using Rust for creating the powerful ecosystem that drives so many people to the project. We can’t wait to see what our vibrant community does next.

[layoffs]: https://blog.mozilla.org/blog/2020/08/11/changing-world-changing-mozilla/
[onepointoh]: https://blog.rust-lang.org/2015/05/15/Rust-1.0.html
[Mozilla Research]: https://research.mozilla.org/
[2006]: https://github.com/graydon/rust-prehistory/commit/b0fd440798ab3cfb05c60a1a1bd2894e1618479e
[2010]: https://github.com/rust-lang/rust/commit/c01efc669f09508b55eced32d3c88702578a7c3e
[talent-directory]: https://talentdirectory.mozilla.org/
[niko-post]: http://smallcultfollowing.com/babysteps/blog/2020/01/09/towards-a-rust-foundation/
[project-group]: https://www.rust-lang.org/governance/teams/core#project-foundation
[mail]: mailto:foundation@rust-lang.org
[sponsors]: https://www.rust-lang.org/sponsors
[taking ownership]: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
[manish-tweet]: https://twitter.com/ManishEarth/status/1294023260770770944
[Discord]: https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f
[Cloudflare]: https://blog.cloudflare.com/enjoy-a-slice-of-quic-and-rust/
[Figma]: https://www.figma.com/blog/rust-in-production-at-figma/
[1Password]: https://blog.1password.com/1passwordx-december-2019-release/
[lucet]: https://www.fastly.com/blog/announcing-lucet-fastly-native-webassembly-compiler-runtime
[firecracker]: https://aws.amazon.com/blogs/aws/firecracker-lightweight-virtualization-for-serverless-computing/
