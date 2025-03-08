use eyre::bail;
use serde::{Deserialize, Serialize};

/// The front matter of a markdown blog post.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub author: String,
    #[serde(default)]
    pub release: bool,
    pub team: Option<String>,
    pub layout: String,
}

/// Extracts the front matter from a markdown file.
///
/// The remaining normal markdown content is returned as the second element of
/// the tuple.
pub fn parse(markdown: &str) -> eyre::Result<(FrontMatter, &str)> {
    if !markdown.starts_with("+++\n") {
        bail!("markdown file must start with the line `+++`");
    }
    let (front_matter, content) = markdown
        .trim_start_matches("+++\n")
        .split_once("\n+++\n")
        .expect("couldn't find the end of the front matter: `+++`");

    Ok((toml::from_str(front_matter)?, content))
}
