+++
layout = "post"
date = 2023-12-11
title = "Cargo cache cleaning"
author = "Eric Huss"
team = "The Cargo Team <https://www.rust-lang.org/governance/teams/dev-tools#cargo>"
+++

Cargo has recently gained an unstable feature on the nightly channel (starting with nightly-2023-11-17) to perform automatic cleaning of cache content within Cargo's home directory.
This post includes:

- A description of what this means ([What is this feature?](#what-is-this-feature))
- A call for help ([What to watch out for](#what-to-watch-out-for) and [Request for feedback](#request-for-feedback))
- Implementation details ([Design considerations and implementation details](#design-considerations-and-implementation-details))
- An outline of what is planned for the future ([Plan for the future](#plan-for-the-future))

In short, we are asking people who use the nightly channel to enable this feature and report any issues you encounter on the [Cargo issue tracker][cargo-issues].
To enable it, place the following in your Cargo config file (typically located in `~/.cargo/config.toml` or `%USERPROFILE%\.cargo\config.toml` for Windows):

```toml
[unstable]
gc = true
```

Or set the `CARGO_UNSTABLE_GC=true` environment variable or use the `-Zgc` CLI flag to turn it on for individual commands.

We'd particularly like people who use unusual filesystems or environments to give it a try, since there are some parts of the implementation which are sensitive and need battle testing before we turn it on for everyone.

[cargo-issues]: https://github.com/rust-lang/cargo/issues/

## What is this feature?

Cargo keeps a variety of cached data within the Cargo home directory.
This cache can grow unbounded and can get quite large (easily reaching many gigabytes).
Community members have developed tools to manage this cache, such as [`cargo-cache`](https://crates.io/crates/cargo-cache), but cargo itself never exposed any ability to manage it.

This cache includes:

- [Registry index data], such as package dependency metadata from [crates.io].
- Compressed `.crate` files downloaded from a registry.
- The uncompressed contents of those `.crate` files, which `rustc` uses to read the source and compile dependencies.
- Clones of git repositories used by git dependencies.

The new garbage collection ("GC") feature adds tracking of this cache data so that cargo can automatically or manually remove unused files.
It keeps an SQLite database which tracks the last time the various cache elements have been used.
Every time you run a cargo command that reads or writes any of this cache data, it will update the database with a timestamp of when that data was last used.

What isn't yet included is cleaning of target directories, see [Plan for the future](#plan-for-the-future).

[crates.io]: https://crates.io/
[registry index data]: https://doc.rust-lang.org/cargo/reference/registry-index.html

### Automatic cleaning

When you run cargo, once a day it will inspect the last-use cache tracker, and determine if any cache elements have not been used in a while.
If they have not, then they will be automatically deleted.
This happens with most commands that would normally perform significant work, like `cargo build` or `cargo fetch`.

The default is to delete data that can be locally recreated if it hasn't been used for 1 month, and to delete data that has to be re-downloaded after 3 months.

Automatic deletion is disabled if cargo is offline such as with `--offline` or `--frozen` to avoid deleting artifacts that may need to be used if you are offline for a long period of time.

The initial implementation has exposed a variety of configuration knobs to control how automatic cleaning works.
However, it is unlikely we will expose too many low-level details when it is stabilized, so this may change in the future (see issue [#13061](https://github.com/rust-lang/cargo/issues/13061)).
See the [Automatic garbage collection] section for more details on this configuration.

[Automatic garbage collection]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#automatic-garbage-collection

### Manual cleaning

If you want to manually delete data from the cache, several options have been added under the `cargo clean gc` subcommand.
This subcommand can be used to perform the normal automatic daily cleaning, or to specify different options on which data to remove.
There are several options for specifying the age of data to delete (such as `--max-download-age=3days`) or specifying the maximum size of the cache (such as `--max-download-size=1GiB`).
See the [Manual garbage collection] section or run `cargo clean gc --help` for more details on which options are supported.

This CLI design is only preliminary, and we are looking at determining what the final design will look like when it is stabilized, see issue [#13060](https://github.com/rust-lang/cargo/issues/13060).

[Manual garbage collection]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#manual-garbage-collection-with-cargo-clean

## What to watch out for

After enabling the gc feature, just go about your normal business of using cargo.
You should be able to observe the SQLite database stored in your cargo home directory at `~/.cargo/.global-cache`.

After the first time you use cargo, it will populate the database tracking all the data that already exists in your cargo home directory.
Then, after 1 month, cargo should start deleting old data, and after 3 months will delete even more data.

The end result is that after that period of time you should start to notice the home directory using less space overall.

You can also try out the `cargo clean gc` command and explore some of its options if you want to try to manually delete some data.

If you run into problems, you can disable the gc feature and cargo should return to its previous behavior.
Please let us know on the [issue tracker][cargo-issues] if this happens.

## Request for feedback

We'd like to hear from you about your experience using this feature.
Some of the things we are interested in are:

- Have you run into any bugs, errors, issues, or confusing problems?
  Please file an issue over at <https://github.com/rust-lang/cargo/issues/>.
- The first time that you use cargo with GC enabled, is there an unreasonably long delay?
  Cargo may need to scan your existing cache data once to detect what already exists from previous versions.
- Do you notice unreasonable delays when it performs automatic cleaning once a day?
- Do you have use cases where you need to do cleaning based on the size of the cache?
  If so, please share them at [#13062](https://github.com/rust-lang/cargo/issues/13062).
- If you think you would make use of manually deleting cache data, what are your use cases for doing that?
  Sharing them on [#13060](https://github.com/rust-lang/cargo/issues/13060) about the CLI interface might help guide us on the overall design.
- Does the default of deleting 3 month old data seem like a good balance for your use cases?

Or if you would prefer to share your experiences on Zulip, head over to the [#t-cargo] stream.

[#t-cargo]: https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo

## Design considerations and implementation details

(These sections are only for the intently curious among you.)

The implementation of this feature had to consider several constraints to try to ensure that it works in nearly all environments, and doesn't introduce a negative experience for users.

### Performance

One big focus was to make sure that the performance of each invocation of `cargo` is not significantly impacted.
Cargo needs to potentially save a large chunk of data every time it runs.
The performance impact will heavily depend on the number of dependencies and your filesystem.
Preliminary testing shows the impact can be anywhere from 0 to about 50ms.

In order to minimize the performance impact of actually deleting files, the automatic GC runs only once a day.
This is intended to balance keeping the cache clean without impacting the performance of daily use.

### Locking

Another big focus is dealing with cache locking.
Previously, cargo had a single lock on the package cache, which cargo would hold while downloading registry data and performing dependency resolution.
When cargo is actually running `rustc`, it previously did not hold a lock under the assumption that existing cache data will not be modified.

However, now that cargo can modify or delete existing cache data, it needs to be careful to coordinate with anything that might be reading from the cache, such as if multiple cargo commands are run simultaneously.
To handle this, cargo now has two separate locks, which are used together to provide three separate locking states.
There is a shared read lock, which allows multiple builds to run in parallel and read from the cache.
There is a write lock held while downloading registry data, which is independent of the read lock which allows concurrent builds to still run while new packages are downloaded.
The third state is a write lock that prevents either of the two previous locks from being held, and ensures exclusive access while cleaning the cache.

Versions of cargo before 1.75 don't know about the exclusive write lock.
We are hoping that in practice it will be rare to concurrently run old and new cargo versions, and that it is unlikely that the automatic GC will need to delete data that is concurrently in use by an older version.

### Error handling and filesystems

Because we do not want problems with GC from disrupting users, the implementation silently skips the GC if it is unable to acquire an exclusive lock on the package cache.
Similarly, when cargo saves the timestamp data on every command, it will silently ignore errors if it is unable to open the database, such as if it is on a read-only filesystem, or it is unable to acquire a write lock.
This may result in the last-use timestamps becoming stale, but hopefully this should not impact most usage scenarios.
For locking, we are paying special attention to scenarios such as Docker container mounts and network filesystems with questionable locking support.

### Backwards compatibility

Since the cache is used by any version of cargo, we have to pay close attention to forwards and backwards compatibility.
We benefit from SQLite's particularly stable on-disk data format which has been stable since 2004.
Cargo has support to do schema migrations within the database that stay backwards compatible.

## Plan for the future

A major aspect of this endeavor is to gain experience with using SQLite in a wide variety of environments, with a plan to extend its usage in several other parts of cargo.

### Registry index metadata

One place where we are looking to introduce SQLite is for the registry index cache.
When cargo downloads registry index data, it stores it in a custom-designed binary file format to improve lookup performance.
However, this index cache uses many small files, which may not perform well on some filesystems.

Additionally, the index cache grows without bound.
Currently the automatic cache cleaning will only delete an entire index cache if the index itself hasn't been used, which is rarely the case for [crates.io].
We may also need to consider finer-grained timestamp tracking or some mechanism to periodically purge this data.

### Target directory change tracking and cleaning

Another place we are looking to introduce SQLite is for managing the target directory.
In cargo's target directory, cargo keeps track of information about each crate that has been built with what is called a *fingerprint*.
These fingerprints help cargo know if it needs to recompile something.
Each artifact is tracked with a set of 4 files, using a mixture of custom formats.

We are looking to replace this system with SQLite which will hopefully bring about several improvements.
A major focus will be to provide cleaning of stale data in the target directory, which tends to use substantial amount of disk space.
Additionally we are looking to implement other improvements, such as more accurate fingerprint tracking, provide information about why cargo thinks something needed to be recompiled, and to hopefully improve performance.
This will be important for the [script feature], which uses a global cache for build artifacts, and the future implementation of a globally-shared build cache.

[script feature]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script
