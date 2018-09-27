use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;
use preprocessing::{split_words, Preprocessor};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::repeat;

// setup

lazy_static! {
    pub static ref DOC_PATTERN: Regex = Regex::new(r"(?s)<DOC>(.*?)</DOC>").unwrap();
    pub static ref DOCNO_PATTERN: Regex = Regex::new(r"<DOCNO>(.*?)</DOCNO>").unwrap();
    pub static ref TEXT_PATTERN: Regex = Regex::new(r"(?s)<TEXT>(.*?)</TEXT>").unwrap();
}

// public interface

#[derive(Debug)]
pub struct Token {
    pub doc_id: String,
    pub term: String,
    pub num_documents_processed: usize,
}

pub fn create_token_stream<'a>(
    filepaths: Vec<String>,
    preprocessor: &'a Preprocessor,
    strip_html_tags: bool,
    strip_html_entities: bool,
    strip_square_bracket_tags: bool,
    min_length: Option<usize>,
) -> Box<Iterator<Item = Token> + 'a> {
    Box::new(
        filepaths
            .into_iter()
            .flat_map(|filepath| regex_parse_documents_from_file(&filepath).unwrap())
            .map(move |document| {
                let (doc_id, content) = document;

                let words = split_words(
                    &content,
                    strip_html_tags,
                    strip_html_entities,
                    strip_square_bracket_tags,
                    min_length,
                );

                (doc_id, words)
            })
            .map(move |(doc_id, words)| {
                let terms = (*preprocessor)(&words);
                (doc_id, terms)
            })
            .enumerate()
            .flat_map(|(doc_num, (doc_id, terms))| {
                repeat((doc_id, doc_num)).take(terms.len()).zip(terms)
            })
            .map(|((doc_id, doc_num), word)| Token {
                doc_id: doc_id,
                term: word,
                num_documents_processed: doc_num,
            }),
    )
}

// private helpers

fn regex_parse_documents_from_file(filepath: &str) -> Result<Vec<(String, String)>, io::Error> {
    let file = File::open(filepath)?;

    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(Encoding::for_label("latin1".as_bytes()))
        .build(file);

    let mut content = String::new();
    decoder.read_to_string(&mut content)?;

    let mut documents: Vec<(String, String)> = vec![];

    for captures in DOC_PATTERN.captures_iter(content.as_str()) {
        let content = match captures.get(1) {
            Some(content) => content.as_str(),
            None => continue,
        };

        let doc_number = match find_capture_at(1, &DOCNO_PATTERN, content) {
            Some(doc_number) => doc_number.to_string(),
            None => continue,
        };

        let text = match find_capture_at(1, &TEXT_PATTERN, content) {
            Some(text) => text.to_string(),
            None => continue,
        };

        documents.push((doc_number, text));
    }

    return Ok(documents);
}

fn find_capture_at<'a>(capture_position: usize, regex: &Regex, text: &'a str) -> Option<&'a str> {
    regex
        .captures(text)
        .and_then(|captures| captures.get(capture_position))
        .map(|capture| capture.as_str().trim())
}
