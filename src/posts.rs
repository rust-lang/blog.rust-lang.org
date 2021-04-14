use crate::blogs::Manifest;
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
    title: String,
    author: String,
    #[serde(default)]
    release: bool,
    team: Option<String>,
    layout: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Post {
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
    pub(crate) fn open(path: &Path, manifest: &Manifest) -> Result<Self, Box<dyn Error>> {
        // yeah this might blow up, but it won't
        let filename = path.file_name().unwrap().to_str().unwrap();

        // we need to get the metadata out of the url
        let mut split = filename.splitn(4, "-");

        // we do some unwraps because these need to be valid
        let year = split.next().unwrap().parse::<i32>().unwrap();
        let month = split.next().unwrap().parse::<u32>().unwrap();
        let day = split.next().unwrap().parse::<u32>().unwrap();
        let filename = split.next().unwrap().to_string();

        let contents = std::fs::read_to_string(path)?;

        // yaml headers.... we know the first four bytes of each file are "---\n"
        // so we need to find the end. we need the fours to adjust for those first bytes
        let end_of_yaml = contents[4..].find("---").unwrap() + 4;
        let yaml = &contents[..end_of_yaml];
        let YamlHeader {
            author,
            title,
            release,
            team: team_string,
            layout,
        } = serde_yaml::from_str(yaml)?;
        // next, the contents. we add + to get rid of the final "---\n\n"
        let options = ComrakOptions {
            render: ComrakRenderOptions {
                unsafe_: true, // Allow rendering of raw HTML
                ..ComrakRenderOptions::default()
            },
            extension: ComrakExtensionOptions {
                header_ids: Some(String::new()),
                footnotes: true,
                table: true,
                ..ComrakExtensionOptions::default()
            },
            ..ComrakOptions::default()
        };

        let contents = comrak::markdown_to_html(&contents[end_of_yaml + 5..], &options);

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
        let (team, team_url) = match team_string {
            Some(s) => {
                lazy_static::lazy_static! {
                    static ref R: Regex = Regex::new(r"(?P<name>[^<]*) <(?P<url>[^>]+)>").unwrap();
                }
                let captures = match R.captures(&s) {
                    Some(c) => c,
                    None => panic!(
                        "team from path `{}` should have format `$name <$url>`",
                        path.display()
                    ),
                };
                (
                    Some(captures["name"].to_string()),
                    Some(captures["url"].to_string()),
                )
            }

            None => (None, None),
        };

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
    chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDate::from_ymd(year, month, day).and_hms(0, 0, seconds),
        chrono::Utc,
    )
    .to_rfc3339()
}
