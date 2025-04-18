+++
path = "2016/04/19/MIR"
title = "Introducing MIR"
authors = ["Niko Matsakis"]
description = "The shift to use MIR in the compiler should unlock many exciting improvements."
aliases = ["2016/04/19/MIR.html"]
+++

We are in the final stages of a grand transformation on the Rust
compiler internals. Over the past year or so, we have been steadily
working on a plan to change our internal compiler pipeline, as shown
here:

![Compiler Flowchart][flow]

That is, we are introducing a new intermediate representation (IR) of
your program that we call **MIR**: MIR stands for *mid-level* IR, because
the MIR comes between the existing HIR ("high-level IR", roughly an
[abstract syntax tree][ast]) and [LLVM][llvm] (the "low-level"
IR). Previously, the "translation" phase in the compiler would convert
from full-blown Rust into machine-code-like LLVM in one rather large
step. But now, it will do its work in two phases, with a vastly
simplified version of Rust -- MIR -- standing in the middle.

If you're not a compiler enthusiast, this all might seem arcane and unlikely to
affect you directly. But in reality, **MIR is the key to ticking off a number of
our highest priorities** for Rust:

-   **Faster compilation time**. We are working to make Rust's compilation
    *incremental*, so that when you *re*-compile code, the compiler recomputes only
    what it has to. MIR has been designed from the start with this use-case in mind,
    so it's much easier for us to save and reload, even if other parts of the program
    have changed in the meantime.

    MIR also provides a foundation for more efficient data structures and removal
    of redundant work in the compiler, both of which should speed up compilation
    across the board.

-   **Faster execution time**. You may have noticed that in the new compiler
    pipeline, *optimization* appears twice. That's no accident: previously, the
    compiler relied solely on LLVM to perform optimizations, but with MIR, we can
    do some *Rust-specific* optimizations before ever hitting LLVM -- or, for that
    matter, before monomorphizing code. Rust's rich type system should provide
    fertile ground for going beyond LLVM's optimizations.

    In addition, MIR will uncork some longstanding performance improvements
    to the code Rust generates, like ["non-zeroing" drop][rfc320].

-   **More precise type checking**. Today's Rust compiler imposes some
    artificial restrictions on *borrowing*, restrictions which largely
    stem from the way the compiler currently represents programs. MIR
    will enable [much more flexible borrowing][811], which will in turn
    improve Rust's ergonomics and learning curve.

Beyond these banner user-facing improvements, **MIR also has substantial
engineering benefits for the compiler**:

- **Eliminating redundancy.** Currently, because we write all of our passes in
  terms of the full Rust language, there is quite a lot of duplication. For
  example, both the safety analyses and the backend which produces LLVM IR must
  agree about how to translate drops, or the precise order in which `match`
  expression arms will be tested and executed (which can get quite
  complex). With MIR, all of that logic is centralized in MIR construction, and
  the later passes can just rely on that.

- **Raising ambitions.** In addition to being more [DRY][dry], working
  with MIR is just plain *easier*, because it contains a much more
  primitive set of operations than ordinary Rust. This simplification
  enables us to do a lot of things that were forbodingly complex
  before. We'll look at one such case in this post -- non-zeroing
  drop -- but as we'll see at the end, there are already many others
  in the pipeline.

Needless to say, we're excited, and the Rust community has stepped up
in a big way to make MIR a reality. The compiler can bootstrap and run
its test suite using MIR, and these tests have to pass on every new
commit. Once we're able to run [Crater][crater] with MIR enabled and
see no regressions across the entire [crates.io](https://crates.io/)
ecosystem, we'll turn it on by default (or, you'll forgive a terrible
(wonderful) pun, [launch MIR into orbit][orbit]).

This blog post begins with an overview of MIR's design, demonstrating
some of the ways that MIR is able to abstract away the full details of
the Rust language. Next, we look at how MIR will help with
implementing [non-zeroing drops][rfc320], a long-desired
optimization. If after this post you find you are hungry for more,
have a look at the [RFC that introduced MIR][rfc1211], or jump right
into [the code][]. (Compiler buffs may be particularly interested in
the [alternatives section][alt], which discusses certain design
choices in detail, such as why MIR does not currently use SSA.)

