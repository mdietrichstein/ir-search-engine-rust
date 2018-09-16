extern crate lazy_static;
extern crate text_retrieval;

use text_retrieval::preprocessing::create_preprocessor;
use text_retrieval::tokenization::create_token_stream;

fn main() {
    let filepaths = vec![
        String::from("./data/TREC8all/Adhoc/latimes/la010189"),
        String::from("./data/TREC8all/Adhoc/latimes/la010190"),
    ];

    let preprocessor = create_preprocessor(true, true, false, true);
    let token_stream = create_token_stream(filepaths, &preprocessor, true, true, true, Some(2));

    token_stream.for_each(|token| {
        println!("{:?}", token);
    });
}
