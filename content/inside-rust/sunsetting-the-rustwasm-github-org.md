+++
path = "inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org"
title = "Sunsetting the rustwasm GitHub org"
authors = ["Alex Crichton"]
aliases = []
+++

In 2024 the Rust and WebAssembly Working Group was [officially
archived][archive] in the Rust project after ~5 years of inactivity with an
intent to shut down the [rustwasm] GitHub Organization soon after. Although a
bit later than intended the purpose of this post is to give a concrete timeline
for this process in which the [rustwasm] GitHub Organization will become fully
archived.

The [`wasm-bindgen`] repository is going to be transferred to a new
[wasm-bindgen organization][wbgorg] with new additional maintainers. All
other repositories in the [rustwasm] organization are going to be archived in
place or transferred to their existing maintainers if they elect to do so.
Transferring [`wasm-bindgen`] will occur in the weeks following this post and
archiving the organization is going to happen September 2025.

The [rustwasm] organization has historically been under the purview of the Rust
and WebAssembly Working Group, and given the historical prominence of some
projects it contained, this post is serving as an announcement of upcoming plans
for repositories in the organization.

[archive]: https://github.com/rust-lang/team/pull/1489
[wbgorg]: https://github.com/wasm-bindgen

## History of the Rust and WebAssembly Working Group

The Rust and WebAssembly Working Group was [created in 2018][create] as part of
the initiative for the 2018 edition at the time. Through 2019 the
working group was quite active and helped Rust's support for WebAssembly
flourish. Tools such as [`wasm-bindgen`] and [`wasm-pack`] were created as part
of this effort and continue to be used to this day. After 2019 though the
organization saw a drastic reduction in activity and most projects have been in
maintenance mode for nearly 5 years at this point.

While the working group has been officially archived for over a year now the
repositories in the [rustwasm] organization continued on and continued to see
some use and minor maintenance. This has perpetuated a confusing situation for
users and maintainers alike where it's not clear what the maintenance story for
many of these repositories are and how to move forward with the repositories
that are actively in use.

## Sunsetting the `rustwasm` Organization

In September of 2025 the [rustwasm] GitHub organization will be archived to
cease all activity within the organization. This will help clarify that
[rustwasm] is no longer a central hub for all things Rust-and-WebAssembly but
instead is a historical archive of efforts.

The [rustwasm] organization has a number of repositories within it, most of
which haven't seen any activity in quite some time. The current plan at this
time is to arrange for [`wasm-bindgen`] to be transferred to a new
[project-specific organization][wbgorg] and additionally add new fresh
maintainers to assist with project direction going forward. All other
repositories in the [rustwasm] organization will be archived or transferred to
their existing maintainers if needed.

### Transferring `wasm-bindgen`

The [`wasm-bindgen`] project is relied on in a number of contexts today and
existing users have expressed interest in both maintaining and growing the
repository over time. The [rustwasm] organization is no longer a great home for
this work so the plan is to transfer it out to a new home in a neutral location
not owned by any one person or company. This transfer will be coupled with
inviting new maintainers to the repository for improved project maintenance in
addition to evolving the project.

While a critical mass of new maintainers has already been reached, if you are
interested in helping out with maintenance [an issue has been
created][wasm-bindgen-help] to coordinate efforts around maintenance with
[`wasm-bindgen`]. Feel free to leave a comment there to help out with this
transition.

### Archiving other repositories

The current plan at this time is to archive all other repositories in the
[rustwasm] organization. This includes repositories such as:

* https://github.com/rustwasm/wasm-pack
* https://github.com/rustwasm/gloo
* https://github.com/rustwasm/twiggy
* https://github.com/rustwasm/walrus
* https://github.com/rustwasm/weedle
* (see https://github.com/orgs/rustwasm/repositories for the full list)

> **Note**: wasm-bindgen depends on crates such as `walrus` and `weedle` at this
> time and won't depend on archived repositories. Depending on how maintainers
> would like to organize it these dependencies may be inlined into the
> wasm-bindgen repository for wasm-bindgen's needs or they may be transferred to
> the new wasm-bindgen organization. Regardless wasm-bindgen will not be using
> unmaintained dependencies.

If you are an preexisting maintainer of one of these repositories please reach
out to have the repository transferred to you. Otherwise it's not clear at this
time whether these repositories are still actively in use or how much they're
relied on. The historically trusted nature of the [rustwasm] organization means
it's not quite as simple as transferring these repositories to the first
volunteer. Instead transferring repositories will require vetting new
maintainers for trustworthiness and reliability and unfortunately the current
admin of the [rustwasm] organization is not prepared to do this.

If your use case critically relies on these repositories it is
recommended to fork the repository. Where required it's possible to leave a
message in the old README pointing to the fork. If a fork is not viable then
understanding is appreciated in that the [rustwasm] organization has been
inactive for over 5 years now and there are no known active maintainers to
reach out to. Charting a path for each repository requires a significant amount
of effort to understand current users and plan for future users and no one is
available to perform this work. The goal of this effort is to do the best with
maintainers/admins that still remain and while not a perfect answer it's
predicted the best answer in many cases here is archiving the repository.

As an absolute last resort you can reach out to [@alexcrichton] on Zulip to
discuss alternatives.

## Timeline for transfers/archives

The [`wasm-bindgen`] repository will be transferred to a new [wasm-bindgen
organization][wbgorg] in the weeks after this post. Some minor details still
need to be sorted out maintainer-wise and that's all that's left to do.
Afterwards a final grace period will be allowed until September at which point
the [rustwasm] organization will become fully archived. Issues will be filed in
various repositories giving a heads up to any interested folks indicating as
such.

[rustwasm]: https://github.com/rustwasm
[create]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018/
[`wasm-bindgen`]: https://github.com/rustwasm/wasm-bindgen
[`wasm-pack`]: https://github.com/rustwasm/wasm-pack
[@alexcrichton]: https://github.com/alexcrichton
[wasm-bindgen-help]: https://github.com/rustwasm/wasm-bindgen/issues/4533

