use glob::{glob_with, MatchOptions};
use text_retrieval::indexing::create_index_simple;
use text_retrieval::preprocessing::create_preprocessor;

fn main() {
    let preprocessor = create_preprocessor(true, false, false, false);

    let glob_pattern = "./data/TREC8all/Adhoc/latimes/*";

    // let glob_pattern = "./data/TREC8all/Adhoc/**/*";

    // let filepaths: Vec<String> = glob_with(
    //     glob_pattern,
    //     &MatchOptions {
    //         case_sensitive: true,
    //         require_literal_separator: true,
    //         require_literal_leading_dot: false,
    //     },
    // ).unwrap()
    //     .filter_map(Result::ok)
    //     .filter(|p| p.is_file())
    //     .map(|s| s.to_string_lossy().into_owned())
    //     .collect();

    let filepaths = vec![
        "./data/TREC8all/Adhoc/latimes/la010189",
        // String::from("./data/TREC8all/Adhoc/latimes/la010190"),
    ];

    create_index_simple(
        filepaths,
        &preprocessor,
        "simple.index",
        "documents.stats",
        true,
        true,
        true,
        Some(2),
    ).unwrap();
}
