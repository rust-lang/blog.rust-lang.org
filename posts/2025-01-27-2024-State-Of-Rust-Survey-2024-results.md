---
layout: post
title: "2024 State of Rust Survey Results"
author: The Rust Survey Team
---

Hello, Rustaceans!

The Rust Survey Team is excited to share the results of our [2024 survey on the Rust Programming language](https://blog.rust-lang.org/2024/12/05/annual-survey-2024-launch.html), conducted between December 5, 2024 and December 23, 2024.
As in previous years, the 2024 State of Rust Survey was focused on gathering insights and feedback from Rust users, and all those who are interested in the future of Rust more generally.

This ninth edition of the survey surfaced new insights and learning opportunities straight from the global Rust language community, which we will summarize below. In addition to this blog post, we have also prepared a [report][report] containing charts with aggregated results of all questions in the survey.

**Our sincerest thanks to every community member who took the time to express their opinions and experiences with Rust over the past year. Your participation will help us make Rust better for everyone.**

There's a lot of data to go through, so strap in and enjoy!

## Participation

| **Survey** | **Started** | **Completed** | **Completion rate** | **Views** |
|:----------:|------------:|--------------:|--------------------:|----------:|
|    2023    |      11 950 |         9 710 |               82.2% |    16 028 |
|    2024    |        TODO |          TODO |               TODO% |      TODO |

TODO - outdated, don't have the data yet

[//]: # (As shown above, in 2023, we have received 37% fewer survey views in vs 2022, but saw a slight uptick in starts and completions. There are many reasons why this could have been the case, but it’s possible that because we released the [2022 analysis blog]&#40;https://blog.rust-lang.org/2023/08/07/Rust-Survey-2023-Results.html&#41; so late last year, the survey was fresh in many Rustaceans’ minds. This might have prompted fewer people to feel the need to open the most recent survey. Therefore, we find it doubly impressive that there were more starts and completions in 2023, despite the lower overall view count.)

## Community

TODO - outdated, don't have the data yet

[//]: # (We saw a 3pp increase in respondents taking this year’s survey in English – 80% in 2023 and 77% in 2022. Across all other languages, we saw only minor variations – all of which are likely due to us offering fewer languages overall this year due to having fewer volunteers.)

Same as every year, we asked our respondents in which country they live in. The top 10 countries represented were, in order: United States (22%), Germany (12%), China (6%), United Kingdom (6%), France (6%), Canada (3%), Russia (3%), Netherlands (3%), Japan (3%), and Poland (3%) . We were interested to see a small reduction in participants taking the survey in the United States in 2023 (down 3pp from the 2022 edition) which is a positive indication of the growing global nature of our community! You can try to find your country in the chart below:

<!-- Chart where-do-you-live start -->
<div>
    <div class="pie-chart" id="where-do-you-live" style="height:600px; width:100%;"><noscript>
<img alt="where-do-you-live" height="600" src="../../../images/2025-01-rust-survey-2024/where-do-you-live.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/where-do-you-live.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/where-do-you-live.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart where-do-you-live end -->

[//]: # (Once again, the majority of our respondents reported being most comfortable communicating on technical topics in English at 92.7% — a slight difference from 93% in 2022. Again, Chinese was the second-highest choice for preferred language for technical communication at 6.1% &#40;7% in 2022&#41;.)

[//]: # ()
[//]: # (<!-- chart: what-are-your-preferred-languages-for-technical-communication &#40;height=400&#41; -->)

[//]: # ()
[//]: # (We also asked whether respondents consider themselves members of a marginalized community. Out of those who answered, 76% selected no, 14% selected yes, and 10% preferred not to say.)

[//]: # ()
[//]: # (We have asked the group that selected “yes” which specific groups they identified as being a member of. The majority of those who consider themselves a member of an underrepresented or marginalized group in technology identify as lesbian, gay, bisexual, or otherwise non-heterosexual. The second most selected option was neurodivergent at 41% followed by trans at 31.4%. Going forward, it will be important for us to track these figures over time to learn how our community changes and to identify the gaps we need to fill.)

[//]: # ()
[//]: # (<!-- chart: which-marginalized-group &#40;height=500&#41; -->)

[//]: # ()
[//]: # (As Rust continues to grow, we must acknowledge the diversity, equity, and inclusivity &#40;DEI&#41;-related gaps that exist in the Rust community. Sadly, Rust is not unique in this regard. For instance, only 20% of 2023 respondents to this representation question consider themselves a member of a racial or ethnic minority and only 26% identify as a woman. We would like to see more equitable figures in these and other categories. In 2023, the Rust Foundation formed a diversity, equity, and inclusion subcommittee on its Board of Directors whose members are aware of these results and are actively discussing ways that the Foundation might be able to better support underrepresented groups in Rust and help make our ecosystem more globally inclusive. One of the central goals of the Rust Foundation board's subcommittee is to analyze information about our community to find out what gaps exist, so this information is a helpful place to start. This topic deserves much more depth than is possible here, but readers can expect more on the subject in the future.)

## Rust usage

The number of respondents that self-identify as a Rust user was quite similar to last year, around 92%. This high number is not surprising, since we primarily target existing Rust developers with this survey.

<!-- Chart do-you-use-rust start -->
<div>
    <div class="bar-chart" id="do-you-use-rust" style="height:300px; width:100%;"><noscript>
<img alt="do-you-use-rust" height="300" src="../../../images/2025-01-rust-survey-2024/do-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/do-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/do-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart do-you-use-rust end -->

Similarly as last year, around 31% of those who did not identify as Rust users cited the perception of difficulty as the primary reason for not using Rust. The most common reason for not using Rust was that the respondents simply haven’t had the chance to try it yet.

<!-- Chart why-dont-you-use-rust start -->
<div>
    <div class="bar-chart" id="why-dont-you-use-rust" style="height:500px; width:100%;"><noscript>
<img alt="why-dont-you-use-rust" height="500" src="../../../images/2025-01-rust-survey-2024/why-dont-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/why-dont-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/why-dont-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/why-dont-you-use-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart why-dont-you-use-rust end -->

Of the former Rust users who participated in the 2024 survey, 36% cited factors outside their control as a reason why they no longer use Rust, which is a 10pp decrease from last year. This year, we also asked respondents if they would consider using Rust again if an opportunity comes up, which turns out to be true for a large fraction of the respondents (63%). That is good to hear!

<!-- Chart why-did-you-stop-using-rust start -->
<div>
    <div class="bar-chart" id="why-did-you-stop-using-rust" style="height:500px; width:100%;"><noscript>
<img alt="why-did-you-stop-using-rust" height="500" src="../../../images/2025-01-rust-survey-2024/why-did-you-stop-using-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/why-did-you-stop-using-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/why-did-you-stop-using-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/why-did-you-stop-using-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart why-did-you-stop-using-rust end -->

> Closed answers marked with N/A were not present in the previous version(s) of the survey.

Of those who used Rust in 2024, 53% did so on a daily (or nearly daily) basis — an increase of 4pp from the previous year. We can observe an upward trend in the frequency of Rust usage over the past few years.

<!-- Chart how-often-do-you-use-rust start -->
<div>
    <div class="bar-chart" id="how-often-do-you-use-rust" style="height:300px; width:100%;"><noscript>
<img alt="how-often-do-you-use-rust" height="300" src="../../../images/2025-01-rust-survey-2024/how-often-do-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/how-often-do-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/how-often-do-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-often-do-you-use-rust end -->

Rust expertise is also continually increasing amongst our respondents! 20% of respondents can write (only) simple programs in Rust (a decrease of 3pp from 2023), while 53% consider themselves productive using Rust — up from 47% in 2023. While the survey is just one tool to measure the changes in Rust expertise overall, these numbers are heartening as they represent knowledge growth for many Rustaceans returning to the survey year over year.

<!-- Chart how-would-you-rate-your-rust-expertise start -->
<div>
    <div class="bar-chart" id="how-would-you-rate-your-rust-expertise" style="height:500px; width:100%;"><noscript>
<img alt="how-would-you-rate-your-rust-expertise" height="500" src="../../../images/2025-01-rust-survey-2024/how-would-you-rate-your-rust-expertise.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/how-would-you-rate-your-rust-expertise.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/how-would-you-rate-your-rust-expertise.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-would-you-rate-your-rust-expertise end -->

## Learning Rust
To use Rust, programmers first have to learn it, so we are always interested in finding out how do they approach that. Based on the survey results, it seems that most users learn from Rust documentation and also from [The Rust Programming Language](https://doc.rust-lang.org/book/) book, which has been a favourite learning resource of new Rustaceans for a long time. Many people also seem to learn by reading the source code of Rust crates. The fact that both the documentation and source code of tens of thousands of Rust crates is available on [docs.rs](https://docs.rs) and GitHub makes this easier.

<!-- Chart what-kind-of-learning-materials-have-you-consumed start -->
<div>
    <div class="bar-chart" id="what-kind-of-learning-materials-have-you-consumed" style="height:500px; width:100%;"><noscript>
<img alt="what-kind-of-learning-materials-have-you-consumed" height="500" src="../../../images/2025-01-rust-survey-2024/what-kind-of-learning-materials-have-you-consumed.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/what-kind-of-learning-materials-have-you-consumed.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-kind-of-learning-materials-have-you-consumed.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-kind-of-learning-materials-have-you-consumed-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart what-kind-of-learning-materials-have-you-consumed end -->

On the other hand, only a very small number of respondents (around 3%) have taken a university Rust course or use university learning materials. It seems that Rust has not yet penetrated university curriculums, as this is typically a very slow moving area.

<!-- Chart have-you-taken-a-rust-course start -->
<div>
    <div class="bar-chart" id="have-you-taken-a-rust-course" style="height:600px; width:100%;"><noscript>
<img alt="have-you-taken-a-rust-course" height="600" src="../../../images/2025-01-rust-survey-2024/have-you-taken-a-rust-course.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/have-you-taken-a-rust-course.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/have-you-taken-a-rust-course.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart have-you-taken-a-rust-course end -->

## Programming environment

In terms of operating systems used by Rustaceans, the situation stayed very similar over the past couple years, with Linux being the most popular choice, followed by macOS and Windows, which have a very similar share of usage. As you can see in the linked [wordcloud](../../../images/2025-01-rust-survey-2024/which-os-do-you-use-wordcloud.png), there are also a few users that prefer Arch, btw.

<!-- Chart which-os-do-you-use start -->
<div>
    <div class="bar-chart" id="which-os-do-you-use" style="height:400px; width:100%;"><noscript>
<img alt="which-os-do-you-use" height="400" src="../../../images/2025-01-rust-survey-2024/which-os-do-you-use.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-use.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-use.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-use-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-os-do-you-use end -->

Rust programmers target a diverse set of platforms with their Rust programs. We saw a slight uptick in users targeting embedded and mobile platforms, but otherwise the distribution of platforms stayed mostly the same as last year. Since the WebAssembly target is quite diverse, we have split it into two separate categories this time. Based on the results it is clear that when using WebAssembly, it is mostly in the context of browsers (23%) rather than other use-cases (7%).

<!-- Chart which-os-do-you-target start -->
<div>
    <div class="bar-chart" id="which-os-do-you-target" style="height:500px; width:100%;"><noscript>
<img alt="which-os-do-you-target" height="500" src="../../../images/2025-01-rust-survey-2024/which-os-do-you-target.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-target.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-target.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-os-do-you-target-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-os-do-you-target end -->

We cannot of course forget the favourite topic of many programmers: which IDE (developer environment) do they use. Although Visual Studio Code still remains the most popular option, its share has dropped by 5pp this year. On the other hand, the Zed editor seems to have gained considerable traction recently.

<!-- Chart what-ide-do-you-use start -->
<div>
    <div class="bar-chart" id="what-ide-do-you-use" style="height:500px; width:100%;"><noscript>
<img alt="what-ide-do-you-use" height="500" src="../../../images/2025-01-rust-survey-2024/what-ide-do-you-use.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/what-ide-do-you-use.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-ide-do-you-use.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-ide-do-you-use-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart what-ide-do-you-use end -->

> You can also take a look at the linked [wordcloud](../../../images/2025-01-rust-survey-2024/what-ide-do-you-use-wordcloud.png) that summarizes open answers to this question (the "Other" category), to see what other editors are also popular.

## Rust at Work

We were excited to see that more and more people use Rust at work for the majority of their coding, 38% vs 34% from last year. There is a clear upward trend in this metric over the past few years.

The usage of Rust within companies also seems to be rising, 45% of respondents answered that their organisation makes non-trivial use of Rust, which is a 7pp increase from 2023.

<!-- Chart do-you-personally-use-rust-at-work start -->
<div>
    <div class="bar-chart" id="do-you-personally-use-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="do-you-personally-use-rust-at-work" height="500" src="../../../images/2025-01-rust-survey-2024/do-you-personally-use-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/do-you-personally-use-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/do-you-personally-use-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart do-you-personally-use-rust-at-work end -->

<!-- Chart how-is-rust-used-at-your-organization start -->
<div>
    <div class="bar-chart" id="how-is-rust-used-at-your-organization" style="height:600px; width:100%;"><noscript>
<img alt="how-is-rust-used-at-your-organization" height="600" src="../../../images/2025-01-rust-survey-2024/how-is-rust-used-at-your-organization.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/how-is-rust-used-at-your-organization.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/how-is-rust-used-at-your-organization.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-is-rust-used-at-your-organization end -->

Once again, the top reason employers of our survey respondents invested in Rust was the ability to build relatively correct and bug-free software. The second most popular reason was Rust’s performance characteristics. 21% of respondents that use Rust at work do so because they already know it, and it's thus their default choice, an uptick of 5pp from 2023. This seems to suggest that Rust is becoming one of the baseline languages of choice for more companies.

<!-- Chart why-you-use-rust-at-work start -->
<div>
    <div class="bar-chart" id="why-you-use-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="why-you-use-rust-at-work" height="500" src="../../../images/2025-01-rust-survey-2024/why-you-use-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/why-you-use-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/why-you-use-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart why-you-use-rust-at-work end -->

Similarly to the previous year, a large percentage of respondents (82%) report that Rust helped their company achieve its goals. In general, it seems that programmers and companies are quite happy with their usage of Rust, which is great!

<!-- Chart which-statements-apply-to-rust-at-work start -->
<div>
    <div class="bar-chart" id="which-statements-apply-to-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="which-statements-apply-to-rust-at-work" height="500" src="../../../images/2025-01-rust-survey-2024/which-statements-apply-to-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/which-statements-apply-to-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-statements-apply-to-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart which-statements-apply-to-rust-at-work end -->

In terms of technology domains, the situation is quite similar to the previous year. Rust to be especially popular for creating server backends, web and networking services and cloud technologies. It also seems to be gaining more traction for embedded use-cases. 

<!-- Chart technology-domain start -->
<div>
    <div class="bar-chart" id="technology-domain" style="height:600px; width:100%;"><noscript>
<img alt="technology-domain" height="600" src="../../../images/2025-01-rust-survey-2024/technology-domain.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/technology-domain.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/technology-domain.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/technology-domain-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart technology-domain end -->

> You can scroll the chart to the right to see more domains. Note that the Automotive domain was not offered as a closed answer in the 2023 survey (it was merely entered through open answers), which might explain the large jump.

It is exciting to see the continued growth of professional Rust usage and the confidence so many users feel in its performance, control, security and safety, enjoyability, and more!

## Challenges

As always, one of the main goals of the State of Rust survey is to shed light on challenges, concerns, and priorities on Rustaceans’ minds over the past year.

We have asked our users about aspects of Rust that limit their productivity. Perhaps unsurprisingly, slow compilation was at the top of the list, followed by subpar support for debugging Rust and high disk usage of Rust compiler artifacts. On the other hand, most Rust users seem to be very happy with its runtime performance, the correctness and stability of the compiler and also Rust's documentation.

<!-- Chart which-problems-limit-your-productivity start -->
<div>
    <div class="matrix-chart" id="which-problems-limit-your-productivity" style="height:1000px; width:100%;"><noscript>
<img alt="which-problems-limit-your-productivity" height="600" src="../../../images/2025-01-rust-survey-2024/which-problems-limit-your-productivity.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/which-problems-limit-your-productivity.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-problems-limit-your-productivity.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-problems-limit-your-productivity-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-problems-limit-your-productivity end -->

In terms of specific unstable (or missing) features that Rust users want to be stabilized (or implemented), the most desired ones were async closures and if/let while chains. Well, we have good news! Async closures will be stabilized in the next version of Rust (1.85), and if/let while chains will hopefully follow [soon after](https://github.com/rust-lang/rust/pull/132833), once Edition 2024 is released (which will also happen in Rust 1.85).

Other coveted features are generators (both sync and async) and more powerful generic const expressions. You can follow the [Rust Project Goals](https://rust-lang.github.io/rust-project-goals/2025h1/goals.html) to track the progress of these (and other) features.

<!-- Chart which-features-do-you-want-stabilized start -->
<div>
    <div class="matrix-chart" id="which-features-do-you-want-stabilized" style="height:800px; width:100%;"><noscript>
<img alt="which-features-do-you-want-stabilized" height="600" src="../../../images/2025-01-rust-survey-2024/which-features-do-you-want-stabilized.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/which-features-do-you-want-stabilized.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-features-do-you-want-stabilized.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/which-features-do-you-want-stabilized-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-features-do-you-want-stabilized end -->

This year, we have also included a new question about the speed of Rust's evolution. While most people seem to be content with the status quo, more than a quarter of people who responded to this question would like Rust to stabilize and/or add features more quickly, and only 7% of respondents would prefer Rust to slow down or completely stop adding new features.

<!-- Chart what-do-you-think-about-rust-evolution start -->
<div>
    <div class="bar-chart" id="what-do-you-think-about-rust-evolution" style="height:600px; width:100%;"><noscript>
<img alt="what-do-you-think-about-rust-evolution" height="600" src="../../../images/2025-01-rust-survey-2024/what-do-you-think-about-rust-evolution.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/what-do-you-think-about-rust-evolution.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-do-you-think-about-rust-evolution.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart what-do-you-think-about-rust-evolution end -->

Interestingly, when we asked respondents about their main worries for the future of Rust, one of the top answers remained the worry that Rust will become too complex. This seems to be in contrast with the answers to the previous question. Perhaps Rust users still seem to consider the complexity of Rust to be manageable, but they worry that one day it might become too much.

We are happy to see that the amount of respondents concerned about Rust Project governance and lacking support of the Rust Foundation has dropped by about 6pp from 2023.

<!-- Chart what-are-your-biggest-worries-about-rust start -->
<div>
    <div class="bar-chart" id="what-are-your-biggest-worries-about-rust" style="height:500px; width:100%;"><noscript>
<img alt="what-are-your-biggest-worries-about-rust" height="500" src="../../../images/2025-01-rust-survey-2024/what-are-your-biggest-worries-about-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2025-01-rust-survey-2024/what-are-your-biggest-worries-about-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-are-your-biggest-worries-about-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2025-01-rust-survey-2024/what-are-your-biggest-worries-about-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart what-are-your-biggest-worries-about-rust end -->

## Looking ahead

Each year, the results of the State of Rust survey help reveal the areas that need improvement in many areas across the Rust Project and ecosystem, as well as the aspects that are working well for our community.

If you have any suggestions for the Rust Annual survey, please [let us know](https://github.com/rust-lang/surveys/issues)!

We are immensely grateful to those who participated in the 2024 State of Rust Survey and facilitated its creation. While there are always challenges associated with developing and maintaining a programming language, this year we were pleased to see a high level of survey participation and candid feedback that will truly help us make Rust work better for everyone.

If you’d like to dig into more details, we recommend you to browse through the full [survey report][report].

[report]: https://raw.githubusercontent.com/rust-lang/surveys/main/surveys/2024-annual-survey/report/annual-survey-2024-report.pdf

<!-- Chart scripts -->

<script charset="utf-8" src="../../../scripts/plotly-basic-2.29.0.min.js"></script>

<script src="../../../scripts/2025-01-rust-survey-2024/charts.js"></script>
