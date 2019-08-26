---
layout: post
title: "Shape of errors to come"
author: Jonathan Turner
---

There are changes afoot in the Rust world. If you've tried out the latest nightly, you'll notice
something is *a little different*. For the past few months we've been working on new way of
reporting errors that's easier to read and understand. This is part of an on-going campaign to
improve Rust's usability across the board. We mentioned ways to help us
[make the transition](http://www.jonathanturner.org/2016/08/helping-out-with-rust-errors.html)
to the new errors, and already many people have jumped in (and thank you to those volunteers!)

Let's dive in and see what's changed.  We'll start with a simple example:

```rust
fn borrow_same_field_twice_mut_mut() {
    let mut foo = make_foo();
    let bar1 = &mut foo.bar1;
    let _bar2 = &mut foo.bar1;
    *bar1;
}
```

Here we make the error of mutably borrowing the same value twice. This is a classic error in Rust.
Sure enough, the error the previous compiler gave us says pretty much that:

![Picture of old error style][old_errors]

The problem though is that it takes a few seconds to look at the message, orient yourself, and find
the critical pieces. This time loss adds up. What if, instead, we cleared away everything that slows
down how you read the error message?

<img src="/images/2016-08-09-Errors/new_errors.png" width="500" />

This is the new error format. It's designed around the fundamental observation that
errors should **focus on the code you wrote**. By doing so, you can much more easily see the context
of what is going on.

# Design

The key insight is putting your source code front and center - everything you see in the output
builds on _your_ code.
By using the code you wrote as the context, we give you an easy way to know at a glance
where the issue is occuring.

![Picture of new constant eval error][new_errors2]

*Constant evaluation errors*

Next, once we know the location, we need to explain what is going wrong. We do this by labeling
points of interest in the code that helped explain
the error.  The most obvious place to begin labeling is where the error has occured. It's the
"what" of the error.

In this example, you can see how we use these primary labels. With them, your eyes can see both the
problematic code, and a few words about the problem. Since this is the most important place to see
first, we give them a bold red look with a
characteristic `^^^` underline. You'll notice in the example that the combination allows you
to quickly spot the error and understand what's going wrong.

![Picture of new trait mismatch][new_errors3]

*Mismatch with trait requirement error*

The source of the error is not the only point of interest.  There are often other points of interest
that help describe "why" an error is occuring. By reading these secondary labels,
you can understand better what is going wrong. These labels are shown in the same order they appear
in your code, again, to ensure you're always able to, at a glance, understand where you are.

In this example, secondary labels show the original requirement from the trait, so you
can see it at the same time and compare the requirement and implementation for yourself.

Primary and secondary labels work together to tell a story about what went wrong. With Rust's big
focus on the borrow-checker and memory safety, users may see unfamiliar concepts when they
encounter one of these errors. These labels help to walk them through how even unfamiliar errors,
like borrow errors, occur.

![Picture of new type name not found][new_errors4]

*Name not in scope, with suggestions*

Sometimes there's too much information to fit on a label, so the new format also supports attaching
additional notes. Just like the previous error format, the new format supports warnings,
suggestions, hints, and more.

# Extended error messages

Once we updated the error message format, we looked for other areas we could apply the lessons we'd
learned. The clear winner here were the `--explain` messages. As the name implies, the `--explain`
feature allows developers to explore unfamiliar error messages with longer, more verbose,
explanations.

Today, when you can call `--explain`, you pass an error code. The compiler then prints out
an extended message that goes into more detail about how errors of that form occur:

```console
rustc --explain E0200

Unsafe traits must have unsafe implementations. This error occurs when an
implementation for an unsafe trait isn't marked as unsafe. This may be resolved
by marking the unsafe implementation as unsafe.

struct Foo;

unsafe trait Bar { }

// this won't compile because Bar is unsafe and impl isn't unsafe
impl Bar for Foo { }
// this will compile
unsafe impl Bar for Foo { }
```

This has been a great way to help bridge between an error and learning an unfamiliar concept in
Rust.

While this message is helpful, it uses a general example that may not be related to your code.
Taking a page from the error message work, we'll be updating the explain
messages to focus on your code. For example, taking the borrow-checker error we
started with, we might have an extended error message that looks like:

```
error[E0499]: cannot borrow `foo.bar1` as mutable more than once at a time
  --> src/test/compile-fail/borrowck/borrowck-borrow-from-owned-ptr.rs:29:22

The borrow checker detected that `foo.bar1` was borrowed multiple
times as a mutable value. In Rust, this can not be done safely because
there may be multiple owners of the value who may write to it at the
same time. If this happens in parallel, the resulting value may be in
an unknown state.

Because this is unsafe, Rust disallows having multiple owners of the
same mutable value.

This is the first time `foo.bar1` is borrowed mutably.

28 |     let bar1 = &mut foo.bar1;
   |                     --------

And this is the second time `foo.bar1` is borrowed mutably. This is
where the error occurs.

29 |     let _bar2 = &mut foo.bar1;
   |                      ^^^^^^^^

Note that the first borrow of `foo.bar1` continues until the borrow
is released. During this time, no additional borrows can occur. This
first borrow ends here:

31 | }
   | -

After the first borrow has ended you are able to borrow it again. To
fix this issue, if you need to borrow a value as mutable more than
once, ensure that the span of time they are borrowed do not overlap.
```

In the above, you see a possible output from this new templated-style. Those of you familiar with
the [Elm] style may recognize that the updated `--explain` messages draw heavy inspiration from
the Elm approach.

Currently, this format is still under design and development. If you'd like to help us shape what
extended errors looks like, come jump into the #rust-cli channel on irc.mozilla.org.

# I want to help!

Great!  We love the enthusiasm. There's
[a lot to do](https://github.com/rust-lang/rust/issues/35233), and a
[lot of skills](http://www.jonathanturner.org/2016/08/helping-out-with-rust-errors.html) that could
help us in different ways. Whether you're good at unit tests, writing docs,
writing code, or working on designs, there are places to jump in.

# Conclusion

Improving Rust is an on-going activity. With the importance of addressing Rust's learning curve a
[key theme in the Rust survey](https://blog.rust-lang.org/2016/06/30/State-of-Rust-Survey-2016.html)
we're as motivated as ever to find any confusing or distracting part of the Rust experience and
give it a healthy amount of polish. Errors are one area where we're applying that polish helps us
improve the learning curve bit by bit, and we're looking forward to seeing how far we can go.





[old_errors]: /images/2016-08-09-Errors/old_errors.png
[new_errors]: /images/2016-08-09-Errors/new_errors.png
[new_errors2]: /images/2016-08-09-Errors/new_errors2.png
[new_errors3]: /images/2016-08-09-Errors/new_errors3.png
[new_errors4]: /images/2016-08-09-Errors/new_errors4.png
[Dybuk]: https://github.com/ticki/dybuk
[Elm]: http://elm-lang.org/blog/compiler-errors-for-humans
