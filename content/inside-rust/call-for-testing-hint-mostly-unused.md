+++
path = "inside-rust/2025/07/14/call-for-testing-hint-mostly-unused"
title = "Call for Testing: Speeding up compilation with `hint-mostly-unused`"
authors = ["Josh Triplett"]
+++

I'm pleased to announce, and call for testing of, the nightly-only `rustc`
`-Zhint-mostly-unused` option, and the corresponding nightly Cargo features
`profile.hint-mostly-unused` and `hints.mostly-unused`. These options can help
accelerate your Rust compile time in some cases, by avoiding compilation of
items from your dependencies that you aren't using. Your feedback will help
evaluate these features and make progress towards stabilizing them in the
future.

## Background

Some crates provide comprehensive APIs with a very large surface area, yet many
of their users need only a few entry points. In such cases, the compiler
currently spends time generating code for the entire crate, and then ends up
throwing most of that code away.

This can waste a substantial amount of compile time. Some large crates can take
minutes to compile, and when you use these large crates as dependencies, they
can take a disproportionate amount of the entire compilation time of your
top-level crate.

In some cases, crates add feature flags to control compilation of their API
surface. This can improve compile time, but adds complexity for users, who now
need to determine which features they need for the APIs they use. Features also
constitute a stable interface of a crate, and changing feature flags can be a
breaking change. And even with feature flags, not every enabled function will
be needed; there is a balance between granularity and ease of use.

## Deferring code generation with `-Zhint-mostly-unused`

The latest nightly `rustc` compiler now supports an option
`-Zhint-mostly-unused`, which tells `rustc` that the crate's API surface will
mostly go unused. This is a hint, and `rustc` doesn't make guarantees about its
exact behavior (so that we can extend or improve it in the future), but
currently it causes the compiler to defer as much of code generation as
possible.

Applying this option to key crates you depend on (and use only a small subset
of) can provide a substantial reduction in compile time, for debug builds and
especially for release builds.

## How does this perform?

Some build timings for clean release builds of a crate depending on various
specific large API crates:

| **Dependency Crate** | **Before** | **`hint-mostly-unused`** | **Delta** |
| :- | -: | -: | -: |
| `windows`, all Graphics/UI features | 18.3s | 10.7s | -42% |
| `windows`, all features | 3m 48s | 2m 55s | -23% |
| `rustix`, `all-apis` feature | 5.9s | 4.3s | -27% |
| `x11rb` and `x11rb-protocol` | 5.3s | 2.6s | -51% |
| `aws-sdk-ec2` | 4m 07s | 2m 04s | -50% |

This performance improvement comes from deferring code generation. For
instance, the `windows` crate in the first example goes from building in 15.1s
of which 49% is codegen, to building in 7.5s of which 1% is codegen.

Note that this option does not provide a universal performance improvement for
every crate; if used when not applicable, this option can make builds much
*slower*. Deferring compilation of the items in a crate can lead to redoing
code generation for those items repeatedly. In particular, avoid using this
hint for crates whose API surface is mostly used, and/or used in multiple
different crates or binaries (e.g. multiple test binaries that each test a
substantial swath of the API). Always do performance analysis when considering
this hint, and only apply it if it applies obvious and substantial wins for
your users.

Also note that this only provides a performance win if you are rebuilding the
dependency. If you're only rebuilding the top-level crate, this won't help.

## Plumbing this through Cargo with profiles

