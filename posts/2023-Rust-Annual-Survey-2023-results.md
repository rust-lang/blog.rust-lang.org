+++
layout = "post"
date = 2024-02-19
title = "2023 Annual Rust Survey Results"
author = "The Rust Survey Team"
+++

Hello, Rustaceans!

The Rust Survey Team is excited to share the results of our [2023 survey on the Rust Programming language](https://blog.rust-lang.org/2023/12/18/survey-launch.html), conducted between December 18, 2023 and January 15, 2024.
As in previous years, the 2023 State of Rust Survey was focused on gathering insights and feedback from Rust users, and all those who are interested in the future of Rust more generally.

This eighth edition of the survey surfaced new insights and learning opportunities straight from the global Rust language community, which we will summarize below. In addition to this blog post, this year we have also prepared a [report][report] containing charts with aggregated results of all questions in the survey. Based on feedback from recent years, we have also tried to provide more comprehensive and interactive charts in this summary blog post. Let us know what you think!

**Our sincerest thanks to every community member who took the time to express their opinions and experiences with Rust over the past year. Your participation will help us make Rust better for everyone.**

There's a lot of data to go through, so strap in and enjoy!

## Participation

| **Survey** | **Started** | **Completed** | **Completion rate** | **Views** |
|:----------:|------------:|--------------:|--------------------:|----------:|
|    2022    |      11 482 |         9 433 |               81.3% |    25 581 |
|    2023    |      11 950 |         9 710 |               82.2% |    16 028 |

As shown above, in 2023, we have received 37% fewer survey views in vs 2022, but saw a slight uptick in starts and completions. There are many reasons why this could have been the case, but it’s possible that because we released the [2022 analysis blog](https://blog.rust-lang.org/2023/08/07/Rust-Survey-2023-Results.html) so late last year, the survey was fresh in many Rustaceans’ minds. This might have prompted fewer people to feel the need to open the most recent survey. Therefore, we find it doubly impressive that there were more starts and completions in 2023, despite the lower overall view count.

## Community

This year, we have relied on automated translations of the survey, and we have asked volunteers to review them. We thank the hardworking volunteers who reviewed these automated survey translations, ultimately allowing us to offer the survey in seven languages: English, Simplified Chinese, French, German, Japanese, Russian, and Spanish. We decided not to publish the survey in languages without a translation review volunteer, meaning we could not issue the survey in Portuguese, Ukrainian, Traditional Chinese, or Korean.

The Rust Survey team understands that there were some issues with several of these translated versions, and we apologize for any difficulty this has caused. We are always looking for ways to improve going forward and are in the process of discussing improvements to this part of the survey creation process for next year.

We saw a 3pp increase in respondents taking this year’s survey in English – 80% in 2023 and 77% in 2022. Across all other languages, we saw only minor variations – all of which are likely due to us offering fewer languages overall this year due to having fewer volunteers.

Rust user respondents were asked which country they live in. The top 10 countries represented were, in order: United States (22%), Germany (12%), China (6%), United Kingdom (6%), France (6%), Canada (3%), Russia (3%), Netherlands (3%), Japan (3%), and Poland (3%) . We were interested to see a small reduction in participants taking the survey in the United States in 2023 (down 3pp from the 2022 edition) which is a positive indication of the growing global nature of our community! You can try to find your country in the chart below:

<!-- Chart where-do-you-live start -->
<div>
    <div class="pie-chart" id="where-do-you-live" style="height:600px; width:100%;"><noscript>
<img alt="where-do-you-live" height="600" src="../../../images/2024-02-rust-survey-2023/where-do-you-live.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/where-do-you-live.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/where-do-you-live.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart where-do-you-live end -->

Once again, the majority of our respondents reported being most comfortable communicating on technical topics in English at 92.7% — a slight difference from 93% in 2022. Again, Chinese was the second-highest choice for preferred language for technical communication at 6.1% (7% in 2022).

<!-- Chart what-are-your-preferred-languages-for-technical-communication start -->
<div>
    <div class="bar-chart" id="what-are-your-preferred-languages-for-technical-communication" style="height:400px; width:100%;"><noscript>
<img alt="what-are-your-preferred-languages-for-technical-communication" height="400" src="../../../images/2024-02-rust-survey-2023/what-are-your-preferred-languages-for-technical-communication.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/what-are-your-preferred-languages-for-technical-communication.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/what-are-your-preferred-languages-for-technical-communication.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart what-are-your-preferred-languages-for-technical-communication end -->

We also asked whether respondents consider themselves members of a marginalized community. Out of those who answered, 76% selected no, 14% selected yes, and 10% preferred not to say.

We have asked the group that selected “yes” which specific groups they identified as being a member of. The majority of those who consider themselves a member of an underrepresented or marginalized group in technology identify as lesbian, gay, bisexual, or otherwise non-heterosexual. The second most selected option was neurodivergent at 41% followed by trans at 31.4%. Going forward, it will be important for us to track these figures over time to learn how our community changes and to identify the gaps we need to fill.

<!-- Chart which-marginalized-group start -->
<div>
    <div class="bar-chart" id="which-marginalized-group" style="height:500px; width:100%;"><noscript>
<img alt="which-marginalized-group" height="500" src="../../../images/2024-02-rust-survey-2023/which-marginalized-group.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-marginalized-group.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-marginalized-group.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart which-marginalized-group end -->

As Rust continues to grow, we must acknowledge the diversity, equity, and inclusivity (DEI)-related gaps that exist in the Rust community. Sadly, Rust is not unique in this regard. For instance, only 20% of 2023 respondents to this representation question consider themselves a member of a racial or ethnic minority and only 26% identify as a woman. We would like to see more equitable figures in these and other categories. In 2023, the Rust Foundation formed a diversity, equity, and inclusion subcommittee on its Board of Directors whose members are aware of these results and are actively discussing ways that the Foundation might be able to better support underrepresented groups in Rust and help make our ecosystem more globally inclusive. One of the central goals of the Rust Foundation board's subcommittee is to analyze information about our community to find out what gaps exist, so this information is a helpful place to start. This topic deserves much more depth than is possible here, but readers can expect more on the subject in the future.

## Rust usage

In 2023, we saw a slight jump in the number of respondents that self-identify as a Rust user, from 91% in 2022 to 93% in 2023.

<!-- Chart do-you-use-rust start -->
<div>
    <div class="bar-chart" id="do-you-use-rust" style="height:300px; width:100%;"><noscript>
<img alt="do-you-use-rust" height="300" src="../../../images/2024-02-rust-survey-2023/do-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/do-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/do-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart do-you-use-rust end -->

Of those who used Rust in 2023, 49% did so on a daily (or nearly daily) basis — a small increase of 2pp from the previous year.

<!-- Chart how-often-do-you-use-rust start -->
<div>
    <div class="bar-chart" id="how-often-do-you-use-rust" style="height:300px; width:100%;"><noscript>
<img alt="how-often-do-you-use-rust" height="300" src="../../../images/2024-02-rust-survey-2023/how-often-do-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/how-often-do-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/how-often-do-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-often-do-you-use-rust end -->

31% of those who did not identify as Rust users cited the perception of difficulty as the primary reason for not having used it, with 67% reporting that they simply haven’t had the chance to prioritize learning Rust yet, which was once again the most common reason.

<!-- Chart why-dont-you-use-rust start -->
<div>
    <div class="bar-chart" id="why-dont-you-use-rust" style="height:500px; width:100%;"><noscript>
<img alt="why-dont-you-use-rust" height="500" src="../../../images/2024-02-rust-survey-2023/why-dont-you-use-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/why-dont-you-use-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/why-dont-you-use-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/why-dont-you-use-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart why-dont-you-use-rust end -->

Of the former Rust users who participated in the 2023 survey, 46% cited factors outside their control (a decrease of 1pp from 2022), 31% stopped using Rust due to preferring another language (an increase of 9pp from 2022), and 24% cited difficulty as the primary reason for giving up (a decrease of 6pp from 2022).

<!-- Chart why-did-you-stop-using-rust start -->
<div>
    <div class="bar-chart" id="why-did-you-stop-using-rust" style="height:500px; width:100%;"><noscript>
<img alt="why-did-you-stop-using-rust" height="500" src="../../../images/2024-02-rust-survey-2023/why-did-you-stop-using-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/why-did-you-stop-using-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/why-did-you-stop-using-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/why-did-you-stop-using-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart why-did-you-stop-using-rust end -->

Rust expertise has generally increased amongst our respondents over the past year! 23% can write (only) simple programs in Rust (a decrease of 6pp from 2022), 28% can write production-ready code (an increase of 1pp), and 47% consider themselves productive using Rust — up from 42% in 2022. While the survey is just one tool to measure the changes in Rust expertise overall, these numbers are heartening as they represent knowledge growth for many Rustaceans returning to the survey year over year.

<!-- Chart how-would-you-rate-your-rust-expertise start -->
<div>
    <div class="bar-chart" id="how-would-you-rate-your-rust-expertise" style="height:500px; width:100%;"><noscript>
<img alt="how-would-you-rate-your-rust-expertise" height="500" src="../../../images/2024-02-rust-survey-2023/how-would-you-rate-your-rust-expertise.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/how-would-you-rate-your-rust-expertise.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/how-would-you-rate-your-rust-expertise.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-would-you-rate-your-rust-expertise end -->

In terms of operating systems used by Rustaceans, the situation is very similar to the results from 2022, with Linux being the most popular choice of Rust users, followed by macOS and Windows, which have a very similar share of usage.

<!-- Chart which-os-do-you-use start -->
<div>
    <div class="bar-chart" id="which-os-do-you-use" style="height:400px; width:100%;"><noscript>
<img alt="which-os-do-you-use" height="400" src="../../../images/2024-02-rust-survey-2023/which-os-do-you-use.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-use.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-use.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-use-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-os-do-you-use end -->

Rust programmers target a diverse set of platforms with their Rust programs, even though the most popular target by far is still a Linux machine. We can see a slight uptick in users targeting WebAssembly, embedded and mobile platforms, which speaks to the versatility of Rust. 

<!-- Chart which-os-do-you-target start -->
<div>
    <div class="bar-chart" id="which-os-do-you-target" style="height:500px; width:100%;"><noscript>
<img alt="which-os-do-you-target" height="500" src="../../../images/2024-02-rust-survey-2023/which-os-do-you-target.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-target.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-target.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-os-do-you-target-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-os-do-you-target end -->

We cannot of course forget the favourite topic of many programmers: which IDE (developer environment) do they use. Visual Studio Code still seems to be the most popular option, with RustRover (which was released last year) also gaining some traction.

<!-- Chart what-ide-do-you-use start -->
<div>
    <div class="bar-chart" id="what-ide-do-you-use" style="height:500px; width:100%;"><noscript>
<img alt="what-ide-do-you-use" height="500" src="../../../images/2024-02-rust-survey-2023/what-ide-do-you-use.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/what-ide-do-you-use.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/what-ide-do-you-use.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/what-ide-do-you-use-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart what-ide-do-you-use end -->

> You can also take a look at the linked [wordcloud](../../../images/2024-02-rust-survey-2023/what-ide-do-you-use-wordcloud.png) that summarizes open answers to this question (the "Other" category), to see what other editors are also popular.

## Rust at Work

We were excited to see a continued upward year-over-year trend of Rust usage at work. 34% of 2023 survey respondents use Rust in the majority of their coding at work — an increase of 5pp from 2022. Of this group, 39% work for organizations that make non-trivial use of Rust.

<!-- Chart do-you-personally-use-rust-at-work start -->
<div>
    <div class="bar-chart" id="do-you-personally-use-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="do-you-personally-use-rust-at-work" height="500" src="../../../images/2024-02-rust-survey-2023/do-you-personally-use-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/do-you-personally-use-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/do-you-personally-use-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart do-you-personally-use-rust-at-work end -->

<!-- how-is-rust-used-at-your-organization -->

Once again, the top reason employers of our survey respondents invested in Rust was the ability to build relatively correct and bug-free software at 86% — a 4pp increase from 2022 responses. The second most popular reason was Rust’s performance characteristics at 83%.

<!-- Chart why-you-use-rust-at-work start -->
<div>
    <div class="bar-chart" id="why-you-use-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="why-you-use-rust-at-work" height="500" src="../../../images/2024-02-rust-survey-2023/why-you-use-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/why-you-use-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/why-you-use-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart why-you-use-rust-at-work end -->

We were also pleased to see an increase in the number of people who reported that Rust helped their company achieve its goals at 79% — an increase of 7pp from 2022. 77% of respondents reported that their organization is likely to use Rust again in the future — an increase of 3pp from the previous year. Interestingly, we saw a decrease in the number of people who reported that using Rust has been challenging for their organization to use: 34% in 2023 and 39% in 2022. We also saw an increase of respondents reporting that Rust has been worth the cost of adoption: 64% in 2023 and 60% in 2022.

<!-- Chart which-statements-apply-to-rust-at-work start -->
<div>
    <div class="bar-chart" id="which-statements-apply-to-rust-at-work" style="height:500px; width:100%;"><noscript>
<img alt="which-statements-apply-to-rust-at-work" height="500" src="../../../images/2024-02-rust-survey-2023/which-statements-apply-to-rust-at-work.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-statements-apply-to-rust-at-work.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-statements-apply-to-rust-at-work.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart which-statements-apply-to-rust-at-work end -->

There are many factors playing into this, but the growing awareness around Rust has likely resulted in the proliferation of resources, allowing new teams using Rust to be better supported.

In terms of technology domains, it seems that Rust is especially popular for creating server backends, web and networking services and cloud technologies.

<!-- Chart technology-domain start -->
<div>
    <div class="bar-chart" id="technology-domain" style="height:600px; width:100%;"><noscript>
<img alt="technology-domain" height="600" src="../../../images/2024-02-rust-survey-2023/technology-domain.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/technology-domain.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/technology-domain.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/technology-domain-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart technology-domain end -->

> You can scroll the chart to the right to see more domains. Note that the Database implementation and Computer Games domains were not offered as closed answers in the 2022 survey (they were merely submitted as open answers), which explains the large jump.

It is exciting to see the continued growth of professional Rust usage and the confidence so many users feel in its performance, control, security and safety, enjoyability, and more!

## Challenges

As always, one of the main goals of the State of Rust survey is to shed light on challenges, concerns, and priorities on Rustaceans’ minds over the past year.

Of those respondents who shared their main worries for the future of Rust (9,374), the majority were concerned about Rust becoming too complex at 43% — a 5pp increase from 2022. 42% of respondents were concerned about a low level of Rust usage in the tech industry. 32% of respondents in 2023 were most concerned about Rust developers and maintainers not being properly supported — a 6pp increase from 2022.

We saw a notable decrease in respondents who were not at all concerned about the future of Rust, 18% in 2023 and 30% in 2022.

Thank you to all participants for your candid feedback which will go a long way toward improving Rust for everyone.

<!-- Chart what-are-your-biggest-worries-about-rust start -->
<div>
    <div class="bar-chart" id="what-are-your-biggest-worries-about-rust" style="height:500px; width:100%;"><noscript>
<img alt="what-are-your-biggest-worries-about-rust" height="500" src="../../../images/2024-02-rust-survey-2023/what-are-your-biggest-worries-about-rust.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/what-are-your-biggest-worries-about-rust.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/what-are-your-biggest-worries-about-rust.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/what-are-your-biggest-worries-about-rust-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart what-are-your-biggest-worries-about-rust end -->

> Closed answers marked with N/A were not present in the previous (2022) version of the survey.

In terms of features that Rust users want to be implemented, stabilized or improved, the most desired improvements are in the areas of traits (trait aliases, associated type defaults, etc.), const execution (generic const expressions, const trait methods, etc.) and async (async closures, coroutines).

<!-- Chart which-features-do-you-want-stabilized start -->
<div>
    <div class="matrix-chart" id="which-features-do-you-want-stabilized" style="height:600px; width:100%;"><noscript>
<img alt="which-features-do-you-want-stabilized" height="600" src="../../../images/2024-02-rust-survey-2023/which-features-do-you-want-stabilized.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-features-do-you-want-stabilized.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-features-do-you-want-stabilized.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-features-do-you-want-stabilized-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-features-do-you-want-stabilized end -->

It is interesting that 20% of respondents answered that they wish Rust to slow down the development of new features, which likely goes hand in hand with the previously mentioned worry that Rust becomes too complex.

The areas of Rust that Rustaceans seem to struggle with the most seem to be asynchronous Rust, the traits and generics system and also the borrow checker.

<!-- Chart which-problems-do-you-remember-encountering start -->
<div>
    <div class="bar-chart" id="which-problems-do-you-remember-encountering" style="height:400px; width:100%;"><noscript>
<img alt="which-problems-do-you-remember-encountering" height="400" src="../../../images/2024-02-rust-survey-2023/which-problems-do-you-remember-encountering.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/which-problems-do-you-remember-encountering.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-problems-do-you-remember-encountering.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/which-problems-do-you-remember-encountering-wordcloud.png" target="_href_" title="Download open answers as wordcloud PNG">Wordcloud of open answers</a>]</span>
    </div>
</div>
<!-- Chart which-problems-do-you-remember-encountering end -->

Respondents of the survey want the Rust maintainers to mainly prioritize fixing compiler bugs (68%), improving the runtime performance of Rust programs (57%) and also improving compile times (45%).

<!-- Chart how-should-work-be-prioritized start -->
<div>
    <div class="matrix-chart" id="how-should-work-be-prioritized" style="height:800px; width:100%;"><noscript>
<img alt="how-should-work-be-prioritized" height="800" src="../../../images/2024-02-rust-survey-2023/how-should-work-be-prioritized.png"/>
</noscript></div>
    <div style="display: flex; margin-bottom: 10px;">
        <span>[<a href="../../../images/2024-02-rust-survey-2023/how-should-work-be-prioritized.png" target="_href_" title="Download chart as PNG">PNG</a>]</span>&nbsp;<span>[<a href="../../../images/2024-02-rust-survey-2023/how-should-work-be-prioritized.svg" target="_href_" title="Download chart as SVG">SVG</a>]</span>
    </div>
</div>
<!-- Chart how-should-work-be-prioritized end -->

Same as in recent years, respondents noted that compilation time is one of the most important areas that should be improved. However, it is interesting to note that respondents also seem to consider runtime performance to be more important than compile times.

## Looking ahead

Each year, the results of the State of Rust survey help reveal the areas that need improvement in many areas across the Rust Project and ecosystem, as well as the aspects that are working well for our community.

We are aware that the survey has contained some confusing questions, and we will try to improve upon that in the next year's survey.
If you have any suggestions for the Rust Annual survey, please [let us know](https://github.com/rust-lang/surveys/issues)!

We are immensely grateful to those who participated in the 2023 State of Rust Survey and facilitated its creation. While there are always challenges associated with developing and maintaining a programming language, this year we were pleased to see a high level of survey participation and candid feedback that will truly help us make Rust work better for everyone.

If you’d like to dig into more details, we recommend you to browse through the full [survey report][report].

[report]: https://raw.githubusercontent.com/rust-lang/surveys/main/surveys/2023-annual-survey/report/annual-survey-2023-report.pdf

<!-- Chart scripts -->

<script charset="utf-8" src="../../../scripts/plotly-basic-2.29.0.min.js"></script>

<script src="../../../scripts/2024-02-rust-survey-2023/charts.js"></script>
