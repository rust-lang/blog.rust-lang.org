+++
path = "inside-rust/2023/01/30/cargo-sparse-protocol"
title = "Help test Cargo's new index protocol"
authors = ["Eric Huss"]
aliases = ["inside-rust/2023/01/30/cargo-sparse-protocol.html"]

[extra]
team = "The Cargo Team"
team_url = "https://www.rust-lang.org/governance/teams/dev-tools#cargo"
+++

Cargo's new index protocol will be available starting in Rust 1.68, which will be released on 2023-03-09.
This new "sparse" protocol should usually provide a significant performance improvement when accessing [crates.io].

We would like your help in testing this new feature and infrastructure.
If you use beta (1.68) or nightly-2023-01-21 or newer, set the environment variable `CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse`, or edit your [`.cargo/config.toml` file](https://doc.rust-lang.org/cargo/reference/config.html) to add:

```toml
[registries.crates-io]
protocol = "sparse"
```

We would like to hear reports on your experience.
If you run into a problem, please [open an issue](https://github.com/rust-lang/cargo/issues/new?assignees=&labels=C-bug&template=bug_report.yml).
If you would like to post general feedback, feel free to leave a message on [this internals thread](https://internals.rust-lang.org/t/cargo-sparse-protocol-feedback-thread/18234).

Along with fetching crates and running `cargo update`, we'd also like to hear if you have any issues when running `cargo publish`.
Another data point that may be helpful is to gauge how many users are behind a restrictive firewall, proxy, or other network environment that prevents access to the index.

This new service will be opt-in as we roll it out and gather information about how it performs.
In the near future we intend to switch to this as the default.

## Background

In order for Cargo to determine which crates exist on [crates.io], it downloads and reads an "index" which lists all versions of all crates.
The index lives in a [git repository](https://github.com/rust-lang/crates.io-index/) hosted on GitHub.
Cargo fetches and stores the index in Cargo's home directory.
This system lets GitHub handle the server-side processing, and provides a convenient way to incrementally fetch new updates.

However, as the index has grown considerably over time, this system has started to hit scaling limitations, and initial fetches and updates continue to slow down. You may have noticed a pause when Cargo displays `Updating crates.io index` or while going through the "resolving deltas" phase:

```
Updating crates.io index
    Fetch [=================>       ]  74.01%, (64415/95919) resolving deltas
```

With [RFC 2789](https://rust-lang.github.io/rfcs/2789-sparse-index.html), we introduced a new protocol to improve the way Cargo accesses the index.
Instead of using git, it fetches files from the index directly over HTTPS.
Cargo will only download information about the specific crate dependencies in your project.

We have introduced a new service at `https://index.crates.io/` for hosting the [crates.io] index.
If you are behind a restrictive firewall or proxy, you may need to explicitly allow Cargo to access this site.

More information about how the index and this new sparse protocol works can be found in [the documentation](https://doc.rust-lang.org/nightly/cargo/reference/registry-index.html#index-protocols).

The [crates.io] git repository will continue to be the source of truth for the index, and Cargo will continue to support git indexes indefinitely.

## Acknowledgements

We would like to give a huge show of gratitude to [Arlo Siemsen](https://github.com/arlosi) who implemented this feature and went to great lengths to bring it to completion.
We would also like to thank the [crates.io team](https://www.rust-lang.org/governance/teams/crates-io) and the [infrastructure team](https://www.rust-lang.org/governance/teams/infra) for implementing, reviewing, and supporting this endeavor.
We would also like to acknowledge the generosity of [GitHub](https://github.com/) for hosting the crates.io index over the past 8 years.

[crates.io]: https://crates.io