In order to support compiling specific dependencies with this option, Cargo
supports a [profile option
`hint-mostly-unused`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#profile-hint-mostly-unused-option)
to mark a crate with this hint:

```toml
[profile.dev.package.huge-mostly-unused-dependency]
hint-mostly-unused = true

[profile.release.package.huge-mostly-unused-dependency]
hint-mostly-unused = true
```

Note that if you build in multiple profiles (e.g. the default dev profile and
the `-r` release profile), you'll want to set this flag for both, as shown
above.

Because this option is still nightly-only, and depends on a nightly-only
`rustc` option as well, enabling it requires passing
`-Zprofile-hint-mostly-unused` on the `cargo` command line. Without this
option, cargo will ignore this with a warning (but not an error, as it's still
just a hint). Note that as with any profile option, it only takes effect when
set in the top-level crate you're building.

You should not, in general, set this flag for all your dependencies, or for
your own crate; you should set it selectively and test to make sure it provides
an improvement. Using the [cargo `--timings`
option](https://doc.rust-lang.org/nightly/cargo/reference/timings.html) can
help to identify crates that might benefit from this hint. And when testing
this hint, `--timings` can help detect whether the build time of *other* crates
in the dependency tree went up.

## Making this automatic: Cargo `[hints]`

A profile hint still requires the top-level crate to configure the hint for
some of its dependencies. However, some crates know that almost all of their
users will want this hint enabled. For such crates, we've introduced a new
`hints` mechanism in Cargo. Unlike profiles, which only apply when set in the
top-level crate you build, hints are set within individual crates in your
dependency graph. Hints provide a default behavior that you can still override.

A crate that knows most of its users will not use most of its API surface can
set this hint in its `Cargo.toml` manifest:

```toml
[hints]
mostly-unused = true
```

Note that setting a hint does *not* increase the Minimum Supported Rust Version
(MSRV) of your crate. Hints are always ignored if not understood. So, you can
safely set this hint immediately, without waiting for this feature to be
stabilized, and users of nightly will immediately benefit (if they pass
`-Zprofile-hint-mostly-unused` to cargo to enable the feature).

### Future hints

The `hints` mechanism in Cargo is a general feature, and we plan to make use of
it for other purposes in the future. For instance, we may offer a
`min-opt-level` option, for crates that are so performance-sensitive (e.g.
numerics code) that most users will want to build them with optimization even
in development mode. As with other hints, such a mechanism would still always
allow the top-level crate to override.

## How do I help?

We'd love for you to test out this feature on the latest Rust nightly compiler[^nightly].

[^nightly]: Make sure to run `rustup update nightly` (or however you manage your Rust releases).

If you maintain a crate that has a large API surface, and you expect that the
typical user might use only a fraction of it, try setting `hints.mostly-unused`
in your `Cargo.toml`:

```toml
[hints]
mostly-unused = true
```

You can test the effect of this by building a *typical* crate that depends on
your crate, with and without this hint set, using nightly Cargo:
`cargo +nightly -Zprofile-hint-mostly-unused build -r`. If this provides a
noticeable performance improvement, consider setting it in your published
crate.

If you're building atop a crate that you only use a small fraction of, you can
try setting the profile option in your own crate:

```toml
[profile.dev.package.huge-mostly-unused-dependency]
hint-mostly-unused = true

[profile.release.package.huge-mostly-unused-dependency]
hint-mostly-unused = true
```

Please report any performance improvements, or unexpected performance issues,
or *especially* any failures you observe, to the [tracking issue for
profile-hint-mostly-unused](https://github.com/rust-lang/cargo/issues/15644).
When reporting, please tell us:
- What hints or profile settings you added
- What crates you added them to
- What top-level crate you're building
- What features you set when building
- What build profile you're using (e.g. the default dev profile, or the release
  profile)
- Whether you did a clean build or an incremental rebuild
- What performance numbers you got with and without the option you added

We'll take this feedback into account to fix any issues with either the rustc
compiler feature or the Cargo features, and to evaluate when those features
have seen enough testing to be ready to stabilize.

## Acknowledgements

Much appreciation to:
- [Ben Kimock](https://github.com/saethlin), whose work towards MIR-only rlibs
  provided inspiration and infrastructure for this work.
- The [Rust All Hands](https://rustweek.org/all-hands/) and its organizers, for
  providing a forum to discuss and progress this work.
