use comrak::ComrakOptions;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
    title: String,
    author: String,
    #[serde(default)]
    release: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Post {
    pub(crate) filename: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) year: String,
    pub(crate) show_year: bool,
    pub(crate) month: String,
    pub(crate) day: String,
    pub(crate) contents: String,
    pub(crate) url: String,
    pub(crate) published: String,
    pub(crate) release: bool,
}

impl Post {
    pub(crate) fn open(path: &Path) -> Result<Self, Box<dyn Error>> {
        // yeah this might blow up, but it won't
        let filename = path.file_name().unwrap().to_str().unwrap();

        // we need to get the metadata out of the url
        let mut split = filename.splitn(4, "-");

        let year = split.next().unwrap().to_string();
        let month = split.next().unwrap().to_string();
        let day = split.next().unwrap().to_string();
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
        } = serde_yaml::from_str(yaml)?;
        // next, the contents. we add + to get rid of the final "---\n\n"
        let options = ComrakOptions {
            ext_header_ids: Some(String::new()),
            ..ComrakOptions::default()
        };

        let contents = comrak::markdown_to_html(&contents[end_of_yaml + 5..], &options);

        // finally, the url.
        let mut url = PathBuf::from(&*filename);
        url.set_extension("html");

        // this is fine
        let url = format!("{}/{}/{}/{}", year, month, day, url.to_str().unwrap());

        // build the published time. this is only approximate, which is fine.
        // we do some unwraps because these need to be valid
        let published = time::Tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: day.parse::<i32>().unwrap(),
            tm_mon: month.parse::<i32>().unwrap() - 1, // 0-11 not 1-12
            tm_year: year.parse::<i32>().unwrap() - 1900, // from the year 1900, not the actual year
            // these next two fields are wrong but we never use them to generate our times
            tm_wday: 1,
            tm_yday: 1,
            tm_isdst: 0,
            tm_utcoff: 0,
            tm_nsec: 0,
        };

        let published = published.rfc3339().to_string();

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
            release,
        })
    }
}
