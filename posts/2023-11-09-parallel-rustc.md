+++
layout = "post"
title = "Faster compilation with the parallel front-end in nightly"
author = "Nicholas Nethercote"
team = "The Parallel Rustc Working Group <https://www.rust-lang.org/governance/teams/compiler#Parallel%20rustc%20working%20group>"
+++

The Rust compiler's front-end can now use parallel execution to significantly
reduce compile times. To try it, run the nightly compiler with the `-Z
threads=8` option. This feature is currently experimental, and we aim to ship
it in the stable compiler in 2024.

Keep reading to learn why a parallel front-end is needed and how it works, or
just skip ahead to the [How to use it](parallel-rustc.html#how-to-use-it)
section.

## Compile times and parallelism

Rust compile times are a perennial concern. The [Compiler Performance Working
Group](https://www.rust-lang.org/governance/teams/compiler#Compiler%20performance%20working%20group)
has continually improved compiler performance for several years. For example,
in the first 10 months of 2023, there were mean reductions in compile time of
[13%](https://perf.rust-lang.org/compare.html?start=2023-01-01&end=2023-10-31&stat=wall-time&nonRelevant=true),
in peak memory use of
[15%](https://perf.rust-lang.org/compare.html?start=2023-01-01&end=2023-10-31&stat=max-rss&nonRelevant=true),
and in binary size of
[7%](https://perf.rust-lang.org/compare.html?start=2023-01-01&end=2023-10-31&stat=size%3Alinked_artifact&nonRelevant=true),
as measured by our performance suite.

However, at this point the compiler has been heavily optimized and new
improvements are hard to find. There is no low-hanging fruit remaining. 

But there is one piece of large but high-hanging fruit: parallelism. Current
Rust compiler users benefit from two kinds of parallelism, and the newly
parallel front-end adds a third kind.

### Existing interprocess parallelism

When you compile a Rust program, Cargo launches multiple rustc processes,
compiling multiple crates in parallel. This works well. Try compiling a large
Rust program with the `-j1` flag to disable this parallelization and it will
take a lot longer than normal.

You can visualise this parallelism if you build with Cargo's
[`--timings`](https://doc.rust-lang.org/cargo/reference/timings.html) flag,
which produces a chart showing how the crates are compiled. The following image
shows the timeline when building [ripgrep](https://crates.io/crates/ripgrep) on
a machine with 28 virtual cores.

![`cargo build --timings` output when compiling ripgrep](../../../images/2023-11-09-parallel-rustc/cargo-build-timings.png)

There are 60 horizontal lines, each one representing a distinct process. Their
durations range from a fraction of a second to multiple seconds. Most of them
are rustc, and the few orange ones are build scripts. The first twenty processes all
start at the same time. This is possible because there are no dependencies
between the relevant crates. But further down the graph, parallelism reduces as
crate dependencies increase. Although the compiler can overlap compilation of
dependent crates somewhat thanks to a feature called [pipelined
compilation](https://github.com/rust-lang/rust/issues/60988), there is much
less parallel execution happening towards the end of compilation, and this is
typical for large Rust programs. Interprocess parallelism is not enough to take
full advantage of many cores. For more speed, we need parallelism within each process.

### Existing intraprocess parallelism: the back-end

The compiler is split into two halves: the front-end and the back-end.

The front-end does many things, including parsing, type checking, and borrow
checking. Until this week, it could not use parallel execution.

The back-end performs code generation. It generates code in chunks called
"codegen units" and then LLVM processes these in parallel. This is a form of
coarse-grained parallelism. 

We can visualize the difference between the serial front-end and the parallel
back-end. The following image shows the output of a profiler called
[Samply](https://github.com/mstange/samply/) measuring rustc as it does a
release build of the final crate in Cargo. The image is superimposed with
markers that indicate front-end and back-end execution.

![Samply output when compiling Cargo, serial](../../../images/2023-11-09-parallel-rustc/samply-serial.png)

Each horizontal line represents a thread. The main thread is labelled "rustc"
and is shown at the bottom. It is busy for most of the execution. The other 16
threads are LLVM threads, labelled "opt cgu.00" through to "opt cgu.15". There
are 16 threads because 16 is the default number of codegen units for a release
build.

There are several things worth noting.
- Front-end execution takes 10.2 seconds.
- Back-end execution takes 6.2 seconds, and the LLVM threads are running
  for 5.9 seconds of that.
- The parallel code generation is highly effective. Imagine if all those LLVM
  executed one after another!
- Even though there are 16 LLVM threads, at no point are all 16 executing at
  the same time, despite this being run on a machine with 28 cores. (The peak
  is 14 or 15.) This is because the main thread translates its internal code
  representation (MIR) to LLVM's code representation (LLVM IR) in serial. This
  takes a brief period for each codegen unit, and explains the staircase shape
  on the left-hand side of the code generation threads. There is some room for
  improvement here.
- The front-end is entirely serial. There is a lot of room for improvement
  here.

### New intraprocess parallelism: the front-end

The front-end is now capable of parallel execution. It uses
[Rayon](https://crates.io/crates/rayon) to perform compilation tasks using
fine-grained parallelism. Many data structures are synchronized by mutexes and
read-write locks, atomic types are used where appropriate, and many front-end
operations are made parallel. The addition of parallelism was done by modifying
a relatively small number of key points in the code. The vast majority of the
front-end code did not need to be changed.

When the parallel front-end is enabled and configured to use eight threads, we
get the following Samply profile when compiling the same example as before.

![Samply output when compiling Cargo, parallel](../../../images/2023-11-09-parallel-rustc/samply-parallel.png)

Again, there are several things worth noting.
- Front-end execution takes 5.9 seconds (down from 10.2 seconds).
- Back-end execution takes 5.3 seconds (down from 6.2 seconds), and the LLVM
  threads are running for 4.9 seconds of that (down from 5.9 seconds).
- There are seven additional threads labelled "rustc" operating in the
  front-end. The reduced front-end time shows they are reasonably effective,
  but the thread utilization is patchy, with the eight threads all having
  periods of inactivity. There is room for significant improvement here.
- Eight of the LLVM threads start at the same time. This is because the eight
  "rustc" threads create the LLVM IR for eight codegen units in parallel. (For
  seven of those threads that is the only work they do in the back-end.) After
  that, the staircase effect returns because only one "rustc" thread does LLVM
  IR generation while seven or more LLVM threads are active. If the number of
  threads used by the front-end was changed to 16 the staircase shape would
  disappear entirely, though in this case the final execution time would barely
  change.

### Putting it all together

Rust compilation has long benefited from interprocess parallelism, via Cargo,
and from intraprocess parallelism in the back-end. It can now also benefit from
intraprocess parallelism in the front-end.

You might wonder how interprocess parallelism and intraprocess parallelism
interact. If we have 20 parallel rustc invocations and each one can have up to
16 threads running, could we end up with hundreds of threads on a machine with
only tens of cores, resulting in inefficient execution as the OS tries its best
to schedule them?

Fortunately no. The compiler uses the [jobserver
protocol](https://www.gnu.org/software/make/manual/html_node/POSIX-Jobserver.html)
to limit the number of threads it creates. If a lot of interprocess parallelism
is occuring, intraprocess parallelism will be limited appropriately, and
the number of threads will not exceed the number of cores.

## How to use it

The nightly compiler is now [shipping with the parallel front-end
enabled](https://github.com/rust-lang/rust/pull/117435). However, **by default
it runs in single-threaded mode** and won't reduce compile times.

Keen users can opt into multi-threaded mode with the `-Z threads` option. For
example:
```
$ RUSTFLAGS="-Z threads=8" cargo build --release
```
Alternatively, to opt in from a
[config.toml](https://doc.rust-lang.org/cargo/reference/config.html) file (for
one or more projects), add these lines:
```
[build]
rustflags = ["-Z", "threads=8"]
```
It may be surprising that single-threaded mode is the default. Why parallelize
the front-end and then run it in single-threaded mode? The answer is simple:
caution. This is a big change! The parallel front-end has a lot of new code.
Single-threaded mode exercises most of the new code, but excludes the
possibility of threading bugs such as deadlocks that can affect multi-threaded
mode. Even in Rust, parallel programs are harder to write correctly than serial
programs. For this reason the parallel front-end also won't be shipped in beta
or stable releases for some time.

### Performance effects

When the parallel front-end is run in single-threaded mode, compilation times
are typically 0% to 2% slower than with the serial front-end. This should be
barely noticeable.

When the parallel front-end is run in multi-threaded mode with `-Z threads=8`,
our [measurements on real-world
code](https://github.com/rust-lang/compiler-team/issues/681) show that compile
times can be reduced by up to 50%, though the effects vary widely and depend on
the characteristics of the code and its build configuration. For example, dev
builds are likely to see bigger improvements than release builds because
release builds usually spend more time doing optimizations in the back-end. A
small number of cases compile more slowly in multi-threaded mode than
single-threaded mode. These are mostly tiny programs that already compile
quickly.

We recommend eight threads because this is the configuration we have tested the
most and it is known to give good results. Values lower than eight will see
smaller benefits, but are appropriate if your hardware has fewer than eight
cores. Values greater than eight will give diminishing returns and may even
give worse performance.

If a 50% improvement seems low when going from one to eight threads, recall
from the explanation above that the front-end only accounts for part of compile
times, and the back-end is already parallel. You can't beat [Amdahl's
Law](https://en.wikipedia.org/wiki/Amdahl%27s_law).

Memory usage can increase significantly in multi-threaded mode. We have seen
increases of up to 35%. This is unsurprising given that various parts of
compilation, each of which requires a certain amount of memory, are now
executing in parallel. 

### Correctness

Reliability in single-threaded mode should be high.

In multi-threaded mode there are some known bugs, including deadlocks. If
compilation hangs, you have probably hit one of them.

The binaries produced by the compiler are expected to be the same no matter
which front-end is being used. Any differences will be considered a bug.

### Feedback

If you have any problems with the parallel front-end, please [check the issues
marked with the "WG-compiler-parallel"
label](https://github.com/rust-lang/rust/labels/WG-compiler-parallel).
If your problem does not match any of the existing issues, please file a new
issue.

For more general feedback, please start a discussion on the [wg-parallel-rustc
Zulip
channel](https://rust-lang.zulipchat.com/#narrow/stream/187679-t-compiler.2Fwg-parallel-rustc).
We are particularly interested to hear the performance effects on the code you
care about.

# Future work

We are working to improve the performance of the parallel front-end. As the
graphs above showed, there is room to improve the utilization of the threads in
the front-end. We are also ironing out the remaining bugs in multi-threaded
mode.

We aim to stabilize the `-Z threads` option and ship the parallel front-end
running by default in multi-threaded mode on stable releases in 2024.

# Acknowledgments

The parallel front-end has been under development for a long time. It was
started by [@Zoxc](https://github.com/Zoxc/), who also did most of the work for
several years. After a period of inactivity, the project was revived this year
by [@SparrowLii](https://github.com/sparrowlii/), who led the effort to get it
shipped. Other members of the Parallel Rustc Working Group have also been
involved with reviews and other activities. Many thanks to everyone involved.
