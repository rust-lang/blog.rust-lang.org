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

* Structural pattern matching of algebraic data types: The `match`
  construct enables case analysis with ergonomics that are vastly
  improved over that provided by a C or Java style `switch` statement.

* Exhaustive case analysis, which ensures that no case is omitted
  when processing an input.

* `match` embraces both imperative and applicative styles of
  programming: The compiler's static analyses work hard to ensure
  statement-oriented programming remains palatable, rather than
  forcing everyone to adopt an expression-oriented mindset.

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

```rust
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

```rust
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

```rust
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

```rust
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

```rust
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
initialized before it is referenced, but the designers have strived to
avoid requiring artifical coding patterns inserted solely to placate
Rust's static analyses (such as requiring one to initialize `string`
above with some dummy data just so that it can be borrowed later).


### Algebraic Data Types and Data Invariants

An `enum` type allows one to define mutually-exclusive classes of
values. The examples shown above used `enum` for simple symbolic tags,
but in Rust, such definitions can define much richer classes of data.

For example, a binary tree is either a leaf, or an internal node with
references to two child trees. Here is one way to encode a tree of
integers in Rust:

```rust
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

```rust
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

```rust
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
taking ownership. In particular, the input to `match` is an *L-value
expression*; this means that the input expression is evaluated to a
*memory location* where the value lives (as opposed to an R-value
expression, which conceptually evaluates to the value itself). Then
`match` works by inspecting the data at that location.

(If the input expression is a variable name or a field/pointer
dereference, then the L-value is just the location of that variable or
field/memory.  If the input expression is a function call or other
operation that generates an unnamed temporary value, then it will be
conceptually stored in a temporary area, and that is the memory
location that `match` will inspect.)

So, if we want a version of `tree_weight` that merely borrows a tree
rather than taking ownership of it, then we will need to make use of
this feature of Rust's `match`.

```rust
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

This code demonstrates this concept by incrementing all of the
values in a given tree.

```rust
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

Note that the code above now binds `payload` by a `ref mut`-pattern;
if it did not use a `ref` pattern, then payload would be bound to a
local copy of the integer, while we want to modify the actual integer
*in the tree itself*. Thus we need a reference to that integer.

Note also that the code is able to bind `left` and `right`
simultaneously in the `Node` arm. The compiler knows that the two
values cannot alias, and thus it allows both `&mut`-references to live
simultaneously.

## Conclusion

Thus ends our tour of `match` and enums in Rust. For more information
on details that were not covered here, such as binding via `ident @
pattern`, or the potentially subtle difference between `{ let id =
expr; ... }` versus `match expr { id => { ... } }`, consult the Rust
documentation, or quiz our awesome community (in `#rust` on IRC, or in
the [user group]).

[user group]: http://users.rust-lang.org/
