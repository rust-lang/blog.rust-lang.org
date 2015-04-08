---
layout: post
title: "Enums: match, mutation, and moves"
author: Felix S. Klock II
description: "A tour of enums in Rust."
---

In this post we explore one corner of the Rust language: defining a
class of data via `enum`, and processing instances of such data via
`match`.

Most of the post will use a single running example: modeling a game of
chess. At the end the post changes gears, shifting to a discussion of
how affine-typing ensures that the demonstrated features do not
introduce unsoundness.

## Tutorial material

The post takes the overall form of a tutorial.

If you are already familiar with defining `enums` and simple uses of
`match`, you may want to skip ahead to the [mutating enums] or
[avoiding unsoundness] sections. This first section is meant to
establish everything one would need to know about enums to understand
the discussion in latter two sections.

[mutating enums]: #mutating-enums
[avoiding unsoundness]: #avoiding-unsoundness

### Defining simple enums

The pieces of chess can be classified as follows:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceShape {
    King, Queen, Rook, Bishop, Knight, Pawn,
}
```

The `derive` line automatically generates support for three common
operations: the `Copy, Clone` gives us the ability to freely copy or
clone instances of `PieceShape`, the `Debug` provides support for
printing each piece-rank into an output buffer, e.g. via
`println!("piece: {:?}", piece)`, and `PartialEq, Eq` provides support
for the `==` operator on instances of the enum.

### Enum discriminant values

An enum like `PieceShape` above, where each variant is just a name, is
represented using an integer value for each variant. The definition
style above allows the Rust compiler to assign the values itself; it
will start from 0 and count upwards.

Alternatively, one can manually assign specific values to enum
variants, interrupting the automatic value assignment performed by the
compiler. Thus, we could instead have written the below, and get the
same assignment of discriminant values:

```rust
#[cfg(alternative)]
#[derive(Copy, Clone, Debug)]
pub enum PieceShape {
    Pawn = 5, Rook = 2, Bishop, Knight, King = 0, Queen, 
}
```

These enums, where each variant is just a name with an associated
(implicitly- or explicitly-assigned) integer value, are known as
"C-style enums" in Rust parlance, since the syntax largely matches
that used for the `enum` construct in the C language.

(However, note that not every C enum immediately corresponds to an
analogous enum in Rust; in particular, Rust enforces an invariant that
each variant be distinct from all other variants in the enum, while C
allows one to assign the same integer value to multiple enum
variants.)

### Constructing enum instances

After defining an enum, one accesses the variants it defines via the
path syntax `enum_name::variant_name`. So using these pieces is
straight-forward enough:

```rust
#[test]
fn demo_debug_format() {
    let q = PieceShape::Queen;
    let p = PieceShape::Pawn;
    let k = PieceShape::King;
    println!("q={:?} p={:?} k={:?}", q, p, k);
}
```

Of course, it can be annoying to have to type the enum name
repeatedly. To make the variants directly accessibly, import
them via `use`.

```rust
#[test]
fn demo_debug_format_2() {
    use self::PieceShape::{Queen, Pawn, King};
    let q = Queen;
    let p = Pawn;
    let k = King;
    println!("q={:?} p={:?} k={:?}", q, p, k);
}
```

or even more simply:

```rust
use self::PieceShape::*;
```

(Note that we have used `use self::...` here; the `use`-import syntax
resolves paths relative to the crate-root by default, but the leading
`self` changes the mode so that the resolution is relative to the
containing module.)

### Destructuring enums via `match`

Once we have created enum instances, we can process them via `match`
expressions. Here, we map each piece rank to a single letter code.

```rust
pub fn one_letter(r: PieceShape) -> char {
    match r {
        Pawn   => 'P',
        Rook   => 'R',
        Knight => 'N',
        Bishop => 'B',
        Queen  => 'Q',
        King   => 'K',
    }
}

#[test]
fn demo_one_letter() {
    assert_eq!(one_letter(Queen), 'Q');
}
```

Or as a more complex example, we can gather a sequence of shapes into a
character string:

```rust
#[test]
fn demo_one_letter_collect() {
    let qpk = [Queen, Pawn, King];
    let qpk: String = qpk.iter().map(|r|{one_letter(*r)}).collect();
    assert_eq!(qpk, "QPK");
}
```

Encoding shapes via ASCII characters has worked fine since the 1960's,
but since we live in a more modern age, it is tempting to find out
whether we could encode our pieces with Unicode pictorial symbols.

Indeed, reviewing the Unicode character charts, we discover that the
chess pieces have dedicated characters, but we see that there are
twelve such characters (starting from `U+2654`, "WHITE CHESS KING"),
not six. This reveals an oversight in our data description thus far:
We have represented shapes, but not the *color* of the piece.

### Enums with payloads

Here is one way of many to add color to our pieces, via a new `enum`
which represents one square of a chess board.

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Piece {
    Black(PieceShape),
    White(PieceShape),
}
```

