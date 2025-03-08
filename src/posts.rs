use super::blogs::Manifest;
use eyre::eyre;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

#[derive(Debug, PartialEq, Deserialize)]
struct TomlHeader {
    title: String,
    author: String,
    #[serde(default)]
    release: bool,
    team: Option<String>,
    layout: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub(crate) filename: String,
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) year: i32,
    pub(crate) show_year: bool,
    pub(crate) month: u32,
    pub(crate) day: u32,
    pub(crate) contents: String,
    pub(crate) url: String,
    pub(crate) published: String,
    pub(crate) updated: String,
    pub(crate) release: bool,
    pub(crate) has_team: bool,
    pub(crate) team: String,
    pub(crate) team_url: String,
}

impl Post {
    pub(crate) fn open(path: &Path, manifest: &Manifest) -> eyre::Result<Self> {
        // yeah this might blow up, but it won't
        let filename = path.file_name().unwrap().to_str().unwrap();

        // we need to get the metadata out of the url
        let mut split = filename.splitn(4, '-');

        // we do some unwraps because these need to be valid
        let year = split.next().unwrap().parse::<i32>().unwrap();
        let month = split.next().unwrap().parse::<u32>().unwrap();
        let day = split.next().unwrap().parse::<u32>().unwrap();
        let filename = split.next().unwrap().to_string();

        let contents = std::fs::read_to_string(path)?;
        if contents.len() < 5 {
            return Err(eyre!(
                "{path:?} is empty, or too short to have valid front matter"
            ));
        }

        // toml headers.... we know the first four bytes of each file are "+++\n"
        // so we need to find the end. we need the fours to adjust for those first bytes
        let end_of_toml = contents[4..].find("+++").unwrap() + 4;
        let toml = &contents[4..end_of_toml];
        let TomlHeader {
            author,
            title,
            release,
            team: team_string,
            layout,
        } = toml::from_str(toml)?;

        let options = comrak::Options {
            render: comrak::RenderOptions::builder().unsafe_(true).build(),
            extension: comrak::ExtensionOptions::builder()
                .header_ids(String::new())
                .strikethrough(true)
                .footnotes(true)
                .table(true)
                .build(),
            ..comrak::Options::default()
        };

        // Content starts after "+++\n" (we don't assume an extra newline)
        let contents = comrak::markdown_to_html(&contents[end_of_toml + 4..], &options);

        // finally, the url.
        let mut url = PathBuf::from(&*filename);
        url.set_extension("html");

        // this is fine
        let url = format!(
            "{:04}/{:02}/{:02}/{}",
            year,
            month,
            day,
            url.to_str().unwrap()
        );

        let published = build_post_time(year, month, day, 0);
        let updated = published.clone();

        // validate for now that the layout is specified as "post"
        match &*layout {
            "post" => (),
            _ => panic!(
                "blog post at path `{}` should have layout `post`",
                path.display()
            ),
        };

        // Enforce extra conditions
        if manifest.requires_team && team_string.is_none() {
            panic!("blog post at path `{}` lacks team metadata", path.display());
        }

        // If they supplied team, it should look like `team-text <team-url>`
        let (team, team_url) = team_string.map_or((None, None), |s| {
            static R: LazyLock<Regex> =
                LazyLock::new(|| Regex::new(r"(?P<name>[^<]*) <(?P<url>[^>]+)>").unwrap());
            let Some(captures) = R.captures(&s) else {
                panic!(
                    "team from path `{}` should have format `$name <$url>`",
                    path.display()
                )
            };
            (
                Some(captures["name"].to_string()),
                Some(captures["url"].to_string()),
            )
        });

        Ok(Self {
            filename,
            title,
            author,
            year,
            show_year: false,
            month,
            day,
            contents,
            url,
            published,
            updated,
            release,
            layout,
            has_team: team.is_some(),
            team: team.unwrap_or_default(),
            team_url: team_url.unwrap_or_default(),
        })
    }

    pub fn set_updated(&mut self, seconds: u32) {
        self.updated = build_post_time(self.year, self.month, self.day, seconds);
    }
}

fn build_post_time(year: i32, month: u32, day: u32, seconds: u32) -> String {
    let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let date_time = date.and_hms_opt(0, 0, seconds).unwrap();
    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(date_time, chrono::Utc).to_rfc3339()
}
