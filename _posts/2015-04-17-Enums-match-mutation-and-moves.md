---
layout: post
title: "Mixing matching, mutation, and moves in Rust"
author: Felix S. Klock II
description: "A tour of matching and enums in Rust."
---

One of the primary goals of the Rust project is to enable safe systems
programming. Systems programming usually implies imperative
programming, which in turns often implies side-effects, reasoning
about shared, aliasable state, et cetera.

At the same time, to provide *safety*, Rust programs and data types
must be structured in a way that allows static checking to ensure
soundness. Rust has features and restrictions that operate in tandem
to ease writing programs that can pass these checks and thus ensure
safety. For example, Rust incorporates the notion of *ownership* deeply
into the language.

Rust's `match` expression is a construct that offers an interesting
combination of such features and restrictions. A `match` expression
takes an input value, classifies it, and then jumps to code written to
handle the identified class of data.

In this post we explore how Rust processes such data via `match`.
The crucial elements that `match` and its counterpart `enum` tie
together are:

* Exhaustive case analysis, which ensures that no case is omitted
  when processing an input.

* `match` embraces both imperative and applicative styles of
   programming.  The compiler's static analyses work hard to ensure
   statement-oriented programming remains palatable, leaving the
   question of whether expression-orientation is better to style
   guides.

* Destructuring bind of *L-values*: Rust encourages the developer to
  think carefully about ownership and borrowing. To ensure that
  processing data does not force one to give up ownership of a value
  prematurely, `match` is designed with support for merely *borrowing*
  substructure within its input (as opposed to always *moving* such
  substructure).

We cover each of the items above in detail below, but first we
establish a foundation for the discussion: What does `match` look
like, and how does it work?

### The Basics of `match`

The `match` expression in Rust has this form:

```rust
match INPUT_EXPRESSION {
    PREDICATE_1 => RESULT_EXPRESSION_1,
    PREDICATE_2 => RESULT_EXPRESSION_2,
    ...
    PREDICATE_n => RESULT_EXPRESSION_n
}
```

where each of the `PREDICATE_i` contains at least one *pattern*. A
pattern describes a subset of the possible values to which
`INPUT_EXPRESSION` could evaluate.
The syntax `PREDICATE => RESULT_EXPRESSION` is called a "match arm",
or simply "arm".

Patterns can match atomic values, like integers or characters; they
can also match user-defined symbolic data, defined via `enum`.

The below code demonstrates generating the next guess (poorly) in a number
guessing game, given the answer from a previous guess.

(Incidentally, nearly all the code in this post is directly
executable; you can cut-and-paste the code snippets into a file
`demo.rs`, compile the file with `--test`, and run the resulting
binary to see the tests run.)

```rust,code
enum Answer {
    Higher,
    Lower,
    Bingo,
}

fn suggest_guess(prior_guess: u32, answer: Answer) {
    match answer {
        Answer::Higher => println!("maybe try {} next", prior_guess + 10),
        Answer::Lower  => println!("maybe try {} next", prior_guess - 1),
        Answer::Bingo  => println!("we won with {}!", prior_guess),
    }
}

#[test]
fn demo_suggest_guess() {
	suggest_guess(10, Answer::Higher);
	suggest_guess(20, Answer::Lower);
	suggest_guess(19, Answer::Bingo);
}
```

Patterns can also match structured data (e.g. tuples, slices, user-defined
data types) via corresponding patterns. In such patterns, one often
binds substructure of the input to local variables (identifer patterns),
for use either in the arm's predicate or in its result.

The special `_` pattern matches any single value, and is often used as
a catch-all; the special `..` pattern generalizes this by matching any
*series* of values or name/value pairs.

Also, one can collapse multiple patterns into one arm by separating the
patterns by vertical bars (`|`); thus that arm matches either this pattern,
or that pattern, et cetera.

These features are illustrated in the following revision to the
guessing-game answer generation strategy:

```rust,code
struct GuessState {
    guess: u32,
    answer: Answer,
    low: u32,
    high: u32,
}

fn suggest_guess_smarter(s: GuessState) {
    match s {
        GuessState { answer: Answer::Bingo, guess: p, .. } => {
            println!("we won with {}!", p);
        }
        GuessState { answer: Answer::Higher, guess: l, low: _, high: h  } |
        GuessState { answer: Answer::Lower, guess: h, low: l, high: _ } => {
            let mid = l + ((h - l) / 2);
            println!("lets try {} next", mid);
        }
    }
}

#[test]
fn demo_guess_state() {
	suggest_guess_smarter(GuessState {
		guess: 20, answer: Answer::Lower, low: 10, high: 1000
	});
}
```

