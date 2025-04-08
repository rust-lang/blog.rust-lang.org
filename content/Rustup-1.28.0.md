+++
path = "2025/03/02/Rustup-1.28.0"
title = "Announcing Rustup 1.28.0"
authors = ["The Rustup Team"]
aliases = ["2025/03/02/Rustup-1.28.0.html"]
+++

The rustup team is happy to announce the release of rustup version 1.28.0.
[Rustup][install] is the recommended tool to install [Rust][rust], a programming language that is empowering everyone to build reliable and efficient software.

## What's new in rustup 1.28.0

This new release of rustup has been a long time in the making and comes with substantial changes.

Before digging into the details, it is worth mentioning that [Chris Denton](https://github.com/chrisdenton) has joined the team.
Chris has a lot of experience contributing to Windows-related parts of the Rust Project -- expertise we were previously lacking -- so we're happy to have him on board to help address Windows-specific issues.

The following improvements might require changes to how you use rustup:

- rustup will no longer automatically install the active toolchain if it is not installed.

  - To ensure its installation, run `rustup toolchain install` with no arguments.
  - The following command installs the active toolchain both before and after this change:
    ```sh
    rustup show active-toolchain || rustup toolchain install
    # Or, on older versions of PowerShell:
    rustup show active-toolchain; if ($LASTEXITCODE -ne 0) { rustup toolchain install }
    ```

- Installing a host-incompatible toolchain via `rustup toolchain install` or `rustup default` will
  now be rejected unless you explicitly add the `--force-non-host` flag.

Rustup now officially supports the following host platforms:

- `aarch64-pc-windows-msvc`
- `loongarch64-unknown-linux-musl`

This release also comes with various quality-of-life improvements, to name a few:

- `rustup show`'s output format has been cleaned up, making it easier to find out about your toolchains' status.
- `rustup doc` now accepts a flag and a topic at the same time, enabling quick navigation to specific parts of more books.
- rustup's `remove` subcommands now support more aliases such as `rm` and `del`.
- Basic support for nushell has been added.

We have additionally made the following internal changes:

- The default download backend has been changed from reqwest with native-tls to reqwest with rustls.
  - `RUSTUP_USE_CURL` and `RUSTUP_USE_RUSTLS` can still be used to change the download backend
    if the new backend causes issues. If issues do happen, please [let us know](https://github.com/rust-lang/rustup/issues/3806).
  - The default backend now uses rustls-platform-verifier to verify server certificates, taking
    advantage of the platform's certificate store on platforms that support it.
- When creating proxy links, rustup will now try symlinks first and fall back to hardlinks,
  as opposed to trying hardlinks first.
- A new `RUSTUP_LOG` environment variable can be used to control tracing-based logging in
  rustup binaries. See the [dev guide](https://rust-lang.github.io/rustup/dev-guide/tracing.html) for more details.

Finally, there are some notable changes to our [official website][install] as well:

- The overall design of the website has been updated to better align with the Rust Project's branding.
- It is now possible to download the prebuilt `rustup-init.sh` installer for the `aarch64-pc-windows-msvc` host platform via https://win.rustup.rs/aarch64.

Further details are available in the [changelog]!

## How to update

If you have a previous version of rustup installed, getting rustup 1.28.0 is as easy as stopping any programs which may be using Rustup (e.g. closing your IDE) and running:

```
$ rustup self update
```

Rustup will also automatically update itself at the end of a normal toolchain update:

```
$ rustup update
```

If you don't have it already, you can [get rustup][install] from the appropriate page on our website.

Rustup's documentation is also available in [the rustup book][book].

## Caveats

Rustup releases can come with problems not caused by rustup itself but just due to having a new release.
As such, we recommend paying attention to the following potential issues in particular:

- Anti-malware scanners might be blocking rustup or stopping it from creating or copying files
  (especially when installing `rust-docs`, since it contains many small files).

- In your CI environment, rustup might fail when trying to perform a self-update.

  This is a [known issue](https://github.com/rust-lang/rustup/issues/3709),
  and in the case where this issue does occur, we recommend applying the following workaround at the beginning of your workflow:

  ```
  $ rustup set auto-self-update disable
  ```

  Also, starting from 1.28.0, rustup will no longer attempt to self-update in CI environments,
  so this workaround should not be necessary in the future.

These issues should be automatically resolved in a few weeks when the anti-malware scanners are updated to be aware of the new rustup release,
and the hosted version is updated across all CI runners.

## Thanks

Thanks again to all the [contributors] who made rustup 1.28.0 possible!

[book]: https://rust-lang.github.io/rustup/
[changelog]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md
[contributors]: https://github.com/rust-lang/rustup/blob/stable/CHANGELOG.md#detailed-changes
[install]: https://rustup.rs
[rust]: https://www.rust-lang.org