<!-- more -->

### Reducing Rust to a simple core

MIR reduces Rust down to a simple core, removing almost all of the
Rust syntax that you use every day, such as `for` loops, `match`
expressions, and even method calls. Instead, those constructs are
translated to a small set of primitives. This does not mean that MIR
is a *subset* of Rust. As we'll see, many of these primitives
operations are not available in real Rust.  This is because those
primitives could be misused to write unsafe or undesirable programs.

The simple core language that MIR supports is not something you would
want to program in. In fact, it makes things almost painfully
explicit. But it's great if you want to write a type-checker or
generate assembly code, as you now only have to handle the core
operations that remain after MIR translation.

To see what I mean, let's start by simplifying a fragment of Rust code.
At first, we'll just break the Rust down into "simpler Rust", but
eventually we'll step away from Rust altogether and into MIR code.

Our Rust example starts out as this simple `for` loop, which iterates
over all the elements in a vector and processes them one by one:

```rust
for elem in vec {
    process(elem);
}
```

Rust itself offers three kinds of loops: `for` loops, like this one;
`while` and `while let` loops, that iterate until some condition is
met; and finally the simple `loop`, which just iterates until you
break out of it. Each of these kinds of loops encapsulates a
particular pattern, so they are quite useful when writing code. But
for MIR, we'd like to reduce all of these into one core concept.

A `for` loop in Rust works by converting a value into an iterator and
then repeatedly calling `next` on that iterator. That means that we
can rewrite the `for` loop we saw before into a `while let` loop
that looks like this:

```rust
let mut iterator = vec.into_iter();
while let Some(elem) = iterator.next() {
    process(elem);
}
```

By applying this rewriting, we can remove all `for` loops, but that
still leaves multiple kinds of loops. So next we can imagine rewrite
all `while let` loops into a simple `loop` combined with a `match`:

```rust
let mut iterator = vec.into_iter();
loop {
    match iterator.next() {
        Some(elem) => process(elem),
        None => break,
    }
}
```

We've already eliminated two constructs (`for` loops and `while`
loops), but we can go further still. Let's turn from loops for a bit
to look at the method calls that we see. In Rust, method calls like
`vec.into_iter()` and `iterator.next()` are also a kind of
[syntactic sugar][ss]. These particular methods are defined in traits,
which are basically pre-defined interfaces.  For example, `into_iter`
is a method in the `IntoIterator` trait.  Types which can be converted
into iterators implement that trait and define how the `into_iter`
method works for them. Similarly, `next` is defined in the `Iterator`
trait. When you write a method call like `iterator.next()`, the Rust
compiler automatically figures out which trait the method belongs to
based on the type of the `iterator` and the set of traits in
scope. But if we prefer to be more explicit, we could instead invoke
the methods in the trait directly, using function call syntax:

```rust
// Rather than `vec.into_iter()`, we are calling
// the function `IntoIterator::into_iter`. This is
// exactly equivalent, just more explicit.
let mut iterator = IntoIterator::into_iter(vec);
loop {
    // Similarly, `iterator.next()` can be rewritten
    // to make clear which trait the `next` method
    // comes from. We see here that the `.` notation
    // was also adding an implicit mutable reference,
    // which is now made explicit.
    match Iterator::next(&mut iterator) {
        Some(elem) => process(elem),
        None => break,
    }
}
```

At this point, we've managed to reduce the set of language features
for our little fragment quite a bit: we now only use `loop` loops and
we don't use method calls. But we could reduce the set of concepts
further if were moved away from `loop` and `break` and towards
something more fundamental: `goto`. Using `goto` we could transform
the previous code example into something like this:

```rust
    let mut iterator = IntoIterator::into_iter(vec);

loop:
    match Iterator::next(&mut iterator) {
        Some(elem) => { process(elem); goto loop; }
        None => { goto break; }
    }

break:
    ...
```