That is `match` in a nutshell.

So, what is the interplay between this construct and Rust's approach to
ownership and safety in general?

### Exhaustive case analysis

One important method of analytical thinking is case analysis: Dividing
a problem into some number of separate cases, and then analyzing each
case individually.

For this method of problem solving to work, the cases must be
*collectively exhaustive*; otherwise, a case that was not covered
would mean a potential problem instance for which no solution has been
identified.

This brings us to one of the fundamental restrictions of Rust's
`match` construct: the collection of provided cases must be exhautive.

So, for example, the following code is rejected at compile-time.

```rust
fn suggest_guess_broken(prior_guess: u32, answer: Answer) {
    let next_guess = match answer {
        Answer::Higher => prior_guess + 10,
        Answer::Lower  => prior_guess - 1,
        // ERROR: non-exhaustive patterns: `Bingo` not covered
    };
    println!("maybe try {} next", next_guess);
}
```

Many other languages offer a pattern matching construct (ML and
various macro-based `match` implementations in Scheme both come to
mind), but not all of them have this restriction.

Rust has this restriction for two reasons:

* First, as noted above, dividing a problem into cases only yields a
general solution if the cases are exhaustive. Exhaustiveness-checking
exposes logical errors.

* Second, since `match` is an expression form, exhaustiveness ensures
that such expressions always evaluates to a value of the correct type
(or jump elsehwere in the program, as illustrated here):

```rust,code
fn suggest_guess_fixed(prior_guess: u32, answer: Answer) {
    let next_guess = match answer {
        Answer::Higher => prior_guess + 10,
        Answer::Lower  => prior_guess - 1,
        Answer::Bingo  => {
            println!("we won!");
            return;
        }
    };
    println!("maybe try {} next", next_guess);
}

#[test]
fn demo_guess_fixed() {
	suggest_guess_fixed(10, Answer::Higher);
	suggest_guess_fixed(20, Answer::Lower);
	suggest_guess_fixed(19, Answer::Bingo);
}
```

### Both expression- and statement-oriented

Unlike many languages that offer pattern matching, Rust *embraces*
both statement- and expression-oriented programming.

Consider writing a function which maps a non-negative integer to a
string rendering it as an ordinal ("1st", "2nd", "3rd", ...).

The following code uses range patterns to simplify things, but also,
it is written in a style similar to a `switch` in a statement-oriented
language like C (or C++, Java, et cetera), where the arms of the
`match` are executed for their side-effect alone:

```rust,code
fn num_to_ordinal(x: u32) -> String {
    let suffix;
    match (x % 10, x % 100) {
        (1, 1) | (1, 21...91) => {
            suffix = "st";
        }
        (2, 2) | (2, 22...92) => {
            suffix = "nd";
        }
        (3, 3) | (3, 23...93) => {
            suffix = "rd";
        }
        _                     => {
            suffix = "th";
        }
    }
    return format!("{}{}", x, suffix);
}

#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(   0),    "0th");
    assert_eq!(num_to_ordinal(   1),    "1st");
    assert_eq!(num_to_ordinal(  12),   "12th");
    assert_eq!(num_to_ordinal(  22),   "22nd");
    assert_eq!(num_to_ordinal(  43),   "43rd");
    assert_eq!(num_to_ordinal(  67),   "67th");
    assert_eq!(num_to_ordinal(1901), "1901st");
}
```

The Rust compiler accepts the above program; this is notable because
its static analysis is ensuring both that `suffix` is always
initialized before we run the `format!` at the end *and* that `suffix`
is assigned at most once during the function's execution (because if
we could assign `suffix` multiple times, the compiler would force us
to mark `suffix` as mutable).

To be clear, the above program certainly *can* be written in an
expression-oriented style. The point is that each of the styles has
its use cases, and switching to a statement-oriented style does not
sacrifice every other feature that Rust provides, such as ensuring
that a non-`mut` binding is assigned at most once.

An important case where this arises is when one wants to
initialize some state and then borrow from it, but only on
*some* control-flow branches.

```rust,code
fn sometimes_initialize(input: i32) {
    let string;
    let borrowed;
    match input {
        0...100 => {
            string = format!("input prints as {}", input);
            borrowed = &string[6..];
        }
        _ => {
            borrowed = "expected between 0 and 100";
        }
    }
    println!("borrowed: {}", borrowed);

    // (Below would cause compile-time error if uncommented.)
    // println!("string: {}", string);
}

#[test]
fn demo_sometimes_initialize() {
    sometimes_initialize(23);
    sometimes_initialize(123);
}
```

