+++
layout = "post"
title = "Announcing the first FFI-unwind project design meeting"
author = "Kyle Strand, Niko Matsakis, and Amanieu d'Antras"
description = "First design meeting for the FFI-unwind project"
team = "the FFI-unwind project group <https://www.rust-lang.org/governance/teams/lang#wg-ffi-unwind>"
+++

The FFI-unwind project group, announced in [this RFC][rfc-announcement], is
working to extend the language to support unwinding that crosses FFI
boundaries.

We have reached our first technical decision point, on a question we have been
discussing internally for quite a while. This blog post lays out the arguments
on each side of the issue and invites the Rust community to join us at an
upcoming meeting to help finalize our decision, which will be formalized and
published as our first language-change RFC. This RFC will propose an "MVP"
specification for well-defined cross-language unwinding.

The meeting will be on [March 2nd][meeting-link].

## Background: what is unwinding?

Exceptions are a familiar control flow mechanism in many programming languages.
They are particularly commonplace in managed languages such as Java, but they
are also part of the C++ language, which introduced them to the world of
unmanaged systems programming.

When an exception is thrown, the runtime _unwinds_ the stack, essentially
traversing it backwards and calling clean-up or error-recovery code such as
destructors or `catch` blocks.

Compilers may implement their own unwinding mechanisms, but in native code such
as Rust binaries, the mechanism is more commonly provided by the platform ABI.

It is well known that Rust does not have exceptions as such. But Rust _does_
support unwinding! There are two scenarios that will cause unwinding to occur:

* By default, Rust's `panic!()` unwinds the stack.
* Using FFI, Rust can call functions in other languages (such as C++) that can
  unwind the stack.
  * There are some special cases where C libraries can actually cause
    unwinding.  For instance, on Microsoft platforms, `longjmp` is implemented
    as "forced unwinding" (see below)

Currently, when foreign (non-Rust) code invokes Rust code, the behavior of a
`panic!()` unwind that "escapes" from Rust is explicitly undefined. Similarly,
when Rust calls a foreign function that unwinds, the behavior once the unwind
operation encounters Rust frames is undefined. The primary reason for this is
to ensure that Rust implementations may use their own unwinding mechanism,
which may not be compatible with the platform-provided "native" unwinding
mechanism. Currently, however, `rustc` uses the native mechanism, and there are
no plans to change this.

### Forced unwinding

Platform ABIs can define a special kind of unwinding called "forced unwinding."
This type of unwinding works regardless of whether the code being unwound
supports unwinding or not. However, destructors may not be executed if the
frames being unwound do not have unwinding support.

There are two common examples of forced unwinding:

* On Windows platforms, `longjmp` is implemented as a forced unwind.
* On glibc Linux, `pthread_exit` and `pthread_cancel` are implemented as a forced unwind.
  * In fact, `pthread_cancel` can cause all manner of C functions to unwind,
    including common functions like `read` and `write`.  (For a complete list,
    search for "cancellation points" in the [pthreads man page][man-pthreads].)

## Requirements for any cross-language unwinding specification

* Unwinding between Rust functions (and in particular unwinding of Rust panics)
  may not necessarily use the system unwinding mechanism
  * In practice, we do use the system mechanism today, but we would like to
    reserve the freedom to change this.
* If you enable `-Cpanic=abort`, we are able to optimize the size of binaries
  to remove most code related to unwinding.
  * Even with `-Cpanic=unwind` it should be possible to optimize away code when
    unwinding is known to never occur.
  * In practice, most "C" functions are never expected to unwind (because they
    are written in C, for example, and not in C++).
    * However, because unwinding is now part of most system ABIs, even C
      functions can unwind &mdash; most notably cancellation points triggered
      by `pthread_cancel`.
* Changing the behavior from `-Cpanic=unwind` to `-Cpanic=abort` should not
  cause Undefined Behavior.
  * However, this may not be tenable, or at least not without making binaries
    much larger. See the discussion below for more details.
  * It may, of course, cause programs to abort that used to execute
    successfully. This could occur if a panic would've been caught and
    recovered.
* We cannot change the ABI (the `"C"` in `extern "C"`) of functions in the libc
  crate, because this would be a breaking change: function pointers of different
  ABIs have different types.
  * This is relevant for the libc functions which may perform forced unwinding
    when `pthread_cancel` is called.

## The primary question: introduce a new ABI, or let the `"C"` ABI permit unwinding?

The core question that we would like to decide is whether the `"C"` ABI, as
defined by Rust, should permit unwinding.

