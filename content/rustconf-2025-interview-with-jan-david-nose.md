+++
path = "2025/11/25/interview-with-jan-david-nose"
title = "Interview with Jan David Nose"
authors = ["Pete LeVasseur"]

[extra]
team = "the Content Team"
team_url = "https://rust-lang.org/governance/teams/launching-pad/#team-content"
+++

On the [Content Team], we had our first whirlwind outing at RustConf 2025 in Seattle, Washington, USA. There we had a chance to speak with folks about interesting things happening in the Project and the wider community.

# Jan David Nose, Infrastructure Team

In this interview, [Xander Cesari] sits down with [Jan David Nose], then one of the full-time engineers on the [Infrastructure Team], which maintains and develops the infrastructure upon which Rust is developed and deployed -- including CI/CD tooling and crates.io.

We released this video on an accelerated timeline, some weeks ago, in light of the recent software supply chain attacks, but the interview was conducted prior to the news of compromised packages in other languages and ecosystems.

Check out the [interview here] or click below.

<iframe width="560" height="315" src="https://www.youtube.com/embed/r7i-2wHtNjw?si=Nc2N2Gqqpry1FKfs" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

----

## Transcript

**Xander Cesari**: Hey, this is Xander Cesari with the Rust Project
Content Team, recording on the last hour of the last day of RustConf
2025 here in Seattle. So it's been a long and amazing two days. And I'm
sitting down here with a team member from the Rust Project Infra Team,
the unsung heroes of the Rust language. Want to introduce yourself and
kind of how you got involved?

**Jan David Nose**: Yeah, sure. I'm JD. Jan David is the full name, but
especially in international contexts, I just go with JD. I've been
working for the Rust Foundation for the past three years as a full-time
employee and I essentially hit the jackpot to work full-time on open
source and I've been in the Infra Team of the Rust Project for the
whole time. For the past two years I've led the team together with
Jake. So the Infra Team is kind of a thing that lets Rust happen and
there's a lot of different pieces.

**Xander Cesari**: Could you give me an overview of the responsibility
of the Infra Team?

**Jan David Nose**: Sure. I think on a high level, we think about this
in terms of, we serve two different groups of people. On one side, we
have users of the language, and on the other side, we really try to
provide good tooling for the maintainers of the language.

**Jan David Nose**: Starting with the maintainer side, this is really
everything about how Rust is built. From the moment someone makes a
contribution or opens a PR, we maintain the continuous integration that
makes sure that the PR actually works. There's a lot of bots and
tooling helping out behind the scenes to kind of maintain a good status
quo, a sane state. Lots of small things like triage tools on GitHub to
set labels and ping people and these kinds of things. And that's kind
of managed by the Infra Team at large.

**Jan David Nose**: And then on the user side, we have a lot of, or the
two most important things are making sure users can actually download
Rust. We don't develop crates.io, but we support the infrastructure to
actually ship crates to users. All the downloads go through content
delivery networks that we provide. The same for Rust releases. So if I
don't do my job well, which has happened, there might be a global
outage of crates.io and no one can download stuff. But those are kind
of the two different buckets of services that we run and operate.

**Xander Cesari**: Gotcha. So on the maintainer side, the Rust
organization on GitHub is a large organization with a lot of activity,
a lot of code. There's obviously a lot of large code bases being
developed on GitHub, but there are not that many languages the size of
Rust being developed on GitHub. Are there unique challenges to
developing a language and the tooling that's required versus developing
other software projects?

**Jan David Nose**: I can think of a few things that have less to do
with the language specifically, but with some of the architecture
decisions that were made very early on in the life cycle of Rust. So
one of the things that actually caused a lot of headache for mostly
GitHub, and then when they complained to us, for us as well, is that
for a long, long time, the index for crates.io was a Git repo on
GitHub. As Rust started to grow, the activity on the repo became so big
that it actually caused some issues, I would say, in a friendly way on
GitHub, just in terms of how much resources that single repository was
consuming. That then kind of started this work on a web-based,
HTTP-based index to shift that away. That's certainly one area where
we've seen how Rust has struggled a little bit with the platform, but
also the platform provider struggled with us.

**Jan David Nose**: I think for Rust itself, especially when we look at
CI, we really want to make sure that Rust works well on all of the
targets and all the platforms we support. That means we have an
extremely wide CI pipeline where, for every Tier 1 target, we want to
run all the tests, we want to build the release artifacts, we want to
upload all of that to S3. We want to do as much as we reasonably can
for Tier 2 targets and, to a lesser extent, maybe even test some stuff
on Tier 3. That has turned into a gigantic build pipeline. Marco gave a
talk today on what we've done with CI over the last year. One of the
numbers that came out of doing the research for this talk is that we
accumulate over three million build minutes per month, which is about
six years of CPU time every month.

