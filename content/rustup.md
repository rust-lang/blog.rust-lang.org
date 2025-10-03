+++
path = "2016/05/13/rustup"
title = "Taking Rust everywhere with rustup"
authors = ["Brian Anderson"]
description = "The rustup toolchain manager makes cross-compilation in Rust a breeze"
aliases = ["2016/05/13/rustup.html"]
+++

*Cross-compilation* is an imposing term for a common kind of desire:

* You want to build an app for Android, or iOS, or your router using your laptop.

* You want to write, test and build code on your Mac, but deploy it to your Linux server.

* You want your Linux-based build servers to produce binaries for all the platforms
  you ship on.

* You want to build an ultraportable binary you can ship to any Linux platform.

* You want to target the browser with [Emscripten] or [WebAssembly].

[Emscripten]: https://kripken.github.io/emscripten-site/
[WebAssembly]: https://webassembly.github.io/

In other words, **you want to develop/build on one "host" platform, but get a
final binary that runs on a different "target" platform**.

Thanks to the [LLVM] backend, it's always been possible *in principle*
to cross-compile Rust code: just tell the backend to use a different
target! And indeed, intrepid hackers have put Rust on embedded systems
[like the Raspberry Pi 3], [bare metal ARM], [MIPS routers running
OpenWRT][OpenWRT], and many others.

[LLVM]: https://llvm.org
[like the Raspberry Pi 3]: https://mirrors.link/posts/cross-compiling-rust-on-os-x-for-raspberry-pi-3
[bare metal ARM]: https://blog.thiago.me/raspberry-pi-bare-metal-programming-with-rust/
[OpenWRT]: https://github.com/japaric/rust-on-openwrt

But in practice, there are a lot of ducks you have to get in a row to
make it work: the appropriate Rust standard library, a cross-compiling
C toolchain including linker, headers and binaries for C libraries,
and so on. This typically involves poring over various blog posts
and package installers to get everything "just so". And the exact set
of tools can be different for every pair of host and target platforms.

**The Rust community has been hard at work toward the goal of "push-button
cross-compilation"**. We want to provide a complete setup for a given
host/target pair with the run of a single command. Today we're happy to announce
that a major portion of this work is reaching beta status: we're building
binaries of the Rust standard library for a wide range of targets, and shipping
them to you via a new tool called **[rustup]**.

[rustup]: https://www.rustup.rs

## Introducing rustup

At its heart, **rustup is a *toolchain manager* for Rust**. It can
download and switch between copies of the Rust compiler and standard
library for all supported platforms, and track Rust's nightly, beta,
and release [channels], as well as specific versions. In this way
rustup is similar to the [rvm], [rbenv] and [pyenv] tools for Ruby and
Python. I'll walk through all of this functionality, and the
situations where it's useful, in the rest of the post.

[rvm]: https://rvm.io/
[rbenv]: https://github.com/rbenv/rbenv
[pyenv]: https://github.com/yyuu/pyenv

Today rustup is a command line application, and I'm going to show you some
examples of what it can do, but it's also a [Rust library], and eventually these
features are expected to be presented through a graphical interface where
appropriate &mdash; particularly on Windows. Getting cross-compilation set up should
eventually be a matter of checking a box in the Rust installer.

Our ambitions go beyond managing just the Rust toolchain: to have a
true push-button experience for cross-compilation, it needs to set up
the C toolchain as well. That functionality is not shipping today, but
it's something we hope to incorporate over the next few months.

[channels]: https://blog.rust-lang.org/2014/10/30/Stability.html
[Rust library]: https://github.com/rust-lang-nursery/rustup.rs/

## Basic toolchain management

Let's start with something simple: installing multiple Rust toolchains. In this
example I create a new library, 'hello', then test it using rustc 1.8, then use
rustup to install and test that same crate on the 1.9 beta.

