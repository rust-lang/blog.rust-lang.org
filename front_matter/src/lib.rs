use eyre::bail;
use serde::{Deserialize, Serialize};
use toml::value::Date;

/// The front matter of a markdown blog post.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FrontMatter {
    pub layout: String,
    pub date: Date,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub team: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub release: bool,
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

/// Normalizes the front matter of a markdown file.
pub fn normalize(markdown: &str) -> eyre::Result<String> {
    let (front_matter, content) = parse(markdown)?;

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

        let posts = fs::read_dir(repo_root.join("posts"))
            .unwrap()
            .chain(fs::read_dir(repo_root.join("posts/inside-rust")).unwrap())
            .map(|p| p.unwrap().path())
            .filter(|p| p.extension() == Some("md".as_ref()));

        for post in posts {
            let content = fs::read_to_string(&post).unwrap();
            let normalized = normalize(&content).unwrap();

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
    │      FIX_FRONT_MATTER=1 cargo test --all front_matter_is_normalized      │
    │                                                                          │
    └──────────────────────────────────────────────────────────────────────────┘
",
                )
            };
        }
    }
}
