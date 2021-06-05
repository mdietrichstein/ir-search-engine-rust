use crate::preprocessing::Preprocessor;
use crate::tokenization::{create_token_stream, Token};

use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::result::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TokenEntry {
    position: usize,
    term: String,
    document_frequency: usize,
    postings: usize,
}

type PostingEntry = (String, usize);

pub fn create_index_simple<P: AsRef<Path>>(
    filepaths: Vec<P>,
    preprocessor: &Preprocessor,
    output_filepath: P,
    document_stats_path: &str,
    strip_html_tags: bool,
    strip_html_entities: bool,
    strip_square_bracket_tags: bool,
    min_length: Option<usize>,
) -> Result<(), Error> {
    
    let token_stream = create_token_stream(
        filepaths,
        preprocessor,
        strip_html_tags,
        strip_html_entities,
        strip_square_bracket_tags,
        min_length,
    );

    let mut tokens = token_stream.collect::<Vec<Token>>();

    let num_documents_processed = if let Some(token) = tokens.last() {
        token.num_documents_processed
    } else {
        0
    };

    tokens.sort_unstable_by(|a, b| a.term.cmp(&b.term));

    let mut current_term: Option<&str> = None;
    let mut document_ids: Vec<&String> = vec![];

    let mut document_terms_counter = HashMap::new();
    let mut document_length_counter = HashMap::new();

    {
        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_filepath)?;

        writeln!(output_file, "{}", num_documents_processed)?;

        for token in tokens.iter() {
            match current_term {
                Some(term) if term == token.term => {
                    // do nothing
                }
                Some(_) | None => {
                    if !document_ids.is_empty() {
                        let postings_list = to_bag_of_words(&document_ids);

                        flush_index_entry(
                            &mut output_file,
                            &token.term,
                            &postings_list,
                            &mut document_terms_counter,
                            &mut document_length_counter,
                        )?
                    }

                    current_term = Some(&token.term);
                    document_ids = vec![];
                }
            }

            document_ids.push(&token.doc_id);
        }

        if !document_ids.is_empty() {
            if let Some(term) = current_term {
                let postings_list = to_bag_of_words(&document_ids);

                flush_index_entry(
                    &mut output_file,
                    term,
                    &postings_list,
                    &mut document_terms_counter,
                    &mut document_length_counter,
                )?;
            }
        }
    }

    write_documents_stats(
        document_stats_path,
        &document_terms_counter,
        &document_length_counter,
    )?;

    Ok(())
}

pub fn create_index_spimi<P: AsRef<Path>>(
    filepaths: Vec<P>,
    preprocessor: &Preprocessor,
    output_filepath: P,
    document_stats_path: &str,
    strip_html_tags: bool,
    strip_html_entities: bool,
    strip_square_bracket_tags: bool,
    min_length: Option<usize>,
) -> Result<(), Error> {

    let mut token_stream = create_token_stream(
        filepaths,
        preprocessor,
        strip_html_tags,
        strip_html_entities,
        strip_square_bracket_tags,
        min_length,
    );

    let mut block_files: Vec<PathBuf> = vec![];
    let mut num_documents_processed: usize = 0;

    let mut is_exhausted = false;

    loop {
        if is_exhausted {
            break;
        }

        spimi_invert(&mut token_stream, 10_000_000);

    }

    Ok(())
}

fn spimi_invert<'a>(
    token_stream: &mut Box<dyn Iterator<Item=Token> + 'a>,
    max_tokens_per_block: usize
) -> (PathBuf, bool, usize) {
    let processed_tokens: usize = 0;

    for token in token_stream {

    }

    (PathBuf::from("bla"), false, 0)
}

fn flush_index_entry(
    file: &mut File,
    term: &str,
    postings_list: &[PostingEntry],
    document_terms_counter: &mut HashMap<String, usize>,
    document_length_counter: &mut HashMap<String, usize>,
) -> Result<(), Error> {
    for (document_id, term_frequency) in postings_list {
        {
            let entry = document_terms_counter
                .entry(document_id.to_string())
                .or_insert(0);
            *entry += 1;
        }

        {
            let entry = document_length_counter
                .entry(document_id.to_string())
                .or_insert(0);
            *entry += *term_frequency;
        }
    }

    write_index_entry(file, term, postings_list)
}

fn write_index_entry(
    file: &mut File,
    term: &str,
    postings_list: &[PostingEntry],
) -> Result<(), Error> {
    let postings = postings_list
        .iter()
        .map(|(document_id, term_frequency)| format!("{}|{}", document_id, term_frequency))
        .collect::<Vec<String>>()
        .join(",");

    writeln!(file, "{}\t{}\t{}", term, postings_list.len(), postings)
}

fn write_documents_stats(
    filepath: &str,
    document_terms_counter: &HashMap<String, usize>,
    document_length_counter: &HashMap<String, usize>,
) -> Result<(), Error> {
    let json_stats = json!({
        "terms": document_terms_counter,
        "length": document_length_counter,
    });

    fs::write(filepath, json_stats.to_string().as_bytes())
}

fn to_bag_of_words(words: &[&String]) -> Vec<PostingEntry> {
    let mut bow: HashMap<String, usize> = HashMap::new();

    for word in words {
        let entry = bow.entry(word.to_string()).or_insert(0);
        *entry += 1;
    }

    bow.into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect::<Vec<PostingEntry>>()
}