```
$ cargo new hello && cd hello
$ rustc --version
rustc 1.8.0 (db2939409 2016-04-11)
$ cargo test
   Compiling hello v0.1.0 (file:///home/user/hello)
     Running target/debug/hello-b4f774924ded32e4

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests hello

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
$ rustup install beta
info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'
info: latest update on 2016-04-11, rust version 1.9.0-beta (e4e8b6668 2016-04-11)
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: installing component 'rust-std'
info: installing component 'rustc'

  beta-x86_64-unknown-linux-gnu installed - rustc 1.9.0-beta (e4e8b6668 2016-04-11)

$ rustup run beta rustc --version
rustc 1.9.0-beta (e4e8b6668 2016-04-11)
$ rustup run beta cargo test
   Compiling hello v0.1.0 (file:///home/user/hello)
     Running target/debug/hello-f4f25bc615ec63f5

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests hello

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

That's an easy way to verify your code works on the next Rust release. That's
good Rust citizenship!

We can use `rustup show` to show us the installed toolchains, and `rustup
update` to keep them up to date with Rust's releases.

```
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/user/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
beta-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.8.0 (db2939409 2016-04-11)

$ rustup update
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'

   stable-x86_64-unknown-linux-gnu unchanged - rustc 1.8.0 (db2939409 2016-04-11)
     beta-x86_64-unknown-linux-gnu unchanged - rustc 1.9.0-beta (e4e8b6668 2016-04-11)

info: cleaning up downloads & tmp directories
```

Finally, rustup can also change the default toolchain with `rustup default`:

```
$ rustc --version
rustc 1.8.0 (db2939409 2016-04-11)
$ rustup default 1.7.0
info: syncing channel updates for '1.7.0-x86_64-unknown-linux-gnu'
info: downloading component 'rust'
info: installing component 'rust'
info: default toolchain set to '1.7.0-x86_64-unknown-linux-gnu'

  1.7.0-x86_64-unknown-linux-gnu installed - rustc 1.7.0 (a5d1e7a59 2016-02-29)

$ rustc --version
rustc 1.7.0 (a5d1e7a59 2016-02-29)
```

On Windows, [where Rust supports both the GNU and MSVC ABI][abi], you
might want to switch from the default stable toolchain on Windows,
which targets the 32-bit x86 architecture and the GNU ABI, to a
stable toolchain that targets the 64-bit, MSVC ABI.

[abi]: https://www.rust-lang.org/downloads.html#win-foot

```
$ rustup default stable-x86_64-pc-windows-msvc
info: syncing channel updates for 'stable-x86_64-pc-windows-msvc'
info: downloading component 'rustc'
info: downloading component 'rust-std'
...

  stable-x86_64-pc-windows-msvc installed - rustc 1.8.0-stable (db2939409 2016-04-11)

```

Here the "stable" toolchain name is appended with an extra identifier indicating
the compiler's architecture, in this case `x86_64-pc-windows-msvc`. This
identifier is called a "target triple": "target" because it specifies a platform
for which the compiler generates (targets) machine code; and "triple" for
historical reasons (in many cases "triples" are actually quads these
days). Target triples are the basic way we refer to particular common platforms;
`rustc` by default knows about 56 of them, and `rustup` today can obtain
compilers for 14, and standard libraries for 30.

## Example: Building static binaries on Linux

Now that we've got the basic pieces in place, let's apply them to
a simple cross-compilation task: building an ultraportable static binary
for Linux.

One of the unique features of Linux that has become increasingly appreciated is
its stable syscall interface. Because the Linux kernel puts exceptional effort
into maintaining a backward-compatible kernel interface, it's possible to
distribute [ELF] binaries with no dynamic library dependencies that will run on
any version of Linux. Besides being one of the features that make Docker
possible, it also allows developers to build self-contained applications and
deploy them to any machine running Linux, regardless of whether it's Ubuntu or
Fedora or any other distribution, and regardless of exact mix of software
libraries they have installed.

[ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

Today's Rust depends on libc, and on most Linuxes that means
glibc. It's technically challenging to fully statically link glibc,
which presents difficulties when using it to produce a truly standalone
binary. Fortunately, an alternative exists: [musl], a small, modern
implementation of libc that can be easily statically linked. Rust has been
compatible with musl since version 1.1, but until recently developers
have needed to build their own compiler to benefit from it.

[musl]: https://www.musl-libc.org/

With that background, let's walk through compiling a statically-linked Linux
executable. For this example you'll want to be running Linux &mdash; that is, your
*host platform* will be Linux, and your *target platform* will also be Linux,
just a different flavor: musl. (Yes, this is technically cross-compilation
even though both targets are Linux).

I'm going to be running on Ubuntu 16.04 (using [this Docker
image]). We'll be building the basic hello world:

[this Docker image]: https://hub.docker.com/r/brson/rustup-demo/

```
rust:~$ cargo new --bin hello && cd hello
rust:~/hello$ cargo run
   Compiling hello v0.1.0 (file:///home/rust/hello)
     Running `target/debug/hello`
Hello, world!
```

That's with the default `x86_64-unknown-linux-gnu` target. And you can
see it has many dynamic dependencies:

```
rust:~/hello$ ldd target/debug/hello
        linux-vdso.so.1 =>  (0x00007ffe5e979000)
        libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fca26d03000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fca26ae6000)
        libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fca268cf000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fca26506000)
        /lib64/ld-linux-x86-64.so.2 (0x000056104c935000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fca261fd000)
