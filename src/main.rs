extern crate lazy_static;
extern crate text_retrieval;

use text_retrieval::preprocessing::create_preprocessor;
// use text_retrieval::tokenization::create_token_stream;
use text_retrieval::indexing::create_index_simple;

fn main() {
    let filepaths = vec![
        String::from("./data/TREC8all/Adhoc/latimes/la010189"),
        // String::from("./data/TREC8all/Adhoc/latimes/la010190"),
    ];

    let preprocessor = create_preprocessor(true, false, false, false);
    // let token_stream = create_token_stream(filepaths, &preprocessor, true, true, true, Some(2));

    // token_stream.for_each(|token| {
    //     println!("{:?}", token);
    // });

    create_index_simple(filepaths, &preprocessor,
                        String::from("simple.index"),
                        String::from("documents.stats"),
                        true, true, true, Some(2)).unwrap();
}