We've gotten pretty far in breaking our example down into simpler
constructs. We're not quite done yet, but before we go further it's
worth stepping back a second to make a few observations:

**Some MIR primitives are more powerful than the structured construct
they replace.** Introducing the `goto` keyword is a big simplification
in one sense: it unifies and replaces a large number of control-flow
keywords. `goto` completely replaces `loop`, `break`, `continue`, but
it also allows us to simplify `if` and `match` as well (we'll see more
on `match` in particular in a bit). However, this simplification is
only possible because `goto` is a *more general* construct than
`loop`, and it's something we would not want to introduce into the
language proper, because we don't want people to be able to write
spaghetti-like-code with complex control-flow that is hard to read and
follow later. But it's fine to have such a construct in MIR, because
we know that it will only be used in particular ways, such as to
express a `loop` or a `break`.

**MIR construction is type-driven.** We saw that all method calls like
`iterator.next()` can be desugared into fully qualified function calls
like `Iterator::next(&mut iterator)`. However, doing this rewrite is
only possible with full type information, since we must (for example)
know the type of `iterator` to determine which trait the `next` method
comes from. In general, constructing MIR is only possible after
type-checking is done.

**MIR makes all types explicit.** Since we are constructing MIR after
the main type-checking is done, MIR can include full type
information. This is useful for analyses like the borrow checker,
which require the types of local variables and so forth to operate,
but also means we can run the type-checker periodically as a kind of
sanity check to ensure that the MIR is well-formed.

### Control-flow graphs

In the previous section, I presented a gradual "deconstruction" of a
Rust program into something resembling MIR, but we stayed in textual
form. Internally to the compiler, though, we never "parse" MIR or have
it in textual form. Instead, we represent MIR as a
[set of data structures][the code] encoding a
[control-flow graph (CFG)][CFG]. If you've ever used a flow-chart,
then the concept of a control-flow graph will be pretty familiar to
you. It's a representation of your program that exposes the underlying
control flow in a very clear way.

A control-flow graph is structured as a set of basic blocks connected
by edges. Each basic block contains a sequence of statements and ends
in a *terminator*, which defines how the blocks are connected to one
another. When using a control-flow graph, a loop simply appears as a
cycle in the graph, and the `break` keyword translates into a path out
of that cycle.

Here is the running example from the previous section, expressed as a
control-flow graph:

![Control-flow-graph][cfg-img]

Building a control-flow graph is typically a first step for any kind
of flow-sensitive analysis. It's also a natural match for LLVM IR,
which is also structured into control-flow graph form. The fact that
MIR and LLVM correspond to one another fairly closely makes
translation quite straight-forward. It also eliminates a vector for
bugs: in today's compiler, the control-flow graph used for analyses is
not necessarily the same as the one which results from LLVM
construction, which can lead to incorrect programs being accepted.

### Simplifying match expressions

The example in the previous section showed how we can reduce all of
Rust's loops into, effectively, gotos in the MIR and how we can remove
methods calls in favor of calls to explicit calls to trait
functions. But it glossed over one detail: **match expressions**.

One of the big goals in MIR was to simplify match expressions into a
very small core of operations. We do this by introducing two
constructs that the main language does not include: *switches* and
*variant downcasts*. Like `goto`, these are things that we would not
want in the base language, because they can be misused to write bad
code; but they are perfectly fine in MIR.

It's probably easiest to explain match handling by example. Let's
consider the `match` expression we saw in the previous section:

```rust
match Iterator::next(&mut iterator) {
    Some(elem) => process(elem),
    None => break,
}
```

Here, the result of calling `next` is of type `Option<T>`, where `T`
is the type of the elements. The `match` expression is thus doing two
things: first, it is determining whether this `Option` was a value
with the `Some` or `None` variant. Then, in the case of the `Some`
variant, it is extracting the value `elem` out.

