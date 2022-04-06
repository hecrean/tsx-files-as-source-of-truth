use anyhow::{Error, Ok, Result};
use html_parser::{Dom, Node};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::{env, fs};
use walkdir::{DirEntry, FilterEntry, WalkDir};

// This example illustrates how to use the library to get all of the anchor-hrefs from a document.
//            #[[:space:]]*
fn extract_html(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (?s:.)*
            return
            [[:space:]]*            
            \(
            [[:space:]]*            
            <Fragment>
            (?P<html>[\s|\S]*)
            </Fragment>
            [[:space:]]*            
            \)",
        )
        .unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("html").map(|html| html.as_str()))
}

fn extract_headers(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HEADER_REGEX: Regex =
            Regex::new(r"<h[[:digit:]][\s|\S]*>(?P<header_content>[\s|\S]*)</h[[:digit:]]>")
                .unwrap();
    }
    HEADER_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fragments() {
        let text = r#"  //another function \n return(  <Fragment> <h1 id='hello'>hello</h1> </Fragment> )  \n"#;

        assert_eq!(extract_html(text), Some(" <h1 id='hello'>hello</h1> "));
    }
    #[test]
    fn headers() {
        let dom_node = "<h1></h1>   <h2 id='1'></h2> ";
        let headers = extract_headers(dom_node);
        assert!(headers.contains("<h2 id='1'></h2>"));
    }
}

fn matchHeadings(s: &str) -> bool {
    match s {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => true,
        _ => false,
    }
}

fn extractHeadings(s: &str) -> Result<Vec<String>> {
    let dom = Dom::parse(s)?;
    let iter = dom.children.get(0).unwrap().into_iter();

    let headings_ids: Vec<_> = iter
        .filter_map(|item| match item {
            Node::Element(ref element) if matchHeadings(element.name.as_str()) => {
                element.id.clone()
            }
            _ => None,
        })
        .collect();

    Ok(headings_ids)
}

fn run() -> Result<()> {
    let current_dir = "/Users/hectorcrean/Desktop/ALL120-beta/app/src/pages/body-contouring/cool-sculpting/1.cryolipolysis-effects/studies/adipose-tissue-response.tsx";

    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
        let metadata = fs::metadata(&entry.path())?;

        if metadata.is_file() {
            let contents =
                fs::read_to_string(&entry.path()).expect("Something went wrong reading the file");

            let html = ["<html>", extract_html(&contents).unwrap(), "</html>"].join(" ");
        }
    }

    Ok(())
}