**Jan David Nose**: Especially when it comes to open source projects, I
think we're one of the biggest consumers of GitHub Actions in that
sense. Not the biggest in total; there are definitely bigger commercial
projects. But that's a unique challenge for us to manage because we
want to provide as good a service as we can to the community and make
sure that what we ship is high quality. That comes at a huge cost in
terms of scaling. As Rust gets more popular and we want to target more
and more platforms, this is like a problem that just continues to
grow.

**Jan David Nose**: We'll probably never remove a lot of targets, so
there's an interesting challenge to think about. If it's already big
now, how does this look in 5 years, 10 years, 15 years, and how can we
make sure we can maintain the level of quality we want to ship? When
you build and run for a target in the CI pipeline, some of those Tier 1
targets you can just ask a cloud service provider to give you a VM
running on that piece of hardware, but some of them are probably not
things that you can just run in the cloud.

**Xander Cesari**: Is there some HIL (Hardware-In-the-Loop) lab
somewhere?

**Jan David Nose**: So you're touching on a conversation that's
happening pretty much as we speak. So far, as part of our target tier
policy, there is a clause that says it needs to be able to run in CI.
That has meant being very selective about only promoting things to Tier
1 that we can actually run and test. For all of this, we had a
prerequisite that it runs on GitHub Actions. So far we've used very
little hardware that is not natively supported or provided by GitHub.

**Jan David Nose**: But this is exactly the point with Rust increasing
in popularity. We just got requests to support IBM platforms and
RISC-V, and those are not natively supported on GitHub. That has kicked
off an internal conversation about how we even support this. How can we
as a project enable companies that can provide us hardware to test on?
What are the implications of that?

**Jan David Nose**: On one side, there are interesting constraints and
considerations. For example, you don't want your PRs to randomly fail
because someone else's hardware is not available. We're already so
resource-constrained on how many PRs we can merge each day that adding
noise to that process would really slow down contributions to Rust. On
the other side, there are security implications. Especially if we talk
about promoting something to Tier 1 and we want to build release
artifacts on that hardware, we need to make sure that those are
actually secure and no one sneaks a back door into the Rust compiler
target for RISC-V.

**Jan David Nose**: So there are interesting challenges for us,
especially in the world we live in where supply chain security is a
massive concern. We need to figure out how we can both support the
growth of Rust and the growth of the language, the community, and the
ecosystem at large while also making sure that the things we ship are
reliable, secure, and performant. That is becoming an increasingly
relevant and interesting piece to work on. So far we've gotten away
with the platforms that GitHub supports, but it's really cool to see
that this is starting to change and people approach us and are willing
to provide hardware, provide sponsorship, and help us test on their
platforms. But essentially we don't have a good answer for this yet.
We're still trying to figure out what this means, what we need to take
into consideration, and what our requirements are to use external
hardware.

**Xander Cesari**: Yeah, everyone is so excited about Rust will run
everywhere, but there's a maintenance cost there that is almost
exponential in scope.

**Jan David Nose**: It's really interesting as well because there's a
tension there. I think with IBM, for example, approaching us, it's an
interesting example. Who has IBM platforms at home? The number of users
for that platform is really small globally, but IBM also invests
heavily in Rust, tries to make this happen, and is willing to provide
the hardware.

**Jan David Nose**: For us, that leads to a set of questions. Is there
a line? Is there a certain requirement? Is there a certain amount of
usage that a platform would need for us to promote it? Or do we say we
want to promote as much as we can to Tier 1? This is a conversation we
haven't really had to have yet. It's only now starting to creep in as
Rust is adopted more widely and companies pour serious money and
resources into it. That's exciting to see.

**Jan David Nose**: In this specific case, companies approach the Infra
Team to figure out how we can add their platforms to CI as a first step
towards Tier 1 support. But it's also a broader discussion we need to
have with larger parts of the Rust Project. For Tier 1 promotions, for
example, the Compiler Team needs to sign off, Infra needs to sign off.
Many more people need to be involved in this discussion of how we can
support the growing needs of the ecosystem at large.

**Xander Cesari**: I get the feeling that's going to be a theme
throughout this interview.

**Jan David Nose**: 100%.

**Xander Cesari**: So one other tool that's part of this pipeline that
I totally didn't know about for a long time, and I think a talk at a
different conference clued me into it, is Crater. It's a tool that
attempts to run all of the Rust code it can find on the internet. Can
you talk about what that tool does and how it integrates into the
release process?

