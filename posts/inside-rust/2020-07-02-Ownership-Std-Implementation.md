+++
layout = "post"
date = 2020-07-02
title = "Ownership of the standard library implementation"
author = "Ashley Mannix"
team = "The Libs team <https://www.rust-lang.org/governance/teams/library>"
+++

Our Rust project is a large and diverse one. Its activities are broadly coordinated by teams that give the community space to find and contribute to the things that matter to them.
We’re trialing a reorganization of standard library activities between the Libs and Compiler teams.
Going forward, the Libs team will own just the public API of the standard library, and the Compiler team will own its implementation.
The goal of this separation of concerns is to better suit the interests of both teams to better support the needs of the standard library.
It's a lot like the existing relationship between the Lang and Compiler teams, where the Lang team owns the Rust language design and the Compiler team owns the code that implements it.
We'll re-evaluate how the trial is going later in the year and decide whether or not to make the change permanent.

The Libs team traditionally selects members who like to design APIs.
A lot of bandwidth is spent supporting libraries in the wider Rust ecosystem and working to consolidate idioms into standard APIs.
This leaves little room for development of the standard library itself, which takes a lot of consistent and dedicated attention.

As a codebase, the standard library is paradoxically specialized.
It has privileged access to compiler internals, deep domain knowledge baked into algorithms (have you ever wondered what it takes to efficiently format a float as text for instance?), platform-specific integration, and a lot of tricky unsafe code.

The Compiler team is used to giving consistent and dedicated attention to big projects.
The standard library is exactly the kind of codebase the Compiler team already has years of experience working on.

Teams aren’t bubbles though, and in practice API design and implementation are going to influence each other.
This is just a shared understanding between the Libs and Compiler teams to make standard library activities more focused.

Do any of those activities appeal to you?
Maybe you’re interested in identifying and capturing idioms as standard APIs.
If so, you can find the Libs team [here](https://forge.rust-lang.org/libs/index.html).
Maybe you’d like to work on a big codebase used by almost every Rust developer.
If so, you can find the Compiler team [here](https://forge.rust-lang.org/compiler/index.html).
Maybe you like the sound of both and anything in-between! Whatever the case, the standard library has something for you.
