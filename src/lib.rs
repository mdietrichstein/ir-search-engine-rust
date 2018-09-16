#[macro_use] extern crate lazy_static;
extern crate caseless;
extern crate regex;
extern crate rust_stemmers;
extern crate wordnet_stemmer;
#[macro_use] extern crate serde_json;

pub mod preprocessing;
pub mod tokenization;
pub mod indexing;