**Jan David Nose**: Whenever someone creates a pull request on GitHub
to add a new feature or bug fix to the Rust compiler, they can start
what's called a Crater run, or an experiment. Crater is effectively a
large fleet of machines that tries to pull in as many crates as it can.
Ideally, we would love to test all crates, but for a variety of reasons
that's not possible. Some crates simply don't build reliably, so we
maintain lists to exclude those. From the top of my head, I think we
currently test against roughly 60% of crates.

**Jan David Nose**: The experiment takes the code from your pull
request, builds the Rust compiler with it, and then uses that compiler
to build all of these crates. It reports back whether there are any
regressions related to the change you proposed. That is a very
important tool for us to maintain backwards compatibility with new
versions and new features in Rust. It lets us ask: does the ecosystem
still compile if we add this feature to the compiler, and where do we
run into issues? Then, and this is more on the Compiler Team side,
there's a decision about how to proceed. Is the breakage acceptable? Do
we need to adjust the feature? Having Crater is what makes that
conversation possible because it gives us real data on the impact on
the wider ecosystem.

**Xander Cesari**: I think that's so interesting because as more and
more companies adopt Rust, they're asking whether the language is going
to be stable and backward compatible. You hear about other programming
languages that had a big version change that caused a lot of drama and
code changes. The fact that if you have code on crates.io, the Compiler
Team is probably already testing against it for backwards compatibility
is pretty reassuring.

**Jan David Nose**: Yeah, the chances are high, I would say. Especially
looking at the whole Python 2 to Python 3 migration, I think as an
industry we've learned a lot from those big version jumps. I can't
really speak for the Compiler Team because I'm not a member and I
wasn't involved in the decision-making, but I feel this is one of the
reasons why backwards compatibility is such a big deal in Rust's
design. We want to make it as painless as possible to stay current,
stay up to date, and make sure we don't accidentally break the language
or create painful migration points where the entire ecosystem has to
move at once.

**Xander Cesari**: Do you know if there are other organizations pulling
in something like Crater and running it on their own internal crate
repositories, maybe some of the big tech companies or other compiler
developers or even other languages? Or is this really bespoke for the
Rust compiler team?

**Jan David Nose**: I don't know of anyone who runs Crater itself as a
tool. Crater is built on a sandboxing framework that we also use in
other places. For example, docs.rs uses some of the same underlying
infrastructure to build all of the documentation. We try to share as
much as we can of the functionality that exists in Crater, but I'm not
aware of anyone using Crater in the same way we do.

**Xander Cesari**: Gotcha. The other big part of your job is that the
Infra Team works on supporting maintainers, but it also supports users
and consumers of Rust who are pulling from crates.io. It sounds like
crates.io is not directly within your team, but you support a lot of
the backend there.

**Jan David Nose**: Yeah, exactly. crates.io has its own team, and that
team maintains the web application and the APIs. The crates themselves,
all the individual files that people download, are hosted within our
infrastructure. The Infra Team maintains the content delivery network
that sits in front of that. Every download of a crate goes through
infrastructure that we maintain. We collaborate very closely with the
crates.io team on this shared interface. They own the app and the API,
and we make sure that the files get delivered to the end user.

**Xander Cesari**: So it sounds like there's a lot of verification of
the files that get uploaded and checks every time someone pushes a new
version to crates.io. That part all happens within crates.io as an
application.

**Jan David Nose**: Cargo uses the crates.io API to upload the crate
file. crates.io has a lot of internal logic to verify that it is valid
and that everything looks correct. For us, as the Infra Team, we treat
that as a black box. crates.io does its work, and if it is happy with
the upload, it stores the file in S3. From that point onward,
infrastructure makes sure that the file is accessible and can be
downloaded so people can start using your crate.

**Xander Cesari**: In this theme of Rust being a bit of a victim of its
own success, I assume all of the traffic graphs and download graphs are
very much up and to the right.

**Jan David Nose**: On the Foundation side, one of our colleagues likes
to check how long it takes for one billion downloads to happen on
crates.io, and that number has been falling quickly. I don't remember
what it was three years ago, but it has come down by orders of
magnitude. In our download traffic we definitely see exponential
growth. Our traffic tends to double year over year, and that trend has
been pretty stable. It really seems like Rust is getting a lot of
adoption in the ecosystem and people are using it for more and more
things.

**Xander Cesari**: How has the Infra Team scaled with that? Are you
staying ahead of it, or are there a lot of late nights?

