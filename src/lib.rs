mod blogs;
mod posts;

use self::blogs::Blog;
use self::posts::Post;
use chrono::Timelike;
use eyre::{WrapErr, eyre};
use rayon::prelude::*;
use sass_rs::{Options, compile_file};
use serde::Serialize;
use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tera::Tera;

struct Generator {
    tera: Tera,
    blogs: Vec<Blog>,
    out_directory: PathBuf,
}

#[derive(Debug, Serialize)]
struct Releases {
    releases: Vec<ReleasePost>,
    feed_updated: String,
}

#[derive(Debug, Serialize)]
struct ReleasePost {
    title: String,
    url: String,
}

impl Generator {
    fn new(
        out_directory: impl AsRef<Path>,
        posts_directory: impl AsRef<Path>,
    ) -> eyre::Result<Self> {
        let mut tera = Tera::new("templates/*")?;
        tera.autoescape_on(vec![]); // disable auto-escape for .html templates
        Ok(Generator {
            tera,
            blogs: self::blogs::load(posts_directory.as_ref())?,
            out_directory: out_directory.as_ref().into(),
        })
    }

    fn file_url(&self, path: &Path) -> String {
        format!(
            "file:///{}/{}",
            self.out_directory
                .canonicalize()
                .unwrap_or_else(|_| self.out_directory.to_owned())
                .display()
                .to_string()
                .trim_start_matches('/')
                .replace(' ', "%20")
                .replace("\\\\?\\", ""),
            path.display()
        )
        .replace(std::path::MAIN_SEPARATOR, "/")
    }

    fn render(&self) -> eyre::Result<()> {
        // make sure our output directory exists
        fs::create_dir_all(&self.out_directory)?;

        for blog in &self.blogs {
            self.render_blog(blog)?;
        }
        self.compile_sass("app")?;
        self.compile_sass("noscript")?;
        self.compile_sass("fonts")?;
        self.concat_vendor_css(vec!["skeleton", "tachyons"])?;
        self.copy_static_files()?;
        Ok(())
    }

    fn compile_sass(&self, filename: &str) -> eyre::Result<()> {
        let scss_file = format!("./sass/{filename}.scss");
        let css_file = format!("./static/styles/{filename}.css");

        let css = compile_file(&scss_file, Options::default())
            .map_err(|error| eyre!(error))
            .wrap_err_with(|| format!("couldn't compile sass: {}", &scss_file))?;
        let mut file = File::create(&css_file)
            .wrap_err_with(|| format!("couldn't make css file: {}", &css_file))?;
        file.write_all(&css.into_bytes())
            .wrap_err_with(|| format!("couldn't write css file: {}", &css_file))?;

        Ok(())
    }

    fn concat_vendor_css(&self, files: Vec<&str>) -> eyre::Result<()> {
        let mut concatted = String::new();
        for filestem in files {
            let vendor_path = format!("./static/styles/{filestem}.css");
            let contents = fs::read_to_string(vendor_path).wrap_err("couldn't read vendor css")?;
            concatted.push_str(&contents);
        }
        fs::write("./static/styles/vendor.css", &concatted)
            .wrap_err("couldn't write vendor css")?;

        Ok(())
    }

    fn render_blog(&self, blog: &Blog) -> eyre::Result<()> {
        std::fs::create_dir_all(self.out_directory.join(blog.path()))?;

        let path = self.render_index(blog)?;

        println!("{}: {}", blog.title(), self.file_url(&path));

        self.render_feed(blog)?;
        self.render_releases_feed(blog)?;

        let paths = blog
            .posts()
            .par_iter()
            .map(|post| self.render_post(blog, post))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(path) = paths.first() {
            println!("└─ Latest post: {}\n", self.file_url(path));
        }

        Ok(())
    }

    fn render_index(&self, blog: &Blog) -> eyre::Result<PathBuf> {
        let data = json!({
            "title": blog.index_title(),
            "section": blog,
            "root": blog.path_back_to_root(),
        });
        let path = blog.path().join("index.html");
        self.render_template(&path, "index.html", data)?;
        Ok(path)
    }