The interesting thing about the above code is that after the `match`,
we are not allowed to directly access `string`, because the compiler
requires that the variable be initialized on every path through the
program. At the same time, we *can* access the data that is held
*within* `string`, because a reference to that data is held by the
`borrowed` variable, which we ensure is initialized on every program
path. (The compiler ensures that no outstanding borrows of the
`string` data could possible outlive `string` itself, and the
generated code ensures that at the end of the scope of `string`, its
data is deallocated if it was previously initialized.)

In short, for soundness, the Rust language ensures that data is always
initialized before it is referenced, but the designers have attempted
to not require artifices like dummy-initializations inserted solely to
placate such requirements.

### Algebraic Data Types and Data Invariants

An `enum` type allows one to define mutually-exclusive classes of
values. The examples shown above used `enum` for simple symbolic tags,
but in Rust, such definitions can define much richer classes of data.

For example, a binary tree is either a leaf, or an internal node with
references to two child trees. Here is one way to encode a tree of
integers in Rust:

```rust,code
enum BinaryTree {
    Leaf(i32),
    Node(Box<BinaryTree>, i32, Box<BinaryTree>)
}
```

(The `Box<V>` type describes an owning reference to a heap-allocated
instance of `V`; if you own a `Box<V>`, then you also own the `V` it
contains, and can mutate it, lend out references to it, et cetera, and
when you finish with the box and let it fall out of scope, it will
automatically clean up the resources associated with the
heap-allocated `V`.)

The above definition ensures that if we are given a `BinaryTree`, it
will always fall into one of the above two cases. One will never
encounter a `BinaryTree::Node` that does not have a left-hand child.
There is no need to check for null.

One *does* need to check whether a given `BinaryTree` is a `Leaf` or
az a `Node`, but the compiler statically ensure such checks are done:
you cannot accidentally interpret the data of a `Leaf` as if it were a
`Node`, nor vice versa.

```rust,code
/// Sum of values in all the nodes and leaves of `t`.
fn tree_weight_v1(t: BinaryTree) -> i32 {
    match t {
        BinaryTree::Leaf(payload) => payload,
        BinaryTree::Node(left, payload, right) => {
            tree_weight_v1(*left) + payload + tree_weight_v1(*right)
        }
    }
}

/// Looks like:
///
/// (4)
///  |
///  +--(2)
///  |   |
///  |   +--[1]
///  |   |
///  |   +--[3]
///  |
///  +--[5]
///
fn sample_tree() -> BinaryTree {
    let l1 = Box::new(BinaryTree::Leaf(1));
    let l3 = Box::new(BinaryTree::Leaf(3));
    let n2 = Box::new(BinaryTree::Node(l1, 2, l3));
    let l5 = Box::new(BinaryTree::Leaf(5));

    BinaryTree::Node(n2, 4, l5)
}

#[test]
fn tree_demo_1() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v1(tree), (1 + 2 + 3) + 4 + 5);
}
```

### Matching L-values

The previous section described a tree datatype, and showed a program
that computed the sum of the integers in a tree instance.

That version of `tree_weight` has one big downside, however: it takes
its input tree by value. Once you pass a tree to `tree_weight_v1`, that
tree is gone (as in, deallocated).

```rust,code
#[test]
fn tree_demo_v1_fails() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v1(tree), (1 + 2 + 3) + 4 + 5);

    // If you uncomment this line below ...
    
    // assert_eq!(tree_weight_v1(tree), (1 + 2 + 3) + 4 + 5);

    // ... you will get: error: use of moved value: `tree`
}
```

This is *not* a consequence, however, of using `match`; it is rather
a consequence of the function signature that was chosen:

```rust
fn tree_weight_v1(t: BinaryTree) -> i32 { 0 }
//                   ^~~~~~~~~~ this means this function takes ownership of `t`
```

In fact, in Rust, `match` is designed to work quite well *without*
taking ownership. In particular, the input to `match` is an L-value;
this means that the input expression is evaluated to a *memory
location*, and then match works by inspecting the data at that
location.

(If the input expression is a variable name or a field/pointer
dereference, then the L-value is just the location of that variable or
field/memory.  If the input expression is a function call or other
operation that generates an unnamed temporary value, then it will be
conceptually stored in a temporary area, and that is the memory
location that `match` will inspect.)