**Jan David Nose**: There have definitely been late nights. In the
three years I've been working in the Infra Team, every year has had a
different theme that was essentially a fire to put out.

**Jan David Nose**: It changes because we fix one thing and then the
next thing breaks. So far, luckily, those fires have been mostly
sequential, not parallel. When I joined, bandwidth was the big topic.
Over the last year, it has been more about CI. About three years ago,
we hit this inflection point where traffic was doubling and the
sponsorship capacity we had at the time was reaching its limits.

**Jan David Nose**: Two or three years ago, Fastly welcomed us into
their Fast Forward program and has been sponsoring all of our bandwidth
since then. That has mostly helped me sleep at night. It has been a
very good relationship. They have been an amazing partner and have
helped us at every step to remove the fear that we might hit limits.
They are very active in the open source community at large; most
famously they also sponsor PyPI and the Python ecosystem, compared to
which we're a tiny fish in a very big pond. That gives us a lot of
confidence that we can sustain this growth and keep providing crates
and releases at the level of quality people expect.

**Xander Cesari**: In some ways, Rust did such a good job of making all
of that infrastructure feel invisible. You just type Cargo commands
into your terminal and it feels magical.

**Jan David Nose**: I'm really happy about that. It's an interesting
aspect of running an infrastructure team in open source. If you look at
the ten-year history since the first stable release, or even the
fifteen years since Rust really started, infrastructure was
volunteer-run for most of that time. I've been here for three years,
and I was the first full-time infrastructure engineer. So for ten to
twelve years, volunteers ran the infrastructure.

**Jan David Nose**: For them, it was crucial that things just worked,
because you can't page volunteers in the middle of the night because a
server caught fire or downloads stopped working. From the beginning,
our infrastructure has been designed to be as simple and as reliable as
possible. The same is true for our CDNs. I always feel a bit bad
because Fastly is an amazing sponsor. Every time we meet them at
conferences or they announce new features, they ask whether we want to
use them or talk about how we use Fastly in production. And every time
I have to say: we have the simplest configuration possible. We set some
HTTP headers. That's pretty much it.

**Jan David Nose**: It's a very cool platform, but we use the smallest
set of features because we need to maintain all of this with a
very small team that is mostly volunteer-based. Our priority has always
been to keep things simple and reliable and not chase every fancy new
technology, so that the project stays sustainable.

**Xander Cesari**: Volunteer-based organizations seem to have to care
about work-life balance, which is probably terrific, and there are
lessons to be learned there.

**Jan David Nose**: Yeah, it's definitely a very interesting
environment to work in. It has different rules than corporations or
commercial teams. We have to think about how much work we can do in a
given timeframe in a very different way, because it's unpredictable
when volunteers have time, when they're around, and what is happening
in their lives.

**Jan David Nose**: Over the last few years, we've tried to reduce the
number of fires that can break out. And when they do happen, we try to
shield volunteers from them and take that work on as full-time
employees. That started with me three years ago. Last year Marco
joined, which increased the capacity we have, because there is so much
to do on the Infra side that even with me working full-time, we simply
did not have enough people.

**Xander Cesari**: So you're two full-time and everything else is
volunteer.

**Jan David Nose**: Exactly. The team is around eight people. Marco and
I work full-time and are paid by the Rust Foundation to focus
exclusively on infrastructure. Then we have a handful of volunteers who
work on different things.

**Jan David Nose**: Because our field of responsibility is so wide, the
Infra Team works more in silos than other teams might. We have people
who care deeply about very specific parts of the infrastructure.
Otherwise there is simply too much to know for any one person. It has
been a really nice mix, and it's amazing to work with the people on the
team.

**Jan David Nose**: As someone who is privileged enough to work
full-time on this and has the time and resources, we try to bear the
bigger burden and create a space that is fun for volunteers to join. We
want them to work on exciting things where there is less risk of
something catching fire, where it's easier to come in, do a piece of
work, and then step away. If your personal life takes over for two
weeks, that's okay, because someone is there to make sure the servers
and the lights stay on.

**Jan David Nose**: A lot of that work lives more on the maintainer
side: the GitHub apps, the bots that help with triage. It's less risky
if something goes wrong there. On the user side, if you push the wrong
DNS setting, as someone might have done, you can end up in a situation
where for 30 minutes no one can download crates. And in this case,
"no one" literally means no user worldwide. That's not
an experience I want volunteers to have. It's extremely stressful and
was ultimately one of the reasons I joined in the first place—there was
a real feeling of burnout from carrying that responsibility.

