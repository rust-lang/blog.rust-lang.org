+++
path = "inside-rust/2026/06/04/how-josh-helps-rust-manage-code-across-multiple-repositories"
title = "How Josh helps Rust manage code across multiple repositories"
authors = ["Jakub Beránek", "Ralf Jung"]
+++

The Rust Project develops and maintains several developer tools, such as Cargo, Clippy, rustfmt, Rust Analyzer and Miri. These tools are developed in separate git repositories, which enables customizing their issue and pull request (PR) management, Continuous Integration (CI) workflows and development processes based on the needs of the specific teams that maintain them. However, these tools also need to be somehow integrated within the main [`rust-lang/rust`][rust-lang/rust] repository, primarily for the following two reasons:

- The CI of this central repo is responsible for distributing the Rustup components containing these tools, which we then ship to Rust programmers.
- When we make a breaking change to the (unstable) internal compiler API, on which some of these tools depend, we sometimes need to fix them in the same pull request, to ensure that they keep working on the `nightly` toolchain.

This means that we have to deal with the classical problem of managing development of code 
across several repositories that depend on one another. This post covers our experience and some problems that we encountered with this cross-repository code sharing and describes how we leverage an awesome git tool called [Josh][Josh] (Just One Single History), which helps us overcome these challenges.

## Problem statement

First, let us define more concretely what is the problem that we are trying to solve. We have one "parent" repository (in our case `rust-lang/rust`), which has a set of dependencies, which we will call "subprojects" (e.g. `rust-lang/miri` or `rust-lang/rust-analyzer`). The parent and the subprojects are developed independently. The parent must have access to a specific snapshot of the subproject's source code. Periodically, the subproject's code in the parent repo should be updated to a newer version. And ideally, the parent should be able to atomically change both its and the subproject's source code.

There are several traditional ways of attempting to solve this issue, which did not fully work for us for various reasons.

## Monorepo

If both the parent and the subprojects all live inside the same repository (a "monorepo"), then the desired properties described above are trivially satisfied. The parent always sees the latest version of each subproject, and it is possible to atomically change both the parent and the subproject inside a single pull request. While this approach has many benefits, and `rust-lang/rust` *is* conceptually a monorepo (it contains the source code of the compiler, the standard library, rustdoc and many other things), it also causes some friction. The compiler repo is quite large, which can be scary for new contributors, and it has quite slow and complex CI. Its development processes are also mostly tuned to the Compiler team's needs, but our developer tools are maintained by other teams that use different review or issue tracking practices. 

Keeping some subprojects (such as Miri or Rust Analyzer) in their own repositories thus makes their CI faster, enables them to be deployed separately from the main Rust toolchain, and simplifies onboarding of new maintainers, who can more easily obtain scoped permissions to merge PRs only for the given subproject, and not "all of Rust".

## git submodules

Perhaps the most straightforward way of including a git repository in another one is to use git submodules. We currently use submodules for [some][submodules] dependencies, such as Cargo and LLVM. Their benefit is that they are easy to set up, you simply point a directory to a specific commit SHA of an external repository, and that's it. Unfortunately, that is where most of its advantages end.

In practice, submodules can be quite annoying to work with. Developers have to properly checkout the submodules using e.g. `git clone --recursive` or `git submodule update`, otherwise they can remain empty or contain the wrong commit. It is in fact a relatively common mistake for someone to accidentally commit and push an unrelated submodule change to their pull request branch, because git sometimes keeps submodules in a "dirty" state when switching branches. We had to build custom logic to checkout specific submodules to the correct commit when building a given Rust artifact in our build tooling (`bootstrap`) to alleviate this, but it is not perfect.

Another big disadvantage of submodules is that they do not allow atomically modifying the subproject and the parent in a single PR, because the submodule is fundamentally only a link to a repository. When we make (an internal) breaking change in `rust-lang/rust`, we would thus have to:

1. Merge the `rust-lang/rust` PR with the breaking change
2. Update the subproject in a separate subproject PR
3. Merge another `rust-lang/rust` PR which updates its submodule to point to the new subproject commit created in step 2.

