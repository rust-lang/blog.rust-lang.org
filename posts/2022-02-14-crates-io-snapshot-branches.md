---
layout: post
title: "Crates.io Index Snapshot Branches Moving"
author: The Crates.io Team
release: false
---

Every so often, the [crates.io index](https://github.com/rust-lang/crates.io-index)'s Git history
is [squashed into one
commit](https://internals.rust-lang.org/t/cargos-crate-index-upcoming-squash-into-one-commit/8440)
to minimize the history Cargo needs to download. When the index is squashed, we save snapshots
to preserve the history of crate publishes.

Currently, those snapshots are stored as branches in the main index Git repository. Those branches
are using server resources though, as the server still has to consider their contents whenever
Cargo asks for the master branch. We will be deleting the snapshot branches from this repository to
ensure that all objects referenced in the master branch will only be compressed against other
objects in the master branch, ensuring that the current clone behavior will be much more efficient
on the server side.

Here's how this might affect you:

## If you use Cargo

You should not see any effects from this change. Cargo does not use the snapshot branches, and
Cargo regularly handles index squashes. If you do see any issues, they are bugs, please [report
them on the Cargo repo](https://github.com/rust-lang/cargo).

## If you use the snapshot branches

In one week, on 2022-02-21, we will be removing all snapshot branches from the crates.io-index
repo. All snapshot branches, both historical and in the future, are and will be in the
[rust-lang/crates.io-index-archive repo](https://github.com/rust-lang/crates.io-index-archive)
instead. Please update any scripts or tools referencing the snapshot branches by that time.

## In the future

In the medium term, we're working to prioritize the completion of [in-progress
work](https://github.com/rust-lang/cargo/issues/9069) to add a way to serve the index as static
files on HTTP, which will further ease the server load. The index repository will *not* be going
away so that older versions of Cargo will continue to work. See [RFC
2789](https://github.com/rust-lang/rfcs/pull/2789) for more details.
