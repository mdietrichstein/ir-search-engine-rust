#[macro_use] extern crate lazy_static;
extern crate text_retrieval;

// use text_retrieval::tokenization::TokenGenerator;
use text_retrieval::preprocessing::split_words;

fn main() {
    // let filepaths = vec![
    //         String::from("./data/Adhoc/latimes/la010189"),
    //         String::from("./data/Adhoc/latimes/la010190")
    //     ];

    // let mut it = filepaths.iter();

    // let token_generator = TokenGenerator::new(&filepaths);

    // for token in token_generator {
    // }


    let text = "Hallo <title>Ein titel</title> [B]WICHTIG[/B] supidupi &amp; &nbsp; A:B !a -f #asdf#";
    let words = split_words(text, true, true, true);

    println!("{:?}", words);
}
