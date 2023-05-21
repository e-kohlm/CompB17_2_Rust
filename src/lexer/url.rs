use logos::{Lexer, Logos};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // Capture link definitions
    #[regex(r#"<a [^h]*href="[^"]*"[^>]*>[^<]*</a[ \s]*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    // Ignore all characters that do not belong to a link definition
    #[regex(r"<[^>]*>|[^<]*", logos::skip)]
    Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // Implement extraction from link definition
    let sliced_string = lex.slice();

    // Search for href and extract starting index
    let start = sliced_string.find(r#"href=""#).unwrap() + 6;
    // Search for "" and extract index
    let end = start + sliced_string[start..].find(r#"""#).unwrap();

    // Same for linktext
    let link_text_start = sliced_string.find(">").unwrap() + 1;
    let link_text_end = link_text_start + sliced_string[link_text_start..].find("<").unwrap();

    // From index create string of URL and linktext
    let link_url = sliced_string[start..end].to_string();
    let link_text = sliced_string[link_text_start..link_text_end].to_string();

    return (LinkUrl(link_url), LinkText(link_text));
}
