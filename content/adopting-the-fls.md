+++
layout = "post"
date = 2025-03-26
title = "Adopting the FLS"
author = "TC"
team = "the Spec Team <https://www.rust-lang.org/governance/teams/lang#team-spec>"
+++

# Adopting the FLS

Some years ago, Ferrous Systems assembled a description of Rust called the FLS[^fls].  They've since been faithfully maintaining and updating this document for new versions of Rust, and they've successfully used it to qualify toolchains based on Rust for use in safety-critical industries.  Seeing this success, others have also begun to rely on the FLS for their own qualification efforts when building with Rust.

[^fls]: The FLS stood for the "Ferrocene Language Specification".  The minimal fork of Rust that Ferrous Systems qualifies and ships to their customers is called "Ferrocene", hence the name.  We'll be dropping the expansion and just calling it the FLS within the Project.

The members of the Rust Project are passionate about shipping high quality tools that enable people to build reliable software at scale.  Such software is exactly the kind needed by those in safety-critical industries, and consequently we've become increasingly interested in better understanding and serving the needs of these customers of our language and of our tools.

It's in that light that we're pleased to announce that we'll be adopting the FLS into the Rust Project as part of our ongoing specification efforts.  This adoption is being made possible by the gracious donation of the FLS by Ferrous Systems.  We're grateful to them for the work they've done in assembling the FLS, in making it fit for qualification purposes, in promoting its use and the use of Rust generally in safety-critical industries, and now, for working with us to take the next step and to bring the FLS into the Project.

With this adoption, we look forward to better integrating the FLS with the processes of the Project and to providing ongoing and increased assurances to all those who use Rust in safety-critical industries and, in particular, to those who use the FLS as part of their qualification efforts.

This adoption and donation would not have been possible without the efforts of the Rust Foundation, and in particular of Joel Marcey, the Director of Technology at the Foundation, who has worked tirelessly to facilitate this on our behalf.  We're grateful to him and to the Foundation for this support.  The Foundation has published its own [post] about this adoption.

[post]: https://rustfoundation.org/media/ferrous-systems-donates-ferrocene-language-specification-to-rust-project/

## I'm relying on the FLS today; what should I expect?

We'll be bringing the FLS within the Project, so expect some URLs to change.  We plan to release updates to the FLS in much the same way as they have been happening up until now.

We're sensitive to the fact that big changes to this document can result in costs for those using it for qualification purposes, and we don't have any immediate plans for big changes here.

## What's this mean for the Rust Reference?

The [Reference] is still the Reference.  Adopting the FLS does not change the status of the Reference, and we plan to continue to improve and expand the Reference as we've been doing.

We'll of course be looking for ways that the Reference can support the FLS, and that the FLS can support the Reference, and in the long term, we're hopeful we can find ways to bring these two documents closer together.

[Reference]: https://github.com/rust-lang/reference
