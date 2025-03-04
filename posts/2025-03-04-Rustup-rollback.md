---
layout: post
title: "Rolling back Rustup 1.28.0"
author: The Rustup Team
---

Due to unexpected impact to many CI systems not being ready for the new
rustup-toolchain.toml behavior, we have rolled back the rustup [1.28.0] release.
Rustup should self-update back to 1.27.1, and new installations will start to
install 1.27.1 again.

The Rustup team will follow up with a more concrete plan for rolling out the
changes in 1.28, but we're mitigating impact through a rollback for now.

[1.28.0]: /2025/03/02/Rustup-1.28.0.html
