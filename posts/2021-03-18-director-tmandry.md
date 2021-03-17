---
layout: post
title: "Rust Foundation Director Series: Tyler Mandry"
author: Tyler Mandry
---

_Over the next five weeks, we are publishing blog posts from each director of
the Rust Foundation, introducing them to the community._

Hello, Rust community! My name is Tyler Mandry, and I’m excited to serve as
the project director for quality on the board of the Rust Foundation.

My journey with Rust started about five years ago. I was what you might call
a reluctant enthusiast. I love learning new ways of doing things, but I’m
also somewhat skeptical of “new tech hype.” At the time, I worked in
high-performance computing. Eventually I realized a huge portion of our time
was spent chasing down bugs, like segfaults and data races, that were
difficult to reproduce and could never have existed in a language like
Rust.[^1]

Feeling the pain on a daily basis that Rust promised to prevent was what
finally got me interested. But what got me hooked – and inspired me to
contribute – was the thoughtfulness and openness of the project itself. The
technical decisions were all made in the open, and it was plain that everyone
cared deeply about making Rust the best it could be. No one knew me in the
Rust community, but I was able to join a working group and contribute changes
right away.

In 2019 I joined Google to work on the Fuchsia operating system, supporting
Rust development, and I began leading the Rust on Fuchsia team in early 2020.
I’ve contributed a number of things in my time here like async/await compiler
optimizations, mentored work like source-based code coverage, and lead the
async foundations working group.

What else should you know about me? Well, when I’m not in front of a computer
I’m often on my bike, most likely listening to a podcast. I just began
learning to play piano – ask me how that’s going in six months.[^2]

## The road ahead

Looking around, it’s becoming clear that Rust will play a big role in
how we write software over the next 5-10 years – or more. Reliability,
efficiency, and productivity is a combination that will be in demand for a
long time to come. I’m extremely excited to see more and more developers
learning Rust every day.

I see the establishment of a Rust foundation as a major step forward in the
maturity of the project, not unlike what releasing Rust 1.0 meant for the
language itself. Starting a foundation makes the project more sustainable
_and_ unlocks a lot of new possibilities. I think the next two years will
involve “unlearning” all the things we thought we couldn’t do, but now can
thanks to dedicated funding.

So, what can the foundation do to improve quality? I have specific ideas, but
I think it’s most important that the project first has an _accurate,
comprehensive, detailed, and up-to-date_ view of how it’s doing. So this year,
I’d like to begin by helping Rust contributors _fix blind spots_ like where
there are gaps in documentation and compiler errors, with the ability to
measure changes over time. Another thing we don’t monitor is how compiled
code actually performs; the compiler team [has wanted this][perf] for a long
time, and the Foundation could help make it happen. Finally, it’d be nice to
have more visibility into changes in dependencies, like LLVM and system
runtime libraries, and how that could affect our pace of development.

This is certainly ambitious, and I don’t expect it all to happen overnight.
But I think the project will benefit greatly from making more informed
decisions, and I hope this will allow Rust to continue the impressive
trajectory it’s had so far.

It’s been a joy to work on Rust, and I get the sense that we’re all just
getting started. I’m looking forward to what the Rust project, including the
foundation, will do in 2021 and beyond. I hope you are too.

[perf]: https://internals.rust-lang.org/t/help-needed-corpus-for-measuring-runtime-performance-of-generated-code/6794

[^1]: That’s not to say that the language we were using at the time _caused_
      the problems, just that it failed to prevent them!
[^2]: And in a year, but that seems too distant to motivate me today. This is
      the biggest accountability stunt I’ve pulled by far.
