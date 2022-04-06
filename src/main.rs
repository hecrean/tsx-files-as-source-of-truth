// #![feature(box_patterns)]
#![feature(string_remove_matches)]

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use swc::atoms::JsWord;
use swc_common::SourceMap;
use swc_common::{sync::Lrc, FileName};
use swc_ecma_ast::*;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};
use walkdir::WalkDir;

/*
Two methods of doing this:
1. traverse the AST, and effectfully print out values when we arrive at certain nodes
2. fold (ie. reduce/destructively traverse) the AST, and only keep the nodes we want
*/
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TsxNodeVisitor {
    text: Vec<String>,
}

impl TsxNodeVisitor {
    fn new(text: Vec<String>) -> Self {
        Self { text }
    }
}

fn tsx_node_visitor(visitor: TsxNodeVisitor) -> impl Fold + VisitMut {
    as_folder(visitor)
}

impl VisitMut for TsxNodeVisitor {
    noop_visit_mut_type!();

    fn visit_mut_jsx_text(&mut self, n: &mut JSXText) -> () {
        let words = JsWord::to_string(&n.value);
        let mut trimmed_words = words.replace("\n", "").trim().to_string();
        // cleanup;
        self.text.push(trimmed_words);
    }
}

fn extract_inner_text(src: &str) -> TsxNodeVisitor {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm.new_source_file(FileName::Custom("test.js".into()), src.into());

    let lexer = Lexer::new(
        //we want to parse ecmascript
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: true,
            dts: false,
            no_early_errors: false,
        }),
        EsVersion::Es2015,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let mut module = parser.parse_module().expect("parser was unsuccessful");

    let mut visitor = TsxNodeVisitor::new(vec![]);

    module.body.visit_mut_with(&mut visitor);

    return visitor;
}

fn sanitise_text(input: &str) -> Option<&str> {
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

fn walk_dir() -> () {
    let curr_dir = "/Users/hectorcrean/Desktop/ALL120-beta/app/src/pages/body-contouring";

    for entry in WalkDir::new(curr_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let metadata = fs::metadata(path).expect("@Error: reading metadata");

        if metadata.is_file() && path.ends_with(".tsx") {
            let tsx = fs::read_to_string(path).expect("something went wrong reading the file");

            let result = extract_inner_text(&tsx.as_str());

            let lossy_path_representation = path.to_string_lossy();
            let formatted_path_string = format!("{}.json", lossy_path_representation);
            let output_path = Path::new(formatted_path_string.as_str());

            match (serde_json::to_string(&result), File::create(output_path)) {
                (Ok(data), Ok(mut file)) => {
                    write!(file, "{}", data);
                }
                _ => (),
            }
        }
    }
}

fn main() {
    // generate_typescript();
    walk_dir();
}