**Jan David Nose**: It's easier to carry that as a full-timer. We have
more time and more ways to manage the stress. I'm honestly extremely
amazed by what the Infra Team was able to do as volunteers. It's
unbelievable what they built and how far they pushed Rust to get to
where we are now.

**Xander Cesari**: I think anyone who's managing web traffic in 2025 is
talking about traffic skyrocketing due to bots and scrapers for AI or
other purposes. Has that hit the Rust network as well?

**Jan David Nose**: Yeah, we've definitely seen that. It's handled by a
slightly different team, but on the docs.rs side in particular we've
seen crawlers hit us hard from time to time, and that has caused
noticeable service degradation. We're painfully aware of the increase
in traffic that comes in short but very intense bursts when crawlers go
wild.

**Jan David Nose**: That introduces a new challenge for our
infrastructure. We need to figure out how to react to that traffic and
protect our services from becoming unavailable to real users who want
to use docs.rs to look up something for their work. On the CDN side,
our providers can usually handle the traffic. It is more often the
application side where things hurt.

**Jan David Nose**: On the CDN side we also see people crawling
crates.io, presumably to vacuum up the entire crates ecosystem into an
LLM. Fortunately, over the last two years we've done a lot of work to
make sure crates.io as an application is less affected by these traffic
spikes. Downloads now bypass crates.io entirely and go straight to the
CDN, so the API is not hit by these bursts. In the past, this would
have looked like a DDoS attack, with so many requests from so many
sources that we couldn't handle it.

**Jan David Nose**: We've done a lot of backend work to keep our stack
reliable, but it's definitely something that has changed the game over
the last year. We can clearly see that crawlers are much more active
than before.

**Xander Cesari**: That makes sense. I'm sure Fastly is working on this
as well. Their business has to adapt to be robust to this new internet.

**Jan David Nose**: Exactly. For example, one of the conversations
we're having right now is about docs.rs. It's still hosted on AWS
behind CloudFront, but we're talking about putting it behind Fastly
because through Fastly we get features like bot protection that can
help keep crawlers out.

**Jan David Nose**: This is a good example of how our conversations
have changed in the last six months. At the start of the year I did not
think this would be a topic we would be discussing. We were focused on
other things. For docs.rs we have long-term plans to rebuild the
infrastructure that powers it, and I expected us to spend our energy
there. But with the changes in the industry and everyone trying to
accumulate as much data as possible, our priorities have shifted. The
problems we face and the order in which we tackle them have changed.

**Xander Cesari**: And I assume as one of the few paid members of a
mostly volunteer team, you often end up working on the fires, not the
interesting next feature that might be more fun.

**Jan David Nose**: That is true, although it sounds a bit negative to
say I only get to work on fires. Sometimes it feels like that because,
as with any technology stack, there is a lot of maintenance overhead.
We definitely pay that price on the infrastructure side.

**Jan David Nose**: Marco, for example, spent time this year going
through all the servers we run, cataloging them, and making sure
they're patched and on the latest operating system version. We updated
our Ubuntu machines to the latest LTS. It feels a bit like busy
work—you just have to do it because it's important and necessary, but
it's not the most exciting project.

**Jan David Nose**: On the other hand, when it comes to things like CDN
configuration and figuring out how bot protection features work and
whether they are relevant to us, that is also genuinely interesting
work. It lets us play with new tools vendors provide, and we're working
on challenges that the wider industry is facing. How do you deal with
this new kind of traffic? What are the implications of banning bots?
How high is the risk of blocking real users? Sometimes someone just
misconfigures a curl script, and from the outside it looks like they're
crawling our site.

**Jan David Nose**: So it's an interesting field to work in, figuring
out how we can use new features and address new challenges. That keeps
it exciting even for us full-timers who do more of the
"boring" work. We get to adapt alongside how the world
around us is changing. If there's one constant, it's change.

**Xander Cesari**: Another ripped-from-the-headlines change around this
topic is software supply chain security, and specifically xz-utils and
the conversation around open source security. How much has that changed
the landscape you work in?

**Jan David Nose**: The xz-utils compromise was scary. I don't want to
call it a wake-up call, because we've been aware that supply chain
security is a big issue and this was not the first compromise. But the
way it happened felt very unsettling. You saw an actor spend a year and
a half building social trust in an open source project and then using
that to introduce a backdoor.

**Jan David Nose**: Thinking about that in the context of Rust: every
team in the project talks about how we need more maintainers, how
there's too much workload on the people who are currently contributing,
and how Rust's growth puts strain on the organization as a whole. We
want to be an open and welcoming project, and right now we also need to
bring new people in. If someone shows up and says, "I'm
willing to help, please onboard me," and they stick around for
a year and then do something malicious, we would be susceptible to
that. I don't think this is unique to Rust. This is an inherent problem
in open source.

