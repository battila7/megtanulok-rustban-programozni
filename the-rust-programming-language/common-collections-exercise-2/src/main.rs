const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    println!("{:?}", convert_to_pig_latin("apple"));
}

fn convert_to_pig_latin(word: &str) -> Option<String> {
    if let Some(first_char) = word.chars().next() {
        match first_char {
            c if c.is_ascii_alphabetic() && VOWELS.contains(&c) => Some(format!("{}-hay", word)),
            c if c.is_ascii_alphabetic() => Some(format!("{}-{}ay", &word[1..], first_char)),
            _ => None
        }
    } else {
        None
    }
}
