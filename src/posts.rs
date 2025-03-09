use super::blogs::Manifest;
use front_matter::FrontMatter;
use regex::Regex;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};
use toml::value::Date;

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub(crate) filename: String,
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) year: u16,
    pub(crate) show_year: bool,
    pub(crate) month: u8,
    pub(crate) day: u8,
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
        let filename = {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            // '@' is used as a disambiguator between file names that were
            // previously identical except for the date prefix (which was
            // removed). The URL doesn't need the disambiguator, because it has
            // the date in it. Also, we must remove it to preserve permalinks.
            match filename.split_once('@') {
                Some((pre, _)) => format!("{pre}.md"),
                None => filename,
            }
        };

        let contents = std::fs::read_to_string(path)?;

        let (
            FrontMatter {
                author,
                title,
                release,
                team: team_string,
                layout,
                date: Date { year, month, day },
                ..
            },
            contents,
        ) = front_matter::parse(&contents)?;

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

        let contents = comrak::markdown_to_html(contents, &options);

        // finally, the url.
        let mut url = PathBuf::from(&filename);
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

fn build_post_time(year: u16, month: u8, day: u8, seconds: u32) -> String {
    let date = chrono::NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).unwrap();
    let date_time = date.and_hms_opt(0, 0, seconds).unwrap();
    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(date_time, chrono::Utc).to_rfc3339()
}