    fn render_post(&self, blog: &Blog, post: &Post) -> eyre::Result<PathBuf> {
        let path = blog
            .path()
            .join(format!("{:04}", &post.year))
            .join(format!("{:02}", &post.month))
            .join(format!("{:02}", &post.day));
        fs::create_dir_all(self.out_directory.join(&path))?;

        // then, we render the page in that path
        let mut filename = PathBuf::from(&post.filename);
        filename.set_extension("html");

        let data = json!({
            "title": format!("{} | {}", post.title, blog.title()),
            "section": blog,
            "page": post,
            "root": blog.path_back_to_root().join("../../../"),
        });

        let path = path.join(filename);
        self.render_template(&path, &format!("{}.html", post.layout), data)?;
        Ok(path)
    }

    fn render_feed(&self, blog: &Blog) -> eyre::Result<()> {
        let posts: Vec<_> = blog.posts().iter().take(10).collect();
        let data = json!({
            "section": blog,
            "pages": posts,
            "feed_updated": chrono::Utc::now().with_nanosecond(0).unwrap().to_rfc3339(),
        });

        self.render_template(blog.path().join("feed.xml"), "feed.xml", data)?;
        Ok(())
    }

    fn render_releases_feed(&self, blog: &Blog) -> eyre::Result<()> {
        let posts = blog.posts().to_vec();
        let is_released: Vec<&Post> = posts.iter().filter(|post| post.release).collect();
        let releases: Vec<ReleasePost> = is_released
            .iter()
            .map(|post| ReleasePost {
                title: post.title.clone(),
                url: blog
                    .path()
                    .join(post.path.clone())
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();
        let data = Releases {
            releases,
            feed_updated: chrono::Utc::now().with_nanosecond(0).unwrap().to_rfc3339(),
        };
        fs::write(
            self.out_directory.join(blog.path()).join("releases.json"),
            serde_json::to_string(&data)?,
        )?;
        Ok(())
    }

    fn copy_static_files(&self) -> eyre::Result<()> {
        copy_dir("static/fonts", &self.out_directory)?;
        copy_dir("static/images", &self.out_directory)?;
        copy_dir("static/styles", &self.out_directory)?;
        copy_dir("static/scripts", &self.out_directory)?;
        Ok(())
    }

    fn render_template(
        &self,
        name: impl AsRef<Path>,
        template: &str,
        data: Value,
    ) -> eyre::Result<()> {
        let out_file = self.out_directory.join(name.as_ref());
        let file = File::create(out_file)?;
        self.tera
            .render_to(template, &tera::Context::from_value(data)?, file)?;
        Ok(())
    }
}

fn copy_dir(source: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), io::Error> {
    let source = source.as_ref();
    let dest = dest.as_ref().join(source.file_name().unwrap());
    assert!(source.is_dir());
    fn copy_inner(source: &Path, dest: &Path) -> Result<(), io::Error> {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let new_dest = dest.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_inner(&entry.path(), &new_dest)?;
            } else {
                fs::copy(entry.path(), &new_dest)?;
            }
        }
        Ok(())
    }
    copy_inner(source, &dest)
}

pub fn main() -> eyre::Result<()> {
    let blog = Generator::new("public", "content")?;

    blog.render()?;

    Ok(())
}

#[test]
fn snapshot() {
    let _ = std::fs::remove_dir_all(concat!(env!("CARGO_MANIFEST_DIR"), "/public"));
    main().unwrap();
    let timestamped_files = ["releases.json", "feed.xml"];
    let inexplicably_non_deterministic_files = ["images/2023-08-rust-survey-2022/experiences.png"];
    insta::glob!("..", "public/**/*", |path| {
        if path.is_dir() {
            return;
        }
        let path = path.display().to_string();
        if timestamped_files
            .into_iter()
            .chain(inexplicably_non_deterministic_files)
            .any(|f| path.ends_with(f))
        {
            // Skip troublesome files, e.g. they might contain timestamps.
            // If possible, they are tested separately below.
            return;
        }
        let content = fs::read(path).unwrap();
        // insta can't deal with non-utf8 strings?
        let content = String::from_utf8_lossy(&content).into_owned();
        insta::assert_snapshot!(content);
    });

    // test files with timestamps filtered
    insta::with_settings!({filters => vec![
        (r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\+\d{2}:\d{2}", "(filtered timestamp)"),
    ]}, {
        for file in timestamped_files {
            let content = fs::read(format!("public/{file}")).unwrap();
            let content = String::from_utf8_lossy(&content).into_owned();
            insta::assert_snapshot!(content);
        }
    });
}
