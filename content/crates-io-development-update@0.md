+++
path = "2024/07/29/crates-io-development-update"
title = "crates.io: development update"
authors = ["Tobias Bieniek"]
aliases = ["2024/07/29/crates-io-development-update.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

Since crates.io does not have releases in the classical sense, there are no release notes either. However, the crates.io team still wants to keep you all updated about the ongoing development of crates.io. This blog post is a summary of the most significant changes that we have made to crates.io in the past months.

## `cargo install`

When looking at crates like [ripgrep](https://crates.io/crates/ripgrep) you will notice that the installation instructions now say `cargo install ripgrep` instead of `cargo add ripgrep`. We implemented this change to make it easier for users to install crates that have binary targets. `cargo add` is still the correct command to use when adding a crate as a dependency to your project, but for binary-only crates like ripgrep, `cargo install` is the way to go.

We achieved this by analyzing the uploaded crate files when they are published to crates.io. If a crate has binary targets, the names of the binaries will now be saved in our database and then conveniently displayed on the crate page:

![Dark Mode Screenshot](../../../images/2024-07-29-crates-io-development-update/cargo-install.png)

After shipping this feature we got notified that some library crates use binaries for local development purposes and the author would prefer to not have the binaries listed on the crate page. The cargo team has been working on a [solution](https://github.com/rust-lang/cargo/pull/13713) for this by using the `exclude` manifest field, which will be shipped soon.


## Dark mode

If your operating system is set to dark mode, you may have noticed that crates.io now automatically switches to a dark user interface theme. If you don't like the dark theme, you can still switch back to the light theme by clicking the color theme icon in the top right corner of the page. By default, the theme will be set based on your operating system's theme settings, but you can also override this setting manually.

![Dark Mode Screenshot](../../../images/2024-07-29-crates-io-development-update/dark-mode.png)

Similar to GitHub, we now also have dark/light theme support for images in your `README.md` files:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://test.crates.io/logo_dark.svg">
  <img src="https://test.crates.io/logo.svg" alt="logo" width="200">
</picture>
```


## RSS feeds

Inspired by our friends at the [Python Package Index](https://warehouse.pypa.io/api-reference/feeds.html), we have introduced a couple of experimental RSS feeds for crates.io:

- <https://static.crates.io/rss/crates.xml>: The latest new crates registered on crates.io (the past 60 minutes, but at least 50 new crates).
- <https://static.crates.io/rss/updates.xml>: The latest version updates on crates.io (the past 60 minutes, but at least 100 versions).
- e.g. <https://static.crates.io/rss/crates/serde.xml>: The latest version updates of the `serde` crate (the past 24 hours, but at least 10 versions).

This will allow you to keep track of the latest crate releases and updates in your favorite RSS reader. The original GitHub issue requested a feed for all the crates you "follow" on crates.io, but we decided that per-crate feeds would be more useful for now. If you have any feedback on this feature, please let us know!


## API token expiry notifications

Our crates.io team member [@hi-rustin](https://github.com/hi-rustin) has been very active in improving our API tokens user experience. If you create an API token with an expiry date, you will now receive a notification email three days before the token expires. This will help you to remember to renew your token before it expires and your scripts stop working.

Following this change, he also implemented a way to create new API tokens based on the configuration of existing tokens, which will make it much easier to renew tokens without having to reconfigure all the permissions. The user interface on the "API tokens" settings page now shows a "Regenerate" button, which will allow you to copy the permissions of existing tokens. Similarly, the token expiry notifications will now also contain a link that directly fills in the permissions of the expiring token, so you can easily create a new token with the same permissions.

![Dark Mode Screenshot](../../../images/2024-07-29-crates-io-development-update/regenerate-button.png)


## Database performance optimizations

Our latest addition to the crates.io team, [@eth3lbert](https://github.com/eth3lbert), has been working on optimizing the database queries that power crates.io. He has been working on a couple of pull requests that aim to reduce the load on the database server and make the website faster for everyone. Some of the changes he has made include:

- [#7865](https://github.com/rust-lang/crates.io/pull/7865): Further speed-up reverse dependencies query
- [#7941](https://github.com/rust-lang/crates.io/pull/7941): Improve crates endpoint performance
- [#8734](https://github.com/rust-lang/crates.io/pull/8734): Add partial index on versions table
- [#8737](https://github.com/rust-lang/crates.io/pull/8737): Improve the performance of reverse dependencies using the `default_versions` table

In addition to that, we have recently migrated our database servers to a new provider with more memory and faster storage. This has also improved the performance of the website and allowed us to run more complex queries without running into performance issues. It was previously taking multiple seconds to load e.g. https://crates.io/crates/syn/reverse_dependencies, but now the server usually responds in much less than a second.

Another piece of the puzzle was archiving old data that is no longer needed for the website. We have moved the download counts older than 90 days into CSV files that are stored on S3 and will soon be publicly available for download via our CDNs. This has reduced the size of the database significantly and improved the performance of some of our background jobs.


## Feedback

We hope you enjoyed this update on the development of crates.io. If you have any feedback or questions, please let us know on [Zulip](https://rust-lang.zulipchat.com/#narrow/stream/318791-t-crates-io) or [GitHub](https://github.com/rust-lang/crates.io/discussions). We are always happy to hear from you and are looking forward to your feedback!