**Xander Cesari**: Yeah, it's antithetical to the culture.

**Jan David Nose**: Exactly. So we're trying to think through how we,
as a project and as an ecosystem, deal with persistent threat actors
who have the time and resources to play a long game. Paying someone to
work full-time on open source for a year is a very different threat
model than what we used to worry about.

**Jan David Nose**: I used to joke that the biggest threat to crates.io
was me accidentally pulling the plug on a CDN. I think that has
changed. Today the bigger threat is someone managing to insert
malicious code into our releases, our supply chain, or crates.io
itself. They could find ways to interfere with our systems in ways
we're simply not prepared for, where, as a largely volunteer
organization, we might be too slow to react to a new kind of attack.

**Jan David Nose**: Looking back over the last three years, this shift
became very noticeable, especially after the first year. Traffic was
doubling, Rust usage was going up a lot, and there were news stories
about Rust being used in the Windows kernel, in Android, and in parts
of iOS. Suddenly Rust is everywhere. If you want to attack
"everywhere," going after Rust becomes attractive.
That definitely puts a target on our back and has changed the game.

**Jan David Nose**: I'm very glad the Rust Foundation has a dedicated
security engineer who has done a lot of threat modeling and worked with
us on infrastructure security. There's also a lot of work happening
specifically around the crates ecosystem and preventing supply chain
attacks through crates. Luckily, it's not something the Infra side has
to solve alone. But it is getting a lot more attention, and I think it
will be one of the big challenges for the future: how a mostly
volunteer-run project keeps up with this looming threat.

**Xander Cesari**: And it is the industry at large. This is not a
unique problem to the Rust package manager. All package registries,
from Python to JavaScript to Nix, deal with this. Is there an
industry-wide conversation about how to help each other out and share
learnings?

**Jan David Nose**: Yeah, there's definitely a lot happening. I have to
smile a bit because, with a lot of empathy but also a bit of relief, we
sometimes share news when another package ecosystem gets compromised.
It is a reminder that it's not just us, sometimes it's npm this time.

**Jan David Nose**: We really try to stay aware of what's happening in
the industry and in other ecosystems: what new threats or attack
vectors are emerging, what others are struggling with. Sometimes that
is security; sometimes it's usability. A year and a half ago, for
example, npm had the "everything" package where
someone declared every package on npm as a dependency, which blew up
the index. We look at incidents like that and ask whether crates.io
would struggle with something similar and whether we need to make
changes.

**Jan David Nose**: On the security side we also follow closely what
others are doing. In the packaging community, the different package
managers are starting to come together more often to figure out which
problems everyone shares. There is a bit of a joke that we're all just
shipping files over the internet. Whether it's an npm package or a
crate, ultimately it's a bunch of text files in a zip. So from an
infrastructure perspective the problems are very similar.

**Jan David Nose**: These communities are now talking more about what
problems PyPI has, what problems crates.io has, what is happening in
the npm space. One thing every ecosystem has seen—even the very
established ones—is a big increase in bandwidth needs, largely
connected to the emergence of AI. PyPI, for example, publishes download
charts, and it's striking. Python had steady growth—slightly
exponential, but manageable—for many years. Then a year or two ago you
see a massive hockey stick. People discovered that PyPI was a great
distribution system for their models. There were no file size limits at
the time, so you could publish precompiled GPU models there.

**Jan David Nose**: That pattern shows up everywhere. It has kicked off
a new era for packaging ecosystems to come together and ask: in a time
where open source is underfunded and traffic needs keep growing, how
can we act together to find solutions to these shared problems?
crates.io is part of those conversations. It's interesting to see how
we, as an industry, share very similar problems across
ecosystems—Python, npm, Rust, and others.

**Xander Cesari**: With a smaller, more hobbyist-focused community, you
can have relaxed rules about what goes into your package manager.
Everyone knows the spirit of what you're trying to do and you can get
away without a lot of hard rules and consequences. Is the Rust world
going to have to think about much harder rules around package sizes,
allowed files, and how you're allowed to distribute things?

**Jan David Nose**: Funnily enough, we're coming at this from the
opposite direction. Compared to other ecosystems, we've always had
fairly strict limits. A crate can be at most around ten megabytes in
size. There are limits on what kinds of files you can put in there.
Ironically, those limits have helped us keep traffic manageable in this
period.

