+++
path = "inside-rust/2026/04/17/crates-io-svelte-public-testing"
title = "crates.io: Help test our new web frontend"
authors = ["Tobias Bieniek"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

We have been porting the crates.io frontend from Ember.js to Svelte 5 ([tracking issue](https://github.com/rust-lang/crates.io/issues/12515)), and the new Svelte app is now **ready for public testing at <https://crates.io/svelte/>**. We'd love your help trying it out before we make it the default.

The Svelte app is intended as a 1:1 port of the Ember.js app for now. It should look and behave the same (our UI test suite, including visual regression testing, already passes against both), so any significant difference you notice is a bug we want to hear about. To keep the two apps easy to compare, a few rough edges of the Ember.js app have been carried over as well. Those will get smoothed out once we are no longer maintaining two frontends in parallel.

Both apps are served from the same domain and share session state and data, so you can just visit <https://crates.io/svelte/> and keep using the site normally without logging in again. If something looks or behaves differently than on the Ember.js app, please let us know, either via [GitHub](https://github.com/rust-lang/crates.io/issues/new?template=bug_report.yml) or on Zulip ([#t-crates-io](https://rust-lang.zulipchat.com/#narrow/channel/318791-t-crates-io)).

If testing goes well, we'll switch the Svelte app to the default in the coming weeks, and we're looking forward to what we can build on top of it from there.

We want to thank the [Ember.js team](https://emberjs.com/teams/) for a framework that served crates.io well for many years, and the [Svelte team](https://svelte.dev) for making the transition to something new so enjoyable. A big thank you also to [@eth3lbert](https://github.com/eth3lbert) from the crates.io team for reviewing the bulk of the migration pull requests.
