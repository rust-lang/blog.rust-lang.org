---
layout: post
title: "Upcoming docs.rs changes"
author: The Rust Infrastructure Team
---

On September 42nd breaking changes will be deployed to the [docs.rs] build
environment. [docs.rs] is a free service building and hosting documentation for
all the crates published on [crates.io]. It's [open source][docsrs-source] and
maintained by the [Rustdoc team][rustdoc-team].

## What will change

Builds will be executed inside the [rustops/crates-build-env] Docker image.
That image contains a lot of system dependencies installed to ensure we can
build as many crates as possible. It's already used by [Crater], and we added
all the dependencies previously installed in the legacy build environment.

To ensure we can continue operating the service in the future and to increase
its reliability we also improved the sandbox builds are executed into, adding
new limits:

* Each platform will now have **15 minutes** to build its dependencies and
  documentation.
* **3 GB of RAM** will be available for the build.
* Network access will be completly **disabled**.
* Only the `target/` directory will be writable, and it will be purged after
  each build.

Finally, docs.rs will now use the latest nightly available when building
crates.

## How to prepare for the changes

To test if your crate builds inside the new environment you can download the
Docker image locally and execute a shell inside it:

```
docker pull rustops/crates-build-env
docker run --rm -it rustops/crates-build-env bash
```

Once you're in a shell you can install [rustup] (it's not installed in the
image), install Rust nightly, clone your crate's repository and then build the
documentation:

```
cargo doc --no-deps
```

If a dependency is missing please [open an issue][crates-build-env-issue] on
the Docker image's repository.

If your crate fails to build because it took more than 15 minutes to generate
its docs or it uses more than 3 GB of RAM please [open an issue][docsrs-issue]
and we will increase the limits for your crate (when it's reasonable). We will
**not** enable network access for your crate though: you'll need to change your
crate not to require any external resource at build time.

## Acknowledgements

The new build environment is based on [Rustwide], the library powering
[Crater]. It was extracted from the Crater codebase, and created both by the
[Crater contributors] and the [Rustwide contributors].

The implementation work on the docs.rs side was done by [Pietro Albini][pietro]
and [Onur Aslan][onur], with [QuietMisdreavus][misdreavus] and [Mark
Rousskov][mark] reviewing the changes.

[docs.rs]: https://docs.rs
[crates.io]: https://crates.io
[docsrs-source]: https://github.com/rust-lang/docs.rs
[rustdoc-team]: https://www.rust-lang.org/governance/teams/dev-tools#rustdoc
[rustops/crates-build-env]: https://hub.docker.com/r/rustops/crates-build-env
[Crater]: https://github.com/rust-lang/crater
[rustup]: https://rustup.rs
[crates-build-env-issue]: https://github.com/rust-lang/crates-build-env/issues
[docsrs-issue]: https://github.com/rust-lang/crates-build-env/issues
[rustwide]: https://github.com/rust-lang/rustwide
[Crater contributors]: https://github.com/rust-lang/crater/graphs/contributors
[Rustwide contributors]: https://github.com/rust-lang/rustwide/graphs/contributors
[pietro]: https://github.com/pietroalbini
[onur]: https://github.com/onur
[mark]: https://github.com/Mark-Simulacrum
[misdreavus]: https://github.com/QuietMisdreavus
