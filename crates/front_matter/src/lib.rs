use eyre::{ContextCompat, bail};
use serde::{Deserialize, Serialize};
use toml::value::Date;

/// The front matter of a markdown blog post.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
    /// Deprecated. Post descriptions are not used anywhere in the templates.
    /// (only section descriptions)
    pub description: Option<String>,
    /// Used for `releases/X.XX.X` redirects and ones from the old URL scheme to
    /// preserve permalinks (e.g. slug.html => slug/).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
pub fn normalize(
    front_matter: &FrontMatter,
    slug: &str,
    inside_rust: bool,
) -> eyre::Result<FrontMatter> {
    let mut front_matter = front_matter.clone();

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

    // validate format of 'path'
    {
        let mut path = front_matter.path.as_str();
        let mut rq_fmt = "YYYY/MM/DD/slug-of-your-choice";
        if inside_rust {
            rq_fmt = "inside-rust/YYYY/MM/DD/slug-of-your-choice";
            if !path.starts_with("inside-rust/") {
                bail!("the path of inside-rust posts must start with 'inside-rust/'");
            }
            path = path.trim_start_matches("inside-rust/");
        }
        let components = path.split('/').collect::<Vec<_>>();
        if components.len() != 4
            || components[0].len() != 4
            || components[0].parse::<u32>().is_err()
            || components[1].len() != 2
            || components[1].parse::<u8>().is_err()
            || components[2].len() != 2
            || components[2].parse::<u8>().is_err()
        {
            bail!("invalid 'path' key in front matter, required format: {rq_fmt}");
        }
    }

    let path = front_matter.path.as_str();
    let date = path.strip_prefix("inside-rust/").unwrap_or(path);
    if date > "2025/04/14" {
        // Make sure that posts created after the migration to Zola don't have
        // the alias for preserving permalinks.
        //
        // Blog authors would often copy-paste an old post, reusing the
        // boilerplate for a new one. In that process, they'd usually preserve
        // that alias as well, probably not knowing what it's for. Technically
        // such an alias doesn't hurt much, it's just a redirect that isn't
        // referenced anywhere. So these aliases were allowed, preferring not to
        // bother blog authors with such unimportant stuff.
        //
        // However, there was a situation where a blog post was merged with
        // the wrong publication date because of this. Shortly before merging,
        // the date was updated, but it was accidentally changed in the useless
        // alias instead of the authoritative `path` key. Because of this
        // footgun, we don't allow creating these new aliases anymore.
        //
        // Blog authors who copy-paste old boilerplate will run into failed CI
        // and have to fix it by removing the alias. It's annoying, but better
        // than publishing a post with the wrong publication date.
        front_matter.aliases.retain(|a| !a.contains(".html"));
    }

    if front_matter.extra.team.is_some() ^ front_matter.extra.team_url.is_some() {
        bail!("extra.team and extra.team_url must always come in a pair");
    }

    // the crate generate_blog may create this placeholder
    if front_matter.aliases.iter().any(|a| a == "releases/?.??.?") {
        bail!("invalid release alias: releases/?.??.?");
    }

    if front_matter.extra.release && !front_matter.aliases.iter().any(|a| a.contains("releases")) {
        // Make sure release posts have a matching `releases/X.XX.X` alias.
        let version = guess_version_from_path(&front_matter.path).unwrap_or("?.??.?".into());
        front_matter.aliases.push(format!("releases/{version}"));
    }

    let serialized = toml::to_string_pretty(&front_matter)?;
    let deserialized = toml::from_str(&serialized)?;

    Ok(deserialized)
}

fn guess_version_from_path(path: &str) -> Option<String> {
    let mut iter = path.split(['-', '.']).filter_map(|s| s.parse::<u32>().ok());
    let major = iter.next()?;
    let minor = iter.next()?;
    let patch = iter.next().unwrap_or(0); // some posts omit the patch version
    Some(format!("{major}.{minor}.{patch}"))
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::PathBuf};

    use super::*;

    #[test]
    fn front_matter_is_normalized() {
        for post in all_posts() {
            let slug = post.file_stem().unwrap().to_str().unwrap();

            let inside_rust = post
                .as_os_str()
                .to_str()
                .unwrap()
                .contains("content/inside-rust/");

            let content = fs::read_to_string(&post).unwrap();
            let (front_matter, rest) = parse(&content).unwrap_or_else(|err| {
                panic!("failed to parse {:?}: {err}", post.display());
            });
            let normalized = normalize(&front_matter, slug, inside_rust).unwrap_or_else(|err| {
                panic!("failed to normalize {:?}: {err}", post.display());
            });

            if front_matter != normalized {
                let normalized = format!(
                    "\
                    +++\n\
                    {}\
                    +++\n\
                    {rest}\
                    ",
                    toml::to_string_pretty(&normalized).unwrap(),
                );
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

    /// This test is run by the merge queue check to make sure a blog post isn't
    /// merged before its date of publication is set. The date of a blog post
    /// is usually a placeholder (path = "9999/12/31/...") until shortly before
    /// it's published.
    #[test]
    #[ignore]
    fn date_is_set() {
        for post in all_posts() {
            let content = fs::read_to_string(&post).unwrap();
            let (front_matter, _) = parse(&content).unwrap();

            if front_matter.path.starts_with("9999/12/31") {
                panic!(
                    "\n\
                    The post {slug} has a placeholder publication date.\n\
                    If you're about to publish it, please set it to today.\n\
                    ",
                    slug = post.file_stem().unwrap().to_str().unwrap(),
                );
            }
        }
    }

    fn all_posts() -> impl Iterator<Item = PathBuf> {
        walkdir::WalkDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../content"))
            .into_iter()
            .filter_map(|e| e.ok().map(|e| e.into_path()))
            .filter(|p| {
                p.is_file()
                    && p.extension() == Some("md".as_ref())
                    && p.file_name() != Some("_index.md".as_ref())
                    && p.file_name() != Some("latest.md".as_ref())
            })
    }
}
