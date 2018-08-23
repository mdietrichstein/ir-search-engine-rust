use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

use preprocessing::split_words;

lazy_static! {
    pub static ref DOC_PATTERN: Regex = Regex::new(r"(?s)<DOC>(.*?)</DOC>").unwrap();
    pub static ref DOCNO_PATTERN: Regex = Regex::new(r"<DOCNO>(.*?)</DOCNO>").unwrap();
    pub static ref TEXT_PATTERN: Regex = Regex::new(r"(?s)<TEXT>(.*?)</TEXT>").unwrap();
}

struct Token { doc_id: String, term: String, num_documents_processed: usize }

pub struct TokenGenerator<'a> {
    filepaths: Box<Iterator<Item = &'a String> + 'a>,
    num_documents_processed: usize
}

impl <'a> TokenGenerator<'a> {
    pub fn new(filepaths: &'a Vec<String>) -> TokenGenerator<'a> {
        TokenGenerator {
            filepaths: Box::new(filepaths.iter()),
            num_documents_processed: 0
        }
    }

    fn regex_parse_documents_from_file(&mut self, filepath: &String) -> Result<Vec<(String, String)>, io::Error> {
        let mut file = File::open(filepath)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut documents: Vec<(String, String)> = vec![];

        for captures in DOC_PATTERN.captures_iter(content.as_str()) {
            let content = match captures.get(1) {
                Some(content) => content.as_str(),
                None => continue
            };

            let doc_number = match find_capture_at(1, &DOCNO_PATTERN, content) {
                Some(doc_number) => doc_number.to_string(),
                None => continue
            };

            let text = match find_capture_at(1, &TEXT_PATTERN, content) {
                Some(text) => text.to_string(),
                None => continue
            };

            documents.push((doc_number, text));
        }

        return Ok(documents);
    }
}

impl <'a> Iterator for TokenGenerator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let filepath = match self.filepaths.next() {
                Some(f) => f,
                None => return None
            };

            let documents = self.regex_parse_documents_from_file(filepath);

            let documents = match documents {
                Ok(documents) => documents,
                Err(_) => continue
            };

            for (doc_id, content) in documents {
                let words = split_words(&content, true, true, true);

                self.num_documents_processed += 1;
            }

            return Some(filepath)
        }

    }
}

fn find_capture_at<'a>(capture_position: usize, regex: &Regex, text: &'a str) -> Option<&'a str> {
    regex.captures(text)
        .and_then(|captures| captures.get(capture_position))
        .map(|capture| capture.as_str().trim())
}