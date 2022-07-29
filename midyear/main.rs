#![feature(iter_intersperse)]

use enum_index::EnumIndex;
use enum_index_derive::EnumIndex;

trait TableHeader {
    const NAME: &'static str;
    const SPAN: usize;
    const STYLE: &'static str;
    fn text(&self) -> &'static str;
}

fn main() {
    #[derive(Copy, Clone, Debug, EnumIndex)]
    enum AnyProgress { Yes, No }
    impl TableHeader for AnyProgress {
        const NAME: &'static str = "any-progress?";
        const SPAN: usize = 2;
        const STYLE: &'static str = "color:darkgreen";
        fn text(&self) -> &'static str { match *self { AnyProgress::Yes => "y", AnyProgress::No => "n" } }
    }
    #[derive(Copy, Clone, Debug, EnumIndex)]
    enum HowBig { Complete, DoneNextSixMo, NotNextSixMo, AdHocProgress, NoGoalsNoProgress, Other(&'static str) }
    impl TableHeader for HowBig {
        const NAME: &'static str = "problem-size?";
        const SPAN: usize = 6;
        const STYLE: &'static str = "color:darkorange";
        fn text(&self) -> &'static str {
            match *self {
                HowBig::Complete => "done",
                HowBig::DoneNextSixMo => "<=6mo",
                HowBig::NotNextSixMo => ">6mo",
                HowBig::AdHocProgress => "ad-hoc",
                HowBig::NoGoalsNoProgress => "who-knows?",
                HowBig::Other(_) => "other",
            }
        }
    }
    #[derive(Copy, Clone, Debug, EnumIndex)]
    enum Resolve { NowSolved, NextSixMo, NextTwoYears, Solveable, NeverEnding, Other(&'static str) }
    impl TableHeader for Resolve {
        const NAME: &'static str = "resolved-when?";
        const SPAN: usize = 6;
        const STYLE: &'static str = "color:maroon";
        fn text(&self) -> &'static str {
            match *self {
                Resolve::NowSolved => "done",
                Resolve::NextSixMo => "<=6mo",
                Resolve::NextTwoYears => "<=2yr",
                Resolve::Solveable => ">2yr",
                Resolve::NeverEnding => "&infin;",
                Resolve::Other(_) => "other",
            }
        }
    }
    #[derive(Copy, Clone, Debug, EnumIndex)]
    enum HowItStarted { UnkProblem, NoGoal, NoPlan, NoMilestones, NoAccomplishments,
                        SomeAccomplishments, PlanNeededRevision, NeedsPolish, Other(&'static str) }
    impl TableHeader for HowItStarted {
        const NAME: &'static str = "how-it-started?";
        const SPAN: usize = 9;
        const STYLE: &'static str = "color:purple";

        fn text(&self) -> &'static str {
            match *self {
                HowItStarted::UnkProblem => "problem-was-unknown",
                HowItStarted::NoGoal => "had-no-goals",
                HowItStarted::NoPlan => "had-no-plan",
                HowItStarted::NoMilestones => "milestones-unestablished",
                HowItStarted::NoAccomplishments => "no-accomplishments",
                HowItStarted::SomeAccomplishments => "some-accomplishments",
                HowItStarted::PlanNeededRevision => "wrong-plan",
                HowItStarted::NeedsPolish => "just-needed-polish",
                HowItStarted::Other(_) => "other",
            }
        }
    }
    #[derive(Copy, Clone, Debug, EnumIndex)]
    enum HowItsGoing { HelpWhatHappened, HelpWhatsNext, BetterUnderstanding, HighLevelPlan,
                       Milestones, Contributors, Schedule, CompletedMilestones, ImplementedSolution,
                       NeedUserFeedback, GettingUserFeedback, Extra(&'static str) }
    impl TableHeader for HowItsGoing {
        const NAME: &'static str = "hows-it-going?";
        const SPAN: usize = 11;
        const STYLE: &'static str = "color:darkblue";

        fn text(&self) -> &'static str {
            match *self {
                HowItsGoing::HelpWhatHappened => "what-happun",
                HowItsGoing::HelpWhatsNext => "whats-next?",
                HowItsGoing::BetterUnderstanding => "improved-understanding",
                HowItsGoing::HighLevelPlan => "now-have-plan",
                HowItsGoing::Milestones => "now-have-milestones",
                HowItsGoing::Contributors => "have-contributors",
                HowItsGoing::Schedule => "have-schedule",
                HowItsGoing::CompletedMilestones => "completed-milestones",
                HowItsGoing::ImplementedSolution => "implemented-solution",
                HowItsGoing::NeedUserFeedback => "need-user-feedback",
                HowItsGoing::GettingUserFeedback => "getting-feedback",
                HowItsGoing::Extra(_) => "additional-info",
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct Entry<'a> {
        name: &'a str,
        url: &'a str,
        inline_url: &'a str,
        any_progress: AnyProgress,
        problem_feels: HowBig,
        solution_feels: &'a [Resolve],
        how_it_started: &'a [HowItStarted],
        how_its_going: &'a [HowItsGoing],
    }

    type Input<'a> = (&'a str, &'a str, &'a str, AnyProgress, HowBig, &'a [Resolve], &'a [HowItStarted], &'a [HowItsGoing]);
    impl<'a> From<&Input<'a>> for Entry<'a> {
        fn from(x: &Input<'a>) -> Self {
            Entry {
                name: x.0,
                url: x.1,
                inline_url: x.2,
                any_progress: x.3,
                problem_feels: x.4,
                solution_feels: x.5,
                how_it_started: x.6,
                how_its_going: x.7,
            }
        }
    }

    use HowItStarted::*;
    use HowItsGoing::*;

    let data: &[Input] =
        &[("async traits",
           "https://hackmd.io/@rust-compiler-team/H1eW9oXdc",
           "#async-traits",
           AnyProgress::Yes,
           HowBig::Other("We had a loosely defined goal of shipping async fn in traits this year. I think we will at least have an implementation landed in nightly of static, and possibly dyn, async fn in traits by the end of the year. Stabilization timeline is unclear."),
           &[Resolve::NextSixMo, Resolve::Other("Major design questions should be resolved or in “resolved to experiment” mode by the end of the year. Nightly-only implementation in the next six months, with part of it possibly moving toward stabilization. Should all be stable in two years or less.")],
           &[NoPlan],
           &[HighLevelPlan, Milestones, CompletedMilestones]),

          ("diagnostics improvements",
           "https://hackmd.io/@rust-compiler-team/HJ_cgnXd5",
           "#diagnostics-aspirations-",
           AnyProgress::Yes,
           HowBig::Other("we didn’t have “planned goals” for the year, but we didn accomplish lots of incremental improvements and started the translation infrastructure which has been in our wishlist for years"),
           &[Resolve::NeverEnding, Resolve::Other("there is specific feature and integration work that can be tackled in bounded amount of times, but the improvement of diagnostics is a fractal one, and every time we raise the bar, the expectation of our users increases and our increased experience makes us attempt bolder things.")],
           &[NoGoal, NoMilestones],
           &[BetterUnderstanding, HighLevelPlan, Milestones, Contributors, NeedUserFeedback, GettingUserFeedback]),

          ("safe transmute", // name
           "https://hackmd.io/@rust-compiler-team/HJ7Y3s7uq", // url
           "#safe-transmute",
           AnyProgress::Yes,
           HowBig::DoneNextSixMo,
           &[Resolve::Other("we think the most important parts of the problem are now (or very soon to be) solved, and additional parts of the solution will be available in the next six months")],
           &[SomeAccomplishments, PlanNeededRevision],
           &[BetterUnderstanding, HighLevelPlan, Milestones, Contributors, CompletedMilestones, ImplementedSolution]),

          ("chalk", // name
           "https://hackmd.io/@rust-compiler-team/HyVh2sQdq", // url
           "#chalk",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::NextTwoYears],
           &[NoGoal],
           &[BetterUnderstanding, HighLevelPlan, Contributors, CompletedMilestones]),

          ("Generic Associated Types", // name
           "https://hackmd.io/@rust-compiler-team/BkM83iX_9", // url
           "#generic-associated-types",
           AnyProgress::Yes,
           HowBig::DoneNextSixMo,
           &[Resolve::NextSixMo],
           &[SomeAccomplishments],
           &[CompletedMilestones, GettingUserFeedback]),

          ("Performance Dashboard", // name
           "https://hackmd.io/@rust-compiler-team/SyaDJ2X_5",  // url
           "#performance-dashboard",
           AnyProgress::Yes,
           HowBig::Other("We had no concrete goals for the dashboard, and representing performance is still a hard problem. I think we are unlikely to achieve any significant milestones in the next 6 months, largely due to lack of clear ideas for solid improvements or bandwidth to experiment."),
           &[Resolve::NextSixMo, Resolve::Solveable],
           &[NoGoal],
           &[HelpWhatsNext, Extra("I think we would like help is not wrong, but it’s also key to note that the largest problem in this domain is room and bandwidth for experimentation.")]),

          // QUEUE

          ("intrinsic MIR fallbacks", // name
           "https://hackmd.io/@rust-compiler-team/HylMg2m_9", // url
           "#ease-writing-new-backends-via-intrinsic-mir-fallbacks",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::NextTwoYears],
           &[NoPlan],
           &[HelpWhatsNext, Extra("it would be wonderful to get mentoring instructions (even just a sketch of them) so that non-experts could help make progress on this")]),

          ("P-high backlog processing", // name
           "https://hackmd.io/@rust-compiler-team/SkS1psm_c", // url
           "#p-high-backlog-processing-aspirations-",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::NextTwoYears],
           &[NoGoal],
           &[BetterUnderstanding, HighLevelPlan, Milestones, CompletedMilestones]),

          ("better integration with trace-based debuggers", // name
           "https://hackmd.io/@rust-compiler-team/BkJSsi7d5", // url
           "#better-integration-with-trace-based-debuggers",
           AnyProgress::No,
           HowBig::NotNextSixMo,
           &[Resolve::NextTwoYears],
           &[NoPlan],
           &[HelpWhatsNext]),

          ("MCVE reduction tooling", // name
           "https://hackmd.io/@rust-compiler-team/r19rJhmu5", // url
           "#mcve-reduction-tooling",
           AnyProgress::No,
           HowBig::NoGoalsNoProgress,
           &[Resolve::NextTwoYears],
           &[NoGoal],
           &[HelpWhatsNext]),

          ("Incremental Compilation", // name
           "https://hackmd.io/@rust-compiler-team/B1D7CiX_c", // url
           "#incremental-compilation-aspirations",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::NextTwoYears],
           &[NoGoal],
           &[HelpWhatsNext, Contributors]),

          ("wg-debugging", // name
           "https://hackmd.io/@rust-compiler-team/HJOtiiQOq", // url
           "#wg-debugging",
           AnyProgress::Yes,
           HowBig::DoneNextSixMo,
           &[Resolve::NextTwoYears, Resolve::Other("(In general, there is always more we can do here but wesleywiser thinks we’ll have made significant, noticable progress within the next two years and probably even within this year)")],
           &[NoGoal],
           &[HelpWhatsNext, BetterUnderstanding, Contributors, CompletedMilestones, NeedUserFeedback, GettingUserFeedback]),

          ("Debugging Aspirations", // name
           "https://hackmd.io/@rust-compiler-team/SyM2poXu9", // url
           "#debugging-aspirations-",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::NextTwoYears],
           &[NoPlan],
           &[BetterUnderstanding, CompletedMilestones, ImplementedSolution]),

          ("improving Rust's debuginfo quality", // name
           "https://hackmd.io/@rust-compiler-team/HJQ25sXOq", // url
           "#improving-debuginfo-quality",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::Other("There’s nearly an unbounded amount of effort that could be spent improving debuginfo quality but I think we are making significant improvement both over the last 6 months and in the final 6 months of this year as well.")],
           &[NoGoal],
           &[HelpWhatsNext, BetterUnderstanding, Contributors, NeedUserFeedback, GettingUserFeedback, Extra("Much of the work wesleywiser is aware of has landed in 1.60 or 1.61 but there are a few small pieces landing in 1.62 (current beta)")]),

          ("GCC backend", // name
           "https://hackmd.io/@rust-compiler-team/H1svghmu9", // url
           "#gcc-backend",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::Other("I believe we’ll be able to ship cg_gcc with rustup within 6 months. Implementing the missing Rust features will take longer.")],
           &[SomeAccomplishments],
           &[HighLevelPlan, CompletedMilestones]),

          ("I-unsound issues", // name
           "https://hackmd.io/@rust-compiler-team/r1Abdj7uq", // url
           "#i-unsound-issues-",
           AnyProgress::Yes,
           HowBig::NotNextSixMo,
           &[Resolve::NextSixMo],
           &[NoGoal],
           &[Milestones, Contributors, CompletedMilestones]),

          ("const-generics and const-eval", // name
           "https://hackmd.io/@rust-compiler-team/HktiComdq", // url
           "#const-generics-and-const-eval",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::Solveable],
           &[NoGoal],
           &[BetterUnderstanding, Contributors]),

          ("async crashdump dissection", // name
           "https://hackmd.io/@rust-compiler-team/SJYL9iQ_9", // url
           "#async-crashdump-dissection",
           AnyProgress::Yes,
           HowBig::NotNextSixMo,
           &[Resolve::NextTwoYears],
           &[NoMilestones],
           &[BetterUnderstanding, CompletedMilestones, NeedUserFeedback]),

          ("Faster Builds", // name
           "https://hackmd.io/@rust-compiler-team/B1O2siXd9", // url
           "#faster-builds-initiatives--%EF%B8%8F",
           AnyProgress::Yes,
           HowBig::DoneNextSixMo,
           &[Resolve::NeverEnding],
           &[NoPlan],
           &[BetterUnderstanding, HighLevelPlan, Milestones, Contributors, CompletedMilestones, NeedUserFeedback]),

          ("MIR tooling", // name
           "https://hackmd.io/@rust-compiler-team/rJngk3mO5", // url
           "#mir-tooling-stable-mir-and-ghost-code",
           AnyProgress::Yes,
           HowBig::NotNextSixMo,
           &[Resolve::NextTwoYears],
           &[NoGoal],
           &[BetterUnderstanding, Milestones, Contributors]),

          ("Cranelift", // name
           "https://hackmd.io/@rust-compiler-team/BJLre2Xu9", // url
           "#cranelift",
           AnyProgress::Yes,
           HowBig::AdHocProgress,
           &[Resolve::Other("In terms of getting cg_clif distributed with rustup I think 6 months is feasible. In terms of implementing all missing rust features I think it will take longer.")],
           &[SomeAccomplishments],
           &[HighLevelPlan, CompletedMilestones]),

          ("supporting split debuginfo", // name
           "https://hackmd.io/@rust-compiler-team/ByXfjiXu5", // url
           "#supporting-split-debuginfo",
           AnyProgress::No,
           HowBig::DoneNextSixMo,
           &[Resolve::NextSixMo],
           &[NeedsPolish],
           &[ImplementedSolution]),

          /* // TEMPLATE
          ("", // name
           "", // url
           AnyProgress::{Yes, No},
           HowBig::{Complete, DoneNextSixMo, NotNextSixMo, AdHoc, NoGoalsNoProgress, Other("") },
           &[Resolve::{NowSolved, NextSixMo, NextTwoYears, Solveable, NeverEnding, Other("")}],
           &[UnkProblem, NoGoal, NoPlan, NoMilestones, NoAccomplishments, SomeAccomplishments, PlanNeededRevision, NeedsPolish, Other("")],
           &[HelpWhatHappened, HelpWhatsNext, BetterUnderstanding, HighLevelPlan, Milestones, Contributors, Schedule, CompletedMilestones, ImplementedSolution, NeedUserFeedback, GettingUserFeedback]),
             */
        ];
    let data: Vec<Entry> = data.into_iter().map(|x|x.into()).collect();
    println!("Hello, world! {:#?}", data);

    let entry_rows: Vec<Vec<String>> = data.iter().map(|e|e.as_td_cells()).collect();
    let mut entry_cols: Vec<(Option<HeaderStart>, Vec<String>)> = Vec::new();
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct HeaderStart;
    fn row_header<T: TableHeader>(header_start: bool, entry_cols: &mut Vec<(Option<HeaderStart>, Vec<String>)>) {
        let th = format!("<th style={ST} rowspan={SP}>{N}</th>", ST=T::STYLE, SP=T::SPAN, N=T::NAME);
        let th_len = th.len();
        entry_cols.push((if header_start { Some(HeaderStart) } else { None }, vec![th]));
        for _ in 0..T::SPAN-1 { entry_cols.push((None, vec![iota_s(" ", th_len)])); }
    }
    row_header::<AnyProgress>(true, &mut entry_cols);
    row_header::<HowBig>(false, &mut entry_cols);
    row_header::<Resolve>(false, &mut entry_cols);
    row_header::<HowItStarted>(true, &mut entry_cols);
    row_header::<HowItsGoing>(false, &mut entry_cols);

    for (i, row,) in entry_rows.into_iter().enumerate() {
        for (j, item) in row.into_iter().enumerate() {
            assert!(i <= entry_cols[j].1.len());
            assert!(entry_cols[j].1.len() <= i+1);
            entry_cols[j].1.push(item);
        }
    }

    let emit_entry_names = || {
        let header: String = data.iter().map(|e|format!("<th><a href=\"{}\">{}</a></th>", e.inline_url, e.name)).collect();
        println!("<tr><th></th>{}</tr>", header);
    };
    for (header_start, e) in entry_cols {
        if header_start.is_some() { emit_entry_names(); }
        let row: String = e.into_iter().collect();
        println!("<tr>{}</tr>", row);
    }

    fn cells(data: &[(&str, usize)], index: usize, fill: usize, style: &str) -> Vec<String> {
        multicell(data, &[index], fill, style)
    }

    use std::iter::repeat;

    fn iota<T: Clone>(s: T, n: usize) -> Vec<T> {
        repeat(s).take(n).collect()
    }

    fn iota_s(s: &str, n: usize) -> String {
        repeat(s).take(n).collect()
    }

    fn multicell(data: &[(&str, usize)], indexes: &[usize], fill: usize, style: &str) -> Vec<String> {
        data.iter()
            .enumerate()
            .map(|(j, (data, len))| {
                if indexes.contains(&j) {
                    format!("<td style={}>{}{}</td>", style, data, iota_s(" ", fill-data.len()))
                } else {
                    format!("<td style={}>{}</td>", style, iota_s(" ", fill))
                }
            })
            .collect()
    }

    impl<'a> Entry<'a> {
        fn as_td_cells(&self) -> Vec<String> {
            let template = [AnyProgress::Yes, AnyProgress::No];
            let template = template.into_iter().map(|e|e.text()).collect::<Vec<_>>();
            let fill = template.iter().map(|x|x.len()).max().unwrap();
            let template: Vec<(&'static str, usize)> = template.into_iter().map(|s| (s, s.len())).collect();
            let idx = self.any_progress.enum_index();
            let any_progress: Vec<String> = cells(&template[..], idx, fill, AnyProgress::STYLE);

            let template = [HowBig::Complete, HowBig::DoneNextSixMo, HowBig::NotNextSixMo, HowBig::AdHocProgress, HowBig::NoGoalsNoProgress, HowBig::Other("")];
            let template = template.into_iter().map(|e|e.text()).collect::<Vec<_>>();
            let fill = template.iter().map(|x|x.len()).max().unwrap();
            let template: Vec<(&'static str, usize)> = template.into_iter().map(|s| (s, s.len())).collect();
            let idx = self.problem_feels.enum_index();
            let how_big: Vec<String> = cells(&template[..], idx, fill, HowBig::STYLE);

            let template = [Resolve::NowSolved, Resolve::NextSixMo, Resolve::NextTwoYears, Resolve::Solveable, Resolve::NeverEnding, Resolve::Other("")];
            let template = template.into_iter().map(|e|e.text()).collect::<Vec<_>>();
            let fill = template.iter().map(|x|x.len()).max().unwrap();
            let template: Vec<(&'static str, usize)> = template.into_iter().map(|s| (s, s.len())).collect();
            let indexes = self.solution_feels.iter().map(|e|e.enum_index()).collect::<Vec<_>>();
            let resolve: Vec<String> = multicell(&template[..], &indexes, fill, Resolve::STYLE);

            let template = [HowItStarted::UnkProblem, HowItStarted::NoGoal, HowItStarted::NoPlan,
                            HowItStarted::NoMilestones, HowItStarted::NoAccomplishments,
                            HowItStarted::SomeAccomplishments, HowItStarted::PlanNeededRevision,
                            HowItStarted::NeedsPolish, HowItStarted::Other("")];
            let template = template.into_iter().map(|e|e.text()).collect::<Vec<_>>();
            let fill = template.iter().map(|x|x.len()).max().unwrap();
            let template: Vec<(&'static str, usize)> = template.into_iter().map(|s| (s, s.len())).collect();
            let indexes = self.how_it_started.iter().map(|e|e.enum_index()).collect::<Vec<_>>();
            let how_it_started: Vec<String> = multicell(&template[..], &indexes, fill, HowItStarted::STYLE);

            let template = [ HowItsGoing::HelpWhatHappened, HowItsGoing::HelpWhatsNext, HowItsGoing::BetterUnderstanding, HowItsGoing::HighLevelPlan,
                             HowItsGoing::Milestones, HowItsGoing::Contributors, HowItsGoing::Schedule, HowItsGoing::CompletedMilestones, HowItsGoing::ImplementedSolution,
                             HowItsGoing::NeedUserFeedback, HowItsGoing::GettingUserFeedback ];
            let template = template.into_iter().map(|e|e.text()).collect::<Vec<_>>();
            let fill = template.iter().map(|x|x.len()).max().unwrap();
            let template: Vec<(&'static str, usize)> = template.into_iter().map(|s| (s, s.len())).collect();
            let indexes = self.how_its_going.iter().map(|e|e.enum_index()).collect::<Vec<_>>();
            let how_its_going: Vec<String> = multicell(&template[..], &indexes[..], fill, HowItsGoing::STYLE);

            any_progress
                .into_iter()
                .chain(how_big.into_iter())
                .chain(resolve.into_iter())
                .chain(how_it_started.into_iter())
                .chain(how_its_going.into_iter())
                .collect()
        }

        fn as_row_string(&self) -> String {
            let tds: String = self.as_td_cells().into_iter().collect();
            format!("<tr>{tds}</tr>")
        }
    }
}
