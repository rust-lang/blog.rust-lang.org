---
layout: post
title: "Launching the Lock Poisoning Survey"
author: Ashley Mannix
team: The Libs team <https://www.rust-lang.org/governance/teams/library>
---

The Libs team is looking at how we can improve the `std::sync` module, by potentially splitting it up into new modules and making some changes to APIs along the way.
One of those API changes we're looking at is non-poisoning implementations of `Mutex` and `RwLock`.
To find the best path forward we're conducting a survey to get a clearer picture of how the standard locks are used out in the wild.

The survey is a Google Form.
[You can fill it out here](https://docs.google.com/forms/d/e/1FAIpQLSehk-GkwoCag_w3YfXDfgeANulR0h5m2d3EzUMQaiY1vRfIEw/viewform).

### What is this survey for?

The survey is intended to answer the following questions:

- When is poisoning on `Mutex` and `RwLock` being used deliberately.
- Whether `Mutex` and `RwLock` (and their guard types) appear in the public API of libraries.
- How much friction there is switching from the poisoning `Mutex` and `RwLock` locks to non-poisoning ones (such as from `antidote` or `parking_lot`).

This information will then inform an RFC that will set out a path to non-poisoning locks in the standard library.
It may also give us a starting point for looking at the tangentially related `UnwindSafe` and `RefUnwindSafe` traits for panic safety.

### Who is this survey for?

If you write code that uses locks then this survey is for you.
That includes the standard library's `Mutex` and `RwLock` as well as locks from `crates.io`, such as `antidote`, `parking_lot`, and `tokio::sync`.

### So what is poisoning anyway?

Let's say you have an `Account` that can update its balance:

```rust
impl Account {
    pub fn update_balance(&mut self, change: i32) {
        self.balance += change;
        self.changes.push(change);
    }
}
```

Let's also say we have the invariant that `balance == changes.sum()`.
We'll call this the _balance invariant_.
So at any point when interacting with an `Account` you can always depend on its `balance` being the sum of its `changes`, thanks to the balance invariant.

There's a point in our `update_balance` method where the balance invariant isn't maintained though:

```rust
impl Account {
    pub fn update_balance(&mut self, change: i32) {
        self.balance += change;
//      self.balance != self.changes.sum()
        self.changes.push(change);
    }
}
```

That seems ok, because we're in the middle of a method with exclusive access to our `Account` and everything is back to good when we return.
There isn't a `Result` or `?` to be seen so we know there's no chance of an early return before the balance invariant is restored. Or so we think.

What if `self.changes.push` didn't return normally?
What if it panicked instead without actually doing anything?
Then we'd return from `update_balance` early without restoring the balance invariant.
That seems ok too, because a panic will start unwinding the thread it was called from, leaving no trace of any data it owned behind.
Ignoring the `Drop` trait, no data means no broken invariants.
Problem solved, right?

What if our `Account` wasn't owned by that thread that panicked?
What if it was shared with other threads as a `Arc<Mutex<Account>>`?
Unwinding one thread isn't going to protect other threads that could still access the `Account`, and they're not going to know that it's now invalid.

This is where poisoning comes in.
The `Mutex` and `RwLock` types in the standard library use a strategy that makes panics (and by extension the possibility for broken invariants) observable.
The next consumer of the lock, such as another thread that didn't unwind, can decide at that point what to do about it.
This is done by storing a switch in the lock itself that's flipped when a panic causes a thread to unwind through its guard.
Once that switch is flipped the lock is considered _poisoned_, and the next attempt to acquire it will receive an error instead of a guard.

The standard approach for dealing with a poisoned lock is to propagate the panic to the current thread by unwrapping the error it returns:

```rust
let mut guard = shared.lock().unwrap();
```

That way nobody can ever observe the possibly violated balance invariant on our shared `Account`.

That sounds great! So why would we want to remove it?

### What's wrong with lock poisoning?

There's nothing wrong with poisoning itself.
It's an excellent pattern for dealing with failures that can leave behind unworkable state.
The question we're really asking is whether it should be used by the _standard locks_, which are `std::sync::Mutex` and `std::sync::RwLock`.
We're asking whether it's a standard lock's job to implement poisoning. Just to avoid any confusion, we'll distinguish the poisoning pattern from the API of the standard locks by calling the former _poisoning_ and the latter _lock poisoning_.
We're just talking about lock poisoning.

In the previous section we motivated poisoning as a way to protect us from possibly broken invariants.
Lock poisoning isn't actually a tool for doing this in the way you might think.
In general, a poisoned lock can't tell whether or not any invariants are _actually_ broken.
It assumes that a lock is shared, so is likely going to outlive any individual thread that can access it.
It also assumes that if a panic leaves any data behind then it's more likely to be left in an unexpected state, because panics aren't part of normal control flow in Rust.
Everything _could_ be fine after a panic, but the standard lock can't guarantee it.
Since there's no guarantee there's an escape hatch.
We can always still get access to the state guarded by a poisoned lock:

```rust
let mut guard = shared.lock().unwrap_or_else(|err| err.into_inner());
```

All Rust code needs to remain free from any possible undefined behavior in the presence of panics, so ignoring panics is always safe.
Rust doesn't try guarantee all safe code is free from logic bugs, so broken invariants that don't potentially lead to undefined behavior aren't strictly considered unsafe.
Since ignoring lock poisoning is also always safe it doesn't really give you a dependable tool to protect state from panics.
You can always ignore it.

So lock poisoning doesn't give you a tool for guaranteeing safety in the presence of panics.
What it does give you is a way to propagate those panics to other threads.
The machinery needed to do this adds costs to using the standard locks.
There's an ergonomic cost in having to call `.lock().unwrap()`, and a runtime cost in having to actually track state for panics.

With the standard locks you pay those costs whether you need to or not.
That's not typically how APIs in the standard library work.
Instead, you compose costs together so you only pay for what you need.
Should it be a standard lock's job to synchronize access _and_ propagate panics?
We're not so sure it is.
If it's not then what should we do about it?
That's where the survey comes in.
We'd like to get a better idea of how you use locks and poisoning in your projects to help decide what to do about lock poisoning.
[You can fill it out here](https://docs.google.com/forms/d/e/1FAIpQLSehk-GkwoCag_w3YfXDfgeANulR0h5m2d3EzUMQaiY1vRfIEw/viewform).
