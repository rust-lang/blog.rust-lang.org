+++
layout = "post"
date = 2021-01-26
title = "Rust & the case of the disappearing stack frames"
author = "Kyle Strand"
description = "introducing an exploration of how `longjmp` and similar functions can be handled in Rust"
team = "the FFI-unwind project group <https://www.rust-lang.org/governance/teams/lang#wg-ffi-unwind>"
+++

Now that the [FFI-unwind Project Group][proj-group-gh] has merged [an
RFC][c-unwind-rfc] specifying the `"C unwind"` ABI and removing some instances
of undefined behavior in the `"C"` ABI, we are ready to establish new goals for
the group.

Our most important task, of course, is to implement the newly-specified
behavior. This work has been undertaken by Katelyn Martin and can be followed
[here][c-unwind-pr].

The requirements of our current charter, and the [RFC creating the
group][proj-group-rfc], are effectively fulfilled by the specification of `"C
unwind"`, so one option is to simply wind down the project group. While
drafting the `"C unwind"` RFC, however, we discovered that the existing
guarantees around `longjmp` and similar functions could be improved. Although
this is not strictly related to unwinding<sup>[1](#longjmp-unwind)</sup>, they
are closesly related: they are both "non-local" control-flow mechanisms that
prevent functions from returning normally. Because one of the goals of the Rust
project is for Rust to interoperate with existing C-like languages, and these
control-flow mechanisms are widely used in practice, we believe that Rust must
have some level of support for them.

This blog post will explain the problem space. If you're interested in helping
specify this behavior, please come join us in [our Zulip
stream][proj-group-zulip]!

## `longjmp` and its ilk

Above, I mentioned `longjmp` and "similar functions". Within the context of the
`"C unwind"` PR, this referred to functions that have different implementations
on different platforms, and which, on *some* platforms, rely on [forced
unwinding][forced-unwinding]. In our next specification effort, however, we
would like to ignore the connection to unwinding entirely, and define a class
of functions with the following characteristic:

> a function that causes a "jump" in control flow by deallocating some number of
> stack frames without performing any additional "clean-up" such as running
> destructors

This is the class of functions we would like to address. The other primary
example is `pthread_exit`. As part of our specification, we would like to
create a name for this type of function, but we have not settled on one yet;
for now, we are referring to them as "cancelable", "`longjmp`-like", or
"stack-deallocating" functions.

## Our constraints

Taking a step back, we have two mandatory constraints on our design:

* There must be sound way to call `libc` functions that may `pthread_cancel`.
* There must be a sound way for Rust code to invoke C code that may `longjmp`
  over Rust frames.

In addition, we would like to adhere to several design principles:

* The specified behavior can't be target-platform-specific; in other words, our
  specification of Rust's interaction with `longjmp` should not depend on
  whether `longjmp` deallocates frames or initiates a forced-unwind.
  Optimizations, however, *can* be target-platform-specific.
* There should be no difference in the specified behavior of frame-deallocation
  performed by `longjmp` versus that performed by `pthread_cancel`.
* We will only permit canceling POFs ("Plain Old Frames", explained in the next
  section).

## POFs and stack-deallocating functions

The `"C unwind"` RFC introduced a new concept designed to help us deal with
force-unwinding or stack-deallocating functions: the [POF, or "Plain Old
Frame"][POF-definition]. These are frames that can be trivially deallocated,
i.e., they do no "cleanup" (such as running `Drop` destructors) before
returning.

From the definition, it should be clear that it is dangerous to call a
stack-deallocating function in a context that could destroy a non-POF stack
frame. A simple specification for Rust's interaction with stack-deallocating
functions, then, could be that they are safe to call as long as only POFs are
deallocated. This would make Rust's guarantees for `longjmp` essentially the
same as C++'s.

For now, however, we are considering POFs to be "necessary but not sufficient."
We believe that a more restrictive specification may provide the following
advantages:

* more opportunities for helpful compiler warnings or errors to prevent misuse
  of stack-deallocation functions
* semantic tracatbility: we can make reliance on stack-frame-deallocation
  visible for all functions involved
* increased optimization potential when cleanup is "guaranteed" (i.e., the
  compiler may turn a POF into a non-POF if it knows that this is safe and that
  the newly inserted cleanup operation is necessary for an optimization)

## Annotating POFs

Our current plan is to introduce a new annotation for frames that are intended
to be safe to cancel. These functions, of course, must be POFs. The
annotation would be "transitive", just like `async`: functions without this
annotation either must not invoke any annotated functions or must guarantee
that they will cause the stack-deallocation to terminate (for instance, a
non-POF, non-annotated function may call `setjmp`).

### Open questions

The name of the annotation should be based on the terminology used to refer to
functions that are safe to deallocate. Because this terminology is not
finalized, we do not yet have a name for the annotation.

It is also not yet clear whether annotated functions should be able to invoke
any functions without this annotation. As long as the function call does not
return a new `Drop` resource (making the annotated function no longer a POF),
it may be safe, as long as we guarantee that the annotated function cannot be
canceled while the un-annotated function is still on the stack; i.e.,
cancelation must happen during an active call to an annotated cancelable
function.

Most importantly, we do not have a plan for how to indicate that a
non-annotated function can safely call an annotated function. The example of
using `setjmp` to ensure that a `longjmp` will not discard a stack frame is
non-trivial:

* `setjmp` is not a function but a C macro. There is no way to call it directly
  in Rust.
* `setjmp` does not prevent arbitrary `longjmp`s from crossing over a frame,
  the way C++'s `catch` can catch any exception. Instead, `setjmp` creates an
  object of type `jmp_buf`, which must be passed to `longjmp`; this causes the
  jump to stop at the corresponding `setjmp` call.

And, of course, `setjmp`/`longjmp` is not the only example of such a mechanism!
Thus, there is probably no way for the compiler to guarantee that this is safe,
and it's unclear what heuristics could be applied to make it as safe as
possible.

### Examples

Let us use `#[pof-longjmp]` as a placeholder for the annotation indicating a
function that can be safely deallocated, and let us assume that the following
function is a wrapper around `longjmp`:

```rust
extern "C" {
    #[pof-longjmp]
    fn longjmp(CJmpBuf) -> !;
}
```

The compiler would not allow this:

```rust
fn has_drop(jmp_buf: CJmpBuf) {
    let s = "string data".to_owned();
    unsafe { longjmp(jmp_buf); }
    println!("{}", s);
}
```

Here, `s` implements `Drop`, so `has_drop` is not a POF. Since `longjmp` is
annotated `#[pof-longjmp]`, the un-annotated function `has_drop` can't call it
(even in an `unsafe` block). If, however, `has_drop` is annotated:

```rust
#[pof-longjmp]
fn has_drop(jmp_buf: CJmpBuf) {
    let s = "string data".to_owned();
    unsafe { longjmp(jmp_buf); }
    println!("{}", s);
}
```

...there is a different error: `#[pof-longjmp]` can only be applied to POFs,
and since `s` implements `Drop`, `has_drop` is not a POF.

An example of a permissible `longjmp` call would be:

```rust
#[pof-longjmp]
fn no_drop(jmp_buf: CJmpBuf) {
    let s = "string data";
    unsafe { longjmp(jmp_buf); }
    println!("{}", s);
}
```

## Join us!

If you would like to help us create this specification and write an RFC for it,
please join us in [zulip][proj-group-zulip]!

#### Footnotes

<a name="longjmp-unwind">1</a>: As mentioned in the RFC, on Windows,
`longjmp` actually *is* an unwinding operation. On other platforms, however,
`longjmp` is unrelated to unwinding.

[proj-group-gh]: https://github.com/rust-lang/project-ffi-unwind
[proj-group-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/2797-project-ffi-unwind.md
[proj-group-zulip]: https://rust-lang.zulipchat.com/#narrow/stream/210922-project-ffi-unwind/topic/welcome.2C.20redux/near/216807277
[c-unwind-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md
[c-unwind-pr]: https://github.com/rust-lang/rust/pull/76570
[forced-unwinding]: https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md#forced-unwinding
[POF-definition]: https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md#plain-old-frames