This demonstrates a twist on `enum` that Rust provides.
Unlike an `enum` in C, which can only define a collection of names,
each variant of a Rust enum can optionally carry a payload of data.

### Matching tree-structured patterns

With this in hand, we can render a piece in a manner similar to how we
converted a shape to a letter in `one_letter` above.

```rust
fn render(p: Piece) -> char {
    match p {
        Piece::White(King)   => '\u{2654}',
        Piece::White(Queen)  => '\u{2655}',
        Piece::White(Rook)   => '\u{2656}',
        Piece::White(Bishop) => '\u{2657}',
        Piece::White(Knight) => '\u{2658}',
        Piece::White(Pawn)   => '\u{2659}',

        Piece::Black(King)   => '\u{265A}',
        Piece::Black(Queen)  => '\u{265B}',
        Piece::Black(Rook)   => '\u{265C}',
        Piece::Black(Bishop) => '\u{265D}',
        Piece::Black(Knight) => '\u{265E}',
        Piece::Black(Pawn)   => '\u{265F}',
    }
}

#[test]
fn demo_render() {
    let row = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
    let black_row: String = row.iter().map(|s| render(Piece::Black(*s))).collect();
    let white_row: String = row.iter().map(|s| render(Piece::White(*s))).collect();
    println!("black_row: {}", black_row);
    println!("white_row: {}", white_row);
}
```

The `demo_render` test prints:

```
black_row: ♜♞♝♛♚♝♞♜
white_row: ♖♘♗♕♔♗♘♖
```

which is well on its way to a fully-rendered chess board.

### Binding in match patterns

Here is an alternative way to write the `render` function.

```rust
fn render_2(p: Piece) -> char {
    use std::char;
    let offset = match p {
        Piece::White(shape) => shape as u32,
        Piece::Black(shape) => shape as u32,
    };
    let king_unicode_value = match p {
        Piece::White(_) => 0x2654,
        Piece::Black(_) => 0x265A,
    };
    char::from_u32(king_unicode_value + offset).unwrap()
}

#[test]
fn check_renders() {
    for &r in &[King, Queen, Rook, Bishop, Knight, Pawn] {
        assert_eq!(render(Piece::White(r)),
                   render_2(Piece::White(r)));
        assert_eq!(render(Piece::Black(r)),
                   render_2(Piece::Black(r)));
    }
}
```

The above code illustrates a collection of features:

 * Rather than matching it as constant, one can bind the payload
   attached to a enum-variant to an identifier.  Thus the patterns in
   the first `match` expression are each binding `shape` to the
   corrresponding shape-payload attached to the piece.

 * One can cast a C-style enum value to the integer value used to
   represent it, via the syntax `enum_value as integer_type`.

 * One can use `_` to ignore substructure in a pattern.  This is used
   in the second match expression, so that the shape held in `p` is
   left in place. (This does not matter too much since `Shape`
   implements `Copy`; it is often more important in cases where the
   payload is not copyable, as further discussed below.)

Thus, the `White` and `Black` `match`-clauses in the above code bind
`shape` to the associated `PieceShape`, and then casts that shape to
the appropriate integer offset from the king in the unicode codepoint
assignment. (The integers in our original enum definition for
`PieceShape` were selected so that they would match the sequence of
piece ranks in the unicode codepoint assignment.)

## Mutating enums

One event that occurs in chess is that when a pawn reaches the eighth
rank of the board, it is promoted to a another piece of the same color
(usually a queen).

It is easy to write such a promotion in a functional programming
style:

```rust
fn promote_to_queen_functional(p: Piece) -> Piece {
    match p {
        Piece::Black(_) => Piece::Black(Queen),
        Piece::White(_) => Piece::White(Queen),
    }
}

#[test]
fn demo_promote_functional() {
    let p = Piece::White(Pawn);
    assert_eq!(promote_to_queen_functional(p), Piece::White(Queen));
}
```

Let us imagine, however, that we want to model an actual chess board,
and we want to implement promotion as an *in-place* modification; that
is, instead of constructing a whole new copy of the board, we want to
instead use an imperative update to represent the promotion.

