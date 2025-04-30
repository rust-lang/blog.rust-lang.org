use std::{error::Error, fmt::Display, fs, path::PathBuf, process::Command};

use front_matter::FrontMatter;
use inquire::autocompletion::Replacement;
use inquire::{Autocomplete, Confirm, CustomUserError, Select, Text, validator::Validation};
use rust_team_data::v1::{Team, Teams};

const BASE_TEAM_WEBSITE_URL: &str = "https://www.rust-lang.org/governance/teams/";

fn main() -> Result<(), Box<dyn Error>> {
    // If we cannot load teams, we won't provide any autocompletion, but the generate
    // command should still work.
    let team_data = match load_teams() {
        Ok(teams) => Some(teams),
        Err(error) => {
            eprintln!("Cannot download team data: {error}");
            None
        }
    };

    println!("\nHi, thanks for writing a post for the Rust blog!\n");

    let title = Text::new("What's the title of your post?")
        .with_validator(|input: &str| {
            if input.is_empty() {
                return Ok(Validation::Invalid("Title cannot be empty".into()));
            }
            Ok(Validation::Valid)
        })
        .prompt()?;

    let slug = title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || ['-', '.'].contains(c))
        .collect::<String>();
    let slug = slug.trim_matches('-').to_string();

    let blog = Select::new(
        "Which blog should the post belong to?",
        vec![Blog::Main, Blog::InsideRust],
    )
    .prompt()?;
    let blog_path = blog.path();

    let release = if blog == Blog::Main {
        Confirm::new("Is this an official Rust release announcement post?")
            .with_default(false)
            .prompt()?
    } else {
        false
    };

    let aliases = if !release {
        vec![]
    } else {
        // parse version from title (the front matter tests check against ?.??.?)
        let version = try_parse_version_from_title(&title).unwrap_or_else(|| "?.??.?".into());
        vec![format!("releases/{version}")]
    };

    let author = if release {
        "The Rust Release Team".into()
    } else {
        let author_prompt = Text::new("Who's the author of the post?").with_help_message(
            "If the post is authored by a team as a whole, write the team name here",
        );
        if let Some(git_user) = guess_author_from_git() {
            author_prompt.with_initial_value(git_user.trim()).prompt()?
        } else {
            author_prompt.prompt()?
        }
    };

    let (team, team_url) = 'team_prompt: {
        if release {
            // For official release annoucement posts, the whole
            // "Rust Release Team" is usually the author.
            break 'team_prompt (None, None);
        }
        if !Confirm::new("Is the post made on behalf of a team?")
            .with_help_message(
                "If the main author is already a team instead of an individual, answer No.",
            )
            .prompt()?
        {
            break 'team_prompt (None, None);
        }
        let mut team_prompt = Text::new("What is the team?");
        if let Some(ref teams) = team_data {
            team_prompt = team_prompt.with_autocomplete(TeamNames::from_teams(teams));
        }

        let team = team_prompt.prompt()?;

        let url = if let Some(url) = team_data
            .as_ref()
            .and_then(|teams| find_team_url(teams, &team))
        {
            url
        } else {
            Text::new("At what URL can people find the team?")
                .with_initial_value(BASE_TEAM_WEBSITE_URL)
                .prompt()?
        };
        (Some(team), Some(url))
    };

    let front_matter = FrontMatter {
        path: format!("{blog_path}9999/12/31/{slug}"),
        title,
        authors: vec![author],
        aliases,
        extra: front_matter::Extra {
            team,
            team_url,
            release,
        },
        ..Default::default()
    };

    let contents = format!(
        "+++\n{front_matter}+++\n{MARKDOWN_BOILERPLATE}",
        front_matter = toml::to_string_pretty(&front_matter).unwrap(),
    );

    let colocate = Confirm::new("Are you planning to include images in the post?")
        .with_help_message(
            "
  To include images in a post, the post will be stored in <slug>/index.md,
  instead of the usualy <slug>.md. Images can then be stored in the directory
  <slug>/ right next to the post itself. They can be included with a relative
  link, e.g. ![alt text](my_impage.png).
",
        )
        .with_default(false)
        .prompt()?;

    let base_path = format!("content/{blog_path}{slug}");

    let path_ending = if colocate { "/index.md" } else { ".md" };

    let mut path = PathBuf::from(format!("{base_path}{path_ending}"));

    'try_write_file: {
        if fs::create_dir_all(path.parent().unwrap()).is_ok()
            && !path.exists()
            && fs::write(&path, &contents).is_ok()
        {
            break 'try_write_file;
        }
        // A blog with that slug already exists. Generate an unambiguous path.
        for i in 2.. {
            path = PathBuf::from(format!("{base_path}@{i}{path_ending}"));
            if fs::create_dir_all(path.parent().unwrap()).is_ok()
                && !path.exists()
                && fs::write(&path, &contents).is_ok()
            {
                break 'try_write_file;
            }
        }
    }

    let path = path.display();
    println!(
        "
Success! A new blog post has been generated at the following path:

{path}

Remember: A post's date of publication is embedded in the `path` key of the
front matter. Keep the generated placeholder (9999/12/31) until the exact date
of publication is known. This is to prevent a post with an incorrect date from
being published - CI checks against the placeholder.
"
    );

    Ok(())
}

