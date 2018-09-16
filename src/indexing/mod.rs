use preprocessing::Preprocessor;
use tokenization::{create_token_stream, Token};

use std::fs;
use std::fs::{File, OpenOptions};
use std::collections::HashMap;
use std::io::{Write, Error};
use std::result::Result;

#[derive(Debug)]
pub struct TokenEntry {
    position: usize,
    term: String,
    document_frequency: usize,
    postings: usize
}

type PostingEntry = (String, usize);

pub fn create_index_simple(filepaths: Vec<String>,
                            preprocessor: &Preprocessor,
                            output_filepath: String,
                            document_stats_path: String,
                            strip_html_tags: bool,
                            strip_html_entities: bool,
                            strip_square_bracket_tags: bool,
                            min_length: Option<usize>) -> Result<(), Error> {

    let tokens = create_token_stream(filepaths, preprocessor,
                                     strip_html_tags, strip_html_entities, strip_square_bracket_tags,
                                     min_length);

    let mut tokens = tokens.collect::<Vec<Token>>();

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
                                .truncate(true)
                                .open(output_filepath)?;

        writeln!(output_file, "{}", num_documents_processed)?;

        for token in tokens.iter() {
            match current_term {
                Some(term) if term == token.term => continue,
                Some(_) | None => {
                    if !document_ids.is_empty() {
                        let postings_list = to_bag_of_words(&document_ids);

                        flush_index_entry(&mut output_file, &token.term,
                                          &postings_list,
                                          &mut document_terms_counter,
                                          &mut document_length_counter)?
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

                flush_index_entry(&mut output_file, term,
                                    &postings_list,
                                    &mut document_terms_counter,
                                    &mut document_length_counter)?;
            }
        }
    }

    write_documents_stats(document_stats_path,
                          &document_terms_counter,
                          &document_length_counter)?;

    Ok(())
}

fn flush_index_entry(file: &mut File, term: &str,
                     postings_list: &[PostingEntry],
                     document_terms_counter: &mut HashMap<String, usize>,
                     document_length_counter: &mut HashMap<String, usize>) -> Result<(), Error> {

    for (document_id, term_frequency) in postings_list {
        {
            let entry = document_terms_counter.entry(document_id.to_string()).or_insert(0);
            *entry += 1;
        }

        {
            let entry = document_length_counter.entry(document_id.to_string()).or_insert(0);
            *entry += term_frequency;
        }
    }

    write_index_entry(file, term, postings_list)
}

fn write_index_entry(file: &mut File, term: &str, postings_list: &[PostingEntry]) -> Result<(), Error> {

    let postings = postings_list.into_iter().map(|(document_id, term_frequency)| 
        format!("{}|{}", document_id, term_frequency)
    ).collect::<Vec<String>>().join(",");

    writeln!(file, "{}\t{}\t{}", term, postings_list.len(), postings)
}

fn write_documents_stats(filepath: String,
                         document_terms_counter: &HashMap<String, usize>,
                         document_length_counter: &HashMap<String, usize>) -> Result<(), Error> {

    let json_stats = json!({
        "terms": document_terms_counter,
        "length": document_length_counter,
    });

    fs::write(filepath, json_stats.to_string().as_bytes())
}

fn to_bag_of_words(words: &Vec<&String>) -> Vec<PostingEntry> {
    let mut bow = HashMap::new();
    
    for word in words {
        let entry = bow.entry(word).or_insert(0);
        *entry += 1;
    }

    bow.into_iter().map(|(k, v)| (k.to_string(), v)).collect::<Vec<PostingEntry>>()
}