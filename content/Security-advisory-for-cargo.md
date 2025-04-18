+++
path = "2019/09/30/Security-advisory-for-cargo"
title = "Security advisory for Cargo"
authors = ["The Rust Security Team"]
aliases = ["2019/09/30/Security-advisory-for-cargo.html"]
+++

> **Note**: This is a cross-post of the [official security advisory]. The official
> post contains a signed version with our PGP key, as well.

The Rust team was recently notified of a security concern when using older versions of Cargo to build crates which use the package rename feature added in newer versions of Cargo. If you're using Rust 1.26.0, released on 2018-05-10, or later you're not affected.

The CVE for this vulnerability is [CVE-2019-16760][0].

## Overview

Cargo can be configured through `Cargo.toml` and the `[dependencies]` section to depend on different crates, such as those from crates.io. There are multiple ways to configure how you depend on crates as well, for example if you depend on `serde` and enable the `derive` feature it would look like:

```toml
serde = { version = "1.0", features = ['derive'] }
```

Rust 1.31.0 [introduced a new feature of Cargo][1] where one of the optional keys you can specify in this map is `package`, a way to [rename a crate locally][2]. For example if you preferred to use `serde1` locally instead of `serde`, you could write:

```toml
serde1 = { version = "1.0", features = ['derive'], package = "serde" }
```

It's the addition of the `package` key that causes Cargo to compile the crate differently. This feature was [first implemented][3] in Rust 1.26.0, but it was unstable at the time. For Rust 1.25.0 and prior, however, Cargo would ignore the `package` key and interpret the dependency line as if it were:

```toml
serde1 = { version = "1.0", features = ['derive'] }
```

This means when compiled with Rust 1.25.0 and prior then it would attempt to download the `serde1` crate. A malicious user could squat the `serde1` name on crates.io to look like `serde 1.0.0` but instead act maliciously when built.

In summary, usage of the `package` key to rename dependencies in `Cargo.toml` is ignored in Rust 1.25.0 and prior. When Rust 1.25.0 and prior is used Cargo will ignore `package` and download the wrong dependency, which could be squatted on crates.io to be a malicious package. This not only affects manifests that you write locally yourself, but also manifests published to crates.io. If you published a crate, for example, that depends on `serde1` to crates.io then users who depend on you may also be vulnerable if they use Rust 1.25.0 and prior.

## Affected Versions

Rust 1.0.0 through Rust 1.25.0 is affected by this advisory because Cargo will ignore the `package` key in manifests. Rust 1.26.0 through Rust 1.30.0 are not affected and typically will emit an error because the `package` key is unstable. Rust 1.31.0 and after are not affected because Cargo understands the `package` key.

In terms of Cargo versions, this affects Cargo up through Cargo 0.26.0. All future versions of Cargo are unaffected.

## Mitigations

We strongly recommend that users of the affected versions update their compiler to the latest available one. Preventing this issue from happening requires updating your compiler to either Rust 1.26.0 or newer.

We will not be issuing a patch release for Rust versions prior to 1.26.0. Users of Rust 1.19.0 to Rust 1.25.0 can instead apply [the provided patches][4] to mitigate the issue.

An audit of existing crates published to crates.io using the `package` key has been performed and there is no evidence that this vulnerability has been exploited in the wild. Our audit only covers the crates currently published on crates.io: if you notice crates exploiting this vulnerability in the future please don't hesitate to email security@rust-lang.org in accordance with [our security policy][5].

## Timeline of events

* Wed, Sep 18, 2019 at 13:54 UTC - Bug reported to security@rust-lang.org
* Wed, Sep 18, 2019 at 15:35 UTC - Response confirming the report
* Wed, Sep 18, 2019 - Cargo, Core, and crates.io teams confer on how best to handle this
* Thu, Sep 19, 2019 - Confirmed with Elichai plan of action and continued to audit existing crates
* Mon, Sep 23, 2019 - Advisory drafted, patches developed, audit completed
* Mon, Sep 30, 2019 - Advisory published, security list informed of this issue

## Acknowledgments

Thanks to Elichai Turkel, who found this bug and reported it to us in accordance
with our [security policy][5].

[0]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2019-16760
[1]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#cargo-features
[2]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml
[3]: https://github.com/rust-lang/cargo/pull/4953
[4]: https://gist.github.com/pietroalbini/0d293b24a44babbeb6187e06eebd4992
[5]: https://www.rust-lang.org/policies/security
[official security advisory]: https://groups.google.com/forum/#!topic/rustlang-security-announcements/rVQ5e3TDnpQ
