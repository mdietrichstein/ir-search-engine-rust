use regex::Regex;

lazy_static! {
    pub static ref HTML_TAG_PATTERN: Regex = Regex::new(r"<.*?>").unwrap();
    pub static ref HTML_ENTITY_PATTERN: Regex = Regex::new(r"&[a-zA-Z][-.a-zA-Z0-9]*[^a-zA-Z0-9]").unwrap();
    pub static ref SQUARE_BRACKET_TAG_PATTERN: Regex = Regex::new(r"\[.*?\]").unwrap();
    // pub static ref SPLIT_WORDS_PATTERN: Regex = Regex::new(r#"\s|.|:|\?|(|)|[|]|\{|\}|<|>|'|!||"|-|,|;|$|\*|%|#"#).unwrap();
    pub static ref SPLIT_WORDS_PATTERN: Regex = Regex::new(r#"\s|\.|:|\?|\(|\)|\[|\]|\{|\}|<|>|'|!|"|-|,|;|\$|\*|%|#"#).unwrap();
}

pub fn split_words( text: &str,
                    strip_html_tags: bool,
                    strip_html_entities: bool,
                    strip_square_bracket_tags: bool) -> Vec<String> {

    let mut result = String::from(text);

    if strip_html_tags {
        result = (&HTML_TAG_PATTERN).replace_all(&result, "").to_string();
    }

    if strip_html_entities {
        result = (&HTML_ENTITY_PATTERN).replace_all(&result, "").to_string();
    }

    if strip_square_bracket_tags {
        result = (&SQUARE_BRACKET_TAG_PATTERN).replace_all(&result, "").to_string();
    }

    return (&SPLIT_WORDS_PATTERN).split(&result)
                                 .map(|s| s.to_string())
                                 .filter(|s| !s.is_empty())
                                 .collect();
}