So, if we want a version of `tree_weight` that merely borrows a tree
rather than taking ownership of it, then we will need to make use of
this feature of Rust's `match`.

```rust,code
/// Sum of values in all the nodes and leaves of `t`.
fn tree_weight_v2(t: &BinaryTree) -> i32 {
    //               ^~~~~~~~~~~ The `&` means we are *borrowing* the tree
    match *t {
        BinaryTree::Leaf(payload) => payload,
        BinaryTree::Node(ref left, payload, ref right) => {
            tree_weight_v2(left) + payload + tree_weight_v2(right)
        }
    }
}

#[test]
fn tree_demo_2() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v2(&tree), (1 + 2 + 3) + 4 + 5);
}
```

The function `tree_weight_v2` looks very much like `tree_weight_v1`.
The only differences are: we take `t` as a borrowed reference (the `&`
in its type), and, importantly, we use `ref`-bindings for `left` and
`right` in the `Node` case.

The `ref`-binding is a crucial part of how destructuring bind of
L-values works.

When matching a value of type `T`, an identifier pattern `I` will, on
a successful match, *move* the value out of the original input and
into `I`. Thus we can always conclude in such a case that `I` has type
`T`, or "`I: T`". (For some types `T`, known as *copyable* `T` or "`T`
implements `Copy`", the value will in fact be copied into `I` for such
identifier patterns; but in general types are not copyable; either
way, such bindings do mean that `I` has ownership of a value of type
`T`.)

Thus, the bindings of `payload` in `tree_weight_v2` both have type
`i32`; the `i32` type implements `Copy`, so the weight is copied into
`payload` in both arms.

However, when matching a value of type `T`, a `ref`-pattern `ref I`
will, on a successful match, merely *borrow* a reference into the
matched data. In other words, a successful `ref I` match of a value of
type `T` will imply that `I: &T`. Thus, in the `Node` arm of
`tree_weight_v2`, `left` will be reference the left-hand box (which
holds a tree), and `right` will reference the right-hand box (which
also holds a tree). Then we can pass those borrowed references to
trees into the recursive calls to `tree_weight_v2`.

Likewise, a `ref mut`-pattern (`ref mut I`) will, on a successful
match, take a borrow a *mutable reference* `&mut T`, (which allows
mutation and ensures there are no other active references to that data
at the same time). An important detail here is the destructuring
binding forms like `match` allows one to take mutable references to
disjoint parts of the data simultaneously.

```rust,code
/// Increment the values in all the nodes and leaves of `t`.
fn tree_grow(t: &mut BinaryTree) {
    //          ^~~~~~~~~~~~~~~ `&mut`: we have unaliased access to the tree
	match *t {
        BinaryTree::Leaf(ref mut payload) => *payload += 1,
        BinaryTree::Node(ref mut left, ref mut payload, ref mut right) => {
            tree_grow(left);
			*payload += 1;
			tree_grow(right);
        }
    }
}

#[test]
fn tree_demo_3() {
    let mut tree = sample_tree();
	tree_grow(&mut tree);
    assert_eq!(tree_weight_v2(&tree), (2 + 3 + 4) + 5 + 6);
}
```

### Avoiding unsoundness

(This section is adapted from the Rust demo we presented at the 2014
ML workshop.)

A potentially non-obvious issue arises when a supposedly-sound
language adds support for `ref mut`, or even just `ref`, in a
non-moving/non-copying `match` expression: How does one prevent
someone from injecting unsound behavior by using mutable references to
overwrite state secretly, subverting the type-system?

Consider the following code:

```rust,code
#[test]
fn sound_code() {
    enum E { A(fn (i8) -> i8), B(usize) }
    fn add3(x:i8) -> i8 { x + 3 }
    let mut a = E::A(add3);       let mut b = E::B(19);
    {
        let m1 = &mut a;              let m2 = &mut b;
        foo(m1, m2);
    }
    match b {
        E::A(_fn) => println!("b is an E::A"),
        E::B(val) => println!("b is an E::B: 0x{:x}", val),
    }

    fn foo(p1: &mut E, p2: &mut E) {
        match p1 {
            &mut E::B(..) => panic!("cannot happen"),
            &mut E::A(ref adder) => {
                /* WATCH: */ *p2 = E::B(0xBadC0de);
                println!("{}", (*adder)(14));
            }
        }
    }
}
```

The above compiles and runs just fine: the references to `a` and `b`
are copied to `p1` and `p2` respectively for the call to `foo`, `b` is
imperatively updated via `p2`, `foo` prints out `17` (= 14 + 3), and
then after `foo` returns it prints out `b is an E::B: 0xbadc0de`.

