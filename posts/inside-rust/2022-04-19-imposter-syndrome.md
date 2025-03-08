+++
layout = "post"
date = 2022-04-19
title = "Imposter Syndrome"
author = "Jane Lusby, Project Director of Collaboration"
team = "Rust Foundation Project Directors <https://foundation.rust-lang.org/about/>"
+++

*Preface: This is in response to some feedback the project directors received
from the Rust Foundation staff. Some of the contributors they'd talked to said
they didn't feel justified in applying for Foundation grants even though they'd
love the opportunity, because they don't feel qualified or deserving of them
compared to the other amazing contributors they look up to within the Rust
project. This was a little bit heart breaking to me, because I know exactly
what that feeling is like[^1], and I also know just how wrong they are.*

Imposter syndrome is an insidious problem within software communities. Many of
us, especially members of marginalized communities, struggle to shake the
feeling that we aren't as qualified as our peers. This makes us feel
unqualified and undeserving compared to those around us. It can make us
hesitate to join communities in the first place and, for those already
involved, it can create a sense of impending doom where you constantly feel
like you're going to get found out and expelled from the community. Overall
it's just not great for mental health, 0/10, would not recommend.

The thing is though, imposter syndrome is a logical fallacy[^2]. Imposter
syndrome occurs when we discount what we know and inflate what we think other
people know, and this effect is often then reinforced by systemic bias for
those of us who don't get the assumption of competence.

![picture of imposter syndrome, left side shows a large circle saying "What I think others know" and a small circle inside of it saying "What I know", right side shows the same small circle saying "What I know" surrounded by many other equally sized small circles labeled "What others know"](../../../../images/2022-04-19-imposter-syndrome/imposter_syndrome.jpg)

