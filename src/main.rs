mod blogs;
mod posts;

use crate::blogs::Blog;
use crate::posts::Post;
use chrono::Timelike;
use tera::{Tera, try_get_value, Context};
use sass_rs::{compile_file, Options};
use serde_derive::Serialize;
use serde_json::{json, Value, to_value};
use std::convert::AsRef;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

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

fn month_name_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let month_num = try_get_value!("month_name", "value", u64, value);

    let month_name = match month_num {
        1 => "Jan.",
        2 => "Feb.",
        3 => "Mar.",
        4 => "Apr.",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "Aug.",
        9 => "Sept.",
        10 => "Oct.",
        11 => "Nov.",
        12 => "Dec.",
        _ => "Error!",
    };

    Ok(to_value(month_name).unwrap())
}

impl Generator {
    fn new(
        out_directory: impl AsRef<Path>,
        posts_directory: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut tera = Tera::new("templates/**/*.html")?;
        tera.register_filter("month_name", month_name_filter);

        Ok(Generator {
            tera,
            blogs: crate::blogs::load(posts_directory.as_ref())?,
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

    fn render(&self) -> Result<(), Box<dyn Error>> {
        // make sure our output directory exists
        fs::create_dir_all(&self.out_directory)?;

        for blog in &self.blogs {
            self.render_blog(blog)?;
        }
        self.compile_sass("app");
        self.compile_sass("fonts");
        self.concat_vendor_css(vec!["skeleton", "tachyons"]);
        self.copy_static_files()?;
        Ok(())
    }

    fn compile_sass(&self, filename: &str) {
        let scss_file = format!("./src/styles/{}.scss", filename);
        let css_file = format!("./static/styles/{}.css", filename);

        let css = compile_file(&scss_file, Options::default())
            .expect(&format!("couldn't compile sass: {}", &scss_file));
        let mut file =
            File::create(&css_file).expect(&format!("couldn't make css file: {}", &css_file));
        file.write_all(&css.into_bytes())
            .expect(&format!("couldn't write css file: {}", &css_file));
    }

    fn concat_vendor_css(&self, files: Vec<&str>) {
        let mut concatted = String::new();
        for filestem in files {
            let vendor_path = format!("./static/styles/{}.css", filestem);
            let contents = fs::read_to_string(vendor_path).expect("couldn't read vendor css");
            concatted.push_str(&contents);
        }
        fs::write("./static/styles/vendor.css", &concatted).expect("couldn't write vendor css");
    }

    fn render_blog(&self, blog: &Blog) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(self.out_directory.join(blog.prefix()))?;

        let path = self.render_index(blog)?;

        println!("{}: {}", blog.title(), self.file_url(&path));

        self.render_feed(blog)?;
        self.render_releases_feed(blog)?;

        for (i, post) in blog.posts().iter().enumerate() {
            let path = self.render_post(blog, post)?;
            if i == 0 {
                println!("└─ Latest post: {}\n", self.file_url(&path));
            }
        }

        Ok(())
    }

    fn render_index(&self, blog: &Blog) -> Result<PathBuf, Box<dyn Error>> {
        let other_blogs: Vec<_> = self
            .blogs
            .iter()
            .filter(|b| b.index_title() != blog.index_title())
            .map(|other_blog| {
                json!({
                    "link_text": other_blog.link_text(),
                    "url": other_blog.prefix().join("index.html"),
                })
            })
            .collect();

        let data = json!({
            "title": blog.index_title(),
            "parent": "layout",
            "blog": blog,
            "other_blogs": other_blogs,
            "root": blog.path_back_to_root(),
        });
        let path = blog.prefix().join("index.html");
        self.render_template(&path, "index", data)?;
        Ok(path)
    }

    fn render_post(&self, blog: &Blog, post: &Post) -> Result<PathBuf, Box<dyn Error>> {
        let path = blog
            .prefix()
            .join(format!("{:04}", &post.year))
            .join(format!("{:02}", &post.month))
            .join(format!("{:02}", &post.day));
        fs::create_dir_all(self.out_directory.join(&path))?;

        // then, we render the page in that path
        let mut filename = PathBuf::from(&post.filename);
        filename.set_extension("html");

        let data = json!({
            "title": format!("{} | {}", post.title, blog.title()),
            "parent": "layout",
            "blog": blog,
            "post": post,
            "root": blog.path_back_to_root().join("../../../"),
        });

        let path = path.join(filename);
        self.render_template(&path, &post.layout, data)?;
        Ok(path)
    }

    fn render_feed(&self, blog: &Blog) -> Result<(), Box<dyn Error>> {
        let posts: Vec<_> = blog.posts().iter().take(10).collect();
        let data = json!({
            "blog": blog,
            "posts": posts,
            "feed_updated": chrono::Utc::now().with_nanosecond(0).unwrap().to_rfc3339(),
        });

        self.render_template(blog.prefix().join("feed.xml"), "feed", data)?;
        Ok(())
    }

    fn render_releases_feed(&self, blog: &Blog) -> Result<(), Box<dyn Error>> {
        let posts = blog.posts().iter().cloned().collect::<Vec<_>>();
        let is_released: Vec<&Post> = posts.iter().filter(|post| post.release).collect();
        let releases: Vec<ReleasePost> = is_released
            .iter()
            .map(|post| ReleasePost {
                title: post.title.clone(),
                url: blog
                    .prefix()
                    .join(post.url.clone())
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();
        let data = Releases {
            releases,
            feed_updated: chrono::Utc::now().with_nanosecond(0).unwrap().to_rfc3339(),
        };
        fs::write(
            self.out_directory.join(blog.prefix()).join("releases.json"),
            serde_json::to_string(&data)?,
        )?;
        Ok(())
    }

    fn copy_static_files(&self) -> Result<(), Box<dyn Error>> {
        use fs_extra::dir::{self, CopyOptions};

        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;

        dir::copy("static/fonts", &self.out_directory, &options)?;
        dir::copy("static/images", &self.out_directory, &options)?;
        dir::copy("static/styles", &self.out_directory, &options)?;
        dir::copy("static/scripts", &self.out_directory, &options)?;

        Ok(())
    }

    fn render_template(
        &self,
        name: impl AsRef<Path>,
        template_name: &str,
        data: serde_json::Value,
    ) -> Result<(), Box<dyn Error>> {
        let out_path = self.out_directory.join(name.as_ref());
        let mut out_file = File::create(out_path)?;
        let context = Context::from_value(data)?;
        let template_file_name = template_name.to_string() + ".html";

        let rendered = self.tera.render(&template_file_name, &context)?;
        out_file.write(rendered.as_bytes())?;
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let blog = Generator::new("site", "posts")?;

    blog.render()?;

    Ok(())
}