The question is: what happens when we tweak the test slightly?

### Attempted subversion via local overwrites

For example, what if we imperatively modify `*p1` instead of `*p2`,
but leave the code otherwise unchanged?

Storing the `E::B(0xBadCode)` would invalidate the data held in
`adder` (indeed, it would put "bad code" there), and so the subsequent
invocation of `adder` will jump to some (potentially invalid) memory
location and try to interpret it as executable code.

But watch:

```rust,code
#[cfg(wont_compile)]
#[test]
fn unsound_code_1() {
    enum E { A(fn (i8) -> i8), B(usize) }
    fn add3(x:i8) -> i8 { x + 3 }
    let mut a = E::A(add3);       let mut b = E::B(19);
    {
        let m1 = &mut a;              let m2 = &mut b;
        foo(m1, m2);
    }
    match b {
        E::A(_fn) => println!("b is an E::A"),
        E::B(val) => println!("b is an E::B: 0x{:x}", val),
    }

    fn foo(p1: &mut E, p2: &mut E) {
        match p1 {
            &mut E::B(..) => panic!("cannot happen"),
            &mut E::A(ref adder) => {
                /* was p2 */ *p1 = E::B(0xBadC0de);
                println!("{}", (*adder)(14));
            }
        }
    }
}
```

This will not compile.  (The `cfg` annotation above actually stops the
compiler from attempting to compile it.)  Removing the `cfg` line and
attempting to compile it yields:

```
enums.rs:569:30: 569:51 error: cannot assign to `*p1` because it is borrowed
enums.rs:569                 /* was p2 */ *p1 = E::B(0xBadC0de);
                                          ^~~~~~~~~~~~~~~~~~~~~
enums.rs:568:23: 568:32 note: borrow of `*p1` occurs here
enums.rs:568             &mut E::A(ref adder) => {
                                   ^~~~~~~~~
```

So, the compiler prevents us from overwriting the function pointer
with bogus data! (Of course, if you *really* want to shoot yourself in
the foot, you can resort to `unsafe` code to sneak the overwrite
through; at that point you are explicitly side-stepping the type
system, which Rust allows.)

### Attempted subversion via aliasing

Furthermore, the same rules that prevented the last example from
compiling are not performing a local check of just the single function
`foo`; they are enforcing a *global* soundness property.  Other
non-local attempts to subvert the type system by sneaking the
overwrite past the static checks are foiled, as shown here:

```rust,code
#[cfg(wont_compile)]
#[test]
fn unsound_code_2() {
    enum E { A(fn (i8) -> i8), B(usize) }
    fn add3(x:i8) -> i8 { x + 3 }
    let mut a = E::A(add3);       let mut b = E::B(19);
    {
        let m1 = &mut a;              let m2 = &mut b;
        foo(m1, m1);
    }
    match b {
        E::A(_fn) => println!("b is an E::A"),
        E::B(val) => println!("b is an E::B: 0x{:x}", val),
    }

    fn foo(p1: &mut E, p2: &mut E) {
        match p1 {
            &mut E::B(..) => panic!("cannot happen"),
            &mut E::A(ref adder) => {
                /* watch? */ *p2 = E::B(0xBadC0de);
                println!("{}", (*adder)(14));
            }
        }
    }
}
```

The above is attempting to create an *alias* between `p1` and `p2`,
thus causing the write to `p2` to actually overwrite the data
we matched in `p1`.

However, this example also fails to compile:

```
enums.rs:612:17: 612:19 error: cannot borrow `*m1` as mutable more than once at a time
enums.rs:612         foo(m1, m1);
                             ^~
enums.rs:612:13: 612:15 note: previous borrow of `*m1` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*m1` until the borrow ends
enums.rs:612         foo(m1, m1);
                         ^~
enums.rs:612:20: 612:20 note: previous borrow ends here
enums.rs:612         foo(m1, m1);
                               ^
```

This illustrates why Rust takes such pains to support affine typing of
`&mut T` and other owned types: It is necessary for type soundness!

## Conclusion

Thus ends our tour of `match` and enums in Rust. For more information
on details that were not covered here, such as binding via `ident @
pattern`, or the potentially subtle difference between `{ let id =
expr; ... }` versus `match expr { id => { ... } }`, consult the Rust
documentation, or quiz our awesome community (in `#rust` on IRC, or in
the [user group]).

[user group]: http://users.rust-lang.org/