fn load_teams() -> Result<Teams, String> {
    let url = format!("{}/teams.json", rust_team_data::v1::BASE_URL);
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    if response.status() != 200 {
        return Err(format!("Cannot download teams data: {}", response.status()));
    }
    let teams: Teams = response
        .into_body()
        .read_json()
        .map_err(|e| e.to_string())?;
    Ok(teams)
}

fn find_team_url(teams: &Teams, team_name: &str) -> Option<String> {
    let team = teams.teams.get(team_name)?;
    let top_level_team = find_top_level_team(teams, team);
    let top_level_page = top_level_team
        .website_data
        .as_ref()
        .map(|w| w.page.as_str())
        .unwrap_or_else(|| top_level_team.name.as_str());

    // E.g. <BASE>compiler#team-miri
    Some(format!(
        "{BASE_TEAM_WEBSITE_URL}{top_level_page}#team-{team_name}"
    ))
}

fn find_top_level_team<'a>(teams: &'a Teams, team: &'a Team) -> &'a Team {
    if team.top_level.unwrap_or(false) {
        return team;
    }
    if let Some(parent) = team.subteam_of.as_ref().and_then(|t| teams.teams.get(t)) {
        return find_top_level_team(teams, parent);
    }
    team
}

#[derive(Clone)]
struct TeamNames(Vec<String>);

impl TeamNames {
    fn from_teams(teams: &Teams) -> Self {
        let names = teams.teams.keys().cloned().collect();
        Self(names)
    }
}

impl Autocomplete for TeamNames {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let mut names = self.0.clone();
        names.retain(|n| n.contains(input));
        Ok(names)
    }

    fn get_completion(
        &mut self,
        _input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(highlighted_suggestion)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Blog {
    #[default]
    Main,
    InsideRust,
}

impl Blog {
    fn path(&self) -> &'static str {
        match self {
            Blog::Main => "",
            Blog::InsideRust => "inside-rust/",
        }
    }
}

impl Display for Blog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Blog::Main => write!(f, "The main Rust blog"),
            Blog::InsideRust => write!(f, r#"The "Inside Rust" blog"#),
        }
    }
}

fn try_parse_version_from_title(title: &str) -> Option<String> {
    // assuming there won't ever be a Rust 2.0
    let (_, rest) = title.split_once("1.")?;
    let (minor, rest) = rest.split_once('.')?;
    let patch = rest
        .get(0..1)
        .and_then(|c| c.parse::<u8>().ok())
        .unwrap_or_default();
    Some(format!("1.{minor}.{patch}"))
}

fn guess_author_from_git() -> Option<String> {
    String::from_utf8(
        Command::new("git")
            .args(["config", "get", "user.name"])
            .output()
            .ok()?
            .stdout,
    )
    .ok()
}

const MARKDOWN_BOILERPLATE: &str = "
**insert your content here**

If you're wondering about available Markdown features, the blog is rendered with [Zola].
Here's an excerpt from its documentation:

> Content is written in [CommonMark], a strongly defined, highly compatible specification of [Markdown].
> Zola uses [pulldown-cmark] to parse markdown files.
> The goal of this library is 100% compliance with the CommonMark spec.
> It adds a few additional features such as parsing [footnotes], Github flavored [tables], Github flavored [task lists] and [strikethrough].

[Zola]: https://www.getzola.org/documentation/getting-started/overview/
[CommonMark]: https://commonmark.org/
[Markdown]: https://www.markdownguide.org/
[pulldown-cmark]: https://github.com/raphlinus/pulldown-cmark#pulldown-cmark
[footnotes]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#footnotes
[tables]: https://github.github.com/gfm/#tables-extension-
[task lists]: https://github.github.com/gfm/#task-list-items-extension-
[strikethrough]: https://github.github.com/gfm/#strikethrough-extension-
";