**Jan David Nose**: At the same time, there is a valid argument that
these limits may not serve all Rust use cases. There are situations
where you might want to include something precompiled in your crate
because it is hard to compile locally, takes a very long time, or
depends on obscure headers no one has. I don't think we've reached the
final state of what the crates.io package format should look like.

**Jan David Nose**: That has interesting security implications. When we
talk about precompiled binaries or payloads, we all have that little
voice in our head every time we see a curl | sh command: can I trust
this? The same is true if you download a crate that contains a
precompiled blob you cannot easily inspect.

**Jan David Nose**: The Rust Foundation is doing a lot of work and
research here. My colleague Adam, who works on the crates.io team, is
working behind the scenes to answer some of these questions. For
example: what kind of security testing can we do before we publish
crates to make sure they are secure and don't contain malicious
payloads? How do we surface this information? How do we tell a
publisher that they included files that are not allowed? And from the
user's perspective, when you visit crates.io, how can you judge how
well maintained and how secure a crate is?

**Jan David Nose**: Those conversations are happening quite broadly in
the ecosystem. On the Infra side we're far down the chain. Ultimately
we integrate with whatever security scanning infrastructure crates.io
builds. We don't have to do the security research ourselves, but we do
have to support it.

**Jan David Nose**: There's still a lot that needs to happen. As
awesome as Rust already is, and as much as I love using it, it's
important to remember that we're still a very young ecosystem. Python
is now very mature and stable, but it's more than 25 years old. Rust is
about ten years old as a stable language. We still have a lot to learn
and figure out.

**Xander Cesari**: Is the Rust ecosystem running into problems earlier
than other languages because we're succeeding at being foundational
software and Rust is used in places that are even more
security-critical than other languages, so you have to hit these hard
problems earlier than the Python world did?

**Jan David Nose**: I think that's true. Other ecosystems probably had
more time to mature and answer these questions. We're operating on a
more condensed timeline. There is also simply more happening now. Open
source has been very successful; it's everywhere. That means there are
more places where security is critical.

**Jan David Nose**: So this comes with the success of open source, with
what is happening in the ecosystem at large, and with the industry
we're in. It does mean we have less time to figure some things out. On
the flip side, we also have less baggage. We have less technical debt
and fifteen fewer years of accumulated history. That lets us be on the
forefront in some areas, like how a package ecosystem can stay secure
and what infrastructure a 21st century open source project needs.

**Jan David Nose**: Here I really want to call out the Rust Foundation.
They actively support this work: hiring people like Marco and me to
work full-time on infrastructure, having Walter and Adam focus heavily
on security, and as an organization taking supply chain considerations
very seriously. The Foundation also works with other ecosystems so we
can learn and grow together and build a better industry.

**Jan David Nose**: Behind the scenes, colleagues constantly work to
open doors for us as a relatively young language, so we can be part of
those conversations and sit at the table with other ecosystems. That
lets us learn from what others have already gone through and also help
shape where things are going. Sustainability is a big part of that: how
do we fund the project long term? How do we make sure we have the human
resources and financial resources to run the infrastructure and support
maintainers? I definitely underestimated how much of my job would be
relationship management and budget planning, making sure credits last
until new ones arrive.

**Xander Cesari**: Most open core business models give away the thing
that doesn't cost much—the software—and charge for the thing that
scales with use—the service. In Rust's case, it's all free, which is
excellent for adoption, but it must require a very creative perspective
on the business side.

**Jan David Nose**: Yeah, and that's where different forces pull in
opposite directions. As an open source project, we want everyone to be
able to use Rust for free. We want great user experience. When we talk
about downloads, there are ways for us to make them much cheaper, but
that might mean hosting everything in a single geographic location.
Then everyone, including people in Australia, would have to download
from, say, Europe, and their experience would get much worse.

**Jan David Nose**: Instead, we want to use services that are more
expensive but provide a better experience for Rust users. There's a
real tension there. On one side we want to do the best we can; on the
other side we need to be realistic that this costs money.

**Xander Cesari**: I had been thinking of infrastructure as a binary:
it either works or it doesn't. But you're right, it's a slider. You can
pick how much money you want to spend and what quality of service you
get. Are there new technologies coming, either for the Rust Infra Team
or the packaging world in general, to help with these security
problems? New sandboxing technologies or higher-level support?

**Jan David Nose**: A lot of people are working on this problem from
different angles. Internally we've talked a lot about it, especially in
the context of Crater. Crater pulls in all of those crates to build
them and get feedback from the Rust compiler. That means if someone
publishes malicious code, we will download it and build it.