```

To compile for musl instead call `cargo` with the argument
`--target=x86_64-unknown-linux-musl`. If we just go ahead and try that
we'll get an error:

```
rust:~/hello$ cargo run --target=x86_64-unknown-linux-musl
   Compiling hello v0.1.0 (file:///home/rust/hello)
error: can't find crate for `std` [E0463]
error: aborting due to previous error
Could not compile `hello`.
...

```

The error tells us that the compiler can't find `std`. That is of
course because we haven't installed it.

To start cross-compiling, you need to acquire a standard library for the target
platform. Previously, this was an error-prone, manual process &mdash; cue those
blog posts I mentioned earlier. But with rustup, it's just part of the usual
workflow:

```
rust:~/hello$ rustup target add x86_64-unknown-linux-musl
info: downloading component 'rust-std' for 'x86_64-unknown-linux-musl'
info: installing component 'rust-std' for 'x86_64-unknown-linux-musl'
rust:~/hello$ rustup show
installed targets for active toolchain
--------------------------------------

x86_64-unknown-linux-gnu
x86_64-unknown-linux-musl

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.8.0 (db2939409 2016-04-11)
```

So I'm running the 1.8 toolchain for Linux on 64-bit x86, as indicated by the
`x86_64-unknown-linux-gnu` target triple, and now I can also target
`x86_64-unknown-linux-musl`. Neat. Surely we are ready to build a slick
statically-linked binary we can release into the cloud. Let's try:

```
rust:~/hello$ cargo run --target=x86_64-unknown-linux-musl
   Compiling hello v0.1.0 (file:///hello)
     Running `target/x86_64-unknown-linux-musl/debug/hello`
Hello, world!
```

And that... just worked! Run `ldd` on it for proof that it's the real
deal:

```
rust:~/hello$ ldd target/x86_64-unknown-linux-musl/debug/hello
        not a dynamic executable
```

Now take that `hello` binary and copy it to any x86_64 machine running Linux and
it'll run just fine.

For more advanced use of musl consider [rust-musl-builder], a Docker
image set up for musl development, which helpfully includes common C
libraries compiled for musl.

[rust-musl-builder]: https://github.com/emk/rust-musl-builder

## Example: Running Rust on Android

One more example. This time building for Android, from Linux, i.e.,
`arm-linux-androideabi` from `x86_64-unknown-linux-gnu`.  This can also be done
from OS X or Windows, though on Windows the setup is slightly different.

To build for Android we need to add the Android target, so let's
set up another 'hello, world' project and install it.

```
rust:~$ cargo new --bin hello && cd hello
rust:~/hello$ rustup target add arm-linux-androideabi
info: downloading component 'rust-std' for 'arm-linux-androideabi'
info: installing component 'rust-std' for 'arm-linux-androideabi'
rust:~/hello$ rustup show
installed targets for active toolchain
--------------------------------------

arm-linux-androideabi
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.8.0 (db2939409 2016-04-11)
```

So let's see what happens if we try to just build our 'hello'
project without installing anything further:

```
rust:~/hello$ cargo build --target=arm-linux-androideabi
   Compiling hello v0.1.0 (file:///home/rust/hello)
error: linking with `cc` failed: exit code: 1
... (lots of noise elided)
error: aborting due to previous error
Could not compile `hello`.
```

The problem is that we don't have a linker that supports Android yet,
so let's take a moment's digression to talk about building for
Android. To develop for Android we need the [Android NDK]. It contains
the linker `rustc` needs to create Android binaries. To just *build*
Rust code that targets Android the only thing we need is the NDK, but
for practical development we'll want the [Android SDK] too.

[Android NDK]: https://developer.android.com/ndk/guides/setup.html#install
[Android SDK]: https://developer.android.com/sdk/index.html

On Linux, download and unpack them with the following commands (the
output of which is not included here):

```
rust:~/home$ cd
rust:~$ curl -O https://dl.google.com/android/android-sdk_r24.4.1-linux.tgz
rust:~$ tar xzf android-sdk_r24.4.1-linux.tgz
rust:~$ curl -O https://dl.google.com/android/repository/android-ndk-r10e-linux-x86_64.zip
rust:~$ unzip android-ndk-r10e-linux-x86_64.zip
```

We further need to create what the NDK calls a ["standalone
toolchain"].  We're going to put ours in a directory called
`android-18-toolchain`:

["standalone toolchain"]: https://developer.android.com/ndk/guides/standalone_toolchain.html

```
rust:~$ android-ndk-r10e/build/tools/make-standalone-toolchain.sh \
      --platform=android-18 --toolchain=arm-linux-androideabi-clang3.6 \
      --install-dir=android-18-toolchain --ndk-dir=android-ndk-r10e/ --arch=arm
Auto-config: --toolchain=arm-linux-androideabi-4.8, --llvm-version=3.6
Copying prebuilt binaries...
Copying sysroot headers and libraries...
Copying c++ runtime headers and libraries...
Copying files to: android-18-toolchain
Cleaning up...
Done.
```

Let's notice a few things about these commands. First, the NDK we
downloaded, `android-ndk-r10e-linux-x86_64.zip` is not the most recent
release (which at the time of this writing is 'r11c'). Rust's `std` is
built against `r10e` and links to symbols that are no longer included
in the NDK. So for now we have to use the older NDK.  Second, in
building the standalone toolchain we passed `--platform=android-18` to
`make-standalone-toolchain.sh`. The "18" here is the Android [API
level](https://developer.android.com/guide/topics/manifest/uses-sdk-element.html#ApiLevels).
Today, Rust's `arm-linux-androideabi` target is built against Android
API level 18, and should theoretically be forwards-compatible with
subsequent Android API levels. So we're picking level 18 to get the
greatest Android compatibility that Rust presently allows.

The final thing for us to do is tell Cargo where to find the android
linker, which is in the standalone NDK toolchain we just created. To
do that we configure the `arm-linux-androideabi` target in
[`.cargo/config`] with the 'linker' value. And while we're doing that
we'll go ahead and set the default target for this project to Android
so we don't have to keep calling cargo with the `--target` option.

[`.cargo/config`]: https://doc.crates.io/config.html

```toml
[build]
target = "arm-linux-androideabi"

[target.arm-linux-androideabi]
linker = "/home/rust/android-18-toolchain/bin/arm-linux-androideabi-clang"
```

Now let's change back to the 'hello' project directory and try
to build again:

```
rust:~$ cd hello
rust:~/hello$ cargo build
   Compiling hello v0.1.0 (file:///home/rust/hello)
```

Success! Of course just getting something to build is not the end of
the story. You've also got to package your code up as an Android
APK. For that you can use [cargo-apk].

[cargo-apk]: https://users.rust-lang.org/t/announcing-cargo-apk/5501

## Rust everywhere else

Rust is a software platform with the potential to run on anything with
a CPU. In this post I showed you a little bit of what Rust can already
do, with the rustup tool. Today Rust runs on most of the platforms you
use daily. Tomorrow it will run everywhere.

So what should you expect next?

In the coming months we're going to continue removing barriers to Rust
cross-compilation. Today rustup provides access to the standard library, but as
we've seen in this post, there's more to cross-compilation than rustc +
std. It's acquiring and configuring the linker and C toolchain that is the most
vexing &mdash; each combination of host and target platform requires something
slightly different. We want to make this easier, and will be adding "NDK
support" to rustup. What this means will again depend on the exact scenario, but
we're going to start working from the most demanded, like Android,
and try to automate as much of the detection, installation and configuration of
the non-Rust toolchain components as we can. On Android for instance, the hope
is to automate everything for a basic initial setup except for accepting the
licenses.

In addition to that there are multiple efforts to improve Rust
cross-compilation tooling, including [xargo], which can be used to
build the standard library for targets unsupported by rustup, and
[cargo-apk], which builds Android packages from Cargo packages.

[cargo-apk]: https://users.rust-lang.org/t/announcing-cargo-apk/5501
[xargo]: https://github.com/japaric/xargo

Finally, the most exciting platform on the horizon for Rust is not a traditional
target for systems languages: the web. With [Emscripten] today it's quite easy
to run *C++* code on the web by converting LLVM IR to JavaScript (or the asm.js
subset of JavaScript). And the upcoming [WebAssembly][] (wasm) standard will
cement the web platform as a first-class target for programming languages.

*Rust is uniquely-positioned to be the most powerful and usable wasm-targeting
language for the immediate future.* The same properties that make Rust so
portable to real hardware makes it nearly trivial to port Rust to wasm. The same
can't be said for languages with complex runtimes that include garbage
collectors.

Rust has [already been ported to Emscripten][em] (at least twice), but the code
has not yet fully landed. This summer it's happening though: Rust +
Emscripten. Rust on the Web. Rust everywhere.

[em]: https://internals.rust-lang.org/t/need-help-with-emscripten-port/3154

## Epilogue

While many people are reporting success with rustup, it remains in beta, with
some [key outstanding bugs], and is not yet the officially recommended
installation method for Rust (though you should try it). We're going to keep
soliciting feedback, applying polish, and fixing bugs. Then we're going to
improve the rustup installation experience on Windows by
[embedding it into a GUI that behaves like a proper Windows installer][gui].

At that point we'll likely update the [download instructions on
www.rust-lang.org][dl] to recommend rustup. I expect all the existing
installation methods to remain available, including the non-rustup
Windows installers, but at that point our focus will be on improving
the installation experience through rustup. It's also plausible that
rustup itself will be packaged for package managers like Homebrew and
apt.

If you want to try rustup for yourself, visit [www.rustup.rs] and follow the
instructions. Then leave feedback on the [dedicated forum thread][irlo], or
[file bugs] on the issue tracker. More information about rustup is available
in the [README].

[dl]: https://www.rust-lang.org/downloads.html
[irlo]: https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/112
[gui]: https://github.com/rust-lang-nursery/rustup.rs/issues/253
[key outstanding bugs]: https://github.com/rust-lang-nursery/rustup.rs/issues?q=is%3Aopen+is%3Aissue+label%3A%22initial+release%22
[file bugs]: https://github.com/rust-lang-nursery/rustup.rs/issues
[www.rustup.rs]: https://www.rustup.rs
[emscripten]: https://kripken.github.io/emscripten-site/
[README]: https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md

## Thanks

Rust would not be the powerful system it is without the help of many
individuals. Thanks to Diggory Blake for creating rustup, to Jorge
Aparicio for fixing lots of cross-compilation bugs and documenting the
process, Tomaka for pioneering Rust on Android, and Alex Crichton for
creating the release infrastructure for Rust's many platforms.

And thanks to all the rustup contributors: Alex Crichton, Brian
Anderson, Corey Farwell, David Salter, Diggory Blake, Jacob Shaffer,
Jeremiah Peschka, Joe Wilm, Jorge Aparicio, Kai Noda, Kamal Marhubi,
Kevin K, llogiq, Mika Attila, NODA, Kai, Paul Padier, Severen Redwood,
Taylor Cramer, Tim Neumann, trolleyman, Vadim Petrochenkov, V Jackson,
Vladimir, Wayne Warren, Yasushi Abe, Y. T. Chung
