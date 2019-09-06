mod posts;

use crate::posts::Post;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use sass_rs::{compile_file, Options};
use serde_derive::Serialize;
use serde_json::json;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

struct Blog {
    handlebars: Handlebars,
    posts: Vec<Post>,
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

fn hb_month_helper<'a>(
    h: &Helper,
    _b: &Handlebars,
    _ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let num: u32 = h
        .param(0)
        .unwrap()
        .value()
        .as_str()
        .unwrap()
        .parse()
        .or_else(|_| Err(RenderError::new("The value is not a number")))?;
    let name = match num {
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
    out.write(name)?;
    Ok(())
}

impl Blog {
    fn new<T>(out_directory: T, posts_directory: T) -> Result<Blog, Box<dyn Error>>
    where
        T: Into<PathBuf>,
    {
        let mut handlebars = Handlebars::new();

        handlebars.set_strict_mode(true);

        handlebars.register_templates_directory(".hbs", "templates")?;

        handlebars.register_helper("month_name", Box::new(hb_month_helper));

        let posts = Blog::load_posts(posts_directory.into())?;

        Ok(Blog {
            handlebars,
            posts,
            out_directory: out_directory.into(),
        })
    }

    fn load_posts(dir: PathBuf) -> Result<Vec<Post>, Box<dyn Error>> {
        let mut posts = Vec::new();

        for entry in fs::read_dir(dir)? {
            let path = entry?.path();

            // ignore vim temporary files
            let filename = path.file_name().unwrap().to_str().unwrap();
            if filename.starts_with(".") && filename.ends_with(".swp") {
                continue;
            }

            posts.push(Post::open(&path)?);
        }

        // finally, sort the posts. oldest first.
        posts.sort_by_key(|post| post.url.clone());
        posts.reverse();

        for i in 1..posts.len() {
            posts[i].show_year = posts[i - 1].year != posts[i].year;
        }

        Ok(posts)
    }

    fn render(&self) -> Result<(), Box<dyn Error>> {
        // make sure our output directory exists
        fs::create_dir_all(&self.out_directory)?;

        self.render_index()?;

        self.render_posts()?;

        self.render_feed()?;

        self.compile_sass("app");
        self.compile_sass("fonts");

        self.concat_vendor_css(vec!["skeleton", "tachyons"]);

        self.copy_static_files()?;

        self.generate_releases_feed()?;

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

    fn render_index(&self) -> Result<(), Box<dyn Error>> {
        let data = json!({
            "title": "The Rust Programming Language Blog",
            "parent": "layout",
            "posts": self.posts,
        });

        self.render_template("index.html", "index", data)?;

        Ok(())
    }

    fn render_posts(&self) -> Result<(), Box<dyn Error>> {
        for post in &self.posts {
            // first, we create the path
            //let path = PathBuf::from(&self.out_directory);

            let path = PathBuf::from(&post.year);
            let path = path.join(&post.month);
            let path = path.join(&post.day);

            fs::create_dir_all(self.out_directory.join(&path))?;

            // then, we render the page in that path
            let mut filename = PathBuf::from(&post.filename);
            filename.set_extension("html");

            let data = json!({
                "title": format!("{} | Rust Blog", post.title),
                "parent": "layout",
                "post": post,
            });

            self.render_template(path.join(filename).to_str().unwrap(), "post", data)?;
        }

        Ok(())
    }

    fn render_feed(&self) -> Result<(), Box<dyn Error>> {
        let posts: Vec<_> = self.posts.iter().by_ref().take(10).collect();
        let data =
            json!({ "posts": posts, "feed_updated":  time::now_utc().rfc3339().to_string() });

        self.render_template("feed.xml", "feed", data)?;
        Ok(())
    }

    fn generate_releases_feed(&self) -> Result<(), Box<dyn Error>> {
        let posts = self.posts.clone();
        let is_released: Vec<&Post> = posts.iter().filter(|post| post.release).collect();
        let releases: Vec<ReleasePost> = is_released
            .iter()
            .map(|post| ReleasePost {
                title: post.title.clone(),
                url: post.url.clone(),
            })
            .collect();
        let data = Releases {
            releases: releases,
            feed_updated: time::now_utc().rfc3339().to_string(),
        };
        fs::write(
            self.out_directory.join("releases.json"),
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

        Ok(())
    }

    fn render_template(
        &self,
        name: &str,
        template: &str,
        data: serde_json::Value,
    ) -> Result<(), Box<dyn Error>> {
        let out_file = self.out_directory.join(name);

        let file = File::create(out_file)?;

        self.handlebars.render_to_write(template, &data, file)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let blog = Blog::new("site", "posts")?;

    blog.render()?;

    Ok(())
}
