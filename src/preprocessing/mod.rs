use caseless::default_case_fold_str;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use wordnet_stemmer::{WordnetStemmer, NOUN};

// From https://www.textfixer.com/tutorials/common-english-words.txt via https://en.wikipedia.org/wiki/Stop_words
static STOP_WORDS: &'static [&str] = &[
    "a", "able", "about", "across", "after", "all", "almost", "also", "am", "among", "an", "and",
    "any", "are", "as", "at", "be", "because", "been", "but", "by", "can", "cannot", "could",
    "dear", "did", "do", "does", "either", "else", "ever", "every", "for", "from", "get", "got",
    "had", "has", "have", "he", "her", "hers", "him", "his", "how", "however", "i", "if", "in",
    "into", "is", "it", "its", "just", "least", "let", "like", "likely", "may", "me", "might",
    "most", "must", "my", "neither", "no", "nor", "not", "of", "off", "often", "on", "only", "or",
    "other", "our", "own", "rather", "said", "say", "says", "she", "should", "since", "so", "some",
    "than", "that", "the", "their", "them", "then", "there", "these", "they", "this", "tis", "to",
    "too", "twas", "us", "wants", "was", "we", "were", "what", "when", "where", "which", "while",
    "who", "whom", "why", "will", "with", "would", "yet", "you", "your",
];

// setup
lazy_static! {
    static ref HTML_TAG_PATTERN: Regex = Regex::new(r"<.*?>").unwrap(); 
    static ref HTML_ENTITY_PATTERN: Regex =
        Regex::new(r"&[a-zA-Z][-.a-zA-Z0-9]*[^a-zA-Z0-9]").unwrap();
    static ref SQUARE_BRACKET_TAG_PATTERN: Regex = Regex::new(r"\[.*?\]").unwrap();
    static ref SPLIT_WORDS_PATTERN: Regex =
        Regex::new(r#"\s|\.|:|\?|\(|\)|\[|\]|\{|\}|<|>|'|!|"|-|,|;|\$|\*|%|#"#).unwrap();
    static ref STEMMER: Stemmer = Stemmer::create(Algorithm::English);
    static ref LEMMATIZER: WordnetStemmer = WordnetStemmer::new("./data/wordnet/dict/").unwrap();
}

// type definitions
type PreprocessingStep = fn(&[String]) -> Vec<String>;

// public interface
pub type Preprocessor = Box<Fn(&[String]) -> Vec<String>>;

pub fn split_words(
    text: &str,
    strip_html_tags: bool,
    strip_html_entities: bool,
    strip_square_bracket_tags: bool,
    min_length: Option<usize>,
) -> Vec<String> {
    let mut result = String::from(text);

    if strip_html_tags {
        result = (&HTML_TAG_PATTERN).replace_all(&result, "").to_string();
    }

    if strip_html_entities {
        result = (&HTML_ENTITY_PATTERN).replace_all(&result, "").to_string();
    }

    if strip_square_bracket_tags {
        result = (&SQUARE_BRACKET_TAG_PATTERN)
            .replace_all(&result, "")
            .to_string();
    }

    (&SPLIT_WORDS_PATTERN)
        .split(&result)
        .map(|s| s.to_string())
        .filter(|s| s.len() > min_length.unwrap_or(0))
        .collect()
}

pub fn create_preprocessor(
    enable_case_folding: bool,
    enable_remove_stop_words: bool,
    enable_stemmer: bool,
    enable_lemmatizer: bool,
) -> Preprocessor {
    let mut steps: Vec<PreprocessingStep> = vec![];

    if enable_case_folding {
        steps.push(case_fold);
    }

    if enable_remove_stop_words {
        steps.push(remove_stops_words);
    }

    if enable_stemmer {
        steps.push(stem);
    }

    if enable_lemmatizer {
        steps.push(lemmatize);
    }

    Box::new(move |words| preprocess(&words, &steps))
}

// private helper methods
fn preprocess(words: &[String], steps: &[PreprocessingStep]) -> Vec<String> {
    let mut processed = words.to_vec();

    for preprocessor in steps {
        processed = preprocessor(&processed);
    }

    processed
}

fn case_fold(words: &[String]) -> Vec<String> {
    words.iter().map(|w| default_case_fold_str(w)).collect()
}

fn remove_stops_words(words: &[String]) -> Vec<String> {
    words
        .iter()
        .filter(|w| !STOP_WORDS.contains(&w.as_str()))
        .map(|w| w.to_string())
        .collect()
}

fn stem(words: &[String]) -> Vec<String> {
    words.iter().map(|w| STEMMER.stem(w).to_string()).collect()
}

fn lemmatize(words: &[String]) -> Vec<String> {
    words.iter().map(|w| LEMMATIZER.lemma(NOUN, w)).collect()
}
