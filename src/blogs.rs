use super::posts::Post;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

static MANIFEST_FILE: &str = "_index.md";
static POSTS_EXT: &str = "md";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Title to display in the "top row".
    pub(crate) title: String,

    /// Title to use in the html header.
    pub(crate) index_title: String,

    /// Description for metadata
    pub(crate) description: String,

    /// Who maintains this blog? Appears in the rss feed.
    pub(crate) maintained_by: String,

    /// Raw html describing the blog to insert into the index page.
    pub(crate) index_html: String,

    /// What text to use when linking to this blog in the "see also"
    /// section from other blogs.
    pub(crate) link_text: String,
}

#[derive(Serialize)]
pub struct Blog {
    title: String,
    index_title: String,
    link_text: String,
    description: String,
    maintained_by: String,
    index_html: String,
    #[serde(serialize_with = "add_postfix_slash")]
    prefix: PathBuf,
    posts: Vec<Post>,
}

impl Blog {
    fn load(prefix: PathBuf, dir: &Path) -> eyre::Result<Self> {
        let manifest_content = std::fs::read_to_string(dir.join(MANIFEST_FILE))?
            .strip_prefix("+++\n")
            .unwrap()
            .strip_suffix("+++\n")
            .unwrap()
            .to_string();
        let manifest: Manifest = toml::from_str(&manifest_content)?;

        let mut posts = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.ends_with("_index.md") {
                continue; // blog manifest is not a post
            }
            let ext = path.extension().and_then(|e| e.to_str());
            if path.metadata()?.file_type().is_file() && ext == Some(POSTS_EXT) {
                posts.push(Post::open(&path)?);
            }
        }

        posts.sort_by_key(|post| {
            format!(
                "{}-{:02}-{:02}-{}",
                post.year, post.month, post.day, post.title
            )
        });
        posts.reverse();

        // Decide which posts should show the year in the index.
        posts[0].show_year = true;
        for i in 1..posts.len() {
            posts[i].show_year = posts[i - 1].year != posts[i].year;
        }

        // Make the updated time is unique, by incrementing seconds for duplicates
        let mut last_matching_updated = 0;
        for i in 1..posts.len() {
            if posts[i].updated == posts[last_matching_updated].updated {
                posts[i].set_updated((i - last_matching_updated) as u32);
            } else {
                last_matching_updated = i;
            }
        }

        Ok(Self {
            title: manifest.title,
            index_title: manifest.index_title,
            description: manifest.description,
            maintained_by: manifest.maintained_by,
            index_html: manifest.index_html,
            link_text: manifest.link_text,
            prefix,
            posts,
        })
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn link_text(&self) -> &str {
        &self.link_text
    }

    pub(crate) fn index_title(&self) -> &str {
        &self.index_title
    }

    pub(crate) fn prefix(&self) -> &Path {
        &self.prefix
    }

    pub(crate) fn posts(&self) -> &[Post] {
        &self.posts
    }
}

/// Recursively load blogs in a directory. A blog is a directory with a
/// `_index.md` file inside it.
pub fn load(base: &Path) -> eyre::Result<Vec<Blog>> {
    let mut blogs = Vec::new();
    load_recursive(base, base, &mut blogs)?;
    Ok(blogs)
}

fn load_recursive(base: &Path, current: &Path, blogs: &mut Vec<Blog>) -> eyre::Result<()> {
    for entry in std::fs::read_dir(current)? {
        let path = entry?.path();
        let file_type = path.metadata()?.file_type();

        if file_type.is_dir() {
            load_recursive(base, &path, blogs)?;
        } else if file_type.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str());
            if let (Some(file_name), Some(parent)) = (file_name, path.parent()) {
                if file_name == MANIFEST_FILE {
                    let prefix = parent
                        .strip_prefix(base)
                        .map_or_else(|_| PathBuf::new(), Path::to_path_buf);
                    blogs.push(Blog::load(prefix, parent)?);
                }
            }
        }
    }
    Ok(())
}

fn add_postfix_slash<S>(path: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut str_repr = path.to_string_lossy().to_string();
    if !str_repr.is_empty() {
        str_repr.push('/');
    }
    serializer.serialize_str(&str_repr)
}