In normal Rust, these two operations are intentionally coupled,
because we don't want you to read the data from an `Option` unless it
has the `Some` variant (to do otherwise would be effectively a C
union, where reads are not checked for correctness).

In MIR, though, we separate the checking of the variant from the
extracting of the data. I'm going to give the equivalent of MIR here
first in a kind of pseudo-code, since there is no actual Rust syntax
for these operations:

```rust
loop:
    // Put the value we are matching on into a temporary variable.
    let tmp = Iterator::next(&mut iterator);
    
    // Next, we "switch" on the value to determine which it has.
    switch tmp {
        Some => {
            // If this is a Some, we can extract the element out
            // by "downcasting". This effectively asserts that
            // the value `tmp` is of the Some variant.
            let elem = (tmp as Some).0;
    
            // The user's original code:
            process(elem);
            
            goto loop;
        }
        None => {
            goto break;
        }
    }
    
break:
    ....
```

Of course, the actual MIR is based on a control-flow-graph, so it
would look something like this:

![Loop-break control-flow graph][loop-break]

### Explicit drops and panics

So now we've seen how we can remove loops, method calls, and matches
out of the MIR, and replace them with simpler equivalents. But there
is still one key area that we can simplify. Interestingly, it's
something that happens almost invisibly in the code today: running
destructors and cleanup in the case of a panic.

In the example control-flow-graph we saw before, we were assuming that
all of the code would execute successfully. But in reality, we can't
know that. For example, any of the function calls that we see could
panic, which would trigger the start of unwinding. As we unwind the
stack, we would have to run destructors for any values we
find. Figuring out precisely which local variables should be freed at
each point of panic is actually somewhat complex, so we would like to
make it explicit in the MIR: this way, MIR construction has to figure
it out, but later passes can just rely on the MIR.

The way we do this is two-fold. First, we make *drops* explicit in the
MIR. Drop is the term we use for running the destructor on a value. In
MIR, whenever control-flow passes a point where a value should be
dropped, we add in a special `drop(...)` operation. Second, we add
explicit edges in the control-flow graph to represent potential
panics, and the cleanup that we have to do.

Let's look at the explicit drops first. If you recall, we started with
an example that was just a for loop:

```rust
for elem in vec {
    process(elem);
}
```

We then transformed this for loop to explicitly invoke
`IntoIterator::into_iter(vec)`, yielding a value `iterator`, from
which we extract the various elements. Well, this value `iterator`
actually has a destructor, and it will need to be freed (in this case,
its job is to free the memory that was used by the vector `vec`; this
memory is no longer needed, since we've finished iterating over the
vector). Using the `drop` operation, we can adjust our MIR
control-flow-graph to show explicitly where the `iterator` value gets
freed. Take a look at the new graph, and in particular what happens
when a `None` variant is found:

![Drop control-flow graph][drop]

Here we see that, when the loop exits normally, we will drop the
iterator once it has finished. But what about if a panic occurs? Any
of the function calls we see here could panic, after all. To account
for that, we introduce panic edges into the graph:

![Panic control-flow graph][drop-unwind]

Here we have introduced `panic` edges onto each of the function
calls. By looking at these edges, you can see that if the call to
`next` or `process` should panic, then we will drop the variable
`iterator`; but if the call to `into_iter` panics, the `iterator`
hasn't been initialized yet, so it should not be dropped.

One interesting wrinkle: we recently approved [RFC 1513][rfc1513],
which allows an application to specify that panics should be treated
as calls to `abort`, rather than triggering unwinding. If the program
is being compiled with "panic as abort" semantics, then this too would
be reflected in the MIR, as the panic edges and handling would simply
be absent from the graph.

### Viewing MIR on play

