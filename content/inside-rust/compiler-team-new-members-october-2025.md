+++
path = "inside-rust/2025/10/28/compiler-team-new-members"
title = "Announcing seven new members of the compiler team"
authors = ["davidtwco and wesleywiser"]

[extra]
team = "the compiler team"
team_url = "https://www.rust-lang.org/governance/teams/compiler"
+++
Just [a few months ago we announced five new members of the compiler team][blog_prev_addition] but now we have seven more members to announce:

- [yaahc](https://github.com/yaahc), team member
    - Jane has been making significant progress on the [Metrics Initiative](https://rust-lang.github.io/rust-project-goals/2025h1/metrics-initiative.html) compiler team project goal to help us quantify unstable feature usage, as well as the "Relink Don't Rebuild" (RDR) project to speed up recompilation of dependency trees when the surface area of a crate hasn't changed.

- [GuillameGomez](https://github.com/GuillaumeGomez), team member
    - Guillaume has been a long time contributor to Rust and is the team lead of the Rustdoc team. In addition to his extensive work on Rustdoc, Guillame is also a frequent contributor to the GCC backend for the Rust compiler.

- [Amanieu](https://github.com/Amanieu), team member
    - Amanieu is also a long time contributor to Rust and is the Library team lead. Amanieu often provides expertise regarding low-level language features like inline assembly and target-specific features.  

- [Enselic](https://github.com/Enselic), team member
    - Enselic has been contributing to issue triage for many years and has made lots of great contributions to throughout the compiler with a focus lately on revitalizing and improving the compiler integration test suite.

- [dianne](https://github.com/dianne), team member
    - Dianne has been heavily involved with improvements to match ergonomics, the implementation of deref patterns and some significant bug fixes to temporary lifetime issues. 

- [JonathanBrouwer](https://github.com/JonathanBrouwer), team member
    - Jonathan has been instrumental in the ongoing effort to refactor the Rust compiler's handling of attributes which has greatly improved the quality of the compiler's code as well as implementing missing validation for attribute arguments.

- [tiif](https://github.com/tiif), team member
    - Tiif has helped contribute a variety of fixes for bugs, implemented support in the compiler for unstable trait impls and improvements to const generics.


Thank you all for your contributions!

[blog_prev_addition]: https://blog.rust-lang.org/inside-rust/2025/05/30/compiler-team-new-members/
