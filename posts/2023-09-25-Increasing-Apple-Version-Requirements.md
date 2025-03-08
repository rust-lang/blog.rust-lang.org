---
layout: post
title: "Increasing the minimum supported Apple platform versions"
author: BlackHoleFox
description: "Modernizing and improving Apple platform support for Rust"
---

As of Rust 1.74 (to be released on November 16th, 2023), the minimum version of Apple's platforms (iOS, macOS, and tvOS) that the Rust toolchain supports will be [increased](https://github.com/rust-lang/rust/pull/104385) to newer baselines. These changes affect both the Rust compiler itself (`rustc`), other host tooling, and most importantly, the standard library and any binaries produced that use it. With these changes in place, any binaries produced will stop loading on older versions or exhibit other, unspecified, behavior.

The new minimum versions are now:
- macOS: 10.12 Sierra (First released 2016)
- iOS: 10 (First released 2016)
- tvOS: 10 (First released 2016)

If your application does not target or support macOS 10.7-10.11 or iOS 7-9 already these changes most likely do not affect you.

# Affected targets

The following contains each affected target, and the comprehensive effects on it:
- `x86_64-apple-darwin` (Minimum OS raised)
- `aarch64-apple-ios` (Minimum OS raised)
- `aarch64-apple-ios-sim` (Minimum iOS *and macOS* version raised.)
- `x86_64-apple-ios` (Minimum iOS *and macOS* version raised. This is also a simulator target.)
- `aarch64-apple-tvos` (Minimum OS raised)
- `armv7-apple-ios` (Target removed. The oldest iOS 10-compatible device uses ARMv7s.)
- `armv7s-apple-ios` (Minimum OS raised)
- `i386-apple-ios` (Minimum OS raised)
- `i686-apple-darwin` (Minimum OS raised)
- `x86_64-apple-tvos` (Minimum tvOS *and macOS* version raised. This is also a simulator target.)

From these changes, only one target has been removed entirely: `armv7-apple-ios`. It was a tier 3 target.

Note that Mac Catalyst and M1/M2 (`aarch64`) Mac targets are not affected, as their minimum OS version already has a higher baseline. Refer to the [Platform Support Guide](https://doc.rust-lang.org/nightly/rustc/platform-support.html) for more information.

# Affected systems

These changes remove support for multiple older mobile devices (iDevices) and many more Mac systems. Thanks to `@madsmtm` for [compiling the list](https://github.com/rust-lang/rust/pull/104385#issuecomment-1317830217).

As of this update, the following device models are no longer supported by the latest Rust toolchain:

### iOS

-   iPhone 4S (Released in 2011)
-   iPad 2 (Released in 2011)
-   iPad, 3rd generation (Released in 2012)
-   iPad Mini, 1st generation (Released in 2012)
-   iPod Touch, 5th generation (Released in 2012)

### macOS

A total of 27 Mac system models, released between 2007 and 2009, are no longer supported.

The affected systems are not comprehensively listed here, but external resources exist which contain lists of the exact models. They can be found [from Apple](https://support.apple.com/kb/SP742?locale=en_US) and [Yama-Mac](https://yama-mac.com/en/macos_correspondence_table/#toc4), for example.

### tvOS

The third generation AppleTV (released 2012-2013) is no longer supported.

# Why are the requirements being changed?

Prior to now, Rust claimed support for very old Apple OS versions, but many never even received passive testing or support. This is a rough place to be for a toolchain, as it hinders opportunities for improvement in exchange for a support level many people, or everyone, will never utilize. For Apple's mobile platforms, many of the old versions are now even unable to receive new software due to App Store publishing restrictions.

Additionally, the past two years have clearly indicated that Apple, which has tight control over toolchains for these targets, is making it difficult-to-impossible to support them anymore. As of XCode 14, last year's toolchain release, building for many old OS versions [became unsupported](https://developer.apple.com/documentation/xcode-release-notes/xcode-14-release-notes). XCode 15 continues this trend. After enough time, continuing to use an older toolchain can even lead to breaking build issues for others.

We want Rust to be a first-class option for developing software for and on Apple's platforms, but to continue this goal we have to set an easier, and more realistic compatibility baseline. The new requirements were determined after surveying what Apple and third-party statistics are available to us and picking a middle ground that balances compatibility with Rusts's needs and limitations.

# Do I need to do anything?

If you or an application you develop are affected by this change, there are different options which may be helpful:
- If possible, raise your minimum supported OS version(s). All OS versions discussed in this post have no support from the vendor. Not even security updates.
-  If you are running the Rust compiler or other previously-supported host tools, consider cross-compiling from a newer host instead. You may also no longer be able to depend on the Rust standard library.
- If none of these options work, you may need to freeze the version of the Rust toolchain your project builds with. Alternatively, you may be able to maintain a custom toolchain that supports your requirements in any sub-component of it (such as libstd).

If your project does not directly support a specific OS version, but instead depends on a default version previously used by Rust, there are some steps you can take
to help improve future compatibility. For example, a number of crates in the ecosystem have hardcoded Rust's previously supported OS baseline versions since they haven't changed for a long time:
- If you use the `cc` crate to include other languages in your project, a [future update](https://github.com/rust-lang/cc-rs/pull/848) will handle this transparently.
- If you need a minimum OS version for anything else, crates should query the new `rustc --print deployment-target` option for a default, or user-set when available, value on toolchains using Rust 1.71 or newer going forward. Hardcoded defaults should only be used for older toolchains where this is unavailable.