At this point, we've reduced our example into something fairly close
to what MIR actually looks like. If you'd like to see for yourself,
you can [view the MIR for our example](https://is.gd/MOfDfg) on
[play.rust-lang.org](https://play.rust-lang.org/). Just
[follow this link](https://is.gd/MOfDfg) and then press the "MIR"
button along the top. You'll wind up seeing the MIR for several
functions, so you have to search through to find the start of the
`example` fn. (I won't reproduce the output here, as it is fairly
lengthy.) In the compiler itself, you can also enable graphviz output.

### Drops and stack flags

By now I think you have a feeling for how MIR represents a simplified
Rust. Let's look at one example of where MIR will allow us to
implement a long-awaited improvement to Rust: the shift to non-zeroing
drop. This is a change to how we detect when destructors must execute,
particularly when values are only *sometimes* moved. This move was
proposed (and approved) in [RFC 320][rfc320], but it has yet to be
implemented. This is primarily because doing it on the pre-MIR
compiler was architecturally challenging.

To better understand what the feature is, consider this function
`send_if`, which conditionally sends a vector to another thread:

```rust
fn send_if(data: Vec<Data>) {
    // If `some_condition` returns *true*, then ownership of `data`
    // moves into the `send_to_other_thread` function, and hence
    // we should not free it (the other thread will free it).
    if some_condition(&data) {
        send_to_other_thread(data);
    }

    post_send();

    // If `some_condition` returned *false*, the ownership of `data`
    // remains with `send_if`, which means that the `data` vector
    // should be freed here, when we return.
}
```

The key point, as indicated in the comments, is that we can't know
statically whether we ought to free `data` or not. It depends on
whether we entered the `if` or not.

To handle this scenario today, the compiler uses *zeroing*. Or, more
accurately, *overwriting*. What this means is that, if ownership of
`data` is moved, we will overwrite the stack slot for `data` with a
specific, distinctive bit pattern that is not a valid pointer (we used
to use zeroes, so we usually call this zeroing, but we've since
shifted to something different). Then, when it's time to free `data`,
we check whether it was overwritten. (As an aside, this is roughly the
same thing that the equivalent C++ code would do.)

But we'd like to do better than that. What we would *like* to do is to
use boolean flags on the stack that tell us what needs to be freed.
So that might look something like this:

```rust
fn send_if(data: Vec<Data>) {
    let mut data_is_owned = true;

    if some_condition(&data) {
        send_to_other_thread(data);
        data_is_owned = false;
    }

    post_send();

    // Free `data`, but only if we still own it:
    if data_is_owned {
        mem::drop(data);
    }
}
```

Of course, you couldn't write code like this in Rust. You're not
allowed to access the variable `data` after the `if`, since it might
have been moved. (This is yet another example of where we can do
things in MIR that we would not want to allow in full Rust.)

Using boolean stack flags like this has a lot of advantages. For one,
it's more efficient: instead of overwriting the entire vector, we only
have to set the one flag. But also, it's easier to optimize: imagine
that, through inlining or some other means, the compiler was able to
determine that `some_condition` would always be true. In that case,
standard constant propagation techniques would tell us that
`data_is_owned` is always false, and hence we can just optimize away
the entire call to `mem::drop`, resulting in tighter code. See
[RFC 320][rfc320] for more details on that.

However, implementing this optimization properly on the current
compiler architecture is quite difficult. With MIR, it becomes
relatively straightforward. The MIR control-flow-graph tells us
explicitly where values will be dropped and when. When MIR is first
generated, we assume that dropping moved data has no effect -- roughly
like the current overwriting semantics. So this means that the MIR for
`send_if` might look like this (for simplicity, I'll ignore unwinding
edges).

![Non-zeroing drop example][nzd]

We can then transform this graph by identifying each place where
`data` is moved or dropped and checking whether any of those places
can reach one another. In this case, the `send_to_other_thread(data)` block can
reach `drop(data)`. This indicates that we will need to introduce a
flag, which can be done rather mechanically:

![Non-zeroing drop with flags][nzd-flags]

Finally, we can apply standard compiler techniques to optimize this
flag (but in this case, the flag is needed, and so the final result
would be the same).

Just to drive home why MIR is useful, let's consider a variation on
the `send_if` function called `send_if2`. This variation checks some
condition and, if it is met, sends the data to another thread for
processing. Otherwise, it processes it locally:

```rust
fn send_if2(data: Vec<Data>) {
    if some_condition(&data) {
        send_to_other_thread(data);
        return;
    }

    process(&data);
}
```

This would generate MIR like:

![Control-flow graph for send_if2][send_if2]

As before, we still generate the drops of `data` in all cases, at
least to start. Since there are still moves that can later reach a
drop, we could now introduce a stack flag variable, just as before:

![send_if2 with flags][send_if2-flags]

But in this case, if we apply constant propagation, we can see that at
each point where we test `data_is_owned`, we know statically whether
it is true or false, which would allow us to remove the stack flag and
optimize the graph above, yielding this result:

![Optimized send_if2][send_if2-opt]

### Conclusion

I expect the use of MIR to be quite transformative in terms of what
the compiler can accomplish. By reducing the language to a core set of
primitives, MIR opens the door to a number of language improvements.
We looked at drop flags in this post. Another example is improving
Rust's lifetime system to
[leverage the control-flow-graph for better precision][811]. But I
think there will be many applications that we haven't foreseen. In
fact, one such example has already arisen: Scott Olson has been making
great strides developing a MIR interpreter [miri][], and the
techniques it is exploring may well form the basis for a more powerful
constant evaluator in the compiler itself.

The transition to MIR in the compiler is not yet complete, but it's
getting quite close. Special thanks go out to Simonas Kazlauskas
([nagisa][]) and Eduard-Mihai Burtescu ([eddyb][]), who have both had
a particularly large impact on pushing MIR towards the finish
line. Our initial goal is to switch our LLVM generation to operate
exclusively from the MIR. Work is also proceeding on porting the
borrow checker. After that, I expect we will port a number of other
pieces on the compiler that are currently using the HIR. If you'd be
interested in contributing, look for
[issues tagged with `A-mir`][issues] or ask around in the
[`#rustc` channel on IRC][community].

[rfc320]: https://github.com/rust-lang/rfcs/blob/master/text/0320-nonzeroing-dynamic-drop.md
[rfc1513]: https://github.com/rust-lang/rfcs/blob/master/text/1513-less-unwinding.md
[issues]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AA-mir
[community]: https://www.rust-lang.org/community.html
[DRY]: https://en.wikipedia.org/wiki/Don%27t_repeat_yourself
[811]: https://github.com/rust-lang/rfcs/issues/811
[miri]: https://github.com/tsion/miri
[rfc1211]: https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md
[ss]: https://en.wikipedia.org/wiki/Syntactic_sugar
[CFG]: https://en.wikipedia.org/wiki/Control_flow_graph
[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[LLVM]: https://llvm.org/
[the code]: https://github.com/rust-lang/rust/blob/f7ec6873ccfbf7dcdbd1908c0857c866b3e7087a/src/librustc/mir/repr.rs
[orbit]: https://en.wikipedia.org/wiki/Mir
[crater]: https://github.com/brson/taskcluster-crater

[pipeline]: https://gist.githubusercontent.com/nikomatsakis/d3942c970fc7b5dc2feee32ea4d5a00a/raw/8712369186243df6e5f156cdb61a16babc0ed464/z-2016-04-compiler-pipeline.jpg
[cfg1]:     https://gist.githubusercontent.com/nikomatsakis/d3942c970fc7b5dc2feee32ea4d5a00a/raw/8712369186243df6e5f156cdb61a16babc0ed464/z-2016-04-cfg-match-1.jpg
[nagisa]: https://github.com/nagisa/
[eddyb]: https://github.com/eddyb/

[cfg-img]: cfg.svg
[drop-unwind]: drop-unwind.svg
[drop]: drop.svg
[flow]: flow.svg
[loop-break]: loop-break.svg
[nzd-flags]: nzd-flags.svg
[nzd]: nzd.svg
[send_if2-flags]: send_if2-flags.svg
[send_if2-opt]: send_if2-opt.svg
[send_if2]: send_if2.svg
[alt]: https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md#alternatives
