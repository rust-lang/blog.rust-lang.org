---
layout: post
title: "State of the crates.io"
author: Sean Griffin
---

If you've been watching Twitter, you may have noticed that there's been a lot of
activity around crates.io recently. I wanted to share some background with
what's been happening, what the state of the project is, and what our plans are
for the future.

The results of the work we've been doing is really exciting, so let's talk about
those first. Our 99th percentile response times are under 100ms across every
endpoint in the application. Based on some initial load testing, we can likely
scale to nearly 10x our current traffic on our current setup, and should easily
be able to scale to accomodate even more.

We've also got much better visibility into our traffic patterns and what's
happening with the site at any given moment, and we've set up an on call
rotation to ensure that incidents get resolved in a timely fashion 24 hours a
day.

After the person previously responsible for operations of the crates.io had to
step away, I sent a lot of frantic pings trying to find out who actually had
access to production. Ashley Williams stepped in and helped get us the
credentials we needed, and the two of us have been handling operations for the
site since then.

When we first took over, things were not in great shape. The first and most
pressing problem was that there was no monitoring set up. When there were major
spikes in error responses, nobody was getting paged. The first thing we did was
set up a pagerduty rotation.

Inevitably when you add monitoring to something that didn't have it before, you
learn that there are more problems than you realized. So we got paged... in the
middle of the night... a lot.

![Three incidents in the middle of the night](/images/2018-04-Crates.io/pages.png){:class="center"}

The first problem was that the endpoint to download a crate was sometimes taking
more than 30 seconds, causing the request to be killed. This endpoint should be
nearly instant, so the fluctuation was surprising. The only thing that endpoint
does is increment a download counter before redirecting to our CDN. So this
should never have been timing out.

The first thing we needed to do was determine whether the thing that was timing
out was waiting for a database connection or executing the query. [We added the
ability to configure more of these knobs with enviornment
variables.][env-var-configs] After deploying this we confirmed that it was the
database query that was timing out.

[env-var-configs]: https://github.com/rust-lang/crates.io/pull/1344

Heroku PostgreSQL has a great dashboard which shows you how long queries take
over time. We could see there was a high variability in how long this simple
query was taking, which made me suspect it was a lock contention issue. After
digging through the logs further, we learned that Heroku automatically logs any
time a query is waiting on a lock for more than 1 second. This confirmed that
we did have a lock contention problem.

After searching for anywhere else in the code accessing this table, we narrowed
the problem to a background job called `update-downloads`. When you download a
crate, the only counter that gets incremented immediately is in a table called
`version_downloads`. There are a bunch of different download stats that we show.
We could calculate all of this from `version_downloads`, but doing that would be
very slow. Instead we have a bunch of different tables which store the
information we care about at each resolution (per version vs per crate, per day
vs all time).

We have a script that runs every 5 minutes to populate all of these other
tables. This script processes rows from `version_downloads` in batches of 1000.
One of the things that it does is indicate how many downloads we've "counted",
which locks the row until the transaction commits. This script was wrapping each
batch of 1000 in a transaction, so the first row it touches would be locked
until 999 other rows get processed. Looking at the logs we could see that
processing a batch of 1000 was taking just over 30 seconds. Bingo.

Once we identified the problem, the solution didn't take long. [We changed the
script to wrap each row in a transaction, instead of each batch][less-locking].
After the fix was deployed, we immediately saw the query drop to sub millisecond
execution time, and the timeouts disappeared.

![Graph of the query execution time going from 152 milliseconds on average to 1
millisecond on average](/images/2018-04-Crates.io/crate-download-query.png){:class="center"}

We also noticed that the query to get the unprocessed downloads was taking far
longer than it should have. [Some index tuning fixed the problem][version-downloads-tuning].

![Graph of the query execution time going from 14 seconds on average to 2
milliseconds on average](/images/2018-04-Crates.io/update-downloads-query.png){:class="center"}

[less-locking]: https://github.com/rust-lang/crates.io/pull/1345
[version-downloads-tuning]: https://github.com/rust-lang/crates.io/pull/1346

While this fixed our timeout problem, we still had a major performance problem.
[People had reported that the home page was taking as much as 6 seconds to
load](https://github.com/rust-lang/crates.io/issues/1304). It was a very
intermittent problem. Sometimes it would load "fast enough", other times it
would take several seconds.

The other endpoint which was extermely slow was "/crates". We started combing
through the logs for these two endpoints to see if there were any patterns
around the times that requests slowed down. At this point we realized we were
being spammed by several crawlers at a rate we couldn't handle.

We could have fixed this by upgrading our database server. We're on a pretty low
tier of database server, and we have a lot of room to scale vertically on that
front. However, not doing that forced us to solve several problems that we need
to solve eventually. For example, we didn't have a published crawler policy ([we
do now!](https://crates.io/policies)). This also forced us to introduce
mechanisms to block traffic which is causing us problems.

Once we identified the problematic bots and blocked their traffic, we saw our
database load drop down, and our response times returned to acceptable levels.

![Graph showing database load dropping from over 100% to
10%](/images/2018-04-Crates.io/db-load.png){:class="center"}

At this point we were no longer getting paged, but a few pages still felt slower
than they should. The main culprits were the homepage (`/summary`) and search
page (`/crates`). The main thing that both of these pages had in common is that
they cared about "recent downloads", the number of downloads a given crate has
had in the past 90 days.

Calculating this number isn't "slow" per-se. The query took about 500ms, which
is reasonable for that sort of operation. But it still made the site feel
sluggish. Plus these are the two endpoints that crawlers want to hit, so we
really shouldn't do anything slow there.

There are a bunch of solutions we could have gone with here, all of which
ultimately come down to some form of caching. [Ultimately we went with a
materialized view](https://github.com/rust-lang/crates.io/pull/1363), since it
was the simplest answer and didn't require introducing any additional services
to the application. Deploying this change made everything *noticeably* faster.
As we continue to grow, tracking downloads will end up being our biggest
bottleneck. That said, I think our current solution will be workable for quite a
long time.

Our final major problem hasn't been fixed yet, but it's one that we know how to
handle. Today when you upload a crate, it gets uploaded to our servers, and then
we upload it to S3 after that. Since we have a strict 30 second upper limit on
response time, this means that if your upload speed is too slow, or your crate
is too large, it simply can't be published. We plan to solve this in the next
few weeks by having Cargo upload directly to S3.

This application has been a joy to work on. We've never had to be concerned with
the performance of Rust. There's no GC to tune, no overhead from a runtime, no
unreasonable memory growth or hard to track down memory leaks. It's liberating.
I'm astonished at the level of performance we've been able to achieve with
virtually no caching.

This is one of the only applications I've worked on where the database is truly
our only bottleneck. We're also able to take advantage of our database more than
I'm used to. Diesel makes it possible for us to use all sorts of advance
features available in PostgreSQL without giving up type safety. That's only
possible due to the flexibility of Rust's type system. I'm excited to see what
the future holds for Rust in the web space.

We've got a lot of big plans for the future. We're hoping refine a lot of our
policies soon (Yes, we are also updating our terms of service for the GDPR...).
Another big to-do is publishing daily dumps of our data in machine readable
format, to reduce the need for bots to crawl us at all. We've also got more than
100 open issues. If you've been looking for a Rust project to get involved in,
we'd be happy to help get you started!
