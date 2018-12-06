use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use comrak::ComrakOptions;

use handlebars::Handlebars;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

use sass_rs::{compile_file, Options};

struct Blog {
    handlebars: Handlebars,
    posts: Vec<Post>,
    out_directory: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    filename: String,
    title: String,
    author: String,
    year: String,
    month: String,
    day: String,
    contents: String,
    url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct YamlHeader {
    title: String,
    author: String,
}

impl Blog {
    fn new<T>(out_directory: T, posts_directory: T) -> Result<Blog, Box<Error>>
    where
        T: Into<PathBuf>,
    {
        let mut handlebars = Handlebars::new();

        handlebars.set_strict_mode(true);

        handlebars.register_templates_directory(".hbs", "templates")?;

        let posts = Blog::load_posts(posts_directory.into())?;

        Ok(Blog {
            handlebars,
            posts,
            out_directory: out_directory.into(),
        })
    }

    fn load_posts(dir: PathBuf) -> Result<Vec<Post>, Box<Error>> {
        let mut posts = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            // yeah this might blow up, but it won't
            let filename = path.file_name().unwrap().to_str().unwrap();

            // we need to get the metadata out of the url
            let mut split = filename.splitn(4, "-");

            let year = split.next().unwrap().to_string();
            let month = split.next().unwrap().to_string();
            let day = split.next().unwrap().to_string();
            let filename = split.next().unwrap().to_string();

            let contents = fs::read_to_string(path)?;

            // yaml headers.... we know the first four bytes of each file are "---\n"
            // so we need to find the end. we need the fours to adjust for those first bytes
            let end_of_yaml = contents[4..].find("---").unwrap() + 4;
            let yaml = &contents[..end_of_yaml];

            let YamlHeader { author, title } = serde_yaml::from_str(yaml)?;

            // next, the contents. we add + to get rid of the final "---\n\n"
            let contents =
                comrak::markdown_to_html(&contents[end_of_yaml + 5..], &ComrakOptions::default());

            // finally, the url.
            let mut url = PathBuf::from(&*filename);
            url.set_extension("html");

            // this is fine
            let url = format!("{}/{}/{}/{}", year, month, day, url.to_str().unwrap());

            let post = Post {
                filename,
                title,
                author,
                year,
                month,
                day,
                contents,
                url,
            };

            posts.push(post);
        }

        // finally, sort the posts. oldest first.

        // we're gonna cheat:
        posts.reverse();

        Ok(posts)
    }

    fn render(&self) -> Result<(), Box<Error>> {
        // make sure our output directory exists
        fs::create_dir_all(&self.out_directory)?;

        self.render_index()?;

        self.render_posts()?;

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

    fn render_index(&self) -> Result<(), Box<Error>> {
        let data = json!({
            "title": "Blog",
            "parent": "layout",
            "posts": self.posts,
        });

        self.render_template("index.html", "index", data)?;

        Ok(())
    }

    fn render_posts(&self) -> Result<(), Box<Error>> {
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
                "title": "The Rust Programming Language Blog",
                "parent": "layout",
                "post": post,
            });

            self.render_template(path.join(filename).to_str().unwrap(), "post", data)?;
        }

        Ok(())
    }

    fn copy_static_files(&self) -> Result<(), Box<Error>> {
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
    ) -> Result<(), Box<Error>> {
        let out_file = self.out_directory.join(name);

        let file = File::create(out_file)?;

        self.handlebars.render_to_write(template, &data, file)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<Error>> {
    let blog = Blog::new("site", "posts")?;

    blog.render()?;

    Ok(())
}