This is a lot of additional busywork, which makes it harder to land changes that affect both the parent and the subproject. Sometimes it is near impossible to make such a change without at least temporarily breaking the CI of one of those two repositories, or producing a `nightly` release where the tool would stop working.
We used such a [system](https://rust-lang-nursery.github.io/rust-toolstate/) for Clippy, Miri and a few other tools in the past, and both the user and developer experience was extremely poor.

## git subtrees

Since submodules are not a great fit for tools that are deeply integrated with the compiler (such as Miri or Rust Analyzer), we historically used git subtrees for them instead. A git subtree essentially integrates the whole repo of the subproject into the parent, which then directly contains all of its source code and its whole git history, rather than just a link (as with submodules). It is thus possible to change both the parent and the subproject's code in a single pull request, to e.g. fix the subproject tests after something changes in the compiler, which is very useful.

The price that has to be paid to enable this workflow is that we need to periodically perform bidirectional syncs between the parent and each subproject that is embedded as a subtree.
There are two kinds of syncs:

- A `pull` operation ports any changes made to the subproject in the parent repo back into the repository of the subproject.
- A `push` operation updates the subproject's source code in the parent based on the latest commit of the subproject repository. 

Performing a sync means that someone has to run a script to do a pull or a push, resolve any merge conflicts or test failures that may arise, and submit a PR to the corresponding repository.

This workflow conceptually worked pretty well for us, but we had a severe problem with the implementation of git subtree: it is *painfully* slow.
The official upstream version is entirely unusable for a repo of our size.
There exists a patch that was never merged upstream that made git subtree fast enough for Clippy and a few other tools. However, on Miri, likely due to its complex history that involves moving a large chunk of code from Miri to rustc in a history-preserving way many years ago, git subtree became entirely unusable: even after many hours of waiting, the sync would simply not finish.

Apart from performance, we also encountered some other issues with subtrees, such as git blame not working properly or commits being duplicated when the subproject is changed in the parent repository.

Luckily, we found an alternative that is much faster and avoids the other problems.

## Josh comes to the rescue

[Josh][Josh] is a tool written in Rust that implements sophisticated and performant filtering operations on git repositories. It provides a set of reversible git [algebraic filters][josh-filters] that allow manipulating the history of git repos, which enables several cool use-cases. For example, it offers a git proxy that can transparently split a repository into a set of subrepositories on demand, rearrange, exclude or extract the git history of specific directories or linearize the history of a repository that uses merge commits. We mostly simply use it as a very fast "git subtree on steroids". In fact, our Josh workflow is very similar to the way we use git subtrees, with the biggest difference being that the sync operations are an order of magnitude faster and the resulting history is (mostly) cleaner.

To make our subproject handling even easier, we built a small Rust tool on top of Josh, called [josh-sync][josh-sync]. This tool provides a lightweight interface on top of the powerful Josh engine, which allows us to unify the way we handle [pulls](https://github.com/rust-lang/rust-analyzer/pull/22389) and [pushes](https://github.com/rust-lang/rust/pull/156437) across all our subprojects that use Josh. We even prepared a reusable GitHub Actions [workflow][ci-action] that is used in several subproject repositories to periodically perform the `pull` operation, open a PR directly from CI, and [let us know][miri-zulip] on Zulip if the sync cannot be performed without manual intervention.

We currently use Josh to handle subtree synchronization for [Miri][miri], [Rust Analyzer][rust-analyzer], [compiler-builtins][compiler-builtins], [stdarch][stdarch] and the [Rust Compiler Development Guide][rustc-dev-guide]. While there are still a few subprojects that use git subtrees (e.g. Clippy), our plan is to eventually migrate all these remaining subprojects from git subtree to Josh, and improve our tooling to make the bidirectional sync even easier and faster for the maintainers of our developer tools.

We started using Josh a few years ago, and we are very happy that something like it exists; otherwise we would probably have to build it ourselves to deal with the scale of our repositories. Thankfully, Josh's maintainers, especially [Christian Schilling](https://github.com/christian-schilling), are very cooperative, and they are continuously improving Josh to meet our (sometimes quite challenging) needs.

For instance, the main downside of Josh that we ran into is that a "pull" sync creates a huge amount of merge commits in the subproject. In response to that, the Josh developers improved the logic for avoiding trivial merges, and they are currently helping us with the non-trivial migration to these better filters.
Conversely, over the years, our use-cases helped uncover several edge-cases in Josh, and they often serve as a stress test for its performance.

So, with that being said, we would like to thank Josh maintainers for enabling us to scale our complex development workflows! If scaling git repos and workflows sounds interesting to you, then check out [Josh][Josh] to see if it might also help your versioning use-cases.

[rust-lang/rust]: https://github.com/rust-lang/rust
[Josh]: https://github.com/josh-project/josh
[josh-sync]: https://github.com/rust-lang/josh-sync
[submodules]: https://github.com/rust-lang/rust/blob/4530eac197dfc6975c23d5d01e85e44bf7f18d69/.gitmodules
[josh-filters]: https://josh-project.github.io/josh/reference/filters.html
[ci-action]: https://github.com/rust-lang/josh-sync/blob/main/.github/workflows/rustc-pull.yml
[miri-zulip]: https://rust-lang.zulipchat.com/#narrow/channel/269128-miri/topic/Miri.20Build.20Failure.20.282026-05.29
[miri]: https://github.com/rust-lang/miri
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[compiler-builtins]: https://github.com/rust-lang/compiler-builtins
[stdarch]: https://github.com/rust-lang/stdarch
[rustc-dev-guide]: https://github.com/rust-lang/rustc-dev-guide
