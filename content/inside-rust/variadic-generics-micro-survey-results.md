+++
path = "inside-rust/2026/99/99/variadic-generics-micro-survey-results"
title = "Variadic Generics Micro Survey results"
authors = ["Olivier Faure"]
aliases = ["inside-rust/2026/99/99/variadic-generics-micro-survey-results.html"]
+++

**TODO: Replace answer lists with charts**

A year ago, we launched the [Variadic Generics Micro Survey](https://blog.rust-lang.org/inside-rust/2025/09/22/variadic-generics-micro-survey/), with the goal of collecting people's use cases for a hypothetical [variadic generics](https://poignardazur.github.io/2021/01/30/variadic-generics/) feature.

Because of its open-ended purview, this survey included a lot of open answers. Not only that, respondents were *encouraged* to fill out answers, in as much detail as they wanted. And they did: we got over 3300 submissions, including 900 text answers to just one question.

We fed these answers to Claude and it told us th- ha, just kidding, if only.

More seriously, we did consider using AI to process the answers, but decided not to over privacy and quality concerns. Instead we ended up going through the dataset by hand, hence why these results took so long to publish.

This article will include both the numerical survey results and some trends observed in the free answers. See also the full report, which includes the polls and examples of answers showcasing the trends.

**TODO: Link to full report**


## Table of contents

* [Background](#background)
* [First use-cases](#first-use-cases)
* [Variadic mappings](#variadic-mappings)
* [Non-linear variadics](#non-linear-variadics)
* [Final questions](#final-questions)


## Background

This first questions were about users' backgrounds.

**How long have you been using Rust?**
- Never (24)
- Zero to two years (493)
- Two to four years (1121)
- Four to six years (867)
- More than six years (804)

**Have you heard about variadic generics before?**
- Yes (2718)
- No (586)

**Where have you heard about variadic generics?**
- Pre-RFCs and discussions on internals.rust-lang.org. (833)
- Discussions on Reddit. (1236)
- Blog posts on poignardazur.github.io. (1176)
- Other programming languages. (1630)
- Other means: (open response) (190)

Among the open responses, the most common were:
- Github (13)
- Zulip (16)
- Bevy (60)
- Discord (45)
  - Specifically, the Bevy Discord server (22)
  - Specifically, the Rust Programming Language Community Server (10)
- This Week in Rust (7)
- Other social apps (14)
- Friends or colleagues (14)
- Existing code or docs (7)
- Solving a problem (15)

The "solving a problem" category is interesting: a few people mentioned wanting to solve a specific problem, doing some research, and finding that they wanted was called "variadic generics".

Here we can already notice the Bevy community is somewhat over-represented in the answers. It's not clear to what extent this is because Bevy users care a lot about variadics, or because there was a lot of advertising for the survey on the Bevy Discord server.


## First use-cases

This code example was given to illustrate variadic generics (with pseudocode syntax):

```rust
impl<...Ts: SomeTrait> SomeTrait for (...Ts) {
    fn do_stuff(&self) -> u32 {
        let mut sum = 0;
        for member in ...self {
            // The trait bounds ensure each member has a do_stuff() method
            sum += member.do_stuff();
        }
        sum
    }
}

let value: u32 = (0, 0.5, "hello", Some("hello"), false).do_stuff();
```

Users then saw this question:

**Are there cases where variadic generics would have made your project easier?**
- Yes (1702)
- No (1319)

This was the question with the most open answers: of the 1702 people who said yes above, 1025 gave more detail, from single-line answers to multi-page explanations.

We've split the open answers into rough categories:

- Implementing `SomeTrait` for all tuples whose members implement `SomeTrait` (227)
    - For UI (47)
    - For serialization (19)
- Implementing traits for all function types (97)
    - Implementing function wrappers (34)
- Combining parsers or state machines (112)
    - For file parsing (17)
    - For Web/RPC/REST endpoints (24)
    - For SQL/ORM queries (38)
- Representing a graph node (38)
    - For a compiler/interpreter (20)
- Language interop (32)
    - With C++ (8)
    - With C variadic functions (6)
    - With Python (6)
    - With Lua (5)
    - With another language (7)
- Variadic functions (65)
    - `zip`-like functions (26)
    - `join`-like functions (8)
    - `println`-like functions, logging (18)
- N-dimensional maths (20)
- Structure-of-arrays and ECS (52)
- Builder pattern (9)
- `io_uring` (4)
- Function pipelines (6)
- Dependency injection (38)

Some quotes (copied verbatim):

> Almost any time you implement a trait for a tuple you almost always want to implement it for all tuple sizes. I've usually seen this done with macros up to some fixed size.

> I have a parser combinator library.
As a parser combinator sugar i had to overload varying number of tuples with their own trait impls. That was annoying

> Mainly around querying (sql) with variable number of columns and / or parameters. This is worked around with macros (a la sqlx) but could be more elegant with tuples.

> I would love to have them for more explicit multi dimensional tensor types. Like Tensor<5,7,9>, this would easily catch dimensions mismatch on building neural networks, for instance.

> Struct-of-Arrays vector wants to decompose a type (struct or enum) into its fields: a tuple of field types or a tuple of field pointers is thus needed. Variadics could be helpful here

(See full report for a list of quotes illustrating each of those categories.)

Some people mentioned wanting variadics for a specific project:

- Bevy (83)
- Axum (21)
- Diesel (9)

A lot of people mentioned wanting to replace macros, or using macros as a substitute for variadics. Most people were neutral about this. A few (about 7) explicitly declared the macros workaround were enough for them.

More people mentioned strongly disliking the macro workaround, for various reasons:

(Total: 51)
- Documentation bloat (10)
- Source code bloat (13)
- Unreadable compile errors (14)
- Long compile times (13)

A few answers mentioned implementing the macro workaround for tuples up to a given size, and then finding out that size wasn't enough for their users.

> I've made safe Rust bindings for C libraries with variadic functions that use printf(3)-style formatting. In doing so, I had to do a fair bit of work to support multiple numbers of arguments... and inevitably a use-case came up which required more arguments (and thus longer tuples) than had been provided for.

The most interesting answers are people who explicitly mentioned they gave up on a feature or an approach because of the lack of variadics.

> These days it's more about the opportunity loss: I know this is impossible, so I don't even think about reaching out for tuples of items implementing the same trait. I'm sure I've written situations where I had to use `Vec<Box<dyn Trait>>` not because I *wanted* boxed elements, but because the more natural thing (tuples of dierently-typed values all implementing a single trait of arbitrary length) doesn't exist.

> Sometimes I have also wanted to implement some crate-local trait for tuples but have found it too cumbersome due to the necessity to use a macro to do it, so I never bothered, even though I sort of wanted to.

> The current "solution"--as I'm sure you're aware--is the classic macro that implements some trait for a tuple of length N of generic parameters. That is so uncomfortable, slow, complex and error-prone I have seen many examples (as a contributor of bevy) where variadric generics were the best solutions to a given problem, but they were rejected because they were not worth the trouble for the current problem (compilation time, mainly).

## Variadic mappings

This code example was given to illustrate variadic mappings:

```rust
impl<...Ts> UnwrapAll for (Option<...Ts>) {
    type Unwrapped = (...Ts);
    fn unwrap_all(self) -> Self::Unwrapped {
        for option in ...self {
            option.unwrap()
        }
    }
}

let my_gift_boxes = (Some(1), Some(true), Some("hello"));
let opened_gifts = my_gift_boxes.unwrap_all();
assert_eq!(opened_gifts, (1, true, "hello"));
```

Users then saw this question:

**Are there cases where variadic mappings would have made your project easier?**

Yes - 1287
No - 1168

Of the 1287 people who said yes, 344 included free answers.

Overall, most of the free answers were variants of "Yes, it would be nice" or "Same use-case as the previous question":

> This feature would also be helpful for the use case described previously.
>
> I should add that, in general, probably 90% of the time I make a macro, it's for something that could be a variadic mapping.

> this would actually be helpful in every-day code

> I've had several instances of traits that make sense for tuples, but also have an associated type that should be "tupled" as well.

A few people pointed out that the pseudo-code syntax in the example was confusing:

> the syntax of \<Option ...Ts\> is kinda awkward for the first sight. It feels like it would be better to see something like ...Option\<Ts\>.
> It's a tuple of a few options, not the Option of some tuple.
> Thanks for the survey!

Because the free answers are overall similar to the ones we already discussed, we'll skip straight to the next section.


## Non-linear variadics

This code example was given to illustrate non-linear generics:

```rust
fn get_children<...Ts>(parents: (...Ts)) -> (for <T of ...Ts where T: Parent> T::Child) {
    for parent in ...parents where parent: Parent {
        parent.child()
    }
}
```

Users then saw this question:

**Are there cases where non-linear mappings would have made your project easier?**

Yes - 696
No - 1537

In retrospect, the example is clearly too leading. The question was supposed to point to any kind of operation on lists of types beside "one-to-one mapping"; but the example oriented responses towards filtering specifically. That example would also, if allowed, be a workaround for trait specialization, which a lot of people focused on.

Of the 696 people who said yes, 205 included free answers. The use-cases in the free answers roughly matched these categories:

- Type filtering (38)
  - Specifically Bevy queries (10)
- As a specialization workaround (18)
- Other specific use-cases (19)

Overall, the sentiment was more cautious than in previous questions. A lot of responses to the first questions were along the lines of "Yes, I absolutely need this yesterday for X/Y/Z", whereas the responses to this question were more likely to hedge or mention hypothetical use-cases:

> Not sure, maybe that can help with some not yet implemented features.

> Feels like this could enhance bevy queries in some way.

> This seems like a useful tool in general to work with bundles of possible behavior in a convenient and efficient way. Imagine defining systems in an ECS in terms of variadic generics that match against entities with patterns of components. Not sure exactly what this would look like, and I'd imagine it might not be possible simply through this function (might need first class types which seem overkill).

Some interesting answers:

> My company uses Rust for financial time series processing. Our data processing library currently uses a kludgy tuple-based method of defining data pipelines, which is constrained by max tuple length and requires a macro to define the implementation of.
> 
> VarArgs would allow us to directly have a function signature like `fn pipe<...Ts>(transforms: *Ts)`, without the extra tuple or extra complexity
> 
> Specifically, I've wanted a variadic version of this function:
> 
> ```rust
> fn pipe(p1: Pipe<T1, T2>, p2: Pipe<T2, T3>, ..., pn: Pipe<Tn-1, Tn>) -> Pipe<T1, Tn>
> ```

> It definitely is an 'advanced' feature, that I ultimately didn't 'need'. My use case was composable structs for building a Query, things like `And<(Has<Position>, Not<Or<...>>)>`, and some of these structs had creation parameters, currently a tuple is created with all parameters for structs like 'And'
>
> But because only a subset of the structs take parameters it makes the api awkward:
`And::new(((),(),...))`
>
> And I wanted to remove the unit types, I tried really hard with traits and unstable features but ultimately abandoned the idea.

> Somewhat similar to how Bevy handles systems, I have built a transaction that can take in an arbitrary number of requests and execute them. Additionally, a custom logic function can be provided that takes in each request (or rather their resulting value) as an argument.
>
> The transaction requests are in arbitrary order for API convenience, but need to be executed in a fixed order. Currently I build an array of `(u8, Priority)` and sort it by priority to get my order, then map the u8 values to each tuple field, execute the request and map the result onto a tuple of `Option<RequestN::Result>`. Finally, I return the tuple unwrapping all fields.
>
> If I could have sorted the tuple, most of this wouldn't have been neccessary.


## Final questions

The second-last question was about people's priorities:

**How high a priority would you say variadics are for you?**

- It should be the highest priority feature (48)
- Very important (286)
- Moderately important (599)
- Nice to have (1015)
- I don't care (182)
- I don't want Rust to have variadics (99)

The last question was **Do you have any further comments related to variadic generics?**, and 572 people answered.

This last question is where skeptics who had skipped the previous questions voiced their concerns and doubts, while enthusiasts reaffirmed their support:

- I really want variadics (34)
- Variadics add too much complexity to the language (22)
- Rust must not become like C++ (18)
- I miss variadics from C++ (22)
- Macros work fine as a workaround (25)
- Non-linear variadics are too complex (13)
- The article's example syntax is bad  (28)
    - The syntax should be keyword-based, not dot-dot-dot (5)
    - The "for where" non-linear syntax is bad (7)
- Feature X is more important (23)
- We should have type-to-type functions (7)
- Variadics should also cover struct fields (8)
- Compliments/thanks (18)

A lot of answers (23) were specifically about why they didn't pick "It should be the highest priority feature", and listed other features they considered more important. In those answers, "specialization" and "better const generics" were common picks.

It's hard to synthetise a coherent takeaway from all the answers at once, since a lot of them expressed incompatible wishes. The overall vibe was "cautious, but interested".

Among the skeptics, the most frequent concern was the idea that Rust might suffer a death by a thousand cuts, that an accumulation of small arcane features might make it impenetrable to new users. The skeptics were a minority, but large enough that future proposals shouldn't simply ignore them.

Future efforts to promote variadics should ideally propose a syntax that feels easy to read, and should demonstrate strong upsides over the current macro workarounds.

Finally, the last type of answer was words of encouragement from readers of the survey:

> Keep up the good work!

> Thank you for all the work you do!

> Anyway, kudos for putting out this survey! Really appreciate the effort to surface the opinions and wishes of us amateur grug-brained type wranglers as well!

On behalf of the survey team, we'd like to return the sentiment, thank all the participants who took the time to write about their needs and concerns, and thank *you* for reading this far.

<tiny>Also, sorry this article took so long.</tiny>

The Rust community's enthusiasm is what keeps the language alive.

Thanks again!