In reality, we're all specialists within the Rust project. We all have areas
where we have deep expertise and other large areas where we only have the
vaguest idea of how things work. [Niko](https://github.com/nikomatsakis), one
of the lang team co-leads, former compiler team lead and core team alumni,
still comes to me to ask questions about error handling. I frequently need to
tell my fellow contributors that I have no idea what the acronyms they're using
mean[^3]. But this doesn't mean we don't deserve our positions within the
project. We don't expect every contributor to know everything, to be perfect, or
to make no mistakes. The only thing we expect from our contributors is the
ability to collaborate effectively with others and a willingness to learn and
grow over time.

The thing that makes the Rust project as good as it is isn't a couple of
prolific contributors lifting mountains by themselves, it's everyone working
together that brought us to where we are today. We all make mistakes. The
project has layer[^4] after layer[^5] of safeguards to make sure we have a
chance to catch and fix them before they affect our users. These incidents are
unavoidable, expected, and honestly fine! This is the most fundamental
philosophy of both the Rust language and the Rust project: we don't think it's
sufficient to build robust systems by only including people who don't make
mistakes; we think it's better to provide tooling and process to catch and
prevent mistakes. It isn't an accident that our motto is "A language empowering
everyone to build reliable and efficient software." We want people to feel
empowered to make changes they're not 100% confident in, to make mistakes, to
learn, and to grow within the Rust project. This is how all of us got to where
we are today!

So, if you look up to people within the Rust project, if the work we do here
interests you, if you have always wanted to contribute, and _especially_ if you
already have contributed, I want you to know that you're one of the people we
want to apply for [Rust Foundation grants and
fellowships](https://foundation.rust-lang.org/grants/). You're one of the
people we want to eventually see join teams. If you're already on a team, I
want you to know that you're there for a good reason, and we value your
judgement. You're not an imposter, and I want you to know that I really look
forward to seeing you around the project.

Edit: After I posted this it was brought to my attention that the image I used
and tweet I cited are not from an original source, and they can actually be
traced back to a series of blog posts by Alicia Liu. These original sources do
a much more subtle exploration of what is and isn't imposter syndrome, and
particularly focus on how imposter syndrome impacts members of marginalized
communities, I highly recommend reading these posts.

- [Overcoming Imposter Syndrome](https://medium.com/counter-intuition/overcoming-impostor-syndrome-bdae04e46ec5)
- [Impostor Syndrome Is Not Just a Confidence Problem](https://medium.com/counter-intuition/impostor-syndrome-is-not-just-a-confidence-problem-dea670e59f6e)
- [You don't have Imposter Syndrome](https://medium.com/counter-intuition/you-don-t-have-impostor-syndrome-126e4c4bdcc)

---

*To help reinforce and normalize this, I've gathered a list of times when
current or past project members have struggled with imposter syndrome, have
made mistakes, have had to ask "basic" questions, and similar experiences that
will hopefully help set more reasonable expectations for new and old
contributors across the project.*

* [Jane Lusby](https://github.com/yaahc/): "I frequently struggle with imposter
  syndrome and feeling like I don't get as much done as my peers. When I do all
  of my work based off of notifications I completely lose track of what I've
  done and end up starving the tasks I wanted to work on. I'm learning to set
  reasonable expectations for myself, getting better at managing distractions,
  and being intentional about when I respond to github/zulip notifications
  which helps me with keeping track of what I've done and making steady
  progress on my priorities."
* [Josh Triplett](https://github.com/joshtriplett/): "I didn't fully understand
  `Pin` until I read fasterthanlime's ["Pin and
  suffering"](https://fasterthanli.me/articles/pin-and-suffering) blog post and
  I gave a talk in 2016 where my [most important
  point](https://www.youtube.com/watch?v=U8Gl3RTXf88#t=24m40s) was that people
  erroneously believe that you have to be an expert to write an RFC or change
  Rust, but that I wasn't, and you don't need to be one either."
* [Ralf Jung](https://github.com/ralfjung): "I am still surprised anyone is
  taking Miri and Stacked Borrows seriously."
* [Forest Anderson](https://github.com/angelonfira/): "As someone who just
  learned last week what `dyn` does, it still amazes me that I have something to
  give as a team lead. I was immersed in Rust communities by writing weekly
  blogs about Veloren (I took this on because I didn't know enough to contribute
  code), which lead to helping with the Rust Gamedev newsletter, which led me to
  helping to run the Cross Team Collaboration Fun Times meetup!"
* [Felix S Klock II](https://github.com/pnkfelix): "Back in 2015, while I was
  presenting a tutorial on Rust, and explaining `&T`, I had someone from the
  audience, a Rust expert, say "ah ah ah! but what about interior mutability";
  and in my mind I thought "... oh no; what is that?", followed by "... what am
  I doing, I'm not qualified to be up here...". All of us "imposters" must
  strive to prevent such moments from becoming barriers to our own
  participation. I've learned a lot about Rust (and group dynamics, and
  organizational behavior) since then, but I'm still learning every day;
  re-learning, in some cases."

[^1]: Quote from <https://yaah.dev/getting-involved>: "What happened at the
  Google meetup you ask? Manish, our wonderful meetup organizer, walked up to
  me, unprompted, and asked “Hey, you’re Jane right?”. I was shocked, how the
  heck did Manish know who I was? It didn’t feel as though I’d done anything
  worthy of notice, and yet here he was asking for me by name."
[^2]: <https://twitter.com/ithinkwellHugh/status/1175900121097220096>
[^3]: <https://github.com/rust-lang/project-error-handling/issues/34#issuecomment-1092269566>
[^4]: Any irreversible changes such as stabilizations require almost everyone
  on the relevant team to approve the change and zero people on the team to
  raise concerns.
[^5]: We double check all changes with
  [crater](https://github.com/rust-lang/crater) before they ever land on stable
  and are careful to [quickly](https://github.com/rust-lang/rust/issues/88967)
  [revert](https://github.com/rust-lang/rust/issues/90904)
  [changes](https://github.com/rust-lang/rust/issues/82913) that cause problems
  on crater or nightly.
