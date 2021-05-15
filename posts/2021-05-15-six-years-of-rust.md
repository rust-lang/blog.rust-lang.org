---
layout: post
title: "Six Years of Rust"
author: The Rust Team
---

Today marks [Rust]'s sixth birthday since it went 1.0 in 2015. A lot has changed since then and especially over the past year, and Rust was no different. In 2020, there was no foundation yet, no const generics, and a lot organisations were still wondering whether Rust was production ready. 

In the midst of the COVID-19 pandemic, hundreds of Rust's global distributed set of team members and volunteers shipped over nine new stable releases of Rust, in addition to various bugfix releases. Today, "Rust in production" isn't a question, but a statement. The newly founded Rust foundation has several members who value using Rust in production enough to help continue to support and contribute to its open development ecosystem.

We wanted to take today to look back at some of the major improvements over the past year, how the community has been using Rust in production, and finally look ahead at some of the work that is currently ongoing to improve and use Rust for small and large scale projects over the next year. Let's get started!

[rust]: https://www.rust-lang.org

## Recent Additions
The Rust language has improved tremendously in the past year, gaining a lot of quality of life features, that while they don't fundamentally change the language, they help make using and maintaining Rust in more places even easier.

-  As of Rust 1.52.0 and the upgrade to LLVM 12, one of few cases of unsoundness around forward progress (such as handling infinite loops) has finally been resolved. This has been a long running collaboration between the Rust teams and the LLVM project, and is a great example of improvements to Rust also benefitting the wider ecosystem of programming languages.

-  On supporting an even wider ecosystem, the introduction of Tier 1 support for 64 bit ARM Linux, and Tier 2 support for ARM macOS & ARM Windows, has made Rust an even better place to easily build your projects across new and different architectures.

-  The most notable exception to the theme of polish has been the major improvements to Rust's compile-time capabilities. The stabilisation of const generics for primitive types, the addition of control flow for `const fn`s, and allowing procedural macros to be used in more places, have allowed completely powerful new types of APIs and crates to be created.

Rustc wasn't the only tool that had significant improvements.

- Cargo just recently stabilised its new feature resolver, that makes it easier to use your dependencies across different targets.

- Rustdoc stabilised its "intra-doc links" feature, allowing you to easily and automatically cross reference Rust types and functions in your documentation.

- Clippy with Cargo now uses a separate build cache that provides much more consistent behaviour.


## Rust In Production
Each year Rust's growth and adoption in the community and industry has been unbelievable, and this past year has been no exception. Once again in 2020, Rust was voted StackOverflow's [Most Loved Programming Language][stackoverflow]. Thank you to everyone in the community for your support, and help making Rust what it is today.

With the formation of the [Rust foundation], Rust has been in a better position to build a sustainable open source ecosystem empowering everyone to build reliable and efficient software. A number of companies that use Rust have formed teams dedicated to maintaining and improving the Rust project, including [AWS](https://aws.amazon.com/blogs/opensource/how-our-aws-rust-team-will-contribute-to-rusts-future-successes/), [Facebook](https://engineering.fb.com/2021/04/29/developer-tools/rust/), and Microsoft.

And it isn't just Rust that has been getting bigger. Larger and larger companies have been adopting Rust in their projects and offering officially supported Rust APIs.

- Both Microsoft and Amazon have just recently announced and released their new officially supported Rust libraries for interacting with [Windows] and [AWS]. Official first party support for these massive APIs helps make Rust people's first choice when deciding what to use for their project.
- The cURL project has released new versions that offer opt-in support for using Rust libraries for handling [HTTP/s] and [TLS] communication. This has been a huge inter-community collaboration between the ISRG, the Hyper & Rustls teams, and the cURL project, and we'd like to thank everyone for their hard work in providing new memory safe backends for a project as massive and widely used as cURL!
- Tokio (an asynchronous runtime written in Rust), released its [1.0 version][tokio-1.0] and announced their three year stability guarantee, providing everyone with a solid, stable foundation for writing reliable network applications without compromising speed.

[stackoverflow]: https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/
[tokio-1.0]: https://tokio.rs/blog/2020-12-tokio-1-0
[http/s]: https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/
[tls]: https://daniel.haxx.se/blog/2021/02/09/curl-supports-rustls/
[rust foundation]: https://foundation.rust-lang.org/posts/2021-02-08-hello-world/
[windows]:https://github.com/microsoft/windows-rs
[aws]: https://github.com/awslabs/aws-sdk-rust

## Future Work
Of course, all that is just to start, we're seeing more and more initiatives putting Rust in exciting new places;

- Critical Section & Ferrous Systems have started [Ferrocene]. A project to make Rust a viable programming language for safety and mission critical systems across the industry.
- Embark Studios have released an initial prototype of [`rust-gpu`], a new compiler backend that allows writing graphics shaders using Rust for GPUs.
- The Linux project is currently [considering a proposal to add Rust as the second language to the kernel][linux-rust] to enable writing safer driver and kernel-space code.
- Google has announced that it [now supports building low level components of the Android OS in Rust][android-rust], and have already begun an effort to rewrite their bluetooth stack with Rust! 

Right now the Rust teams are planning and coordinating the 2021 edition of Rust. Much like this past year, a lot of themes of the changes are around improving quality of life. You can check out our recent post about ["The Plan for the Rust 2021 Edition"][edition-plan] to see what the changes the teams are planning.

And that's just the tip of the iceberg; there are a lot more changes being worked on, and exciting new open projects being started every day in Rust. We can't wait to see what you all build in the year ahead!

---

Are there changes, or projects from the past year that you're excited about? Are you looking to get started with Rust? Do you want to help contribute to the 2021 edition? Then come on over, introduce yourself, and join the discussion over on our [Discourse] forum and [Zulip] chat! Everyone is welcome, we are committed to providing a friendly, safe and welcoming environment for all, regardless of gender, sexual orientation, disability, ethnicity, religion, or similar personal characteristic.

[ferrocene]: https://ferrous-systems.com/ferrocene
[`rust-gpu`]: https://github.com/EmbarkStudios/rust-gpu
[linux-rust]: https://lore.kernel.org/lkml/CANiq72khBa2GcB6-PHM3A44Y90d6vzYAS=BVpk3nT4B6u+NVDw@mail.gmail.com/T/#mb5e524dae9d5a5815c6e68eb36b9bde4e87c861d
[edition-plan]: https://blog.rust-lang.org/2021/05/11/edition-2021.html
[discourse]: https://users.rust-lang.org/
[zulip]: https://rust-lang.zulipchat.com/
[android-rust]: https://security.googleblog.com/2021/04/rust-in-android-platform.html
