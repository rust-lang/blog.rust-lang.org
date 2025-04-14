use eyre::{ContextCompat, bail};
use serde::{Deserialize, Serialize};
use toml::value::Date;

/// The front matter of a markdown blog post.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    /// Deprecated. The plan was probably to have more specialized templates
    /// at some point. That didn't materialize, all posts are rendered with the
    /// same template. Once we migrate to Zola, this can be achieved with the
    /// "template" key.
    #[serde(default, skip_serializing)]
    pub layout: Option<String>,
    /// Deprecated. Zola doesn't do any path templating based on things like
    /// the date. So, in order to preserve our URL structure (YYYY/MM/DD/...)
    /// we have to set the path explicitly. Duplicating the date would
    /// be inconvenient for content authors who need to keep the date of
    /// publication updated.
    #[serde(default, skip_serializing)]
    pub date: Option<Date>,
    #[serde(default)]
    pub path: String,
    pub title: String,
    /// Deprecated. Zola uses an "authors" key with an array instead. The front
    /// matter tests can do the migration automatically.
    #[serde(default, skip_serializing)]
    pub author: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: Option<String>,
    /// Used to generate redirects from the old URL scheme to preserve
    /// permalinks.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Moved to the `extra` table.
    #[serde(default, skip_serializing)]
    pub team: Option<String>,
    /// Moved to the `extra` table.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub release: bool,
    #[serde(default, skip_serializing_if = "Extra::is_empty")]
    pub extra: Extra,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Extra {
    pub team: Option<String>,
    pub team_url: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub release: bool,
}

impl Extra {
    fn is_empty(&self) -> bool {
        self.team.is_none() && !self.release
    }
}

/// Extracts the front matter from a markdown file.
///
/// The remaining normal markdown content is returned as the second element of
/// the tuple.
pub fn parse(markdown: &str) -> eyre::Result<(FrontMatter, &str)> {
    if !markdown.starts_with("+++\n") {
        bail!("missing start of TOML front matter (+++)");
    }
    let (front_matter, content) = markdown
        .trim_start_matches("+++\n")
        .split_once("\n+++\n")
        .context("missing end of TOML front matter (+++)")?;

    Ok((toml::from_str(front_matter)?, content))
}

/// Normalizes the front matter of a markdown file.
pub fn normalize(markdown: &str, slug: &str, inside_rust: bool) -> eyre::Result<String> {
    let (mut front_matter, content) = parse(markdown)?;

    // migrate "author" to "authors" key
    if let Some(author) = front_matter.author.take() {
        front_matter.authors = vec![author];
    }
    // migrate "team" to "extra" section
    if let Some(team) = front_matter.team.take() {
        let (team, url) = team.split_once(" <").unwrap();
        let url = url.strip_suffix('>').unwrap();
        front_matter.extra.team = Some(team.into());
        front_matter.extra.team_url = Some(url.into());
    }
    // migrate "release" to "extra" section
    if front_matter.release {
        front_matter.release = false;
        front_matter.extra.release = true;
    }
    // migrate "date" to "path" key
    if let Some(date) = front_matter.date.take() {
        front_matter.path = format!(
            "{inside_rust}{year}/{month:02}/{day:02}/{slug}",
            inside_rust = if inside_rust { "inside-rust/" } else { "" },
            year = date.year,
            month = date.month,
            day = date.day,
            // remove @ suffix, used for disambiguation only in the source
            slug = slug.split_once('@').map(|(s, _)| s).unwrap_or(slug),
        );
    }
    front_matter.aliases = vec![format!("{}.html", front_matter.path)];

    if front_matter.extra.team.is_some() ^ front_matter.extra.team_url.is_some() {
        bail!("extra.team and extra.team_url must always come in a pair");
    }

    Ok(format!(
        "\
+++
{}\
+++
{content}",
        toml::to_string_pretty(&front_matter)?
    ))
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use super::*;

    #[test]
    fn front_matter_is_normalized() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");

        let posts = fs::read_dir(repo_root.join("content"))
            .unwrap()
            .chain(fs::read_dir(repo_root.join("content/inside-rust")).unwrap())
            .map(|p| p.unwrap().path())
            .filter(|p| p.is_file() && p.file_name() != Some("_index.md".as_ref()));

        for post in posts {
            let slug = post.file_stem().unwrap().to_str().unwrap();

            let inside_rust = post
                .as_os_str()
                .to_str()
                .unwrap()
                .contains("content/inside-rust/");

            let content = fs::read_to_string(&post).unwrap();
            let normalized = normalize(&content, slug, inside_rust).unwrap_or_else(|err| {
                panic!("failed to normalize {:?}: {err}", post.file_name().unwrap());
            });

            if content != normalized {
                if env::var("FIX_FRONT_MATTER").is_ok() {
                    fs::write(post, normalized).unwrap();
                    continue;
                }

                let post = post.file_name().unwrap().to_str().unwrap();
                let actual = content
                    .rsplit_once("+++")
                    .map(|(f, _)| format!("{f}+++"))
                    .unwrap_or(content);
                let expected = normalized
                    .rsplit_once("+++")
                    .map(|(f, _)| format!("{f}+++"))
                    .unwrap_or(normalized);

                // better error message than assert_eq!()
                panic!(
                    "
The post {post} has abnormal front matter.

    actual:
{actual}

    expected:
{expected}

    ┌──────────────────────────────────────────────────────────────────────────┐
    │                                                                          │
    │                You can fix this automatically by running:                │
    │                                                                          │
    │              FIX_FRONT_MATTER=1 cargo test -p front_matter               │
    │                                                                          │
    └──────────────────────────────────────────────────────────────────────────┘
",
                )
            };
        }
    }
}
