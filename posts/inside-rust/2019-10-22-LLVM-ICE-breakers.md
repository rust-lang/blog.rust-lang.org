---
layout: post
title: "Announcing the LLVM ICE-breaker group"
author: Niko Matsakis
description: "A new blog where the Rust team can post updates on the latest developments"
team: the compiler team <https://www.rust-lang.org/governance/teams/compiler>
---

Today I'm announcing a new experiment in the compiler team, the **LLVM ICE-breaker group**. If you're familiar with LLVM and would like to contribute to rustc -- but without taking on a large commitment -- then the LLVM ICE-breaker group might well be for you!

### What is the LLVM ICE-breaker group?

At its heart, the LLVM ICE-breaker group is just a list of people who would like to be notified when we come across LLVM bugs. You can add yourself to this list very easily -- just [open a PR]! When we come across a suitable bug, we'll write a message that `@`-mentions every Github user on that list. If you have some time, maybe you can fix one of them, or at least offer some words of wisdom to help somebody else figure out what's going on.

[open a PR]: https://rust-lang.github.io/rustc-guide/ice-breaker/about.html#join

There are a few other things associated with the group too, however. For example, we've got a [guide] that offers some tips for how to fix LLVM-related bugs and may help you get started (particularly if you're not that familiar with rustc).

[guide]: https://rust-lang.github.io/rustc-guide/ice-breaker/llvm.html

### What kind of bugs are we talking about?

The goal is to identify "self-contained" bugs that are unlikely to require large-scale compiler refactorings or to get entangled in other big projects.

### Who should join?

This group is a great fit for anyone who is familiar with LLVM and who would like to learn more about Rust or to get more involved in the Rust project. Even if you don't have time to open PRs against rustc, there are many other ways to help:

* identifying open LLVM bugs that we may be running into;
* reducing Rust sources to minimal reducers;
* reducing LLVM IR to minimal problem cases;
* and so forth.

(For that matter, feel free to join the group even if you're *not* all that familiar with LLVM -- no better way to learn!)

### What's with the name ICE-breaker anyway?

An "ICE" is an "internal compiler error". It actually refers specifically to the case where the compiler panics (which tends to be an easy sort of bug to fix). In fact, very few LLVM bugs cause real ICEs, but the name was too good to pass up.

But of course we also hope that these ICE-breaker groups can help people to get more acquainted with hacking on rustc; to ["break the ice"], if you will (ha! I kill me).

["break the ice"]: https://en.wiktionary.org/wiki/break_the_ice

### Will there be more ICE-breaker groups?

I certainly hope so! As I mentioned before, this is an experiment, but presuming that it works out well we fully intend to create more ICE-breaker groups.

### So how do I sign up again?

Easy! Just [open a PR]!
