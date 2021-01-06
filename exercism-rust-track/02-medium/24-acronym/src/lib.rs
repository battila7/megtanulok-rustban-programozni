pub fn abbreviate(phrase: &str) -> String {
    let mut prepended = String::from(" ");
    prepended.push_str(phrase);

    prepended.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|pair| is_part_of_abbreviation(pair))
        .map(|pair: &[char]| pair[1])
        .collect::<String>()
        .to_ascii_uppercase()
}

fn is_part_of_abbreviation(pair: &[char]) -> bool {
    lowercase_followed_by_uppercase(pair) || punctuation_followed_by_alphanumeric(pair)
}

fn lowercase_followed_by_uppercase(pair: &[char]) -> bool {
    pair[0].is_ascii_lowercase() && pair[1].is_ascii_uppercase()
}

fn punctuation_followed_by_alphanumeric(pair: &[char]) -> bool {
    (pair[0].is_ascii_whitespace() || pair[0] == '_' || pair[0] == '-') && pair[1].is_ascii_alphanumeric()
}
