---
layout: post
title: "Const evaluatable feature compatability"
author: Bastian Kauschke
team: the lang team <https://www.rust-lang.org/governance/teams/lang>
---

In version 1.43.0 we accidentially allowed the usage of generic parameters in the length of repeat expressions
as long as they are not relevant for the final value of the constant. We now emit a future compatability lint
in these cases which will get changed to a hard error in the far future. There might be a subset of affected code
which will instead be allowed without requiring any changes, but this still needs further consideration.
See [#76200] for more details.

[#76200]:https://github.com/rust-lang/rust/issues/76200

## Examples of affected code

```rust
struct Foo<T>;

impl Foo<T> {
    const ASSOC: usize = 4;
    
    fn foo() {
        // `Self` is `Foo<T>` here and therefore mentions `T` => future compat warning
        let _ = [0; Self::ASSOC];
    }
}

fn test<T>() {
    // The repeat expression mentions `T` and relies on it => already a hard error
    let _ = [0; std::mem::size_of::<T>()];
    // The repeat expression mentions `T` but does not rely on it => future compat warning
    let _ = [0; std::mem::size_of::<*mut T>()];
    let _ = [0; Foo::<T>::ASSOC];
    // The repeat expression does not mention any generic paramters => ok
    let _ = [0; std::mem::size_of::<*mut ()>()];
}
```
## What should I do if my code is affected?

As affected code does not use the mentioned generic parameters, you should be
able to replace them with dummy types to remove this warning. For generic associated
constants it is also possible to move these into a concrete context.

```rust
fn test<T>() {
    let _ = [0; std::mem::size_of::<*mut T>()]; // warns
    let _ = [0; std::mem::size_of::<*mut ()>()]; // does not warn
}
```
