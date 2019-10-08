---
layout: post
title: "Increasing Rust’s Reach"
author: Carol Nichols
---

**EDIT: We've heard that Google Forms is not easily accessible in all countries; if that applies to you, please find the [application's questions in this text file](/images/2017-06-Increasing-Rusts-Reach/application.txt) and send the answers via email to carol.nichols@gmail.com.**

One of [Rust's 2017 goals](https://blog.rust-lang.org/2017/02/06/roadmap.html) is to make it easier
to become productive in Rust by reducing its learning curve. We believe Rust has potential as an
enabling technology, empowering a people who would not traditionally do system programming to take
it on with confidence. But there’s a bit of a bootstrapping problem here: if we want to reach new
people, we can’t do it by relying solely on the skills and perspectives of our existing community.
So we’ve decided to do an experiment, and we need your help!

We’re looking for people inside and outside Rust’s current community from groups and backgrounds
that are underrepresented in the Rust world and the technology world more generally. We want to
partner with you to make Rust a more inclusive, approachable and impactful project, while
supporting your success on personal goals.

We have a team of Rust community leaders to pair you with. This group isn’t particularly diverse;
this is where the Rust community is right now. We acknowledge that we have lots of work to do, and
this initiative is part of that work. We’re all committed to improving the diversity of the Rust
community.

The Rust team leaders have proposed projects in a variety of areas, such as improving Rust itself,
working on Rust web tools, and improving usability. We’re looking for a variety of experiences and
skill sets! We’re also open to project ideas of your own.

We’re asking for a time investment of 3-5 hours per week between Aug 7 and Nov 6 (or a shorter time
period within those dates). The exact scope of each of the projects is flexible, depending on your
availability and goals. We expect you to check in at least weekly with a summary of your progress,
and we expect you to produce a deliverable by the end of your time working on the project. The
deliverable will vary based on the project; it might be pull requests, recommendations,
documentation, or tutorials, for example.

As thanks for your participation, Mozilla will cover flight, hotel, and ticket costs to the Rust
conference of your choice out of:

- RustConf, Aug 18-19 in Portland, OR, USA
- RustFest, Sept 30-Oct 1 in Zurich, Switzerland
- Rust Belt Rust, Oct 26-27 in Columbus, OH, USA

Your pair will give you the context and tools needed to help you make an impact on an important
area of Rust. You’ll also have access to a private Slack to chat with the other participants and
Rust team members involved in this initiative. We’re planning on highlighting the outcomes of this
experiment and recognizing your contributions explicitly; we value these projects and your
contributions to them highly!

Some groups that are underrepresented in technology and in the Rust community that we would
especially love insights from include women (cis & trans), nonbinary folks, people of color,
non-native English speakers, people who learned programming later in life (older, or only in
college, or at a bootcamp as part of a midlife career change), people with disabilities, or people
who have different learning styles.

**[Apply via this form by July 18!](https://docs.google.com/forms/d/e/1FAIpQLSfbSGuoyZE9dctdEoC_XEZ7j2ox7jQT1zghAOF4iGds2PfBCA/viewform)** (when the day is over in all time zones). We will notify all applicants by Aug 1.

Please contact Carol Nichols via email at carol.nichols@gmail.com or on Twitter at
[@carols10cents](https://twitter.com/Carols10cents) with any questions.

----------

## Projects

### 1. User Experience of Rust Documentation & Code Browsing Tools

Rust partners: Nick Cameron and Steve Klabnik

We’re looking for someone with background in graphic design, web frontend engineering, or user
experience to work on the frontend/UI/UX/design for
[documentation](https://doc.rust-lang.org/stable/std/index.html) and [code browsing
tools](https://github.com/nrc/rustw). These tools have web frontends (HTML, CSS, Ember or React)
and Rust backends and the tools must deeply understand Rust source code. This project will be a
great way to apply experience from the design and frontend worlds, while giving you the opportunity
to learn Rust in detail. We’re looking for innovative ideas to make it easier to use Rust and
libraries written in Rust by improving the experience of using documentation and browsing through
code.

This project might choose multiple applicants that would work together as a team.

Who should apply for this project:

- Background in graphic design, web design, web frontend development, or user experience
- Interest in improving usability of developer-facing documentation and tools
- Interest in learning in detail about Rust syntax and semantics

<img src="/images/2017-06-Increasing-Rusts-Reach/nrc.jpg" alt="Nick Cameron" class="right-thumbnail">

*About Nick Cameron: I'm a Rust core team member and lead the dev-tools team. I work on a bunch of
tools including Rustdoc, Rustfmt, the Rust Language Server, and the Rust compiler. I want to make
Rust developer tools awesome, and user experience is a key part of that. I want Rust to change the
world (or at least the programming world), and we can't do that without making the Rust language
and community more accessible to more people.*

<img src="/images/2017-06-Increasing-Rusts-Reach/steveklabnik.jpeg" alt="Steve Klabnik" class="right-thumbnail">

*About Steve Klabnik: I used to work on Ruby on Rails before Rust, and one of the reasons why I love
Rust is that as a project, we're committed to helping people learn Rust as their first low-level
language. So in some sense, I see this as a continuation of that.*

*However, by the same token, many low-level programmers are not web developers. This means that much
of Rust's web stuff is... not exactly great. Since I come from the web world, I'm trying to help;
but I'm only one person! And not only that, I'm more of a backend person, and so am personally
lacking experience with front-end stuff.*

*So, I see this project as a classic skills exchange: you bring your expertise in web technologies,
we bring our expertise in Rust, everyone learns, and our code gets better!*

### 2. Adding Code Lints to the Clippy Developer Tool

Rust partner: Manish Goregaokar

Clippy is the linter for Rust. It’s a static analysis tool that finds issues in your code and
reports them to the developer, often with suggested fixes. This project will involve adding more
such lints and improving the existing ones to be more helpful and user-friendly. We would love your
ideas and help in the following ways:

1. Improving & fixing bugs in existing Lints.
2. Adding lints focused on helping people make the transition to Rust from a particular language
3. Lints that serve as a tutorial that introduce ideas and teach code improvements
4. Your idea!

Who should apply for this project:

- Some experience using a linter in another language (examples: rubocop in Ruby, jslint in
  JavaScript, pylint in Python, Coverity, etc)
- Interest in improving the new Rust developer experience
- Interest in learning how to improve and create lints

<img src="/images/2017-06-Increasing-Rusts-Reach/manish.png" alt="Manish Goregaokar" class="right-thumbnail">

*About Manish Goregaokar: I'm a research engineer at Mozilla working on Servo. I care a lot about
improving workflows and tooling, especially around the newcomer experience.*

*I consider open source to be one of the best ways to gain proficiency in programming, and to that
end I care about making it more accessible and easy to contribute to, both by systemic improvements
and individual mentorship.*

### 3. Improve the Approachability of the Design of rust-lang.org and/or crates.io

Rust Partner: Aaron Turon

I'd love to pair up with one or more people on the design of [our main
website](https://www.rust-lang.org/en-US/) and/or [crates.io](https://crates.io/). Both of them
could *really* be improved in a way that could make a big impact on Rust's approachability, and
I've got a lot of ideas (and complaints) to start with. There's also potential to bootstrap a whole
new design subteam from the project, if successful.

This project might choose multiple applicants that would work together as a team.

Who should apply for this project:

- Background in graphic design, web design, information architecture, or user experience
- Interest in improving approachability of Rust
- Interest in investigating the motivations behind visitors to the main website and/or crates.io

<img src="/images/2017-06-Increasing-Rusts-Reach/aturon.jpg" alt="Aaron Turon" class="right-thumbnail">

*About Aaron Turon: I'm a Rust core team member, manage the Mozilla Rust team, and currently lead
the library, infrastructure, and Cargo teams. On the technical side, I'm most driven by language
design and end-to-end user experience. On the people side, I love fostering consensus, building
teams, and empowering people. I live in Portland, Oregon with my partner and two daughters.*

*Working in an open source community has shown me, over and over, how vital a diversity of
perspectives and backgrounds can be. My hope is that, by investing in initiatives like this one, we
can welcome a broader range of people and empower them within the Rust community.*

### 4. Improving the Video Tutorials at intorust.com

Rust partner: Niko Matsakis

[intorust.com](http://intorust.com/) is a website with a collection of screencasts that aim to
teach key Rust concepts in an easy and accessible way. I would love to work with someone both on
expanding the material to cover more parts of Rust as well as making sure that it is understandable
to as broad an audience as possible. Another interesting avenue might be expanding the site to also
cover background topics like the role of the stack and the heap.

Who should apply for this project:

- Background in teaching or tutoring
- Background or interest in creating visual teaching tools like diagrams or drawings a plus
- Interest in learning Rust concepts and teaching what you learn to others

<img src="/images/2017-06-Increasing-Rusts-Reach/nmatsakis.jpeg" alt="Niko Matsakis" class="right-thumbnail">

*About Niko Matsakis: I'm a member of the Rust core team as well as the Rust language and compiler
teams. I focus mainly on the design and implementation of the language itself. I want to do what I
can to make learning Rust as smooth as possible for as many people as possible. I think that the
best way to achieve this goal is to work with people with different backgrounds and experiences,
since that will affect how they learn the material.*

### 5. Write tutorials for Rocket and Diesel, Parts of Rust’s Web Framework Story

Rust Partner: Sean Griffin

I’m looking for someone with experience in web development in some other web framework to help
improve the documentation around Rust's frameworks in the web development space. I’d love to
improve the new user experience for using [Rocket](https://rocket.rs/) (a web framework) and
[Diesel](http://diesel.rs/) (an ORM) together. This could include writing tutorials, creating
screencasts, making example applications, or improving the API documentation.

Who should apply for this project:

- Background in teaching or tutoring
- Experience using a web framework written in some other language
- Interest in learning how to write web applications in Rust and teaching what you learn to others

<img src="/images/2017-06-Increasing-Rusts-Reach/sgrif.jpeg" alt="Sean Griffin" class="right-thumbnail">

*About Sean Griffin: In addition to my Rust work, I am one of the maintainers of Ruby on Rails. One
of the great things about Rails (and web development) is that it has really helped to lower the
barrier to entry. I think Rust can have a similar impact (both for low level programming, and
competing as a high level language). However, at the moment Rust has a notoriously bad learning
curve. Improving that with input from as many viewpoints as possible seems like the best way to
help even the playing field once again.*

### 6. Extending the Literate Programming Tool Tango

Rust partner: Felix Klock

I'm looking for someone interested in ["programs as
literature"](https://en.wikipedia.org/wiki/Literate_programming) to help me extend
[`tango`](https://github.com/pnkfelix/tango/) (a literate programming tool for Rust) so that it can
be used for more than just demos and slideshows. If you have experience writing meta-commentary
about your own code, documenting how it works for the purpose of teaching others about it, then you
might be the person I am seeking! Check out [this markdown
file](https://github.com/pnkfelix/mon-artist/blob/a3388c11e8b1910cc4eb4c31bd1540a46851618b/src/lit/s
rc/format.md) that tango is able to turn into an executable Rust code file for an example of what
tango can do. You can see some ideas for extensions from the
[`tango`](https://github.com/pnkfelix/tango/issues)
[issues](https://github.com/pnkfelix/tango/issues) page.

Who should apply for this project:

- Background in writing, teaching, or documentation
- Interest in, and opinions about, improving tools for creating and displaying code documentation

<img src="/images/2017-06-Increasing-Rusts-Reach/pnkfelix.jpeg" alt="Felix Klock" class="right-thumbnail">

*About Felix Klock: I'm the main developer of tango (as well as a member of the Rust compiler and
language teams). I actively use tango for authoring my presentation slides about Rust. I
hypothesize that literate Rust code can act as advertisement for Rust itself, and would like to
figure out how we could make the tool more useful for real-world crates.*

*More generally: I am a long adherent to the idea that computers can be a powerful [educational
tool](http://www.ccs.neu.edu/home/matthias/HtDP2e/part_preface.html). Rust provides a unique vessel
for the intrepid explorers plunging into and past the levels of abstraction that lie atop the
machine. I want Rust to maximize its [accessibility to
all](https://www.mozilla.org/en-US/mission/), so that it does not fall into an echo chamber of
thought and end up as a technology only usable by elite wizards, thus missing this educational
opportunity.*

### 7. Finding Missing Pieces in the Crates Ecosystem

Rust partner: Andrew Gallant

I’m interested in working with someone to discover where there might be gaps in the kinds of crates
that are available. This project would involve first writing a small application in the language or
framework that you’re most comfortable with. Then we’d attempt to port that application to Rust and
keep track of where there are libraries missing or functionality missing from the libraries that
are available. The Rust community can then take your findings to fill in those gaps and make Rust
usable in more scenarios.

Who should apply for this project:

- Experience using libraries to build applications in some other programming language
- Interest in learning how to translate an application to Rust
- Interest in researching and documenting features that libraries have or don’t have

<img src="/images/2017-06-Increasing-Rusts-Reach/burntsushi.jpeg" alt="Andrew Gallant" class="right-thumbnail">

*About Andrew Gallant: I'm a member of the Rust library team that works with Rust in my free time. I
am very interested in information retrieval, fast text search and generally improving Rust's
ecosystem. I'm driven both by own personal interest in technical topics and the desire to teach.
Aside from my own technical interest in languages like Rust, I really love working on the project
because it provides an outlet to teach others about the things I've learned. I'm particularly
interested in improving the learning outcomes for as many people as possible, and would relish the
opportunity to extend that to folks that I might not have been able to reach otherwise. I live in
central Massachusetts with my wife.*

### 8. Finding Missing Pieces in the Experience of building a Command Line Interface (CLI) Program

Rust partner: Kamal Marhubi

I find Rust to be a great language for writing small command line tools, but I think it could be
even better. I'd love to work with someone who wants to write a CLI program, or has one they want
to port to Rust. There are almost certainly rough edges, and working together would be a great way
to find them and improve the tooling for everyone.

Who should apply for this project:

- Experience building and using tools with a command line interface in some other language
- Interest in learning how to write or translate a CLI program to Rust
- Interest in researching and documenting features that libraries have or don’t have

<img src="/images/2017-06-Increasing-Rusts-Reach/kamalmarhubi.jpeg" alt="Kamal Marhubi" class="right-thumbnail">

*About Kamal Marhubi: I've been writing Rust for about a year and a half. I help maintain nix, a
library that gives a Rusty interface to unix systems APIs. I've also contributed to rustfmt, rustup,
and the standard library.*

----

We're excited to get [your application](https://docs.google.com/forms/d/e/1FAIpQLSfbSGuoyZE9dctdEoC_XEZ7j2ox7jQT1zghAOF4iGds2PfBCA/viewform) before July 18! We will notify all applicants by Aug 1.