This is not a question we expected to be debating. We've long declared that
unwinding through Rust's `"C"` ABI is undefined behavior. In part, this is
because nobody had spent the time to figure out what the correct behavior would
be, or how to implement it, although (as we'll see shortly) there are other
good reasons for this choice. 

In any case, in PR #65646, @Amanieu proposed that we could, in fact, simply
define the behavior of unwinding across `"C"` boundaries. In discussing this,
discovered that the question of whether the `"C"` ABI should permit unwinding was
less clear-cut than we had assumed.

If the `"C"` ABI does not permit unwinding, a new ABI, called `"C unwind"`,
will be introduced specifically to support unwinding.

## Three specific proposals

The project group has narrowed the design space down to three specific
proposals. Two of these introduce the new `"C unwind"` ABI, and one does not.

Each proposal specifies the behavior of each type of unwind (Rust `panic!`,
foreign (e.g. C++), and forced (e.g. `pthread_exit`)) when it encounters an
ABI boundary under either the `panic=unwind` or `panic=abort` compile-mode.

Note that currently, `catch_unwind` does not intercept foreign unwinding
(forced or unforced), and our initial RFCs will not change that. We may decide
at a later date to define a way for Rust code to intercept foreign exceptions.

Throughout, the  unwind generated by `panic!` will be referred to as
`panic`-unwind.

### Proposal 1: Introduce `"C unwind"`, minimal specification

* `"C"` ABI boundary, `panic=<any>`
  * `panic`-unwind: program aborts
  * forced unwinding, no destructors: unwind behaves normally
  * other foreign unwinding: undefined behavior
* `"C unwind"` ABI boundary
  * With `panic=unwind`: all types of unwinding behave normally
  * With `panic=abort`: all types of unwinding abort the program

This proposal provides 2 ABIs, each suited for different purposes: you would
generally use `extern "C"` when interacting with C APIs (making sure to avoid
destructors where `longjmp` might be used), and `extern "C unwind"`
when interacting with C++ APIs. The main advantage of this proposal is that
switching between `panic=unwind` and `panic=abort` does not introduce UB if you
have correctly marked all potential unwinding calls as `"C unwind"` (your
program will abort instead).

### Proposal 2: Introduce `"C unwind"`, forced unwinding always permitted

This is the same as the previous design, except that when compiled with
`panic=abort`, forced unwinding would *not* be intercepted at `"C unwind"` ABI
boundaries; that is, they would behave normally (though still UB if there are
any destructors), without causing the program to abort. `panic`-unwind and
non-forced foreign exceptions would still cause the program to abort.

The advantage of treating forced unwinding differently is that it reduces
portability incompatibilities. Specifically, it ensures that using `"C unwind"`
cannot cause `longjmp` or `pthread_exit` to stop working (abort the program)
when the target platform and/or compile flags are changed.  With proposal 1,
`longjmp` will be able to cross `"C unwind"` boundaries _except_ on Windows
with MSVC under `panic=abort`, and `pthread_exit` will work inside `"C unwind"`
functions _except_ when linked with glibc under `panic=abort`. The downside of
this proposal is that the abort stubs around `"C unwind"` calls in `panic=abort`
become more complicated since they need to distinguish between different types
of foreign exceptions.

### Proposal 3: No new ABI

* `panic=unwind`: unwind behaves normally
* `panic=abort`:
  * `panic`-unwind: does not exist; `panic!` aborts the program
  * forced unwinding, no destructors: unwind behaves normally
  * other foreign unwinding: undefined behavior

The main advantage of this proposal is its simplicity: there is only one ABI and
the behavior of `panic=abort` is identical to that of `-fno-exceptions` in C++.
However this comes with the downside that switching to `panic=abort` may in some
cases introduce UB (though only in unsafe code) if FFI calls unwind through Rust
code.

Another advantage is that forced unwinding from existing functions defined in
the `libc` crate such as `pthread_exit` and `longjmp` will be able to unwind
frames with destructors when compiled with `panic=unwind`, which is not possible
with the other proposals.

## Comparison table for the proposed designs

In this table, "UB" stands for "undefined behavior". We believe that all of
these instances of undefined behavior could be detected at runtime, but the
code to do so would impose an undesirable code-size penalty, entirely negating
the optimizations made possible by using `panic=unwind` or the non-unwinding
`"C"` ABI. This code would therefore only be appropriate for debug builds.
Additionally, the complexity of implementing such checks may outweight their
benefits.

Note that unwinding through a frame that has destructors without running those
destructors (e.g. because they have been optimized out by `panic=abort`) is
always undefined behavior.

<table>
<thead>
<tr>
<th></th>
<th><code>panic</code>-unwind</th>
<th>Forced unwind, no destructors</th>
<th>Forced unwind with destructors</th>
<th>Other foreign unwind</th>
</tr>
</thead>
<tbody>
<tr>
<td>Proposals 1 &amp; 2, <code>"C"</code> boundary, <code>panic=unwind</code></td>
<td>abort</td>
<td>unwind</td>
<td>UB</td>
<td>UB</td>
</tr>
<tr>
<td>Proposals 1 &amp; 2, <code>"C"</code> boundary, <code>panic=abort</code></td>
<td><code>panic!</code> aborts (no unwinding occurs)</td>
<td>unwind</td>
<td>UB</td>
<td>UB</td>
</tr>
<tr>
<td>Proposals 1 &amp; 2, <code>"C unwind"</code> boundary, <code>panic=unwind</code></td>
<td>unwind</td>
<td>unwind</td>
<td>unwind</td>
<td>unwind</td>
</tr>
<tr>
<td>Proposal 1, <code>"C unwind"</code> boundary, <code>panic=abort</code></td>
<td><code>panic!</code> aborts</td>
<td>abort</td>
<td>abort</td>
<td>abort</td>
</tr>
<tr>
<td>Proposal 2, <code>"C unwind"</code> boundary, <code>panic=abort</code></td>
<td><code>panic!</code> aborts</td>
<td>unwind</td>
<td>UB</td>
<td>abort</td>
</tr>
<tr>
<td>Proposal 3,  <code>"C"</code> boundary, <code>panic=unwind</code></td>
<td>unwind</td>
<td>unwind</td>
<td>unwind</td>
<td>unwind</td>
</tr>
<tr>
<td>Proposal 3, <code>"C"</code> boundary, <code>panic=abort</code></td>
<td><code>panic!</code> aborts</td>
<td>unwind</td>
<td>UB</td>
<td>UB</td>
</tr>
</tbody>
</table>

[rfc-announcement]: https://github.com/rust-lang/rfcs/pull/2797
[meeting-link]: https://arewemeetingyet.com/UTC/2020-03-02/17:00/Lang%20Team%20Design%20Meeting:%20FFI-unwind#eyJ1cmwiOiJodHRwczovL21vemlsbGEuem9vbS51cy9qLzc2ODIzMTc2MCJ9
[man-pthreads]: http://man7.org/linux/man-pages/man7/pthreads.7.html