### A first bumbling attempt

Here is a first, admittedly bumbling, attempt to implement in-place
promotion from a pawn to a queen: It just removes the `-> Piece`
return type and shoehorns an assignment into the code, in the hopes
that everything will work out.

So, first try:

```rust
#[cfg(promote_attempt_1)]
fn promote_to_queen_1(mut pawn: Piece) {
    pawn = match pawn {
        Piece::Black(_) => Piece::Black(Queen),
        Piece::White(_) => Piece::White(Queen),
    };
}

#[cfg(promote_attempt_1)]
#[test]
fn demo_promote_1() {
    let p = Piece::White(Pawn);
    assert_eq!(p, Piece::White(Pawn));
    promote_to_queen_1(p);
    assert_eq!(p, Piece::White(Queen));
}
```

Compiling this (enabling it via `--cfg promote_attempt_1`) yields:

```
enums.rs:238:5: 238:9 warning: value assigned to `pawn` is never read, #[warn(unused_assignments)] on by default
enums.rs:238     pawn = match pawn {
                 ^~~~
```

So that's not a good sign. Then running it yields:

```
test demo_promote_1 ... FAILED

failures:

---- demo_promote_1 stdout ----
thread 'demo_promote_1' panicked at 'assertion failed: `(left == right)` (left: `White(Pawn)`, right: `White(Queen)`)', enums.rs:250
```

This demonstrates, among other things, that static analysis cannot
stop someone who is determined to write buggy code. But the assertion
failure message is fairly clear about what the problem is: We had
hoped the piece `p` would be replaced with a queen, but it is in fact
still a pawn.

The crucial mistake being made here is that if you want to mutate a
piece of state in place, then you need to have a mutable *reference*
to that state. The `mut pawn` formal argument to `promote_to_queen` is
indeed mutable, but those mutations are not visible from outside the
function, because the argument has already been moved into the
parameter's location. The invocation `promote_to_queen` is not passing
a reference (mutable or otherwise) to `p`; it is creating a *copy* of
the `Piece`.

### A second stumble, caught at compile-time

To make this really concrete, let us consider another kind of data in
our chess board.  A chess board is made up of squares, usually
arranged in an 8-by-8 grid. Each `Square` is either unoccupied
(`Empty`) or is occupied by a single piece.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
enum Square {
    Empty,
    Occupied(Piece),
}
```

Note that we have left off the `Copy` derivation; the intention is
that while `Rank` and `Piece` can be freely copied via assignment
statements, the square on a chess board has its own identity, and
should not be freely copied via assignment statements.

With that in mind, let us try writing `promote_to_queen` again, but
this time passing in a `Square`.

(This example is also taking advantage of another feature of Rust's
`match` syntax: one can combine multiple match arms that have
identical code into one arm by writing all their patterns pairwise
separated by a vertical bar (`|`)):


```rust
#[cfg(promote_attempt_2)]
fn promote_to_queen_2(mut s: Square) {
    match s {
        Square::Empty => {}
        Square::Occupied(Piece::White(mut shape)) |
        Square::Occupied(Piece::Black(mut shape)) => {
            shape = Queen
        }
    }
}