**Jan David Nose**: In Rust this is a particular challenge because
build scripts can essentially do anything on your machine. For us that
means we need strong sandboxing. We've built our own sandboxing
framework so every crate build runs in an isolated container, which
prevents malicious code from escaping and messing with the host systems.

**Jan David Nose**: We feel that pain in Crater, but if we can solve it
in a way that isn't exclusive to Crater—if it also protects user
machines from the same vulnerabilities—that would be ideal. People like
Walter on the Foundation side are actively working on that. I'm sure
there are conversations in the Cargo and crates teams as well, because
every team that deals with packages sees a different angle of the
problem. We all have to come together to solve it, and there is a lot
of interesting work happening in that area.

**Xander Cesari**: I hope help is coming.

**Jan David Nose**: I'm optimistic.

**Xander Cesari**: We have this exponential curve with traffic and
everything else. It seems like at some point it has to taper off.

**Jan David Nose**: We'll see. Rust is a young language. I don't know
when that growth will slow down. I think there's a good argument that
it will continue for quite a while as adoption grows.

**Jan David Nose**: Being at a conference like RustConf, it's exciting
to see how the mix of companies has changed over time. We had a talk
from Rivian on how they use Rust in their cars. We've heard from other
car manufacturers exploring it. Rust is getting into more and more
applications that a few years ago would have been hard to imagine or
where the language simply wasn't mature enough yet.

**Jan David Nose**: As that continues, I think we'll see new waves of
growth that sustain the exponential curve we currently have, because
we're moving into domains that are new for us. It's amazing to see who
is talking about Rust and how they're using it, sometimes in areas like
space that you wouldn't expect.

**Jan David Nose**: I'm very optimistic about Rust's future. With this
increase in adoption, we'll see a lot of interesting lessons about how
to use Rust and a lot of creative ideas from people building with it.
With more corporate adoption, I also expect a new wave of investment
into the ecosystem: companies paying people to work full-time on
different parts of Rust, both in the ecosystem and in the core project.
I'm very curious what the next ten years will look like, because I
genuinely don't know.

**Xander Cesari**: The state of Rust right now does feel a bit like the
dog that caught the car and now doesn't know what to do with it.

**Jan David Nose**: Yeah, I think that's a good analogy. Suddenly we're
in a situation where we realize we haven't fully thought through every
consequence of success. It's fascinating to see how the challenges
change every year. We keep running into new growing pains where
something that wasn't an issue a year ago suddenly becomes one because
growth keeps going up.

**Jan David Nose**: We're constantly rebuilding parts of our
infrastructure to keep up with that growth, and I don't see that
stopping soon. As a user, that makes me very excited. With the language
and the ecosystem growing at this pace, there are going to be very
interesting things coming that I can't predict today.

**Jan David Nose**: For the project, it also means there are real
challenges: financing the infrastructure we need, finding maintainers
and contributors, and creating a healthy environment where people can
work without burning out. There is a lot of work to be done, but it's
an exciting place to be.

**Xander Cesari**: Well, thank you for all your work keeping those
magic Cargo commands I can type into my terminal just working in the
background. If there's any call to action from this interview, it's
that if you're a company using Rust, maybe think about donating to keep
the Infra Team working.

**Jan David Nose**: We always love new Rust Foundation members.
Especially if you're a company, that's one of the best ways to support
the work we do. Membership gives us a budget we can use either to fund
people who work full-time on the project or to fill gaps in our
infrastructure sponsorship where we don't get services for free and
have to pay real money.

**Jan David Nose**: And if you're not a company, we're always looking
for people to help out. The Infra Team has a lot of Rust-based bots and
other areas where people can contribute relatively easily.

**Xander Cesari**: Small scoped bots that you can wrap your head around
and help out with.

**Jan David Nose**: Exactly. It is a bit harder on the Infra side
because we can't give people access to our cloud infrastructure. There
are areas where it's simply not possible to contribute as a volunteer
because you can't have access to the production systems. But there is
still plenty of other work that can be done.

**Jan David Nose**: Like every other team in the project, we're a bit
short-staffed. So when you're at conferences, come talk to me or Marco.
We have work to do.

**Xander Cesari**: Well, thank you for doing the work that keeps Rust
running.

**Jan David Nose**: I'm happy to.

**Xander Cesari**: Awesome. Thank you so much.

[Content Team]: https://rust-lang.org/governance/teams/launching-pad/#team-content
[Xander Cesari]: https://github.com/MerrimanInd
[Jan David Nose]: https://github.com/jdno
[Infrastructure Team]: https://rust-lang.org/governance/teams/infra/
[interview here]: https://www.youtube.com/watch?v=r7i-2wHtNjw
