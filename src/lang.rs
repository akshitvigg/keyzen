pub fn get_words(lang: &str) -> Vec<&'static str> {
    match lang {
        "rust" => vec![
            "fn", "macro", "let", "impl", "trait", "struct", "enum", "crate", "mut",
        ],
        "english" => vec![
            "hell",
            "heaven",
            "me ",
            "silicon",
            "valley",
            "keyboard",
            "computer",
            "rust",
            "processor",
            "cat",
        ],
        _ => vec!["hello", "world"],
    }
}