#[cfg(promote_attempt_2)]
#[test]
fn demo_promote_2() {
    let mut s = Square::Occupied(Piece::White(Pawn));
    promote_to_queen_2(s);
    assert_eq!(s, Square::Occupied(Piece::White(Queen)));
}
```

This second attempt has the same mistake that we made with
`promote_to_queen_2`: we again have failed to take a mutable reference
to the square, so no in-place update is possible. But *now* we get a
compile-time error:

```
<std macros>:3:11: 3:23 error: use of moved value: `s`
<std macros>:3 match ( & ( $ left ) , & ( $ right ) ) {
                         ^~~~~~~~~~~~
<std macros>:1:1: 9:39 note: in expansion of assert_eq!
enums.rs:319:5: 319:58 note: expansion site
enums.rs:318:24: 318:25 note: `s` moved here because it has type `Square`, which is non-copyable
enums.rs:318     promote_to_queen_2(s);
                                    ^
```

The error message is telling us exactly what happened: the argument to
`promote_to_queen_2` is moved when the function is called; thus the
attempt to reference `s` in the `assert_eq!` invocation fails, because
`s` has been moved.

(This illustrates why one might *not* add `derive(Copy)` to every data
type where it would otherwise be legal; leaving out `derive(Copy)` can
catch certain bugs.)

### Try, try again; taking a reference

So, here's a third try, where we take a reference to the `Square`.

```rust
#[cfg(promote_attempt_3)]
fn promote_to_queen_3(s: &mut Square) {
    match s {
        &mut Square::Empty => {}
        &mut Square::Occupied(Piece::White(mut shape)) |
        &mut Square::Occupied(Piece::Black(mut shape)) => {
            shape = Queen
        }
    }
}

#[cfg(promote_attempt_3)]
#[test]
fn demo_promote_3() {
    let mut p = Square::Occupied(Piece::White(Pawn));
    promote_to_queen_3(&mut p);
    assert_eq!(p, Square::Occupied(Piece::White(Queen)));
}
```

We again see the warning:

```
enums.rs:355:44: 355:49 warning: variable `shape` is assigned to, but never used, #[warn(unused_variables)] on by default
enums.rs:355         &mut Square::Occupied(Piece::White(mut shape)) |
                                                        ^~~~~~~~~
enums.rs:357:13: 357:14 warning: value assigned to `shape` is never read, #[warn(unused_assignments)] on by default
enums.rs:357             shape = Queen
                         ^~~~~
```

This warning is again warranted, as the test fails:

```
test demo_promote_3 ... FAILED

failures:

---- demo_promote_3 stdout ----
thread 'demo_promote_3' panicked at 'assertion failed: `(left == right)` (left: `Occupied(White(Pawn))`, right: `Occupied(White(Queen))`)', enums.rs:367
```

### Keeping `ref`s into referenced data

So, what happened here?

The answer is: Even though we have ensured that `Square` is not
accidentally copied, there is nothing stopping the *contents* of
`Square` from being copied. So in a `match` arm like
```
&mut Square::Occupied(Piece::Black(mut shape)) => { ... }
```
if the discriminant value matches that arm, then it will put a
copy of its shape into `shape`.

This brings us to a very important point about how `match` works in
Rust: it matches its inputs in-place, but normal bindings like the
`mut shape` above will move (or copy) the matched substructure into
fresh locations.

If you want to capture a reference into the matched substructure,
either to avoid extraneous copying, or to enable modification of the
original input value, then you need to use a `ref`-pattern.

With this knowledge in hand, let us try again.

In addition, our fourth try simplifies the code slightly by
dereferencing the argument `*s`.

```rust
fn promote_to_queen_4(s: &mut Square) {
    match *s {
        Square::Empty => {}
        Square::Occupied(Piece::White(ref mut shape)) |
        Square::Occupied(Piece::Black(ref mut shape)) => {
            *shape = Queen
        }
    }
}

#[test]
fn demo_promote_4() {
    let mut p = Square::Occupied(Piece::White(Pawn));
    promote_to_queen_4(&mut p);
    assert_eq!(p, Square::Occupied(Piece::White(Queen)));
}
```

Compiling and running our code now yields:

```
running 7 tests
test demo_debug_format ... ok
test demo_one_letter_collect ... ok
test demo_one_letter ... ok
test demo_debug_format_2 ... ok
test demo_promote_4 ... ok
test check_renders ... ok
test demo_promote_functional ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured
```

Huzzah, it works!

The dereference `*s` works because `match` works on both "L-value" and
"R-value" expressions; L-value expressions evaluate to memory
locations, while R-value expressions evaluate to values that are then
stored, if necessary, in temporary memory locations. It simplified our
code because allowed us to sidestep writing `&mut` at the beginning of
each of our patterns, which was not such a great burden for this tiny
code snippet, but can be much more annoying when working with `match`
expressions that have many more arms, amplifying the redundancy.

## Avoiding unsoundness

(This section is adapted from the Rust demo we presented at the 2014
ML workshop.)

A potentially non-obvious issue arises when a supposedly-sound
language adds support for `ref mut`, or even just `ref`, in a
non-moving/non-copying `match` expression: How does one prevent
someone from injecting unsound behavior by using mutable references to
overwrite state secretly, subverting the type-system?

Consider the following code:

```rust
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

```rust
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

As you might have inferred from the `cfg` annotation above, this
will not compile. Attempting to compile it yields:

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

```rust
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

Thus ends our tour of enums in Rust. For more information on details
that were not covered here, such as binding via `ident @ pattern`, or
the potentially subtle difference between `{ let id = expr; ... }`
versus `match expr { id => { ... } }`, consult the Rust documentation,
or quiz our awesome community (in `#rust` on IRC, or in the
[user group]).

[user group]: http://users.rust-lang.org/
