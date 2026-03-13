+++
path = "9999/12/31/increasing-the-minimum-supported-ptx-isa-version-and-gpu-architecture-for-the-nvptx-target"
title = "Increasing the minimum supported PTX ISA version and GPU architecture for the nvptx target"
authors = ["Kjetil Kjeka"]
+++

The `nvptx64-nvidia-cuda` target is a compilation target for NVIDIA GPUs. When using this target, the final output is PTX with (at least) two important version choices:
- a GPU architecture (for example `sm_70`, `sm_80`, …), which determines which GPUs can run the PTX and  
- a PTX ISA version, which determines which CUDA driver versions can load (and JIT-compile) the PTX.

In Rust 1.96 (scheduled for release on 28 May 2026), the minimum supported PTX ISA version and GPU architecture for `nvptx64-nvidia-cuda` will be [increased](https://github.com/rust-lang/compiler-team/issues/965). These changes affect both the Rust compiler (`rustc`) and related host tooling, and they make it impossible to generate PTX artifacts compatible with older GPUs and older CUDA drivers.

The new minimum supported versions will be:
- **PTX ISA 7.0** (requires a CUDA 11 driver or newer)  
- **SM 7.0** (GPUs with compute capability below 7.0 are no longer supported)

## Why are the requirements being changed?

Until now, Rust has supported emitting PTX for a wide range of GPU architectures and PTX ISA versions. In practice, several defects existed that could cause valid Rust code to trigger compiler crashes or miscompilations. Removing support for older hardware and software is considered the best path forward for resolving these issues.

## What happens when I update to Rust 1.96?

If you need to target a CUDA driver that does not support PTX ISA 7.0 (CUDA 10–era drivers and older), Rust 1.96 will no longer be able to generate PTX compatible with that environment. Similarly, if you need to run on GPUs with compute capability below 7.0 (for example Maxwell or Pascal), Rust 1.96 will no longer be able to generate compatible PTX for those GPUs.

Assuming you are targeting a CUDA driver compatible with CUDA 11 or newer and using GPUs with compute capability 7.0 or newer:
- If you do **not** specify `-C target-cpu`, the new default will be `sm_70`, and your build should continue to work (but will no longer be compatible with pre-Volta GPUs).
- If you currently specify an older `-C target-cpu` (for example `sm_60`), you will need to either:
  - remove that flag and let it default to `sm_70`, or
  - update it to `sm_70` or a newer architecture.
- If you already specify `-C target-cpu=sm_70` (or newer), there should be no behavioural changes from this update.

For more details on building and configuring `nvptx64-nvidia-cuda`, see [the platform support documentation](https://doc.rust-lang.org/rustc/platform-support/nvptx64-nvidia-cuda.html)