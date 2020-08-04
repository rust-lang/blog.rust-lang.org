use comrak::{
    nodes::{AstNode, NodeValue},
    Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
};
use kl_hyphenate::{Hyphenator, Language, Load, Standard};
use std::error::Error;

const SOFT_HYPHEN: char = '\u{00AD}';
const HYPHENATION_DICTIONARY: &str = "hyphenation-en-us.bincode";

pub(crate) fn render(input: &str) -> Result<String, Box<dyn Error>> {
    let options = ComrakOptions {
        render: ComrakRenderOptions {
            unsafe_: true, // Allow rendering of raw HTML
            ..ComrakRenderOptions::default()
        },
        extension: ComrakExtensionOptions {
            header_ids: Some(String::new()),
            ..ComrakExtensionOptions::default()
        },
        ..ComrakOptions::default()
    };

    let hyphenator = Standard::from_path(Language::EnglishUS, HYPHENATION_DICTIONARY)?;

    let arena = Arena::new();
    let ast = comrak::parse_document(&arena, input, &options);

    hyphenate(&ast, &hyphenator);

    let mut output = Vec::new();
    comrak::format_html(&ast, &options, &mut output)?;
    Ok(String::from_utf8(output)?)
}

// Pre-compute points inside words where browsers can add hyphens during rendering.
//
// Support for the CSS rule `hyphens: auto`, which tells the browser to split words by adding
// hyphens when there is no space left on the line, is quite low across browsers, preventing us
// from using it on the blog.
//
// A widely supported alternative is the `hyphens: manual` rule, which moves the burden of deciding
// *where* to break the word to the website. To properly use that rule, the website has to insert
// the "soft hyphen" unicode character (U+00AD) in every position the browser is allowed to break
// the word.
//
// The following piece of code walks through the Markdown AST adding those characters in every
// suitable place, thanks to the kl-hyphenate library.

fn hyphenate<'a>(node: &'a AstNode<'a>, hyphenator: &Standard) {
    match &mut node.data.borrow_mut().value {
        NodeValue::Text(content) => {
            if let Ok(string) = std::str::from_utf8(&content) {
                let hyphenated = add_soft_hyphens(string, hyphenator);
                *content = hyphenated.as_bytes().to_vec();
            }
        }
        _ => {}
    }
    for child in node.children() {
        hyphenate(child, hyphenator);
    }
}

fn add_soft_hyphens(content: &str, hyphenator: &Standard) -> String {
    let mut output = String::with_capacity(content.len());
    for (i, word) in content.split(' ').enumerate() {
        if i != 0 {
            output.push(' ');
        }
        let hyphenated = hyphenator.hyphenate(word);
        for (j, segment) in hyphenated.into_iter().segments().enumerate() {
            if j != 0 {
                output.push(SOFT_HYPHEN);
            }
            output.push_str(&segment);
        }
    }
    output
}
