+++
path = "inside-rust/2026/08/04/funding-team-progress-update-july-2026"
title = "Funding team progress update — July 2026"
authors = ["Niko Matsakis"]

[extra]
team = "the funding team"
team_url = "https://github.com/rust-lang/leadership-council/issues/294"
+++

Hi all! This is an update from the **funding team**. [Established by the leadership council](https://github.com/rust-lang/leadership-council/issues/294), the funding team is tasked with administering the Rust Foundation Maintainers Fund. Per [RFC #3931][], our initial focus has been on establishing the **Maintainer in Residence (MiR)** program. The goal of the MiR program is to offer a long-term, stable option to support people doing [maintenance work][wima].

[wima]: https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/

While [RFC #3931][] laid out the high-level structure of the program, going from the "idea" of a Maintainer in Residence program to an actual functioning entity involves a lot of small decisions. This post covers the questions we've been discussing. For some of those questions, we have preliminary decisions, for others we are still deliberating. In any case, we welcome your input.

[RFC #3931]: https://github.com/rust-lang/rfcs/pull/3931

## Funding total

We have so far received generous donations totaling $350K. We'll go into details on the sponsors when we announce our first Maintainer-in-Residence.

**Want to help?** You can sponsor the Rust Foundation Maintainers Fund directly on [GitHub Sponsors](https://github.com/sponsors/rustfoundation). For companies and larger donations, reach out to `funding@rust-lang.org`.

We're also brainstorming ideas for an individual fundraising campaign to help grow the fund; if you'd like to pitch in ideas or follow along, join the discussion on the [funding stream on Zulip](https://rust-lang.zulipchat.com/#narrow/channel/548261-funding/topic/Fundraising.20campaign/with/611653646).

## Gathering details: what we learned

We sent out messages to existing Rust team members asking people to fill out a form if they were interested in being supported by the funding team. We heard from **45** individuals looking for funding and **3** teams gave us a general update.

Broken out by area (with some double counting as some individuals opted for more than one area):

| Area                            | Count |
|---------------------------------|-------|
| compiler                        | 21    |
| libs                            | 4     |
| clippy                          | 3     |
| rustdoc                         | 2     |
| a-mir-formality                 | 1     |
| cargo                           | 1     |
| crates.io                       | 1     |
| infra                           | 1     |
| lang                            | 1     |
| mods                            | 1     |
| Rust Analyzer                   | 1     |
| rustup                          | 1     |
| Safety Critical Rust Consortium | 1     |

Of the respondents, **34** indicated interest in a general maintenance (MiR) role and **13** indicated interest in being funded towards specific *project goals* (some people indicated both). We also asked about desired arrangement and compensation; as expected, there was a wide variance, with many preferring full-time employment but also many looking for contracts or part-time work.

### 2025 survey results show the need is real

There is a definite need for funding. In November of 2025 we ran a survey amongst active Rust maintainers. In that survey we found that 35% of contributors who filled the survey are currently funded for their Rust contributions (and this also includes people who only receive a minimal monthly amount e.g. through GitHub Sponsors). A similar number of people (~35%) never received *any* funding for their upstream Rust work. Even though many of those people are not funded at all, the average number of hours spent by a Rust Project contributor on improving Rust is around 40 hours each month!

## Hashing out the details of the MiR program

The goal of the MiR program is to provide **open-ended funding to support Rust maintainers and by proxy also the Rust teams that they operate in**. As we discussed in our blog post ["What is Maintenance, anyway?"][wima], the precise needs of teams vary, so this can be tricky to define.

The important thing is that MiRs help "enable other contributors to evolve and improve the project". For some teams, this might mean helping to run triage meetings, or burning through a PR review backlog. For others, it might mean focusing on a big refactor that everybody on the team knows is necessary, but nobody has the time to do.

### General shape: year-long contracts with expectation to renew

To begin with, the MiR program will consist of year-long contracts that come with the expectation *but not guarantee* that they will be renewed year-over-year. It's too early in this program to judge how stable our funding will be and thus we are not able to make long-term promises around renewal. 

**Flat rate or negotiated rates?** One of the obvious questions that had to be decided is whether we should pay all Maintainers in Residence the same rate or whether we should negotiate on a case-by-case basis. We've decided to start with a **flat rate, independent of geography**. This is more transparent and easier to administer. It avoids questions of unfairness where someone is getting paid more because they're a better negotiator or because of where they live. On the flip side, it does mean that the MiR program will not be a good fit for people who live in expensive areas or who are looking for a higher salary than what we offer.

**How much to pay?** The next question of course is how much to pay. We looked at the incoming data that we received on what rates people are looking for, both in our recent survey as well as the November of 2025 survey, and also looked at comparable setups at other foundations (e.g., Zig). We've settled on the following flat rates, though in individual cases we may make small adjustments.  Note that we've opted not to document this as an *hourly* rate, but rather a *monthly* rate with various levels of expected activity. This is because we don't intend to have people submit hourly reports to account for their time.

| Activity level      | Rate per month |
|---------------------|----------------|
| Full time (~40h/wk) | $10K USD       |
| Half time (~20h/wk) | $5K USD        |
| Day       (~8h/wk)  | $2K USD        |
| Half-day  (~4h/wk)  | $1K USD        |

Note that these are the amounts the Rust Foundation pays to the person or entity taking on the work; they are not a gross salary or take-home figure, and how they translate to net pay will depend on your situation and jurisdiction.

**Exclusive?** The MiR contracts are not "exclusive" so if a MiR wishes to take additional contracts on the side, that is fine, so long as they do not interfere with their ability to do maintenance work as discussed above.

**Under discussion: Off-ramp period.** As noted earlier, we expect to renew MiR contracts year over year but we are not in a position to guarantee that. Therefore, we are considering including a certain amount of "padding" in our budgeting so that if we are not able to renew the contract after one year, we can give MiR recipients an "off-ramp". The idea would be that if we are not able to renew the contract for another year, we can give a short-term payout to help cover expenses (and any maintenance they may choose to do...) while they find a new role. The goal is that MiR recipients can focus on maintenance worry-free for one year, even if funding is not renewed. Questions to be decided:

* *Do we do this off-ramp?* We want to, but it does mean that we have fewer total MiRs.
* *Do we do this off-ramp for everyone?* We are thinking we would offer the off-ramp only to full-time MiR (or perhaps full- and half-time), since if the MiR is only covering a day, that implies that the MiR is less dependent on the salary.
* *How long is the off-ramp?* Our initial thought was 25% (3 months), but shorter would mean we can fund more work.

**Why not employment?** For many people, employment is the ideal setup (though by no means everyone), but we are not in a position to offer it at this time, because employment requires ongoing funding. As the MiR program evolves, we hope to be able to support long-term employment for established Maintainers in Residence, but that is something we'll have to figure out as the program matures. 

### How do we decide where to fund a MiR role?

We decided that rather than starting with individuals seeking funding, it was better to look at the **teams** that need funding first. But we know we're not going to be able to provide for all the teams at the level we would like to. Therefore, we wanted some rubric for how we prioritize which teams to fund. We settled on the following axioms that we can use to guide decisions:

* **End-user impact first.** We prefer to fund maintenance of areas that have a real (direct or indirect) impact on Rust users' experience.
* **Dire needs first.** Address the teams with the greatest need first — raise the floor before raising the ceiling.
* **Fund what no one else will.** We lean toward maintenance, review, and triage: work with no natural corporate sponsor and no realistic path to directed funding.
* **Scale to demand.** For areas with lots of potential for directed funding, we still want to fund maintainers to keep the code healthy between feature work.
* **Shovel-ready.** Between equal-priority needs, we start with the one that can begin the soonest.

These axioms are written in priority order and they're meant to help us resolve tough decisions. For example, if we have to pick between two teams, both of which have dire need, but one of which has more visible end-user impact, we'll pick the one with more visible end-user impact because that will help us in raising additional funding later. (At the same time, these are rough guidelines, not iron laws: there may be particulars of the situation that lead us the other way.)

**Short-list of teams.** Based on the identified criteria, we are focused first on the following teams: rustup, rustfmt, clippy, rustdoc, libs, cargo, and mods (in no particular order).

**Under discussion: what if there are no MiR applicants?** There are teams that we would like to support but where none of the existing team members are interested in a MiR role (perhaps they have another full-time job or just like working on a volunteer basis). [We are discussing how best to manage this situation when it arises.](https://rust-lang.zulipchat.com/#narrow/channel/548261-funding/topic/.22Recruiting.22.20for.20a.20team/with/609598431) For example, we might approach the team and see if there are any active contributors they would support for a MiR role or perhaps put out a call for applications.

**Restricted funding.** So far, all the funds in the MiR were given **unrestricted**, which means that the funding team and the project can decide how to spend them. However, we have also had sponsors raise the idea of **restricted funding**, where a sponsor wants to fund a MiR for a particular team (say, rustfmt, or cargo) because they are concerned about that area, or even wants to fund a specific *individual* who they know does good work. We decided that we would **happily accept restricted funding so long as the person is being funded to do maintenance work and generally treated like any other MiR**.

## What comes next?

This is our overall roadmap:

* [x] *Rust Foundation:* Announce program and begin fundraising
* [x] *Leadership Council:* Establish funding team and outlines of the MiR program ([RFC #3931]) 
* [x] *Funding team:* Gather information on who is looking for funding and what work they'd like to do  (👈 subject of this update)
* [x] *Funding team:* Hash out the details of the MiR program (👈 subject of this update)
* [ ] *Funding team:* Narrow down to a short list of candidate MiRs and begin individual discussions (currently ongoing)
* [ ] *Funding team:* Announce MiR selection (hopefully coming soon!)
* [ ] *MiRs:* Do your thing.

At this point we have assembled a short list of candidates for our initial round of MiRs and we are planning to begin discussing specifics with them. This will help us to narrow to the final list. We are aiming to have that list announced by the end of August.
