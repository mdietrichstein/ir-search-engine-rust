#[macro_use] extern crate lazy_static;
extern crate caseless;
extern crate regex;
extern crate rust_stemmers;
extern crate wordnet_stemmer;
#[macro_use] extern crate serde_json;
extern crate encoding_rs;
extern crate encoding_rs_io;

pub mod preprocessing;
pub mod tokenization;
pub mod indexing;
